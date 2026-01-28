# Claude Code 模板


## 写入位置

```text
~/.claude/
├── commands/
│   └── prismctl/           # Prismctl 命名空间（完全受管）
├── agents/
│   └── prismctl/           # Prismctl 命名空间（完全受管）
├── output-styles/      # Prismctl 输出风格（完全受管）
└── skills/             # Claude skills（由 skill 命令管理）
```

> 提示：所有路径均可通过 `--home "<PATH>"` 或 `PRISMCTL_HOME` 重定向到沙箱 HOME。

## 内置 commands（`commands/prismctl/`）

| 模板 | 说明 |
|------|------|
| `workflow.md` | 六阶段开发工作流 |
| `git-commit.md` | Git 提交助手 |
| `git-worktree.md` | Git worktree 管理 |
| `git-rollback.md` | Git 回滚助手 |
| `git-cleanBranches.md` | 清理过期分支 |
| `init-project.md` | 项目初始化 |
| `feat.md` | 功能开发流程 |
| `bmad-init.md` | BMAD 初始化（精简版） |

## 内置 agents（`agents/prismctl/`）

- `common/get-current-datetime.md`
- `common/init-architect.md`
- `plan/planner.md`
- `plan/ui-ux-designer.md`

## 内置 output styles（6）

- `prismctl-engineer-professional`
- `prismctl-laowang-engineer`
- `prismctl-leibus-engineer`
- `prismctl-nekomata-engineer`
- `prismctl-ojousama-engineer`
- `prismctl-rem-engineer`

## 使用方式

### commands

在 Claude Code 中通过斜杠命令使用（文件名去掉 `.md`）：

```text
/workflow
/git-commit
```

### output styles

```bash
prismctl claude output-style use --name "prismctl-engineer-professional" --apply
```

该命令会更新 `~/.claude/settings.json` 的 `outputStyle` 字段（详见：`../commands/claude.md`）。

### skills

```bash
prismctl skill list
prismctl skill install --name "explain-code" --apply
```

技能机制与目录结构说明见：`../skills/overview.md`。
