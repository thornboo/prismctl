# `ekko init`


初始化内置模板（首次安装建议使用）。

```bash
ekko init --tool <codex|claude|gemini|all> [--home <PATH>] [--lang <zh-CN|en>] [--dry-run|--apply]
```

示例：

```bash
# 预览所有工具的模板写入
ekko init --tool all

# 写入到沙箱 HOME
ekko init --tool all --home "/tmp/ekko-home" --apply

# 仅写入 Claude 模板
ekko init --tool claude --apply
```

写入策略与安全模型见：

- `../concepts/safety-model.md`
- `../concepts/managed-write-strategy.md`

