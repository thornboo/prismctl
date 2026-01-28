# Gemini CLI 模板


## 写入位置

```text
~/.gemini/
├── .env                # Gemini CLI 环境变量
├── GEMINI.md           # 全局记忆（受管块写入）
├── settings.json       # 用户级设置（structured upsert）
└── prismctl/
    └── WORKFLOWS.md    # 说明性文档（命名空间文件）
```

> 提示：所有路径均可通过 `--home "<PATH>"` 或 `PRISMCTL_HOME` 重定向到沙箱 HOME。

## 受管块（GEMINI.md）

Prismctl 只更新标记块内的内容，块外内容完全保留：

```markdown
<!-- prismctl:start -->
Prismctl 管理的内容
<!-- prismctl:end -->
```

对应命令：

- `prismctl init --tool gemini ...` / `prismctl update --tool gemini ...`：更新 `~/.gemini/GEMINI.md` 的受管块
- `prismctl project init ...`：更新 `<project>/.gemini/GEMINI.md` 的受管块

## 受管块（.env）

`prismctl gemini env set` 会在 `~/.gemini/.env` 中维护一个 Prismctl 受管块（`# prismctl:start` / `# prismctl:end`），用于写入：

- `GEMINI_API_KEY`

此外也支持项目级：`<project>/.gemini/.env`（通过 `--scope project`）。

## settings.json（structured upsert）

`prismctl gemini settings set` 会在 `settings.json` 中 upsert `model.name`：

- 用户级：`~/.gemini/settings.json`
- 项目级：`<project>/.gemini/settings.json`

## MCP（依赖 gemini CLI）

`prismctl gemini mcp ...` 委托 `gemini mcp ...`，把 MCP servers 写入 `settings.json` 的 `mcpServers`（scope=user/project 对应用户级/项目级 settings.json）。

## 记忆加载层级（Gemini CLI）

Gemini CLI 会按目录层级加载 `GEMINI.md` 作为上下文：

1. 全局：`~/.gemini/GEMINI.md`
2. 项目：`<project>/.gemini/GEMINI.md`
3. 子目录：`<subdir>/.gemini/GEMINI.md`
