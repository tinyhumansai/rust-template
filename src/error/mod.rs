//! Crate-wide error and result types.

/// Errors returned by this crate.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum Error {
    /// A required name was empty.
    #[error("name must not be empty")]
    EmptyName,
}

/// The crate's standard result type.
pub type Result<T> = std::result::Result<T, Error>;
