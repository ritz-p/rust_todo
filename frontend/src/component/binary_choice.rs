use patternfly_yew::components::{button::Button, form::ActionGroup};
use yew::{function_component, html, Callback, Html};

use crate::props::binary_choice_props::BinaryChoiceProps;

#[function_component]
pub fn BinaryChoice(props: &BinaryChoiceProps) -> Html {
    html!(
        <ActionGroup>
            <Button
                label={props.first_button_label.clone()}
                onclick={props.second_button_onclick.clone().unwrap_or_else(|| Callback::from(|_| {}))}
                variant={props.first_button_variant}
                r#type={props.first_button_type}
            />
            <Button
                label={props.second_button_label.clone()}
                onclick={props.second_button_onclick.clone().unwrap_or_else(|| Callback::from(|_| {}))}
                variant={props.second_button_variant}
                r#type={props.second_button_type}
            />
        </ActionGroup>
    )
}
