use std::{fmt::Write as _, net::IpAddr};

use hyper::{Method, Request, StatusCode};
use json::object;
use rand::{TryRngCore as _, rngs::OsRng};
use rusqlite::ErrorCode;

use crate::{
    db::CommentDb,
    models::ip::TruncatedIp,
    server::{
        RequestResult,
        util::{json_to_response, options_response, request_to_json},
    },
};

pub(crate) async fn verify_token_endpoint_get(
    request: Request<hyper::body::Incoming>,
    addr: IpAddr,
    db: CommentDb,
) -> RequestResult {
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
pub(crate) async fn change_color_endpoint_post(
    request: Request<hyper::body::Incoming>,
    addr: IpAddr,
    _db: CommentDb,
) -> RequestResult {
    let mut response_object = object! {};
    match *request.method() {
        Method::OPTIONS => Ok(options_response()),
        Method::POST => {
            response_object["error"] = "not done with this yet oops".into();
            Ok(json_to_response(
                response_object,
                StatusCode::NOT_IMPLEMENTED,
            ))
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

pub(crate) async fn register_name_endpoint_post(
    request: Request<hyper::body::Incoming>,
    addr: IpAddr,
    db: CommentDb,
) -> RequestResult {
    let truncated_ip = TruncatedIp::new(addr);
    let mut response_object = object! {};
    match *request.method() {
        Method::OPTIONS => Ok(options_response()),
        Method::POST => {
            let json = request_to_json(request).await?;

            if let Some(user) = json["display_name"].as_str()
                && !user.is_empty()
            {
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

                match db.add_user(user, &token, truncated_ip) {
                    Ok(_) => {}
                    Err(err) => {
                        if err.sqlite_error_code() == Some(ErrorCode::ConstraintViolation) {
                            eprintln!("IP: {addr}: error adding user: {err}");
                            response_object["error"] = "NAME_TAKEN".into();
                            return Ok(json_to_response(response_object, StatusCode::OK));
                        }
                        eprintln!("IP: {addr}: error adding user: {err}");
                        response_object["error"] = "Internal Error".into();
                        return Ok(json_to_response(response_object, StatusCode::OK));
                    }
                };

                response_object["token"] = token.into();
                Ok(json_to_response(response_object, StatusCode::OK))
            } else {
                response_object["error"] = "No display_name field".into();
                Ok(json_to_response(response_object, StatusCode::OK))
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
