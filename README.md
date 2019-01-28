# Doodle

[![Build Status](https://travis-ci.org/snorrwe/doodle-rs.svg?branch=master)](https://travis-ci.org/snorrwe/doodle-rs)
![[doodle](https://crates.io/crates/doodle)](https://img.shields.io/crates/v/doodle.svg?label=doodle)
![[doodle_derive](https://crates.io/crates/doodle_derive)](https://img.shields.io/crates/v/doodle_derive.svg?label=doodle_derive)
![[Documentation](https://docs.rs/doodle/)](https://docs.rs/doodle/badge.svg)
![[Licence](https://github.com/snorrwe/doodle-rs/blob/master/LICENSE)](https://img.shields.io/github/license/snorrwe/doodle-rs.svg)

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
extern crate serde_json;

use doodle::*;
use serde_json::json;

#[derive(Schema)]
struct Woods {
    pub id: i32,
    pub epic: Vec<String>,
}

fn main() {
    // Initialize an empty dict
    let mut schema = json! {{}};
    let map = schema.as_object_mut().unwrap();

    // Append our schemas
    Woods::append_to_schema(map);

    // print the output
    let output = serde_json::to_string_pretty(&schema).unwrap();
    println!("Schema:\n\n{}", output);
}
```


__Outputs__

```
{
    "Woods": {
        "type": "object",
        "properties": {
            "id": {
                "type": "i32"
            },
            "epic": {
                "type": "Vec < String >"
            }
        },
    }
}
```
