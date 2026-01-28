# `prismctl doctor`


Print resolved paths for Prismctl and supported tools (no writes).

Typical use cases:

- Verify `--home "<PATH>"` / `PRISMCTL_HOME` is applied (avoid writing into your real HOME)
- Locate where Prismctl will read/write configs and templates

```bash
prismctl doctor [--home <PATH>]
```

Example output:

```text
Prismctl HOME: /Users/you
Codex root: /Users/you/.codex
Claude root: /Users/you/.claude
Claude settings: /Users/you/.claude/settings.json
Gemini root: /Users/you/.gemini
```

Notes:

- `Prismctl HOME` is the current "logical HOME" (affected by `--home`, `PRISMCTL_HOME`, or system HOME)
- `<tool> root` are the directories Prismctl targets (e.g. `.codex/`, `.claude/`, `.gemini/`)
