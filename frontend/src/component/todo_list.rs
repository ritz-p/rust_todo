use crate::utils::request_struct::FetchArgsToJsValue;
use crate::{props::todo_list_props::TodoListProps, utils::request_struct::FetchArgs};
use crate::utils::{wasm::invoke,js_bind::FromJsValue};
use patternfly_yew::components::{
    list::{List, ListItem, ListType},
    page::{PageSection, PageSectionGroup},
};
use serde_wasm_bindgen::Error;
use shared_struct::todo::mount::object::todo::Todo;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[function_component]
pub fn TodoList(props: &TodoListProps) -> Html {
    let todo_list = use_state(Vec::new);
    let url = props.url.clone();
    let function = props.function.clone();
    {
        let todo_list = todo_list.clone();
        use_effect_with(true, move |_| {
            spawn_local(async move {
                let args = FetchArgs::url_to_js_value(url);
                match args {
                    Ok(serialized_args) => {
                        let response: Result<Vec<Todo>,Error> = FromJsValue::from_js_value(invoke(&function, serialized_args).await);
                        match response {
                            Ok(new_todo_list) => {
                                todo_list.set(new_todo_list);
                            }
                            Err(err) => {
                                println!("{}", err);
                            }
                        }
                    }
                    Err(err) => {
                        println!("{}", err);
                    }
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



