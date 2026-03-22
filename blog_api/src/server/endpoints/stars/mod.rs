use std::net::IpAddr;

use hyper::{Method, Request, StatusCode};
use json::object;

use crate::{
    db::CommentDb,
    json::extract_json_field,
    server::{
        RequestError, RequestResult,
        endpoints::{POST_IDENT_FIELD_NAME, TOKEN_FIELD_NAME},
        util::{json_to_response, options_response, request_to_json},
    },
};

pub(crate) async fn post_star_endpoint_post(
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

            let post_ident: &str = extract_json_field(POST_IDENT_FIELD_NAME, &json)?;

            if post_ident.is_empty() {
                return Err(RequestError::EmptyFieldError(
                    POST_IDENT_FIELD_NAME.try_into().unwrap(),
                ));
            }

            let post = db.get_post_with_ident(post_ident)?;

            db.star_post(post.get_id(), user.get_id())?;

            Ok(json_to_response(object! {}, StatusCode::OK))
        }
        _ => Err(RequestError::InvalidMethod),
    }
}
