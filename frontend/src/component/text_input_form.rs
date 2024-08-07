use crate::props::task_input_form_props::TextInputFormProps;
use patternfly_yew::{
    components::{
        button::Button,
        form::{ActionGroup, Form, FormGroup},
        page::{PageSection, PageSectionGroup},
    },
    prelude::*,
};
use yew::prelude::*;

#[function_component]
pub fn TextInputForm(props: &TextInputFormProps) -> Html {
    let text_input = use_state_eq(|| props.text_input.clone());
    let onchange = use_callback(text_input.clone(), |new_text_input, text_input| {
        text_input.set(new_text_input)
    });
    let onreset = use_callback(text_input.clone(), |_, text_input| {
        text_input.set("".to_string())
    });
    let onsubmit = {
        let text_input = text_input.clone();
        let onsubmit = props.onsubmit.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            onsubmit.emit((*text_input).clone());
            text_input.set(String::new());
        })
    };
    html!(
        <PageSectionGroup>
            <PageSection>
                <Form onsubmit={onsubmit}>
                    <FormGroup
                        label={props.form_label.clone()}
                        required=true
                    >
                        <TextInput {onchange} value={(*text_input).clone()} placeholder={props.text_input_placeholder.clone()} required=true/>
                    </FormGroup>
                    <ActionGroup>
                        <Button variant={ButtonVariant::Primary} label="Submit" r#type={ButtonType::Submit}/>
                        <Button onclick={onreset} variant={ButtonVariant::Secondary} label="Reset" r#type={ButtonType::Reset}/>
                    </ActionGroup>
                </Form>
            </PageSection>
        </PageSectionGroup>
    )
}
