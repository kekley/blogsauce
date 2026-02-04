pub mod endpoints;
pub mod util;

use crate::db::CommentDb;
use crate::server::endpoints::comments::delete_comment_endpoint_post;
use crate::server::endpoints::comments::edit_comment_endpoint_post;
use crate::server::endpoints::comments::get_comments_endpoint_post;
use crate::server::endpoints::comments::post_comment_endpoint_post;
use crate::server::endpoints::shouts::delete_shout_endpoint_post;
use crate::server::endpoints::shouts::edit_shout_endpoint_post;
use crate::server::endpoints::shouts::get_shouts_endpoint_get;
use crate::server::endpoints::shouts::new_shouts_endpoint_get;
use crate::server::endpoints::shouts::post_shout_endpoint_post;
use crate::server::endpoints::splashes::get_splash_text_endpoint_get;
use crate::server::endpoints::stars::post_star_endpoint_post;
use crate::server::endpoints::user::change_color_endpoint_post;
use crate::server::endpoints::user::get_user_endpoint_post;
use crate::server::endpoints::user::verify_token_endpoint_get;
use bytes::Bytes;
use http_body_util::Full;
use hyper::Response;
use hyper::StatusCode;
use hyper::header::HeaderValue;
use std::net::IpAddr;
use thiserror::Error;

use hyper::Request;

#[derive(Debug, Error)]
pub enum RequestError {
    #[error("{0}")]
    HttpError(#[from] hyper::http::Error),
    #[error("{0}")]
    JsonError(#[from] json::Error),
    #[error("{0}")]
    BodyError(#[from] hyper::Error),
    #[error("{0}")]
    Utf8Error(#[from] std::str::Utf8Error),
    #[error("{0}")]
    MissingField(String),
    #[error("Internal Error: {0:?}")]
    InternalError(InternalErrorKind),
}

#[derive(Debug)]
pub enum InternalErrorKind {
    GetUser,
    GetComments,
    PostComment,
    DeleteComment,
    EditComment,
    GetShouts,
    PostShout,
    DeleteShout,
    EditShout,
    StarPost,
}

pub async fn handle_request(
    request: Request<hyper::body::Incoming>,
    addr: IpAddr,
    db: CommentDb,
) -> Result<Response<Full<Bytes>>, RequestError> {
    match request.uri().path() {
        "/getSplash" => get_splash_text_endpoint_get(request, addr).await,
        "/getComments" => get_comments_endpoint_post(request, addr, db).await,
        "/registerName" => get_user_endpoint_post(request, addr, db).await,
        "/changeColor" => change_color_endpoint_post(request, addr, db).await,
        "/verifyToken" => verify_token_endpoint_get(request, addr, db).await,
        "/star" => post_star_endpoint_post(request, addr, db).await,
        "/editComment" => edit_comment_endpoint_post(request, addr, db).await,
        "/deleteComment" => delete_comment_endpoint_post(request, addr, db).await,
        "/postComment" => post_comment_endpoint_post(request, addr, db).await,
        "/newShouts" => new_shouts_endpoint_get(request, addr, db).await,
        "/getShouts" => get_shouts_endpoint_get(request, addr, db).await,
        "/postShout" => post_shout_endpoint_post(request, addr, db).await,
        "/editShout" => edit_shout_endpoint_post(request, addr, db).await,
        "/deleteShout" => delete_shout_endpoint_post(request, addr, db).await,
        _ => {
            eprintln!("IP: {} Invalid Endpoint", addr);
            Ok(Response::builder()
                .header("Access-Control-Allow-Origin", HeaderValue::from_static("*"))
                .status(StatusCode::NOT_FOUND)
                .body(Full::new(Bytes::from_static(b"Invalid endpoint :(")))
                .expect(""))
        }
    }
}
