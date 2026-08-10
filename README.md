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

## Development

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo build --all-targets --all-features
cargo test --all-features
```

## License

GPL-3.0-only. See [LICENSE](LICENSE).
