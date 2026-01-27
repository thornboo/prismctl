# `ekko gemini`


This page covers Gemini CLI subcommands: env configuration and managed block updates.

## `ekko gemini env set`

Write/update `~/.gemini/.env` while only maintaining an Ekko-managed block (preserves content outside the block).

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

Files written:

- `~/.gemini/.env` (or `<home>/.gemini/.env` under `--home`)

Managed block format (example):

```dotenv
# other user-managed keys...

# ekko:start
GOOGLE_GEMINI_BASE_URL="https://example.com"
GEMINI_API_KEY="xxx"
GEMINI_MODEL="gemini-2.0-flash"
# ekko:end
```

> Recommendation: keep your own variables outside the Ekko-managed block to avoid being overwritten by Ekko updates.
