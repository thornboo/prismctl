# `ekko update`


更新内置模板（覆盖 Ekko 命名空间文件、更新受管块）。

```bash
ekko update --tool <codex|claude|gemini|all> [--home <PATH>] [--lang <zh-CN|en>] [--dry-run|--apply]
```

示例：

```bash
# 预览变更
ekko update --tool all

# 应用更新
ekko update --tool all --apply
```

