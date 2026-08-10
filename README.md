# Rust Template

A production-ready Rust 2024 library template used by TinyHumans AI.

## Use this template

Choose **Use this template** on GitHub, create a repository, then update:

- the package name, description, repository, keywords, and categories in `Cargo.toml`;
- this README and the package-level documentation in `src/lib.rs`;
- the security contact and repository links in the community files;
- the license if GPL-3.0-only is not appropriate for the new project.

Search for `rust-template` and `tinyhumansai/rust-template` to find the remaining
template-specific values.

## Layout

```text
src/
├── lib.rs
├── error/
│   └── mod.rs
└── greeting/
    ├── mod.rs
    └── test.rs
tests/
└── public_api.rs
examples/
└── basic.rs
```

Feature areas use directory modules. Their implementation and exports live in
`mod.rs`, substantial types can move to `types.rs`, and unit tests live in
`test.rs`. `AGENTS.md` contains the complete repository guidance, while
`CLAUDE.md` points to the same instructions so coding agents share one source of
truth.

## Development

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo build --all-targets --all-features
cargo test --all-features
cargo run --example basic
```

## License

GPL-3.0-only. See [LICENSE](LICENSE).
