use std::{fmt::Write as _, net::IpAddr};

use bytes::Bytes;
use http_body_util::Full;
use hyper::{Method, Request, Response, StatusCode};
use json::object;
use rand::{TryRngCore as _, rngs::OsRng};

use crate::{
    db::CommentDb,
    server::{
        RequestError,
        util::{json_to_response, options_response, request_to_json},
    },
};

pub(crate) async fn verify_token(
    request: Request<hyper::body::Incoming>,
    addr: IpAddr,
    db: CommentDb,
) -> Result<Response<Full<Bytes>>, RequestError> {
    let mut response_object = object! {};
    match *request.method() {
        Method::OPTIONS => Ok(options_response()),
        Method::POST => {
            let Ok(json) = request_to_json(request).await else {
                response_object["error"] = "Invalid JSON in body".into();
                return Ok(json_to_response(response_object, StatusCode::BAD_REQUEST));
            };
            if let Some(token) = json["token"].as_str()
                && let Ok(_) = db.get_user_from_token(token)
            {
                response_object["is_valid"] = true.into();
                Ok(json_to_response(response_object, StatusCode::OK))
            } else {
                response_object["is_valid"] = false.into();
                Ok(json_to_response(response_object, StatusCode::OK))
            }
        }
        _ => {
            eprintln!("IP: {addr} Invalid Method on verify user endpoint");
            Ok(json_to_response(
                response_object,
                StatusCode::METHOD_NOT_ALLOWED,
            ))
        }
    }
}

pub(crate) async fn get_user_endpoint_post(
    request: Request<hyper::body::Incoming>,
    addr: IpAddr,
    db: CommentDb,
) -> Result<Response<Full<Bytes>>, RequestError> {
    let mut response_object = object! {};
    match *request.method() {
        Method::OPTIONS => Ok(options_response()),
        Method::POST => {
            let json = request_to_json(request).await?;

            if let Some(user) = json["display_name"].as_str() {
                let mut buf = [0u8; 16];
                match OsRng.try_fill_bytes(&mut buf) {
                    Ok(_) => {}
                    Err(err) => {
                        eprintln!("IP: {addr}: failed to generate user token : {err}");
                        response_object["error"] = "Error generating user token".into();
                    }
                }

                let mut token = String::new();

                for byte in buf {
                    let _ = write!(&mut token, "{byte:02X}");
                }

                match db.add_user(user, &token) {
                    Ok(_) => {}
                    Err(err) => {
                        eprintln!("IP: {addr}: error adding user: {err}");
                        response_object["error"] = "NAME_TAKEN".into();
                        return Ok(json_to_response(response_object, StatusCode::IM_A_TEAPOT));
                    }
                };

                response_object["token"] = token.into();
                Ok(json_to_response(response_object, StatusCode::OK))
            } else {
                todo!();
            }
        }
        _ => {
            eprintln!("IP: {addr} Invalid Method on Comment Get endpoint");
            Ok(json_to_response(
                response_object,
                StatusCode::METHOD_NOT_ALLOWED,
            ))
        }
    }
}
