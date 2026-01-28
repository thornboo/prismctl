# `prismctl claude`


This page covers Claude Code subcommands: env configuration and output style selection.

## `prismctl claude env set`

Write/update the `env` field in `~/.claude/settings.json` (a key/value map).

Common keys:

- `ANTHROPIC_AUTH_TOKEN`
- `ANTHROPIC_BASE_URL` (optional)
- `ANTHROPIC_MODEL` (optional)
- `ANTHROPIC_DEFAULT_HAIKU_MODEL` (optional)
- `ANTHROPIC_DEFAULT_SONNET_MODEL` (optional)
- `ANTHROPIC_DEFAULT_OPUS_MODEL` (optional)

```bash
prismctl claude env set [--home <PATH>] [--dry-run|--apply] [--auth-token <VALUE>] [--base-url <VALUE>] [--model <VALUE>] [--haiku-model <VALUE>] [--sonnet-model <VALUE>] [--opus-model <VALUE>]
```

Example:

```bash
prismctl claude env set --auth-token "sk-xxx" --model "claude-sonnet-4" --apply
```

Files written:

- `~/.claude/settings.json` (or `<home>/.claude/settings.json` under `--home`)

> Tip: this command only upserts the related keys; it does not wipe other `settings.json` fields.

## `prismctl claude output-style use`

Set `outputStyle` in `~/.claude/settings.json`.

```bash
prismctl claude output-style use --name <VALUE> [--home <PATH>] [--dry-run|--apply]
```

Example:

```bash
prismctl claude output-style use --name "prismctl-engineer-professional" --apply
```

For available names, see: `../templates/claude.md`.
