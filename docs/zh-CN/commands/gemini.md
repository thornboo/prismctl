# `prismctl gemini`

本页覆盖 Gemini CLI 相关子命令：`.env`、`settings.json` 与 MCP 配置。

> 说明：MCP 配置委托给 Gemini CLI（`gemini mcp ...`），因此本机需要已安装 `gemini` 命令。

## `prismctl gemini env set`

写入/更新用户级或项目级的 `.env`，并且仅维护一个 Prismctl 受管块（不会覆盖块外内容）。

受管块 keys：

- `GEMINI_API_KEY`

```bash
prismctl gemini env set [--scope <user|project>] [--project-path <PATH>] [--home <PATH>] [--dry-run|--apply] --api-key <VALUE>
```

示例：

```bash
# 用户级：~/.gemini/.env
prismctl gemini env set --api-key "xxx" --apply

# 项目级：<project>/.gemini/.env（留空=当前目录）
prismctl gemini env set --scope project --project-path "/path/to/your/project" --api-key "xxx" --apply
```

受管块格式（示意）：

```dotenv
# other user-managed keys...

# prismctl:start
GEMINI_API_KEY="xxx"
# prismctl:end
```

> 建议：把你自己维护的变量放在 Prismctl 受管块之外，避免被 Prismctl 的受管块更新覆盖。

## `prismctl gemini settings set`

在 `settings.json` 中 upsert `model.name`（其他字段会保留，但会以 pretty JSON 形式重写文件格式）。

```bash
prismctl gemini settings set [--scope <user|project>] [--project-path <PATH>] [--home <PATH>] [--dry-run|--apply] --model <VALUE>
```

示例：

```bash
# 用户级：~/.gemini/settings.json
prismctl gemini settings set --model "gemini-2.5-pro" --apply

# 项目级：<project>/.gemini/settings.json
prismctl gemini settings set --scope project --project-path "/path/to/your/project" --model "gemini-2.5-pro" --apply
```

## `prismctl gemini mcp ...`（依赖 gemini CLI）

管理 Gemini CLI 的 MCP servers（委托 `gemini mcp`），并使用 Prismctl 的内置 MCP catalog 进行选择/配置。

可用子命令：

- `prismctl gemini mcp list [--scope <user|project>] [--project-path <PATH>] [--home <PATH>]`
- `prismctl gemini mcp builtin`
- `prismctl gemini mcp add --name <VALUE> [--scope <user|project>] [--project-path <PATH>] [--home <PATH>] [--dry-run|--apply] [--yes]`
- `prismctl gemini mcp remove --name <VALUE> [--scope <user|project>] [--project-path <PATH>] [--home <PATH>] [--dry-run|--apply] [--yes]`
- `prismctl gemini mcp enable --name <VALUE> [--project-path <PATH>] [--home <PATH>] [--dry-run|--apply] [--yes]`
- `prismctl gemini mcp disable --name <VALUE> [--project-path <PATH>] [--home <PATH>] [--dry-run|--apply] [--yes]`

示例：

```bash
# 先预览（不会写入）
prismctl gemini mcp add --name context7 --scope project --project-path "/path/to/your/project"

# 真正写入（需要显式确认）
prismctl gemini mcp add --name context7 --scope project --project-path "/path/to/your/project" --apply --yes
```

