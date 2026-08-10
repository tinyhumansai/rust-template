# Repository Guidelines

## Project structure

This is a Rust 2024 library crate. Keep public exports centralized in
`src/lib.rs`, integration tests in `tests/`, runnable examples in `examples/`,
and architecture or module notes in `docs/`.

Each feature area belongs in a focused module directory under `src/`. Use
`mod.rs` as the module root, move substantial type definitions into `types.rs`,
and put module-local unit tests in a dedicated `test.rs`. Wire those tests from
the bottom of the module root with:

```rust
#[cfg(test)]
mod test;
```

Do not accumulate inline `mod tests` blocks in implementation files. A module
root should explain the module, wire its pieces together, and expose the
smallest useful API. Put shared error variants in `src/error/mod.rs` and return
the crate-wide `Result<T>` from fallible public APIs.

## Build and test

- `cargo fmt --all -- --check` verifies formatting.
- `cargo clippy --all-targets --all-features -- -D warnings` runs strict lints.
- `cargo build --all-targets --all-features` compiles every target and feature.
- `cargo test --all-features` runs the complete test suite.

## Style and documentation

Use standard rustfmt output and Rust 2024 idioms. Use `snake_case` for modules,
files, functions, and fields, and `PascalCase` for public types and traits.
Document public APIs, errors, panics, and safety requirements. Start every
`mod.rs` and `test.rs` with a concise module-level `//!` description. Add focused
tests with behavior changes and keep README examples working.

Complex modules must include a module-level `README.md` covering their design,
public surface, and important constraints. Keep every Markdown file, including
this one, at 500 lines or fewer; split longer topics into focused linked files.

## Pull requests

Keep changes small and focused. Pull requests should summarize the change, call
out public API or behavior changes, and list the validation commands run.
