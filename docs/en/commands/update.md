# `prismctl update`


Update built-in templates (overwrite Prismctl-owned namespace files, update managed blocks).

Semantically:

- `init`: first-time bootstrap ("put Prismctl templates in place")
- `update`: sync to a newer Prismctl version ("upgrade Prismctl templates")

In the current version, both commands write the same set of Prismctl-managed template files, so you can think of `update` as "re-run init, but for upgrades".

```bash
prismctl update --tool <codex|claude|gemini|all> [--home <PATH>] [--lang <zh-CN|en>] [--dry-run|--apply]
```

Quick alias:

```bash
prismctl u --tool <codex|claude|gemini|all> [--lang <zh-CN|en>] [--home <PATH>] [--dry-run|--apply]
```

Examples:

```bash
# Preview
prismctl update --tool all

# Apply
prismctl update --tool all --apply
```

## FAQ

### Will `update` overwrite my edits under `~/.codex/prompts/prismctl/*`?

Yes. `prompts/prismctl/` is an Prismctl-owned namespace. If you want to keep local changes:

1. Copy your customized version to a non-`prismctl/` location
2. Configure the tool to use your custom version rather than the Prismctl namespaced file
