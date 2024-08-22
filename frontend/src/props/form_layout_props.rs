use yew::{Callback, Html, Properties, SubmitEvent};

#[derive(Properties, PartialEq, Clone)]
pub struct FormLayoutProps {
    pub label: String,
    pub onsubmit: Callback<SubmitEvent>,
    pub required: bool,
    pub child: Html,
}

impl FormLayoutProps {
    pub fn new(
        label: String,
        onsubmit: Callback<SubmitEvent>,
        required: bool,
        child: Html,
    ) -> Self {
        Self {
            label,
            onsubmit,
            required,
            child,
        }
    }
}
