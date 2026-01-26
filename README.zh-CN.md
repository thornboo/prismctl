# Ekko

Ekko 是一个 Rust CLI，用于统一管理多个 AI 编程工具（Codex、Claude Code、Gemini CLI）的配置、工作流模板与风格。

## 核心特性

- **安全默认**：所有写入类命令默认 `--dry-run`，只有显式 `--apply` 才会落盘
- **沙箱隔离**：`--home "<PATH>"` 或 `EKKO_HOME` 可把所有读写重定向到沙箱目录
- **受管策略**：区分命名空间文件/受管块/显式覆盖，默认不破坏用户已有配置
- **内置模板**：工作流、Git 常用操作、Agents、Output Styles（支持 `zh-CN`/`en`）
- **Skills 管理**：安装/创建/删除 Claude skills（删除需要显式确认）

## 安装

### 通过 crates.io

```bash
cargo install ekko
```

### 从源码安装

```bash
git clone "https://github.com/thornboo/ekko.git"
cd "ekko"
cargo install --path "crates/ekko-cli"
```

## 快速开始

```bash
# 查看解析后的路径（不会写入）
ekko doctor

# 初始化全部工具模板（默认 dry-run）
ekko init --tool all

# 在沙箱中应用更改
ekko init --tool all --home "/tmp/ekko-home" --apply

# 初始化项目级配置
ekko project init --path "/path/to/your/project" --apply
```

## 受管写入策略（防止覆盖用户配置）

为避免 CLI bug 造成“覆盖用户配置不可恢复”，Ekko 将写入分为三类：

1. **命名空间文件写入（默认）**
   - 仅在工具配置目录下写入 `ekko/` 命名空间文件（例如 `~/.codex/prompts/ekko/*`、`~/.claude/commands/ekko/*`）
   - 这类文件由 Ekko 完全管理，可安全覆盖更新

2. **受管块写入（保留用户内容）**
   - 对“官方固定文件且用户可能自定义”的场景（例如 `~/.gemini/GEMINI.md`），只更新标记块，块外内容完全保留

   ```markdown
   <!-- ekko:start -->
   Ekko 管理的内容
   <!-- ekko:end -->
   ```

3. **显式覆盖（危险操作）**
   - 对“没有可靠合并语义”的文件（例如 `~/.codex/AGENTS.md`），需要 `--apply --yes`
   - 写入前会自动备份旧文件

## 常用命令

```bash
# 初始化/更新模板
ekko init --tool all --apply
ekko update --tool all --apply

# Skills
ekko skill list
ekko skill install --name "explain-code" --apply
ekko skill create --name "my-skill" --apply
ekko skill remove --name "my-skill" --apply --yes

# Codex Provider 预设
ekko codex provider set --provider "openrouter" --api-key "sk-xxx" --apply

# Claude 配置
ekko claude env set --auth-token "sk-xxx" --apply
ekko claude output-style use --name "ekko-engineer-professional" --apply

# Gemini 配置
ekko gemini env set --api-key "xxx" --apply
```

