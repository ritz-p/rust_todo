use serde::{de::DeserializeOwned, Serialize};
use serde_wasm_bindgen::{to_value, Error};
use wasm_bindgen::JsValue;

#[derive(Serialize)]
pub struct PostArgs<T>
where
    T: Serialize + DeserializeOwned,
{
    pub url: String,
    pub body: T,
}
pub trait PostArgsToJsValue<T> {
    fn url_to_js_value(url: String, body: T) -> Result<JsValue, Error>
    where
        T: DeserializeOwned;
}

impl<T> PostArgs<T>
where
    T: Serialize + DeserializeOwned,
{
    pub fn new(url: String, body: T) -> Self {
        Self { url, body }
    }
}

impl<T> PostArgsToJsValue<T> for PostArgs<T>
where
    T: Serialize + DeserializeOwned,
{
    fn url_to_js_value(url: String, body: T) -> Result<JsValue, Error> {
        to_value(&PostArgs::new(url, body))
    }
}
