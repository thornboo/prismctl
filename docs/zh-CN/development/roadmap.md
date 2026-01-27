# 发展路线图


本文档包含 Ekko 项目的设计原则、功能规划和行动计划。

---

## 当前状态

- v0.1.0：已完成（本地 tag：`v0.1.0`）

## 核心设计原则

Ekko 必须遵循以下原则：

| 原则 | 说明 | 实践 |
|------|------|------|
| **干净** | 不留垃圾文件，卸载后无残留 | 所有文件写入明确的命名空间 |
| **一键配置** | 单命令完成所有设置 | `ekko init --tool all --apply` |
| **轻量级** | 最小依赖，快速启动 | 仅 serde/serde_json，无运行时 |
| **非侵入式** | 不修改用户现有配置 | 仅管理 `ekko/` 命名空间，受管块保留用户内容 |
| **配置文件清晰** | 人类可读，易于调试 | JSON/TOML 格式化输出，有注释 |

---

## 模板来源

### 溯源链

```text
BMAD-METHOD (31.8k ⭐) ─────┐
claude-code-router ────────┼──→ zcf (5.3k ⭐) ──→ Ekko
CCometixLine ──────────────┘
```

### 移植策略：精简核心

| 类别 | 移植 | 文件 |
|------|------|------|
| **Output Styles** | ✓ 全部 | 6 个风格 |
| **Git Workflows** | ✓ 全部 | git-commit, git-worktree, git-rollback, git-cleanBranches |
| **sixStep Workflow** | ✓ | workflow.md |
| **Agents** | ✓ 精选 | planner, ui-ux-designer, init-architect |
| **Commands** | ✓ 精选 | init-project, feat |
| **BMAD** | ✗ 暂不 | 复杂度高，后续按需 |

---

## v0.1.0 功能规划 (MVP)

### 核心功能

```text
Ekko v0.1.0
├── 模板管理
│   ├── Output Styles (6个)
│   │   ├── ekko-engineer-professional
│   │   ├── ekko-laowang-engineer
│   │   ├── ekko-leibus-engineer
│   │   ├── ekko-nekomata-engineer
│   │   ├── ekko-ojousama-engineer
│   │   └── ekko-rem-engineer
│   ├── Git Workflows (4个)
│   │   ├── git-commit
│   │   ├── git-worktree
│   │   ├── git-rollback
│   │   └── git-cleanBranches
│   ├── Workflow
│   │   └── workflow (六阶段)
│   ├── Agents (3个)
│   │   ├── planner
│   │   ├── ui-ux-designer
│   │   └── init-architect
│   └── Commands (2个)
│       ├── init-project
│       └── feat
│
├── Skills 管理 ⭐ 核心功能
│   ├── ekko skill list          # 列出已安装 skills
│   ├── ekko skill install       # 安装 skill
│   ├── ekko skill create        # 创建 skill 模板
│   └── ekko skill remove        # 删除 skill（危险操作）
│
├── 配置管理
│   ├── Claude Code
│   │   ├── env set (API 配置)
│   │   └── output-style use
│   ├── Codex
│   │   ├── provider set
│   │   └── agent use
│   └── Gemini CLI
│       └── env set
│
├── 服务商支持（对齐 zcf）
│   ├── OpenRouter
│   ├── DeepSeek
│   ├── Ollama
│   ├── Gemini
│   ├── Volcengine (火山引擎)
│   └── SiliconFlow
│
└── 工具安装
    └── install/upgrade (npm/brew)
```

### Skills 功能设计

Skills 是 Claude Code 的轻量级扩展机制，Ekko 需要支持管理它们。

**存储位置**：
```text
~/.claude/skills/
├── <skill-name>/
│   ├── SKILL.md      # 必需：指令和配置
│   ├── scripts/      # 可选：脚本
│   └── examples/     # 可选：示例
```

**命令设计**：

```bash
# 列出已安装的 skills
ekko skill list

# 安装内置 skill
ekko skill install --name <NAME> [--dry-run|--apply] [--home <PATH>]

# 从 URL 安装（未来）
# ekko skill install --url <URL> [--dry-run|--apply] [--home <PATH>]

# 创建 skill 模板
ekko skill create --name <NAME> [--dry-run|--apply] [--home <PATH>]

# 删除 skill
ekko skill remove --name <NAME> --apply --yes [--home <PATH>]
```

**内置 Skills**（v0.1.0）：
- `explain-code` - 代码解释（带图表和类比）
- `codebase-visualizer` - 代码库可视化
- `pr-summary` - PR 摘要生成

---

## v0.2.0 功能规划

```text
Ekko v0.2.0
├── 状态管理
│   ├── ekko status              # 配置状态总览
│   ├── ekko backup              # 备份配置
│   └── ekko restore             # 恢复配置
│
├── 模板增强
│   ├── ekko template list       # 列出所有模板
│   ├── ekko template show       # 查看模板内容
│   └── ekko diff                # 本地与模板差异
│
└── Skills 增强
    └── 从 GitHub/URL 安装
```

