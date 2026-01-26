# 贡献指南

感谢你对 Ekko 的兴趣！本仓库是一个 Rust CLI（workspace：`ekko-cli` + `ekko-core`）。

## 开发环境

- Rust（建议使用 rustup）
- Git

## 常用命令

```bash
# 格式化
cargo fmt --all

# Lint（要求 0 warnings）
cargo clippy -- -D warnings

# 测试
cargo test --all

# 本地运行
cargo run -p ekko -- --help
```

## 代码规范（节选）

- 保持简单：KISS / DRY / YAGNI
- 公共 API 需要 `///` 文档注释
- 仅对“非显而易见”的逻辑写注释，注释使用英文且尽量一行

