use std::env;

use axum::Router;
use axum::routing::{get, post};
use dotenv::dotenv;
use serde::Deserialize;
use tokio::net::TcpListener;

mod routes;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    dotenv().ok();

    let router = Router::new()
        .route("/", get(|| async { "Hello world" }))
        .route("/v1/chat/completions", post(routes::completions::handler));

    let listener = TcpListener::bind(
        env::var("SERVER_ADDRESS").unwrap_or("0.0.0.0:3000".to_string())
    ).await.unwrap();

    axum::serve(listener, router).await.unwrap();
}