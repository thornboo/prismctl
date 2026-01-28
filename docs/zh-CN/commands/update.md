# `prismctl update`


更新内置模板（覆盖 Prismctl 命名空间文件、更新受管块）。

从行为上看，`update` 与 `init` 的区别主要在于语义：

- `init`：用于首次初始化（“把 Prismctl 内置模板放到位”）
- `update`：用于后续升级（“把 Prismctl 内置模板同步到最新版本”）

当前版本中，两者会写入同一批 Prismctl 管理的模板文件；因此你可以把 `update` 理解为“重复执行 init，但目的是升级”。

```bash
prismctl update --tool <codex|claude|gemini|all> [--home <PATH>] [--lang <zh-CN|en>] [--dry-run|--apply]
```

快捷别名：

```bash
prismctl u --tool <codex|claude|gemini|all> [--lang <zh-CN|en>] [--home <PATH>] [--dry-run|--apply]
```

示例：

```bash
# 预览变更
prismctl update --tool all

# 应用更新
prismctl update --tool all --apply
```

## 常见问题

### 我修改了 `~/.codex/prompts/prismctl/*`，update 会覆盖吗？

会。`prompts/prismctl/` 属于 Prismctl 命名空间文件，默认可安全覆盖更新。如果你需要保留本地改动，建议：

1. 把自定义版本复制到非 `prismctl/` 命名空间目录
2. 在工具侧引用你的自定义版本，而不是直接改 Prismctl 命名空间文件
