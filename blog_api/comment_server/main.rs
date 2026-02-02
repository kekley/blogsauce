use areq_smol::ClientExt;
use areq_smol::body::BodyExt as _;
use areq_smol::http1::Http1;
use areq_smol::smol::{Connect, Handle};
use areq_smol::tls::Tls;
use async_channel::unbounded;
use async_executor::Executor;
use clap::Parser;
use comment_server::db::CommentDb;
use comment_server::server::handle_request;
use easy_parallel::Parallel;
use futures_lite::future;
use hyper::StatusCode;
use hyper::service::service_fn;
use hyper::{Response, server::conn::http1::Builder};
use json::JsonValue;
use mimalloc::MiMalloc;
use smol::net::TcpListener;
use smol::spawn;
use std::path::Path;
use std::str::FromStr;
use std::time::Duration;
use std::{net::SocketAddr, path::PathBuf};

/*
async fn rate_limit_cleanup(limits: Arc<Mutex<HashMap<IpAddr, RateEntry>>>) {
    let mut interval = tokio::time::interval(CLEANUP_INTERVAL);

    loop {
        interval.tick().await;
        let now = Instant::now();

        let mut map = limits.lock().await;
        let before = map.len();

        map.retain(|_, entry| now.duration_since(entry.window_start) <= RATE_WINDOW);

        let after = map.len();
        if before != after {
            println!(
                "Rate limiter cleanup: removed {} stale entries",
                before - after
            );
        }
    }
}


#[derive(Debug)]
struct RateEntry {
    count: u32,
    window_start: Instant,
}

async fn check_rate_limit(limits: &Arc<Mutex<HashMap<IpAddr, RateEntry>>>, ip: IpAddr) -> bool {
    let mut map = limits.lock().await;
    let now = Instant::now();

    let entry = map.entry(ip).or_insert(RateEntry {
        count: 0,
        window_start: now,
    });

    if now.duration_since(entry.window_start) > RATE_WINDOW {
        entry.count = 0;
        entry.window_start = now;
    }

    entry.count += 1;
    entry.count <= RATE_LIMIT
}
*/

fn main() {}

async fn server_loop() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr: SocketAddr = ([0, 0, 0, 0], listen_port).into();
    let listener = TcpListener::bind(addr).await?;
    if !json_posts_url.is_empty() {
        spawn(post_list_updater(
            post_list_update_interval_secs,
            json_posts_url,
            CommentDb::from_pooled_conn(pool_arc.get().unwrap()),
        ))
        .detach();
    }

    let passwords_file_path: &'static Path = passwords_file_path.leak();

    //    let rate_limits: Arc<Mutex<HashMap<IpAddr, RateEntry>>> = Arc::new(Mutex::new(HashMap::new()));

    println!("Listening on http://{}", addr);

    loop {
        let (tcp, addr) = listener.accept().await?;

        //Wrap the stream in a type that implements the hyper read and write traits
        let io = smol_hyper::rt::FuturesIo::new(tcp);

        let pool_clone = pool_arc.clone();
        let passwords_path = passwords_file_path;
        //        let rate_limits = rate_limits.clone();
        let _ = spawn(async move {
            let Ok(passwords_contents) = smol::fs::read_to_string(passwords_path).await else {
                panic!("Could not read passwords file");
            };
            if let Err(err) = Builder::new()
                .keep_alive(false)
                .serve_connection(
                    io,
                    service_fn(move |request| {
                        // :') satisfying send + sync + 'static requirements
                        let value = pool_clone.clone();
                        let passwords_contents = passwords_contents.clone();
                        //                        let rate_limits = rate_limits.clone();
                        let ip = addr.ip();

                        async move {
                            //Rate limit the getUser endpoint because it'd probably be really obnoxious to
                            //have our db filled with random user strings if this endpoint gets spammed
                            if request.uri().path() == "/getUser"
                            //                                && !check_rate_limit(&rate_limits, ip).await
                            {
                                return Ok::<_, hyper::http::Error>(
                                    Response::builder()
                                        .status(StatusCode::TOO_MANY_REQUESTS)
                                        .body("Too many requests\n".into())
                                        .unwrap(),
                                );
                            }
                            handle_request(
                                passwords_contents,
                                request,
                                addr,
                                CommentDb::from_pooled_conn(value.get().unwrap()),
                            )
                            .await
                        }
                    }),
                )
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        })
        .await;
    }
}
