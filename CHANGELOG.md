# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- **Claude Code**: `prismctl claude mcp ...` (delegates to `claude mcp`) to manage MCP servers with a built-in catalog
- **Codex**: `prismctl codex agent use --scope <user|project>` to support project-scoped `AGENTS.md` (with backups)
- **Gemini CLI**: `prismctl gemini settings set` to upsert `model.name` in `settings.json`
- **Gemini CLI**: `prismctl gemini mcp ...` (delegates to `gemini mcp`) to manage MCP servers with a built-in catalog

### Changed
- **Gemini CLI**: `prismctl gemini env set` now only manages `GEMINI_API_KEY` inside a Prismctl-managed block (supports user/project scope)
- **doctor**: now reports both user-level and project-level configuration presence (based on current working directory)

## [0.1.1] - 2026-01-27

### Added
- mdBook documentation with per-locale builds (`en` at site root, `zh-CN` under `/zh-CN/`)
- Language dropdown in the docs top bar
- Catppuccin theme for mdBook (Latte/Mocha defaults)
- GitHub Pages deployment workflow for docs

### Fixed
- GitHub Actions docs build by upgrading mdBook in CI to v0.5.2
- mdBook test failures caused by non-Rust code fences in docs

## [0.1.0] - 2026-01-26

### Added

#### Template Management
- **Output Styles** (6 styles): prismctl-engineer-professional, prismctl-laowang-engineer, prismctl-leibus-engineer, prismctl-nekomata-engineer, prismctl-ojousama-engineer, prismctl-rem-engineer
- **Git Workflows** (4 commands): git-commit, git-worktree, git-rollback, git-cleanBranches
- **Workflow**: Six-phase development workflow (workflow.md)
- **Agents** (4 agents): planner, ui-ux-designer, init-architect, get-current-datetime
- **Commands** (3 commands): init-project, feat, bmad-init
- Multi-language support: zh-CN and en

#### Skills Management
- `prismctl skill list` - List installed and built-in skills
- `prismctl skill install --name <NAME>` - Install built-in skill
- `prismctl skill create --name <NAME>` - Create skill template
- `prismctl skill remove --name <NAME> --apply --yes` - Remove installed skill
- Built-in skills: explain-code, codebase-visualizer, pr-summary

#### Provider Presets (Codex)
- OpenRouter preset (`--provider openrouter`)
- DeepSeek preset (`--provider deepseek`)
- Ollama preset (`--provider ollama`)
- Volcengine preset (`--provider volcengine`)
- SiliconFlow preset (`--provider siliconflow`)

#### Configuration Management
- **Codex**: `prismctl codex provider set` with provider presets
- **Codex**: `prismctl codex agent use` for agent selection
- **Claude Code**: `prismctl claude env set` for API configuration
- **Claude Code**: `prismctl claude output-style use` for style selection
- **Gemini CLI**: `prismctl gemini env set` for API configuration

#### Installation / Upgrade
- `prismctl install --tool <...> --install-method <auto|npm|brew>` - Install supported tools
- `prismctl upgrade --tool <...> --install-method <auto|npm|brew>` - Upgrade supported tools

#### Core Features
- `prismctl init --tool <all|codex|claude|gemini>` - Initialize templates
- `prismctl update --tool <all|codex|claude|gemini>` - Update templates
- `prismctl doctor` - Show resolved paths and configuration
- `prismctl project init` - Initialize project-level configuration
- Home sandbox via `--home` or `PRISMCTL_HOME` environment variable
- Dry-run by default, `--apply` to execute changes
- Managed block strategy for non-invasive updates

### Security
- All operations are dry-run by default
- Home sandbox isolates all file operations
- Managed blocks preserve user content outside markers
- Dangerous operations require explicit `--yes`

[Unreleased]: https://github.com/thornboo/Prismctl/compare/v0.1.1...HEAD
[0.1.1]: https://github.com/thornboo/Prismctl/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/thornboo/Prismctl/releases/tag/v0.1.0
