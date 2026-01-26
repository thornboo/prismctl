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

使用 `--home` 或 `EKKO_HOME` 将所有读写隔离到临时目录，避免破坏本机真实配置。

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

## 3. 版本一致性

- `Cargo.toml` / `crates/*/Cargo.toml` 的 `version` 一致
- `CHANGELOG.md`（英文，根目录）与 `docs/zh-CN/changelog.md`（中文）同步更新（如采用双语）

## 4. Git 标签（本地）

```bash
git tag -a "vX.Y.Z" -m "vX.Y.Z"
```

> 说明：若需要自动化发布（GitHub Actions / crates.io），再讨论 push/tag 策略。
