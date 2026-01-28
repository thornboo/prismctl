# Safety Model


Prismctl is designed with safety-first defaults to reduce the risk of accidental user config damage.

## 1. Dry-run by default (no writes)

- Commands that may write/delete files run in dry-run mode by default and only print the planned changes.
- Pass `--apply` to actually write to disk.

This lets you "rehearse" safely:

```bash
prismctl init --tool all
prismctl update --tool all
prismctl install --tool all --install-method auto
```

## 2. HOME sandbox (recommended)

Use `--home "<PATH>"` or `PRISMCTL_HOME` to redirect all Prismctl I/O into a sandbox directory:

```bash
export PRISMCTL_HOME="/tmp/prismctl-home"
prismctl update --tool all          # dry-run
prismctl update --tool all --apply  # writes into /tmp/prismctl-home/.codex/.claude/.gemini
```

This is useful for:

- First-time usage and experimentation
- CI / integration tests
- Avoiding changes to your real machine config during development

## 3. Dangerous operations require --yes (explicit confirmation)

Operations that are not safely mergeable (or are destructive) require `--yes`, for example:

- overwriting Codex `AGENTS.md`
- removing a skill directory
- global install/upgrade (runs `npm` or `brew`)

Typical usage:

```bash
prismctl codex agent use --name prismctl-engineer-professional --apply --yes
prismctl skill remove --name my-skill --apply --yes
prismctl install --tool all --install-method auto --apply --yes
```

## 4. Managed write strategy

Prismctl uses three write strategies (see `./managed-write-strategy.md`):

- namespaced files (safe to overwrite)
- managed blocks (preserve user content outside markers)
- explicit overwrite (requires `--yes` + backup)

## 5. Traceability (plan is visible)

Prismctl prints every planned change (e.g. `mkdir -p ...`, `write ...`, `run brew ...`) before you apply it.

Treat the dry-run output as an audit step: confirm it looks right, then rerun with `--apply`.
