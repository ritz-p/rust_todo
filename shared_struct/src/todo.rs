use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use validator::Validate;

pub mod error;
pub mod repository;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Eq, FromRow)]
pub struct Todo {
    pub id: i32,
    pub text: String,
    pub completed: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Validate)]
pub struct CreateTodo {
    #[validate(length(min = 1, max = 100, message = "text must be between 1 ~ 100"))]
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Validate)]
pub struct UpdateTodo {
    #[validate(length(min = 1, max = 100, message = "text must be between 1 ~ 100"))]
    pub text: Option<String>,
    pub completed: Option<bool>,
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

impl CreateTodo {
    pub fn new(text: String) -> Self {
        Self { text }
    }
}
