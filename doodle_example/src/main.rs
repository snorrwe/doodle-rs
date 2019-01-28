extern crate doodle;
#[macro_use]
extern crate serde_json;

use doodle::*;

#[derive(Schema)]
struct Woods {
    pub id: i32,
    pub name: String,
    pub epic: Vec<Tale>,
    pub generics: Generic<i32>,
}

#[derive(Schema)]
struct Tale {
    pub id: f32,
    pub value: &'static str,
}

#[derive(Schema)]
struct Generic<T> {
    pub value: T,
}

fn main() {
    // Initialize an empty dict
    let mut schema = json! {{}};
    let map = schema.as_object_mut().unwrap();

    // Append our schemas
    Woods::append_to_schema(map);
    Tale::append_to_schema(map);
    // Rust requires a type specification for generics
    // This however won't show up in the schema definition
    Generic::<u8>::append_to_schema(map);

    let output = serde_json::to_string_pretty(&schema).unwrap();
    println!("Schema:\n\n{}", output);
}

