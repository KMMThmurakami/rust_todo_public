use crate::repositories::todo::{CreateTodo, TodoRepository, UpdateTodo};
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use std::sync::Arc;

const ERR_STR_EMPTY: &str = "Error!: Can not be Empty";
const ERR_STR_OVER: &str = "Error!: Over text length";
const ERR_STR_NOT_FOUND: &str = "Todo not found";

pub async fn create_todo<T: TodoRepository>(
    Extension(repository): Extension<Arc<T>>,
    Json(payload): Json<CreateTodo>,
) -> Result<impl IntoResponse, StatusCode> {
    let response = match payload.text.len() {
        len if len <= 0 => (StatusCode::BAD_REQUEST, ERR_STR_EMPTY.to_string()).into_response(),
        len if len > 100 => (StatusCode::BAD_REQUEST, ERR_STR_OVER.to_string()).into_response(),
        _ => {
            let todo = repository
                .create(payload)
                .await
                .or(Err(StatusCode::NOT_FOUND))?;
            (StatusCode::CREATED, Json(todo)).into_response()
        }
    };

    Ok(response)
}

pub async fn find_todo<T: TodoRepository>(
    Path(id): Path<i32>,
    Extension(repository): Extension<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    let todo = repository.find(id).await.or(Err(StatusCode::NOT_FOUND))?;
    let response = (StatusCode::CREATED, Json(todo)).into_response();

    Ok(response)
}

pub async fn all_todo<T: TodoRepository>(
    Extension(repository): Extension<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    let todos = repository.all().await.unwrap();
    Ok((StatusCode::OK, Json(todos)).into_response())
}

pub async fn update_todo<T: TodoRepository>(
    Extension(repository): Extension<Arc<T>>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateTodo>,
) -> Result<impl IntoResponse, StatusCode> {
    let response = match payload.text.as_deref().unwrap_or("").len() {
        len if len <= 0 => (StatusCode::BAD_REQUEST, ERR_STR_EMPTY.to_string()).into_response(),
        len if len > 100 => (StatusCode::BAD_REQUEST, ERR_STR_OVER.to_string()).into_response(),
        _ => match repository.update(id, payload).await {
            Ok(todo) => (StatusCode::CREATED, Json(todo)).into_response(),
            Err(_) => (StatusCode::NOT_FOUND, ERR_STR_NOT_FOUND.to_string()).into_response(),
        },
    };

    Ok(response)
}

pub async fn delete_todo<T: TodoRepository>(
    Path(id): Path<i32>,
    Extension(repository): Extension<Arc<T>>,
) -> StatusCode {
    let response = match repository.delete(id).await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::NOT_FOUND,
    };

    response
}
