# Managed Write Strategy


To avoid breaking user-owned configuration, Ekko uses three write strategies.

## 1) Namespaced files (default)

Ekko only writes into an `ekko/` namespace directory and fully owns those files, so it is safe to overwrite/update them (`init` and `update` will overwrite these).

Examples:

- Codex: `~/.codex/prompts/ekko/*`
- Claude Code: `~/.claude/commands/ekko/*`, `~/.claude/agents/ekko/*`, `~/.claude/output-styles/*` (Ekko built-ins)
- Gemini CLI: `~/.gemini/ekko/WORKFLOWS.md`

## 2) Managed blocks (preserve user content)

For shared files that users may extend (e.g. Gemini `GEMINI.md`), Ekko only updates the content inside markers and keeps everything else untouched:

```markdown
User content...

<!-- ekko:start -->
Ekko-managed content
<!-- ekko:end -->

User content...
```

Current managed-block targets:

- `~/.gemini/GEMINI.md` (global memory; markers `<!-- ekko:start -->` / `<!-- ekko:end -->`)
- `~/.gemini/.env` (env var block; markers `# ekko:start` / `# ekko:end`)
- `<project>/.gemini/GEMINI.md` (project memory; created by `ekko project init`)

## 3) Explicit overwrite (dangerous)

For files without safe merge semantics (e.g. Codex `AGENTS.md`), Ekko requires explicit confirmation:

- `--apply --yes`
- automatic backup before overwriting

Current explicit-overwrite target:

- `~/.codex/AGENTS.md` (via `ekko codex agent use`; backups go to `~/.codex/backup/ekko/<timestamp>/AGENTS.md`, and the same under `--home`)
