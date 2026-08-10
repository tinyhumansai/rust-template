# Repository Guidelines

## Project structure

This is a Rust 2024 library crate. Keep public exports in `src/lib.rs`, focused
implementation modules under `src/`, integration tests in `tests/`, runnable
examples in `examples/`, and architecture or module notes in `docs/`.

Prefer small modules with narrow responsibilities. Put shared error variants in
`src/error.rs` and return the crate-wide `Result<T>` from fallible public APIs.

## Build and test

- `cargo fmt --all -- --check` verifies formatting.
- `cargo clippy --all-targets --all-features -- -D warnings` runs strict lints.
- `cargo build --all-targets --all-features` compiles every target and feature.
- `cargo test --all-features` runs the complete test suite.

## Style and documentation

Use standard rustfmt output and Rust 2024 idioms. Use `snake_case` for modules,
files, functions, and fields, and `PascalCase` for public types and traits.
Document public APIs, errors, panics, and safety requirements. Add focused tests
with behavior changes and keep README examples working.

## Pull requests

Keep changes small and focused. Pull requests should summarize the change, call
out public API or behavior changes, and list the validation commands run.
