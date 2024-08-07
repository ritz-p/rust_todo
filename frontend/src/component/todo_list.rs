// use crate::http_client::HttpClient;
use crate::props::todo_list_props::TodoListProps;
use patternfly_yew::components::{
    list::{List, ListItem, ListType},
    page::{PageSection, PageSectionGroup},
};
use shared_struct::todo::mount::object::todo::Todo;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[function_component]
pub fn TodoList(props: &TodoListProps) -> Html {
    let todo_list = props.todo_list.clone();
    let endpoint = props.endpoint.clone();
    // let http_client = HttpClient::new("http://localhost:8080");
    // {
    //     let todo_list = todo_list.clone();
    //     use_effect_with(true, move |_| {
    //         spawn_local(async move {
    //             let response = http_client.get(&endpoint).await;
    //             match response {
    //                 Ok(text) => match serde_json::from_str::<Vec<Todo>>(&text) {
    //                     Ok(todo_list_json) => todo_list.set(todo_list_json),
    //                     Err(err) => error!("Error parsing todo list: {:?}", err),
    //                 },
    //                 Err(err) => error!("Error fetching data:{:?}", err),
    //             }
    //         })
    //     });
    // }
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
