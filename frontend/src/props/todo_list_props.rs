use shared_struct::structs::Todo;
use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct TodoListProps {
    pub todo_list: UseStateHandle<Vec<Todo>>,
}
