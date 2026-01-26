# Safety Model


Ekko is designed with safety-first defaults to reduce the risk of accidental user config damage.

## 1. Dry-run by default

- Commands that may write/delete files run in dry-run mode by default and only print the planned changes.
- Pass `--apply` to actually write to disk.

## 2. HOME sandbox (recommended)

Use `--home "<PATH>"` or `EKKO_HOME` to redirect all Ekko I/O into a sandbox directory:

```bash
export EKKO_HOME="/tmp/ekko-home"
ekko update --tool all          # dry-run
ekko update --tool all --apply  # writes into /tmp/ekko-home/.codex/.claude/.gemini
```

This is useful for:

- First-time usage and experimentation
- CI / integration tests
- Avoiding changes to your real machine config during development

## 3. Dangerous operations require --yes

Operations that are not safely mergeable (or are destructive) require `--yes`, for example:

- overwriting Codex `AGENTS.md`
- removing a skill directory

Typical usage:

```bash
ekko codex agent use --name ekko-engineer-professional --apply --yes
ekko skill remove --name my-skill --apply --yes
```

## 4. Managed write strategy

Ekko uses three write strategies (see `./managed-write-strategy.md`):

- namespaced files (safe to overwrite)
- managed blocks (preserve user content outside markers)
- explicit overwrite (requires `--yes` + backup)

