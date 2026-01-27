# Gemini CLI Templates


## Paths

```text
~/.gemini/
├── .env                # Gemini CLI env file
├── GEMINI.md           # global memory (managed block)
└── ekko/
    └── WORKFLOWS.md    # documentation (namespaced file)
```

> Tip: all paths can be redirected via `--home "<PATH>"` or `EKKO_HOME`.

## Managed block (GEMINI.md)

Ekko only updates the content inside markers and preserves everything else:

```markdown
<!-- ekko:start -->
Ekko-managed content
<!-- ekko:end -->
```

Related commands:

- `ekko init --tool gemini ...` / `ekko update --tool gemini ...`: manage a block inside `~/.gemini/GEMINI.md`
- `ekko project init ...`: manage a block inside `<project>/.gemini/GEMINI.md`

## Managed block (.env)

`ekko gemini env set` maintains an Ekko-managed block in `~/.gemini/.env` (`# ekko:start` / `# ekko:end`) for:

- `GEMINI_API_KEY`
- `GOOGLE_GEMINI_BASE_URL` (optional)
- `GEMINI_MODEL` (optional)

## Memory precedence (Gemini CLI)

Gemini CLI loads `GEMINI.md` by directory hierarchy:

1. Global: `~/.gemini/GEMINI.md`
2. Project: `<project>/.gemini/GEMINI.md`
3. Subdir: `<subdir>/.gemini/GEMINI.md`
