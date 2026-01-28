# `prismctl init`


Initialize built-in templates (recommended for first-time setup).

It writes Prismctl-managed, namespaced template files into each tool's config directory (see `../templates/index.md`). Prismctl is `dry-run` by default: preview first, then add `--apply`.

```bash
prismctl init --tool <codex|claude|gemini|all> [--home <PATH>] [--lang <zh-CN|en>] [--dry-run|--apply]
```

Quick alias (hybrid):

```bash
prismctl i --tool <codex|claude|gemini|all> [--lang <zh-CN|en>] [--home <PATH>] [--dry-run|--apply]
```

## What you get

- Codex: `~/.codex/prompts/prismctl/*` (workflow and git helpers)
- Claude Code: `~/.claude/commands/prismctl/*`, `~/.claude/agents/prismctl/*`, `~/.claude/output-styles/*`
- Gemini CLI: `~/.gemini/prismctl/WORKFLOWS.md` and an Prismctl-managed block in `~/.gemini/GEMINI.md`

> Note: namespaced files are fully owned by Prismctl and will be overwritten by `update`. If you need long-lived customization, copy them to a non-`prismctl/` location and reference your copy from the tool.

Examples:

```bash
# Preview all planned template writes
prismctl init --tool all

# Apply into a sandbox HOME
prismctl init --tool all --home "/tmp/prismctl-home" --apply

# Only initialize Claude templates
prismctl init --tool claude --apply
```

See also:

- `../concepts/safety-model.md`
- `../concepts/managed-write-strategy.md`
