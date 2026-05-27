
use axum::{Router, routing::{get, delete}, extract::{State, Path}, Json};
use tokio::net::TcpListener;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, Deserialize)]
struct Task {
    id: i32,
    date: String,
    task: String,
    summary: String,
}

#[derive(Deserialize)]
struct CreateTask {
    date: String,
    task: String,
    summary: String,
}

async fn get_tasks(
    State(pool): State<PgPool>,
) -> impl axum::response::IntoResponse {
    let tasks = sqlx::query_as!(Task, "SELECT * FROM tasks")
        .fetch_all(&pool)
        .await
        .unwrap();

    Json(tasks)
}

async fn create_task(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateTask>,
) -> impl axum::response::IntoResponse {
    sqlx::query!(
        "INSERT INTO tasks (date, task, summary) VALUES ($1, $2, $3)",
        payload.date,
        payload.task,
        payload.summary,
    )
    .execute(&pool)
    .await
    .unwrap();

    axum::http::StatusCode::CREATED
}

async fn delete_task(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> impl axum::response::IntoResponse {
    sqlx::query!("DELETE FROM tasks WHERE id = $1", id)
        .execute(&pool)
        .await
        .unwrap();

    axum::http::StatusCode::OK

}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let pool = PgPool::connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let app = Router::new()
        .route("/tasks", get(get_tasks).post(create_task))
        .route("/tasks/{id}", delete(delete_task))
        .with_state(pool);

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("listening on port 8080");
    axum::serve(listener, app).await.unwrap();
}
