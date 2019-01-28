#[macro_use]
extern crate serde_json;
extern crate doodle_derive;

pub use doodle_derive::*;

use self::serde_json::Value;
use std::collections::HashMap;

pub trait Schema {
    /// Return the (name, type) of every field
    fn get_fields() -> &'static [(&'static str, &'static str)];

    /// Return a json representation of the schema
    fn get_fields_openapi() -> Value {
        let map = Self::get_fields()
            .iter()
            .map(|x| *x)
            .map(|(k, v)| (k, json!({ "type": v })))
            .collect::<HashMap<_, _>>();
        json!(map)
    }
}
