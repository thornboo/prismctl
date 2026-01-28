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

用户级文件（存在性）：
  - ~/.claude/settings.json: 存在
  - ~/.claude.json: 存在
  - ~/.codex/config.toml: 存在
  - ~/.codex/auth.json: 存在
  - ~/.codex/AGENTS.md: 存在
  - ~/.gemini/.env: 存在
  - ~/.gemini/settings.json: 存在

项目根目录（当前工作目录）: /path/to/project
项目级文件（存在性）：
  - .mcp.json: 不存在
  - AGENTS.md: 不存在
  - .gemini/.env: 不存在
  - .gemini/settings.json: 不存在
  - .gemini/GEMINI.md: 不存在
```

说明：

- `Prismctl HOME` 是 Prismctl 当前的“逻辑 HOME”（受 `--home` / `PRISMCTL_HOME` / 系统 HOME 影响）
- `<tool> root` 是 Prismctl 将要写入模板/配置的根目录（例如 `.codex/`、`.claude/`、`.gemini/`）
- `项目根目录（当前工作目录）` 是基于当前工作目录（cwd）的 best-effort 输出，用于提示“项目级”配置是否存在（不会解析/合并配置语义）
