use axum::{
    http::StatusCode,
    response::IntoResponse,
    extract::{Extension,Path},Json,
};
use std::sync::Arc;
use crate::repositories::{CreateTodo,TodoRepository, UpdateTodo};

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

pub async fn find_todo<T>(
    Path(id): Path<i32>,
    Extension(repository): Extension<Arc<T>>
) -> Result<impl IntoResponse,StatusCode>
where
    T: TodoRepository
{
    let todo = repository.find(id).ok_or(StatusCode::NOT_FOUND)?;
    Ok((StatusCode::OK,Json(todo)))
}

pub async fn all_todo<T>(
    Extension(repository): Extension<Arc<T>>
) -> impl IntoResponse
where 
    T: TodoRepository
{
    let todo = repository.all();
    (StatusCode::OK,Json(todo))
}

pub async fn update_todo<T>(
    Path(id): Path<i32>,
    Json(payload): Json<UpdateTodo>,
    Extension(repository): Extension<Arc<T>>
) -> Result<impl IntoResponse,StatusCode>
where 
    T: TodoRepository + Send + Sync + 'static,
{
    let todo = repository.update(id,payload).or(Err(StatusCode::NOT_FOUND))?;
    Ok((StatusCode::OK,Json(todo)))
}

pub async fn delete_todo<T>(
    Path(id): Path<i32>,
    Extension(repository): Extension<Arc<T>>
) -> StatusCode
where 
    T: TodoRepository
{
    repository.delete(id).map(|_| StatusCode::NO_CONTENT).unwrap_or(StatusCode::NOT_FOUND)
}