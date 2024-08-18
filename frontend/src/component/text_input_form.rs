use crate::utils::js_bind::FromJsValue;
use crate::utils::request_struct::PostArgsToJsValue;
use crate::utils::wasm::invoke;
use crate::{props::task_input_form_props::TextInputFormProps, utils::request_struct::PostArgs};

use patternfly_yew::{
    components::{
        button::Button,
        form::{ActionGroup, Form, FormGroup},
        page::{PageSection, PageSectionGroup},
    },
    prelude::*,
};
use shared_struct::todo::mount::object::create_todo::CreateTodo;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[function_component]
pub fn TextInputForm(props: &TextInputFormProps) -> Html {
    let url = props.url.clone();
    let function = props.function.clone();
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
            let new_todo = CreateTodo::new((*text_input).clone());
            let args =
                PostArgs::url_to_js_value(PostArgs::new(url.clone(), new_todo), "todo".to_string());
            let function = function.clone();
            spawn_local(async move {
                match args {
                    Ok(serialized_args) => {
                        invoke(&function, serialized_args).await;
                    }
                    Err(_) => todo!(),
                }
            })
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
