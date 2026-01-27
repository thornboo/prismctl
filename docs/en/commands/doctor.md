# `ekko doctor`


Print resolved paths for Ekko and supported tools (no writes).

Typical use cases:

- Verify `--home "<PATH>"` / `EKKO_HOME` is applied (avoid writing into your real HOME)
- Locate where Ekko will read/write configs and templates

```bash
ekko doctor [--home <PATH>]
```

Example output:

```text
Ekko HOME: /Users/you
Codex root: /Users/you/.codex
Claude root: /Users/you/.claude
Claude settings: /Users/you/.claude/settings.json
Gemini root: /Users/you/.gemini
```

Notes:

- `Ekko HOME` is the current "logical HOME" (affected by `--home`, `EKKO_HOME`, or system HOME)
- `<tool> root` are the directories Ekko targets (e.g. `.codex/`, `.claude/`, `.gemini/`)
