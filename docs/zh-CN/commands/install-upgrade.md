# `ekko install` / `ekko upgrade`


全局安装/升级 AI 工具（Codex / Claude Code / Gemini CLI）。

⚠️ 危险操作：执行时需要 `--apply --yes`。

## 语法

```bash
ekko install --tool <codex|claude|gemini|all> [--install-method <auto|npm|brew>] [--dry-run|--apply] [--yes]
ekko upgrade --tool <codex|claude|gemini|all> [--install-method <auto|npm|brew>] [--dry-run|--apply] [--yes]
```

## 安装方式

- `npm`：使用 npm 全局安装
- `brew`：使用 Homebrew 安装（macOS）
- `auto`：macOS 优先 brew，其次 npm；其他平台使用 npm

## 工具与包名映射

| 方式 | Codex | Claude Code | Gemini CLI |
|------|-------|-------------|------------|
| npm | `@openai/codex` | `@anthropic-ai/claude-code` | `@google/gemini-cli` |
| brew | `codex`（cask） | `claude-code`（cask） | `gemini-cli`（formula） |

## 示例

```bash
# 预览安装计划
ekko install --tool all --install-method auto

# 实际执行
ekko install --tool all --install-method auto --apply --yes
```
