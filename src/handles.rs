use crate::repositories::{CreateTodo, TodoRepository, UpdateTodo};
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use std::sync::Arc;

pub async fn create_todo<T: TodoRepository>(
    Extension(repository): Extension<Arc<T>>,
    Json(payload): Json<CreateTodo>,
) -> impl IntoResponse {
    match payload.text.len() {
        // TODO:BAD_REQUESTを送りたい
        len if len <= 0 => panic!("Error!: Can not be Empty"),
        len if len > 100 => panic!("Error!: Over text length"),
        _ => (StatusCode::CREATED, Json(repository.create(payload)))
    }
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
    let todo = repository.all();
    (StatusCode::OK, Json(todo))
}

pub async fn update_todo<T: TodoRepository>(
    Extension(repository): Extension<Arc<T>>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateTodo>,
) -> Result<impl IntoResponse, StatusCode> {
    let validation = payload.clone();
    match validation.text.unwrap().len() {
        // TODO:BAD_REQUESTを送りたい
        len if len <= 0 => panic!("Error!: Can not be Empty"),
        len if len > 100 => panic!("Error!: Over text length"),
        _ => {}
    }
    let todo = repository
        .update(id, payload)
        .or(Err(StatusCode::NOT_FOUND))?;
    Ok((StatusCode::CREATED, Json(todo)))
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
