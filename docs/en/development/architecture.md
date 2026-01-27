# Architecture


This document describes Ekko's workspace layout and core abstractions.

## Workspace layout

```text
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

Design intent: `ekko-core` does not depend on a CLI framework and can be reused by other frontends.

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

Templates are embedded into the binary via `include_str!` and written out by `init`/`update`.

Key paths:

- Codex templates: `~/.codex/prompts/ekko/*`
- Claude templates: `~/.claude/commands/ekko/*`, `~/.claude/agents/ekko/*`, `~/.claude/output-styles/*`
- Gemini docs: `~/.gemini/ekko/WORKFLOWS.md`

### Managed blocks

For shared files that users may extend, Ekko only updates content inside markers and preserves everything else.

Examples:

- `~/.gemini/GEMINI.md` (`<!-- ekko:start -->` / `<!-- ekko:end -->`)
- `~/.gemini/.env` (`# ekko:start` / `# ekko:end`)

Implementation: `crates/ekko-core/src/managed_block.rs`

### Config edits (JSON/TOML)

Ekko edits JSON and TOML as text with minimal, targeted upserts:

- JSON: `crates/ekko-core/src/json_text.rs` (used for `settings.json`, `auth.json`)
- TOML: `crates/ekko-core/src/toml_text.rs` (used for `config.toml`)

This keeps dependencies small and avoids large, unintended formatting churn.

## Testing strategy

- Unit tests validate text upserts and changeset planning without running real commands.
- End-to-end style tests use a sandbox home (`--home`) to apply changes into a temp directory.
Templates are embedded via `include_str!` from `crates/ekko-core/assets/` and are rendered into a `ChangeSet` for each tool.

### Managed blocks

For shared files (e.g. `GEMINI.md`) Ekko updates only the content inside markers and preserves everything else.
