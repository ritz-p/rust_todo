use crate::component::binary_choice::BinaryChoice;
use crate::component::form_layout::FormLayout;
use crate::component::text_input_field::TextInputField;
use crate::props::binary_choice_props::BinaryChoiceProps;
use crate::props::form_layout_props::FormLayoutProps;
use crate::props::text_input_field_props::TextInputFieldProps;
use crate::utils::wasm::invoke;
use crate::{
    props::task_input_form_props::TextInputFormProps,
    utils::request::post::{PostArgs, PostArgsToJsValue},
};

use patternfly_yew::{
    components::{
        form::{Form, FormGroup},
        page::{PageSection, PageSectionGroup},
    },
    prelude::*,
};
use shared_struct::todo::mount::object::create_todo::CreateTodo;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use web_sys::{console, window, HtmlInputElement};
use yew::prelude::*;

#[function_component]
pub fn TextInputForm(props: &TextInputFormProps) -> Html {
    let text_input = use_state(|| props.text_input.clone());
    let url = props.url.clone();
    let function = props.function.clone();
    let oninput = {
        let text_input = text_input.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            text_input.set(input.value())
        })
    };
    let onreset = use_callback(text_input.clone(), |_, text_input| {
        text_input.set("".to_string())
    });
    let binary_choice_props = BinaryChoiceProps::new(
        "Submit".to_string(),
        ButtonType::Submit,
        ButtonVariant::Primary,
        None,
        "Reset".to_string(),
        ButtonType::Reset,
        ButtonVariant::Secondary,
        Some(onreset),
    );
    let text_input_field_props = TextInputFieldProps::new(
        props.text_input.clone(),
        "Task Name".to_string(),
        oninput,
        true,
    );
    let text_input_field = html! {
        <TextInputField ..text_input_field_props />
    };
    let onsubmit = {
        let text_input = text_input.clone();
        let onsubmit = props.onsubmit.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            onsubmit.emit((*text_input).clone());
            let new_todo = CreateTodo::new((*text_input).clone());
            let args = PostArgs::url_to_js_value(url.clone(), new_todo);
            let function = function.clone();
            spawn_local(async move {
                match args {
                    Ok(serialized_args) => {
                        console::log_1(&serialized_args);
                        invoke(&function, serialized_args).await;
                        if let Some(window) = window() {
                            let reload = window.location().reload();
                            match reload {
                                Ok(_) => {
                                    console::log_1(&JsValue::from_str("Reload Succeeded"));
                                }
                                Err(err) => {
                                    console::error_1(&err);
                                }
                            }
                        }
                    }
                    Err(err) => {
                        console::error_1(&err.into());
                    }
                }
            })
        })
    };
    let form_layout_props =
        FormLayoutProps::new("Task Name".to_string(), onsubmit, true, text_input_field);
    html!(
        <PageSectionGroup>
            <PageSection>
                <FormLayout ..form_layout_props></FormLayout>
                <BinaryChoice ..binary_choice_props></BinaryChoice>
            </PageSection>
        </PageSectionGroup>
    )
}
