use crate::props::todo_list_props::TodoListProps;
use patternfly_yew::components::{
    list::{List, ListItem, ListType},
    page::{PageSection, PageSectionGroup},
};

use log::error;
use reqwest::Client;
use shared_struct::todo::mount::object::todo::Todo;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
#[function_component]
pub fn TodoList(props: &TodoListProps) -> Html {
    let todo_list = props.todo_list.clone();
    let endpoint = props.endpoint.clone();
    {
        let todo_list = todo_list.clone();
        use_effect(move || {
            spawn_local(async move {
                let client = Client::new();
                let response = client.get(&endpoint).send().await;
                match response {
                    Ok(data) => {
                        let text = data.text().await;
                        match text {
                            Ok(json) => match serde_json::from_str::<Vec<Todo>>(&json) {
                                Ok(todo_list_json) => todo_list.set(todo_list_json),
                                Err(err) => error!("Error parsing todo list: {:?}", err),
                            },
                            Err(err) => error!("Error fetching text:{:?}", err),
                        }
                    }
                    Err(err) => error!("Error fetching data:{:?}", err),
                }
            })
        });
    };
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
