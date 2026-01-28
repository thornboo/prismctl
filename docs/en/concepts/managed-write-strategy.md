# Managed Write Strategy


To avoid breaking user-owned configuration, Prismctl uses four write strategies.

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
- `~/.gemini/.env` (env var block; markers `# prismctl:start` / `# prismctl:end`; currently only manages `GEMINI_API_KEY`)
- `<project>/.gemini/.env` (project env var block; written via `prismctl gemini env set --scope project`)
- `<project>/.gemini/GEMINI.md` (project memory; created by `prismctl project init`)

## 3) Structured upsert (JSON/TOML merge)

For structured config files, Prismctl parses JSON/TOML and upserts (inserts/updates) only the intended fields while preserving everything else.

Note: Prismctl may rewrite the file in a stable pretty format (indent/order changes) while keeping the semantics intact.

Examples:

- `~/.claude/settings.json` (Claude env / outputStyle)
- `~/.codex/config.toml`, `~/.codex/auth.json` (Codex provider / key)
- `~/.gemini/settings.json`, `<project>/.gemini/settings.json` (Gemini settings such as `model.name`)

## 4) Explicit overwrite (dangerous)

For files without safe merge semantics (e.g. Codex `AGENTS.md`), Prismctl requires explicit confirmation:

- `--apply --yes`
- automatic backup before overwriting

Current explicit-overwrite target:

- `AGENTS.md` (via `prismctl codex agent use`; supports user/project scope)
  - user scope: overwrite `~/.codex/AGENTS.md`, backup to `~/.codex/backup/prismctl/<timestamp>/AGENTS.md`
  - project scope: overwrite `<project>/AGENTS.md`, backup to `<project>/.prismctl/backup/prismctl/<timestamp>/AGENTS.md`
