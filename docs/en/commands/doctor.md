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

User-scoped files (existence):
  - ~/.claude/settings.json: yes
  - ~/.claude.json: yes
  - ~/.codex/config.toml: yes
  - ~/.codex/auth.json: yes
  - ~/.codex/AGENTS.md: yes
  - ~/.gemini/.env: yes
  - ~/.gemini/settings.json: yes

Project root (cwd): /path/to/project
Project-scoped files (existence):
  - .mcp.json: no
  - AGENTS.md: no
  - .gemini/.env: no
  - .gemini/settings.json: no
  - .gemini/GEMINI.md: no
```

Notes:

- `Prismctl HOME` is the current "logical HOME" (affected by `--home`, `PRISMCTL_HOME`, or system HOME)
- `<tool> root` are the directories Prismctl targets (e.g. `.codex/`, `.claude/`, `.gemini/`)
- `Project root (cwd)` is a best-effort hint based on the current working directory. It only checks file existence (no config parsing/merging semantics).
