use std::{borrow::Cow, convert::Infallible};

use bytes::Bytes;
use http_body_util::{BodyExt, Full, combinators::BoxBody};
use hyper::{Request, Response, StatusCode, header::HeaderValue};
use json::JsonValue;

use crate::server::RequestError;

pub(crate) fn options_response() -> Response<BoxBody<Bytes, Infallible>> {
    let body = Full::new(Bytes::new()).boxed();
    Response::builder()
        .status(StatusCode::OK)
        .header("Allow", "POST, GET, OPTIONS")
        .header("Access-Control-Allow-Headers", "Content-Type")
        .header("Access-Control-Allow-Origin", "*")
        .body(body)
        .expect("OPTIONS response should be valid")
}

pub(crate) fn extract_key_from_query<'a>(query: &'a str, key: &str) -> Option<Cow<'a, str>> {
    query
        .split("&")
        .flat_map(|pair| pair.split_once("="))
        .find(|(query_key, _)| query_key.eq(&key))
        .and_then(|(_, value)| urlencoding::decode(value).ok())
}
pub(crate) fn json_to_response(
    json: JsonValue,
    status_code: StatusCode,
) -> Response<BoxBody<Bytes, Infallible>> {
    let body = Full::new(Bytes::from(json.dump())).boxed();
    Response::builder()
        .header("Access-Control-Allow-Origin", HeaderValue::from_static("*"))
        .status(status_code)
        .body(body)
        .unwrap_or_default()
}

pub(crate) async fn request_to_json(
    request: Request<hyper::body::Incoming>,
) -> Result<JsonValue, RequestError> {
    let body = request.collect().await?.to_bytes();
    let as_str = str::from_utf8(&body)?;
    Ok(json::parse(as_str)?)
}
