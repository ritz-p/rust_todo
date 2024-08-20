use serde::{de::DeserializeOwned, Serialize};
use serde_wasm_bindgen::{to_value, Error};
use wasm_bindgen::JsValue;

#[derive(Serialize)]
pub struct PatchArgs<T>
where
    T: Serialize + DeserializeOwned,
{
    pub url: String,
    pub body: T,
}
pub trait PatchArgsToJsValue<T> {
    fn url_to_js_value(url: String,body: T) -> Result<JsValue, Error>
    where
        T: DeserializeOwned;
}

impl<T> PatchArgs<T>
where
    T: Serialize + DeserializeOwned,
{
    pub fn new(url: String, body: T) -> Self {
        Self { url, body }
    }
}

impl<T> PatchArgsToJsValue<T> for PatchArgs<T>
where
    T: Serialize + DeserializeOwned,
{
    fn url_to_js_value(url: String,body: T) -> Result<JsValue, Error> {
        to_value(&PatchArgs::new(url,body))
    }
}
