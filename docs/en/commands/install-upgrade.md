# `prismctl install` / `prismctl upgrade`


Install/upgrade AI tools globally (Codex / Claude Code / Gemini CLI).

⚠️ Dangerous operation: requires `--apply --yes`.

## Syntax

```bash
prismctl install --tool <codex|claude|gemini|all> [--install-method <auto|npm|brew>] [--dry-run|--apply] [--yes]
prismctl upgrade --tool <codex|claude|gemini|all> [--install-method <auto|npm|brew>] [--dry-run|--apply] [--yes]
```

## Install methods

- `npm`: install via npm (global)
- `brew`: install via Homebrew (macOS)
- `auto`: prefer brew on macOS, otherwise npm

## What Prismctl actually runs

Prismctl does not download binaries itself; it calls your local package manager:

- npm: `npm install -g <package>@latest`
- brew: `brew install <name>` / `brew upgrade <name>` (cask/formula depends on the tool)

Make sure `npm` or `brew` is installed and available in your `PATH`.

## Package mapping

| Method | Codex | Claude Code | Gemini CLI |
|------|-------|-------------|------------|
| npm | `@openai/codex` | `@anthropic-ai/claude-code` | `@google/gemini-cli` |
| brew | `codex` (cask) | `claude-code` (cask) | `gemini-cli` (formula) |

## Examples

```bash
# Preview
prismctl install --tool all --install-method auto

# Apply
prismctl install --tool all --install-method auto --apply --yes
```

Recommended workflow:

1. Run dry-run to review the planned commands
2. Rerun with `--apply --yes` to execute
