# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build Commands
- Build: `cargo build`
- Release build: `cargo build --release`

## Test Commands
- Run all tests: `cargo test`
- Run unit tests: `cargo test --test unit_tests`
- Run integration tests: `cargo test --test integration_tests`
- Run library tests with real API: `cargo test --test library_test -- --ignored`
- Run single test: `cargo test test_name -- --exact`
- Test with coverage: `cargo tarpaulin --out Xml --out Json --workspace --exclude-files "tests/*"`

## Lint Commands
- Run clippy: `cargo clippy -- -D warnings`
- Format code: `cargo fmt`

## WSL Environment Setup
If running in WSL on Windows, create the following config to help cargo find OpenSSL:
```
# ~/.cargo/config.toml
[env]
OPENSSL_DIR = "/usr"
OPENSSL_LIB_DIR = "/usr/lib/x86_64-linux-gnu"
```

## Code Style Guidelines
- Use `thiserror` for error definition with descriptive messages
- Result type alias: `XbrlResult<T>` for consistent error handling
- Types/Structs: PascalCase (e.g., `XbrlClient`)
- Methods/Variables: snake_case (e.g., `get_taxonomies`)
- Document public APIs with `///` comments (include examples where appropriate)
- Follow Rust's standard formatting rules
- Use explicit error propagation with `?` operator
- Maintain 80% code coverage minimum