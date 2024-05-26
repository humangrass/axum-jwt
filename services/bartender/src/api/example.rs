use axum::http::StatusCode;
use serde_json::json;
use axum::Json;
use jwt::auth::Auth;
use jwt::jwt::User;
use jwt::response::JsonResponse;

pub async fn public_handler() -> (StatusCode, Json<JsonResponse<serde_json::Value>>) {
    let response = JsonResponse::new(true, json!({
        "message": "This data is visible to all users"
    }));
    (StatusCode::OK, Json(response))
}

pub async fn secret_handler(Auth(user): Auth) -> (StatusCode, Json<JsonResponse<User>>) {
    let response = JsonResponse::new(true, user);
    (StatusCode::OK, Json(response))
}
