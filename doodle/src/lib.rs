//! # Doodle
//!
//! The *Doodle* library provides means for data structures to document themselves loosely
//! following the [OpenAPI specification](https://swagger.io/docs/specification/) specification
//!
//! Types that implement the `Schema` trait can document themselves using the methods available.
//! See the `Schema` trait for more info
//!
//! The crate also re-exports `doodle_derive` which provides the derive `Schema` which implements
//! The neccessary methods to serialize the data structure
//!
//! ## Usage example
//!
//! __Cargo.toml__:
//!
//! ```toml
//! [dependencies]
//! serde_json = "1.0.0"
//! doodle = { version = "0.1.1", features = ["derive"] }
//! ```
//!
//! ```
//! use doodle::*;
//! use serde_json::json;
//!
//! #[derive(Schema)]
//! struct Woods {
//!     pub id: i32,
//!     pub epic: Vec<Vec<Tale>>,
//! }
//!
//! #[derive(Schema)]
//! struct Tale {
//!     pub value: Option<String>,
//! }
//!
//! // Initialize an empty dict
//! let mut schema = json! {{}};
//! let map = schema.as_object_mut().unwrap();
//!
//! // Append our schemas
//! Woods::append_to_schema(map);
//! Tale::append_to_schema(map);
//!
//! // print the output
//! let output = serde_json::to_string_pretty(&schema).unwrap();
//! println!("Schema:\n\n{}", output);
//!
//!
//! ////////////////////// Test the example //////////////////////
//!
//!
//! let expected = json! {{
//! "Tale": {                                         
//!   "properties": {                                 
//!     "value": {                                    
//!       "type": "string",                            
//!       "nullable": true
//!     }                                             
//!   },                                              
//!   "type": "object"                                
//! },                                                
//! "Woods": {                                        
//!   "properties": {                                 
//!     "epic": {                                     
//!       "items": {                                  
//!         "items": {                                
//!           "$ref": "#/components/schemas/Tale"     
//!         },                                        
//!         "type": "array"                           
//!       },                                          
//!       "type": "array"                             
//!     },                                            
//!     "id": {                                       
//!       "type": "number"                            
//!     }                                             
//!   },                                              
//!   "type": "object"                                
//! }                                                 
//! }};
//!
//! assert_eq!(expected, schema);
//!
//! ```

// Reexport serde_json
// Not public API.
#[macro_use]
#[allow(unused_imports)]
pub extern crate serde_json;

// Reexport `Value` which is used by our traits
pub use serde_json::Value;

// Reexport doodle_derive
#[cfg(feature = "doodle_derive")]
#[macro_use]
#[allow(unused_imports)]
pub extern crate doodle_derive;

#[cfg(feature = "doodle_derive")]
pub use doodle_derive::*;

use serde_json::map::Map;
use std::collections::HashMap;

/// Used to retreive meta information for a `struct`
pub trait SchemaMeta {
    /// Return the (name, type) of every field
    fn get_fields() -> &'static [Field];

    /// Return the name of Self
    fn get_name() -> &'static str;
}

/// Provides methods to get the `serde_json::Value` representations of the type schema
pub trait Schema: SchemaMeta {
    /// Return a json representation of the schema
    fn get_fields_openapi() -> Value {
        let properties = Self::get_fields()
            .iter()
            .map(|f: &Field| (f.name, f.json_ty.to_json()))
            .collect::<HashMap<_, _>>();
        json!({
            "type": "object",
            "properties": properties
        })
    }

    /// Append the definition of Self to an existing json dictionary
    fn append_to_schema(schema: &mut Map<String, Value>) {
        let key = Self::get_name().to_string();
        let fields = Self::get_fields_openapi();
        schema.insert(key, fields);
    }
}

/// Represents meta information of a single field of a struct
#[derive(Debug, Clone)]
pub struct Field {
    pub name: &'static str,
    pub json_ty: Type,
}

#[derive(Debug, Clone)]
pub enum Type {
    Simple(&'static str),
    Ref(&'static str),
    Array(&'static Type),
    Nullable(&'static Type),
}

impl Type {
    pub fn to_json(&self) -> Value {
        match self {
            Type::Array(inner_type) => json!({
                "type": "array",
                "items": inner_type.to_json()
            }),
            Type::Simple(ty) => json!({ "type": ty }),
            Type::Ref(ty) => json!({ "$ref": format!("#/components/schemas/{}", ty) }),
            Type::Nullable(ty) => {
                let mut json = ty.to_json();
                json["nullable"] = json!(true);
                json
            }
        }
    }
}

