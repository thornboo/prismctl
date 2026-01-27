# `ekko codex`


This page covers Codex subcommands: provider config and agent (AGENTS.md) management.

## `ekko codex provider set`

Configure Codex model provider.

```bash
ekko codex provider set [--home <PATH>] [--dry-run|--apply] [--provider <VALUE>] [--api-key <VALUE>] [--base-url <VALUE>] [--model <VALUE>] [--wire-api <VALUE>] [--default]
```

Flags:

- `--provider` uses a built-in preset (`openrouter` / `deepseek` / `ollama` / `volcengine` / `siliconflow`)
- `--base-url` / `--model` / `--wire-api` explicitly override preset fields
- `--api-key` writes `EKKO_CODEX_API_KEY` into `auth.json` (not stored in plaintext in `config.toml`)
- `--default` sets `model_provider = "ekko"` (make Ekko the default provider)

Files written (under default HOME):

- `~/.codex/config.toml`: upsert `[model_providers.ekko]`, optionally set `model_provider = "ekko"`
- `~/.codex/auth.json`: write `EKKO_CODEX_API_KEY`

Example:

```bash
ekko codex provider set \
  --provider "openrouter" \
  --api-key "sk-xxx" \
  --default \
  --apply
```

More patterns:

- Only set default provider (no base_url/model changes): `ekko codex provider set --default --apply`
- Only write API key (no config.toml changes): `ekko codex provider set --api-key "sk-xxx" --apply`
- Fully custom provider (no preset):

```bash
ekko codex provider set \
  --base-url "https://api.example.com/v1" \
  --wire-api "chat" \
  --model "gpt-4.1" \
  --api-key "sk-xxx" \
  --apply
```

## `ekko codex agent list`

List built-in Codex agent templates.

```bash
ekko codex agent list
```

## `ekko codex agent use` (dangerous)

Switch Codex system prompt (AGENTS.md).

WARNING: Dangerous: when used with `--apply`, it overwrites existing `AGENTS.md`, requires `--yes`, and creates a backup before overwriting (if an old file exists).

```bash
ekko codex agent use --name <VALUE> [--home <PATH>] [--lang <zh-CN|en>] [--dry-run|--apply] [--yes]
```

Behavior details:

- Target file: `~/.codex/AGENTS.md` (or `<home>/.codex/AGENTS.md` under `--home`)
- Backup path: `~/.codex/backup/ekko/<timestamp>/AGENTS.md` (only when an old file exists and is non-empty)
- `dry-run` does not require `--yes` and does not write/backup

Examples:

```bash
# Preview first (no overwrite)
ekko codex agent use --name "ekko-engineer-professional"

# Apply (explicit confirmation required)
ekko codex agent use --name "ekko-engineer-professional" --apply --yes
```
