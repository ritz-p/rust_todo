use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct TextInputFormProps {
    pub url: String,
    pub function: String,
    pub form_label: String,
    pub text_input: String,
    pub text_input_placeholder: String,
    pub submit_label: String,
    pub reset_lable: String,
    pub onsubmit: Callback<String>,
}
