use async_broadcast::broadcast;
use async_channel::unbounded;
use blog_api::args::parse_settings_from_args;
use blog_api::db::sqlite::CommentDb;
use blog_api::server::endpoints::splashes::splash_file_watcher;
use blog_api::server::handle_request;
use easy_parallel::Parallel;
use hyper::Request;
use hyper::body::Incoming;
use hyper::server::conn::http1::Builder;
use hyper::service::service_fn;
use json::JsonValue;
use nano_get::NanoGetError;
use smol::net::TcpListener;
use smol::{future, spawn};
use smol_hyper::rt::SmolTimer;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr as _;
use std::time::Duration;

fn main() {
    eprintln!("Starting Server!");

    let ex = smol::Executor::new();
    let (shutdown_signal, shutdown) = unbounded::<()>();

    Parallel::new()
        .each(0..4, |_| future::block_on(ex.run(shutdown.recv())))
        .finish(|| {
            future::block_on(async {
                match server().await {
                    Ok(_) => {}
                    Err(err) => {
                        eprintln!("Server exited with error:{err}");
                    }
                }
                drop(shutdown_signal);
            })
        });
}

async fn server() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let args = std::env::args_os().skip(1);
    let settings = parse_settings_from_args(args)?;

    let addr: SocketAddr = ([0, 0, 0, 0], settings.listen_port).into();

    let db_connection_pool = CommentDb::create_db(&settings.database_path);

    eprintln!("Connected to DB");

    let listener = TcpListener::bind(addr).await?;

    eprintln!("Listening on {addr}");

    if let Some(posts_url) = settings.post_list_url {
        eprintln!("Looking for post list json at: {}", posts_url);
        spawn(post_list_updater(
            3,
            posts_url,
            CommentDb::from_pooled_conn(db_connection_pool.get().unwrap()),
        ))
        .detach();
    }
    let _watcher = if !settings.splash_file_path.exists() {
        eprintln!("Watching splashes file at {:?}", &settings.splash_file_path);

        Some(splash_file_watcher(settings.splash_file_path))
    } else {
        None
    };

    let (mut shout_tx, _shout_rx) = broadcast(10);
    shout_tx.set_overflow(true);

    loop {
        let shout_sender = shout_tx.clone();
        let (tcp, socket_addr) = listener.accept().await?;
        let ip_addr = socket_addr.ip();
        let pool = db_connection_pool.clone();

        //Wrap the stream in a type that implements the hyper read and write traits
        let io = smol_hyper::rt::FuturesIo::new(tcp);

        spawn(async move {
            if let Err(err) = Builder::new()
                .timer(SmolTimer::new())
                .keep_alive(true)
                .serve_connection(
                    io,
                    service_fn(move |request: Request<Incoming>| {
                        let s = shout_sender.clone();
                        let db = CommentDb::from_pooled_conn(pool.get().unwrap());
                        let origin = request
                            .headers()
                            .get("x-forwarded-for")
                            .and_then(|value| value.to_str().ok())
                            .map(|s| s.split(',').next().unwrap_or(s).trim());
                        let ip_addr = if let Some(origin) = origin {
                            IpAddr::from_str(origin).unwrap_or(ip_addr)
                        } else {
                            ip_addr
                        };

                        eprintln!(
                            "Received connection from: {ip_addr} for {}",
                            request.uri().path()
                        );

                        handle_request(request, ip_addr, db, s)
                    }),
                )
                .await
            {
                eprintln!("Error serving connection: {err}");
            }
        })
        .detach();
    }
}

async fn post_list_updater(interval: u32, json_posts_url: String, db: CommentDb) {
    loop {
        //TODO have a channel for signaling that a post fetch failed so we can try a refresh before
        // returning an error
        if let Ok(str) = fetch_url(&json_posts_url).await
            && let Ok(json) = json::parse(&str)
            && let JsonValue::Array(posts) = &json["posts"]
        {
            let posts_str = posts.iter().flat_map(|v| v.as_str());
            eprintln!("Updated post list");

            db.update_posts(posts_str);
        }
        smol::Timer::interval(Duration::from_secs(interval.into())).await;
    }
}

async fn fetch_url(url: &str) -> Result<String, NanoGetError> {
    nano_get::get(url)
}
