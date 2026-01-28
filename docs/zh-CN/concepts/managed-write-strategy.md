# 受管写入策略


Prismctl 为了“尽量不破坏用户已有配置”，将写入分为三种策略。

## 1) 命名空间文件（默认）

只在工具配置目录下写入 `prismctl/` 命名空间文件，由 Prismctl 完全管理，可安全覆盖更新（`init`/`update` 都会覆盖这些文件）。

示例：

- Codex：`~/.codex/prompts/prismctl/*`
- Claude Code：`~/.claude/commands/prismctl/*`、`~/.claude/agents/prismctl/*`、`~/.claude/output-styles/*`（Prismctl 内置的 output styles）
- Gemini CLI：`~/.gemini/prismctl/WORKFLOWS.md`

## 2) 受管块（保留用户内容）

用于“共享文件且用户可能自行扩展”的场景（例如 Gemini 的 `GEMINI.md`）。

Prismctl 只更新标记块内的内容，块外内容完全保留：

```markdown
用户内容...

<!-- prismctl:start -->
Prismctl 管理的内容
<!-- prismctl:end -->

用户内容...
```

当前使用受管块的文件：

- `~/.gemini/GEMINI.md`（全局记忆；使用 `<!-- prismctl:start -->` / `<!-- prismctl:end -->`）
- `~/.gemini/.env`（环境变量块；使用 `# prismctl:start` / `# prismctl:end`）
- `<project>/.gemini/GEMINI.md`（项目级记忆；由 `prismctl project init` 创建）

## 3) 显式覆盖（危险操作）

用于“没有可靠合并语义”的文件（例如 Codex 的 `AGENTS.md`）。

策略：

- 需要显式确认：`--apply --yes`
- 覆盖前自动备份旧文件到备份目录

目前显式覆盖的文件：

- `~/.codex/AGENTS.md`（通过 `prismctl codex agent use` 切换；会在写入前把旧文件备份到 `~/.codex/backup/prismctl/<timestamp>/AGENTS.md`，`--home` 场景下同理）
