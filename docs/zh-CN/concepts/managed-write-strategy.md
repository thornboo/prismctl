# 受管写入策略


Prismctl 为了“尽量不破坏用户已有配置”，将写入分为四种策略。

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
- `~/.gemini/.env`（环境变量块；使用 `# prismctl:start` / `# prismctl:end`；目前仅管理 `GEMINI_API_KEY`）
- `<project>/.gemini/.env`（项目级环境变量块；通过 `prismctl gemini env set --scope project` 写入）
- `<project>/.gemini/GEMINI.md`（项目级记忆；由 `prismctl project init` 创建）

## 3) 结构化 upsert（JSON/TOML merge）

用于“结构化配置文件”的场景：Prismctl 会解析 JSON/TOML，按字段 upsert（插入/更新）指定配置，并保留未触及字段。

注意：为了保证输出稳定与可读性，Prismctl 通常会以 pretty 格式重写文件（可能改变缩进/排序），但语义保持不变。

当前使用结构化 upsert 的文件（示例）：

- `~/.claude/settings.json`（Claude env / outputStyle 等）
- `~/.codex/config.toml`、`~/.codex/auth.json`（Codex provider / key）
- `~/.gemini/settings.json`、`<project>/.gemini/settings.json`（Gemini settings，例如 `model.name`）

## 4) 显式覆盖（危险操作）

用于“没有可靠合并语义”的文件（例如 Codex 的 `AGENTS.md`）。

策略：

- 需要显式确认：`--apply --yes`
- 覆盖前自动备份旧文件到备份目录

目前显式覆盖的文件：

- `AGENTS.md`（通过 `prismctl codex agent use` 切换；支持 user/project scope）
  - user scope：覆盖 `~/.codex/AGENTS.md`，并备份到 `~/.codex/backup/prismctl/<timestamp>/AGENTS.md`
  - project scope：覆盖 `<project>/AGENTS.md`，并备份到 `<project>/.prismctl/backup/prismctl/<timestamp>/AGENTS.md`
