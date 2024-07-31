mod handlers;
mod repositories;

use axum::{
    routing::{get, post},
    Extension, Router,
};
use dotenv::dotenv;
use handles::{all_todo, create_todo, delete_todo, find_todo, update_todo};
use repositories::{TodoRepository, TodoRepositoryForDb};
use sqlx::PgPool;
use std::{env, sync::Arc};
use hyper::header::CONTENT_TYPE;
use tower_http::cors::{Any, CorsLayer, AllowOrigin};

#[tokio::main]
async fn main() {
    // loggingの初期化
    // RUST_LOG=debug cargo run
    let log_level = env::var("RUST_LOG").unwrap_or("info".to_string());
    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt::init();
    dotenv().ok();

    let database_url = &env::var("DATABASE_URL").expect("undefined [DATABASE_URL]");
    tracing::debug!("start connect database...");
    let pool = PgPool::connect(database_url)
        .await
        .expect(&format!("fail connect database, url is [{}]", database_url));
    let repository = TodoRepositoryForDb::new(pool.clone());
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
    let allowed_origins = vec![
        "http://localhost:3001".parse().unwrap(),
        "http://127.0.0.1:3001".parse().unwrap(),
    ];

    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::list(allowed_origins))
        .allow_methods(Any)
        .allow_headers(vec![CONTENT_TYPE]);

    Router::new()
        .route("/", get(root))
        .route("/todos", post(create_todo::<T>).get(all_todo::<T>))
        .route(
            "/todos/:id",
            get(find_todo::<T>)
                .delete(delete_todo::<T>)
                .patch(update_todo::<T>),
        )
        .layer(Extension(Arc::new(repository)))
        .layer(cors)
}

async fn root() -> &'static str {
    "Hello, World!"
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::repositories::{test_utils::TodoRepositoryForMemory, CreateTodo, Todo};
    use axum::{
        body::{to_bytes, Body},
        http::{header, Method, Request, StatusCode},
        response::Response,
    };
    use std::usize;
    use tower::ServiceExt;

    fn build_todo_req_with_json(path: &str, method: Method, json_body: String) -> Request<Body> {
        Request::builder()
            .uri(path)
            .method(method)
            .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(json_body))
            .unwrap()
    }

    fn build_todo_req_with_empty(method: Method, path: &str) -> Request<Body> {
        Request::builder()
            .uri(path)
            .method(method)
            .body(Body::empty())
            .unwrap()
    }

    async fn res_to_todo(res: Response) -> Todo {
        // axum 0.4.8, hyper 0.14.16
        // let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
        // axum 0.7.5, hyper 1.4.1
        let bytes = to_bytes(res.into_body(), usize::MAX).await.unwrap();
        let body: String = String::from_utf8(bytes.to_vec()).unwrap();
        let todo: Todo = serde_json::from_str(&body)
            .expect(&format!("cannnot convert Todo instance. body:{}", body));
        todo
    }

    #[tokio::test]
    async fn should_craeted_todo() {
        let expected = Todo::new(1, "should_return_created_todo".to_string());
        let repository = TodoRepositoryForMemory::new();
        let req = build_todo_req_with_json(
            "/todos",
            Method::POST,
            r#"{ "text": "should_return_created_todo" }"#.to_string(),
        );
        let res = create_app(repository).oneshot(req).await.unwrap();
        let todo = res_to_todo(res).await;
        assert_eq!(expected, todo);
    }

    #[tokio::test]
    async fn should_find_todo() {
        let expected = Todo::new(1, "should_find_todo".to_string());
        let repository = TodoRepositoryForMemory::new();
        repository
            .create(CreateTodo::new("should_find_todo".to_string()))
            .await
            .expect("failed create todo");
        let req = build_todo_req_with_empty(Method::GET, "/todos/1");
        let res = create_app(repository).oneshot(req).await.unwrap();
        let todo = res_to_todo(res).await;
        assert_eq!(expected, todo);
    }

    #[tokio::test]
    async fn should_get_all_todo() {
        let expected = Todo::new(1, "should_get_all_todo".to_string());
        let repository = TodoRepositoryForMemory::new();
        repository
            .create(CreateTodo::new("should_get_all_todo".to_string()))
            .await
            .expect("failed create todo");
        let req = build_todo_req_with_empty(Method::GET, "/todos");
        let res = create_app(repository).oneshot(req).await.unwrap();
        let bytes = to_bytes(res.into_body(), usize::MAX).await.unwrap();
        let body: String = String::from_utf8(bytes.to_vec()).unwrap();
        let todo: Vec<Todo> = serde_json::from_str(&body)
            .expect(&format!("cannot convert Todo instance. body:{}", body));
        assert_eq!(vec![expected], todo);
    }

    #[tokio::test]
    async fn should_update_todo() {
        let expected = Todo::new(1, "should_update_todo".to_string());
        let repository = TodoRepositoryForMemory::new();
        repository
            .create(CreateTodo::new("should_update_todo".to_string()))
            .await
            .expect("failed create todo");
        let req = build_todo_req_with_json(
            "/todos/1",
            Method::PATCH,
            r#"{ "id": 1, "text": "should_update_todo", "completed": false }"#.to_string(),
        );
        let res = create_app(repository).oneshot(req).await.unwrap();
        let todo = res_to_todo(res).await;
        assert_eq!(expected, todo);
    }

    #[tokio::test]
    async fn should_delete_todo() {
        let repository = TodoRepositoryForMemory::new();
        repository
            .create(CreateTodo::new("should_delete_todo".to_string()))
            .await
            .expect("failed create todo");
        let req = build_todo_req_with_empty(Method::DELETE, "/todos/1");
        let res = create_app(repository).oneshot(req).await.unwrap();
        assert_eq!(StatusCode::NO_CONTENT, res.status());
    }
}
