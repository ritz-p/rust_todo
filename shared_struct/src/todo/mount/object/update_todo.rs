use serde::{Deserialize, Serialize};
use validator::Validate;
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Validate)]
pub struct UpdateTodo {
    #[validate(length(min = 1, max = 100, message = "text must be between 1 ~ 100"))]
    pub text: Option<String>,
    pub completed: Option<bool>,
}
