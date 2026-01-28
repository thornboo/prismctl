# Main menu
menu-title = Select an option:
menu-tip-non-tty = Tip: non-TTY (CI/piping) cannot enter interactive mode.

menu-quick-init = Quick Init (Recommended)
menu-configure-claude = Configure Claude Code
menu-configure-codex = Configure Codex
menu-configure-gemini = Configure Gemini CLI
menu-manage-output-style = Manage Output Style (Claude outputStyle)
menu-manage-skills = Manage Skills
menu-view-config = View Current Config (doctor + skills)
menu-language = Language
menu-help = Help
menu-exit = Exit

# Common submenu
menu-back = Back

# Claude submenu
menu-claude-title = Claude Code Menu:
menu-claude-wizard = Guided setup (wizard)
menu-claude-mcp = Configure MCP
menu-claude-output-style = Configure outputStyle
menu-claude-skills = Manage Skills

# Codex submenu
menu-codex-title = Codex Menu:
menu-codex-wizard = Guided setup (wizard)
menu-codex-provider = Configure Provider (provider)
menu-codex-agent = Configure Agent (agent)
menu-codex-agent-scope-title = Select Codex agent scope:
menu-codex-agent-scope-user = user (writes ~/.codex/AGENTS.md)
menu-codex-agent-scope-project = project (writes AGENTS.md in project root)
menu-codex-agent-project-path-prompt = Target project path for project scope (empty = current dir):

# Gemini submenu
menu-gemini-title = Gemini CLI Menu:
menu-gemini-wizard = Guided setup (wizard)
menu-gemini-mcp = Configure MCP
menu-gemini-env = Configure API Key (.env)
menu-gemini-settings = Configure settings.json
menu-gemini-env-scope-title = Select .env scope:
menu-gemini-env-scope-user = user (writes ~/.gemini/.env)
menu-gemini-env-scope-project = project (writes .gemini/.env in project root)
menu-gemini-env-project-path-prompt = Target project path (empty = current dir):
menu-gemini-settings-scope-title = Select settings scope:
menu-gemini-settings-scope-user = user (writes ~/.gemini/settings.json)
menu-gemini-settings-scope-project = project (writes .gemini/settings.json in project root)
menu-gemini-settings-project-path-prompt = Target project path (empty = current dir):

# Gemini MCP submenu
menu-gemini-mcp-title = Gemini MCP Menu:
menu-gemini-mcp-list = List configured MCP (gemini mcp list)
menu-gemini-mcp-builtin = List built-in MCP (prismctl catalog)
menu-gemini-mcp-add = Add built-in MCP
menu-gemini-mcp-remove = Remove MCP (gemini mcp remove)
menu-gemini-mcp-enable = Enable MCP (gemini mcp enable)
menu-gemini-mcp-disable = Disable MCP (gemini mcp disable)
menu-gemini-mcp-pick = Select MCP servers to add (multi-select):
menu-gemini-mcp-empty = No MCP server selected. Cancelled.
menu-gemini-mcp-name-prompt = Enter MCP server name:
menu-gemini-mcp-scope-title = Select MCP scope:
menu-gemini-mcp-scope-project = project (default; writes .gemini/settings.json in project root)
menu-gemini-mcp-scope-user = user (writes ~/.gemini/settings.json)
menu-gemini-mcp-project-path-prompt = Target project path for project scope (empty = current dir):
menu-gemini-mcp-project-path-prompt-opt = Target project path (optional; empty = current dir):

# Claude MCP submenu
menu-claude-mcp-title = Claude MCP Menu:
menu-claude-mcp-list = List configured MCP (claude mcp list)
menu-claude-mcp-builtin = List built-in MCP (prismctl catalog)
menu-claude-mcp-add = Add built-in MCP
menu-claude-mcp-get = Show MCP details (claude mcp get)
menu-claude-mcp-remove = Remove MCP (claude mcp remove)
menu-claude-mcp-pick = Select MCP servers to add (multi-select):
menu-claude-mcp-empty = No MCP server selected. Cancelled.
menu-claude-mcp-name-prompt = Enter MCP server name:
menu-claude-mcp-project-path-prompt = Target project path for project scope (empty = current dir):
menu-claude-mcp-project-path-prompt-opt = Target project path (optional; empty = current dir):
menu-claude-mcp-scope-title = Select MCP scope:
menu-claude-mcp-scope-local = local (default)
menu-claude-mcp-scope-project = project (writes .mcp.json in project root)
menu-claude-mcp-scope-user = user (writes ~/.claude.json)

# Language
language-title = Select language:
language-zh-cn = Chinese (Simplified)
language-en = English
language-back = Back
language-changed = Language updated

# Skills menu
skill-menu-title = Skills Menu:
skill-menu-list = List skills (prismctl skill list)
skill-menu-install = Install skill (prismctl skill install)
skill-menu-create = Create skill (prismctl skill create)
skill-menu-remove = Remove skill (prismctl skill remove)
skill-menu-back = Back

# Output style management
output-style-title = Manage Output Style (Claude outputStyle)
output-style-note = Note: This only sets the outputStyle field in settings.json; the output-style file must be written first via `prismctl init/update --tool claude`.
output-style-prompt-name = Enter outputStyle name (e.g. prismctl-engineer-professional):
