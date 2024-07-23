pub mod error;
pub mod mount;

use mount::object::{create_todo::CreateTodo, todo::Todo};

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
