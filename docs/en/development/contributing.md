# Contributing


This section is a quick entry point for contributors. Full guidelines live in the repo root:

- `../../../CONTRIBUTING.md`

## Local setup (quick)

```bash
git clone "https://github.com/thornboo/ekko.git"
cd "ekko"
cargo build
```

## Quality gates

```bash
cargo fmt --all
cargo clippy --all-targets -- -D warnings
cargo test --all
```

## Docs preview

```bash
mdbook serve "docs/en" -n 127.0.0.1 -p 3000
mdbook serve "docs/zh-CN" -n 127.0.0.1 -p 3001
```
