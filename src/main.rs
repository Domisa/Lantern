
use axum::{Router, routing::{get, delete}, extract::{State, Path}, Json};
use tokio::net::TcpListener;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use axum::response::IntoResponse;

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
) -> impl IntoResponse {
    let tasks = sqlx::query_as!(Task, "SELECT * FROM tasks")
        .fetch_all(&pool)
        .await;

    match tasks {
        Ok(tasks) => Json(tasks).into_response(),
        Err(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response(),
    }
}

async fn create_task(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateTask>,
) -> impl IntoResponse {
    let result: Result<_, sqlx::Error> = sqlx::query!(
        "INSERT INTO tasks (date, task, summary) VALUES ($1, $2, $3)",
        payload.date,
        payload.task,
        payload.summary,
    )
    .execute(&pool)
    .await;

    match result {
        Ok(_) => axum::http::StatusCode::CREATED.into_response(),
        Err(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response(),
    }

}

async fn delete_task(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let result: Result<_, sqlx::Error> = sqlx::query!("DELETE FROM tasks WHERE id = $1", id)
        .execute(&pool)
        .await;

    match result {
        Ok(_) => axum::http::StatusCode::OK.into_response(),
        Err(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response(),
    }

}

async fn update_task(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(payload): Json<CreateTask>
) -> impl IntoResponse {
    let result: Result<_, sqlx::Error> = sqlx::query!(
        "UPDATE tasks SET date = $1, task = $2, summary = $3 WHERE id = $4", 
        payload.date,
        payload.task,
        payload.summary,
        id
    )
        .execute(&pool)
        .await;

    match result {
        Ok(_) => axum::http::StatusCode::OK.into_response(),
        Err(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response(),
    }
}

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
