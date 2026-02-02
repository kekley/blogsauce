use std::borrow::Cow;

use bytes::Bytes;
use http_body_util::{BodyExt as _, Full};
use hyper::{Request, Response, StatusCode, header::HeaderValue};
use json::JsonValue;

use crate::server::RequestError;

pub(crate) fn options_response() -> Response<Full<Bytes>> {
    Response::builder()
        .status(StatusCode::OK)
        .header("Allow", "POST, GET, OPTIONS")
        .header("Access-Control-Allow-Headers", "Content-Type")
        .header("Access-Control-Allow-Origin", "*")
        .body(Full::new(Bytes::new()))
        .expect("OPTIONS response should be valid")
}
pub(crate) fn extract_post_identifier(query: &str) -> Option<Cow<'_, str>> {
    query
        .split("&")
        .flat_map(|pair| pair.split_once("="))
        .find(|(key, _)| key.eq(&"post"))
        .and_then(|(_, value)| urlencoding::decode(value).ok())
}
pub(crate) fn json_to_response(json: JsonValue, status_code: StatusCode) -> Response<Full<Bytes>> {
    Response::builder()
        .header("Access-Control-Allow-Origin", HeaderValue::from_static("*"))
        .status(status_code)
        .body(Full::new(Bytes::from(json.dump())))
        .unwrap_or_default()
}

pub(crate) async fn request_to_json(
    request: Request<hyper::body::Incoming>,
) -> Result<JsonValue, RequestError> {
    let body = request.collect().await?.to_bytes();
    let as_str = str::from_utf8(&body)?;
    Ok(json::parse(as_str)?)
}
