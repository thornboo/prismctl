# Non-TTY (script/CI) error
error-non-tty-title = Error: non-interactive environment detected (stdin/stdout is not TTY); cannot run parameter completion / interactive confirmation.
error-non-tty-interactive-title = Error: non-interactive environment detected (stdin/stdout is not TTY); cannot enter Prismctl interactive wizard.
error-non-tty-scope = Scope:
error-non-tty-running =   - Running: { $invocation }
error-non-tty-missing =   - Missing / requires confirmation: { $missing }
error-non-tty-solutions = Solutions:
error-non-tty-solution-tty =   - Retry in an interactive terminal (TTY) to enable completion/confirmation
error-non-tty-interactive-solution-menu =   - Run `prismctl` or `prismctl config` in an interactive terminal to enter the wizard
error-non-tty-interactive-solution-subcmd =   - Or use subcommands (script/CI), e.g. `prismctl init --tool all` / `prismctl update --tool all`
error-non-tty-solution-args =   - Or pass flags explicitly (script/CI), e.g. `--tool codex` / `--name xxx` / `--yes`
error-non-tty-solution-help =   - Full help: `prismctl --help`

error-unknown-command = Unknown command: { $cmd }

# Common CLI parsing errors
error-flag-missing-value = Missing value for { $flag }
error-non-tty-missing-flag = Error: missing required flag { $flag } in non-interactive environment
error-home-not-found = Error: cannot determine HOME. Set PRISMCTL_HOME (or system HOME/USERPROFILE)
error-config-dir-create = Error: failed to create config directory: { $path }: { $error }
error-config-write = Error: failed to write config file: { $path }: { $error }

# legacy/commands.rs (common parsing/usage errors)
error-unsupported-args-with-help = Unsupported arguments for { $cmd }: { $args }
error-missing-subcommand-with-help = Missing subcommand for { $cmd }
error-unknown-subcommand-with-help = Unknown { $cmd } subcommand: { $sub }
error-missing-flag = Missing argument { $flag }
error-current-dir = Failed to get current directory: { $error }
error-timestamp = Failed to get timestamp: { $error }
error-lang-flag-invalid = Missing/invalid --lang <zh-CN|en>
error-unknown-agent = Unknown agent: { $name }
error-tool-value-unsupported = Unsupported --tool value: { $value }
error-lang-value-unsupported = Unsupported --lang value: { $value }
error-install-method-value-unsupported = Unsupported --install-method value: { $value }
error-codex-provider-set-needs-args = codex provider set requires at least one of: --provider/--api-key/--base-url/--model/--wire-api/--default
error-claude-env-set-needs-args = claude env set requires at least one of: --auth-token/--base-url/--model/--haiku-model/--sonnet-model/--opus-model
error-gemini-env-set-needs-args = gemini env set requires at least one of: --api-key/--base-url/--model

# skill name validation (prismctl-core)
error-skill-name-empty = Skill name cannot be empty
error-skill-name-dot-prefix = Skill name cannot start with '.'
error-skill-name-has-separator = Skill name cannot contain path separators
error-skill-name-invalid-chars = Skill name only allows ASCII letters/digits/hyphen(-)/underscore(_)
error-skill-unknown-builtin = Unknown built-in skill: { $name } (available: { $available })

# Quick init/update (aliases: `prismctl i`, `prismctl u`)
quick-provider-presets-title = (Optional) provider presets (Codex):
quick-skip = 0) Skip
quick-prompt = >
quick-api-key-prompt = Enter API Key (leave blank to skip):

# prompted.rs completion prompts
prompt-confirm-continue = Please confirm to continue?
prompt-tool-init = Select tool to init (--tool):
prompt-tool-update = Select tool to update (--tool):
prompt-tool-install = Select tool to install (--tool):
prompt-tool-upgrade = Select tool to upgrade (--tool):
error-tool-flag-invalid = Missing/invalid --tool <codex|claude|gemini|all>
prompt-skill-subcommand = Select skill subcommand:
error-missing-subcommand = Missing subcommand
prompt-skill-name-flag = Enter skill name (--name):
prompt-codex-subcommand = Select codex subcommand:
prompt-codex-agent-flag = Select Codex agent (--name):
prompt-codex-agent-custom = custom (manual input)
prompt-codex-agent-name = Enter agent name:
prompt-project-subcommand = Select project subcommand:

# legacy/commands.rs common output
changeset-preview-title = Planned changes (mode={ $mode }):
changeset-no-changes = No changes.
apply-applied = Applied.
apply-completed = Done.

dry-run-hint-write = Note: this is dry-run. Pass --apply to write.
dry-run-hint-write-skill-files = Note: this is dry-run. Pass --apply to write skill files.
dry-run-hint-create-skill = Note: this is dry-run. Pass --apply to create the skill.
dry-run-hint-remove-skill = Note: this is dry-run. Pass --apply to remove the skill.
dry-run-hint-write-project = Note: this is dry-run. Pass --apply to write project files.
dry-run-hint-install-upgrade = Note: this is dry-run. Pass --apply to perform global install/upgrade.

codex-agent-list-title = Available Codex agents (built-in):
codex-agent-switched = Switched Codex AGENTS.md to: { $name }
codex-provider-default-set = Set Codex default model_provider to: prismctl
codex-provider-preset = Using provider preset: { $provider }
codex-auth-key-hidden = Will write Codex auth.json key: PRISMCTL_CODEX_API_KEY (value hidden)
claude-auth-token-hidden = Will write Claude settings.json env key: ANTHROPIC_AUTH_TOKEN (value hidden)
claude-output-style-set = Set Claude outputStyle to: { $name }
gemini-env-managed-keys = Managing Gemini env block keys: GEMINI_API_KEY, GOOGLE_GEMINI_BASE_URL, GEMINI_MODEL

skill-builtin-title = Built-in skills:
skill-installed-title = Installed skills:
skill-none = (none)

# Dangerous operations (legacy requires --yes; prompted will ask in TTY)
danger-title = [!] Dangerous operation detected!
danger-confirm-need-yes = Confirmation required: pass \"--yes\"

danger-skill-remove-type = Operation: remove skill
danger-skill-remove-scope = Scope: recursively delete ~/.claude/skills/{ $name } (or sandbox path under --home)
danger-skill-remove-risk = Risk: may delete user custom scripts/config; irreversible unless you have backups

danger-install-type = Operation: { $op }
danger-install-scope = Scope: will call brew or npm and modify system-level toolchain/global deps
danger-install-risk = Risk: may override existing versions / affect PATH; failures may require manual fixes
danger-install-op-install = global install
danger-install-op-upgrade = global upgrade

danger-codex-agent-type = Operation: overwrite Codex system prompt file (AGENTS.md)
danger-codex-agent-scope = Scope: changes Codex global system prompt/output style
danger-codex-agent-risk = Risk: may overwrite your custom prompt; a backup is planned if an old file exists
