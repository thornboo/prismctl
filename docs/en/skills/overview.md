# Skills


Prismctl `skill` subcommands manage Claude Code skills under `~/.claude/skills/`.

Skills are a lightweight extension mechanism in Claude Code: each skill is a directory that contains at least `SKILL.md` (typically with YAML frontmatter).

## Safety

- Dry-run by default (no writes)
- Use `--apply` to write/delete
- Removal is dangerous and requires `--yes`
- Use `--home "/tmp/prismctl-home"` to practice in a sandbox first

## Directory conventions

A typical skill directory looks like:

```text
~/.claude/skills/<skill-name>/
├── SKILL.md              # required: behavior spec (often with frontmatter)
├── scripts/              # optional: scripts/tools
├── examples/             # optional: examples
└── references/           # optional: snippets/references
```

Common `SKILL.md` frontmatter:

```yaml
---
name: explain-code
description: Explain code with diagrams and actionable notes
---
```

## Built-in skills

Prismctl ships 3 built-in skills:

- `explain-code`
- `codebase-visualizer`
- `pr-summary`

Install path: `~/.claude/skills/<name>/`

## Common commands

```bash
prismctl skill list
prismctl skill install --name "explain-code" --apply
prismctl skill create --name "my-skill" --apply
prismctl skill remove --name "my-skill" --apply --yes
```

Full syntax: `../commands/skill.md`.

## About codebase-visualizer

`codebase-visualizer` installs a Python script:

```bash
python ~/.claude/skills/codebase-visualizer/scripts/visualize.py .
```

It generates `codebase-map.html` in the current directory and may try to open it in your browser.

> Note: behavior depends on your local Python and GUI/browser environment. In headless environments it typically only generates the HTML file.
