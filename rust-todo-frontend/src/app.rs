use std::{borrow::{Borrow, BorrowMut}, cell::RefCell, rc::Rc};

use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use web_sys::HtmlInputElement;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[function_component(App)]
pub fn app() -> Html {
    // let counter = use_state(|| 0);
    // let onclick = {
    //     let counter = counter.clone();
    //     Callback::from(move |_| counter.set(*counter + 1))
    // };

    let todo_list = use_state(|| Vec::<Todo>::new());
    let onclick_todo = {
        let todo_list = todo_list.clone();
        Callback::from(move |_| {
            let mut todos = (*todo_list).clone();
            todos.push(Todo{id:1,text: "test".to_string(),completed:false});
            todo_list.set(todos.to_vec())
        })
    };

    html! {
        <div>
            <button onclick={onclick_todo}>{ "Increment value" }</button>
            <p>
                <b>{ "Current value: " }</b>
                { for todo_list.iter().map(|todo| html!{
                    <li key={todo.id}>
                        {&todo.text}
                    </li>
                }) }
            </p>
        </div>
    }
}

#[derive(Clone,PartialEq)]
struct Todo{
    id: usize,
    text: String,
    completed: bool
}