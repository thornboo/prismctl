# 受管写入策略


Ekko 为了“尽量不破坏用户已有配置”，将写入分为三种策略。

## 1) 命名空间文件（默认）

只在工具配置目录下写入 `ekko/` 命名空间文件，由 Ekko 完全管理，可安全覆盖更新（`init`/`update` 都会覆盖这些文件）。

示例：

- Codex：`~/.codex/prompts/ekko/*`
- Claude Code：`~/.claude/commands/ekko/*`、`~/.claude/agents/ekko/*`、`~/.claude/output-styles/*`（Ekko 内置的 output styles）
- Gemini CLI：`~/.gemini/ekko/WORKFLOWS.md`

## 2) 受管块（保留用户内容）

用于“共享文件且用户可能自行扩展”的场景（例如 Gemini 的 `GEMINI.md`）。

Ekko 只更新标记块内的内容，块外内容完全保留：

```markdown
用户内容...

<!-- ekko:start -->
Ekko 管理的内容
<!-- ekko:end -->

用户内容...
```

当前使用受管块的文件：

- `~/.gemini/GEMINI.md`（全局记忆；使用 `<!-- ekko:start -->` / `<!-- ekko:end -->`）
- `~/.gemini/.env`（环境变量块；使用 `# ekko:start` / `# ekko:end`）
- `<project>/.gemini/GEMINI.md`（项目级记忆；由 `ekko project init` 创建）

## 3) 显式覆盖（危险操作）

用于“没有可靠合并语义”的文件（例如 Codex 的 `AGENTS.md`）。

策略：

- 需要显式确认：`--apply --yes`
- 覆盖前自动备份旧文件到备份目录

目前显式覆盖的文件：

- `~/.codex/AGENTS.md`（通过 `ekko codex agent use` 切换；会在写入前把旧文件备份到 `~/.codex/backup/ekko/<timestamp>/AGENTS.md`，`--home` 场景下同理）
