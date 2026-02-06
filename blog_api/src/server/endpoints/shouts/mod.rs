use std::{convert::Infallible, net::IpAddr, str::FromStr, sync::Arc, time::Duration};

use async_broadcast::Sender;
use bytes::Bytes;
use futures::FutureExt as _;
use http_body_util::{BodyExt, Full, StreamBody, combinators::BoxBody};
use hyper::{
    Method, Request, Response, StatusCode,
    body::{Body, Frame},
    header::{CACHE_CONTROL, CONNECTION, CONTENT_TYPE, HeaderValue},
};
use jiff::Timestamp;
use json::object;
use smol::Timer;

use crate::{
    db::CommentDb,
    models::{
        shout::{Shout, ShoutEvent},
        user::Color,
    },
    server::{
        RequestError, RequestResult,
        util::{extract_key_from_query, json_to_response, options_response, request_to_json},
    },
};

pub(crate) async fn post_shout_endpoint_post(
    request: Request<hyper::body::Incoming>,
    addr: IpAddr,
    db: CommentDb,
    shout_events: Sender<Arc<ShoutEvent>>,
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

                let r = match db.add_shout(user.get_id(), content) {
                    Ok(_) => Ok(json_to_response(response_object, StatusCode::OK)),
                    Err(_err) => {
                        response_object["error"] = "Error posting shout".into();
                        Ok(json_to_response(response_object, StatusCode::BAD_REQUEST))
                    }
                };
                if r.is_ok() {
                    let _ = shout_events
                        .broadcast(Arc::new(ShoutEvent {
                            display_name: user.get_display_name().to_string(),
                            content: ammonia::clean(content),
                            user_color: user.get_color().to_string(),
                        }))
                        .await;
                }
                r
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
) -> RequestResult {
    let mut response_object = object! {};
    match *request.method() {
        Method::OPTIONS => Ok(options_response()),
        Method::POST => {
            let json = request_to_json(request).await;
            let shouts_before_id = json
                .as_ref()
                .ok()
                .and_then(|json| json["shouts_before_id"].as_i64());

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
            eprintln!("IP: {addr} Invalid Method on get shouts endpoint");
            Ok(json_to_response(
                response_object,
                StatusCode::METHOD_NOT_ALLOWED,
            ))
        }
    }
}

pub(crate) async fn subscribe_shouts_endpoint(
    request: Request<hyper::body::Incoming>,
    addr: IpAddr,
    _db: CommentDb,
    shout_events: Sender<Arc<ShoutEvent>>,
) -> Result<Response<BoxBody<Bytes, Infallible>>, RequestError> {
    match *request.method() {
        Method::OPTIONS => Ok(options_response()),
        Method::GET => {
            let mut rx = shout_events.new_receiver();

            let stream = async_stream::stream! {
                loop {
                    futures::select! {
                        msg = rx.recv().fuse() => {
                            match msg {
                                Ok(shout) => {
                                    let json = object!{
                                        user_color: shout.user_color.as_str(),
                                        display_name: shout.display_name.as_str(),
                                        content: shout.content.as_str(),
                                    };

                                    yield Ok::<Frame<Bytes>, Infallible>(
                                        Frame::data(Bytes::from(format!("data: {json}\n\n")))
                                    );
                                }
                                Err(async_broadcast::RecvError::Overflowed(_)) => {
                                    continue;
                                }
                                Err(async_broadcast::RecvError::Closed) => {
                                    dbg!("lol");
                                    break;
                                }
                            }
                        }

                        _ = Timer::after(Duration::from_secs(1)).fuse() => {
                            // SSE heartbeat comment
                            yield Ok::<Frame<Bytes>, Infallible>(
                                Frame::data(Bytes::from(": keep-alive\n\n"))
                            );
                        }
                    }
                }
            };

            let boxed = StreamBody::new(stream).boxed();

            Ok(Response::builder()
                .status(StatusCode::OK)
                .header(CACHE_CONTROL, "no-cache")
                .header(CONNECTION, "keep-alive")
                .header("Access-Control-Allow-Origin", HeaderValue::from_static("*"))
                .header("X-Accel-Buffering", "no")
                .header(CONTENT_TYPE, "text/event-stream; charset=utf-8")
                .body(boxed)
                .unwrap())
        }
        _ => {
            eprintln!("IP: {addr} Invalid Method on new shouts endpoint");
            Ok(json_to_response(object! {}, StatusCode::METHOD_NOT_ALLOWED))
        }
    }
}
