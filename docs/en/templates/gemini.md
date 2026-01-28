# Gemini CLI Templates


## Paths

```text
~/.gemini/
├── .env                # Gemini CLI env file
├── GEMINI.md           # global memory (managed block)
├── settings.json       # user settings (structured upsert)
└── prismctl/
    └── WORKFLOWS.md    # documentation (namespaced file)
```

> Tip: all paths can be redirected via `--home "<PATH>"` or `PRISMCTL_HOME`.

## Managed block (GEMINI.md)

Prismctl only updates the content inside markers and preserves everything else:

```markdown
<!-- prismctl:start -->
Prismctl-managed content
<!-- prismctl:end -->
```

Related commands:

- `prismctl init --tool gemini ...` / `prismctl update --tool gemini ...`: manage a block inside `~/.gemini/GEMINI.md`
- `prismctl project init ...`: manage a block inside `<project>/.gemini/GEMINI.md`

## Managed block (.env)

`prismctl gemini env set` maintains an Prismctl-managed block in `~/.gemini/.env` (`# prismctl:start` / `# prismctl:end`) for:

- `GEMINI_API_KEY`

Project scope is also supported via `<project>/.gemini/.env` (use `--scope project`).

## settings.json (structured upsert)

`prismctl gemini settings set` upserts `model.name` in:

- user scope: `~/.gemini/settings.json`
- project scope: `<project>/.gemini/settings.json`

## MCP (delegates to gemini CLI)

`prismctl gemini mcp ...` delegates to `gemini mcp ...` and writes MCP server configs into `mcpServers` inside the appropriate `settings.json` (user/project scope).

## Memory precedence (Gemini CLI)

Gemini CLI loads `GEMINI.md` by directory hierarchy:

1. Global: `~/.gemini/GEMINI.md`
2. Project: `<project>/.gemini/GEMINI.md`
3. Subdir: `<subdir>/.gemini/GEMINI.md`
