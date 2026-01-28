# 更新日志（中文）


本项目的所有重要变更都会记录在此文件中。

格式参考 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.1.0/)，
并遵循 [语义化版本](https://semver.org/lang/zh-CN/spec/v2.0.0.html)。

## [未发布]

### 新增
- **Claude Code**：`prismctl claude mcp ...`（委托 `claude mcp`）管理 MCP servers（内置 catalog）
- **Codex**：`prismctl codex agent use --scope <user|project>`，支持项目级 `AGENTS.md`（并自动备份）
- **Gemini CLI**：`prismctl gemini settings set`，在 `settings.json` 中 upsert `model.name`
- **Gemini CLI**：`prismctl gemini mcp ...`（委托 `gemini mcp`）管理 MCP servers（内置 catalog）

### 变更
- **Gemini CLI**：`prismctl gemini env set` 现在只管理受管块中的 `GEMINI_API_KEY`（支持 user/project scope）
- **doctor**：同时输出用户级与项目级配置存在性（项目级基于当前工作目录）

## [0.1.1] - 2026-01-27

### 新增
- mdBook 文档系统：按语言独立构建（英文为站点根路径，中文位于 `/zh-CN/`）
- 文档右上角语言下拉切换器
- mdBook Catppuccin 主题（默认 Latte/Mocha）
- 文档 GitHub Pages 自动部署工作流

### 修复
- CI 中 mdBook 版本升级至 v0.5.2，修复主题渲染失败
- 修复部分文档代码块导致的 mdBook test 失败

## [0.1.0] - 2026-01-26

### 新增

#### 模板管理
- **Output Styles**（6 种）：prismctl-engineer-professional、prismctl-laowang-engineer、prismctl-leibus-engineer、prismctl-nekomata-engineer、prismctl-ojousama-engineer、prismctl-rem-engineer
- **Git Workflows**（4 个）：git-commit、git-worktree、git-rollback、git-cleanBranches
- **Workflow**：六阶段开发工作流（workflow.md）
- **Agents**（4 个）：planner、ui-ux-designer、init-architect、get-current-datetime
- **Commands**（3 个）：init-project、feat、bmad-init
- 多语言模板：zh-CN / en

#### Skills 管理
- `prismctl skill list`：列出已安装与内置 skills
- `prismctl skill install --name <NAME>`：安装内置 skill
- `prismctl skill create --name <NAME>`：创建 skill 模板
- `prismctl skill remove --name <NAME> --apply --yes`：删除已安装 skill（危险操作）
- 内置 skills：explain-code、codebase-visualizer、pr-summary

#### Provider 预设（Codex）
- OpenRouter（`--provider openrouter`）
- DeepSeek（`--provider deepseek`）
- Ollama（`--provider ollama`）
- Volcengine（`--provider volcengine`）
- SiliconFlow（`--provider siliconflow`）

#### 配置管理
- **Codex**：`prismctl codex provider set`（支持 provider 预设）
- **Codex**：`prismctl codex agent use`（选择 agent 风格）
- **Claude Code**：`prismctl claude env set`（API 配置）
- **Claude Code**：`prismctl claude output-style use`（输出风格选择）
- **Gemini CLI**：`prismctl gemini env set`（API 配置）

#### 安装 / 升级
- `prismctl install --tool <...> --install-method <auto|npm|brew>`：安装受支持工具
- `prismctl upgrade --tool <...> --install-method <auto|npm|brew>`：升级受支持工具

#### 核心能力
- `prismctl init --tool <all|codex|claude|gemini>`：初始化模板
- `prismctl update --tool <all|codex|claude|gemini>`：更新模板
- `prismctl doctor`：显示解析后的路径与配置
- `prismctl project init`：初始化项目级配置
- `--home` 或环境变量 `PRISMCTL_HOME`：将所有读写重定向到沙箱 HOME
- 默认 dry-run，使用 `--apply` 才会执行写入
- 受管块策略：避免侵入式覆盖（保留标记块之外的用户内容）

### 安全
- 默认 dry-run：不写入任何文件
- Home 沙箱：隔离全部文件读写
- 受管块：保留用户自定义内容
- 危险操作需要显式 `--yes`

[未发布]: https://github.com/thornboo/Prismctl/compare/v0.1.1...HEAD
[0.1.1]: https://github.com/thornboo/Prismctl/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/thornboo/Prismctl/releases/tag/v0.1.0
