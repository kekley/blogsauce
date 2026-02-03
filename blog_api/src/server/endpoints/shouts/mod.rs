use std::{net::IpAddr, str::FromStr};

use bytes::Bytes;
use http_body_util::Full;
use hyper::{Method, Request, Response, StatusCode};
use jiff::Timestamp;
use json::object;

use crate::{
    db::CommentDb,
    models::user::Color,
    server::{
        RequestError,
        util::{extract_key_from_query, json_to_response, options_response, request_to_json},
    },
};

pub(crate) async fn post_shout_endpoint_post(
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
                && let Ok(user) = db.get_user_from_token(token)
            {
                let Some(content) = json["content"].as_str() else {
                    response_object["error"] = "Missing comment content".into();
                    return Ok(json_to_response(response_object, StatusCode::BAD_REQUEST));
                };
                if content.is_empty() {
                    response_object["error"] = "Empty comment".into();
                    return Ok(json_to_response(response_object, StatusCode::BAD_REQUEST));
                }

                match db.add_shout(user.get_id(), content) {
                    Ok(_) => Ok(json_to_response(response_object, StatusCode::OK)),
                    Err(_err) => {
                        response_object["error"] = "Error posting shout".into();
                        Ok(json_to_response(response_object, StatusCode::BAD_REQUEST))
                    }
                }
            } else {
                response_object["error"] = "Invalid user token".into();
                Ok(json_to_response(response_object, StatusCode::BAD_REQUEST))
            }
        }
        _ => {
            eprintln!("IP: {addr} Invalid Method on Shout Post endpoint");
            Ok(json_to_response(
                response_object,
                StatusCode::METHOD_NOT_ALLOWED,
            ))
        }
    }
}

pub(crate) async fn edit_shout_endpoint_post(
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
                let Some(content) = json["content"].as_str() else {
                    response_object["error"] = "Missing comment content".into();
                    return Ok(json_to_response(response_object, StatusCode::BAD_REQUEST));
                };
                if content.is_empty() {
                    response_object["error"] = "Empty comment".into();
                    return Ok(json_to_response(response_object, StatusCode::BAD_REQUEST));
                }

                let Some(shout_id) = json["shout_id"].as_i64() else {
                    response_object["error"] = "Missing shout_id".into();
                    return Ok(json_to_response(response_object, StatusCode::BAD_REQUEST));
                };
                let Ok(shout) = db.get_shout_from_id(shout_id) else {
                    response_object["error"] = "Invalid shout id".into();
                    return Ok(json_to_response(response_object, StatusCode::BAD_REQUEST));
                };

                match db.edit_shout(shout.get_id(), content) {
                    Ok(_) => Ok(json_to_response(response_object, StatusCode::OK)),
                    Err(_err) => {
                        response_object["error"] = "Error posting shout".into();
                        Ok(json_to_response(response_object, StatusCode::BAD_REQUEST))
                    }
                }
            } else {
                response_object["error"] = "Invalid user token".into();
                Ok(json_to_response(response_object, StatusCode::BAD_REQUEST))
            }
        }
        _ => {
            eprintln!("IP: {addr} Invalid Method on Shout Post endpoint");
            Ok(json_to_response(
                response_object,
                StatusCode::METHOD_NOT_ALLOWED,
            ))
        }
    }
}

pub(crate) async fn delete_shout_endpoint_post(
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
                let Some(shout_id) = json["shout_id"].as_i64() else {
                    response_object["error"] = "Missing shout_id".into();
                    return Ok(json_to_response(response_object, StatusCode::BAD_REQUEST));
                };
                let Ok(shout) = db.get_shout_from_id(shout_id) else {
                    response_object["error"] = "Invalid shout id".into();
                    return Ok(json_to_response(response_object, StatusCode::BAD_REQUEST));
                };

                match db.delete_shout(shout.get_id()) {
                    Ok(_) => Ok(json_to_response(response_object, StatusCode::OK)),
                    Err(_err) => {
                        response_object["error"] = "Error posting shout".into();
                        Ok(json_to_response(response_object, StatusCode::BAD_REQUEST))
                    }
                }
            } else {
                response_object["error"] = "Invalid user token".into();
                Ok(json_to_response(response_object, StatusCode::BAD_REQUEST))
            }
        }
        _ => {
            eprintln!("IP: {addr} Invalid Method on Shout Post endpoint");
            Ok(json_to_response(
                response_object,
                StatusCode::METHOD_NOT_ALLOWED,
            ))
        }
    }
}

///Returns the 10 most recent comments. `shouts_before` can be specified to get the 10 most recent
///comments before the specified date
pub(crate) async fn get_shouts_endpoint_get(
    request: Request<hyper::body::Incoming>,
    addr: IpAddr,
    db: CommentDb,
) -> Result<Response<Full<Bytes>>, RequestError> {
    let mut response_object = object! {};
    match *request.method() {
        Method::OPTIONS => Ok(options_response()),
        Method::GET => {
            let json = request_to_json(request).await;
            let date = json
                .as_ref()
                .ok()
                .and_then(|json| json["shouts_before"].as_str());

            let shouts = db
                .get_all_shouts()
                .expect("We should be able to query the shouts table");

            let mut shouts_vec = Vec::with_capacity(shouts.len());

            for shout in shouts {
                let mut shout_json = object! {};
                let user = db.get_user_by_id(shout.get_user_id());
                let (display_name, color) = user
                    .as_ref()
                    .map(|user| (user.get_display_name(), user.get_color()))
                    .unwrap_or(("DELETED_USER", Color::WHITE));

                shout_json["display_name"] = display_name.into();
                shout_json["user_color"] = color.to_string().into();
                shout_json["content"] = shout.get_content().into();
                shout_json["edited"] = shout.was_edited().into();
                shout_json["date"] = shout.get_datetime().to_string().into();
                shouts_vec.push(shout_json);
            }

            response_object["shouts"] = json::JsonValue::Array(shouts_vec);

            Ok(json_to_response(response_object, StatusCode::OK))
        }
        _ => {
            eprintln!("IP: {addr} Invalid Method on Shout Post endpoint");
            Ok(json_to_response(
                response_object,
                StatusCode::METHOD_NOT_ALLOWED,
            ))
        }
    }
}
