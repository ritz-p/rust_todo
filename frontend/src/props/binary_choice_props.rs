use patternfly_yew::components::button::{ButtonType, ButtonVariant};
use yew::{Callback, MouseEvent, Properties};

#[derive(Properties, PartialEq, Clone)]
pub struct BinaryChoiceProps {
    pub first_button_label: String,
    pub first_button_type: ButtonType,
    pub first_button_variant: ButtonVariant,
    pub first_button_onclick: Option<Callback<MouseEvent>>,
    pub second_button_label: String,
    pub second_button_type: ButtonType,
    pub second_button_variant: ButtonVariant,
    pub second_button_onclick: Option<Callback<MouseEvent>>,
}

impl BinaryChoiceProps {
    pub fn new(
        first_button_label: String,
        first_button_type: ButtonType,
        first_button_variant: ButtonVariant,
        first_button_onclick: Option<Callback<MouseEvent>>,
        second_button_label: String,
        second_button_type: ButtonType,
        second_button_variant: ButtonVariant,
        second_button_onclick: Option<Callback<MouseEvent>>,
    ) -> Self {
        Self {
            first_button_label,
            first_button_type,
            first_button_variant,
            first_button_onclick,
            second_button_label,
            second_button_type,
            second_button_variant,
            second_button_onclick,
        }
    }
}
