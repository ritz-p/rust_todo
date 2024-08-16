use crate::props::todo_list_props::TodoListProps;
use patternfly_yew::components::{
    list::{List, ListItem, ListType},
    page::{PageSection, PageSectionGroup},
};

use serde::Serialize;
use serde_wasm_bindgen::{from_value, to_value, Error};
use shared_struct::todo::mount::object::todo::Todo;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window.__TAURI__.tauri"])]
    async fn invoke(cmd: &str,args: JsValue) -> JsValue;
}

#[function_component]
pub fn TodoList(props: &TodoListProps) -> Html {
    let todo_list = use_state(Vec::new);
    let url = props.url.clone();
    let function = props.function.clone();
    {
        let todo_list = todo_list.clone();
        use_effect_with(true, move |_| {
            spawn_local(async move {
                let args = to_value(&FetchArgs{url});
                match args{
                    Ok(serialized_args) => {
                        let response = invoke(&function,serialized_args).await;
                        match convert_js_value(response){
                            Ok(new_todo_list) => {
                                todo_list.set(new_todo_list);
                            },
                            Err(err) => {
                                println!("{}",err);
                            },
                        }
                    },
                    Err(err) => {
                        println!("{}",err);
                    },
                }
            })
        });
    }
    html!(
        <PageSectionGroup>
            <PageSection>
                <List r#type={ListType::Bordered}>
                    { for todo_list.iter().map(|todo| html_nested!{
                        <ListItem>
                            {todo.text.clone()}
                        </ListItem>
                    }) }
                </List>
            </PageSection>
        </PageSectionGroup>
    )
}

<<<<<<< HEAD
fn convert_js_value(result: JsValue) -> Result<Vec<Todo>,Error> {
    if result.is_object(){
        let todos: Vec<Todo> = serde_wasm_bindgen::from_value(result).map_err(|err| JsValue::from_str(&err.to_string()))?;
        Ok(todos)
    }else{
        Err(result.into())
    }
}

#[derive(Serialize)]
struct FetchArgs{
    url: String,
}
=======
#[wasm_bindgen(module = "/public/invoke.js")]
extern "C" {
    #[wasm_bindgen(js_name = get_todo_list,catch)]
    async fn get_todo_list() -> Result<JsValue, JsValue>;
}

fn convert_js_value(result: Result<JsValue, JsValue>) -> Result<Vec<Todo>, JsValue> {
    match result {
        Ok(js_value) => {
            let todos: Vec<Todo> = serde_wasm_bindgen::from_value(js_value)
                .map_err(|err| JsValue::from_str(&err.to_string()))?;
            Ok(todos)
        }
        Err(err) => Err(err),
    }
}
>>>>>>> 533091ee7e53da252489ac411bf2e08a237ef312
