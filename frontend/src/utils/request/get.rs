use serde::Serialize;
use wasm_bindgen::JsValue;
use serde_wasm_bindgen::{to_value, Error};
#[derive(Serialize)]
pub struct GetArgs {
    pub url: String,
}

pub trait GetArgsToJsValue {
    fn new(url: String) -> Self;
    fn url_to_js_value(url: String) -> Result<JsValue, Error>;
}

impl GetArgsToJsValue for GetArgs {
    fn new(url: String) -> Self {
        Self { url }
    }

    fn url_to_js_value(url: String) -> Result<JsValue, Error> {
        to_value(&GetArgs::new(url))
    }
}