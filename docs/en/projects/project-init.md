# Project Init


`ekko project init` creates Ekko's project workflow directories and initializes Gemini CLI project memory.

## Quick usage

```bash
# Run in project root (dry-run)
cd "/path/to/your/project"
ekko project init

# Apply
ekko project init --apply

# Target another directory
ekko project init --path "/path/to/other/project" --apply

# Use English templates
ekko project init --lang en --apply
```

## What it creates

```text
<project>/
├── .ekko/
│   └── plan/
│       ├── current/        # active plans
│       ├── history/        # archived plans
│       └── README.md       # conventions
└── .gemini/
    └── GEMINI.md           # project memory (managed block)
```

### `.gemini/GEMINI.md` (managed block)

If the file already exists, Ekko only updates the content between markers and preserves everything else:

```markdown
<!-- ekko:start -->
Ekko-managed project context...
<!-- ekko:end -->
```

## Why this exists

- **Consistent plan locations**: built-in workflows reference `.ekko/plan/*`.
- **Team-shared context**: `.gemini/GEMINI.md` can be committed to share project memory.
- **Traceability**: archived plans under `history/` help future contributors understand decisions.

## Gemini memory precedence

Gemini CLI loads memory files by directory hierarchy (lower to higher priority):

1. `~/.gemini/GEMINI.md` (global)
2. `<project>/.gemini/GEMINI.md` (project)
3. `<subdir>/.gemini/GEMINI.md` (subdir)

## Next

- Templates: `../templates/index.md`
- Command reference: `../commands/index.md`

