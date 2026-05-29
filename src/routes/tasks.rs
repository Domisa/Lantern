use axum::{extract::{State, Path}, Json, response::IntoResponse};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use axum::http::StatusCode;

#[derive(Serialize, Deserialize)]
pub struct Task {
    id: i32,
    date: String,
    task: String,
    summary: String,
}

#[derive(Deserialize)]
pub struct CreateTask {
    date: String,
    task: String,
    summary: String,
}

pub async fn get_tasks(
    State(pool): State<PgPool>,
) -> impl IntoResponse {
    let tasks = sqlx::query_as!(Task, "SELECT * FROM tasks")
        .fetch_all(&pool)
        .await;

    match tasks {
        Ok(tasks) => Json(tasks).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response(),
    }
}

pub async fn create_task(
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
        Ok(_) => StatusCode::CREATED.into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response(),
    }

}

pub async fn delete_task(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let result: Result<_, sqlx::Error> = sqlx::query!("DELETE FROM tasks WHERE id = $1", id)
        .execute(&pool)
        .await;

    match result {
        Ok(_) => StatusCode::OK.into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response(),
    }

}

pub async fn update_task(
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
        Ok(_) => StatusCode::OK.into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response(),
    }
}