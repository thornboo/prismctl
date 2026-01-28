# Quick Start


This page assumes you already installed `prismctl` (see `./installation.md`).

## 0. Interactive wizard (TTY only)

Run `prismctl` (or `prismctl config`) to enter the interactive wizard.

> Note: in non-TTY environments (CI / piping), interactive mode exits with an error to avoid hanging on prompts.

```bash
prismctl
```

## 1. Recommended: use a sandbox first

Prismctl can redirect all reads/writes into a "sandbox HOME" via `--home "<PATH>"` or `PRISMCTL_HOME`.

```bash
export PRISMCTL_HOME="/tmp/prismctl-home"

# Preview planned changes (dry-run)
prismctl init --tool all

# Apply into sandbox directory
prismctl init --tool all --apply
```

You can also use `prismctl doctor` to verify the resolved paths:

```bash
prismctl doctor
```

## 2. Init / update templates (real HOME)

Once sandbox results look good, run against your real HOME:

```bash
unset PRISMCTL_HOME

prismctl init --tool all          # dry-run
prismctl init --tool all --apply  # apply
```

Update templates (managed writes to avoid breaking user config):

```bash
prismctl update --tool all --apply
```

Quick aliases (optional):

```bash
prismctl i --tool all --apply
prismctl u --tool all --apply
```

> Note: `init` and `update` overwrite Prismctl-managed files under the `prismctl/` namespace (e.g. `~/.codex/prompts/prismctl/*`). "Preserve user config" mainly means Prismctl does not touch files outside its namespace, and uses managed blocks for a small set of shared files.

## 3. Configure the 3 tools

### Codex (provider)

```bash
prismctl codex provider set \
  --provider "openrouter" \
  --api-key "sk-xxx" \
  --default \
  --apply
```

This updates:

- `~/.codex/config.toml`: upserts Prismctl provider (`model_providers.prismctl`)
- `~/.codex/auth.json`: writes `PRISMCTL_CODEX_API_KEY` (not stored in plaintext in `config.toml`)

### Claude Code (env / output style)

```bash
prismctl claude env set --auth-token "sk-xxx" --model "claude-sonnet-4" --apply
prismctl claude output-style use --name "prismctl-engineer-professional" --apply
```

### Gemini CLI (env)

```bash
prismctl gemini env set --api-key "xxx" --apply
prismctl gemini settings set --model "gemini-2.0-flash" --apply
```

This maintains an Prismctl-managed block inside `~/.gemini/.env` (preserves content outside the block).

## 4. Project init

Generate `.prismctl/plan/` and `.gemini/GEMINI.md` (managed block):

```bash
prismctl project init --path "/path/to/your/project" --apply
```

See: `../projects/project-init.md`.

## 5. Install / upgrade the AI tools globally (npm / brew)

WARNING: Dangerous operation: requires `--apply --yes`.

```bash
prismctl install --tool all --install-method auto          # dry-run
prismctl install --tool all --install-method auto --apply --yes
```

## Next

- Command reference: `../commands/index.md`
- Templates: `../templates/index.md`
- Skills: `../skills/overview.md`
