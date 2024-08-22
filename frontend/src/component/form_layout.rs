use patternfly_yew::components::form::{Form, FormGroup};
use yew::{function_component, html, Html};

use crate::props::form_layout_props::FormLayoutProps;

#[function_component]
pub fn FormLayout(props: &FormLayoutProps) -> Html {
    html!(
        <Form onsubmit={props.onsubmit.clone()}>
            <FormGroup
                label={props.label.clone()}
                required={props.required.clone()}
            >
                {props.child.clone()}
            </FormGroup>
        </Form>
    )
}
