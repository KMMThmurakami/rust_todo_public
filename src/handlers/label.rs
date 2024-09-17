use crate::repositories::label::{CreateLabel, LabelRepository};
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use std::sync::Arc;

const ERR_STR_EMPTY: &str = "Error!: Can not be Empty";
const ERR_STR_OVER: &str = "Error!: Over text length";

pub async fn create_label<T: LabelRepository>(
    Extension(repository): Extension<Arc<T>>,
    Json(payload): Json<CreateLabel>,
) -> Result<impl IntoResponse, StatusCode> {
    let response = match payload.name.len() {
        0 => (StatusCode::BAD_REQUEST, ERR_STR_EMPTY.to_string()).into_response(),
        len if len > 100 => (StatusCode::BAD_REQUEST, ERR_STR_OVER.to_string()).into_response(),
        _ => {
            let label = repository
                .create(payload.name)
                .await
                .or(Err(StatusCode::NOT_FOUND))?;
            (StatusCode::CREATED, Json(label)).into_response()
        }
    };

    Ok(response)
}

pub async fn all_label<T: LabelRepository>(
    Extension(repository): Extension<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    let labels = repository.all().await.unwrap();
    Ok((StatusCode::OK, Json(labels)).into_response())
}

pub async fn delete_label<T: LabelRepository>(
    Path(id): Path<i32>,
    Extension(repository): Extension<Arc<T>>,
) -> StatusCode {
    match repository.delete(id).await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::NOT_FOUND,
    }
}
