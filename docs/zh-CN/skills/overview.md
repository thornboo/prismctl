# Skills


Prismctl 的 `skill` 子命令用于管理 Claude Code 的 skills 目录：`~/.claude/skills/`。

Skills 是 Claude Code 的可插拔扩展机制：每个 skill 以一个目录存在，目录中至少包含 `SKILL.md`（通常带 YAML frontmatter）。

## 安全模型

- 默认 `--dry-run`：只打印将执行的变更，不落盘
- 传入 `--apply` 才会写入/删除
- 删除是危险操作：必须额外传入 `--yes`
- 推荐先用 `--home "/tmp/prismctl-home"` 沙箱演练

## 目录结构约定

一个典型的 skill 目录如下：

```text
~/.claude/skills/<skill-name>/
├── SKILL.md              # 必需：行为说明（通常含 frontmatter）
├── scripts/              # 可选：脚本/工具
├── examples/             # 可选：示例
└── references/           # 可选：参考资料/片段
```

`SKILL.md` 的常见 frontmatter：

```yaml
---
name: explain-code
description: Explain code with diagrams and actionable notes
---
```

## 内置 skills

Prismctl 内置 3 个 skills，可直接安装：

- `explain-code`
- `codebase-visualizer`
- `pr-summary`

安装位置：`~/.claude/skills/<name>/`

## 常用命令

```bash
prismctl skill list
prismctl skill install --name "explain-code" --apply
prismctl skill create --name "my-skill" --apply
prismctl skill remove --name "my-skill" --apply --yes
```

完整语法见：`../commands/skill.md`。

## 关于 codebase-visualizer

`codebase-visualizer` 会安装一个 Python 脚本：

```bash
python ~/.claude/skills/codebase-visualizer/scripts/visualize.py .
```

脚本会在当前目录生成 `codebase-map.html` 并尝试自动打开浏览器。

> 注意：该脚本的行为依赖本机 Python 与浏览器环境；在无 GUI 的环境中通常只生成 HTML 文件而不会自动打开。
