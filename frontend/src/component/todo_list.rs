use crate::props::todo_list_props::TodoListProps;
use patternfly_yew::components::{
    list::{List, ListItem, ListType},
    page::{PageSection, PageSectionGroup},
};
use serde_wasm_bindgen::from_value;
use shared_struct::todo::mount::object::todo::Todo;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[function_component]
pub fn TodoList(props: &TodoListProps) -> Html {
    let todo_list = use_state(Vec::new);
    let endpoint = props.endpoint.clone();
    {
        let todo_list = todo_list.clone();
        use_effect_with(true, move |_| {
            spawn_local(async move {
                let response = get_todo_list().await;
                match convert_js_value(response) {
                    Ok(new_todo_list) => {
                        todo_list.set(new_todo_list);
                    }
                    Err(_) => todo!(),
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
