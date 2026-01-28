# 快速开始


本页假设你已经安装好 `prismctl`（见：`./installation.md`）。

## 0. 交互式向导（仅 TTY）

直接运行 `prismctl`（或 `prismctl config`）即可进入交互式向导。

> 注意：在非 TTY 环境（CI / 管道）下，交互式模式会直接报错退出，避免等待输入导致假死。

```bash
prismctl
```

## 1. 强烈推荐：先用沙箱演练

Prismctl 支持通过 `--home "<PATH>"` 或 `PRISMCTL_HOME` 把所有读写重定向到一个“沙箱 HOME”，从而做到可控、可回收的演练。

```bash
export PRISMCTL_HOME="/tmp/prismctl-home"

# 预览将要写入的文件（dry-run）
prismctl init --tool all

# 真正写入（但只写到沙箱目录）
prismctl init --tool all --apply
```

你也可以用 `prismctl doctor` 验证路径映射是否符合预期：

```bash
prismctl doctor
```

## 2. 初始化/更新模板（真实 HOME）

确认沙箱结果符合预期后，再对真实 HOME 执行：

```bash
unset PRISMCTL_HOME

prismctl init --tool all          # dry-run
prismctl init --tool all --apply  # apply
```

更新模板（受管写入，尽量不破坏用户配置）：

```bash
prismctl update --tool all --apply
```

快捷别名（可选）：

```bash
prismctl i --tool all --apply
prismctl u --tool all --apply
```

> 说明：`init`/`update` 都会覆盖 Prismctl 命名空间内的模板文件（例如 `~/.codex/prompts/prismctl/*`）。它们的“保留用户配置”主要指：不触碰 `prismctl/` 之外的文件，且对少数共享文件使用受管块写入。

## 3. 配置三套工具

### Codex（Provider）

```bash
prismctl codex provider set \
  --provider "openrouter" \
  --api-key "sk-xxx" \
  --default \
  --apply
```

该命令会更新：

- `~/.codex/config.toml`：新增/更新 Prismctl provider（`model_providers.prismctl`）
- `~/.codex/auth.json`：写入 `PRISMCTL_CODEX_API_KEY`（值不会出现在 `config.toml` 明文里）

### Claude Code（env / output style）

```bash
prismctl claude env set --auth-token "sk-xxx" --model "claude-sonnet-4" --apply
prismctl claude output-style use --name "prismctl-engineer-professional" --apply
```

### Gemini CLI（env）

```bash
prismctl gemini env set --api-key "xxx" --model "gemini-2.0-flash" --apply
```

该命令会在 `~/.gemini/.env` 中维护一个 Prismctl 受管块（不会覆盖块外内容）。

## 4. 项目初始化

在项目目录生成 `.prismctl/plan/` 与 `.gemini/GEMINI.md`（受管块）：

```bash
prismctl project init --path "/path/to/your/project" --apply
```

详见：`../projects/project-init.md`。

## 5. 全局安装/升级 AI 工具（npm / brew）

⚠️ 危险操作：需要 `--apply --yes`

```bash
prismctl install --tool all --install-method auto          # dry-run
prismctl install --tool all --install-method auto --apply --yes
```

## 下一步

- 命令参考：`../commands/index.md`
- 模板说明：`../templates/index.md`
- Skills：`../skills/overview.md`
