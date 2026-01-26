# Managed Write Strategy


To avoid breaking user-owned configuration, Ekko uses three write strategies.

## 1) Namespaced files (default)

Ekko only writes into an `ekko/` namespace directory and fully owns those files, so it is safe to overwrite/update them.

Examples:

- Codex: `~/.codex/prompts/ekko/*`
- Claude Code: `~/.claude/commands/ekko/*`, `~/.claude/agents/ekko/*`

## 2) Managed blocks (preserve user content)

For shared files that users may extend (e.g. Gemini `GEMINI.md`), Ekko only updates the content inside markers and keeps everything else untouched:

```markdown
User content...

<!-- ekko:start -->
Ekko-managed content
<!-- ekko:end -->

User content...
```

## 3) Explicit overwrite (dangerous)

For files without safe merge semantics (e.g. Codex `AGENTS.md`), Ekko requires explicit confirmation:

- `--apply --yes`
- automatic backup before overwriting

