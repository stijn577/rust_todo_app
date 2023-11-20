use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Person {
    name: String,
    age: u8,
}

impl Person {
    pub fn new(name: impl Into<String>, age: u8) -> Self {
        Self { name: name.into(), age }
    }
}
