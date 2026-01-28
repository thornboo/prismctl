# Contributing

Thanks for your interest in Prismctl! This repository is a Rust workspace with:

- `crates/prismctl-core`: core library (business logic)
- `crates/prismctl-cli`: CLI binary (`prismctl`)

## Prerequisites

- Rust (recommended: rustup)
- Git

## Common Commands

```bash
# Format
cargo fmtall

# Lint (must be zero warnings)
cargo lint

# Tests
cargo testall

# Run locally
cargo run -p prismctl -- --help
```

## Code Guidelines (Excerpt)

- Keep it simple: KISS / DRY / YAGNI
- Public APIs should have `///` doc comments when appropriate
- Comment only non-obvious logic; keep comments in English and short
