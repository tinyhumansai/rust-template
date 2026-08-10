//! Unit tests for the greeting module.

use super::*;

#[test]
fn greets_a_named_person() {
    assert_eq!(greet("Ferris").unwrap(), "Hello, Ferris!");
}

#[test]
fn trims_the_name() {
    assert_eq!(greet("  Ferris  ").unwrap(), "Hello, Ferris!");
}

#[test]
fn rejects_an_empty_name() {
    assert_eq!(greet("  ").unwrap_err(), Error::EmptyName);
}
