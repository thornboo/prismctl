# Gemini CLI Templates


## Paths

```
~/.gemini/
├── .env                # Gemini CLI env file
├── GEMINI.md           # global memory (managed block)
└── ekko/
    └── WORKFLOWS.md    # documentation (namespaced file)
```

## Managed block (GEMINI.md)

Ekko only updates the content inside markers and preserves everything else:

```markdown
<!-- ekko:start -->
Ekko-managed content
<!-- ekko:end -->
```

## Memory precedence (Gemini CLI)

Gemini CLI loads `GEMINI.md` by directory hierarchy:

1. Global: `~/.gemini/GEMINI.md`
2. Project: `<project>/.gemini/GEMINI.md`
3. Subdir: `<subdir>/.gemini/GEMINI.md`
