# `ekko codex`


本页覆盖 Codex 相关子命令：provider 配置与 agent（AGENTS.md）管理。

## `ekko codex provider set`

配置 Codex 模型提供商。

```bash
ekko codex provider set [--home <PATH>] [--dry-run|--apply] [--provider <VALUE>] [--api-key <VALUE>] [--base-url <VALUE>] [--model <VALUE>] [--wire-api <VALUE>] [--default]
```

说明：

- `--provider`：使用内置预设（openrouter/deepseek/ollama/volcengine/siliconflow）
- 显式参数（`--base-url/--model/--wire-api`）优先级高于预设
- API Key 会写入 `~/.codex/auth.json`（对应的 `temp_env_key`），不会写入 `config.toml` 明文
- `--default` 会把 Ekko provider 设置为默认（具体行为取决于 Codex 配置格式）

示例：

```bash
ekko codex provider set \
  --provider "openrouter" \
  --api-key "sk-xxx" \
  --default \
  --apply
```

写入文件：

- `~/.codex/config.toml`：新增/更新 `[model_providers.ekko]`
- `~/.codex/auth.json`：写入 `EKKO_CODEX_API_KEY`

## `ekko codex agent list`

列出内置的 Codex agent 模板。

```bash
ekko codex agent list
```

## `ekko codex agent use`（危险操作）

切换 Codex 系统提示（AGENTS.md）。

⚠️ 危险操作：会覆盖现有 `AGENTS.md`，需要 `--yes`，并在覆盖前自动备份。

```bash
ekko codex agent use --name <VALUE> [--home <PATH>] [--lang <zh-CN|en>] [--dry-run|--apply] --yes
```
