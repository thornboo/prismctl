# Claude Code Templates


## Paths

```text
~/.claude/
├── commands/
│   └── prismctl/           # Prismctl namespace (fully owned)
├── agents/
│   └── prismctl/           # Prismctl namespace (fully owned)
├── output-styles/      # Prismctl output styles (fully owned)
└── skills/             # Claude skills (managed via `prismctl skill`)
```

> Tip: all paths can be redirected via `--home "<PATH>"` or `PRISMCTL_HOME`.

## Built-in commands (`commands/prismctl/`)

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

## Built-in agents (`agents/prismctl/`)

- `common/get-current-datetime.md`
- `common/init-architect.md`
- `plan/planner.md`
- `plan/ui-ux-designer.md`

## Built-in output styles (6)

- `prismctl-engineer-professional`
- `prismctl-laowang-engineer`
- `prismctl-leibus-engineer`
- `prismctl-nekomata-engineer`
- `prismctl-ojousama-engineer`
- `prismctl-rem-engineer`

## Usage

### commands

Use slash commands in Claude Code (filename without `.md`):

```text
/workflow
/git-commit
```

### output styles

```bash
prismctl claude output-style use --name "prismctl-engineer-professional" --apply
```

This updates `outputStyle` in `~/.claude/settings.json` (see `../commands/claude.md`).

### skills

```bash
prismctl skill list
prismctl skill install --name "explain-code" --apply
```

For skill structure and conventions, see: `../skills/overview.md`.
