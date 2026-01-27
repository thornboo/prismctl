# `ekko project init`


Initialize project-level workflow directories and Gemini project memory file.

```bash
ekko project init [--path <PATH>] [--lang <zh-CN|en>] [--dry-run|--apply]
```

Example:

```bash
ekko project init --path "/path/to/your/project" --apply
```

It creates/updates:

- `<project>/.ekko/plan/`: workflow plan directories (`current/` and `history/`)
- `<project>/.gemini/GEMINI.md`: project memory (managed block; preserves content outside the block)

See: `../projects/project-init.md`.
