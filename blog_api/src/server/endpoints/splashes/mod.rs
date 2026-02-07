use std::{net::IpAddr, path::PathBuf, sync::Arc};

use hotwatch::{EventKind, Hotwatch};
use hyper::{Method, Request, StatusCode};
use json::object;
use rand::seq::IndexedRandom;
use smol::lock::{OnceCell, RwLock};

use crate::server::{
    RequestResult,
    util::{json_to_response, options_response},
};

pub static SPLASHES: OnceCell<smol::lock::RwLock<Vec<Arc<str>>>> = OnceCell::new();

pub(crate) async fn get_splash_text_endpoint_get(
    request: Request<hyper::body::Incoming>,
    addr: IpAddr,
) -> RequestResult {
    match *request.method() {
        Method::OPTIONS => Ok(options_response()),
        Method::GET => {
            let splashes = SPLASHES.wait().await.read().await;
            let mut rng = rand::rng();
            let splash: &str = splashes
                .choose(&mut rng)
                .map(|arc| &arc[..])
                .unwrap_or_default();
            let mut response_object = object! {};
            response_object["splash"] = splash.into();
            Ok(json_to_response(response_object, StatusCode::OK))
        }
        _ => {
            eprintln!("IP: {addr} Invalid Method on new shouts endpoint");
            Ok(json_to_response(object! {}, StatusCode::METHOD_NOT_ALLOWED))
        }
    }
}

pub fn splash_file_watcher(mut file_path: PathBuf) -> Hotwatch {
    let mut file_watcher =
        Hotwatch::new().unwrap_or_else(|_| panic!("Could not watch {file_path:?}"));
    let splashes = std::fs::read_to_string(&file_path)
        .map(|file| file.lines().map(Arc::from).collect::<Vec<Arc<str>>>())
        .expect("Could not initialize splash text");

    SPLASHES
        .set_blocking(RwLock::new(splashes))
        .expect("Could not initialize splash text");
    //Watch the folder instead of the file to get around some weird behavior
    file_path.pop();
    file_watcher
        .watch(&file_path, |event| match event.kind {
            EventKind::Create(_) | EventKind::Modify(_) => {
                if let Ok(file_contents) = std::fs::read_to_string(&event.paths[0]) {
                    let lock = SPLASHES.wait_blocking();
                    let mut guard = lock.write_blocking();
                    guard.clear();
                    guard.extend(file_contents.lines().map(Arc::from));
                }
            }
            _ => {}
        })
        .unwrap_or_else(|_| panic!("Could not watch {file_path:?}"));
    file_watcher
}
