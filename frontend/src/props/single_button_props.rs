use patternfly_yew::components::button::{ButtonType, ButtonVariant};
use yew::{Callback, MouseEvent, Properties};

#[derive(Properties, PartialEq, Clone)]
pub struct SingleButtonProps {
    pub label: String,
    pub button_type: ButtonType,
    pub button_variant: ButtonVariant,
    pub url: String,
}
