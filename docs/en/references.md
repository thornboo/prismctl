# References


## Official docs

### Codex CLI

- npm: https://www.npmjs.com/package/@openai/codex
- GitHub: https://github.com/openai/codex

### Claude Code CLI

- npm: https://www.npmjs.com/package/@anthropic-ai/claude-code
- Docs: https://docs.anthropic.com/claude-code

### Gemini CLI

- GitHub: https://github.com/google-gemini/gemini-cli
- npm: https://www.npmjs.com/package/@google/gemini-cli
- Configuration: https://github.com/google-gemini/gemini-cli/blob/main/docs/get-started/configuration.md

## Design references

### zcf

- GitHub: https://github.com/UfoMiao/zcf

## Rust ecosystem

### Serialization

- serde: https://serde.rs
- serde_json: https://docs.rs/serde_json

> Note: Prismctl currently only depends on `serde` / `serde_json`. TOML updates are implemented as lightweight text upserts (see `crates/prismctl-core/src/toml_text.rs`) to keep dependencies minimal and avoid unintended formatting changes.

## Config locations

| Tool | Global dir | Key files |
|------|------------|-----------|
| Codex | `~/.codex/` | `config.toml`, `auth.json`, `AGENTS.md` |
| Claude Code | `~/.claude/` | `settings.json` |
| Gemini CLI | `~/.gemini/` | `.env`, `GEMINI.md` |

## Standards

- Conventional Commits: https://www.conventionalcommits.org
- Semantic Versioning: https://semver.org
- Keep a Changelog: https://keepachangelog.com
