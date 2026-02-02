use areq_smol::ClientExt as _;
use areq_smol::body::BodyExt as _;
use areq_smol::http1::Http1;
use areq_smol::smol::{Connect as _, Handle as _};
use areq_smol::tls::Tls;
use async_channel::unbounded;
use blog_api::db::CommentDb;
use blog_api::server::endpoints::splashes::splash_file_watcher;
use blog_api::server::handle_request;
use clap::Parser;
use easy_parallel::Parallel;
use hyper::Request;
use hyper::body::Incoming;
use hyper::server::conn::http1::Builder;
use hyper::service::service_fn;
use json::JsonValue;
use mimalloc::MiMalloc;
use smol::net::TcpListener;
use smol::{future, spawn};
use smol_hyper::rt::SmolTimer;
use std::net::{IpAddr, SocketAddr};
use std::path::PathBuf;
use std::str::FromStr as _;
use std::time::Duration;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[derive(Debug, Parser)]
pub struct Settings {
    #[arg(
        help = "The path where the database resides or will be created",
        short = 'd',
        long,
        default_value = "./comments.sqlite"
    )]
    database_path: PathBuf,
    #[arg(
        help = "The path where the tab title splashes will be read from",
        short = 'd',
        long,
        default_value = "../splashes.txt"
    )]
    splashes_path: PathBuf,

    #[arg(
        help = "The port the server will listen on",
        short = 'p',
        long,
        default_value_t = 3000
    )]
    listen_port: u16,
    #[arg(
        help = "The url where the list of posts will be fetched from. If empty, the post list will not be updated automatically",
        short = 'j',
        long,
        default_value = ""
    )]
    json_posts_url: String,
    #[arg(
        help = "The interval at which the post list is updated from the url",
        short = 'u',
        long,
        default_value_t = 300
    )]
    post_list_update_interval_secs: u32,

    #[arg(
        help = "The window for rate limiting the getUser endpoint",
        short = 'r',
        long,
        default_value = "300"
    )]
    rate_limit_window_secs: u32,
    #[arg(
        help = "The interval at which rate limit entries are cleaned up",
        short = 'c',
        long,
        default_value = "300"
    )]
    rate_limit_cleanup_interval_secs: u32,
    #[arg(
        help = "The number of allowed hits to the getUser endpoint before rate limiting",
        short = 'l',
        long,
        default_value = "10"
    )]
    rate_limit: u32,
}

fn main() {
    let ex = smol::Executor::new();
    let (signal, shutdown) = unbounded::<()>();

    Parallel::new()
        // Run four executor threads.
        .each(0..4, |_| future::block_on(ex.run(shutdown.recv())))
        // Run the main future on the current thread.
        .finish(|| {
            future::block_on(async {
                match server().await {
                    Ok(_) => {}
                    Err(err) => {
                        eprintln!("Server exited with error: {err}");
                    }
                }
                drop(signal);
            })
        });
}

async fn server() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let Settings {
        database_path,
        listen_port,
        json_posts_url,
        post_list_update_interval_secs,
        rate_limit_window_secs,
        rate_limit_cleanup_interval_secs,
        rate_limit,
        splashes_path,
    } = Settings::parse();

    let addr: SocketAddr = ([0, 0, 0, 0], listen_port).into();

    let db_connection_pool = CommentDb::create_db(&database_path);

    let listener = TcpListener::bind(addr).await?;
    if !json_posts_url.is_empty() {
        spawn(post_list_updater(
            post_list_update_interval_secs,
            json_posts_url,
            CommentDb::from_pooled_conn(db_connection_pool.get().unwrap()),
        ))
        .detach();
    }
    if !splashes_path.to_string_lossy().is_empty() {
        splash_file_watcher(splashes_path);
    }
    println!("Listening on http://{}", addr);

    loop {
        let (tcp, socket_addr) = listener.accept().await?;
        let ip_addr = socket_addr.ip();
        let pool = db_connection_pool.clone();

        //Wrap the stream in a type that implements the hyper read and write traits
        let io = smol_hyper::rt::FuturesIo::new(tcp);

        spawn(async move {
            if let Err(err) = Builder::new()
                .timer(SmolTimer::new())
                .keep_alive(false)
                .serve_connection(
                    io,
                    service_fn(move |request: Request<Incoming>| {
                        let db = CommentDb::from_pooled_conn(pool.get().unwrap());
                        let origin = request
                            .headers()
                            .get("x-forwarded-for")
                            .and_then(|value| value.to_str().ok())
                            .map(|s| s.split(',').next().unwrap_or(s).trim());
                        let ip_addr = if let Some(origin) = origin {
                            dbg!(origin);
                            IpAddr::from_str(origin).unwrap_or(ip_addr)
                        } else {
                            ip_addr
                        };

                        println!("IP: {ip_addr}, Endpoint: {}", request.uri().path());

                        handle_request(request, ip_addr, db)
                    }),
                )
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        })
        .detach();
    }
}

async fn post_list_updater(interval: u32, json_posts_url: String, db: CommentDb) {
    loop {
        //TODO have a channel for signaling that a post fetch failed
        if let Ok(str) = fetch_url(&json_posts_url).await
            && let Ok(json) = json::parse(&str)
            && let JsonValue::Array(posts) = &json["posts"]
        {
            let posts_str = posts.iter().flat_map(|v| v.as_str());

            db.update_posts(posts_str);
        }
        smol::Timer::interval(Duration::from_secs(interval.into())).await;
    }
}

#[derive(Debug, thiserror::Error)]
enum UrlFetchError {
    #[error("Invalid URI: {0}")]
    InvalidUri(#[from] areq_smol::http::uri::InvalidUri),
    #[error("{0}")]
    AreqError(#[from] areq_smol::Error),
    #[error("{0}")]
    IoError(#[from] std::io::Error),
}

async fn fetch_url(url: &str) -> Result<String, UrlFetchError> {
    let uri = areq_smol::http::Uri::from_str(url)?;
    let tls = Tls::with_webpki_roots(Http1::default());
    Ok(tls
        .connect(&uri)
        .await?
        .handle(async move |client| client.get(uri, ()).await?.text().await)
        .await?)
}
