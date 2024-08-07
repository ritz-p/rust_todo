use serde::{Deserialize, Serialize};
use validator::Validate;

#[cfg_attr(feature = "back", derive(sqlx::prelude::FromRow))]
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Eq, Validate)]
pub struct UpdateTodo {
    #[validate(length(min = 1, max = 100, message = "text must be between 1 ~ 100"))]
    pub text: Option<String>,
    pub completed: Option<bool>,
}
