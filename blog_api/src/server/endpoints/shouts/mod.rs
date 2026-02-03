use std::net::IpAddr;

use hyper::Request;

use crate::{db::CommentDb, server::util::extract_key_from_query};

pub(crate) async fn post_shout_endpoint_post(
    request: Request<hyper::body::Incoming>,
    addr: IpAddr,
    db: CommentDb,
) {
}

pub(crate) async fn edit_shout_endpoint_post(
    request: Request<hyper::body::Incoming>,
    addr: IpAddr,
    db: CommentDb,
) {
}

pub(crate) async fn delete_shout_endpoint_post(
    request: Request<hyper::body::Incoming>,
    addr: IpAddr,
    db: CommentDb,
) {
}

///Returns the 10 most recent comments. `start_comment` can be specified to get the 10 most recent
///comments after the specified comment id
pub(crate) async fn get_shouts_endpoint_get(
    request: Request<hyper::body::Incoming>,
    addr: IpAddr,
    db: CommentDb,
) {
    let starting_comment_id = request.uri().query().and_then(|query| {
        extract_key_from_query(query, "start_comment").and_then(|val| val.parse::<i32>().ok())
    });
}
