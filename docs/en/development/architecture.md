# Architecture


This document describes Ekko's workspace layout and core abstractions.

## Workspace layout

```
Ekko/
├── Cargo.toml              # workspace config
├── crates/
│   ├── ekko-cli/           # CLI binary (`ekko`)
│   └── ekko-core/          # core business logic
└── docs/
```

Responsibilities:

- `ekko-cli`: argument parsing, validation, output formatting, orchestrating dry-run vs apply
- `ekko-core`: pure business logic (paths, changesets, templates, config edits, install plans)

## Key abstractions

### HOME sandbox

`--home "<PATH>"` or `EKKO_HOME` redirects all I/O under a sandbox HOME:

- `~/.codex` -> `<sandbox>/.codex`
- `~/.claude` -> `<sandbox>/.claude`
- `~/.gemini` -> `<sandbox>/.gemini`

### ChangeSet

Ekko models side effects as a `ChangeSet`, then applies it depending on mode:

- `DryRun`: print planned changes
- `Apply`: execute filesystem writes / command runs

### Templates

Templates are embedded via `include_str!` from `crates/ekko-core/assets/` and are rendered into a `ChangeSet` for each tool.

### Managed blocks

For shared files (e.g. `GEMINI.md`) Ekko updates only the content inside markers and preserves everything else.

