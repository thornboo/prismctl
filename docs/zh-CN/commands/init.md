# `ekko init`


初始化内置模板（首次安装建议使用）。

它会把 Ekko 内置的“命名空间模板文件”写入到各工具的配置目录下（详见：`../templates/index.md`）。默认 `dry-run`，先预览再 `--apply`。

```bash
ekko init --tool <codex|claude|gemini|all> [--home <PATH>] [--lang <zh-CN|en>] [--dry-run|--apply]
```

快捷别名（混合模式）：

```bash
ekko i --tool <codex|claude|gemini|all> [--lang <zh-CN|en>] [--home <PATH>] [--dry-run|--apply]
```

## 你将得到什么

- Codex：`~/.codex/prompts/ekko/*`（工作流、Git 辅助命令等）
- Claude Code：`~/.claude/commands/ekko/*`、`~/.claude/agents/ekko/*`、`~/.claude/output-styles/*`
- Gemini CLI：`~/.gemini/ekko/WORKFLOWS.md`，以及 `~/.gemini/GEMINI.md` 的 Ekko 受管块

> 注意：命名空间文件属于“Ekko 完全管理”的内容，后续 `update` 会覆盖更新；如果你想深度定制，建议复制一份到非 `ekko/` 命名空间再改。

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
