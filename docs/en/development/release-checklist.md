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

Use `--home` or `PRISMCTL_HOME` to isolate all writes into a temporary directory.

```bash
export PRISMCTL_HOME="/tmp/prismctl-release-test"
rm -rf "$PRISMCTL_HOME"
mkdir -p "$PRISMCTL_HOME"

prismctl doctor
prismctl init --tool all --home "$PRISMCTL_HOME" --apply

# Skills
prismctl skill list --home "$PRISMCTL_HOME"
prismctl skill install --name explain-code --home "$PRISMCTL_HOME" --apply

# Providers / env
prismctl codex provider set --provider openrouter --api-key "test-key" --home "$PRISMCTL_HOME" --apply
prismctl claude env set --auth-token "test-token" --home "$PRISMCTL_HOME" --apply
prismctl gemini env set --api-key "test-gemini-key" --home "$PRISMCTL_HOME" --apply
```

## 3. Version consistency

- `version` in `Cargo.toml` and `crates/*/Cargo.toml` stays consistent
- Update changelog (if you keep bilingual changelogs, keep them in sync)

## 4. Git tag (local)

```bash
git tag -a "vX.Y.Z" -m "vX.Y.Z"
```

