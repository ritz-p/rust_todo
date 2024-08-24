use yew::{Callback, Html, Properties, SubmitEvent};

#[derive(Properties, PartialEq, Clone)]
pub struct FormLayoutProps {
    pub label: String,
    pub onsubmit: Callback<SubmitEvent>,
    pub required: bool,
    pub onchange: Callback<String>,
    pub value: String,
    pub placeholder: String,
    pub child: Html,
}

impl FormLayoutProps {
    pub fn new(
        label: String,
        onsubmit: Callback<SubmitEvent>,
        required: bool,
        onchange: Callback<String>,
        value: String,
        placeholder: String,
        child: Html,
    ) -> Self {
        Self {
            label,
            onsubmit,
            required,
            onchange,
            value,
            placeholder,
            child,
        }
    }
}
