use crate::props::todo_list_props::TodoListProps;
use patternfly_yew::components::{
        page::{PageSection,PageSectionGroup},
        list::{List,ListType,ListItem}
    };
use yew::prelude::*;
#[function_component]
pub fn TodoList(props: &TodoListProps) -> Html{
    let todo_list = props.todo_list.clone();

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