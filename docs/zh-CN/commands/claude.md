# `prismctl claude`


本页覆盖 Claude Code 相关子命令：环境变量配置、输出风格与 MCP 配置。

> 说明：MCP 配置委托给 Claude Code CLI（`claude mcp ...`），因此本机需要已安装 `claude` 命令。

## `prismctl claude env set`

写入/更新 `~/.claude/settings.json` 的 `env` 字段（一个键值表）。

常用键：

- `ANTHROPIC_AUTH_TOKEN`
- `ANTHROPIC_BASE_URL`（可选）
- `ANTHROPIC_MODEL`（可选）
- `ANTHROPIC_DEFAULT_HAIKU_MODEL`（可选）
- `ANTHROPIC_DEFAULT_SONNET_MODEL`（可选）
- `ANTHROPIC_DEFAULT_OPUS_MODEL`（可选）

```bash
prismctl claude env set [--home <PATH>] [--dry-run|--apply] [--auth-token <VALUE>] [--base-url <VALUE>] [--model <VALUE>] [--haiku-model <VALUE>] [--sonnet-model <VALUE>] [--opus-model <VALUE>]
```

示例：

```bash
prismctl claude env set --auth-token "sk-xxx" --model "claude-sonnet-4" --apply
```

写入位置：

- `~/.claude/settings.json`（`--home` 场景下对应 `<home>/.claude/settings.json`）

> 提示：该命令只会 upsert/更新相关键，不会清空你的其他 `settings.json` 配置项。

## `prismctl claude output-style use`

设置 `~/.claude/settings.json` 的 `outputStyle`。

```bash
prismctl claude output-style use --name <VALUE> [--home <PATH>] [--dry-run|--apply]
```

示例：

```bash
prismctl claude output-style use --name "prismctl-engineer-professional" --apply
```

相关模板与可用名称见：`../templates/claude.md`。

## `prismctl claude mcp ...`（依赖 claude CLI）

管理 Claude Code 的 MCP servers（委托 `claude mcp`），并使用 Prismctl 的内置 MCP catalog 进行选择/配置。

可用子命令：

- `prismctl claude mcp list [--project-path <PATH>] [--home <PATH>]`
- `prismctl claude mcp builtin`
- `prismctl claude mcp add --name <VALUE> [--scope <local|project|user>] [--project-path <PATH>] [--home <PATH>] [--dry-run|--apply] [--yes]`
- `prismctl claude mcp get --name <VALUE> [--project-path <PATH>] [--home <PATH>]`
- `prismctl claude mcp remove --name <VALUE> [--project-path <PATH>] [--home <PATH>] [--dry-run|--apply] [--yes]`

说明：

- `--project-path` 会作为执行 `claude` 时的工作目录（Claude Code 需要它来定位 project/local scope 的落盘位置）。
- `add` 的 `--scope` 默认是 `local`。

示例：

```bash
# 先预览（不会写入）
prismctl claude mcp add --name context7 --scope project --project-path "/path/to/your/project"

# 真正写入（需要显式确认）
prismctl claude mcp add --name context7 --scope project --project-path "/path/to/your/project" --apply --yes
```
