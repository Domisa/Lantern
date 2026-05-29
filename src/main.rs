mod routes;

use axum::{Router, routing::{get, delete}};
use tokio::net::TcpListener;
use routes::tasks::{get_tasks, create_task, delete_task, update_task};
use sqlx::PgPool;


#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let pool = PgPool::connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let app = Router::new()
        .route("/tasks", get(get_tasks).post(create_task))
        .route("/tasks/{id}", delete(delete_task).put(update_task))
        .with_state(pool);

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("listening on port 8080");
    axum::serve(listener, app).await.unwrap();
}
