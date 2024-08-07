use anyhow::Result;
use axum::async_trait;
use shared_struct::todo::{
    error::RepositoryError, repository::TodoRepository, CreateTodo, Todo, UpdateTodo,
};
use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct TodoRepositoryForDb {
    pool: PgPool,
}

impl TodoRepositoryForDb {
    pub fn new(pool: PgPool) -> Self {
        TodoRepositoryForDb { pool }
    }
}

#[async_trait]
impl TodoRepository for TodoRepositoryForDb {
    async fn create(&self, payload: CreateTodo) -> Result<Todo> {
        let todo = sqlx::query_as::<_, Todo>(
            r#"insert into todos (text, completed) values ($1,false) returning *"#,
        )
        .bind(payload.text.clone())
        .fetch_one(&self.pool)
        .await?;

        Ok(todo)
    }

    async fn find(&self, id: i32) -> Result<Todo> {
        let todo = sqlx::query_as::<_, Todo>(r#"select * from todos where id=$1"#)
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => RepositoryError::NotFound(id),
                _ => RepositoryError::Unexpected(e.to_string()),
            })?;
        Ok(todo)
    }

    async fn all(&self) -> Result<Vec<Todo>> {
        let todo_list = sqlx::query_as::<_, Todo>(r#"select * from todos order by id desc"#)
            .fetch_all(&self.pool)
            .await?;

        Ok(todo_list)
    }

    async fn update(&self, id: i32, payload: UpdateTodo) -> Result<Todo> {
        let current_todo = self.find(id).await?;
        let new_todo = sqlx::query_as::<_, Todo>(
            r#"update todos set text=$1, completed=$2 where id=$3 returning *"#,
        )
        .bind(payload.text.unwrap_or(current_todo.text))
        .bind(payload.completed.unwrap_or(current_todo.completed))
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(new_todo)
    }

    async fn delete(&self, id: i32) -> Result<()> {
        sqlx::query(r#"delete from todos where id=$1"#)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => RepositoryError::NotFound(id),
                _ => RepositoryError::Unexpected(e.to_string()),
            })?;

        Ok(())
    }
}

#[cfg(test)]
#[cfg(feature = "database-test")]
mod test {
    use super::*;
    use dotenv::dotenv;
    use sqlx::PgPool;
    use std::env;

    #[tokio::test]
    async fn crud_scenario() {
        dotenv().ok();
        let db_user = env::var("DB_USER").expect("DB_USER undefined");
        let db_password = env::var("DB_PASSWORD").expect("DB_PASSWORD undefined");
        let db_host = env::var("DB_HOST").expect("DB_HOST undefined");
        let db_port = env::var("DB_PORT").expect("DB_PORT undefined");
        let db_name = env::var("DB_NAME").expect("DB_NAME undefined");
        let database_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            db_user, db_password, db_host, db_port, db_name
        );
        let pool = PgPool::connect(&database_url).await.expect(&format!(
            "failed to connect database, url is [{}]",
            database_url
        ));
        let repository = TodoRepositoryForDb::new(pool.clone());
        let todo_text = "[crud_scenario] text";

        let created = repository
            .create(CreateTodo::new(todo_text.to_string()))
            .await
            .expect("[create] returned Err");
        assert_eq!(created.text, todo_text);
        assert!(!created.completed);

        let todo = repository
            .find(created.id)
            .await
            .expect("[find] returned Err");
        assert_eq!(created, todo);

        let todos = repository.all().await.expect("[all] returned Err");
        let todo = todos.first().unwrap();
        assert_eq!(created, *todo);

        let updated_text = "[crud_scenario] updated text";
        let todo = repository
            .update(
                todo.id,
                UpdateTodo {
                    text: Some(updated_text.to_string()),
                    completed: Some(true),
                },
            )
            .await
            .expect("[update] returned Err");

        assert_eq!(created.id, todo.id);
        assert_eq!(todo.text, updated_text);

        repository
            .delete(todo.id)
            .await
            .expect("[delete] returned Err");

        let res = repository.find(created.id).await;
        assert!(res.is_err());

        let todo_rows = sqlx::query(r#"select * from todos where id=$1"#)
            .bind(todo.id)
            .fetch_all(&pool)
            .await
            .expect("[delete] todo_labels fetch error");

        assert!(todo_rows.len() == 0)
    }
}

#[cfg(test)]
pub mod test_utils {
    use super::*;
    use anyhow::Context;
    use std::{
        collections::HashMap,
        sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
    };

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

        fn write_store_ref(&self) -> RwLockWriteGuard<TodoDataMap> {
            self.store.write().unwrap()
        }

        fn read_store_ref(&self) -> RwLockReadGuard<TodoDataMap> {
            self.store.read().unwrap()
        }
    }

    #[async_trait]
    impl TodoRepository for TodoRepositoryForMemory {
        async fn create(&self, payload: CreateTodo) -> Result<Todo> {
            let mut store = self.write_store_ref();
            let id = (store.len() + 1) as i32;
            let todo = Todo::new(id, payload.text.clone());
            store.insert(id, todo.clone());
            Ok(todo)
        }

        async fn find(&self, id: i32) -> Result<Todo> {
            let store = self.read_store_ref();
            let todo = store
                .get(&id)
                .map(|todo| todo.clone())
                .ok_or(RepositoryError::NotFound(id))?;
            Ok(todo)
        }

        async fn all(&self) -> Result<Vec<Todo>> {
            let store = self.read_store_ref();
            Ok(Vec::from_iter(store.values().map(|todo| todo.clone())))
        }

        async fn update(&self, id: i32, payload: UpdateTodo) -> Result<Todo> {
            let mut store = self.write_store_ref();
            let todo_con = store.get(&id).context(RepositoryError::NotFound(id))?;
            let text = payload.text.unwrap_or(todo_con.text.clone());
            let completed = payload.completed.unwrap_or(todo_con.completed);

            let todo = Todo {
                id,
                text,
                completed,
            };
            store.insert(id, todo.clone());
            Ok(todo)
        }

        async fn delete(&self, id: i32) -> Result<()> {
            let mut store = self.write_store_ref();
            store.remove(&id).ok_or(RepositoryError::NotFound(id))?;
            Ok(())
        }
    }
    mod test {
        use super::*;

        #[tokio::test]
        async fn todo_crud_scenario() {
            let text = "todo text".to_string();
            let id = 1;
            let expected = Todo::new(id, text.clone());

            let repository = TodoRepositoryForMemory::new();
            let todo = repository
                .create(CreateTodo { text })
                .await
                .expect("failed to create todo");

            assert_eq!(expected, todo);

            let todo = repository.find(todo.id).await.unwrap();
            assert_eq!(expected, todo);

            let todo = repository.all().await.unwrap();
            assert_eq!(vec![expected], todo);

            let text = "update todo text".to_string();
            let todo = repository
                .update(
                    1,
                    UpdateTodo {
                        text: Some(text.clone()),
                        completed: Some(true),
                    },
                )
                .await
                .expect("failed update todo.");
            assert_eq!(
                Todo {
                    id,
                    text,
                    completed: true,
                },
                todo
            );

            let res = repository.delete(id).await;
            assert!(res.is_ok());
        }
    }
}
