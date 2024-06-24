use anyhow::Context;
use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    env,
    marker::{Send, Sync},
    sync::{Arc, RwLock},
};
use thiserror::Error;
use tokio::net::TcpListener;

#[derive(Debug, Error)]
enum RepositoryError {
    #[error("NotFound, id is {0}")]
    NotFound(i32),
}

pub trait TodoRepository: Clone + Send + Sync + 'static {
    fn create(&self, payload: CreateTodo) -> Todo;
    fn find(&self, id: i32) -> Option<Todo>;
    fn all(&self) -> Vec<Todo>;
    fn update(&self, id: i32, payload: UpdateTodo) -> anyhow::Result<Todo>;
    fn delete(&self, id: i32) -> anyhow::Result<()>;
}

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
            completed: false,
        }
    }
}

type TodoDataMap = HashMap<i32, Todo>;

#[derive(Debug, Clone)]
pub struct TodoRepositoryForMemory {
    store: Arc<RwLock<TodoDataMap>>,
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
        let todo = Todo::new(1, payload.text);
        let mut store = self.store.write().unwrap();
        store.insert(todo.id, todo.clone());
        todo
    }

    fn find(&self, id: i32) -> Option<Todo> {
        todo!()
    }

    fn all(&self) -> Vec<Todo> {
        todo!()
    }

    fn update(&self, id: i32, payload: UpdateTodo) -> anyhow::Result<Todo> {
        todo!()
    }

    fn delete(&self, id: i32) -> anyhow::Result<()> {
        todo!()
    }
}

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

pub async fn create_todo<T>(
    Json(payload): Json<CreateTodo>,
    Extension(repository): Extension<Arc<T>>,
) -> impl IntoResponse
where
    T: TodoRepository,
{
    let todo = repository.create(payload);

    (StatusCode::CREATED, Json(todo))
}

async fn root() -> &'static str {
    "Hello, World!"
}

#[cfg(test)]
mod test {

    use super::*;
    use axum::{
        body::Body,
        http::{Request},
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
