use crate::repositories::{CreateTodo, TodoRepository};
use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use std::sync::Arc;

pub async fn create_todo<T: TodoRepository>(
    Extension(repository): Extension<Arc<T>>,
    Json(payload): Json<CreateTodo>,
) -> impl IntoResponse {
    let todo = repository.create(payload);

    (StatusCode::CREATED, Json(todo))
}
