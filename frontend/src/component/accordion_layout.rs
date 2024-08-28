use std::collections::HashSet;

use patternfly_yew::components::{
    accordion::{Accordion, AccordionItem},
    button::{ButtonType, ButtonVariant},
};
use yew::{function_component, html, html_nested, use_state, virtual_dom::VChild, Callback, Html};

use crate::{
    component::single_button::SingleButton,
    props::{accordion_layout_props::AccordionLayoutProps, single_button_props::SingleButtonProps},
};

#[function_component]
pub fn AccordionLayout(props: &AccordionLayoutProps) -> Html {
    let accordion_item_list = props.accordion_item_list.clone();
    let state = use_state(HashSet::<usize>::new);
    let toggle = |key: usize| {
        let state = state.clone();
        Callback::from(move |_| {
            let mut selected = (*state).clone();
            if selected.contains(&key) {
                selected.remove(&key);
            } else {
                selected.insert(key);
            }
            state.set(selected);
        })
    };
    html!(
        <Accordion bordered={props.bordered.clone()} large={props.large.clone()}>
            { for accordion_item_list.iter().enumerate().map(|(index, item_props)| {
                let single_button_props = SingleButtonProps::new("Delete".to_string(),ButtonType::Button,ButtonVariant::Danger,"http://localhost:3001/todos/".to_string() + &item_props.task_id.to_string());
                let item: VChild<AccordionItem> = html_nested! {
                    <AccordionItem
                        title={item_props.title.clone()}
                        onclick={toggle(index)}
                        expanded={state.contains(&index)}
                    >
                        {item_props.content.clone()}
                        <SingleButton ..single_button_props>
                        </SingleButton>
                    </AccordionItem>
                }.into();
                item
            }) }
        </Accordion>
    )
}
