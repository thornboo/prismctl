# 发布检查清单


本清单用于在发布新版本前做最终验证，避免把明显问题带到发布包里。

## 1. 代码质量

```bash
cargo fmt --all --check
cargo clippy -- -D warnings
cargo test --all
cargo build --release
```

## 2. 沙箱冒烟测试（推荐）

使用 `--home` 或 `PRISMCTL_HOME` 将所有读写隔离到临时目录，避免破坏本机真实配置。

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

## 3. 版本一致性

- `Cargo.toml` / `crates/*/Cargo.toml` 的 `version` 一致
- `CHANGELOG.md`（英文，根目录）与 `docs/zh-CN/changelog.md`（中文）同步更新（如采用双语）

## 4. Git 标签（本地）

```bash
git tag -a "vX.Y.Z" -m "vX.Y.Z"
```

> 说明：若需要自动化发布（GitHub Actions / crates.io），再讨论 push/tag 策略。
