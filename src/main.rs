use axum::{
    http::StatusCode, response::IntoResponse, routing::{get, post}, Extension, Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env, sync::{Arc, RwLock}};
use thiserror::Error;

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

pub async fn create_todo<T: TodoRepository>(
    Json(payload): Json<CreateTodo>,
    Extension(repository): Extension<Arc<T>>
) -> impl IntoResponse {
    let todo = repository.create(payload);

    (StatusCode::CREATED, Json(todo))
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_user(
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    let user = User {
        id: 1337,
        username: payload.username,
    };

    (StatusCode::CREATED, Json(user))
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct CreateUser {
    username: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct User {
    id: u64,
    username: String,
}

// point 1
#[derive(Debug, Error)]
enum RepositoryError {
    #[error("NotFound, id is {0}")]
    NotFound(i32),
}

// point 2
pub trait TodoRepository: Clone + std::marker::Send + std::marker::Sync + 'static {
    fn create(&self, payload: CreateTodo) -> Todo;
    fn find(&self, id: i32) -> Option<Todo>;
    fn all(&self) -> Vec<Todo>;
    fn update(&self, id: i32, payload: UpdateTodo) -> anyhow::Result<Todo>;
    fn delete(&self, id: i32) -> anyhow::Result<()>;
}

// point 3
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Todo {
    id: i32,
    text: String,
    completed: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct CreateTodo {
    text: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct UpdateTodo {
    text: Option<String>,
    completed: Option<bool>,
}

impl Todo {
    pub fn new(id: i32, text: String) -> Self {
        Self {
            id,
            text,
            completed: false
        }
    }
}

type TodoDatas = HashMap<i32, Todo>;

#[derive(Debug, Clone)]
pub struct TodoRepositoryForMemory {
    store: Arc<RwLock<TodoDatas>>,
}

impl TodoRepositoryForMemory {
    pub fn new() -> Self {
        TodoRepositoryForMemory {
            store: Arc::default(),
        }
    }
}

impl TodoRepository for TodoRepositoryForMemory {
    fn create(&self, payload: CreateTodo) -> Todo {
        todo!();
    }

    fn find(&self, id: i32) -> Option<Todo> {
        todo!();
    }

    fn all(&self) -> Vec<Todo> {
        todo!();
    }

    fn update(&self, id: i32, payload: UpdateTodo) -> anyhow::Result<Todo> {
        todo!();
    }

    fn delete(&self, id: i32) -> anyhow::Result<()> {
        todo!();
    }
}
#[cfg(test)]
mod test {
    use std::usize;
    use super::*;
    use axum::{
        body::Body,
        body::to_bytes,
        http::{header, Method, Request},
    };
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

    #[tokio::test]
    async fn should_return_user_data() {
        let repository = TodoRepositoryForMemory::new();
        let req = Request::builder().uri("/users").method(Method::POST).header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref()).body(Body::from(r#"{ "username": "taro" }"#)).unwrap();
        let res = create_app(repository).oneshot(req).await.unwrap();
        // axum 0.4.8, hyper 0.14.16
        // let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
        // axum 0.7.5, hyper 1.4.1
        let bytes = to_bytes(res.into_body(), usize::MAX).await.unwrap();
        let body: String = String::from_utf8(bytes.to_vec()).unwrap();
        let user: User = serde_json::from_str(&body).expect("cannot convert User instance");
        assert_eq!(user, User {
            id: 1337,
            username: "taro".to_string(),
        });
    }
}
