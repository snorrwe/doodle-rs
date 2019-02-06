extern crate doodle;
#[macro_use]
extern crate serde_json;

use doodle::*;

#[derive(Schema)]
struct Woods {
    pub id: i32,
    pub epic: Vec<Tale>,
}

#[derive(Schema)]
struct Tale {
    pub name: Option<String>,
}

fn main() {
    // Initialize an empty dict
    let mut schema = json! {{}};
    let map = schema.as_object_mut().unwrap();

    // Append our schemas
    Woods::append_to_schema(map);
    Tale::append_to_schema(map);

    let output = serde_json::to_string_pretty(&schema).unwrap();
    println!("Schema:\n\n{}", output);
}

