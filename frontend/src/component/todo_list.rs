use crate::props::todo_list_props::TodoListProps;
use patternfly_yew::components::{
    list::{List, ListItem, ListType},
    page::{PageSection, PageSectionGroup},
};

use gloo::console;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
#[function_component]
pub fn TodoList(props: &TodoListProps) -> Html {
    let todo_list = use_state(|| props.todo_list.clone());
    let endpoint = props.endpoint.clone();
    let response_data = use_state(|| "response".to_string());
    let send = {
        let response_data = response_data.clone();
        move || {
            let response_data = response_data.clone();
            console::log!("send");
            spawn_local(async move {
                let client = reqwest::Client::new();
                let req = client
                    .get(endpoint)
                    .send()
                    .await
                    .expect("failed to send request");

                console::log!(format!("headers: {:?}", req.headers()));

                let data = req.text().await.expect("failed to get response");

                response_data.set(format!("{:?}", data));
            });
        }
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
