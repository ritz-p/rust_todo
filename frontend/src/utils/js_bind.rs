use serde::de::DeserializeOwned;
use serde_wasm_bindgen::Error;
use wasm_bindgen::prelude::*;

pub trait FromJsValue {
    fn from_js_value(js_value: JsValue) -> Result<Self, Error>
    where
        Self: Sized;
}

impl<T> FromJsValue for T
where
    T: DeserializeOwned,
{
    fn from_js_value(js_value: JsValue) -> Result<T, Error> {
        if js_value.is_object() {
            let converted = serde_wasm_bindgen::from_value(js_value)
                .map_err(|err| JsValue::from_str(&err.to_string()))?;
            Ok(converted)
        } else {
            Err(js_value.into())
        }
    }
}
