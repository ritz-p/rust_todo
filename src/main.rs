mod handlers;
mod repositories;

use crate::repositories::{
    TodoRepository,TodoRepositoryForMemory
};
use handlers::create_todo;

use axum::{
    routing::{get, post},
    Extension, Router,
};
use dotenv::dotenv;
use std::{
    env,
    sync::Arc,
};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let log_level = env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt::init();

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let address = format!("{}:{}", host, port);
    let repository = TodoRepositoryForMemory::new();
    let app = create_app(repository);
    let listener = TcpListener::bind(&address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
    tracing::debug!("listening on {}", address);
}

fn create_app<T>(repository: T) -> Router
where
    T: TodoRepository,
{
    let repository = Arc::new(repository);
    Router::new()
        .route("/", get(root))
        .route("/todos", post({
            let repository = Arc::clone(&repository);
            move |payload| create_todo::<T>(payload, Extension(Arc::clone(&repository)))
        }))
        .layer(Extension(Arc::new(repository)))
}

async fn root() -> &'static str {
    "Hello, World!"
}

#[cfg(test)]
mod test {

    use super::*;
    use axum::{
        body::Body,
        http::Request,
    };
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    #[tokio::test]
    async fn should_return_hello_world() {
        let repository = TodoRepositoryForMemory::new();
        let req = Request::builder().uri("/").body(Body::empty()).unwrap();

        let res = create_app(repository).oneshot(req).await.unwrap();

        let bytes = res.into_body().collect().await.unwrap().to_bytes();

        let body = String::from_utf8(bytes.to_vec()).unwrap();

        assert_eq!(body, "Hello, World!");
    }
}
