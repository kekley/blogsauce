use std::{convert::Infallible, net::IpAddr, sync::Arc, time::Duration};

use async_broadcast::Sender;
use async_stream_lite::async_stream;
use bytes::Bytes;
use http_body_util::{BodyExt, StreamBody, combinators::BoxBody};
use hyper::{
    Method, Request, Response, StatusCode,
    body::Frame,
    header::{CACHE_CONTROL, CONNECTION, CONTENT_TYPE, HeaderValue},
};
use json::object;
use smol::Timer;

use crate::{
    db::sqlite::CommentDb,
    json::extract_json_field,
    models::joins::shouts::JoinedShout,
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
    shout_events: Sender<Arc<JoinedShout>>,
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
                return Err(RequestError::EmptyField(
                    CONTENT_FIELD_NAME.try_into().unwrap(),
                ));
            }

            match db.add_shout(user.get_id(), content) {
                Ok(shout_id) => {
                    if let Ok(shout) = db.get_joined_shout_by_id(shout_id) {
                        let _ = shout_events.broadcast(Arc::new(shout)).await;
                    }
                    Ok(json_to_response(response_object, StatusCode::OK))
                }
                Err(_err) => {
                    response_object["error"] = "Error posting shout".into();
                    Ok(json_to_response(response_object, StatusCode::BAD_REQUEST))
                }
            }
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
                return Err(RequestError::EmptyField(
                    CONTENT_FIELD_NAME.try_into().unwrap(),
                ));
            }
            let shout_id: i64 = extract_json_field(SHOUT_ID_FIELD_NAME, &json)?;

            let shout = db.get_shout_by_id(shout_id)?;

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

            let shout = db.get_shout_by_id(shout_id)?;

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
                let mut shout_json = shout.to_json();
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

enum Event<T> {
    Msg(Result<T, async_broadcast::RecvError>),
    KeepAlive,
}

pub(crate) async fn subscribe_shouts_endpoint_get(
    request: Request<hyper::body::Incoming>,
    _addr: IpAddr,
    db: CommentDb,
    shout_events: Sender<Arc<JoinedShout>>,
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
            let stream = async_stream(|yielder| async move {
                loop {
                    let event = smol::future::or(async { Event::Msg(rx.recv().await) }, async {
                        Timer::after(Duration::from_secs(15)).await;
                        Event::KeepAlive
                    })
                    .await;
                    match event {
                        Event::Msg(Ok(shout)) => {
                            let mut json = shout.to_json();
                            json["editable"] = user_opt
                                .as_ref()
                                .is_some_and(|user| user.get_id() == shout.get_user_id())
                                .into();

                            yielder
                                .r#yield(Ok::<Frame<Bytes>, Infallible>(Frame::data(Bytes::from(
                                    format!("data: {json}\n\n"),
                                ))))
                                .await;
                        }

                        Event::Msg(Err(async_broadcast::RecvError::Overflowed(_))) => {
                            continue;
                        }

                        Event::Msg(Err(async_broadcast::RecvError::Closed)) => {
                            break;
                        }

                        Event::KeepAlive => {
                            yielder
                                .r#yield(Ok::<Frame<Bytes>, Infallible>(Frame::data(
                                    Bytes::from_static(b": keep-alive\n\n"),
                                )))
                                .await;
                        }
                    }
                }
            });

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
