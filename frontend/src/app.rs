use crate::component::{text_input_form::TextInputForm, todo_list::TodoList};
use shared_struct::todo::mount::object::todo::Todo;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let url = "http://localhost:8080/todos";
    let todo_list = use_state(|| Vec::<Todo>::new());
    let onclick_todo = { Callback::from(move |_: String| {}) };

    html! {
        <>
            <TextInputForm url={url} function="post" form_label="Task name" text_input="" text_input_placeholder="Sample Todo" submit_label="Submit" reset_lable="Reset" onsubmit={onclick_todo}/>
            <TodoList todo_list={todo_list} function="fetch" url={url}/>
        </>
    }
}
