use serde::{Deserialize, Serialize};
// use shared_struct::todo::mount::object::todo::Todo;
use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct TodoListProps {
    pub endpoint: String,
    pub todo_list: UseStateHandle<Vec<Todo>>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Eq)]
pub struct Todo {
    pub id: i32,
    pub text: String,
    pub completed: bool,
}
