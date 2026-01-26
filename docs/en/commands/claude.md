# `ekko claude`


This page covers Claude Code subcommands: env configuration and output style selection.

## `ekko claude env set`

Write/update the `env` field in `~/.claude/settings.json`.

Common keys:

- `ANTHROPIC_AUTH_TOKEN`
- `ANTHROPIC_BASE_URL` (optional)
- `ANTHROPIC_MODEL` (optional)

```bash
ekko claude env set [--home <PATH>] [--dry-run|--apply] [--auth-token <VALUE>] [--base-url <VALUE>] [--model <VALUE>] [--haiku-model <VALUE>] [--sonnet-model <VALUE>] [--opus-model <VALUE>]
```

Example:

```bash
ekko claude env set --auth-token "sk-xxx" --model "claude-sonnet-4" --apply
```

## `ekko claude output-style use`

Set `outputStyle` in `~/.claude/settings.json`.

```bash
ekko claude output-style use --name <VALUE> [--home <PATH>] [--dry-run|--apply]
```
