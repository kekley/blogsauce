use std::{convert::Infallible, net::IpAddr, sync::Arc, time::Duration};

use async_broadcast::Sender;
use bytes::Bytes;
use futures::FutureExt as _;
use http_body_util::{BodyExt, StreamBody, combinators::BoxBody};
use hyper::{
    Method, Request, Response, StatusCode,
    body::Frame,
    header::{CACHE_CONTROL, CONNECTION, CONTENT_TYPE, HeaderValue},
};
use json::object;
use smol::Timer;

use crate::{
    db::CommentDb,
    json::extract_json_field,
    models::{shout::ShoutEvent, user::Color},
    server::{
        RequestError, RequestResult,
        endpoints::{CONTENT_FIELD_NAME, SHOUT_ID_FIELD_NAME, TOKEN_FIELD_NAME},
        util::{extract_key_from_query, json_to_response, options_response, request_to_json},
    },
};

pub(crate) async fn post_shout_endpoint_post(
    request: Request<hyper::body::Incoming>,
    _addr: IpAddr,
    db: CommentDb,
    shout_events: Sender<Arc<ShoutEvent>>,
) -> RequestResult {
    let mut response_object = object! {};
    match *request.method() {
        Method::OPTIONS => Ok(options_response()),
        Method::POST => {
            let json = request_to_json(request).await?;
            let token: &str = extract_json_field(TOKEN_FIELD_NAME, &json)?;
            let user = db.get_user_from_token(token)?;

            let content: &str = extract_json_field(CONTENT_FIELD_NAME, &json)?;
            if content.is_empty() {
                return Err(RequestError::EmptyFieldError(
                    CONTENT_FIELD_NAME.try_into().unwrap(),
                ));
            }

            let shout_result = match db.add_shout(user.get_id(), content) {
                Ok(_) => Ok(json_to_response(response_object, StatusCode::OK)),
                Err(_err) => {
                    response_object["error"] = "Error posting shout".into();
                    Ok(json_to_response(response_object, StatusCode::BAD_REQUEST))
                }
            };
            if shout_result.is_ok() {
                shout_events
                    .broadcast(Arc::new(ShoutEvent {
                        display_name: user.get_display_name().to_string(),
                        content: ammonia::clean(content),
                        user_color: user.get_color().to_string(),
                        user_id: user.get_id(),
                    }))
                    .await
                    .unwrap();
            }
            shout_result
        }
        _ => Err(RequestError::InvalidMethod),
    }
}

pub(crate) async fn edit_shout_endpoint_post(
    request: Request<hyper::body::Incoming>,
    _addr: IpAddr,
    db: CommentDb,
) -> RequestResult {
    match *request.method() {
        Method::OPTIONS => Ok(options_response()),
        Method::POST => {
            let json = request_to_json(request).await?;
            let token: &str = extract_json_field(TOKEN_FIELD_NAME, &json)?;
            let user = db.get_user_from_token(token)?;
            let content: &str = extract_json_field(CONTENT_FIELD_NAME, &json)?;
            if content.is_empty() {
                return Err(RequestError::EmptyFieldError(
                    CONTENT_FIELD_NAME.try_into().unwrap(),
                ));
            }
            let shout_id: i64 = extract_json_field(SHOUT_ID_FIELD_NAME, &json)?;

            let shout = db.get_shout_from_id(shout_id)?;

            if shout.get_user_id() != user.get_id() {
                return Err(RequestError::NotAllowed);
            }

            db.edit_shout(shout.get_id(), content)?;

            Ok(json_to_response(object! {}, StatusCode::OK))
        }
        _ => Err(RequestError::InvalidMethod),
    }
}

pub(crate) async fn delete_shout_endpoint_post(
    request: Request<hyper::body::Incoming>,
    _addr: IpAddr,
    db: CommentDb,
) -> RequestResult {
    match *request.method() {
        Method::OPTIONS => Ok(options_response()),
        Method::POST => {
            let json = request_to_json(request).await?;
            let token: &str = extract_json_field(TOKEN_FIELD_NAME, &json)?;
            let user = db.get_user_from_token(token)?;

            let shout_id: i64 = extract_json_field(SHOUT_ID_FIELD_NAME, &json)?;

            let shout = db.get_shout_from_id(shout_id)?;

            if shout.get_user_id() != user.get_id() {
                return Err(RequestError::NotAllowed);
            }

            db.delete_shout(shout.get_id())?;
            Ok(json_to_response(object! {}, StatusCode::OK))
        }
        _ => Err(RequestError::InvalidMethod),
    }
}

//TODO this just gets all the shouts for now
///Returns the 10 most recent comments. `shouts_before` can be specified to get the 10 most recent
///comments before the specified date
pub(crate) async fn get_shouts_endpoint_post(
    request: Request<hyper::body::Incoming>,
    _addr: IpAddr,
    db: CommentDb,
) -> RequestResult {
    let mut response_object = object! {};
    match *request.method() {
        Method::OPTIONS => Ok(options_response()),
        Method::POST => {
            let json = request_to_json(request).await.ok();
            let _shouts_before_id = json
                .as_ref()
                .and_then(|json| json["shouts_before_id"].as_i64());
            let user_opt = json.as_ref().and_then(|json| {
                let token = json["token"].as_str()?;
                db.get_user_from_token(token).ok()
            });

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
                shout_json["editable"] = user_opt
                    .as_ref()
                    .is_some_and(|user| user.get_id() == shout.get_user_id())
                    .into();
                shouts_vec.push(shout_json);
            }

            response_object["shouts"] = json::JsonValue::Array(shouts_vec);

            Ok(json_to_response(response_object, StatusCode::OK))
        }
        _ => Err(RequestError::InvalidMethod),
    }
}

pub(crate) async fn subscribe_shouts_endpoint_get(
    request: Request<hyper::body::Incoming>,
    _addr: IpAddr,
    db: CommentDb,
    shout_events: Sender<Arc<ShoutEvent>>,
) -> Result<Response<BoxBody<Bytes, Infallible>>, RequestError> {
    match *request.method() {
        Method::OPTIONS => Ok(options_response()),
        Method::GET => {
            let token = request
                .uri()
                .query()
                .and_then(|query| extract_key_from_query(query, "token"));
            let user_opt = token
                .as_ref()
                .and_then(|token| db.get_user_from_token(token).ok());

            let mut rx = shout_events.new_receiver();

            let stream = async_stream::stream! {
                loop {
                    futures::select! {
                        msg = rx.recv().fuse() => {
                            match msg {
                                Ok(shout_event) => {
                                    let json = object!{
                                        user_color: shout_event.user_color.as_str(),
                                        display_name: shout_event.display_name.as_str(),
                                        content: shout_event.content.as_str(),
                                        editable: user_opt.as_ref().is_some_and(|user|user.get_id()==shout_event.user_id)
                                    };

                                    yield Ok::<Frame<Bytes>, Infallible>(
                                        Frame::data(Bytes::from(format!("data: {json}\n\n")))
                                    );
                                }
                                Err(async_broadcast::RecvError::Overflowed(_)) => {
                                    continue;
                                }
                                Err(async_broadcast::RecvError::Closed) => {
                                    break;
                                }
                            }
                        }

                        _ = Timer::after(Duration::from_secs(15)).fuse() => {
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
        _ => Err(RequestError::InvalidMethod),
    }
}
