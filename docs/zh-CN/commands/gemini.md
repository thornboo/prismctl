# `ekko gemini`


本页覆盖 Gemini CLI 相关子命令：环境配置与受管块写入。

## `ekko gemini env set`

写入/更新 `~/.gemini/.env`。

受管块 keys：

- `GEMINI_API_KEY`
- `GOOGLE_GEMINI_BASE_URL`（可选）
- `GEMINI_MODEL`（可选）

```bash
ekko gemini env set [--home <PATH>] [--dry-run|--apply] [--api-key <VALUE>] [--base-url <VALUE>] [--model <VALUE>]
```

示例：

```bash
ekko gemini env set --api-key "xxx" --model "gemini-2.0-flash" --apply
```
