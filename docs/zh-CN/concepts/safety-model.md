# 安全模型


Prismctl 默认以“安全优先”为设计目标：尽量避免 CLI bug 对用户真实配置造成不可逆破坏。

## 1. 默认 dry-run（不落盘）

- 所有会写入/删除文件的命令，默认仅打印计划（dry-run），不会落盘。
- 只有显式传入 `--apply` 才会执行变更。

这意味着你可以先用同一条命令“预演”：

```bash
prismctl init --tool all
prismctl update --tool all
prismctl install --tool all --install-method auto
```

## 2. HOME 沙箱（强烈推荐，先演练再落盘）

使用 `--home "<PATH>"` 或环境变量 `PRISMCTL_HOME`，把 Prismctl 的所有读写重定向到指定目录：

```bash
export PRISMCTL_HOME="/tmp/prismctl-home"
prismctl update --tool all          # dry-run
prismctl update --tool all --apply  # 写入到 /tmp/prismctl-home 下的 .codex/.claude/.gemini
```

这适用于：

- 首次上手/演练命令
- CI / 集成测试
- 你不希望破坏本机真实配置的场景

## 3. 危险操作需要 --yes（显式确认）

对“不可安全合并或需要额外确认”的操作，Prismctl 会要求额外提供 `--yes`：

- 覆盖 Codex 的 `AGENTS.md`（会改变 Codex 全局系统提示/输出风格）
- 修改 Claude Code 的 MCP 配置（委托 `claude mcp`）
- 修改 Gemini CLI 的 MCP 配置（委托 `gemini mcp`）
- 删除 Claude Code 的某个 skill（递归删除目录）
- 全局安装/升级工具（会调用 `npm` 或 `brew` 修改系统环境）

通常需要组合使用：

```bash
prismctl codex agent use --name prismctl-engineer-professional --apply --yes
prismctl claude mcp add --name context7 --apply --yes
prismctl gemini mcp add --name context7 --apply --yes
prismctl skill remove --name my-skill --apply --yes
prismctl install --tool all --install-method auto --apply --yes
```

## 4. 受管写入策略

Prismctl 将写入分为四类（详见：`./managed-write-strategy.md`）：

- 命名空间文件：仅写入 `prismctl/` 命名空间目录（可安全覆盖）
- 受管块：只更新标记块，保留用户内容
- 结构化 upsert：解析 JSON/TOML，按字段合并更新（可能重写格式，但保留语义）
- 显式覆盖：需要 `--yes`，并在覆盖前备份（如存在旧文件）

## 5. 可追溯性（变更可见）

Prismctl 的所有“将要执行的变更”都会先以人类可读的方式打印出来（例如 `mkdir -p ...`、`write ...`、`run brew ...`）。

建议你把 dry-run 输出当作审计记录：确认无误后再加 `--apply`。
