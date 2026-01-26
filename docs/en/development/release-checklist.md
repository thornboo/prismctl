# Release Checklist


Use this checklist before cutting a new release.

## 1. Code quality

```bash
cargo fmt --all --check
cargo clippy -- -D warnings
cargo test --all
cargo build --release
```

## 2. Sandbox smoke test (recommended)

Use `--home` or `EKKO_HOME` to isolate all writes into a temporary directory.

```bash
export EKKO_HOME="/tmp/ekko-release-test"
rm -rf "$EKKO_HOME"
mkdir -p "$EKKO_HOME"

ekko doctor
ekko init --tool all --home "$EKKO_HOME" --apply

# Skills
ekko skill list --home "$EKKO_HOME"
ekko skill install --name explain-code --home "$EKKO_HOME" --apply

# Providers / env
ekko codex provider set --provider openrouter --api-key "test-key" --home "$EKKO_HOME" --apply
ekko claude env set --auth-token "test-token" --home "$EKKO_HOME" --apply
ekko gemini env set --api-key "test-gemini-key" --home "$EKKO_HOME" --apply
```

## 3. Version consistency

- `version` in `Cargo.toml` and `crates/*/Cargo.toml` stays consistent
- Update changelog (if you keep bilingual changelogs, keep them in sync)

## 4. Git tag (local)

```bash
git tag -a "vX.Y.Z" -m "vX.Y.Z"
```

