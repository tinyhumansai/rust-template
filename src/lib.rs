//! A small starting point for a production Rust library.

mod error;

pub use error::{Error, Result};

/// Returns a friendly greeting for `name`.
///
/// # Errors
///
/// Returns [`Error::EmptyName`] when `name` is empty or only whitespace.
pub fn greet(name: &str) -> Result<String> {
    let name = name.trim();
    if name.is_empty() {
        return Err(Error::EmptyName);
    }

    Ok(format!("Hello, {name}!"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greets_a_named_person() {
        assert_eq!(greet("Ferris").unwrap(), "Hello, Ferris!");
    }

    #[test]
    fn rejects_an_empty_name() {
        assert_eq!(greet("  ").unwrap_err(), Error::EmptyName);
    }
}
