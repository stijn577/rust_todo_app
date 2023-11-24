use js_sys::wasm_bindgen::JsValue;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Person {
    name: String,
    age: u8,
}

impl Person {
    pub fn new(name: impl Into<String>, age: u8) -> Self {
        Self {
            name: name.into(),
            age,
        }
    }
}

impl From<Person> for js_sys::wasm_bindgen::JsValue {
    fn from(value: Person) -> Self {
     JsValue::from_str(format!("{:?}", value).as_str())
    }
}
