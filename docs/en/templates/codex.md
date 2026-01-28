# Codex Templates


## Paths

```text
~/.codex/
├── prompts/
│   └── prismctl/           # Prismctl namespace (fully owned)
│       ├── workflow.md
│       ├── git-commit.md
│       └── ...
└── AGENTS.md           # system prompt (explicit overwrite, dangerous)
```

> Tip: all paths can be redirected via `--home "<PATH>"` or `PRISMCTL_HOME`.

## Built-in prompts (`prompts/prismctl/`)

| Template | Description |
|------|------|
| `workflow.md` | six-phase development workflow |
| `git-commit.md` | conventional commit helper |
| `git-worktree.md` | worktree helper |
| `git-rollback.md` | rollback helper |
| `git-cleanBranches.md` | clean merged branches |
| `init-project.md` | project init helper |
| `feat.md` | feature workflow |
| `bmad-init.md` | BMAD init (trimmed) |

## Usage

### prompts

Use `/prompts:<name>` (without extension) in Codex CLI, for example:

```text
/prompts:workflow
```

These templates are Prismctl-owned; `prismctl update` will overwrite them to sync to the latest Prismctl version.

### AGENTS (dangerous)

There are 6 built-in agent templates:

- `prismctl-engineer-professional`
- `prismctl-laowang-engineer`
- `prismctl-leibus-engineer`
- `prismctl-nekomata-engineer`
- `prismctl-ojousama-engineer`
- `prismctl-rem-engineer`

Switching overwrites `~/.codex/AGENTS.md`, requires `--yes`, and creates a backup before overwriting.

Backup location:

- `~/.codex/backup/prismctl/<timestamp>/AGENTS.md`

Recovery suggestion:

1. Switch to a built-in template close to what you want
2. Manually copy/merge your custom changes from the backup directory
