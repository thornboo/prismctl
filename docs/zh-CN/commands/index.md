# 命令参考


本节是 Ekko CLI 的命令参考，按功能拆分为多个页面。

## 通用约定（强烈建议先读）

- Ekko 默认 `dry-run`：只打印计划，不会写入/删除文件，也不会执行外部命令。
- 只有显式传入 `--apply` 才会真正落盘或执行安装命令。
- 通过 `--home "<PATH>"` 或环境变量 `EKKO_HOME` 可以把所有读写重定向到“沙箱 HOME”，避免污染真实配置。
- 对不可逆/高风险操作，Ekko 额外要求 `--yes`（例如：删除 skill、全局安装/升级、覆盖 Codex 的 `AGENTS.md`）。

## 常用参数速查

> 说明：不是所有参数对所有命令都适用；以对应命令页为准。

| 参数 | 说明 |
|------|------|
| `--home "<PATH>"` | 重定向所有读写到指定目录（HOME 沙箱） |
| `--dry-run` | 仅预览变更（默认） |
| `--apply` | 实际执行变更 |
| `--lang <zh-CN|en>` | 模板语言（默认 `zh-CN`） |
| `--tool <codex|claude|gemini|all>` | 选择初始化/更新/安装的目标工具 |
| `--yes` | 对危险操作的显式确认（通常与 `--apply` 搭配） |
| `--verbose` | 报错时附加调试上下文（cmd/args） |
| `-h, --help` | 显示帮助信息 |

## 基础命令

- `ekko` / `ekko config`：交互式向导入口（仅 TTY）
- `config <CMD> ...`：镜像前缀（`ekko config <CMD> ...` ≡ `ekko <CMD> ...`）
- `i`：quick init（混合模式）
- `u`：quick update
- `doctor`：`./doctor.md`
- `init`：`./init.md`
- `update`：`./update.md`

## 配置与管理

- 安装/升级（npm/brew）：`./install-upgrade.md`
- Skills：`./skill.md`
- Codex：`./codex.md`
- Claude Code：`./claude.md`
- Gemini CLI：`./gemini.md`
- 项目初始化：`./project.md`
