# `prismctl init`


初始化内置模板（首次安装建议使用）。

它会把 Prismctl 内置的“命名空间模板文件”写入到各工具的配置目录下（详见：`../templates/index.md`）。默认 `dry-run`，先预览再 `--apply`。

```bash
prismctl init --tool <codex|claude|gemini|all> [--home <PATH>] [--lang <zh-CN|en>] [--dry-run|--apply]
```

快捷别名（混合模式）：

```bash
prismctl i --tool <codex|claude|gemini|all> [--lang <zh-CN|en>] [--home <PATH>] [--dry-run|--apply]
```

## 你将得到什么

- Codex：`~/.codex/prompts/prismctl/*`（工作流、Git 辅助命令等）
- Claude Code：`~/.claude/commands/prismctl/*`、`~/.claude/agents/prismctl/*`、`~/.claude/output-styles/*`
- Gemini CLI：`~/.gemini/prismctl/WORKFLOWS.md`，以及 `~/.gemini/GEMINI.md` 的 Prismctl 受管块

> 注意：命名空间文件属于“Prismctl 完全管理”的内容，后续 `update` 会覆盖更新；如果你想深度定制，建议复制一份到非 `prismctl/` 命名空间再改。

示例：

```bash
# 预览所有工具的模板写入
prismctl init --tool all

# 写入到沙箱 HOME
prismctl init --tool all --home "/tmp/prismctl-home" --apply

# 仅写入 Claude 模板
prismctl init --tool claude --apply
```

写入策略与安全模型见：

- `../concepts/safety-model.md`
- `../concepts/managed-write-strategy.md`
