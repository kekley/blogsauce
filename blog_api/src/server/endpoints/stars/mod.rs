use std::net::IpAddr;

use bytes::Bytes;
use http_body_util::Full;
use hyper::{Method, Request, Response, StatusCode, header::HeaderValue};
use json::object;

use crate::{
    db::CommentDb,
    server::{
        RequestError, RequestResult,
        util::{json_to_response, options_response, request_to_json},
    },
};

pub(crate) async fn post_star_endpoint_post(
    request: Request<hyper::body::Incoming>,
    addr: IpAddr,
    db: CommentDb,
) -> RequestResult {
    let mut response_object = object! {};
    match *request.method() {
        Method::OPTIONS => Ok(options_response()),
        Method::POST => {
            let json = request_to_json(request).await?;

            if let Some(token) = json["token"].as_str()
                && let Ok(user) = db.get_user_from_token(token)
            {
                let Some(post_ident) = json["post"].as_str() else {
                    eprintln!("IP: {addr}: Missing post identifier");
                    response_object["error"] = "Error posting comment".into();
                    return Ok(json_to_response(response_object, StatusCode::BAD_REQUEST));
                };

                if post_ident.is_empty() {
                    eprintln!("IP: {addr}: Empty post ident");
                    response_object["error"] = "Error posting comment".into();
                    return Ok(json_to_response(response_object, StatusCode::BAD_REQUEST));
                }

                let post = match db.get_post_with_ident(post_ident) {
                    Ok(post) => post,
                    Err(err) => {
                        eprintln!("IP: {addr}: error fetching post: {err}");

                        response_object["error"] = "Error posting comment".into();
                        return Ok(json_to_response(response_object, StatusCode::BAD_REQUEST));
                    }
                };

                match db.star_post(post.get_id(), user.get_id()) {
                    Ok(_) => Ok(json_to_response(response_object, StatusCode::OK)),
                    Err(err) => {
                        eprintln!("IP: {addr}: error starring post: {err}");
                        Ok(json_to_response(response_object, StatusCode::FORBIDDEN))
                    }
                }
            } else {
                todo!();
            }
        }
        _ => {
            eprintln!("IP: {addr} Invalid Method on Star Post endpoint");
            Ok(json_to_response(
                response_object,
                StatusCode::METHOD_NOT_ALLOWED,
            ))
        }
    }
}
