# `prismctl gemini`


本页覆盖 Gemini CLI 相关子命令：环境配置与受管块写入。

## `prismctl gemini env set`

写入/更新 `~/.gemini/.env`，并且仅维护一个 Prismctl 受管块（不会覆盖块外内容）。

受管块 keys：

- `GEMINI_API_KEY`
- `GOOGLE_GEMINI_BASE_URL`（可选）
- `GEMINI_MODEL`（可选）

```bash
prismctl gemini env set [--home <PATH>] [--dry-run|--apply] [--api-key <VALUE>] [--base-url <VALUE>] [--model <VALUE>]
```

示例：

```bash
prismctl gemini env set --api-key "xxx" --model "gemini-2.0-flash" --apply
```

写入位置：

- `~/.gemini/.env`（`--home` 场景下对应 `<home>/.gemini/.env`）

受管块格式（示意）：

```dotenv
# other user-managed keys...

# prismctl:start
GOOGLE_GEMINI_BASE_URL="https://example.com"
GEMINI_API_KEY="xxx"
GEMINI_MODEL="gemini-2.0-flash"
# prismctl:end
```

> 建议：把你自己维护的变量放在 Prismctl 受管块之外，避免被 Prismctl 的受管块更新覆盖。
