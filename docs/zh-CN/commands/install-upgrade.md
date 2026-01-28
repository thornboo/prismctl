# `prismctl install` / `prismctl upgrade`


全局安装/升级 AI 工具（Codex / Claude Code / Gemini CLI）。

⚠️ 危险操作：执行时需要 `--apply --yes`。

## 语法

```bash
prismctl install --tool <codex|claude|gemini|all> [--install-method <auto|npm|brew>] [--dry-run|--apply] [--yes]
prismctl upgrade --tool <codex|claude|gemini|all> [--install-method <auto|npm|brew>] [--dry-run|--apply] [--yes]
```

## 安装方式

- `npm`：使用 npm 全局安装
- `brew`：使用 Homebrew 安装（macOS）
- `auto`：macOS 优先 brew，其次 npm；其他平台使用 npm

## 实际会执行什么

Prismctl 不会直接下载二进制，而是调用你机器上的包管理器：

- npm：`npm install -g <package>@latest`
- brew：`brew install <name>` / `brew upgrade <name>`（cask/formula 取决于工具）

因此在使用前请确保你的环境里已正确安装并可执行 `npm` 或 `brew`。

## 工具与包名映射

| 方式 | Codex | Claude Code | Gemini CLI |
|------|-------|-------------|------------|
| npm | `@openai/codex` | `@anthropic-ai/claude-code` | `@google/gemini-cli` |
| brew | `codex`（cask） | `claude-code`（cask） | `gemini-cli`（formula） |

## 示例

```bash
# 预览安装计划
prismctl install --tool all --install-method auto

# 实际执行
prismctl install --tool all --install-method auto --apply --yes
```

建议流程：

1. 先跑一遍 dry-run，确认将要执行的命令正确
2. 再加上 `--apply --yes` 真正执行
