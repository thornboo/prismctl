# `ekko init`


Initialize built-in templates (recommended for first-time setup).

It writes Ekko-managed, namespaced template files into each tool's config directory (see `../templates/index.md`). Ekko is `dry-run` by default: preview first, then add `--apply`.

```bash
ekko init --tool <codex|claude|gemini|all> [--home <PATH>] [--lang <zh-CN|en>] [--dry-run|--apply]
```

## What you get

- Codex: `~/.codex/prompts/ekko/*` (workflow and git helpers)
- Claude Code: `~/.claude/commands/ekko/*`, `~/.claude/agents/ekko/*`, `~/.claude/output-styles/*`
- Gemini CLI: `~/.gemini/ekko/WORKFLOWS.md` and an Ekko-managed block in `~/.gemini/GEMINI.md`

> Note: namespaced files are fully owned by Ekko and will be overwritten by `update`. If you need long-lived customization, copy them to a non-`ekko/` location and reference your copy from the tool.

Examples:

```bash
# Preview all planned template writes
ekko init --tool all

# Apply into a sandbox HOME
ekko init --tool all --home "/tmp/ekko-home" --apply

# Only initialize Claude templates
ekko init --tool claude --apply
```

See also:

- `../concepts/safety-model.md`
- `../concepts/managed-write-strategy.md`
