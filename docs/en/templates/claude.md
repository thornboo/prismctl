# Claude Code Templates


## Paths

```
~/.claude/
├── commands/
│   └── ekko/           # Ekko namespace (fully owned)
├── agents/
│   └── ekko/           # Ekko namespace (fully owned)
├── output-styles/      # Ekko output styles (fully owned)
└── skills/             # Claude skills (managed via `ekko skill`)
```

## Built-in commands (`commands/ekko/`)

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

## Built-in agents (`agents/ekko/`)

- `common/get-current-datetime.md`
- `common/init-architect.md`
- `plan/planner.md`
- `plan/ui-ux-designer.md`

## Built-in output styles (6)

- `ekko-engineer-professional`
- `ekko-laowang-engineer`
- `ekko-leibus-engineer`
- `ekko-nekomata-engineer`
- `ekko-ojousama-engineer`
- `ekko-rem-engineer`

## Usage

### commands

Use slash commands in Claude Code (filename without `.md`):

```text
/workflow
/git-commit
```

### output styles

```bash
ekko claude output-style use --name "ekko-engineer-professional" --apply
```

### skills

```bash
ekko skill list
ekko skill install --name "explain-code" --apply
```
