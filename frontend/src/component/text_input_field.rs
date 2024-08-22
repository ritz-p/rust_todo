use crate::props::text_input_field_props::TextInputFieldProps;

use patternfly_yew::components::{
    form::TextInputType,
    text_input_group::{TextInputGroup, TextInputGroupMain},
};
use yew::prelude::*;

#[function_component(TextInputField)]
pub fn text_input_field(props: &TextInputFieldProps) -> Html {
    html! {
        <TextInputGroup>
            <TextInputGroupMain
                value={props.value.clone()}
                placeholder={props.placeholder.clone()}
                oninput={props.oninput.clone()}
                r#type={TextInputType::Text}>
            </TextInputGroupMain>
        </TextInputGroup>
    }
}
