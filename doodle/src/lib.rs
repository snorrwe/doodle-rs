#[macro_use]
extern crate serde_json;

// Reexport doodle_derive
#[cfg(feature = "doodle_derive")]
#[macro_use]
#[allow(unused_imports)]
pub extern crate doodle_derive;

#[cfg(feature = "doodle_derive")]
pub use doodle_derive::*;

use serde_json::Value;
use std::collections::HashMap;

pub trait Schema {
    /// Return the (name, type) of every field
    fn get_fields() -> &'static [(&'static str, &'static str)];

    /// Return a json representation of the schema
    fn get_fields_openapi() -> Value {
        let properties = Self::get_fields()
            .iter()
            .map(|(k, v)| (k, json!({ "type": v })))
            .collect::<HashMap<_, _>>();
        json!({
            "type": "object",
            "properties": properties
        })
    }
}

