# Command Reference


This section documents Ekko CLI commands, split by feature area.

## Conventions (read this first)

- Ekko is `dry-run` by default: it prints the plan but does not write/delete files, and does not execute external commands.
- Only `--apply` performs real writes / installations.
- Use `--home "<PATH>"` or `EKKO_HOME` to redirect all I/O into a sandbox HOME.
- For irreversible/high-risk operations, Ekko requires an explicit `--yes` (e.g. removing a skill, global install/upgrade, overwriting Codex `AGENTS.md`).

## Common Flags

> Not every flag applies to every command; refer to the specific command page.

| Option | Description |
|------|------|
| `--home <PATH>` | Redirect all I/O into a sandbox HOME |
| `--dry-run` | Preview changes without writing (default) |
| `--apply` | Apply changes (write to disk) |
| `--lang <zh-CN|en>` | Template language (default: `zh-CN`) |
| `--tool <codex|claude|gemini|all>` | Select target tools for init/update/install |
| `--yes` | Explicit confirmation for dangerous operations (usually with `--apply`) |
| `-h, --help` | Show help |

## Basics

- `doctor`: `./doctor.md`
- `init`: `./init.md`
- `update`: `./update.md`

## Config & Management

- Install/upgrade tools (npm/brew): `./install-upgrade.md`
- Skills: `./skill.md`
- Codex: `./codex.md`
- Claude Code: `./claude.md`
- Gemini CLI: `./gemini.md`
- Project init: `./project.md`
