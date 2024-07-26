use crate::repositories::{CreateTodo, TodoRepository, UpdateTodo};
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use std::sync::Arc;

const ERR_STR_EMPTY: &str = "Error!: Can not be Empty";
const ERR_STR_OVER: &str = "Error!: Over text length";
const ERR_STR_NOT_FOUND: &str = "Todo not found";

pub async fn create_todo<T: TodoRepository>(
    Extension(repository): Extension<Arc<T>>,
    Json(payload): Json<CreateTodo>,
) -> impl IntoResponse {
    let response = if payload.text.len() <= 0 {
        (StatusCode::BAD_REQUEST, ERR_STR_EMPTY.to_string()).into_response()
    } else if payload.text.len() > 100 {
        (StatusCode::BAD_REQUEST, ERR_STR_OVER.to_string()).into_response()
    } else {
        let todo = repository.create(payload);
        (StatusCode::CREATED, Json(todo)).into_response()
    };

    response
}

pub async fn find_todo<T: TodoRepository>(
    Path(id): Path<i32>,
    Extension(repository): Extension<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    let response = match repository.find(id) {
        Some(todo) => (StatusCode::OK, Json(todo)).into_response(),
        None => (StatusCode::NOT_FOUND, ERR_STR_NOT_FOUND.to_string()).into_response(),
    };

    Ok(response)
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
    let response = if payload.text.as_deref().unwrap_or("").len() <= 0 {
        (StatusCode::BAD_REQUEST, ERR_STR_EMPTY.to_string()).into_response()
    } else if payload.text.as_deref().unwrap_or("").len() > 100 {
        (StatusCode::BAD_REQUEST, ERR_STR_OVER.to_string()).into_response()
    } else {
        match repository.update(id, payload) {
            Ok(todo) => (StatusCode::CREATED, Json(todo)).into_response(),
            Err(_) => (StatusCode::NOT_FOUND, ERR_STR_NOT_FOUND.to_string()).into_response(),
        }
    };

    Ok(response)
}

pub async fn delete_todo<T: TodoRepository>(
    Path(id): Path<i32>,
    Extension(repository): Extension<Arc<T>>,
) -> StatusCode {
    let response = if repository.delete(id).is_ok() {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    };

    response
}
