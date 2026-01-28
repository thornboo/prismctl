# Wizard common
wizard-prompt-name = Enter skill name (built-in or custom):
wizard-prompt-new-name = Enter new skill name:
wizard-prompt-remove-name = Enter skill name to remove:
wizard-confirm-install = Confirm? Type "yes" to continue (any other key to cancel):
wizard-confirm-create = Confirm creation? Type "yes" to continue (any other key to cancel):
wizard-confirm-remove = WARNING: This will delete the skill directory. Confirm? Type "yes" to continue (any other key to cancel):
wizard-confirm-remove-short = WARNING: This will delete the skill directory. Confirm?

# Quick init wizard
wizard-quick-init-title = Quick Init Wizard
wizard-quick-init-desc = This will help you:
wizard-quick-init-desc-init = Write templates (init)
wizard-quick-init-desc-config = Configure tools (optional)
wizard-quick-init-select-tools = Select tools to initialize (multi-select):
wizard-quick-init-confirm-write = Confirm writing the templates?
wizard-quick-init-continue-title = Continue configuring tools (optional)?
wizard-quick-init-continue = Continue?
wizard-quick-init-configure-claude = Configure Claude Code?
wizard-quick-init-configure-codex = Configure Codex?
wizard-quick-init-configure-gemini = Configure Gemini CLI?

# Gemini wizard
wizard-gemini-title = Gemini CLI Setup Wizard
wizard-gemini-prompt-api-key = API key (empty = use env default, "-" = skip):
wizard-gemini-prompt-model = model.name (writes settings.json; empty = do not set):
wizard-no-changes-cancelled = No changes provided. Cancelled.

# Gemini MCP (delegates to gemini CLI)
wizard-gemini-mcp-title = Configure Gemini MCP servers (optional)?
wizard-gemini-mcp-confirm = Configure MCP servers?
wizard-gemini-mcp-select = Select MCP servers to add (multi-select):
wizard-gemini-mcp-empty-skip = No MCP server selected. Skipping.
wizard-gemini-mcp-scope-title = Select MCP scope:
wizard-gemini-mcp-scope-project = project (default; writes .gemini/settings.json in project root)
wizard-gemini-mcp-scope-user = user (writes ~/.gemini/settings.json)
wizard-gemini-mcp-project-path-prompt = Target project path for project scope (empty = current dir):
wizard-gemini-mcp-confirm-write = Confirm writing the selected MCP servers?

# Codex wizard
wizard-codex-title = Codex Setup Wizard
wizard-codex-provider-title = Select API provider:
wizard-codex-provider-custom = custom (base url / model)
wizard-codex-prompt-base-url = Base URL (empty = do not set):
wizard-codex-prompt-wire-api = Wire API (empty = do not set):
wizard-codex-prompt-model = Model (empty = do not set):
wizard-codex-prompt-api-key = API key (empty = use env default, "-" = skip):
wizard-codex-set-default = Set as default provider (recommended)?
wizard-codex-switch-agent-title = Switch Codex agent (optional)?
wizard-codex-switch-agent = Switch agent?
wizard-codex-agent-title = Select Codex agent:
wizard-codex-agent-custom = custom (manual input)
wizard-codex-agent-name = Agent name:
wizard-codex-confirm-overwrite-agents = Overwrite Codex AGENTS.md?

# Claude wizard
wizard-claude-title = Claude Code Setup Wizard
wizard-claude-provider-title = Select API provider:
wizard-claude-provider-custom = custom (base url)
wizard-claude-provider-skip = Skip (do not set base url)
wizard-claude-prompt-base-url = Base URL:
wizard-claude-prompt-auth-token = Auth token (empty = use env default, "-" = skip):
wizard-claude-prompt-model = Model (empty = do not set):
wizard-claude-prompt-haiku-model = Default haiku model (empty = do not set):
wizard-claude-prompt-sonnet-model = Default sonnet model (empty = do not set):
wizard-claude-prompt-opus-model = Default opus model (empty = do not set):

wizard-claude-env-skip = No Claude env changes provided. Skipping.

wizard-claude-output-style-title = Set Claude outputStyle (optional)?
wizard-claude-output-style-confirm = Set outputStyle?
wizard-claude-output-style-select = Select outputStyle:
wizard-claude-output-style-custom = custom (manual input)
wizard-claude-output-style-name = outputStyle name:

wizard-claude-skills-title = Install Claude skills (optional)?
wizard-claude-skills-confirm = Install skills?
wizard-claude-skills-empty-skip = No skills selected. Skipping.
wizard-claude-skills-confirm-write = Confirm writing the selected skills?
wizard-claude-skills-builtin-title = Built-in skills:
wizard-claude-skills-instruction = Enter skill names separated by commas (e.g. explain-code,pr-summary).
wizard-claude-skills-empty = Empty to skip.

# Claude MCP (delegates to claude CLI)
wizard-claude-mcp-title = Configure Claude MCP servers (optional)?
wizard-claude-mcp-confirm = Configure MCP servers?
wizard-claude-mcp-select = Select MCP servers to add (multi-select):
wizard-claude-mcp-empty-skip = No MCP server selected. Skipping.
wizard-claude-mcp-scope-title = Select MCP scope:
wizard-claude-mcp-scope-local = local (default; only for current project; stored in ~/.claude.json under project path)
wizard-claude-mcp-scope-project = project (team-shared; stored in .mcp.json at project root)
wizard-claude-mcp-scope-user = user (cross-project; stored in ~/.claude.json)
wizard-claude-mcp-confirm-write = Confirm writing the selected MCP servers?
wizard-claude-mcp-project-path-prompt = Target project path for project scope (empty = current dir):
