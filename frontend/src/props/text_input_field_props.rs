use yew::{Callback, InputEvent, Properties};

#[derive(Properties, PartialEq, Clone)]
pub struct TextInputFieldProps {
    pub value: String,
    pub placeholder: String,
    pub oninput: Callback<InputEvent>,
    pub required: bool,
}

impl TextInputFieldProps{
    pub fn new(value: String,placeholder: String,oninput: Callback<InputEvent>,required: bool) -> Self{
        Self{
            value,
            placeholder,
            oninput,
            required
        }
    }
}