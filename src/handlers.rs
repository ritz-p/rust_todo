use axum::{
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use std::sync::Arc;
use crate::repositories::{CreateTodo,TodoRepository};

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