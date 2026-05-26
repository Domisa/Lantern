
use axum::{Router, routing::get, extract::State, Json};
use tokio::net::TcpListener;
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
struct TaskPath {
    id: i32,
}

async fn get_tasks(
    State(_pool): State<PgPool>,
) -> impl axum::response::IntoResponse {

}

#[tokio::main]
async fn main() {

    let pool = PgPool::connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let app = Router::new()
        .route("/tasks", get(get_tasks))
        .with_state(pool);

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("listening on port 8080");
    axum::serve(listener, app).await.unwrap();
}