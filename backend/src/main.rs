mod handlers;
mod repositories;

use crate::repositories::TodoRepositoryForDb;
use axum::{
    routing::{get, post},
    Extension, Router,
};
use dotenv::dotenv;
use handlers::{all_todo, create_todo, delete_todo, find_todo, update_todo};
use shared_struct::todo::repository::TodoRepository;
use sqlx::PgPool;
use std::{env, sync::Arc};
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

    let db_user = env::var("DB_USER").expect("DB_USER undefined");
    let db_password = env::var("DB_PASSWORD").expect("DB_PASSWORD undefined");
    let db_host = env::var("DB_HOST").expect("DB_HOST undefined");
    let db_port = env::var("DB_PORT").expect("DB_PORT undefined");
    let db_name = env::var("DB_NAME").expect("DB_NAME undefined");
    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        db_user, db_password, db_host, db_port, db_name
    );
    tracing::debug!("start connecting database");
    let pool = PgPool::connect(&database_url).await.expect(&format!(
        "failed to connect database,url is [{}]",
        database_url
    ));
    let repository = TodoRepositoryForDb::new(pool);
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
        .route(
            "/todos",
            post({
                let repo = Arc::clone(&repository);
                move |payload| create_todo::<T>(payload, Extension(Arc::clone(&repo)))
            })
            .get(all_todo::<T>),
        )
        .route(
            "/todos/:id",
            get(find_todo::<T>).delete(delete_todo::<T>).patch({
                let repo = Arc::clone(&repository);
                move |id, payload| update_todo::<T>(id, payload, Extension(Arc::clone(&repo)))
            }),
        )
        .layer(Extension(repository))
}

async fn root() -> &'static str {
    "Hello, World!"
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::repositories::test_utils::TodoRepositoryForMemory;
    use axum::{
        body::Body,
        http::{header, Method, Request, StatusCode},
        response::Response,
    };
    use http_body_util::BodyExt;
    use shared_struct::todo::{CreateTodo, Todo};
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
        let bytes = res.into_body().collect().await.unwrap().to_bytes();
        let body = String::from_utf8(bytes.to_vec()).unwrap();

        let todo: Todo = serde_json::from_str(&body)
            .expect(&format!("cannot convert Todo instane. body: {}", body));
        todo
    }

    #[tokio::test]
    async fn should_created_todo() {
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
            .expect("failed to create todo");

        let req = build_todo_req_with_empty(Method::GET, "/todos/1");

        let res = create_app(repository).oneshot(req).await.unwrap();

        let todo = res_to_todo(res).await;

        assert_eq!(expected, todo);
    }

    #[tokio::test]
    async fn should_get_all_todos() {
        let expected = Todo::new(1, "should_get_all_todos".to_string());

        let repository = TodoRepositoryForMemory::new();
        repository
            .create(CreateTodo::new("should_get_all_todos".to_string()))
            .await
            .expect("failed to create todo");

        let req = build_todo_req_with_empty(Method::GET, "/todos");

        let res = create_app(repository).oneshot(req).await.unwrap();

        let bytes = res.into_body().collect().await.unwrap().to_bytes();
        let body = String::from_utf8(bytes.to_vec()).unwrap();

        let todo_list: Vec<Todo> = serde_json::from_str(&body)
            .expect(&format!("cannot convert Todo instance. body: {}", body));

        assert_eq!(vec![expected], todo_list);
    }

    #[tokio::test]
    async fn should_update_todo() {
        let expected = Todo::new(1, "should_update_todo".to_string());

        let repository = TodoRepositoryForMemory::new();
        repository
            .create(CreateTodo::new("before_update_todo".to_string()))
            .await
            .expect("failed to create todo");

        let req = build_todo_req_with_json(
            "/todos/1",
            Method::PATCH,
            r#"{ "id": 1,"text": "should_update_todo", "completed": false}"#.to_string(),
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
            .expect("failed to create todo");

        let req = build_todo_req_with_empty(Method::DELETE, "/todos/1");

        let res = create_app(repository).oneshot(req).await.unwrap();

        assert_eq!(StatusCode::NO_CONTENT, res.status());
    }
}
