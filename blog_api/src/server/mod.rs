pub mod endpoints;
pub mod util;

use crate::db::CommentDb;
use crate::db::DbError;
use crate::json::HEAPLESS_STRING_LEN;
use crate::models::shout::ShoutEvent;
use crate::models::user::ColorParseError;
use crate::server::endpoints::comments::delete_comment_endpoint_post;
use crate::server::endpoints::comments::edit_comment_endpoint_post;
use crate::server::endpoints::comments::get_comments_endpoint_post;
use crate::server::endpoints::comments::post_comment_endpoint_post;
use crate::server::endpoints::shouts::delete_shout_endpoint_post;
use crate::server::endpoints::shouts::edit_shout_endpoint_post;
use crate::server::endpoints::shouts::get_shouts_endpoint_post;
use crate::server::endpoints::shouts::post_shout_endpoint_post;
use crate::server::endpoints::shouts::subscribe_shouts_endpoint_get;
use crate::server::endpoints::splashes::get_splash_text_endpoint_get;
use crate::server::endpoints::stars::post_star_endpoint_post;
use crate::server::endpoints::user::change_color_endpoint_post;
use crate::server::endpoints::user::register_name_endpoint_post;
use crate::server::endpoints::user::verify_token_endpoint_get;
use crate::server::util::json_to_response;
use async_broadcast::Sender;
use bytes::Bytes;
use http_body_util::combinators::BoxBody;
use hyper::Response;
use hyper::StatusCode;
use json::object;
use std::convert::Infallible;
use std::net::IpAddr;
use std::sync::Arc;
use thiserror::Error;
use tracing::Level;
use tracing::event;

use hyper::Request;

pub type RequestResult = Result<Response<BoxBody<Bytes, Infallible>>, RequestError>;

#[derive(Debug, Error)]
pub enum RequestError {
    #[error("HTTP Error")]
    HttpError(#[from] hyper::http::Error),
    #[error("Error parsing request JSON")]
    JsonError(#[from] json::Error),
    #[error("Error Streaming HTTP Body")]
    BodyError(#[from] hyper::Error),
    #[error("Error Parsing a string in the request")]
    Utf8Error(#[from] std::str::Utf8Error),
    #[error("{0}")]
    JsonFieldError(#[from] crate::json::JsonFieldError),
    #[error("{0}")]
    EmptyFieldError(heapless::String<HEAPLESS_STRING_LEN>),
    #[error("USER_TAKEN")]
    UsernameTaken,
    #[error("No post matched the provided 'post' field")]
    InvalidPost,
    #[error("Internal error")]
    DbError(#[from] DbError),
    #[error("Invalid endpoint")]
    InvalidEndpoint,
    #[error("Invalid method")]
    InvalidMethod,
    #[error("Invalid token")]
    InvalidToken,
    #[error("Invalid ID")]
    InvalidId,
    #[error("Not allowed")]
    NotAllowed,

    #[error("Error parsing provided color:")]
    ColorParsingError(#[from] ColorParseError),
}

impl RequestError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::HttpError(_) => StatusCode::BAD_REQUEST,
            Self::JsonError(_) => StatusCode::BAD_REQUEST,
            Self::BodyError(_) => StatusCode::BAD_REQUEST,
            Self::Utf8Error(_) => StatusCode::BAD_REQUEST,
            Self::JsonFieldError(_) => StatusCode::BAD_REQUEST,
            Self::EmptyFieldError(_) => StatusCode::BAD_REQUEST,
            Self::UsernameTaken => StatusCode::CONFLICT,
            Self::InvalidPost => StatusCode::NOT_FOUND,
            Self::DbError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::InvalidEndpoint => StatusCode::NOT_FOUND,
            Self::InvalidMethod => StatusCode::METHOD_NOT_ALLOWED,
            Self::InvalidToken => StatusCode::UNAUTHORIZED,
            Self::InvalidId => StatusCode::BAD_REQUEST,
            Self::NotAllowed => StatusCode::FORBIDDEN,
            Self::ColorParsingError(_) => StatusCode::BAD_REQUEST,
        }
    }

    pub fn into_error_string(self) -> String {
        format!("{self}")
    }
}

pub async fn handle_request(
    request: Request<hyper::body::Incoming>,
    addr: IpAddr,
    db: CommentDb,
    shout_events: Sender<Arc<ShoutEvent>>,
) -> RequestResult {
    let path = request.uri().path().to_owned();
    let result = match request.uri().path() {
        "/getSplash" => get_splash_text_endpoint_get(request, addr).await,
        "/getComments" => get_comments_endpoint_post(request, addr, db).await,
        "/registerName" => register_name_endpoint_post(request, addr, db).await,
        "/changeColor" => change_color_endpoint_post(request, addr, db).await,
        "/verifyToken" => verify_token_endpoint_get(request, addr, db).await,
        "/star" => post_star_endpoint_post(request, addr, db).await,
        "/editComment" => edit_comment_endpoint_post(request, addr, db).await,
        "/deleteComment" => delete_comment_endpoint_post(request, addr, db).await,
        "/postComment" => post_comment_endpoint_post(request, addr, db).await,
        "/subscribeShouts" => subscribe_shouts_endpoint_get(request, addr, db, shout_events).await,
        "/getShouts" => get_shouts_endpoint_post(request, addr, db).await,
        "/postShout" => post_shout_endpoint_post(request, addr, db, shout_events).await,
        "/editShout" => edit_shout_endpoint_post(request, addr, db).await,
        "/deleteShout" => delete_shout_endpoint_post(request, addr, db).await,
        _ => Err(RequestError::InvalidEndpoint),
    };

    match result {
        Ok(r) => Ok(r),
        Err(err) => {
            event!(
                Level::ERROR,
                "Error handling {addr}'s request on endpoint {path}: {err}",
            );
            let status_code = err.status_code();
            let error_string = err.into_error_string();

            let json = object! {
                error : error_string,
            };

            Ok(json_to_response(json, status_code))
        }
    }
}
