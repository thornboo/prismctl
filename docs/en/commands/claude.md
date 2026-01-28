# `prismctl claude`


This page covers Claude Code subcommands: env configuration, output style selection, and MCP configuration.

> Note: MCP operations delegate to Claude Code CLI (`claude mcp ...`), so you need `claude` installed locally.

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

## `prismctl claude mcp ...` (delegates to claude CLI)

Manage Claude Code MCP servers via `claude mcp`, using Prismctl's built-in MCP catalog.

Available subcommands:

- `prismctl claude mcp list [--project-path <PATH>] [--home <PATH>]`
- `prismctl claude mcp builtin`
- `prismctl claude mcp add --name <VALUE> [--scope <local|project|user>] [--project-path <PATH>] [--home <PATH>] [--dry-run|--apply] [--yes]`
- `prismctl claude mcp get --name <VALUE> [--project-path <PATH>] [--home <PATH>]`
- `prismctl claude mcp remove --name <VALUE> [--project-path <PATH>] [--home <PATH>] [--dry-run|--apply] [--yes]`

Notes:

- `--project-path` is used as the working directory when running `claude` (so Claude Code can resolve project/local scope files).
- `--scope` defaults to `local` for `add`.

Example:

```bash
# Preview (no writes)
prismctl claude mcp add --name context7 --scope project --project-path "/path/to/your/project"

# Apply (explicit confirmation required)
prismctl claude mcp add --name context7 --scope project --project-path "/path/to/your/project" --apply --yes
```
