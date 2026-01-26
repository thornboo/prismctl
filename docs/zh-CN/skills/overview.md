# Skills


Ekko 的 `skill` 子命令用于管理 Claude Code 的 skills 目录：`~/.claude/skills/`。

## 安全模型

- 默认 `--dry-run`：只打印将执行的变更，不落盘
- 传入 `--apply` 才会写入/删除
- 删除是危险操作：必须额外传入 `--yes`
- 推荐先用 `--home "/tmp/ekko-home"` 沙箱演练

## 内置 skills

Ekko 内置 3 个 skills，可直接安装：

- `explain-code`
- `codebase-visualizer`
- `pr-summary`

安装位置：`~/.claude/skills/<name>/`

## 常用命令

```bash
ekko skill list
ekko skill install --name "explain-code" --apply
ekko skill create --name "my-skill" --apply
ekko skill remove --name "my-skill" --apply --yes
```

完整语法见：`../commands/skill.md`。

## 关于 codebase-visualizer

`codebase-visualizer` 会安装一个 Python 脚本：

```bash
python ~/.claude/skills/codebase-visualizer/scripts/visualize.py .
```

脚本会在当前目录生成 `codebase-map.html` 并尝试自动打开浏览器。

