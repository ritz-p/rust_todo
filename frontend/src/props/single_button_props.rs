use patternfly_yew::components::button::{ButtonType, ButtonVariant};
use yew::Properties;

#[derive(Properties, PartialEq, Clone)]
pub struct SingleButtonProps {
    pub label: String,
    pub button_type: ButtonType,
    pub button_variant: ButtonVariant,
    pub url: String,
}

impl SingleButtonProps {
    pub fn new(
        label: String,
        button_type: ButtonType,
        button_variant: ButtonVariant,
        url: String,
    ) -> Self {
        Self {
            label,
            button_type,
            button_variant,
            url,
        }
    }
}
