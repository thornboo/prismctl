# `prismctl gemini`

This page covers Gemini CLI subcommands: `.env`, `settings.json`, and MCP configuration.

> Note: MCP operations delegate to Gemini CLI (`gemini mcp ...`), so you need `gemini` installed locally.

## `prismctl gemini env set`

Write/update a user-scoped or project-scoped `.env` while only maintaining a Prismctl-managed block (preserves content outside the block).

Managed block keys:

- `GEMINI_API_KEY`

```bash
prismctl gemini env set [--scope <user|project>] [--project-path <PATH>] [--home <PATH>] [--dry-run|--apply] --api-key <VALUE>
```

Example:

```bash
# User scope: ~/.gemini/.env
prismctl gemini env set --api-key "xxx" --apply

# Project scope: <project>/.gemini/.env (empty path = current dir)
prismctl gemini env set --scope project --project-path "/path/to/your/project" --api-key "xxx" --apply
```

Files written:

- user scope: `~/.gemini/.env` (or `<home>/.gemini/.env` under `--home`)
- project scope: `<project>/.gemini/.env`

Managed block format (example):

```dotenv
# other user-managed keys...

# prismctl:start
GEMINI_API_KEY="xxx"
# prismctl:end
```

> Recommendation: keep your own variables outside the Prismctl-managed block to avoid being overwritten by Prismctl updates.

## `prismctl gemini settings set`

Upsert `model.name` in `settings.json` (other fields are preserved, but the file is rewritten as pretty JSON).

```bash
prismctl gemini settings set [--scope <user|project>] [--project-path <PATH>] [--home <PATH>] [--dry-run|--apply] --model <VALUE>
```

Examples:

```bash
# User scope: ~/.gemini/settings.json
prismctl gemini settings set --model "gemini-2.5-pro" --apply

# Project scope: <project>/.gemini/settings.json
prismctl gemini settings set --scope project --project-path "/path/to/your/project" --model "gemini-2.5-pro" --apply
```

## `prismctl gemini mcp ...` (delegates to gemini CLI)

Manage Gemini CLI MCP servers via `gemini mcp`, using Prismctl's built-in MCP catalog.

Available subcommands:

- `prismctl gemini mcp list [--scope <user|project>] [--project-path <PATH>] [--home <PATH>]`
- `prismctl gemini mcp builtin`
- `prismctl gemini mcp add --name <VALUE> [--scope <user|project>] [--project-path <PATH>] [--home <PATH>] [--dry-run|--apply] [--yes]`
- `prismctl gemini mcp remove --name <VALUE> [--scope <user|project>] [--project-path <PATH>] [--home <PATH>] [--dry-run|--apply] [--yes]`
- `prismctl gemini mcp enable --name <VALUE> [--project-path <PATH>] [--home <PATH>] [--dry-run|--apply] [--yes]`
- `prismctl gemini mcp disable --name <VALUE> [--project-path <PATH>] [--home <PATH>] [--dry-run|--apply] [--yes]`

Example:

```bash
# Preview (no writes)
prismctl gemini mcp add --name context7 --scope project --project-path "/path/to/your/project"

# Apply (explicit confirmation required)
prismctl gemini mcp add --name context7 --scope project --project-path "/path/to/your/project" --apply --yes
```
