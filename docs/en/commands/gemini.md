# `ekko gemini`


This page covers Gemini CLI subcommands: env configuration and managed block updates.

## `ekko gemini env set`

Write/update `~/.gemini/.env`.

Managed block keys:

- `GEMINI_API_KEY`
- `GOOGLE_GEMINI_BASE_URL` (optional)
- `GEMINI_MODEL` (optional)

```bash
ekko gemini env set [--home <PATH>] [--dry-run|--apply] [--api-key <VALUE>] [--base-url <VALUE>] [--model <VALUE>]
```

Example:

```bash
ekko gemini env set --api-key "xxx" --model "gemini-2.0-flash" --apply
```
