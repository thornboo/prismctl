# Managed Write Strategy


To avoid breaking user-owned configuration, Prismctl uses three write strategies.

## 1) Namespaced files (default)

Prismctl only writes into an `prismctl/` namespace directory and fully owns those files, so it is safe to overwrite/update them (`init` and `update` will overwrite these).

Examples:

- Codex: `~/.codex/prompts/prismctl/*`
- Claude Code: `~/.claude/commands/prismctl/*`, `~/.claude/agents/prismctl/*`, `~/.claude/output-styles/*` (Prismctl built-ins)
- Gemini CLI: `~/.gemini/prismctl/WORKFLOWS.md`

## 2) Managed blocks (preserve user content)

For shared files that users may extend (e.g. Gemini `GEMINI.md`), Prismctl only updates the content inside markers and keeps everything else untouched:

```markdown
User content...

<!-- prismctl:start -->
Prismctl-managed content
<!-- prismctl:end -->

User content...
```

Current managed-block targets:

- `~/.gemini/GEMINI.md` (global memory; markers `<!-- prismctl:start -->` / `<!-- prismctl:end -->`)
- `~/.gemini/.env` (env var block; markers `# prismctl:start` / `# prismctl:end`)
- `<project>/.gemini/GEMINI.md` (project memory; created by `prismctl project init`)

## 3) Explicit overwrite (dangerous)

For files without safe merge semantics (e.g. Codex `AGENTS.md`), Prismctl requires explicit confirmation:

- `--apply --yes`
- automatic backup before overwriting

Current explicit-overwrite target:

- `~/.codex/AGENTS.md` (via `prismctl codex agent use`; backups go to `~/.codex/backup/prismctl/<timestamp>/AGENTS.md`, and the same under `--home`)
