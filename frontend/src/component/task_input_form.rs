use crate::component::binary_choice::BinaryChoice;
use crate::component::form_layout::FormLayout;
use crate::props::binary_choice_props::BinaryChoiceProps;
use crate::props::form_layout_props::FormLayoutProps;
use crate::utils::wasm::invoke;
use crate::{
    props::task_input_form_props::TextInputFormProps,
    utils::request::post::{PostArgs, PostArgsToJsValue},
};

use patternfly_yew::{
    components::page::{PageSection, PageSectionGroup},
    prelude::*,
};
use shared_struct::todo::mount::object::create_todo::CreateTodo;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use web_sys::{console, window};
use yew::prelude::*;

#[function_component]
pub fn TextInputForm(props: &TextInputFormProps) -> Html {
    let text_input = use_state_eq(|| props.text_input.clone());
    let onchange = use_callback(text_input.clone(), |new_text_input, text_input| {
        text_input.set(new_text_input);
    });
    let url = props.url.clone();
    let function = props.function.clone();
    let onreset = use_callback(text_input.clone(), |_, text_input| {
        text_input.set("".to_string());
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
    let binary_choice = html! {
        <BinaryChoice ..binary_choice_props></BinaryChoice>
    };
    let onsubmit = {
        let text_input = text_input.clone();
        let onsubmit = props.onsubmit.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            console::log_1(&JsValue::from_str("Before emit create todo"));
            onsubmit.emit((*text_input).clone());
            let new_todo = CreateTodo::new((*text_input).clone());
            let args = PostArgs::url_to_js_value(url.clone(), new_todo);
            let function = function.clone();
            spawn_local(async move {
                match args {
                    Ok(serialized_args) => {
                        console::log_1(&JsValue::from_str("Post request serialized args"));
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
    let form_layout_props = FormLayoutProps::new(
        "Task Name".to_string(),
        onsubmit,
        true,
        onchange,
        (*text_input).clone(),
        "Enter task name".to_string(),
        binary_choice,
    );
    html!(
        <PageSectionGroup>
            <PageSection>
                <FormLayout ..form_layout_props></FormLayout>
            </PageSection>
        </PageSectionGroup>
    )
}
