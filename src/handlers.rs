use crate::repositories::{CreateTodo, TodoRepository, UpdateTodo};
use axum::{
    async_trait, extract::{rejection::{self, FormRejection, JsonRejection}, Extension, FromRequest, FromRequestParts, Path, Request}, http::StatusCode, response::IntoResponse, BoxError, Form, Json
};
use serde::de::DeserializeOwned;
use std::sync::Arc;
use validator::Validate;

#[derive(Debug)]
pub struct ValidateJson<T>(T);

#[async_trait]
impl<T,S> FromRequest<S> for ValidateJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Json<T>: FromRequest<S,Rejection = JsonRejection>
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: Request,state: &S) -> Result<Self,Self::Rejection>{
        let Json(value) = Json::<T>::from_request(req,state).await.map_err(|rejection|{
            let message = format!("Json parse error: [{}]",rejection);
            (StatusCode::BAD_REQUEST,message)
        })?;
        value.validate().map_err(|rejection| {
            let message = format!("Validation error: [{}]",rejection).replace('\n', ", ");
            (StatusCode::BAD_REQUEST,message)
        })?;
        Ok(ValidateJson(value))
    }
}

pub async fn create_todo<T>(
    ValidateJson(payload): ValidateJson<CreateTodo>,
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
    Extension(repository): Extension<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode>
where
    T: TodoRepository,
{
    let todo = repository.find(id).ok_or(StatusCode::NOT_FOUND)?;
    Ok((StatusCode::OK, Json(todo)))
}

pub async fn all_todo<T>(Extension(repository): Extension<Arc<T>>) -> impl IntoResponse
where
    T: TodoRepository,
{
    let todo = repository.all();
    (StatusCode::OK, Json(todo))
}

pub async fn update_todo<T>(
    Path(id): Path<i32>,
    ValidateJson(payload): ValidateJson<UpdateTodo>,
    Extension(repository): Extension<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode>
where
    T: TodoRepository + Send + Sync + 'static,
{
    let todo = repository
        .update(id, payload)
        .or(Err(StatusCode::NOT_FOUND))?;
    Ok((StatusCode::OK, Json(todo)))
}

pub async fn delete_todo<T>(
    Path(id): Path<i32>,
    Extension(repository): Extension<Arc<T>>,
) -> StatusCode
where
    T: TodoRepository,
{
    repository
        .delete(id)
        .map(|_| StatusCode::NO_CONTENT)
        .unwrap_or(StatusCode::NOT_FOUND)
}
