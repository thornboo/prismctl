# `ekko skill`


Manage Claude Code skills (default directory: `~/.claude/skills/`).

Ekko only manages the on-disk files/directories (still following `dry-run` / `--apply` and the `--home` sandbox). Skill loading/execution behavior is defined by Claude Code itself.

## List

```bash
ekko skill list [--home <PATH>]
```

The output includes:

- Ekko built-in skills (and whether each is installed)
- Installed skills on your machine (parsed from each `~/.claude/skills/<name>/SKILL.md` frontmatter `name/description`)

## Install a built-in skill

```bash
ekko skill install --name <NAME> [--home <PATH>] [--dry-run|--apply]
```

Install writes (overwrites) files under `~/.claude/skills/<NAME>/`.

## Create a skill template

```bash
ekko skill create --name <NAME> [--home <PATH>] [--dry-run|--apply]
```

Create does not overwrite an existing `SKILL.md` (it is "write-if-missing"), so you can rerun it without clobbering your local edits.

## Remove a skill (dangerous)

```bash
ekko skill remove --name <NAME> [--home <PATH>] [--dry-run|--apply] [--yes]
```

WARNING: Only `--apply` performs the deletion; when applying, `--yes` is required.

## Naming rules

To prevent path traversal, `--name` only allows ASCII letters/digits plus `-` and `_`.

See: `../skills/overview.md`.
