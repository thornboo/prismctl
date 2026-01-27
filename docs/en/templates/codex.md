# Codex Templates


## Paths

```text
~/.codex/
├── prompts/
│   └── ekko/           # Ekko namespace (fully owned)
│       ├── workflow.md
│       ├── git-commit.md
│       └── ...
└── AGENTS.md           # system prompt (explicit overwrite, dangerous)
```

## Built-in prompts (`prompts/ekko/`)

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

### AGENTS (dangerous)

There are 6 built-in agent templates:

- `ekko-engineer-professional`
- `ekko-laowang-engineer`
- `ekko-leibus-engineer`
- `ekko-nekomata-engineer`
- `ekko-ojousama-engineer`
- `ekko-rem-engineer`

Switching overwrites `~/.codex/AGENTS.md`, requires `--yes`, and creates a backup before overwriting.
