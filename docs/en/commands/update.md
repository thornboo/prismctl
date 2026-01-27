# `ekko update`


Update built-in templates (overwrite Ekko-owned namespace files, update managed blocks).

Semantically:

- `init`: first-time bootstrap ("put Ekko templates in place")
- `update`: sync to a newer Ekko version ("upgrade Ekko templates")

In the current version, both commands write the same set of Ekko-managed template files, so you can think of `update` as "re-run init, but for upgrades".

```bash
ekko update --tool <codex|claude|gemini|all> [--home <PATH>] [--lang <zh-CN|en>] [--dry-run|--apply]
```

Quick alias:

```bash
ekko u --tool <codex|claude|gemini|all> [--lang <zh-CN|en>] [--home <PATH>] [--dry-run|--apply]
```

Examples:

```bash
# Preview
ekko update --tool all

# Apply
ekko update --tool all --apply
```

## FAQ

### Will `update` overwrite my edits under `~/.codex/prompts/ekko/*`?

Yes. `prompts/ekko/` is an Ekko-owned namespace. If you want to keep local changes:

1. Copy your customized version to a non-`ekko/` location
2. Configure the tool to use your custom version rather than the Ekko namespaced file
