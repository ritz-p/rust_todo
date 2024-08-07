use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "back", derive(sqlx::prelude::FromRow))]
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Eq)]
pub struct Todo {
    pub id: i32,
    pub text: String,
    pub completed: bool,
}
