use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Eq, FromRow)]
pub struct Todo {
    pub id: i32,
    pub text: String,
    pub completed: bool,
}
