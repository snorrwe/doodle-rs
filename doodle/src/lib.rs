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
#[macro_use]
extern crate serde_json;

// Reexport doodle_derive
#[cfg(feature = "doodle_derive")]
#[macro_use]
#[allow(unused_imports)]
pub extern crate doodle_derive;

#[cfg(feature = "doodle_derive")]
pub use doodle_derive::*;

pub mod traits;
pub use self::traits::*;

