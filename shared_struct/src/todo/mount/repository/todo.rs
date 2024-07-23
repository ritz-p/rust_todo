use anyhow::Result;
use axum::async_trait;

#[async_trait]
pub trait TodoRepository<Todo, CreateTodo, UpdateTodo>: Clone + Send + Sync + 'static {
    async fn create(&self, payload: CreateTodo) -> Result<Todo>;
    async fn find(&self, id: i32) -> Result<Todo>;
    async fn all(&self) -> Result<Vec<Todo>>;
    async fn update(&self, id: i32, payload: UpdateTodo) -> Result<Todo>;
    async fn delete(&self, id: i32) -> Result<()>;
}
