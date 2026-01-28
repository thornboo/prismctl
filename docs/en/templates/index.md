# Templates


Prismctl ships built-in templates (workflows, Git commands, agents, output styles) and supports `zh-CN` / `en`.

## Write strategy

Prismctl uses a managed write strategy to avoid breaking user config (see `../concepts/managed-write-strategy.md`).

One-line rule of thumb:

- Prismctl fully owns template files under the `prismctl/` namespace (safe to overwrite/update)
- For a small set of shared files, Prismctl only updates managed blocks (preserves content outside markers)
- For files without safe merge semantics, Prismctl requires `--yes` and performs automatic backups

## Categories

- Codex: `./codex.md`
- Claude Code: `./claude.md`
- Gemini CLI: `./gemini.md`
- Project templates: `./project.md`

## Init & update

```bash
prismctl init --tool all --apply
prismctl update --tool all --apply
```

## Where should I customize templates?

Treat Prismctl templates like an upstream distribution:

- Want smooth upgrades: avoid editing files under the `prismctl/` namespace directly
- Need customization: copy the template into your own namespace/directory and reference your copy from the tool
