# 贡献指南（中文）


感谢你对 Ekko 项目的兴趣！本文档说明如何参与开发。

## 开发环境

### 前置要求

- Rust 1.70+（推荐使用 rustup 安装）
- Git

### 克隆和构建

```bash
git clone https://github.com/thornboo/ekko.git
cd ekko
cargo build
```

### 运行测试

```bash
cargo test --all
```

### 运行 Clippy

```bash
cargo clippy --all-targets -- -D warnings
```

### 格式化代码

```bash
cargo fmt --all
```

---

## 项目结构

```text
Ekko/
├── crates/
│   ├── ekko-cli/       # CLI 入口
│   └── ekko-core/      # 核心业务逻辑
├── docs/               # 文档
└── tests/              # 集成测试（规划中）
```

详细架构说明见 [architecture.md](./architecture.md)。

---

## 开发流程

### 1. 创建分支

```bash
git checkout -b feature/your-feature-name
```

分支命名约定：
- `feature/xxx` - 新功能
- `fix/xxx` - Bug 修复
- `docs/xxx` - 文档更新
- `refactor/xxx` - 重构

### 2. 编写代码

遵循以下原则：

**代码风格**：
- 使用 `cargo fmt` 格式化
- 通过 `cargo clippy` 检查
- 函数名使用 `snake_case`
- 类型名使用 `PascalCase`

**文档注释**：
- 公共 API 必须有文档注释
- 使用 `///` 格式

```rust,ignore
/// 发现并解析 Ekko HOME 目录。
///
/// # 优先级
/// 1. 显式传入的 `home` 参数
/// 2. 环境变量 `EKKO_HOME`
/// 3. 系统 HOME 目录
///
/// # 错误
/// 如果无法确定 HOME 目录，返回错误。
pub fn discover(home: Option<PathBuf>) -> Result<Self, String> {
    // ...
}
```

**测试**：
- 新功能需要有对应测试
- 修复 bug 需要有回归测试

### 3. 提交代码

使用 Conventional Commits 格式：

```text
<type>(<scope>): <description>

[optional body]

[optional footer]
```

**Type**：
- `feat` - 新功能
- `fix` - Bug 修复
- `docs` - 文档
- `style` - 格式（不影响代码运行）
- `refactor` - 重构
- `test` - 测试
- `chore` - 构建/工具

**示例**：
```text
feat(claude): add output-style list command

Add a new command to list all available output styles.

Closes #123
```

### 4. 提交 Pull Request

- 确保所有测试通过
- 确保 Clippy 无警告
- 填写 PR 模板
- 关联相关 Issue

---

## 代码规范

### 注释规范

**必须遵守**：
1. 关键逻辑必须有注释
2. 注释使用英文
3. 注释简洁，不超过一行
4. 不注释显而易见的代码

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
    // 这个函数用于发现 HOME 目录...
    ...
}

// Bad: comment obvious code
fn is_empty(&self) -> bool {
    // Check if changes is empty  <- unnecessary
    self.changes.is_empty()
}
```

### 错误处理

当前使用 `Result<T, String>`，计划迁移到 `thiserror`：

```rust,ignore
// Current
fn foo() -> Result<(), String> {
    Err("error message".to_string())
}

// Future (recommended)
fn foo() -> Result<(), EkkoError> {
    Err(EkkoError::InvalidArgument("...".into()))
}
```

### 命名约定

```rust,ignore
// 函数：动词开头
fn parse_home() { }
fn apply_changeset() { }

// 布尔变量/函数：is_/has_/can_ 前缀
fn is_empty() -> bool { }
let has_api_key = api_key.is_some();

// 常量：SCREAMING_SNAKE_CASE
const DEFAULT_LANG: &str = "zh-CN";
```

### 模块组织

```rust,ignore
// 导入顺序
use std::...;           // 标准库
use external_crate::...; // 外部 crate
use crate::...;         // 本 crate
use super::...;         // 父模块
```

---

## 添加新功能

### 添加新命令

1. **在 ekko-core 中实现业务逻辑**：

```rust,ignore
// crates/ekko-core/src/new_feature.rs
pub fn plan_new_feature(home: &EkkoHome) -> ChangeSet {
    let mut cs = ChangeSet::new();
    // 添加变更...
    cs
}
```

2. **在 ekko-cli 中添加命令处理**：

```rust,ignore
// crates/ekko-cli/src/main.rs
fn cmd_new_feature(mut args: Vec<String>) -> Result<(), String> {
    let home = parse_home(&mut args)?;
    let mode = parse_apply_mode(&mut args)?;

    let home = EkkoHome::discover(home)?;
    let cs = new_feature::plan_new_feature(&home);

    // 执行变更...
}
```

3. **更新帮助文档**：

```rust,ignore
fn help() -> String {
    // 添加新命令说明
}
```

4. **添加测试**。

5. **更新文档**（`docs/zh-CN/commands/index.md` + `docs/en/commands/index.md`）。

### 添加新模板

1. 在 `crates/ekko-core/assets/<tool>/<lang>/` 创建模板文件
2. 在 `templates.rs` 中使用 `include_str!` 引入
3. 在 `plan_templates_*` 函数中添加写入逻辑
4. 更新 `docs/zh-CN/templates/index.md` + `docs/en/templates/index.md`

---

## 测试指南

### 单元测试

```rust,ignore
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_name() {
        // Arrange
        let input = "...";

        // Act
        let result = function_under_test(input);

        // Assert
        assert_eq!(result, expected);
    }
}
```

### 使用沙箱测试

```rust,ignore
#[test]
fn test_with_sandbox() {
    let temp_dir = tempfile::tempdir().unwrap();
    let home = EkkoHome::discover(Some(temp_dir.path().to_path_buf())).unwrap();

    // 测试逻辑...
}
```

---

## 文档贡献

### 文档结构

```text
docs/
├── README.md           # 入口（按语言跳转）
├── zh-CN/              # 中文文档（结构需与 en 对齐）
├── en/                 # English docs (mirrors zh-CN structure)
└── _shared/            # 共享资源（图片等）
    └── mdbook/         # mdBook 的界面资源（按语言在 docs/<locale>/_shared/mdbook/）
```

### 文档风格

- 使用简洁清晰的语言
- 提供代码示例
- 保持中英双语同步（如适用）

---

## 发布流程

参见 [roadmap.md](./roadmap.md) 中的发布检查清单。

---

## 获取帮助

- 提交 Issue：https://github.com/thornboo/ekko/issues
- 阅读文档：本目录下的其他文档

感谢你的贡献！
