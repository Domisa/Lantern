
use axum::{Router, routing::get};
use tokio::net::TcpListener;
// Taking a small break to better learn axum before continuing.
#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async {" Lantern is running"}));

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("listening on port 8080");
    axum::serve(listener, app).await.unwrap();
}