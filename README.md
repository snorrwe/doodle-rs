# Doodle

[![Build Status](https://travis-ci.org/snorrwe/doodle-rs.svg?branch=master)](https://travis-ci.org/snorrwe/doodle-rs)
[![doodle](https://img.shields.io/crates/v/doodle.svg?label=doodle)](https://crates.io/crates/doodle)
[![doodle_derive](https://img.shields.io/crates/v/doodle_derive.svg?label=doodle_derive)](https://crates.io/crates/doodle_derive)
[![Documentation](https://docs.rs/doodle/badge.svg)](https://docs.rs/doodle/)
[![Licence](https://img.shields.io/github/license/snorrwe/doodle-rs.svg)](https://github.com/snorrwe/doodle-rs/blob/master/LICENSE)

The *Doodle* library provides means for data structures to document themselves _loosely_
following the [OpenAPI specification](https://swagger.io/docs/specification/) specification

Types that implement the `Schema` trait can document themselves using the methods available.
See the `Schema` trait for more info

The crate also re-exports `doodle_derive` which provides the derive `Schema` which implements
The neccessary methods to serialize the data structure

## Usage example

__Cargo.toml__:

```toml
[dependencies]
doodle = { version = "0.1.1", features = ["derive"] }
serde_json = "1.0.0" # To pretty print our result
```

__main.rs__:

```
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

```


__Outputs__

```
Schema:

{
  "Tale": {
    "properties": {
      "name": {
        "nullable": true,
        "type": "string"
      }
    },
    "type": "object"
  },
  "Woods": {
    "properties": {
      "epic": {
        "items": {
          "$ref": "#/components/schemas/Tale"
        },
        "type": "array"
      },
      "id": {
        "type": "number"
      }
    },
    "type": "object"
  }
}
```
