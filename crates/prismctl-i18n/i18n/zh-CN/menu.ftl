# 主菜单
menu-title = 请选择功能：
menu-tip-non-tty = 提示：非 TTY（CI/管道）无法进入交互模式。

menu-quick-init = 快速初始化（推荐新用户）
menu-configure-claude = 配置 Claude Code
menu-configure-codex = 配置 Codex
menu-configure-gemini = 配置 Gemini CLI
menu-manage-output-style = 管理输出样式（Claude outputStyle）
menu-manage-skills = 管理技能（Skills）
menu-view-config = 查看当前配置（doctor + skills）
menu-language = 切换语言
menu-help = 帮助（help）
menu-exit = 退出（quit）

# 子菜单通用
menu-back = 返回

# Claude 子菜单
menu-claude-title = Claude Code 菜单：
menu-claude-wizard = 一键配置（向导）
menu-claude-mcp = 配置 MCP
menu-claude-output-style = 配置输出样式（outputStyle）
menu-claude-skills = 管理技能（Skills）

# Codex 子菜单
menu-codex-title = Codex 菜单：
menu-codex-wizard = 一键配置（向导）
menu-codex-provider = 配置 Provider（provider）
menu-codex-agent = 配置 Agent（agent）
menu-codex-agent-scope-title = 选择 Codex agent scope：
menu-codex-agent-scope-user = user（写入 ~/.codex/AGENTS.md）
menu-codex-agent-scope-project = project（写入项目根目录 AGENTS.md）
menu-codex-agent-project-path-prompt = project scope 目标项目路径（留空=当前目录）：

# Gemini 子菜单
menu-gemini-title = Gemini CLI 菜单：
menu-gemini-wizard = 一键配置（向导）
menu-gemini-mcp = 配置 MCP
menu-gemini-env = 配置 API Key（.env）
menu-gemini-settings = 配置 settings.json
menu-gemini-env-scope-title = 选择 .env scope：
menu-gemini-env-scope-user = user（写入 ~/.gemini/.env）
menu-gemini-env-scope-project = project（写入项目根目录 .gemini/.env）
menu-gemini-env-project-path-prompt = 目标项目路径（留空=当前目录）：
menu-gemini-settings-scope-title = 选择 settings scope：
menu-gemini-settings-scope-user = user（写入 ~/.gemini/settings.json）
menu-gemini-settings-scope-project = project（写入项目根目录 .gemini/settings.json）
menu-gemini-settings-project-path-prompt = 目标项目路径（留空=当前目录）：

# Gemini MCP 子菜单
menu-gemini-mcp-title = Gemini MCP 菜单：
menu-gemini-mcp-list = 查看已配置 MCP（gemini mcp list）
menu-gemini-mcp-builtin = 查看内置 MCP（prismctl 内置库）
menu-gemini-mcp-add = 添加内置 MCP
menu-gemini-mcp-remove = 删除 MCP（gemini mcp remove）
menu-gemini-mcp-enable = 启用 MCP（gemini mcp enable）
menu-gemini-mcp-disable = 禁用 MCP（gemini mcp disable）
menu-gemini-mcp-pick = 选择要添加的 MCP servers（可多选）：
menu-gemini-mcp-empty = 未选择任何 MCP server，已取消。
menu-gemini-mcp-name-prompt = 请输入 MCP server 名称：
menu-gemini-mcp-scope-title = 选择 MCP scope：
menu-gemini-mcp-scope-project = project（默认；写入项目根目录 .gemini/settings.json）
menu-gemini-mcp-scope-user = user（写入 ~/.gemini/settings.json）
menu-gemini-mcp-project-path-prompt = project scope 目标项目路径（留空=当前目录）：
menu-gemini-mcp-project-path-prompt-opt = 目标项目路径（可选；留空=当前目录）：

# Claude MCP 子菜单
menu-claude-mcp-title = Claude MCP 菜单：
menu-claude-mcp-list = 查看已配置 MCP（claude mcp list）
menu-claude-mcp-builtin = 查看内置 MCP（prismctl 内置库）
menu-claude-mcp-add = 添加内置 MCP
menu-claude-mcp-get = 查看 MCP 详情（claude mcp get）
menu-claude-mcp-remove = 删除 MCP（claude mcp remove）
menu-claude-mcp-pick = 选择要添加的 MCP servers（可多选）：
menu-claude-mcp-empty = 未选择任何 MCP server，已取消。
menu-claude-mcp-name-prompt = 请输入 MCP server 名称：
menu-claude-mcp-project-path-prompt = project scope 目标项目路径（留空=当前目录）：
menu-claude-mcp-project-path-prompt-opt = 目标项目路径（可选；留空=当前目录）：
menu-claude-mcp-scope-title = 选择 MCP scope：
menu-claude-mcp-scope-local = local（默认）
menu-claude-mcp-scope-project = project（写入项目根目录 .mcp.json）
menu-claude-mcp-scope-user = user（写入 ~/.claude.json）

# 语言切换
language-title = 选择语言：
language-zh-cn = 中文（简体）
language-en = English
language-back = 返回
language-changed = 已切换语言

# 技能菜单
skill-menu-title = Skills 菜单：
skill-menu-list = 列出 skills（prismctl skill list）
skill-menu-install = 安装 skill（prismctl skill install）
skill-menu-create = 创建 skill（prismctl skill create）
skill-menu-remove = 删除 skill（prismctl skill remove）
skill-menu-back = 返回

# 输出样式管理
output-style-title = 管理输出样式（Claude outputStyle）
output-style-note = 注意：这只会设置 settings.json 的 outputStyle 字段；output-style 文件需要先通过 `prismctl init/update --tool claude` 写入。
output-style-prompt-name = 请输入 outputStyle 名称（例如 prismctl-engineer-professional）：
