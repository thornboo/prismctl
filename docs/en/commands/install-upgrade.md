# `ekko install` / `ekko upgrade`


Install/upgrade AI tools globally (Codex / Claude Code / Gemini CLI).

⚠️ Dangerous operation: requires `--apply --yes`.

## Syntax

```bash
ekko install --tool <codex|claude|gemini|all> [--install-method <auto|npm|brew>] [--dry-run|--apply] [--yes]
ekko upgrade --tool <codex|claude|gemini|all> [--install-method <auto|npm|brew>] [--dry-run|--apply] [--yes]
```

## Install methods

- `npm`: install via npm (global)
- `brew`: install via Homebrew (macOS)
- `auto`: prefer brew on macOS, otherwise npm

## Package mapping

| Method | Codex | Claude Code | Gemini CLI |
|------|-------|-------------|------------|
| npm | `@openai/codex` | `@anthropic-ai/claude-code` | `@google/gemini-cli` |
| brew | `codex` (cask) | `claude-code` (cask) | `gemini-cli` (formula) |

## Examples

```bash
# Preview
ekko install --tool all --install-method auto

# Apply
ekko install --tool all --install-method auto --apply --yes
```
