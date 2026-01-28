# Prismctl

[![CI](https://github.com/thornboo/prismctl/actions/workflows/ci.yml/badge.svg)](https://github.com/thornboo/prismctl/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/prismctl.svg)](https://crates.io/crates/prismctl)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Unified CLI for managing AI coding tools: **Codex**, **Claude Code**, and **Gemini CLI**.

Docs: [English](docs/en/README.md) | [Chinese (Simplified)](docs/zh-CN/README.md)
Documentation site: https://thornboo.github.io/prismctl/

## Features

- **Template Management** - Output styles, git workflows, agents, and commands
- **Skills Management** - Install, create, and manage Claude Code skills
- **Provider Presets** - Quick setup for OpenRouter, DeepSeek, Ollama, and more
- **Safe by Default** - Dry-run mode, home sandbox, managed blocks
- **Multi-language** - zh-CN and English templates

## Language

Prismctl CLI supports zh-CN and English for interactive prompts (work in progress):

```bash
# English
PRISMCTL_LANG=en prismctl

# Chinese
PRISMCTL_LANG=zh-CN prismctl
```

You can also set language via a global flag (must be placed before the command):

```bash
prismctl --lang en --help
prismctl --lang zh-CN init --tool all
```

In the interactive wizard, you can switch language from the menu (persisted to `~/.prismctl/config.toml`).

Config file format:

```toml
[cli]
lang = "en"
```

## Installation

### From crates.io

```bash
cargo install prismctl
```

### Shell install (GitHub Releases)

This installs the latest release binary to `~/.local/bin`:

```bash
curl -fsSL "https://raw.githubusercontent.com/thornboo/prismctl/HEAD/install.sh" | sh
```

Custom install dir:

```bash
PRISMCTL_INSTALL_DIR="$HOME/bin" curl -fsSL "https://raw.githubusercontent.com/thornboo/prismctl/HEAD/install.sh" | sh
```

### From source

```bash
git clone https://github.com/thornboo/prismctl.git
cd prismctl
cargo install --path crates/prismctl-cli
```

### Build (local)

```bash
# Development
cargo run -p prismctl -- --help

# Release binary
cargo build -p prismctl --release
./target/release/prismctl --help
```

## Quick Start

```bash
# Interactive wizard (TTY only)
prismctl

# Show resolved paths (no writes)
prismctl doctor

# Initialize all tools (dry-run by default)
prismctl init --tool all

# Quick init (hybrid, supports flags)
prismctl i --tool all

# Apply changes to a sandbox
prismctl init --tool all --home "/tmp/prismctl-home" --apply

# Mirror prefix: `prismctl config <CMD> ...` ≡ `prismctl <CMD> ...`
prismctl config doctor

# Initialize project-level configuration
prismctl project init --path "/path/to/your/project" --apply
```

## Icons

Prismctl uses Nerd Fonts (via the `devicons` crate) to render file/folder icons in TTY change previews.

Disable icons:

```bash
PRISMCTL_NO_ICONS=1 prismctl init --tool all
```

## Usage

### Interactive Wizard

```bash
# Main entry point (TTY only)
prismctl
prismctl config
```

### Template Management

```bash
# Initialize templates for all tools
prismctl init --tool all --apply

# Update templates (preserves user content)
prismctl update --tool all --apply

# Quick update
prismctl u --tool all --apply

# Initialize specific tool
prismctl init --tool codex --apply
prismctl init --tool claude --apply
prismctl init --tool gemini --apply
```

### Skills Management

```bash
# List available skills
prismctl skill list

# Install a built-in skill
prismctl skill install --name explain-code --apply

# Create a new skill template
prismctl skill create --name my-skill --apply

# Remove a skill
prismctl skill remove --name my-skill --apply --yes
```

### Provider Configuration

#### Codex

```bash
# Use provider preset (OpenRouter, DeepSeek, Ollama, etc.)
prismctl codex provider set --provider openrouter --api-key "sk-xxx" --apply

# Custom provider
prismctl codex provider set --base-url "https://api.example.com/v1" --model "gpt-4" --api-key "sk-xxx" --apply

# Select agent style
prismctl codex agent use --name prismctl-engineer-professional --apply --yes
```

#### Claude Code

```bash
# Configure API
prismctl claude env set --auth-token "sk-xxx" --base-url "https://api.example.com" --apply

# Set output style
prismctl claude output-style use --name prismctl-engineer-professional --apply

# MCP (delegates to claude CLI)
prismctl claude mcp add --name context7 --scope user --apply --yes
```

#### Gemini CLI

```bash
# Configure API
prismctl gemini env set --api-key "xxx" --apply

# Set model.name (writes settings.json)
prismctl gemini settings set --model "gemini-2.5-pro" --apply

# MCP (delegates to gemini CLI)
prismctl gemini mcp add --name context7 --scope user --apply --yes
```

## Safety Model

Prismctl is designed with safety in mind:

| Feature | Description |
|---------|-------------|
| **Dry-run Default** | No files written unless `--apply` is passed |
| **Home Sandbox** | `--home` or `PRISMCTL_HOME` redirects all I/O |
| **Managed Blocks** | Only updates content between markers, preserves user content |
| **Namespaced** | Templates written to `prismctl/` directories |

## Built-in Templates

### Output Styles (6)
- `prismctl-engineer-professional` - Professional engineering style
- `prismctl-laowang-engineer` - Laowang style
- `prismctl-leibus-engineer` - Leibus style
- `prismctl-nekomata-engineer` - Nekomata style
- `prismctl-ojousama-engineer` - Ojousama style
- `prismctl-rem-engineer` - Rem style

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
Prismctl/
├── crates/
│   ├── prismctl-cli/       # CLI entry point
│   ├── prismctl-core/      # Core business logic
│   │   └── assets/     # Built-in templates
│   └── prismctl-i18n/      # i18n support (ftl + keys)
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
