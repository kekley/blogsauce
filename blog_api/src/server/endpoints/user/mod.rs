use std::{fmt::Write as _, net::IpAddr};

use hyper::{Method, Request, StatusCode};
use json::object;
use rand::{TryRng as _, rngs::SysRng};

use crate::{
    db::sqlite::CommentDb,
    json::extract_json_field,
    models::{ip::TruncatedIp, user::Color},
    server::{
        RequestError, RequestResult,
        endpoints::{COLOR_FIELD_NAME, TOKEN_FIELD_NAME},
        util::{json_to_response, options_response, request_to_json},
    },
};

const DISPLAY_NAME_FIELD: &str = "display_name";

pub(crate) async fn verify_token_endpoint_get(
    request: Request<hyper::body::Incoming>,
    _addr: IpAddr,
    db: CommentDb,
) -> RequestResult {
    let mut response_object = object! {};
    match *request.method() {
        Method::OPTIONS => Ok(options_response()),
        Method::POST => {
            let json = request_to_json(request).await?;

            let token: &str = extract_json_field(TOKEN_FIELD_NAME, &json)?;
            let user = db.get_user_from_token(token)?;

            response_object["is_valid"] = true.into();
            response_object["display_name"] = user.get_display_name().into();
            response_object["current_color"] = user.get_color().to_string().into();

            Ok(json_to_response(response_object, StatusCode::OK))
        }
        _ => Err(crate::server::RequestError::InvalidMethod),
    }
}

pub(crate) async fn change_color_endpoint_post(
    request: Request<hyper::body::Incoming>,
    _addr: IpAddr,
    db: CommentDb,
) -> RequestResult {
    match *request.method() {
        Method::OPTIONS => Ok(options_response()),
        Method::POST => {
            let json = request_to_json(request).await?;

            let token: &str = extract_json_field(TOKEN_FIELD_NAME, &json)?;
            let color: &str = extract_json_field(COLOR_FIELD_NAME, &json)?;
            let user = db.get_user_from_token(token)?;

            let color = Color::from_hex_str(color)?;

            db.change_user_color(user.get_id(), color)?;

            Ok(json_to_response(object! {}, StatusCode::OK))
        }
        _ => Err(RequestError::InvalidMethod),
    }
}

pub(crate) async fn register_name_endpoint_post(
    request: Request<hyper::body::Incoming>,
    addr: IpAddr,
    db: CommentDb,
) -> RequestResult {
    let truncated_ip = TruncatedIp::new(addr);
    let mut response_object = object! {};
    match *request.method() {
        Method::OPTIONS => Ok(options_response()),
        Method::POST => {
            let json = request_to_json(request).await?;

            let desired_name: &str = extract_json_field(DISPLAY_NAME_FIELD, &json)?;

            if desired_name.is_empty() {
                return Err(crate::server::RequestError::EmptyField(
                    DISPLAY_NAME_FIELD.try_into().unwrap(),
                ));
            }

            let mut buf = [0u8; 16];
            SysRng.try_fill_bytes(&mut buf).map_err(|err| {
                eprintln!("OS Error: {err}");
                RequestError::Internal
            })?;

            let mut token = heapless::String::<32>::new();

            for byte in buf {
                let _ = write!(&mut token, "{byte:02X}");
            }

            db.add_user(desired_name, &token, truncated_ip)?;

            response_object[TOKEN_FIELD_NAME] = token.as_str().into();
            Ok(json_to_response(response_object, StatusCode::OK))
        }
        _ => Err(RequestError::InvalidMethod),
    }
}
