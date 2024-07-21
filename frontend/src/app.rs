use std::{
    borrow::{Borrow, BorrowMut},
    cell::RefCell,
    rc::Rc,
};

use js_sys::JSON::parse;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{window, HtmlInputElement};
use yew::prelude::*;

mod component;
use component::task_input_form;
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
        Callback::from(move |_| {
            let id = todo_list.len() + 1;
            let task_name = get_value_by_id("task_name").parse().unwrap();
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
        <div>
            <task_input_form::TaskInputForm/>
            <button onclick={onclick_todo}>{ "Add Todo" }</button>
            <p>
                <b>{ "Current value: " }</b>
                { for todo_list.iter().map(|todo| html!{
                    <li key={todo.id}>
                        {format!("{}. {}",&todo.id,&todo.text)}
                    </li>
                }) }
            </p>
        </div>
    }
}

#[derive(Clone, PartialEq)]
struct Todo {
    id: usize,
    text: String,
    completed: bool,
}

fn get_value_by_id(id: &str) -> String {
    window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id(id)
        .unwrap()
        .dyn_ref::<HtmlInputElement>()
        .unwrap()
        .value()
}
