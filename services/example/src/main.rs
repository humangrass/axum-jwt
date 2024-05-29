use std::process;
use axum::{routing::get, Router, Json};
use axum::http::StatusCode;
use serde_json::json;
use tokio::net::TcpListener;
use jwt::auth::Auth;
use jwt::jwt::{check_jwt_secret, User};
use jwt::response::JsonResponse;
use logger::log_level::LogLevel;
use logger::logger::new_tracer_logger;


#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    println!("Example service started!");
    check_jwt_secret();

    if let Err(err) = run().await {
        eprintln!("Fatal error occurred: {}", err);
        process::exit(1);
    }
}

async fn run() -> anyhow::Result<()> {
    new_tracer_logger(LogLevel::Info);

    let routes = Router::new()
        .route("/public", get(public_handler))
        .route("/secret", get(secret_handler));

    let tcp_listener = TcpListener::bind("localhost:3001")
        .await?;
    axum::serve(tcp_listener, routes)
        .await?;

    Ok(())
}

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
