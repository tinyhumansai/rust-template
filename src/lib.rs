//! A small starting point for a production Rust library.

mod error;
mod greeting;

pub use error::{Error, Result};
pub use greeting::greet;
