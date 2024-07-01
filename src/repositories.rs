use anyhow::{Result};
use axum::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use thiserror::Error;
use validator::Validate;

#[derive(Debug, Error)]
enum RepositoryError {
    #[error("NotFound, id is {0}")]
    NotFound(i32),
}

#[async_trait]
pub trait TodoRepository: Clone + Send + Sync + 'static {
    async fn create(&self, payload: CreateTodo) -> Result<Todo>;
    async fn find(&self, id: i32) -> Result<Todo>;
    async fn all(&self) -> Result<Vec<Todo>>;
    async fn update(&self, id: i32, payload: UpdateTodo) -> Result<Todo>;
    async fn delete(&self, id: i32) -> Result<()>;
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Todo {
    id: i32,
    text: String,
    completed: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Validate)]
pub struct CreateTodo {
    #[validate(length(min = 1, max = 100, message = "text must be between 1 ~ 100"))]
    text: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Validate)]
pub struct UpdateTodo {
    #[validate(length(min = 1, max = 100, message = "text must be between 1 ~ 100"))]
    text: Option<String>,
    completed: Option<bool>,
}

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
        todo!()
    }

    async fn find(&self, id: i32) -> Result<Todo> {
        todo!()
    }

    async fn all(&self) -> Result<Vec<Todo>> {
        todo!()
    }

    async fn update(&self, id: i32, payload: UpdateTodo) -> Result<Todo> {
        todo!()
    }

    async fn delete(&self, id: i32) -> Result<()> {
        todo!()
    }
}

#[cfg(test)]
pub mod test_utils{
    use anyhow::Context;
    use std::{
        collections::HashMap,
        sync::{Arc,RwLock,RwLockReadGuard,RwLockWriteGuard}
    };
    use super::*;

    impl Todo {
        pub fn new(id: i32, text: String) -> Self {
            Self {
                id,
                text,
                completed: false,
            }
        }
    }

    impl CreateTodo {
        pub fn new(text: String) -> Self {
            Self { text }
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