# Templates


Ekko ships built-in templates (workflows, Git commands, agents, output styles) and supports `zh-CN` / `en`.

## Write strategy

Ekko uses a managed write strategy to avoid breaking user config (see `../concepts/managed-write-strategy.md`).

## Categories

- Codex: `./codex.md`
- Claude Code: `./claude.md`
- Gemini CLI: `./gemini.md`
- Project templates: `./project.md`

## Init & update

```bash
ekko init --tool all --apply
ekko update --tool all --apply
```

