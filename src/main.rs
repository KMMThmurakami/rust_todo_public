mod handles;
mod repositories;

use axum::{
    routing::{get, post},
    Extension, Router,
};
use handles::create_todo;
use repositories::{TodoRepository, TodoRepositoryForMemory};
use std::{env, sync::Arc};

#[tokio::main]
async fn main() {
    // loggingの初期化
    let log_level = env::var("RUST_LOG").unwrap_or("info".to_string());
    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt::init();

    let repository = TodoRepositoryForMemory::new();
    let app = create_app(repository);

    // axum 0.4.8
    // let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();

    // axum 0.7.5
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {:?}", listener);
    axum::serve(listener, app).await.unwrap();
}

fn create_app<T: TodoRepository>(repository: T) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/todos", post(create_todo::<T>))
        .layer(Extension(Arc::new(repository)))
}

async fn root() -> &'static str {
    "Hello, World!"
}

// async fn create_user(Json(payload): Json<CreateUser>) -> impl IntoResponse {
//     let user = User {
//         id: 1337,
//         username: payload.username,
//     };

//     (StatusCode::CREATED, Json(user))
// }

// #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
// struct CreateUser {
//     username: String,
// }

// #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
// struct User {
//     id: u64,
//     username: String,
// }

#[cfg(test)]
mod test {
    use super::*;
    use axum::{body::to_bytes, body::Body, http::Request};
    use std::usize;
    use tower::ServiceExt;

    #[tokio::test]
    async fn should_return_hello_world() {
        let repository = TodoRepositoryForMemory::new();
        let req = Request::builder().uri("/").body(Body::empty()).unwrap();
        let res = create_app(repository).oneshot(req).await.unwrap();
        // axum 0.4.8, hyper 0.14.16
        // let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
        // axum 0.7.5, hyper 1.4.1
        let bytes = to_bytes(res.into_body(), usize::MAX).await.unwrap();
        let body: String = String::from_utf8(bytes.to_vec()).unwrap();
        assert_eq!(body, "Hello, World!")
    }

    // #[tokio::test]
    // async fn should_return_user_data() {
    //     let repository = TodoRepositoryForMemory::new();
    //     let req = Request::builder()
    //         .uri("/users")
    //         .method(Method::POST)
    //         .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
    //         .body(Body::from(r#"{ "username": "taro" }"#))
    //         .unwrap();
    //     let res = create_app(repository).oneshot(req).await.unwrap();
    //     // axum 0.4.8, hyper 0.14.16
    //     // let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
    //     // axum 0.7.5, hyper 1.4.1
    //     let bytes = to_bytes(res.into_body(), usize::MAX).await.unwrap();
    //     let body: String = String::from_utf8(bytes.to_vec()).unwrap();
    //     let user: User = serde_json::from_str(&body).expect("cannot convert User instance");
    //     assert_eq!(
    //         user,
    //         User {
    //             id: 1337,
    //             username: "taro".to_string(),
    //         }
    //     );
    // }
}
