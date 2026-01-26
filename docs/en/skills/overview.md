# Skills


Ekko `skill` subcommands manage Claude Code skills under `~/.claude/skills/`.

## Safety

- Dry-run by default (no writes)
- Use `--apply` to write/delete
- Removal is dangerous and requires `--yes`
- Use `--home "/tmp/ekko-home"` to practice in a sandbox first

## Built-in skills

Ekko ships 3 built-in skills:

- `explain-code`
- `codebase-visualizer`
- `pr-summary`

Install path: `~/.claude/skills/<name>/`

## Common commands

```bash
ekko skill list
ekko skill install --name "explain-code" --apply
ekko skill create --name "my-skill" --apply
ekko skill remove --name "my-skill" --apply --yes
```

Full syntax: `../commands/skill.md`.

## About codebase-visualizer

`codebase-visualizer` installs a Python script:

```bash
python ~/.claude/skills/codebase-visualizer/scripts/visualize.py .
```

It generates `codebase-map.html` in the current directory and may try to open it in your browser.

