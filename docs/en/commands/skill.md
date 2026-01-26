# `ekko skill`


Manage Claude Code skills (default directory: `~/.claude/skills/`).

## List

```bash
ekko skill list [--home <PATH>]
```

## Install a built-in skill

```bash
ekko skill install --name <NAME> [--home <PATH>] [--dry-run|--apply]
```

## Create a skill template

```bash
ekko skill create --name <NAME> [--home <PATH>] [--dry-run|--apply]
```

## Remove a skill (dangerous)

```bash
ekko skill remove --name <NAME> [--home <PATH>] [--dry-run|--apply] --yes
```

## Naming rules

To prevent path traversal, `--name` only allows ASCII letters/digits plus `-` and `_`.

See: `../skills/overview.md`.

