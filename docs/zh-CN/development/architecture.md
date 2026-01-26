# 架构概览


本文档面向贡献者和二次开发者，描述 Ekko 的代码结构与关键抽象。

## Workspace 布局

```
Ekko/
├── Cargo.toml              # Workspace 配置
├── crates/
│   ├── ekko-cli/           # CLI 入口
│   │   ├── Cargo.toml
│   │   └── src/main.rs
│   └── ekko-core/          # 核心业务逻辑
│       ├── Cargo.toml
│       ├── src/
│       │   ├── lib.rs
│       │   ├── changeset.rs
│       │   ├── paths.rs
│       │   ├── templates.rs
│       │   ├── installer.rs
│       │   ├── claude.rs
│       │   ├── codex.rs
│       │   ├── managed_block.rs
│       │   ├── json_text.rs
│       │   ├── toml_text.rs
│       │   └── project.rs
│       └── assets/         # 内置模板资源
└── docs/                   # 文档
```

### 职责划分

| Crate | 职责 |
|-------|------|
| `ekko-cli` | 命令解析、参数验证、输出格式化、dry-run/apply 流程编排 |
| `ekko-core` | 纯业务逻辑、路径解析、变更集、模板渲染、安装计划 |

**设计原则**：`ekko-core` 不依赖任何 CLI 框架，可被其他上游调用。

---

## 核心抽象

### 1. HOME 沙箱

通过 `--home "<PATH>"` 或 `EKKO_HOME` 环境变量，将所有工具配置目录的读写重定向到沙箱：

```
原始路径              沙箱路径
~/.codex/     →      <sandbox>/.codex/
~/.claude/    →      <sandbox>/.claude/
~/.gemini/    →      <sandbox>/.gemini/
```

**实现位置**：`crates/ekko-core/src/paths.rs`

```rust
pub struct EkkoHome {
    home_dir: PathBuf,
}

impl EkkoHome {
    /// HOME 发现优先级：
    /// 1. 显式传入的 home 参数
    /// 2. EKKO_HOME 环境变量
    /// 3. HOME 环境变量（Unix）
    /// 4. USERPROFILE 环境变量（Windows）
    pub fn discover(home: Option<PathBuf>) -> Result<Self, String>;

    pub fn tool_root(&self, tool: Tool) -> PathBuf;
}
```

### 2. ChangeSet（变更计划）

所有写入/执行行为先被建模为 `ChangeSet`，再通过 `ApplyMode` 控制是否真正执行：

**实现位置**：`crates/ekko-core/src/changeset.rs`

```rust
pub enum Change {
    CreateDirAll { path: PathBuf },
    WriteFile { path: PathBuf, bytes: Vec<u8>, overwrite: bool },
    RunCommand { program: String, args: Vec<String> },
}

pub struct ChangeSet {
    changes: Vec<Change>,
}

pub enum ApplyMode {
    DryRun,  // 仅打印计划
    Apply,   // 实际执行
}
```

**设计理念**：
- 所有副作用都通过 `ChangeSet` 建模
- `DryRun` 模式让用户先预览变更
- 可注入 `FileSystem` 和 `CommandRunner` trait 进行测试

### 3. 模板系统

模板文件以 `include_str!` 打包进二进制，`init`/`update` 生成对应 `ChangeSet`：

**实现位置**：
- `crates/ekko-core/src/templates.rs`
- `crates/ekko-core/assets/`

```rust
pub fn plan_templates_codex(home: &EkkoHome, lang: TemplateLang) -> ChangeSet;
pub fn plan_templates_claude(home: &EkkoHome, lang: TemplateLang) -> ChangeSet;
pub fn plan_templates_gemini(home: &EkkoHome, lang: TemplateLang) -> ChangeSet;
```

**资源目录结构**：
```
assets/
├── claude/
│   ├── agents/ekko/zh-CN/
│   ├── commands/ekko/zh-CN/
│   └── output-styles/zh-CN/
├── codex/
│   ├── agents/zh-CN/
│   └── prompts/ekko/zh-CN/
└── gemini/
    └── zh-CN/
```

### 4. 受管块（Managed Block）

对需要保留用户内容的文件，采用标记块替换：

**实现位置**：`crates/ekko-core/src/managed_block.rs`

