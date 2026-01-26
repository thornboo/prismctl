# 安全模型


Ekko 默认以“安全优先”为设计目标：尽量避免 CLI bug 对用户真实配置造成不可逆破坏。

## 1. 默认 dry-run

- 所有会写入/删除文件的命令，默认仅打印计划（dry-run），不会落盘。
- 只有显式传入 `--apply` 才会执行变更。

## 2. HOME 沙箱（强烈推荐）

使用 `--home "<PATH>"` 或环境变量 `EKKO_HOME`，把 Ekko 的所有读写重定向到指定目录：

```bash
export EKKO_HOME="/tmp/ekko-home"
ekko update --tool all          # dry-run
ekko update --tool all --apply  # 写入到 /tmp/ekko-home 下的 .codex/.claude/.gemini
```

这适用于：

- 首次上手/演练命令
- CI / 集成测试
- 你不希望破坏本机真实配置的场景

## 3. 危险操作需要 --yes

对“不可安全合并或需要额外确认”的操作，Ekko 会要求额外提供 `--yes`：

- 例如：覆盖 Codex 的 `AGENTS.md`、删除 skill 等

通常需要组合使用：

```bash
ekko codex agent use --name ekko-engineer-professional --apply --yes
ekko skill remove --name my-skill --apply --yes
```

## 4. 受管写入策略

Ekko 将写入分为三类（详见：`./managed-write-strategy.md`）：

- 命名空间文件：仅写入 `ekko/` 命名空间目录（可安全覆盖）
- 受管块：只更新标记块，保留用户内容
- 显式覆盖：需要 `--yes`，并在覆盖前备份

