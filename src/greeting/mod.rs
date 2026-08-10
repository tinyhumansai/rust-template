//! Greeting behavior used to demonstrate the template's module layout.

use crate::{Error, Result};

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
mod test;