```rust
pub fn upsert_managed_block(
    content: &str,
    start_marker: &str,
    end_marker: &str,
    block: &str,
) -> String;

pub fn extract_managed_block(
    content: &str,
    start_marker: &str,
    end_marker: &str,
) -> Option<String>;
```

**使用示例**：
```markdown
用户自定义内容...

<!-- ekko:start -->
Ekko 管理的内容（会被更新）
<!-- ekko:end -->

用户自定义内容...
```

### 5. 配置文件操作

纯文本方式操作 JSON 和 TOML 文件：

| 模块 | 用途 |
|------|------|
| `json_text.rs` | 操作 `settings.json`、`auth.json` |
| `toml_text.rs` | 操作 `config.toml` |

**设计选择**：使用文本操作而非完整解析，以保持格式稳定、减少意外变更。

---

## 架构图

```
┌─────────────────────────────────────────────────────────┐
│                      ekko-cli                           │
│  ┌─────────────────────────────────────────────────┐   │
│  │                   CLI Layer                      │   │
│  │    命令解析 → 参数验证 → 调用 core → 输出结果    │   │
│  └─────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────┐
│                      ekko-core                          │
│                                                         │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐              │
│  │ ChangeSet│  │ Templates│  │ Installer│              │
│  │ 变更计划  │  │ 模板渲染  │  │ 安装计划  │              │
│  └──────────┘  └──────────┘  └──────────┘              │
│                                                         │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐              │
│  │  Paths   │  │  Config  │  │  Text    │              │
│  │ 路径沙箱  │  │ 配置管理  │  │ 文本操作  │              │
│  │          │  │ (claude/ │  │ (json/   │              │
│  │          │  │  codex)  │  │  toml)   │              │
│  └──────────┘  └──────────┘  └──────────┘              │
└─────────────────────────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────┐
│                    File System                          │
│     ~/.codex/       ~/.claude/       ~/.gemini/         │
└─────────────────────────────────────────────────────────┘
```

---

## 数据流

### init/update 命令

```
用户输入                    ekko-core                    文件系统
   │                           │                            │
   │  ekko update --tool all   │                            │
   │ ─────────────────────────>│                            │
   │                           │                            │
   │                     plan_templates_*()                 │
   │                           │                            │
   │                     ChangeSet                          │
   │                           │                            │
   │     [DryRun] 打印计划     │                            │
   │ <─────────────────────────│                            │
   │                           │                            │
   │     [Apply] 执行变更      │      写入文件              │
   │                           │ ──────────────────────────>│
```

### codex provider set 命令

```
用户输入                    ekko-core                    文件系统
   │                           │                            │
   │  --base-url --model       │                            │
   │ ─────────────────────────>│                            │
   │                           │                            │
   │                    读取现有配置                         │
   │                           │<───────────────────────────│
   │                           │                            │
   │                    upsert_codex_provider()             │
   │                           │                            │
   │                    ChangeSet                           │
   │                           │                            │
   │                    写入 config.toml                    │
   │                           │───────────────────────────>│
```

---

## 测试策略

### 单元测试

通过 trait 注入实现测试隔离：

```rust
pub trait FileSystem {
    fn create_dir_all(&self, path: &Path) -> io::Result<()>;
    fn write_file(&self, path: &Path, bytes: &[u8], overwrite: bool) -> io::Result<()>;
    fn path_exists(&self, path: &Path) -> bool;
}

pub trait CommandRunner {
    fn run(&self, program: &str, args: &[String]) -> io::Result<ExitStatus>;
}
```

测试时可注入 mock 实现，验证 `ChangeSet` 内容而不实际执行。

### 集成测试

使用 `--home` 沙箱进行端到端测试，验证完整流程。

---

## 扩展点

### 添加新工具支持

1. 在 `paths.rs` 中添加 `Tool` 枚举变体
2. 在 `templates.rs` 中添加模板加载函数
3. 在 `assets/` 中添加模板文件
4. 在 `main.rs` 中添加命令处理

### 添加新命令

1. 在 `ekko-core` 中实现业务逻辑，返回 `ChangeSet`
2. 在 `ekko-cli` 中添加命令解析和调用

### 添加新模板

1. 在 `assets/<tool>/<lang>/` 中添加模板文件
2. 在 `templates.rs` 中使用 `include_str!` 引入
3. 在对应的 `plan_templates_*` 函数中添加写入逻辑
