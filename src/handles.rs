use crate::repositories::{CreateTodo, TodoRepository, UpdateTodo};
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use std::sync::Arc;

pub async fn create_todo<T: TodoRepository>(
    Extension(repository): Extension<Arc<T>>,
    Json(payload): Json<CreateTodo>,
) -> impl IntoResponse {
    match payload.text.len() {
        // returnを使う理由とinto_response()について調べる
        len if len <= 0 => return (StatusCode::BAD_REQUEST, "Error!: Can not be Empty".to_string()).into_response(),
        len if len > 100 => return (StatusCode::BAD_REQUEST, "Error!: Over text length".to_string()).into_response(),
        _ => {}
    }
    let todo = repository.create(payload);
    (StatusCode::CREATED, Json(todo)).into_response()
}

pub async fn find_todo<T: TodoRepository>(
    Path(id): Path<i32>,
    Extension(repository): Extension<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    let todo = repository.find(id).ok_or(StatusCode::NOT_FOUND)?;
    Ok((StatusCode::OK, Json(todo)))
}

pub async fn all_todo<T: TodoRepository>(
    Extension(repository): Extension<Arc<T>>,
) -> impl IntoResponse {
    let todos = repository.all();
    (StatusCode::OK, Json(todos)).into_response()
}

pub async fn update_todo<T: TodoRepository>(
    Extension(repository): Extension<Arc<T>>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateTodo>,
) -> Result<impl IntoResponse, StatusCode> {
    let validation = payload.clone();
    match validation.text.as_deref().unwrap_or("").len() {
        // returnを使う理由とinto_response()について調べる
        len if len <= 0 => return Ok((StatusCode::BAD_REQUEST, "Error!: Can not be Empty".to_string()).into_response()),
        len if len > 100 => return Ok((StatusCode::BAD_REQUEST, "Error!: Over text length".to_string()).into_response()),
        _ => {}
    }
    let todo = repository
        .update(id, payload)
        .or(Err(StatusCode::NOT_FOUND))?;
    Ok((StatusCode::CREATED, Json(todo)).into_response())
}

pub async fn delete_todo<T: TodoRepository>(
    Path(id): Path<i32>,
    Extension(repository): Extension<Arc<T>>,
) -> StatusCode {
    repository
        .delete(id)
        .map(|_| StatusCode::NO_CONTENT)
        .unwrap_or(StatusCode::NOT_FOUND)
}
