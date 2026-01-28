# Architecture


This document describes Prismctl's workspace layout and core abstractions.

## Workspace layout

```text
Prismctl/
├── Cargo.toml              # workspace config
├── crates/
│   ├── prismctl-cli/           # CLI binary (`prismctl`)
│   └── prismctl-core/          # core business logic
└── docs/
```

Responsibilities:

- `prismctl-cli`: argument parsing, validation, output formatting, orchestrating dry-run vs apply
- `prismctl-core`: pure business logic (paths, changesets, templates, config edits, install plans)

Design intent: `prismctl-core` does not depend on a CLI framework and can be reused by other frontends.

## Key abstractions

### HOME sandbox

`--home "<PATH>"` or `PRISMCTL_HOME` redirects all I/O under a sandbox HOME:

- `~/.codex` -> `<sandbox>/.codex`
- `~/.claude` -> `<sandbox>/.claude`
- `~/.gemini` -> `<sandbox>/.gemini`

### ChangeSet

Prismctl models side effects as a `ChangeSet`, then applies it depending on mode:

- `DryRun`: print planned changes
- `Apply`: execute filesystem writes / command runs

### Templates

Templates are embedded into the binary via `include_str!` and written out by `init`/`update`.

Key paths:

- Codex templates: `~/.codex/prompts/prismctl/*`
- Claude templates: `~/.claude/commands/prismctl/*`, `~/.claude/agents/prismctl/*`, `~/.claude/output-styles/*`
- Gemini docs: `~/.gemini/prismctl/WORKFLOWS.md`

### Managed blocks

For shared files that users may extend, Prismctl only updates content inside markers and preserves everything else.

Examples:

- `~/.gemini/GEMINI.md` (`<!-- prismctl:start -->` / `<!-- prismctl:end -->`)
- `~/.gemini/.env` (`# prismctl:start` / `# prismctl:end`)

Implementation: `crates/prismctl-core/src/managed_block.rs`

### Config edits (JSON/TOML)

Prismctl edits JSON and TOML as text with minimal, targeted upserts:

- JSON: `crates/prismctl-core/src/json_text.rs` (used for `settings.json`, `auth.json`)
- TOML: `crates/prismctl-core/src/toml_text.rs` (used for `config.toml`)

This keeps dependencies small and avoids large, unintended formatting churn.

## Testing strategy

- Unit tests validate text upserts and changeset planning without running real commands.
- End-to-end style tests use a sandbox home (`--home`) to apply changes into a temp directory.
Templates are embedded via `include_str!` from `crates/prismctl-core/assets/` and are rendered into a `ChangeSet` for each tool.

### Managed blocks

For shared files (e.g. `GEMINI.md`) Prismctl updates only the content inside markers and preserves everything else.
