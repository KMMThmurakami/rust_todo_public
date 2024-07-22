use axum::{routing::get, Router};
use std::env;

#[tokio::main]
async fn main() {
    // loggingの初期化
    let log_level = env::var("RUST_LOG").unwrap_or("info".to_string());
    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/", get(root));

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

async fn root() -> &'static str {
    "Hello, world!"
}
