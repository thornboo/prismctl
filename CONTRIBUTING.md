# Contributing

Thanks for your interest in Ekko! This repository is a Rust workspace with:

- `crates/ekko-core`: core library (business logic)
- `crates/ekko-cli`: CLI binary (`ekko`)

## Prerequisites

- Rust (recommended: rustup)
- Git

## Common Commands

```bash
# Format
cargo fmt --all

# Lint (must be zero warnings)
cargo clippy -- -D warnings

# Tests
cargo test --all

# Run locally
cargo run -p ekko -- --help
```

## Code Guidelines (Excerpt)

- Keep it simple: KISS / DRY / YAGNI
- Public APIs should have `///` doc comments when appropriate
- Comment only non-obvious logic; keep comments in English and short
