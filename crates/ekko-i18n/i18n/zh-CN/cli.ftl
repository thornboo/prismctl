# 非 TTY（脚本/CI）错误提示
error-non-tty-title = 错误: 检测到非交互式环境（stdin/stdout 不是 TTY），无法进行参数补全/交互确认。
error-non-tty-interactive-title = 错误: 检测到非交互式环境（stdin/stdout 不是 TTY），无法进入 Ekko 交互式向导。
error-non-tty-scope = 影响范围：
error-non-tty-running =   - 你正在运行：{ $invocation }
error-non-tty-missing =   - 缺少/需要确认：{ $missing }
error-non-tty-solutions = 解决方案：
error-non-tty-solution-tty =   - 在交互式终端中重试（TTY）以启用补全/确认
error-non-tty-interactive-solution-menu =   - 在交互式终端中运行 `ekko` 或 `ekko config` 进入向导
error-non-tty-interactive-solution-subcmd =   - 或改用子命令（适用于脚本/CI），例如：`ekko init --tool all` / `ekko update --tool all`
error-non-tty-solution-args =   - 或显式补全参数（适用于脚本/CI），例如：`--tool codex` / `--name xxx` / `--yes`
error-non-tty-solution-help =   - 查看完整帮助：`ekko --help`

error-unknown-command = 未知命令: { $cmd }

# 通用 CLI 解析错误
error-flag-missing-value = 参数 { $flag } 缺少值
error-non-tty-missing-flag = 错误: 在非交互式环境中缺少必填参数 { $flag }
error-home-not-found = 错误: 无法确定 HOME。请设置 EKKO_HOME（或系统 HOME/USERPROFILE）
error-config-dir-create = 错误: 创建配置目录失败: { $path }: { $error }
error-config-write = 错误: 写入配置文件失败: { $path }: { $error }

# legacy/commands.rs（通用解析/用法错误）
error-unsupported-args-with-help = { $cmd } 不支持的参数: { $args }
error-missing-subcommand-with-help = { $cmd } 缺少子命令
error-unknown-subcommand-with-help = 未知 { $cmd } 子命令: { $sub }
error-missing-flag = 缺少参数 { $flag }
error-current-dir = 获取当前目录失败: { $error }
error-timestamp = 获取时间戳失败: { $error }
error-lang-flag-invalid = 缺少/不合法的 --lang <zh-CN|en>
error-unknown-agent = 未知 agent: { $name }
error-tool-value-unsupported = 不支持的 --tool 值: { $value }
error-lang-value-unsupported = 不支持的 --lang 值: { $value }
error-install-method-value-unsupported = 不支持的 --install-method 值: { $value }
error-codex-provider-set-needs-args = codex provider set 需要至少传入一个参数：--provider/--api-key/--base-url/--model/--wire-api/--default
error-claude-env-set-needs-args = claude env set 需要至少传入一个参数：--auth-token/--base-url/--model/--haiku-model/--sonnet-model/--opus-model
error-gemini-env-set-needs-args = gemini env set 需要至少传入一个参数：--api-key/--base-url/--model

# Quick init/update（别名：`ekko i`、`ekko u`）
quick-provider-presets-title = （可选）provider 预设（Codex）：
quick-skip = 0) 跳过
quick-prompt = >
quick-api-key-prompt = 请输入 API Key（留空跳过）:

# prompted.rs 交互补全提示
prompt-confirm-continue = 请确认是否继续？
prompt-tool-init = 选择要初始化的工具（--tool）：
prompt-tool-update = 选择要更新的工具（--tool）：
prompt-tool-install = 选择要安装的工具（--tool）：
prompt-tool-upgrade = 选择要升级的工具（--tool）：
error-tool-flag-invalid = 缺少/不合法的 --tool <codex|claude|gemini|all>
prompt-skill-subcommand = 选择 skill 子命令：
error-missing-subcommand = 缺少子命令
prompt-skill-name-flag = 请输入 skill 名称（--name）：
prompt-codex-subcommand = 选择 codex 子命令：
prompt-codex-agent-flag = 选择 Codex agent（--name）：
prompt-codex-agent-custom = custom（手动输入）
prompt-codex-agent-name = 请输入 agent 名称：
prompt-project-subcommand = 选择 project 子命令：

# legacy/commands.rs 通用输出
changeset-preview-title = 将执行以下变更（mode={ $mode }）：
changeset-no-changes = 无变更。
apply-applied = 已应用。
apply-completed = 已完成。

dry-run-hint-write = 提示：这是 dry-run。传入 --apply 才会真正写入。
dry-run-hint-write-skill-files = 提示：这是 dry-run。传入 --apply 才会真正写入 skill 文件。
dry-run-hint-create-skill = 提示：这是 dry-run。传入 --apply 才会真正创建 skill。
dry-run-hint-remove-skill = 提示：这是 dry-run。传入 --apply 才会真正删除 skill。
dry-run-hint-write-project = 提示：这是 dry-run。传入 --apply 才会真正写入项目文件。
dry-run-hint-install-upgrade = 提示：这是 dry-run。传入 --apply 才会真正执行全局安装/升级。

codex-agent-list-title = 可用 Codex agent（内置）：
codex-agent-switched = 将 Codex 的 AGENTS.md 切换为: { $name }
codex-provider-default-set = 将 Codex 的默认 model_provider 设置为: ekko
codex-provider-preset = 使用 provider 预设: { $provider }
codex-auth-key-hidden = 将写入 Codex auth.json 键: EKKO_CODEX_API_KEY（值已隐藏）
claude-auth-token-hidden = 将写入 Claude settings.json env 键: ANTHROPIC_AUTH_TOKEN（值已隐藏）
claude-output-style-set = 将 Claude 的 outputStyle 设置为: { $name }
gemini-env-managed-keys = 将管理 Gemini 环境变量块（keys）：GEMINI_API_KEY, GOOGLE_GEMINI_BASE_URL, GEMINI_MODEL

skill-builtin-title = 内置 skills：
skill-installed-title = 已安装 skills：
skill-none = (none)

# 危险操作（legacy 默认要求 --yes；prompted 在 TTY 下会改为交互确认）
danger-title = [!] 危险操作检测！
danger-confirm-need-yes = 请确认是否继续？需要显式传入 "--yes"（等价于确认继续）

danger-skill-remove-type = 操作类型：删除 skill
danger-skill-remove-scope = 影响范围：将递归删除 ~/.claude/skills/{ $name } 目录（在 --home 沙箱下则删除沙箱内对应目录）
danger-skill-remove-risk = 风险评估：可能删除用户自定义脚本/配置，操作不可逆（除非你有备份）

danger-install-type = 操作类型：{ $op }
danger-install-scope = 影响范围：将调用 brew 或 npm 修改系统级工具链与全局依赖
danger-install-risk = 风险评估：可能覆盖现有版本、影响 PATH/环境，失败时可能需要手动修复
danger-install-op-install = 全局安装
danger-install-op-upgrade = 全局升级

danger-codex-agent-type = 操作类型：覆盖 Codex 系统提示文件（AGENTS.md）
danger-codex-agent-scope = 影响范围：将改变 Codex 的全局系统提示/输出风格
danger-codex-agent-risk = 风险评估：可能覆盖你当前自定义提示；已自动计划备份（如存在旧文件）
