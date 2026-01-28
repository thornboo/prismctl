# Ekko

[![CI](https://github.com/thornboo/ekko/actions/workflows/ci.yml/badge.svg)](https://github.com/thornboo/ekko/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/ekko.svg)](https://crates.io/crates/ekko)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Unified CLI for managing AI coding tools: **Codex**, **Claude Code**, and **Gemini CLI**.

Docs: [English](docs/en/README.md) | [Chinese (Simplified)](docs/zh-CN/README.md)
Documentation site: https://thornboo.github.io/Ekko/

## Features

- **Template Management** - Output styles, git workflows, agents, and commands
- **Skills Management** - Install, create, and manage Claude Code skills
- **Provider Presets** - Quick setup for OpenRouter, DeepSeek, Ollama, and more
- **Safe by Default** - Dry-run mode, home sandbox, managed blocks
- **Multi-language** - zh-CN and English templates

## Language

Ekko CLI supports zh-CN and English for interactive prompts (work in progress):

```bash
# English
EKKO_LANG=en ekko

# Chinese
EKKO_LANG=zh-CN ekko
```

You can also set language via a global flag (must be placed before the command):

```bash
ekko --lang en --help
ekko --lang zh-CN init --tool all
```

In the interactive wizard, you can switch language from the menu (persisted to `~/.ekko/config.toml`).

Config file format:

```toml
[cli]
lang = "en"
```

## Installation

### From crates.io

```bash
cargo install ekko
```

### Shell install (GitHub Releases)

This installs the latest release binary to `~/.local/bin`:

```bash
curl -fsSL "https://raw.githubusercontent.com/thornboo/ekko/HEAD/install.sh" | sh
```

Custom install dir:

```bash
EKKO_INSTALL_DIR="$HOME/bin" curl -fsSL "https://raw.githubusercontent.com/thornboo/ekko/HEAD/install.sh" | sh
```

### From source

```bash
git clone https://github.com/thornboo/ekko.git
cd ekko
cargo install --path crates/ekko-cli
```

### Build (local)

```bash
# Development
cargo run -p ekko -- --help

# Release binary
cargo build -p ekko --release
./target/release/ekko --help
```

## Quick Start

```bash
# Interactive wizard (TTY only)
ekko

# Show resolved paths (no writes)
ekko doctor

# Initialize all tools (dry-run by default)
ekko init --tool all

# Quick init (hybrid, supports flags)
ekko i --tool all

# Apply changes to a sandbox
ekko init --tool all --home "/tmp/ekko-home" --apply

# Mirror prefix: `ekko config <CMD> ...` ≡ `ekko <CMD> ...`
ekko config doctor

# Initialize project-level configuration
ekko project init --path "/path/to/your/project" --apply
```

## Icons

Ekko uses Nerd Fonts (via the `devicons` crate) to render file/folder icons in TTY change previews.

Disable icons:

```bash
EKKO_NO_ICONS=1 ekko init --tool all
```

## Usage

### Interactive Wizard

```bash
# Main entry point (TTY only)
ekko
ekko config
```

### Template Management

```bash
# Initialize templates for all tools
ekko init --tool all --apply

# Update templates (preserves user content)
ekko update --tool all --apply

# Quick update
ekko u --tool all --apply

# Initialize specific tool
ekko init --tool codex --apply
ekko init --tool claude --apply
ekko init --tool gemini --apply
```

### Skills Management

```bash
# List available skills
ekko skill list

# Install a built-in skill
ekko skill install --name explain-code --apply

# Create a new skill template
ekko skill create --name my-skill --apply

# Remove a skill
ekko skill remove --name my-skill --apply --yes
```

### Provider Configuration

#### Codex

```bash
# Use provider preset (OpenRouter, DeepSeek, Ollama, etc.)
ekko codex provider set --provider openrouter --api-key "sk-xxx" --apply

# Custom provider
ekko codex provider set --base-url "https://api.example.com/v1" --model "gpt-4" --api-key "sk-xxx" --apply

# Select agent style
ekko codex agent use --name ekko-engineer-professional --apply --yes
```

#### Claude Code

```bash
# Configure API
ekko claude env set --auth-token "sk-xxx" --base-url "https://api.example.com" --apply

# Set output style
ekko claude output-style use --name ekko-engineer-professional --apply
```

#### Gemini CLI

```bash
# Configure API
ekko gemini env set --api-key "xxx" --model "gemini-pro" --apply
```

## Safety Model

Ekko is designed with safety in mind:

| Feature | Description |
|---------|-------------|
| **Dry-run Default** | No files written unless `--apply` is passed |
| **Home Sandbox** | `--home` or `EKKO_HOME` redirects all I/O |
| **Managed Blocks** | Only updates content between markers, preserves user content |
| **Namespaced** | Templates written to `ekko/` directories |

## Built-in Templates

### Output Styles (6)
- `ekko-engineer-professional` - Professional engineering style
- `ekko-laowang-engineer` - Laowang style
- `ekko-leibus-engineer` - Leibus style
- `ekko-nekomata-engineer` - Nekomata style
- `ekko-ojousama-engineer` - Ojousama style
- `ekko-rem-engineer` - Rem style

### Git Workflows (4)
- `git-commit` - Conventional commit helper
- `git-worktree` - Git worktree management
- `git-rollback` - Safe rollback operations
- `git-cleanBranches` - Clean merged branches

### Skills (3)
- `explain-code` - Code explanation with diagrams
- `codebase-visualizer` - Interactive codebase visualization
- `pr-summary` - Pull request summary generator

## Provider Presets

| Provider | Base URL | Default Model |
|----------|----------|---------------|
| OpenRouter | `https://openrouter.ai/api/v1` | `google/gemini-2.5-pro-preview` |
| DeepSeek | `https://api.deepseek.com/v1` | `deepseek-chat` |
| Ollama | `http://localhost:11434/v1` | `qwen2.5-coder:latest` |
| Volcengine | `https://ark.cn-beijing.volces.com/api/v3` | `deepseek-v3-250324` |
| SiliconFlow | `https://api.siliconflow.cn/v1` | `moonshotai/Kimi-K2-Instruct` |

## Project Structure

```
Ekko/
├── crates/
│   ├── ekko-cli/       # CLI entry point
│   ├── ekko-core/      # Core business logic
│   │   └── assets/     # Built-in templates
│   └── ekko-i18n/      # i18n support (ftl + keys)
```

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for development guidelines.

```bash
# Run tests
cargo test --all

# Run clippy
cargo clippy -- -D warnings

# Format code
cargo fmt --all
```

## License

MIT License - see [LICENSE](LICENSE) for details.

## Acknowledgments

Template sources traced from:
- [BMAD-METHOD](https://github.com/bmadcode/BMAD-METHOD)
- [claude-code-router](https://github.com/musistudio/claude-code-router)
- [zcf](https://github.com/UfoMiao/zcf)
