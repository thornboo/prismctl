# `ekko skill`


管理 Claude Code 的 skills（默认目录：`~/.claude/skills/`）。

## 列表

```bash
ekko skill list [--home <PATH>]
```

## 安装内置 skill

```bash
ekko skill install --name <NAME> [--home <PATH>] [--dry-run|--apply]
```

## 创建 skill 模板

```bash
ekko skill create --name <NAME> [--home <PATH>] [--dry-run|--apply]
```

## 删除 skill（危险操作）

```bash
ekko skill remove --name <NAME> [--home <PATH>] [--dry-run|--apply] --yes
```

## 命名规则

为避免路径穿越等风险，`--name` 仅允许：ASCII 字母/数字/连字符（`-`）/下划线（`_`）。

更多说明见：`../skills/overview.md`。

