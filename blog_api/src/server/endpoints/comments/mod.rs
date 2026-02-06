use std::net::IpAddr;

use bytes::Bytes;
use http_body_util::Full;
use hyper::{Method, Request, Response, StatusCode};
use json::object;

use crate::{
    db::CommentDb,
    server::{
        RequestError, RequestResult,
        util::{extract_key_from_query, json_to_response, options_response, request_to_json},
    },
};
//function names: path+'endpoint'+method

pub(crate) async fn post_comment_endpoint_post(
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
                && let Ok(user) = db.get_user_from_token(token)
            {
                let Some(post_ident) = json["post"].as_str() else {
                    response_object["error"] = "Missing post identifier".into();
                    return Ok(json_to_response(response_object, StatusCode::BAD_REQUEST));
                };
                if post_ident.is_empty() {
                    response_object["error"] = "Missing post identifier".into();
                    return Ok(json_to_response(response_object, StatusCode::BAD_REQUEST));
                }
                let Some(content) = json["content"].as_str() else {
                    response_object["error"] = "Missing comment content".into();
                    return Ok(json_to_response(response_object, StatusCode::BAD_REQUEST));
                };
                if content.is_empty() {
                    response_object["error"] = "Empty comment".into();
                    return Ok(json_to_response(response_object, StatusCode::BAD_REQUEST));
                }

                let post = match db.get_post_with_ident(post_ident) {
                    Ok(post) => post,
                    Err(_err) => {
                        response_object["error"] = "Invalid post indentifier".into();
                        return Ok(json_to_response(response_object, StatusCode::BAD_REQUEST));
                    }
                };

                match db.add_comment(post.get_id(), user.get_id(), content) {
                    Ok(_) => Ok(json_to_response(response_object, StatusCode::OK)),
                    Err(_err) => {
                        response_object["error"] = "Error posting comment".into();
                        Ok(json_to_response(response_object, StatusCode::BAD_REQUEST))
                    }
                }
            } else {
                response_object["error"] = "Invalid user token".into();
                Ok(json_to_response(response_object, StatusCode::BAD_REQUEST))
            }
        }
        _ => {
            eprintln!("IP: {addr} Invalid Method on Comment Post endpoint");
            Ok(json_to_response(
                response_object,
                StatusCode::METHOD_NOT_ALLOWED,
            ))
        }
    }
}

pub(crate) async fn delete_comment_endpoint_post(
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
                && let Ok(user) = db.get_user_from_token(token)
            {
                let Some(comment_id) = json["comment_id"].as_i64() else {
                    eprintln!("IP: {addr}: Missing comment id");
                    response_object["error"] = "Missing comment id".into();
                    return Ok(json_to_response(response_object, StatusCode::BAD_REQUEST));
                };

                let comment = match db.get_comment_from_id(comment_id) {
                    Ok(comment) => comment,
                    Err(err) => {
                        eprintln!("IP: {addr}: Invalid comment id: {err}");
                        response_object["error"] = "Error: Invalid comment id".into();
                        return Ok(json_to_response(response_object, StatusCode::BAD_REQUEST));
                    }
                };

                if comment.get_user_id() != user.get_id() {
                    eprintln!("IP: {addr}: Not the comment creator");
                    response_object["error"] = "Not the comment creator".into();
                    return Ok(json_to_response(response_object, StatusCode::FORBIDDEN));
                }

                match db.delete_comment(comment.get_id()) {
                    Ok(_) => Ok(json_to_response(response_object, StatusCode::OK)),
                    Err(err) => {
                        eprintln!("IP: {addr}: error deleting comment: {err}");
                        response_object["error"] = "Error deleting comment".into();
                        Ok(json_to_response(response_object, StatusCode::BAD_REQUEST))
                    }
                }
            } else {
                response_object["error"] = "Invalid user token".into();
                Ok(json_to_response(response_object, StatusCode::BAD_REQUEST))
            }
        }
        _ => {
            eprintln!("IP: {addr} Invalid Method on Comment Post endpoint");
            Ok(json_to_response(
                response_object,
                StatusCode::METHOD_NOT_ALLOWED,
            ))
        }
    }
}

