use axum::http::StatusCode;
use axum::Json;
use log::debug;
use serde::Serialize;
use jwt::jwt::{get_jwt, User};
use jwt::response::JsonResponse;

#[derive(Serialize)]
pub struct Token {
    token: String,
}

pub async fn token_handler(Json(user): Json<User>) -> (StatusCode, Json<JsonResponse<Token>>) {
    match get_jwt(&user) {
        Ok(token) => {
            let response = JsonResponse::new(true, Token { token });
            (StatusCode::OK, Json(response))
        }
        Err(error) => {
            debug!("{}", error);
            let response = JsonResponse::new(false, Token { token: String::new() });
            (StatusCode::BAD_REQUEST, Json(response))
        }
    }
}
