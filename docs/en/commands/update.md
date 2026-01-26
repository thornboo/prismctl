# `ekko update`


Update built-in templates (overwrite Ekko-owned namespace files, update managed blocks).

```bash
ekko update --tool <codex|claude|gemini|all> [--home <PATH>] [--lang <zh-CN|en>] [--dry-run|--apply]
```

Examples:

```bash
# Preview
ekko update --tool all

# Apply
ekko update --tool all --apply
```

