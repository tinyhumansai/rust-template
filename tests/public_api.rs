//! Integration tests for the public crate surface.

use rust_template::greet;

#[test]
fn greeting_is_available_to_consumers() {
    assert_eq!(greet("Rust").unwrap(), "Hello, Rust!");
}