---

## v0.3.0 功能规划

```text
Ekko v0.3.0
├── 分发
│   ├── Homebrew tap (thornboo/homebrew-ekko)
│   └── 更多平台支持
│
├── 架构优化
│   ├── CLI 重构 (clap)
│   └── 错误处理标准化 (thiserror)
│
└── 扩展
    ├── 更多服务商
    └── 插件系统
```

---

## 代码规范

### 注释要求

```rust,ignore
// Good: English, concise, at key points
fn discover(home: Option<PathBuf>) -> Result<Self> {
    // Priority: CLI arg > EKKO_HOME > HOME
    ...
}

// Good: explain non-obvious logic
fn upsert_managed_block(content: &str, ...) -> String {
    // Keep content outside markers unchanged
    ...
}

// Bad: verbose or non-English
fn discover(home: Option<PathBuf>) -> Result<Self> {
    // 这个函数用于发现 HOME 目录，首先检查传入的参数...
    ...
}

// Bad: comment obvious code
fn is_empty(&self) -> bool {
    // Check if changes is empty
    self.changes.is_empty()
}
```

**规则**：
1. 关键逻辑必须有注释
2. 注释使用英文
3. 注释简洁，不超过一行
4. 不注释显而易见的代码

### 命名约定

```rust,ignore
// Functions: snake_case, verb first
fn parse_home() { }
fn apply_changeset() { }
fn install_skill() { }

// Types: PascalCase
struct EkkoHome { }
enum ApplyMode { }

// Constants: SCREAMING_SNAKE_CASE
const DEFAULT_LANG: &str = "zh-CN";
```

---

## 行动计划

### Phase 1：模板和 Skills（1-2 周）

| 任务 | 优先级 | 状态 |
|------|--------|------|
| 从 zcf 移植 output-styles | P0 | 已完成 |
| 从 zcf 移植 git workflows | P0 | 已完成 |
| 从 zcf 移植 workflow.md | P0 | 已完成 |
| 从 zcf 移植 agents (3个) | P0 | 已完成 |
| 从 zcf 移植 commands (2个) | P0 | 已完成 |
| 完成 templates.rs | P0 | 已完成 |
| 实现 skill list/install/create/remove | P0 | 已完成 |
| 添加内置 skills (3个) | P1 | 已完成 |

**验收标准**：
- `cargo build` 成功
- `ekko init --tool all` 生成所有模板
- `ekko skill list` 正常列出内置/已安装 skills
- `ekko skill install --name explain-code --apply` 安装成功

### Phase 2：服务商和测试（1-2 周）

| 任务 | 优先级 | 状态 |
|------|--------|------|
| 支持 OpenRouter（Codex provider preset） | P0 | 已完成 |
| 支持 DeepSeek（Codex provider preset） | P0 | 已完成 |
| 支持 Ollama（Codex provider preset） | P1 | 已完成 |
| 支持 Volcengine（Codex provider preset） | P1 | 已完成 |
| 支持 SiliconFlow（Codex provider preset） | P1 | 已完成 |
| 核心模块单元测试（providers/presets） | P0 | 已完成 |

### Phase 3：发布准备（1 周）

状态：已完成（待实际发布与打 tag）。

| 任务 | 优先级 | 状态 |
|------|--------|------|
| 补全 Cargo.toml 元数据 | P0 | 已完成 |
| 配置 CI/CD | P0 | 已完成 |
| 发布到 GitHub Releases | P0 | 已配置（待执行） |
| 发布到 crates.io | P0 | 已配置（待执行） |

### 里程碑

| 里程碑 | 目标 | 交付物 |
|--------|------|--------|
| M1 | +2 周 | 模板 + Skills 完成 |
| M2 | +3 周 | 服务商 + 测试完成 |
| M3 | +4 周 | v0.1.0 正式发布 |

---

## 发布计划

### v0.1.0 发布渠道

| 渠道 | 优先级 | 状态 |
|------|--------|------|
| GitHub Releases | P0 | 已配置（待执行） |
| crates.io | P0 | 已配置（待执行） |

### 后续发布渠道

| 渠道 | 优先级 | 说明 |
|------|--------|------|
| Homebrew tap | P1 | 需创建 `thornboo/homebrew-ekko` 仓库 |
| homebrew-core | P2 | 需 75+ stars，成熟后考虑 |

### 发布检查清单

- [x] `cargo fmt --all --check` 通过
- [x] `cargo clippy` 无警告
- [x] `cargo test --all` 通过
- [x] README 更新
- [x] CHANGELOG 创建
- [ ] 版本号更新
- [ ] Git tag 创建

---

## 长期展望

### v0.4.0+

- 配置同步（通过 Git 仓库）
- 模板市场（远程模板仓库）
- Web UI（可选）
- 跨平台 GUI（Tauri）
