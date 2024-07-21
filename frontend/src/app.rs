use crate::component::{text_input_form::TextInputForm,todo_list::TodoList};
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use shared_struct::structs::Todo;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[function_component(App)]
pub fn app() -> Html {
    let todo_list = use_state(|| Vec::<Todo>::new());
    let onclick_todo = {
        let todo_list = todo_list.clone();
        Callback::from(move |task_name: String| {
            let id = todo_list.len() + 1;
            let mut todos = (*todo_list).clone();
            todos.push(Todo {
                id,
                text: task_name,
                completed: false,
            });
            todo_list.set(todos.to_vec())
        })
    };

    html! {
        <>
            <TextInputForm form_label="Task name" text_input="" text_input_placeholder="Sample Todo" submit_label="Submit" reset_lable="Reset" onsubmit={onclick_todo}/>
            <TodoList todo_list={todo_list}/>
        </>
    }
}