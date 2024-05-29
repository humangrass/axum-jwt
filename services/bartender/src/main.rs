mod api;

use std::process;
use axum::{routing::post, Router};
use tokio::net::TcpListener;
use jwt::jwt::check_jwt_secret;
use logger::log_level::LogLevel;
use logger::logger::new_tracer_logger;
use crate::api::token::token_handler;

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    println!("Bartender service started!");
    check_jwt_secret();

    if let Err(err) = run().await {
        eprintln!("Fatal error occurred: {}", err);
        process::exit(1);
    }
}

async fn run() -> anyhow::Result<()> {
    new_tracer_logger(LogLevel::Info);

    let routes = Router::new()
        .route("/token", post(token_handler));

    let tcp_listener = TcpListener::bind("localhost:3000")
        .await?;
    axum::serve(tcp_listener, routes)
        .await?;

    Ok(())
}
