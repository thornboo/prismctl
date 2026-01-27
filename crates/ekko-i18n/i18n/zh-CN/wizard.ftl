# 向导通用
wizard-prompt-name = 请输入 skill 名称（内置或自定义）：
wizard-prompt-new-name = 请输入新 skill 名称：
wizard-prompt-remove-name = 请输入要删除的 skill 名称：
wizard-confirm-install = 确认写入？输入 "yes" 继续（其它任意键取消）:
wizard-confirm-create = 确认创建？输入 "yes" 继续（其它任意键取消）:
wizard-confirm-remove = WARNING: 将删除 skill 目录。确认继续？输入 "yes" 继续（其它任意键取消）:
wizard-confirm-remove-short = WARNING: 将删除 skill 目录。确认继续？

# 快速初始化向导
wizard-quick-init-title = 快速初始化向导
wizard-quick-init-desc = 这会帮助你：
wizard-quick-init-desc-init = 写入模板（init）
wizard-quick-init-desc-config = 配置工具（按需）
wizard-quick-init-select-tools = 选择要初始化的工具（可多选）：
wizard-quick-init-confirm-write = 确认写入以上模板？
wizard-quick-init-continue-title = 是否继续配置各工具（可选）？
wizard-quick-init-continue = 继续配置？
wizard-quick-init-configure-claude = 配置 Claude Code？
wizard-quick-init-configure-codex = 配置 Codex？
wizard-quick-init-configure-gemini = 配置 Gemini CLI？

# Gemini 向导
wizard-gemini-title = Gemini CLI 配置向导
wizard-gemini-prompt-api-key = api key（留空=使用环境变量默认值；输入 "-"=跳过不写入）：
wizard-gemini-prompt-base-url = base url（留空不设置）：
wizard-gemini-prompt-model = model（留空不设置）：
wizard-no-changes-cancelled = 未提供任何修改，已取消。

# Codex 向导
wizard-codex-title = Codex 配置向导
wizard-codex-provider-title = 选择 API 提供商：
wizard-codex-provider-custom = custom（自定义 base url / model）
wizard-codex-prompt-base-url = base url（留空不设置）：
wizard-codex-prompt-wire-api = wire api（留空不设置）：
wizard-codex-prompt-model = model（留空不设置）：
wizard-codex-prompt-api-key = api key（留空不写入 auth.json）：
wizard-codex-set-default = 设为默认 provider（建议）？
wizard-codex-switch-agent-title = 是否切换 Codex agent（可选）？
wizard-codex-switch-agent = 切换 agent？
wizard-codex-agent-title = 选择 Codex agent：
wizard-codex-agent-custom = custom（手动输入）
wizard-codex-agent-name = agent 名称：
wizard-codex-confirm-overwrite-agents = 确认覆盖 Codex 的 AGENTS.md？

# Claude 向导
wizard-claude-title = Claude Code 配置向导
wizard-claude-provider-title = 选择 API 提供商：
wizard-claude-provider-custom = custom（自定义 base url）
wizard-claude-provider-skip = 跳过（不设置 base url）
wizard-claude-prompt-base-url = base url:
wizard-claude-prompt-auth-token = auth token（留空=使用环境变量默认值；输入 "-"=跳过不写入）：
wizard-claude-prompt-model = model（留空不设置）：
wizard-claude-prompt-haiku-model = default haiku model（留空不设置）：
wizard-claude-prompt-sonnet-model = default sonnet model（留空不设置）：
wizard-claude-prompt-opus-model = default opus model（留空不设置）：

wizard-claude-output-style-title = 是否设置 Claude outputStyle（可选）？
wizard-claude-output-style-confirm = 设置 outputStyle？
wizard-claude-output-style-select = 选择 outputStyle：
wizard-claude-output-style-custom = custom（手动输入）
wizard-claude-output-style-name = outputStyle 名称:

wizard-claude-skills-title = 是否安装 Claude skills（可选）？
wizard-claude-skills-confirm = 安装 skills？
wizard-claude-skills-empty-skip = 未选择任何 skill，跳过。
wizard-claude-skills-confirm-write = 确认写入以上 skills？
wizard-claude-skills-builtin-title = 可选内置 skills：
wizard-claude-skills-instruction = 输入要安装的 skill 名称列表，用逗号分隔（例如: explain-code,pr-summary）。
wizard-claude-skills-empty = 留空表示不安装。
