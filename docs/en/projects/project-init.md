# Project Init


`prismctl project init` creates Prismctl's project workflow directories and initializes Gemini CLI project memory.

## Quick usage

```bash
# Run in project root (dry-run)
cd "/path/to/your/project"
prismctl project init

# Apply
prismctl project init --apply

# Target another directory
prismctl project init --path "/path/to/other/project" --apply

# Use English templates
prismctl project init --lang en --apply
```

## What it creates

```text
<project>/
├── .prismctl/
│   └── plan/
│       ├── current/        # active plans
│       ├── history/        # archived plans
│       └── README.md       # conventions
└── .gemini/
    └── GEMINI.md           # project memory (managed block)
```

### `.gemini/GEMINI.md` (managed block)

If the file already exists, Prismctl only updates the content between markers and preserves everything else:

```markdown
<!-- prismctl:start -->
Prismctl-managed project context...
<!-- prismctl:end -->
```

## Why this exists

- **Consistent plan locations**: built-in workflows reference `.prismctl/plan/*`.
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

