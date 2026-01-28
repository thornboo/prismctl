# Codex 模板


## 写入位置

```text
~/.codex/
├── prompts/
│   └── prismctl/           # Prismctl 命名空间（完全受管）
│       ├── workflow.md
│       ├── git-commit.md
│       └── ...
└── AGENTS.md               # 系统提示（显式覆盖，危险操作；user scope）
```

> 提示：所有路径均可通过 `--home "<PATH>"` 或 `PRISMCTL_HOME` 重定向到沙箱 HOME。

## 内置 prompts（`prompts/prismctl/`）

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

## 使用方式

### prompts

在 Codex CLI 中使用 `/prompts:<name>`（不带扩展名）：

例如：

```text
/prompts:workflow
```

这些模板文件属于 Prismctl 命名空间，`prismctl update` 会覆盖同步到新版本。

### AGENTS（危险操作）

AGENTS 模板共 6 个：

- `prismctl-engineer-professional`
- `prismctl-laowang-engineer`
- `prismctl-leibus-engineer`
- `prismctl-nekomata-engineer`
- `prismctl-ojousama-engineer`
- `prismctl-rem-engineer`

切换方式：

```bash
prismctl codex agent list
prismctl codex agent use --name "prismctl-engineer-professional" --apply --yes

# 项目级（写入项目根目录 AGENTS.md）
prismctl codex agent use --name "prismctl-engineer-professional" --scope project --project-path "/path/to/your/project" --apply --yes
```

切换会覆盖目标 `AGENTS.md`，覆盖前会备份旧文件到：

- user scope：`~/.codex/backup/prismctl/<timestamp>/AGENTS.md`
- project scope：`<project>/.prismctl/backup/prismctl/<timestamp>/AGENTS.md`

恢复建议：

1. 先用 `prismctl codex agent use ...` 切到一个“接近”的内置模板
2. 再从备份目录手动复制/合并你的自定义内容
