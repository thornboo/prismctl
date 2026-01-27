# Templates


Ekko ships built-in templates (workflows, Git commands, agents, output styles) and supports `zh-CN` / `en`.

## Write strategy

Ekko uses a managed write strategy to avoid breaking user config (see `../concepts/managed-write-strategy.md`).

One-line rule of thumb:

- Ekko fully owns template files under the `ekko/` namespace (safe to overwrite/update)
- For a small set of shared files, Ekko only updates managed blocks (preserves content outside markers)
- For files without safe merge semantics, Ekko requires `--yes` and performs automatic backups

## Categories

- Codex: `./codex.md`
- Claude Code: `./claude.md`
- Gemini CLI: `./gemini.md`
- Project templates: `./project.md`

## Init & update

```bash
ekko init --tool all --apply
ekko update --tool all --apply
```

## Where should I customize templates?

Treat Ekko templates like an upstream distribution:

- Want smooth upgrades: avoid editing files under the `ekko/` namespace directly
- Need customization: copy the template into your own namespace/directory and reference your copy from the tool
