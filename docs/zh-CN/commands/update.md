# `ekko update`


更新内置模板（覆盖 Ekko 命名空间文件、更新受管块）。

从行为上看，`update` 与 `init` 的区别主要在于语义：

- `init`：用于首次初始化（“把 Ekko 内置模板放到位”）
- `update`：用于后续升级（“把 Ekko 内置模板同步到最新版本”）

当前版本中，两者会写入同一批 Ekko 管理的模板文件；因此你可以把 `update` 理解为“重复执行 init，但目的是升级”。

```bash
ekko update --tool <codex|claude|gemini|all> [--home <PATH>] [--lang <zh-CN|en>] [--dry-run|--apply]
```

快捷别名：

```bash
ekko u --tool <codex|claude|gemini|all> [--lang <zh-CN|en>] [--home <PATH>] [--dry-run|--apply]
```

示例：

```bash
# 预览变更
ekko update --tool all

# 应用更新
ekko update --tool all --apply
```

## 常见问题

### 我修改了 `~/.codex/prompts/ekko/*`，update 会覆盖吗？

会。`prompts/ekko/` 属于 Ekko 命名空间文件，默认可安全覆盖更新。如果你需要保留本地改动，建议：

1. 把自定义版本复制到非 `ekko/` 命名空间目录
2. 在工具侧引用你的自定义版本，而不是直接改 Ekko 命名空间文件
