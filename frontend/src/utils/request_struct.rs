use serde::{de::DeserializeOwned, Serialize, Serializer};
use serde_wasm_bindgen::{to_value, Error};
use wasm_bindgen::JsValue;

#[derive(Serialize)]
pub struct FetchArgs {
    pub url: String,
}

#[derive(Serialize)]
pub struct PostArgs<T>
where
    T: DeserializeOwned,
{
    pub url: String,
    pub todo: T,
}

impl FetchArgs {
    fn new(url: String) -> Self {
        Self { url }
    }
}

impl<T> PostArgs<T>
where
    T: DeserializeOwned,
{
    pub fn new(url: String, todo: T) -> Self {
        Self { url, todo }
    }
}

impl FetchArgsToJsValue for FetchArgs {
    fn url_to_js_value(url: String) -> Result<JsValue, Error> {
        to_value(&FetchArgs::new(url))
    }
}

impl<T> PostArgsToJsValue<T> for PostArgs<T>
where
    T: DeserializeOwned + Serialize,
{
    fn url_to_js_value(self, _key: String) -> Result<JsValue, Error> {
        to_value(&PostArgs::new(self.url, self.todo))
    }
}

pub trait FetchArgsToJsValue {
    fn url_to_js_value(url: String) -> Result<JsValue, Error>;
}

pub trait PostArgsToJsValue<T> {
    fn url_to_js_value(self, key: String) -> Result<JsValue, Error>
    where
        T: DeserializeOwned;
}
