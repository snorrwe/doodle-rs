use serde_json::map::Map;
use serde_json::Value;
use std::collections::HashMap;

/// Used to retreive meta information for a struct
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
            .map(|f: &Field| (f.name, json!({ "type": f.json_ty })))
            .collect::<HashMap<_, _>>();
        json!({
            "type": "object",
            "properties": properties
        })
    }

    /// Append the definition of Self to an existing json dictionary
    fn append_to_schema(schema: &mut Map<String, Value>) {
        schema.insert(Self::get_name().to_string(), Self::get_fields_openapi());
    }
}

#[derive(Debug, Clone)]
pub struct Field {
    pub name: &'static str,
    pub json_ty: &'static str,
}

