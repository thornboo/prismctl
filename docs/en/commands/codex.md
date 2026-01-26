# `ekko codex`


This page covers Codex subcommands: provider config and agent (AGENTS.md) management.

## `ekko codex provider set`

Configure Codex model provider.

```bash
ekko codex provider set [--home <PATH>] [--dry-run|--apply] [--provider <VALUE>] [--api-key <VALUE>] [--base-url <VALUE>] [--model <VALUE>] [--wire-api <VALUE>] [--default]
```

Notes:

- `--provider` uses built-in presets (openrouter/deepseek/ollama/volcengine/siliconflow)
- explicit flags (`--base-url/--model/--wire-api`) override preset values
- API key is stored in `~/.codex/auth.json` (via `temp_env_key`), not in `config.toml` plaintext
- `--default` sets the Ekko provider as default (exact behavior depends on Codex config format)

Example:

```bash
ekko codex provider set \
  --provider "openrouter" \
  --api-key "sk-xxx" \
  --default \
  --apply
```

Files written:

- `~/.codex/config.toml`: add/update `[model_providers.ekko]`
- `~/.codex/auth.json`: write `EKKO_CODEX_API_KEY`

## `ekko codex agent list`

List built-in Codex agent templates.

```bash
ekko codex agent list
```

## `ekko codex agent use` (dangerous)

Switch Codex system prompt (AGENTS.md).

⚠️ Dangerous: overwrites existing `AGENTS.md`, requires `--yes`, and creates a backup before overwriting.

```bash
ekko codex agent use --name <VALUE> [--home <PATH>] [--lang <zh-CN|en>] [--dry-run|--apply] --yes
```
