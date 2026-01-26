# 快速开始


本页假设你已经安装好 `ekko`（见：`./installation.md`）。

## 1. 强烈推荐：先用沙箱演练

```bash
export EKKO_HOME="/tmp/ekko-home"

# 预览将要写入的文件（dry-run）
ekko init --tool all

# 真正写入（但只写到沙箱目录）
ekko init --tool all --apply
```

## 2. 初始化/更新模板（真实 HOME）

确认沙箱结果符合预期后，再对真实 HOME 执行：

```bash
unset EKKO_HOME

ekko init --tool all          # dry-run
ekko init --tool all --apply  # apply
```

更新模板（受管写入，尽量不破坏用户配置）：

```bash
ekko update --tool all --apply
```

## 3. 配置三套工具

### Codex（Provider）

```bash
ekko codex provider set \
  --provider "openrouter" \
  --api-key "sk-xxx" \
  --default \
  --apply
```

### Claude Code（env / output style）

```bash
ekko claude env set --auth-token "sk-xxx" --model "claude-sonnet-4" --apply
ekko claude output-style use --name "ekko-engineer-professional" --apply
```

### Gemini CLI（env）

```bash
ekko gemini env set --api-key "xxx" --model "gemini-2.0-flash" --apply
```

## 4. 项目初始化

在项目目录生成 `.ekko/plan/` 与 `.gemini/GEMINI.md`（受管块）：

```bash
ekko project init --path "/path/to/your/project" --apply
```

详见：`../projects/project-init.md`。

## 5. 全局安装/升级 AI 工具（npm / brew）

⚠️ 危险操作：需要 `--apply --yes`

```bash
ekko install --tool all --install-method auto          # dry-run
ekko install --tool all --install-method auto --apply --yes
```

## 下一步

- 命令参考：`../commands/index.md`
- 模板说明：`../templates/index.md`
- Skills：`../skills/overview.md`

