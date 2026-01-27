# `ekko skill`


管理 Claude Code 的 skills（默认目录：`~/.claude/skills/`）。

Ekko 的 `skill` 命令只负责对“文件结构”进行增删改（同样遵循 `dry-run` / `--apply` / `--home` 沙箱）；至于 skill 的实际执行与加载机制由 Claude Code 本身决定。

## 列表

```bash
ekko skill list [--home <PATH>]
```

输出会同时显示：

- Ekko 内置 skills（以及是否已安装）
- 你本机已安装 skills（从 `~/.claude/skills/<name>/SKILL.md` 读取 frontmatter 的 `name/description`）

## 安装内置 skill

```bash
ekko skill install --name <NAME> [--home <PATH>] [--dry-run|--apply]
```

安装会写入（覆盖）`~/.claude/skills/<NAME>/` 下的文件。

## 创建 skill 模板

```bash
ekko skill create --name <NAME> [--home <PATH>] [--dry-run|--apply]
```

创建默认不会覆盖已存在的 `SKILL.md`（等价于“仅在缺失时写入”），方便你在同名目录里反复执行而不破坏已有内容。

## 删除 skill（危险操作）

```bash
ekko skill remove --name <NAME> [--home <PATH>] [--dry-run|--apply] [--yes]
```

⚠️ 仅当你传入 `--apply` 时才会执行删除；此时必须额外传入 `--yes`。

## 命名规则

为避免路径穿越等风险，`--name` 仅允许：ASCII 字母/数字/连字符（`-`）/下划线（`_`）。

更多说明见：`../skills/overview.md`。
