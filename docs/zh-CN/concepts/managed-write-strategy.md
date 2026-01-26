# 受管写入策略


Ekko 为了“尽量不破坏用户已有配置”，将写入分为三种策略。

## 1) 命名空间文件（默认）

只在工具配置目录下写入 `ekko/` 命名空间文件，由 Ekko 完全管理，可安全覆盖更新。

示例：

- Codex：`~/.codex/prompts/ekko/*`
- Claude Code：`~/.claude/commands/ekko/*`、`~/.claude/agents/ekko/*`

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

## 3) 显式覆盖（危险操作）

用于“没有可靠合并语义”的文件（例如 Codex 的 `AGENTS.md`）。

策略：

- 需要显式确认：`--apply --yes`
- 覆盖前自动备份旧文件到备份目录

