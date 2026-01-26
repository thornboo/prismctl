# `ekko init`


Initialize built-in templates (recommended for first-time setup).

```bash
ekko init --tool <codex|claude|gemini|all> [--home <PATH>] [--lang <zh-CN|en>] [--dry-run|--apply]
```

Examples:

```bash
# Preview all planned template writes
ekko init --tool all

# Apply into a sandbox HOME
ekko init --tool all --home "/tmp/ekko-home" --apply

# Only initialize Claude templates
ekko init --tool claude --apply
```

See also:

- `../concepts/safety-model.md`
- `../concepts/managed-write-strategy.md`

