use serde::{de::DeserializeOwned, Serialize};
use serde_wasm_bindgen::{to_value, Error};
use wasm_bindgen::JsValue;

#[derive(Serialize)]
pub struct FetchArgs {
    pub url: String,
}

#[derive(Serialize)]
pub struct PostArgs<T>
where
    T: Serialize + DeserializeOwned,
{
    pub url: String,
    pub body: T,
}

pub trait FetchArgsToJsValue {
    fn url_to_js_value(url: String) -> Result<JsValue, Error>;
}

pub trait PostArgsToJsValue<T> {
    fn url_to_js_value(url: String,body: T) -> Result<JsValue, Error>
    where
        T: DeserializeOwned;
}

impl FetchArgs {
    fn new(url: String) -> Self {
        Self { url }
    }
}

impl<T> PostArgs<T>
where
    T: Serialize + DeserializeOwned,
{
    pub fn new(url: String, body: T) -> Self {
        Self { url, body }
    }
}

impl FetchArgsToJsValue for FetchArgs {
    fn url_to_js_value(url: String) -> Result<JsValue, Error> {
        to_value(&FetchArgs::new(url))
    }
}

impl<T> PostArgsToJsValue<T> for PostArgs<T>
where
    T: Serialize + DeserializeOwned,
{
    fn url_to_js_value(url: String,body: T) -> Result<JsValue, Error> {
        to_value(&PostArgs::new(url,body))
    }
}
