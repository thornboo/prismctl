# `ekko claude`


本页覆盖 Claude Code 相关子命令：环境变量配置与输出风格。

## `ekko claude env set`

写入/更新 `~/.claude/settings.json` 的 `env` 字段。

常用键：

- `ANTHROPIC_AUTH_TOKEN`
- `ANTHROPIC_BASE_URL`（可选）
- `ANTHROPIC_MODEL`（可选）

```bash
ekko claude env set [--home <PATH>] [--dry-run|--apply] [--auth-token <VALUE>] [--base-url <VALUE>] [--model <VALUE>] [--haiku-model <VALUE>] [--sonnet-model <VALUE>] [--opus-model <VALUE>]
```

示例：

```bash
ekko claude env set --auth-token "sk-xxx" --model "claude-sonnet-4" --apply
```

## `ekko claude output-style use`

设置 `~/.claude/settings.json` 的 `outputStyle`。

```bash
ekko claude output-style use --name <VALUE> [--home <PATH>] [--dry-run|--apply]
```
