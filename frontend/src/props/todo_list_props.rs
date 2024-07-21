use yew::prelude::*;
use shared_struct::structs::Todo;
#[derive(PartialEq, Properties)]
pub struct TodoListProps {
    pub todo_list: UseStateHandle<Vec<Todo>>
}