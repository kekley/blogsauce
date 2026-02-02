use std::{net::IpAddr, path::PathBuf, sync::Arc};

use bytes::Bytes;
use hotwatch::{EventKind, Hotwatch};
use http_body_util::Full;
use hyper::{Request, Response, StatusCode};
use json::object;
use rand::seq::IndexedRandom;
use smol::lock::{OnceCell, RwLock};

use crate::server::{RequestError, util::json_to_response};

pub static SPLASHES: OnceCell<smol::lock::RwLock<Vec<Arc<str>>>> = OnceCell::new();

pub(crate) async fn get_splash_text_endpoint_get(
    _request: Request<hyper::body::Incoming>,
    _addr: IpAddr,
) -> Result<Response<Full<Bytes>>, RequestError> {
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

pub fn splash_file_watcher(file_path: PathBuf) {
    let mut file_watcher =
        Hotwatch::new().unwrap_or_else(|_| panic!("Could not watch {file_path:?}"));
    let splashes = std::fs::read_to_string(&file_path)
        .map(|file| file.lines().map(Arc::from).collect::<Vec<Arc<str>>>())
        .expect("Could not initialize splash text");

    SPLASHES
        .set_blocking(RwLock::new(splashes))
        .expect("Could not initialize splash text");
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
            EventKind::Remove(_) => {}
            _ => {}
        })
        .unwrap_or_else(|_| panic!("Could not watch {file_path:?}"))
}
