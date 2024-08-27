use std::rc::Rc;

use crate::component::accordion_layout::AccordionLayout;
use crate::props::accordion_layout_props::{AccordionItemProps, AccordionLayoutProps};
use crate::utils::{js_bind::FromJsValue, wasm::invoke};
use crate::{
    props::todo_list_props::TodoListProps,
    utils::request::get::{GetArgs, GetArgsToJsValue},
};
use patternfly_yew::components::{
    page::{PageSection, PageSectionGroup},
    title::{Level, Title},
};
use patternfly_yew::core::Size;
use serde_wasm_bindgen::Error;
use shared_struct::todo::mount::object::todo::Todo;
use wasm_bindgen_futures::spawn_local;
use web_sys::console;
use yew::prelude::*;

#[function_component]
pub fn TodoList(props: &TodoListProps) -> Html {
    let todo_list = use_state(Vec::new);
    let url = Rc::new(props.url.clone());
    let function = props.function.clone();
    {
        let todo_list = todo_list.clone();
        let url = url.clone();
        use_effect_with(true, move |_| {
            spawn_local(async move {
                let args = GetArgs::url_to_js_value(url.clone().to_string());
                match args {
                    Ok(serialized_args) => {
                        console::log_1(&serialized_args);
                        let response: Result<Vec<Todo>, Error> =
                            FromJsValue::from_js_value(invoke(&function, serialized_args).await);
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

    let accordion_item_props_list:Vec<AccordionItemProps> = (todo_list).iter().enumerate().map(|(index,todo)| {
        AccordionItemProps::new(todo.text.clone(), index.to_string())
    }).collect();
    let accordion_layout_props = AccordionLayoutProps::new(false, false, accordion_item_props_list);
    html!(
        <>
            <Title level={Level::H5} size={Size::Large}>{"Todo List"}</Title>
            <PageSectionGroup>
                <PageSection>
                    <AccordionLayout ..accordion_layout_props></AccordionLayout>
                </PageSection>
            </PageSectionGroup>
        </>
    )
}