pub(crate) async fn edit_comment_endpoint_post(
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
                && let Ok(user) = db.get_user_from_token(token)
            {
                let Some(comment_id) = json["comment_id"].as_i64() else {
                    response_object["error"] = "Missing comment id".into();
                    return Ok(json_to_response(response_object, StatusCode::BAD_REQUEST));
                };

                let Some(content) = json["content"].as_str() else {
                    response_object["error"] = "Missing comment content".into();
                    return Ok(json_to_response(response_object, StatusCode::BAD_REQUEST));
                };

                if content.is_empty() {
                    response_object["error"] = "Empty comment".into();
                    return Ok(json_to_response(response_object, StatusCode::BAD_REQUEST));
                }

                let comment = match db.get_comment_from_id(comment_id) {
                    Ok(comment) => comment,
                    Err(_err) => {
                        response_object["error"] = "Invalid comment id".into();
                        return Ok(json_to_response(response_object, StatusCode::BAD_REQUEST));
                    }
                };

                if comment.get_user_id() != user.get_id() {
                    response_object["error"] = "Not the comment owner".into();
                    return Ok(json_to_response(response_object, StatusCode::FORBIDDEN));
                }

                match db.edit_comment(comment.get_id(), content) {
                    Ok(_) => Ok(json_to_response(response_object, StatusCode::OK)),
                    Err(_err) => {
                        response_object["error"] = "Error editing comment".into();
                        Ok(json_to_response(response_object, StatusCode::BAD_REQUEST))
                    }
                }
            } else {
                todo!();
            }
        }
        _ => {
            eprintln!("IP: {addr} Invalid Method on Comment Post endpoint");
            Ok(json_to_response(
                response_object,
                StatusCode::METHOD_NOT_ALLOWED,
            ))
        }
    }
}

pub(crate) async fn get_comments_endpoint_post(
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
            let json::JsonValue::Array(posts) = &json["post_idents"] else {
                response_object["error"] = "Missing 'posts' field".into();
                return Ok(json_to_response(response_object, StatusCode::BAD_REQUEST));
            };

            if let Some(token) = json["token"].as_str()
                && let Ok(user) = db.get_user_from_token(token)
            {
                let mut posts_response = Vec::with_capacity(posts.len());
                let post_idents_iter = posts.iter().flat_map(|json_value| json_value.as_str());
                for post_ident in post_idents_iter {
                    let post = match db.get_post_with_ident(&post_ident) {
                        Ok(post) => post,
                        Err(_err) => {
                            response_object["error"] = "could not fetch comments".into();
                            return Ok(json_to_response(response_object, StatusCode::BAD_REQUEST));
                        }
                    };

                    let comments = match db.get_post_comments(post.get_id()) {
                        Ok(comments) => comments,
                        Err(_err) => {
                            response_object["error"] = "could not fetch comments".into();
                            return Ok(json_to_response(response_object, StatusCode::BAD_REQUEST));
                        }
                    };
                    let star_count = db.get_post_star_count(post.get_id()).unwrap_or_default();

                    let mut post_json = object! {comments:[],stars:0};

                    for comment in comments {
                        let mut comment_json = object! {};
                        comment_json["id"] = comment.get_id().into();
                        comment_json["username"] = todo!();
                        comment_json["content"] = comment.get_content().into();
                        comment_json["editable"] = (comment.get_user_id() == user.get_id()).into();
                        comment_json["created"] = comment.get_datetime().to_string().into();
                        comment_json["edited"] = comment.was_edited().into();
                        let _ = post_json["comments"].push(comment_json);
                    }

                    post_json["stars"] = star_count.into();

                    post_json["starrable"] =
                        match db.is_post_starred_by(post.get_id(), user.get_id()) {
                            Ok(starable) => !starable,
                            Err(_err) => true,
                        }
                        .into();
                    posts_response.push(post_json);
                }
                Ok(json_to_response(response_object, StatusCode::OK))
            } else {
                todo!();
            }
        }
        _ => {
            eprintln!("IP: {addr} Invalid Method on Comments endpoint");
            Ok(json_to_response(
                response_object,
                StatusCode::METHOD_NOT_ALLOWED,
            ))
        }
    }
}
