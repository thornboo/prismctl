# `ekko codex`


本页覆盖 Codex 相关子命令：provider 配置与 agent（AGENTS.md）管理。

## `ekko codex provider set`

配置 Codex 模型提供商。

```bash
ekko codex provider set [--home <PATH>] [--dry-run|--apply] [--provider <VALUE>] [--api-key <VALUE>] [--base-url <VALUE>] [--model <VALUE>] [--wire-api <VALUE>] [--default]
```

参数说明：

- `--provider`：使用内置预设（`openrouter` / `deepseek` / `ollama` / `volcengine` / `siliconflow`）
- `--base-url` / `--model` / `--wire-api`：显式覆盖对应字段（优先级高于预设）
- `--api-key`：写入到 `auth.json` 的 `EKKO_CODEX_API_KEY`（不会明文落到 `config.toml`）
- `--default`：把 `model_provider` 设为 `ekko`（即让 Ekko provider 成为默认 provider）

写入文件（默认 HOME 下）：

- `~/.codex/config.toml`：新增/更新 `[model_providers.ekko]`，可选更新 `model_provider = "ekko"`
- `~/.codex/auth.json`：写入 `EKKO_CODEX_API_KEY`

示例：

```bash
ekko codex provider set \
  --provider "openrouter" \
  --api-key "sk-xxx" \
  --default \
  --apply
```

常用用法补充：

- 仅更新默认 provider（不改 base_url/model）：`ekko codex provider set --default --apply`
- 仅写入 API Key（不改 config.toml）：`ekko codex provider set --api-key "sk-xxx" --apply`
- 完全自定义 provider（不使用预设）：

```bash
ekko codex provider set \
  --base-url "https://api.example.com/v1" \
  --wire-api "chat" \
  --model "gpt-4.1" \
  --api-key "sk-xxx" \
  --apply
```

## `ekko codex agent list`

列出内置的 Codex agent 模板。

```bash
ekko codex agent list
```

## `ekko codex agent use`（危险操作）

切换 Codex 系统提示（AGENTS.md）。

⚠️ 危险操作：在 `--apply` 时会覆盖现有 `AGENTS.md`，需要 `--yes`，并在覆盖前自动备份（如存在旧文件）。

```bash
ekko codex agent use --name <VALUE> [--home <PATH>] [--lang <zh-CN|en>] [--dry-run|--apply] [--yes]
```

行为细节：

- 目标文件：`~/.codex/AGENTS.md`（`--home` 场景下会写入 `<home>/.codex/AGENTS.md`）
- 备份位置：`~/.codex/backup/ekko/<timestamp>/AGENTS.md`（仅当旧文件存在且非空）
- `dry-run` 不需要 `--yes`，并且不会写入/备份

示例：

```bash
# 先预览（不会覆盖）
ekko codex agent use --name "ekko-engineer-professional"

# 真正切换（需要显式确认）
ekko codex agent use --name "ekko-engineer-professional" --apply --yes
```
