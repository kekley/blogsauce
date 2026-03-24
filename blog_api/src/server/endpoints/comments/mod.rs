use crate::json::JsonFieldError;
use crate::server::RequestError;
use crate::server::endpoints::COMMENT_ID_FIELD_NAME;
use crate::server::endpoints::CONTENT_FIELD_NAME;
use crate::server::endpoints::POST_GET_IDENTS_FIELD_NAME;
use crate::server::endpoints::POST_IDENT_FIELD_NAME;
use crate::server::endpoints::TOKEN_FIELD_NAME;
use std::net::IpAddr;

use hyper::{Method, Request, StatusCode};
use json::object;

use crate::{
    db::{CommentDb, DbError},
    json::extract_json_field,
    server::{
        RequestResult,
        util::{json_to_response, options_response, request_to_json},
    },
};

//function names: path+'endpoint'+method

pub(crate) async fn post_comment_endpoint_post(
    request: Request<hyper::body::Incoming>,
    _addr: IpAddr,
    db: CommentDb,
) -> RequestResult {
    match *request.method() {
        Method::OPTIONS => Ok(options_response()),
        Method::POST => {
            let json = request_to_json(request).await?;
            println!("got json!");

            let token: &str = extract_json_field(TOKEN_FIELD_NAME, &json)?;

            println!("got token!");
            let user = db.get_user_from_token(token)?;
            println!("got user!");

            let post_ident: &str = extract_json_field(POST_IDENT_FIELD_NAME, &json)?;
            println!("got post ident!");

            if post_ident.is_empty() {
                return Err(crate::server::RequestError::EmptyFieldError(
                    POST_IDENT_FIELD_NAME.try_into().unwrap(),
                ));
            }
            println!("post ident not empty!");

            let content: &str = extract_json_field(CONTENT_FIELD_NAME, &json)?;
            println!("got content!");

            if content.is_empty() {
                return Err(crate::server::RequestError::EmptyFieldError(
                    CONTENT_FIELD_NAME.try_into().unwrap(),
                ));
            }
            println!("content not empty!");

            let post = match db.get_post_with_ident(post_ident) {
                Ok(post) => post,
                Err(err) => {
                    if let DbError::NoResults = err {
                        return Err(crate::server::RequestError::InvalidPost);
                    }
                    return Err(err.into());
                }
            };
            println!("got post");

            db.add_comment(post.get_id(), user.get_id(), content)?;
            println!("made comment");

            Ok(json_to_response(object! {}, StatusCode::OK))
        }
        _ => Err(crate::server::RequestError::InvalidMethod),
    }
}

pub(crate) async fn delete_comment_endpoint_post(
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

            let comment_id: i64 = extract_json_field(COMMENT_ID_FIELD_NAME, &json)?;
            let comment = db.get_comment_from_id(comment_id)?;

            if comment.get_user_id() != user.get_id() {
                return Err(crate::server::RequestError::NotAllowed);
            }

            db.delete_comment(comment.get_id())?;

            Ok(json_to_response(object! {}, StatusCode::OK))
        }
        _ => Err(crate::server::RequestError::InvalidMethod),
    }
}

pub(crate) async fn edit_comment_endpoint_post(
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

            let comment_id = extract_json_field(COMMENT_ID_FIELD_NAME, &json)?;

            let content: &str = extract_json_field(CONTENT_FIELD_NAME, &json)?;

            if content.is_empty() {
                return Err(crate::server::RequestError::EmptyFieldError(
                    COMMENT_ID_FIELD_NAME.try_into().unwrap(),
                ));
            }

            let comment = db.get_comment_from_id(comment_id)?;

            if comment.get_user_id() != user.get_id() {
                return Err(crate::server::RequestError::NotAllowed);
            }

            db.edit_comment(comment.get_id(), content)?;
            Ok(json_to_response(object! {}, StatusCode::OK))
        }
        _ => Err(crate::server::RequestError::InvalidMethod),
    }
}

pub(crate) async fn get_comments_endpoint_post(
    request: Request<hyper::body::Incoming>,
    _addr: IpAddr,
    db: CommentDb,
) -> RequestResult {
    let mut response_object = object! {};

    match *request.method() {
        Method::OPTIONS => Ok(options_response()),
        Method::POST => {
            let json = request_to_json(request).await?;

            let json::JsonValue::Array(posts) = &json[POST_GET_IDENTS_FIELD_NAME] else {
                return Err(RequestError::JsonFieldError(JsonFieldError::MissingField(
                    POST_GET_IDENTS_FIELD_NAME.try_into().unwrap(),
                )));
            };

            let token = extract_json_field::<&str>(TOKEN_FIELD_NAME, &json).ok();
            let user = token.and_then(|d| db.get_user_from_token(d).ok());

            //TODO Replace this with joins
            let mut posts_response = Vec::with_capacity(posts.len());
            let post_idents_iter = posts.iter().flat_map(|json_value| json_value.as_str());
            for post_ident in post_idents_iter {
                let post = match db.get_post_with_ident(post_ident) {
                    Ok(post) => post,
                    Err(_err) => {
                        continue;
                    }
                };

                let comments = match db.get_post_comments(post.get_id()) {
                    Ok(comments) => comments,
                    Err(_err) => {
                        continue;
                    }
                };
                let star_count = db.get_post_star_count(post.get_id()).unwrap_or_default();

                let mut post_json = object! {post_ident:post.get_ident(),comments:[],stars:0};

                for comment in comments {
                    let mut comment_json = object! {};
                    comment_json["id"] = comment.get_comment_id().into();
                    comment_json["content"] = comment.get_content().into();
                    comment_json["display_name"] = comment.get_user_display_name().into();
                    comment_json["user_color"] = comment.get_user_color().to_string().into();
                    comment_json["editable"] = user
                        .as_ref()
                        .is_some_and(|user| comment.get_user_id() == user.get_id())
                        .into();
                    comment_json["created"] = comment.updated_on().to_string().into();
                    comment_json["edited"] = comment.was_edited().into();
                    let _ = post_json["comments"].push(comment_json);
                }

                post_json["stars"] = star_count.into();

                post_json["starrable"] = user
                    .as_ref()
                    .is_some_and(
                        |user| match db.is_post_starred_by(post.get_id(), user.get_id()) {
                            Ok(starable) => !starable,
                            Err(_err) => true,
                        },
                    )
                    .into();

                posts_response.push(post_json);
            }
            response_object["posts"] = posts_response.into();
            Ok(json_to_response(response_object, StatusCode::OK))
        }
        _ => Err(crate::server::RequestError::InvalidMethod),
    }
}
