# Ekko

[![CI](https://github.com/thornboo/ekko/actions/workflows/ci.yml/badge.svg)](https://github.com/thornboo/ekko/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/ekko.svg)](https://crates.io/crates/ekko)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Unified CLI for managing AI coding tools: **Codex**, **Claude Code**, and **Gemini CLI**.

[中文文档](README.zh-CN.md)

## Features

- **Template Management** - Output styles, git workflows, agents, and commands
- **Skills Management** - Install, create, and manage Claude Code skills
- **Provider Presets** - Quick setup for OpenRouter, DeepSeek, Ollama, and more
- **Safe by Default** - Dry-run mode, home sandbox, managed blocks
- **Multi-language** - zh-CN and English templates

## Installation

### From crates.io

```bash
cargo install ekko
```

### From source

```bash
git clone https://github.com/thornboo/ekko.git
cd ekko
cargo install --path crates/ekko-cli
```

## Quick Start

```bash
# Show resolved paths (no writes)
ekko doctor

# Initialize all tools (dry-run by default)
ekko init --tool all

# Apply changes to a sandbox
ekko init --tool all --home "/tmp/ekko-home" --apply

# Initialize project-level configuration
ekko project init --path "/path/to/your/project" --apply
```

## Usage

### Template Management

```bash
# Initialize templates for all tools
ekko init --tool all --apply

# Update templates (preserves user content)
ekko update --tool all --apply

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
│   └── ekko-core/      # Core business logic
│       └── assets/     # Built-in templates
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
