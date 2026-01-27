# Quick Start


This page assumes you already installed `ekko` (see `./installation.md`).

## 1. Recommended: use a sandbox first

Ekko can redirect all reads/writes into a "sandbox HOME" via `--home "<PATH>"` or `EKKO_HOME`.

```bash
export EKKO_HOME="/tmp/ekko-home"

# Preview planned changes (dry-run)
ekko init --tool all

# Apply into sandbox directory
ekko init --tool all --apply
```

You can also use `ekko doctor` to verify the resolved paths:

```bash
ekko doctor
```

## 2. Init / update templates (real HOME)

Once sandbox results look good, run against your real HOME:

```bash
unset EKKO_HOME

ekko init --tool all          # dry-run
ekko init --tool all --apply  # apply
```

Update templates (managed writes to avoid breaking user config):

```bash
ekko update --tool all --apply
```

> Note: `init` and `update` overwrite Ekko-managed files under the `ekko/` namespace (e.g. `~/.codex/prompts/ekko/*`). "Preserve user config" mainly means Ekko does not touch files outside its namespace, and uses managed blocks for a small set of shared files.

## 3. Configure the 3 tools

### Codex (provider)

```bash
ekko codex provider set \
  --provider "openrouter" \
  --api-key "sk-xxx" \
  --default \
  --apply
```

This updates:

- `~/.codex/config.toml`: upserts Ekko provider (`model_providers.ekko`)
- `~/.codex/auth.json`: writes `EKKO_CODEX_API_KEY` (not stored in plaintext in `config.toml`)

### Claude Code (env / output style)

```bash
ekko claude env set --auth-token "sk-xxx" --model "claude-sonnet-4" --apply
ekko claude output-style use --name "ekko-engineer-professional" --apply
```

### Gemini CLI (env)

```bash
ekko gemini env set --api-key "xxx" --model "gemini-2.0-flash" --apply
```

This maintains an Ekko-managed block inside `~/.gemini/.env` (preserves content outside the block).

## 4. Project init

Generate `.ekko/plan/` and `.gemini/GEMINI.md` (managed block):

```bash
ekko project init --path "/path/to/your/project" --apply
```

See: `../projects/project-init.md`.

## 5. Install / upgrade the AI tools globally (npm / brew)

⚠️ Dangerous operation: requires `--apply --yes`.

```bash
ekko install --tool all --install-method auto          # dry-run
ekko install --tool all --install-method auto --apply --yes
```

## Next

- Command reference: `../commands/index.md`
- Templates: `../templates/index.md`
- Skills: `../skills/overview.md`
