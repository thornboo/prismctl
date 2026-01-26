# 参考链接


## 官方文档

### Codex CLI

- **npm 包**：https://www.npmjs.com/package/@openai/codex
- **GitHub**：https://github.com/openai/codex

### Claude Code CLI

- **npm 包**：https://www.npmjs.com/package/@anthropic-ai/claude-code
- **官方文档**：https://docs.anthropic.com/claude-code

### Gemini CLI

- **GitHub**：https://github.com/google-gemini/gemini-cli
- **npm 包**：https://www.npmjs.com/package/@google/gemini-cli
- **配置指南**：https://github.com/google-gemini/gemini-cli/blob/main/docs/get-started/configuration.md

---

## 设计参考

### zcf

Ekko 的设计思路参考了 zcf 项目：

- **GitHub**：https://github.com/UfoMiao/zcf

zcf 是一个 Node.js 实现的类似工具，Ekko 使用 Rust 重写以获得更好的性能和安全性。

---

## Rust 生态

### CLI 开发

- **clap**：https://docs.rs/clap - 命令行参数解析
- **thiserror**：https://docs.rs/thiserror - 错误处理

### 测试

- **tempfile**：https://docs.rs/tempfile - 临时文件/目录
- **assert_cmd**：https://docs.rs/assert_cmd - 命令行测试

### 序列化

- **serde**：https://serde.rs - 序列化框架
- **serde_json**：https://docs.rs/serde_json - JSON 支持
- **toml**：https://docs.rs/toml - TOML 支持

---

## 工具配置位置

| 工具 | 全局配置目录 | 主要配置文件 |
|------|-------------|-------------|
| Codex | `~/.codex/` | `config.toml`, `auth.json`, `AGENTS.md` |
| Claude Code | `~/.claude/` | `settings.json` |
| Gemini CLI | `~/.gemini/` | `.env`, `GEMINI.md` |

---

## 相关规范

- **Conventional Commits**：https://www.conventionalcommits.org
- **Semantic Versioning**：https://semver.org
- **Keep a Changelog**：https://keepachangelog.com
