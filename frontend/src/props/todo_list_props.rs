use shared_struct::todo::mount::object::todo::Todo;
use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct TodoListProps {
    pub todo_list: UseStateHandle<Vec<Todo>>,
}
