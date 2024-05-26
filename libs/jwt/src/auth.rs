use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, header, StatusCode},
    response::Response,
};
use crate::jwt::{decode_jwt, User};
use crate::response::JsonResponse;
use serde_json::json;

pub struct Auth(pub User);

#[async_trait]
impl<S> FromRequestParts<S> for Auth
    where S: Send + Sync,
{
    type Rejection = Response<String>;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let token = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .and_then(|auth_header| auth_header.split_whitespace().nth(1));

        if let Some(token) = token {
            match decode_jwt(token) {
                Ok(user) => Ok(Auth(user)),
                Err(_) => Err(unauthorized_response("Invalid token")),
            }
        } else {
            Err(unauthorized_response("No token provided"))
        }
    }
}

fn unauthorized_response(message: &str) -> Response<String> {
    let response = JsonResponse::new(false, json!({ "message": message }));
    Response::builder()
        .status(StatusCode::UNAUTHORIZED)
        .header(header::CONTENT_TYPE, "application/json")
        .body(serde_json::to_string(&response).unwrap_or_else(|_| "Internal Server Error".into()))
        .unwrap_or_else(|_| Response::new("Internal Server Error".into()))
}
