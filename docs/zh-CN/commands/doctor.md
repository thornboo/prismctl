# `prismctl doctor`


诊断并显示 Prismctl 解析后的目录映射（不会写入任何文件）。

你通常在以下场景使用它：

- 确认 `--home "<PATH>"` / `PRISMCTL_HOME` 是否生效（避免误写真实 HOME）
- 快速定位 Prismctl 将会读写的目录与关键配置文件

```bash
prismctl doctor [--home <PATH>]
```

输出示例：

```text
Prismctl HOME: /Users/you
Codex root: /Users/you/.codex
Claude root: /Users/you/.claude
Claude settings: /Users/you/.claude/settings.json
Gemini root: /Users/you/.gemini
```

说明：

- `Prismctl HOME` 是 Prismctl 当前的“逻辑 HOME”（受 `--home` / `PRISMCTL_HOME` / 系统 HOME 影响）
- `<tool> root` 是 Prismctl 将要写入模板/配置的根目录（例如 `.codex/`、`.claude/`、`.gemini/`）
