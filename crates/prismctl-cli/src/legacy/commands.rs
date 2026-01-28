use prismctl_core::changeset::{ApplyMode, Change, ChangeSet, RealCommandRunner, RealFileSystem};
use prismctl_core::claude::{self, ClaudeEnvPatch};
use prismctl_core::codex::{self, CodexProviderConfig};
use prismctl_core::installer::{InstallAction, InstallMethod, ToolInstallTarget};
use prismctl_core::managed_block::{extract_managed_block, upsert_managed_block};
use prismctl_core::mcp::{self, ClaudeMcpScope};
use prismctl_core::paths::{PrismctlHome, Tool};
use prismctl_core::providers;
use prismctl_core::skill;
use prismctl_core::templates::{self, TemplateLang};
use prismctl_i18n::{keys, t, tf};
use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

fn localize_skill_error(raw: &str) -> String {
    // prismctl-core intentionally returns stable error codes for input validation,
    // so the CLI can localize without coupling core to i18n.
    match raw {
        "PRISMCTL_SKILL_NAME_EMPTY" => t!(keys::ERROR_SKILL_NAME_EMPTY),
        "PRISMCTL_SKILL_NAME_DOT_PREFIX" => t!(keys::ERROR_SKILL_NAME_DOT_PREFIX),
        "PRISMCTL_SKILL_NAME_HAS_SEPARATOR" => t!(keys::ERROR_SKILL_NAME_HAS_SEPARATOR),
        "PRISMCTL_SKILL_NAME_INVALID_CHARS" => t!(keys::ERROR_SKILL_NAME_INVALID_CHARS),
        _ => {
            if let Some(name) = raw.strip_prefix("PRISMCTL_SKILL_UNKNOWN_BUILTIN:") {
                let available = skill::list_builtin_skills().join(", ");
                return tf!(
                    keys::ERROR_SKILL_UNKNOWN_BUILTIN,
                    "name" => name,
                    "available" => available
                );
            }
            raw.to_string()
        }
    }
}

pub fn help() -> String {
    match prismctl_i18n::current_locale() {
        prismctl_i18n::Locale::En => help_en(),
        prismctl_i18n::Locale::ZhCN => help_zh_cn(),
    }
}

fn help_zh_cn() -> String {
    [
        "Prismctl (early)\n",
        "用法:",
        "  prismctl                              # 交互式菜单（仅 TTY；非 TTY 将报错）",
        "  prismctl config                       # 交互式菜单（仅 TTY；非 TTY 将报错）",
        "  prismctl config <CMD> [ARGS...]        # 镜像入口：等价于 `prismctl <CMD> [ARGS...]`\n",
        "  prismctl doctor [--home <PATH>]",
        "  prismctl d                            # doctor 的短命令（等价于 prismctl doctor）",
        "  prismctl init --tool <codex|claude|gemini|all> [--home <PATH>] [--lang <zh-CN|en>] [--dry-run|--apply]\n",
        "  prismctl i [-t|--tool <TOOL>] [-p|--provider <NAME>] [-k|--api-key <KEY>] [--lang <zh-CN|en>] [--home <PATH>] [--dry-run|--apply] [-s|--silent]",
        "  prismctl update --tool <codex|claude|gemini|all> [--home <PATH>] [--lang <zh-CN|en>] [--dry-run|--apply]\n",
        "  prismctl u [-t|--tool <TOOL>] [--lang <zh-CN|en>] [--home <PATH>] [--dry-run|--apply] [-s|--silent]",
        "  prismctl install --tool <codex|claude|gemini|all> [--install-method <auto|npm|brew>] [--dry-run|--apply] [--yes]",
        "  prismctl upgrade --tool <codex|claude|gemini|all> [--install-method <auto|npm|brew>] [--dry-run|--apply] [--yes]\n",
        "  prismctl skill list [--home <PATH>]",
        "  prismctl skill install --name <VALUE> [--home <PATH>] [--dry-run|--apply]",
        "  prismctl skill create --name <VALUE> [--home <PATH>] [--dry-run|--apply]",
        "  prismctl skill remove --name <VALUE> [--home <PATH>] [--dry-run|--apply] [--yes]\n",
        "  prismctl codex agent list",
        "  prismctl codex agent use --name <VALUE> [--scope <user|project>] [--project-path <PATH>] [--home <PATH>] [--lang <zh-CN|en>] [--dry-run|--apply] [--yes]\n",
        "  prismctl codex provider set [--home <PATH>] [--dry-run|--apply] [--provider <VALUE>] [--api-key <VALUE>] [--base-url <VALUE>] [--model <VALUE>] [--wire-api <VALUE>] [--default]\n",
        "  prismctl claude env set [--home <PATH>] [--dry-run|--apply] [--auth-token <VALUE>] [--base-url <VALUE>] [--model <VALUE>] [--haiku-model <VALUE>] [--sonnet-model <VALUE>] [--opus-model <VALUE>]\n",
        "  prismctl claude output-style use --name <VALUE> [--home <PATH>] [--dry-run|--apply]\n",
        "  prismctl claude mcp list [--project-path <PATH>] [--home <PATH>]\n",
        "  prismctl claude mcp builtin\n",
        "  prismctl claude mcp add --name <VALUE> [--scope <local|project|user>] [--project-path <PATH>] [--home <PATH>] [--dry-run|--apply] [--yes]\n",
        "  prismctl claude mcp get --name <VALUE> [--project-path <PATH>] [--home <PATH>]\n",
        "  prismctl claude mcp remove --name <VALUE> [--project-path <PATH>] [--home <PATH>] [--dry-run|--apply] [--yes]\n",
        "  prismctl gemini env set [--scope <user|project>] [--project-path <PATH>] [--home <PATH>] [--dry-run|--apply] --api-key <VALUE>\n",
        "  prismctl gemini settings set [--scope <user|project>] [--project-path <PATH>] [--home <PATH>] [--dry-run|--apply] --model <VALUE>\n",
        "  prismctl gemini mcp list [--scope <user|project>] [--project-path <PATH>] [--home <PATH>]\n",
        "  prismctl gemini mcp builtin\n",
        "  prismctl gemini mcp add --name <VALUE> [--scope <user|project>] [--project-path <PATH>] [--home <PATH>] [--dry-run|--apply] [--yes]\n",
        "  prismctl gemini mcp remove --name <VALUE> [--scope <user|project>] [--project-path <PATH>] [--home <PATH>] [--dry-run|--apply] [--yes]\n",
        "  prismctl gemini mcp enable --name <VALUE> [--project-path <PATH>] [--home <PATH>] [--dry-run|--apply] [--yes]\n",
        "  prismctl gemini mcp disable --name <VALUE> [--project-path <PATH>] [--home <PATH>] [--dry-run|--apply] [--yes]\n",
        "  prismctl project init [--path <PATH>] [--lang <zh-CN|en>] [--dry-run|--apply]\n",
        "行为:",
        "  - 默认 dry-run：仅打印将执行的变更，不会写入任何文件。",
        "  - 只有传入 --apply 才会真正落盘。",
        "  - --home 或 PRISMCTL_HOME 可将所有读写重定向到沙箱 HOME，避免破坏真实配置。\n",
        "  - --verbose 会在报错时附加调试上下文（cmd/args）。\n",
    ]
    .join("\n")
}

fn help_en() -> String {
    [
        "Prismctl (early)\n",
        "Usage:",
        "  prismctl                              # interactive menu (TTY only; non-TTY will error)",
        "  prismctl config                       # interactive menu (TTY only; non-TTY will error)",
        "  prismctl config <CMD> [ARGS...]        # mirror entry: same as `prismctl <CMD> [ARGS...]`\n",
        "  prismctl doctor [--home <PATH>]",
        "  prismctl d                            # short for doctor\n",
        "  prismctl init --tool <codex|claude|gemini|all> [--home <PATH>] [--lang <zh-CN|en>] [--dry-run|--apply]\n",
        "  prismctl i [-t|--tool <TOOL>] [-p|--provider <NAME>] [-k|--api-key <KEY>] [--lang <zh-CN|en>] [--home <PATH>] [--dry-run|--apply] [-s|--silent]",
        "  prismctl update --tool <codex|claude|gemini|all> [--home <PATH>] [--lang <zh-CN|en>] [--dry-run|--apply]\n",
        "  prismctl u [-t|--tool <TOOL>] [--lang <zh-CN|en>] [--home <PATH>] [--dry-run|--apply] [-s|--silent]",
        "  prismctl install --tool <codex|claude|gemini|all> [--install-method <auto|npm|brew>] [--dry-run|--apply] [--yes]",
        "  prismctl upgrade --tool <codex|claude|gemini|all> [--install-method <auto|npm|brew>] [--dry-run|--apply] [--yes]\n",
        "  prismctl skill list [--home <PATH>]",
        "  prismctl skill install --name <VALUE> [--home <PATH>] [--dry-run|--apply]",
        "  prismctl skill create --name <VALUE> [--home <PATH>] [--dry-run|--apply]",
        "  prismctl skill remove --name <VALUE> [--home <PATH>] [--dry-run|--apply] [--yes]\n",
        "  prismctl codex agent list",
        "  prismctl codex agent use --name <VALUE> [--scope <user|project>] [--project-path <PATH>] [--home <PATH>] [--lang <zh-CN|en>] [--dry-run|--apply] [--yes]\n",
        "  prismctl codex provider set [--home <PATH>] [--dry-run|--apply] [--provider <VALUE>] [--api-key <VALUE>] [--base-url <VALUE>] [--model <VALUE>] [--wire-api <VALUE>] [--default]\n",
        "  prismctl claude env set [--home <PATH>] [--dry-run|--apply] [--auth-token <VALUE>] [--base-url <VALUE>] [--model <VALUE>] [--haiku-model <VALUE>] [--sonnet-model <VALUE>] [--opus-model <VALUE>]\n",
        "  prismctl claude output-style use --name <VALUE> [--home <PATH>] [--dry-run|--apply]\n",
        "  prismctl claude mcp list [--project-path <PATH>] [--home <PATH>]\n",
        "  prismctl claude mcp builtin\n",
        "  prismctl claude mcp add --name <VALUE> [--scope <local|project|user>] [--project-path <PATH>] [--home <PATH>] [--dry-run|--apply] [--yes]\n",
        "  prismctl claude mcp get --name <VALUE> [--project-path <PATH>] [--home <PATH>]\n",
        "  prismctl claude mcp remove --name <VALUE> [--project-path <PATH>] [--home <PATH>] [--dry-run|--apply] [--yes]\n",
        "  prismctl gemini env set [--scope <user|project>] [--project-path <PATH>] [--home <PATH>] [--dry-run|--apply] --api-key <VALUE>\n",
        "  prismctl gemini settings set [--scope <user|project>] [--project-path <PATH>] [--home <PATH>] [--dry-run|--apply] --model <VALUE>\n",
        "  prismctl gemini mcp list [--scope <user|project>] [--project-path <PATH>] [--home <PATH>]\n",
        "  prismctl gemini mcp builtin\n",
        "  prismctl gemini mcp add --name <VALUE> [--scope <user|project>] [--project-path <PATH>] [--home <PATH>] [--dry-run|--apply] [--yes]\n",
        "  prismctl gemini mcp remove --name <VALUE> [--scope <user|project>] [--project-path <PATH>] [--home <PATH>] [--dry-run|--apply] [--yes]\n",
        "  prismctl gemini mcp enable --name <VALUE> [--project-path <PATH>] [--home <PATH>] [--dry-run|--apply] [--yes]\n",
        "  prismctl gemini mcp disable --name <VALUE> [--project-path <PATH>] [--home <PATH>] [--dry-run|--apply] [--yes]\n",
        "  prismctl project init [--path <PATH>] [--lang <zh-CN|en>] [--dry-run|--apply]\n",
        "Behavior:",
        "  - Default is dry-run: prints planned changes without writing files.",
        "  - Only `--apply` writes changes to disk.",
        "  - `--home` or `PRISMCTL_HOME` redirects all I/O to a sandbox home.\n",
        "  - `--verbose` adds cmd/args context on errors.\n",
    ]
    .join("\n")
}

pub fn cmd_doctor(mut args: Vec<String>) -> Result<(), String> {
    let home = parse_home(&mut args)?;
    if !args.is_empty() {
        return Err(err_unsupported_args_with_help("doctor", &args));
    }

    let home = PrismctlHome::discover(home).map_err(crate::errors::usage)?;
    let codex = home.tool_root(Tool::Codex);
    let claude = home.tool_root(Tool::ClaudeCode);
    let gemini = home.tool_root(Tool::GeminiCli);
    let claude_settings = home.claude_settings_path();

    let exists_word = |b: bool| -> &'static str {
        match prismctl_i18n::current_locale() {
            prismctl_i18n::Locale::ZhCN => {
                if b { "存在" } else { "不存在" }
            }
            prismctl_i18n::Locale::En => {
                if b { "yes" } else { "no" }
            }
        }
    };

    match prismctl_i18n::current_locale() {
        prismctl_i18n::Locale::ZhCN => {
            println!("Prismctl HOME: {}", home.home_dir().display());
            println!("Codex root: {}", codex.display());
            println!("Claude root: {}", claude.display());
            println!("Claude settings: {}", claude_settings.display());
            println!("Gemini root: {}", gemini.display());
            println!();

            println!("用户级文件（存在性）：");
        }
        prismctl_i18n::Locale::En => {
            println!("Prismctl HOME: {}", home.home_dir().display());
            println!("Codex root: {}", codex.display());
            println!("Claude root: {}", claude.display());
            println!("Claude settings: {}", claude_settings.display());
            println!("Gemini root: {}", gemini.display());
            println!();

            println!("User-scoped files (existence):");
        }
    }

    let claude_json = home.home_dir().join(".claude.json");
    let codex_config = codex.join("config.toml");
    let codex_auth = codex.join("auth.json");
    let codex_agents = codex.join("AGENTS.md");
    let gemini_env = gemini.join(".env");
    let gemini_settings = gemini.join("settings.json");

    println!(
        "  - ~/.claude/settings.json: {}",
        exists_word(claude_settings.exists())
    );
    println!("  - ~/.claude.json: {}", exists_word(claude_json.exists()));
    println!("  - ~/.codex/config.toml: {}", exists_word(codex_config.exists()));
    println!("  - ~/.codex/auth.json: {}", exists_word(codex_auth.exists()));
    println!("  - ~/.codex/AGENTS.md: {}", exists_word(codex_agents.exists()));
    println!("  - ~/.gemini/.env: {}", exists_word(gemini_env.exists()));
    println!(
        "  - ~/.gemini/settings.json: {}",
        exists_word(gemini_settings.exists())
    );
    println!();

    // Best-effort project context (based on current working directory).
    // Keep it simple: existence checks only (no parsing/merging guesses).
    let cwd = env::current_dir().ok();
    if let Some(project_root) = cwd {
        let mcp_json = project_root.join(".mcp.json");
        let gemini_dir = project_root.join(".gemini");
        let gemini_env = gemini_dir.join(".env");
        let gemini_settings = gemini_dir.join("settings.json");
        let gemini_md = gemini_dir.join("GEMINI.md");
        let codex_agents = project_root.join("AGENTS.md");

        match prismctl_i18n::current_locale() {
            prismctl_i18n::Locale::ZhCN => {
                println!("项目根目录（当前工作目录）: {}", project_root.display());
                println!("项目级文件（存在性）：");
            }
            prismctl_i18n::Locale::En => {
                println!("Project root (cwd): {}", project_root.display());
                println!("Project-scoped files (existence):");
            }
        }

        println!("  - .mcp.json: {}", exists_word(mcp_json.exists()));
        println!("  - AGENTS.md: {}", exists_word(codex_agents.exists()));
        println!("  - .gemini/.env: {}", exists_word(gemini_env.exists()));
        println!(
            "  - .gemini/settings.json: {}",
            exists_word(gemini_settings.exists())
        );
        println!("  - .gemini/GEMINI.md: {}", exists_word(gemini_md.exists()));
    }

    Ok(())
}

pub fn cmd_init(mut args: Vec<String>) -> Result<(), String> {
    let home = parse_home(&mut args)?;
    let mode = parse_apply_mode(&mut args)?;
    let lang = parse_lang(&mut args)?;

    let tool = parse_tool(&mut args)?;
    if !args.is_empty() {
        return Err(err_unsupported_args_with_help("init", &args));
    }

    let home = PrismctlHome::discover(home).map_err(crate::errors::usage)?;
    let gemini_existing =
        fs::read_to_string(home.tool_root(Tool::GeminiCli).join("GEMINI.md")).unwrap_or_default();
    let mut cs = ChangeSet::new();

    match tool {
        ToolSelection::Codex => cs.extend(templates::plan_templates_codex(&home, lang)),
        ToolSelection::Claude => cs.extend(templates::plan_templates_claude(&home, lang)),
        ToolSelection::Gemini => cs.extend(templates::plan_templates_gemini_with_existing(
            &home,
            &gemini_existing,
            lang,
        )),
        ToolSelection::All => {
            cs.extend(templates::plan_templates_codex(&home, lang));
            cs.extend(templates::plan_templates_claude(&home, lang));
            cs.extend(templates::plan_templates_gemini_with_existing(
                &home,
                &gemini_existing,
                lang,
            ));
        }
    }

    if cs.is_empty() {
        println!("{}", t!(keys::CHANGESET_NO_CHANGES));
        return Ok(());
    }

    let title = tf!(keys::CHANGESET_PREVIEW_TITLE, "mode" => format!("{:?}", mode));
    println!("{}", title);
    for c in cs.iter() {
        println!("  - {}", crate::icons::render_change(c));
    }

    if mode == ApplyMode::DryRun {
        println!("\n{}", t!(keys::DRY_RUN_HINT_WRITE));
        return Ok(());
    }

    let fs = RealFileSystem;
    let runner = RealCommandRunner;
    cs.apply(mode, &fs, &runner)?;
    println!("\n{}", t!(keys::APPLY_APPLIED));
    Ok(())
}

pub fn cmd_update(mut args: Vec<String>) -> Result<(), String> {
    let home = parse_home(&mut args)?;
    let mode = parse_apply_mode(&mut args)?;
    let lang = parse_lang(&mut args)?;
    let tool = parse_tool(&mut args)?;

    if !args.is_empty() {
        return Err(err_unsupported_args_with_help("update", &args));
    }

    let home = PrismctlHome::discover(home).map_err(crate::errors::usage)?;
    let gemini_existing =
        fs::read_to_string(home.tool_root(Tool::GeminiCli).join("GEMINI.md")).unwrap_or_default();
    let mut cs = ChangeSet::new();
    match tool {
        ToolSelection::Codex => cs.extend(templates::plan_templates_codex(&home, lang)),
        ToolSelection::Claude => cs.extend(templates::plan_templates_claude(&home, lang)),
        ToolSelection::Gemini => cs.extend(templates::plan_templates_gemini_with_existing(
            &home,
            &gemini_existing,
            lang,
        )),
        ToolSelection::All => {
            cs.extend(templates::plan_templates_codex(&home, lang));
            cs.extend(templates::plan_templates_claude(&home, lang));
            cs.extend(templates::plan_templates_gemini_with_existing(
                &home,
                &gemini_existing,
                lang,
            ));
        }
    }

    if cs.is_empty() {
        println!("{}", t!(keys::CHANGESET_NO_CHANGES));
        return Ok(());
    }

    let title = tf!(keys::CHANGESET_PREVIEW_TITLE, "mode" => format!("{:?}", mode));
    println!("{}", title);
    for c in cs.iter() {
        println!("  - {}", crate::icons::render_change(c));
    }

    if mode == ApplyMode::DryRun {
        println!("\n{}", t!(keys::DRY_RUN_HINT_WRITE));
        return Ok(());
    }

    let fs = RealFileSystem;
    let runner = RealCommandRunner;
    cs.apply(mode, &fs, &runner)?;
    println!("\n{}", t!(keys::APPLY_APPLIED));
    Ok(())
}

pub fn cmd_project(mut args: Vec<String>) -> Result<(), String> {
    let Some(sub) = args.first().cloned() else {
        return Err(err_missing_subcommand_with_help("project"));
    };
    args.remove(0);

    match sub.as_str() {
        "init" => cmd_project_init(args),
        _ => Err(err_unknown_subcommand_with_help("project", &sub)),
    }
}

pub fn cmd_skill(mut args: Vec<String>) -> Result<(), String> {
    let Some(sub) = args.first().cloned() else {
        return Err(err_missing_subcommand_with_help("skill"));
    };
    args.remove(0);

    match sub.as_str() {
        "list" => cmd_skill_list(args),
        "install" => cmd_skill_install(args),
        "create" => cmd_skill_create(args),
        "remove" => cmd_skill_remove(args),
        _ => Err(err_unknown_subcommand_with_help("skill", &sub)),
    }
}

pub fn cmd_install(mut args: Vec<String>) -> Result<(), String> {
    cmd_install_or_upgrade(InstallAction::Install, &mut args)
}

pub fn cmd_upgrade(mut args: Vec<String>) -> Result<(), String> {
    cmd_install_or_upgrade(InstallAction::Upgrade, &mut args)
}

pub fn cmd_codex(mut args: Vec<String>) -> Result<(), String> {
    let Some(sub) = args.first().cloned() else {
        return Err(err_missing_subcommand_with_help("codex"));
    };
    args.remove(0);

    match sub.as_str() {
        "provider" => cmd_codex_provider(args),
        "agent" => cmd_codex_agent(args),
        _ => Err(err_unknown_subcommand_with_help("codex", &sub)),
    }
}

pub fn cmd_claude(mut args: Vec<String>) -> Result<(), String> {
    let Some(sub) = args.first().cloned() else {
        return Err(err_missing_subcommand_with_help("claude"));
    };
    args.remove(0);

    match sub.as_str() {
        "env" => cmd_claude_env(args),
        "output-style" => cmd_claude_output_style(args),
        "mcp" => cmd_claude_mcp(args),
        _ => Err(err_unknown_subcommand_with_help("claude", &sub)),
    }
}

fn cmd_claude_mcp(mut args: Vec<String>) -> Result<(), String> {
    let Some(action) = args.first().cloned() else {
        return Err(err_missing_subcommand_with_help("claude mcp"));
    };
    args.remove(0);

    match action.as_str() {
        "list" => cmd_claude_mcp_list(args),
        "builtin" => cmd_claude_mcp_builtin(args),
        "add" => cmd_claude_mcp_add(args),
        "get" => cmd_claude_mcp_get(args),
        "remove" => cmd_claude_mcp_remove(args),
        _ => Err(err_unknown_subcommand_with_help("claude mcp", &action)),
    }
}

fn parse_project_path_opt(args: &mut Vec<String>) -> Result<Option<PathBuf>, String> {
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--project-path" {
            let v = args.get(i + 1).ok_or_else(|| {
                crate::errors::usage(tf!(
                    keys::ERROR_FLAG_MISSING_VALUE,
                    "flag" => "--project-path"
                ))
            })?;
            let p = PathBuf::from(v);
            args.drain(i..=i + 1);
            return Ok(Some(p));
        }
        i += 1;
    }
    Ok(None)
}

fn cmd_claude_mcp_list(mut args: Vec<String>) -> Result<(), String> {
    let home = parse_home(&mut args)?;
    let project_path = parse_project_path_opt(&mut args)?;
    if !args.is_empty() {
        return Err(err_unsupported_args_with_help("claude mcp list", &args));
    }

    let home = PrismctlHome::discover(home).map_err(crate::errors::usage)?;

    // Listing is read-only; we run it immediately (no ChangeSet).
    let envs = vec![
        ("HOME".to_string(), home.home_dir().to_string_lossy().to_string()),
        ("USERPROFILE".to_string(), home.home_dir().to_string_lossy().to_string()),
    ];
    let mut cmd = std::process::Command::new("claude");
    cmd.args(["mcp", "list"]).envs(envs);
    if let Some(cwd) = project_path {
        cmd.current_dir(cwd);
    }
    let status = cmd.status()
        .map_err(|e| format!("执行命令失败: claude: {}", e))?;
    if !status.success() {
        return Err(format!("claude mcp list 失败（exit={}）", status));
    }
    Ok(())
}

fn cmd_claude_mcp_builtin(args: Vec<String>) -> Result<(), String> {
    if !args.is_empty() {
        return Err(err_unsupported_args_with_help("claude mcp builtin", &args));
    }
    println!("内置 MCP servers（Claude Code）：");
    for s in mcp::list_builtin_claude_mcp_servers() {
        println!("  - {} ({})", s.id, s.transport.as_flag_value());
        match s.transport {
            prismctl_core::mcp::McpTransport::Stdio => {
                let cmd = s.command.unwrap_or("<missing>");
                let args_joined = s.args.join(" ");
                if args_joined.is_empty() {
                    println!("      command: {}", cmd);
                } else {
                    println!("      command: {} {}", cmd, args_joined);
                }
                if !s.env.is_empty() {
                    let keys = s.env.iter().map(|(k, _)| *k).collect::<Vec<_>>().join(", ");
                    println!("      env: {}", keys);
                }
            }
            _ => {
                if let Some(url) = s.url {
                    println!("      url: {}", url);
                }
                if !s.headers.is_empty() {
                    let keys = s
                        .headers
                        .iter()
                        .map(|(k, _)| *k)
                        .collect::<Vec<_>>()
                        .join(", ");
                    println!("      headers: {}", keys);
                }
            }
        }
    }
    Ok(())
}

fn parse_claude_mcp_scope(args: &mut Vec<String>) -> Result<ClaudeMcpScope, String> {
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--scope" {
            let v = args
                .get(i + 1)
                .ok_or_else(|| crate::errors::usage(tf!(keys::ERROR_FLAG_MISSING_VALUE, "flag" => "--scope")))?;
            let scope = match v.as_str() {
                "local" => ClaudeMcpScope::Local,
                "project" => ClaudeMcpScope::Project,
                "user" => ClaudeMcpScope::User,
                _ => {
                    return Err(crate::errors::usage(tf!(
                        keys::ERROR_INVALID_CHOICE,
                        "choice" => v
                    )))
                }
            };
            args.drain(i..=i + 1);
            return Ok(scope);
        }
        i += 1;
    }
    Ok(ClaudeMcpScope::Local)
}

fn cmd_claude_mcp_add(mut args: Vec<String>) -> Result<(), String> {
    let home = parse_home(&mut args)?;
    let mode = parse_apply_mode(&mut args)?;
    let yes = take_flag(&mut args, "--yes");
    let scope = parse_claude_mcp_scope(&mut args)?;
    let project_path = parse_project_path_opt(&mut args)?;

    let name = parse_required_value(&mut args, "--name")?;
    if !args.is_empty() {
        return Err(err_unsupported_args_with_help("claude mcp add", &args));
    }

    let home = PrismctlHome::discover(home).map_err(crate::errors::usage)?;
    let cs =
        mcp::plan_claude_mcp_add(&home, scope, &name, project_path).map_err(crate::errors::usage)?;

    let title = tf!(keys::CHANGESET_PREVIEW_TITLE, "mode" => format!("{:?}", mode));
    println!("{}", title);
    for c in cs.iter() {
        println!("  - {}", crate::icons::render_change(c));
    }

    if mode == ApplyMode::DryRun {
        println!("\n{}", t!(keys::DRY_RUN_HINT_WRITE));
        return Ok(());
    }

    if !yes {
        return Err(crate::errors::usage(danger_claude_mcp_confirmation()));
    }

    let fs = RealFileSystem;
    let runner = RealCommandRunner;
    cs.apply(mode, &fs, &runner)?;
    println!("\n{}", t!(keys::APPLY_APPLIED));
    Ok(())
}

fn cmd_claude_mcp_get(mut args: Vec<String>) -> Result<(), String> {
    let home = parse_home(&mut args)?;
    let project_path = parse_project_path_opt(&mut args)?;
    let name = parse_required_value(&mut args, "--name")?;
    if !args.is_empty() {
        return Err(err_unsupported_args_with_help("claude mcp get", &args));
    }

    let home = PrismctlHome::discover(home).map_err(crate::errors::usage)?;
    let envs = vec![
        ("HOME".to_string(), home.home_dir().to_string_lossy().to_string()),
        ("USERPROFILE".to_string(), home.home_dir().to_string_lossy().to_string()),
    ];
    let mut cmd = std::process::Command::new("claude");
    cmd.args(["mcp", "get", &name]).envs(envs);
    if let Some(cwd) = project_path {
        cmd.current_dir(cwd);
    }
    let status = cmd
        .status()
        .map_err(|e| format!("执行命令失败: claude: {}", e))?;
    if !status.success() {
        return Err(format!("claude mcp get 失败（exit={}）", status));
    }
    Ok(())
}

fn cmd_claude_mcp_remove(mut args: Vec<String>) -> Result<(), String> {
    let home = parse_home(&mut args)?;
    let mode = parse_apply_mode(&mut args)?;
    let yes = take_flag(&mut args, "--yes");
    let project_path = parse_project_path_opt(&mut args)?;
    let name = parse_required_value(&mut args, "--name")?;
    if !args.is_empty() {
        return Err(err_unsupported_args_with_help("claude mcp remove", &args));
    }

    let home = PrismctlHome::discover(home).map_err(crate::errors::usage)?;
    let cs = mcp::plan_claude_mcp_remove(&home, &name, project_path);

    let title = tf!(keys::CHANGESET_PREVIEW_TITLE, "mode" => format!("{:?}", mode));
    println!("{}", title);
    for c in cs.iter() {
        println!("  - {}", crate::icons::render_change(c));
    }

    if mode == ApplyMode::DryRun {
        println!("\n{}", t!(keys::DRY_RUN_HINT_WRITE));
        return Ok(());
    }

    if !yes {
        return Err(crate::errors::usage(danger_claude_mcp_confirmation()));
    }

    let fs = RealFileSystem;
    let runner = RealCommandRunner;
    cs.apply(mode, &fs, &runner)?;
    println!("\n{}", t!(keys::APPLY_APPLIED));
    Ok(())
}

pub(crate) fn danger_claude_mcp_confirmation() -> String {
    [
        t!(keys::DANGER_TITLE),
        t!(keys::DANGER_CLAUDE_MCP_TYPE),
        t!(keys::DANGER_CLAUDE_MCP_SCOPE),
        t!(keys::DANGER_CLAUDE_MCP_RISK),
        String::new(),
        t!(keys::DANGER_CONFIRM_NEED_YES),
    ]
    .join("\n")
}

pub fn cmd_gemini(mut args: Vec<String>) -> Result<(), String> {
    let Some(sub) = args.first().cloned() else {
        return Err(err_missing_subcommand_with_help("gemini"));
    };
    args.remove(0);

    match sub.as_str() {
        "env" => cmd_gemini_env(args),
        "settings" => cmd_gemini_settings(args),
        "mcp" => cmd_gemini_mcp(args),
        _ => Err(err_unknown_subcommand_with_help("gemini", &sub)),
    }
}

// ---- parsing helpers ----

fn err_unsupported_args_with_help(cmd: &str, args: &Vec<String>) -> String {
    crate::errors::usage(format!(
        "{}\n\n{}",
        tf!(
            keys::ERROR_UNSUPPORTED_ARGS_WITH_HELP,
            "cmd" => cmd,
            "args" => format!("{:?}", args)
        ),
        help()
    ))
}

fn err_missing_subcommand_with_help(cmd: &str) -> String {
    crate::errors::usage(format!(
        "{}\n\n{}",
        tf!(keys::ERROR_MISSING_SUBCOMMAND_WITH_HELP, "cmd" => cmd),
        help()
    ))
}

fn err_unknown_subcommand_with_help(cmd: &str, sub: &str) -> String {
    crate::errors::usage(format!(
        "{}\n\n{}",
        tf!(
            keys::ERROR_UNKNOWN_SUBCOMMAND_WITH_HELP,
            "cmd" => cmd,
            "sub" => sub
        ),
        help()
    ))
}

fn parse_home(args: &mut Vec<String>) -> Result<Option<PathBuf>, String> {
    let mut home: Option<PathBuf> = None;
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--home" {
            let value = args
                .get(i + 1)
                .ok_or_else(|| {
                    crate::errors::usage(tf!(keys::ERROR_FLAG_MISSING_VALUE, "flag" => "--home"))
                })?
                .to_string();
            home = Some(PathBuf::from(value));
            args.drain(i..=i + 1);
            continue;
        }
        i += 1;
    }
    Ok(home)
}

fn parse_apply_mode(args: &mut Vec<String>) -> Result<ApplyMode, String> {
    let mut mode = ApplyMode::DryRun;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--dry-run" => {
                mode = ApplyMode::DryRun;
                args.remove(i);
            }
            "--apply" => {
                mode = ApplyMode::Apply;
                args.remove(i);
            }
            _ => i += 1,
        }
    }
    Ok(mode)
}

fn parse_lang(args: &mut Vec<String>) -> Result<TemplateLang, String> {
    let mut lang = TemplateLang::ZhCn;
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--lang" {
            let v = args
                .get(i + 1)
                .ok_or_else(|| {
                    crate::errors::usage(tf!(keys::ERROR_FLAG_MISSING_VALUE, "flag" => "--lang"))
                })?
                .as_str();
            lang = match v {
                "zh-CN" | "zh-cn" => TemplateLang::ZhCn,
                "en" | "en-US" | "en-us" => TemplateLang::En,
                _ => {
                    return Err(crate::errors::usage(tf!(
                        keys::ERROR_LANG_VALUE_UNSUPPORTED,
                        "value" => v
                    )))
                }
            };
            args.drain(i..=i + 1);
            continue;
        }
        i += 1;
    }
    Ok(lang)
}

#[derive(Debug, Clone, Copy)]
enum ToolSelection {
    Codex,
    Claude,
    Gemini,
    All,
}

fn parse_tool(args: &mut Vec<String>) -> Result<ToolSelection, String> {
    let mut tool: Option<ToolSelection> = None;
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--tool" {
            let value = args
                .get(i + 1)
                .ok_or_else(|| {
                    crate::errors::usage(tf!(keys::ERROR_FLAG_MISSING_VALUE, "flag" => "--tool"))
                })?
                .as_str();
            tool = Some(match value {
                "codex" => ToolSelection::Codex,
                "claude" | "claude-code" => ToolSelection::Claude,
                "gemini" | "gemini-cli" => ToolSelection::Gemini,
                "all" => ToolSelection::All,
                _ => {
                    return Err(crate::errors::usage(tf!(
                        keys::ERROR_TOOL_VALUE_UNSUPPORTED,
                        "value" => value
                    )));
                }
            });
            args.drain(i..=i + 1);
            continue;
        }
        i += 1;
    }

    tool.ok_or_else(|| crate::errors::usage(t!(keys::ERROR_TOOL_FLAG_INVALID)))
}

fn take_flag(args: &mut Vec<String>, flag: &str) -> bool {
    let mut i = 0;
    while i < args.len() {
        if args[i] == flag {
            args.remove(i);
            return true;
        }
        i += 1;
    }
    false
}

fn parse_install_method(args: &mut Vec<String>) -> Result<InstallMethod, String> {
    let mut method = InstallMethod::Auto;
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--install-method" {
            let v = args
                .get(i + 1)
                .ok_or_else(|| {
                    crate::errors::usage(tf!(
                        keys::ERROR_FLAG_MISSING_VALUE,
                        "flag" => "--install-method"
                    ))
                })?
                .as_str();
            method = match v {
                "auto" => InstallMethod::Auto,
                "npm" => InstallMethod::Npm,
                "brew" => InstallMethod::Brew,
                _ => {
                    return Err(crate::errors::usage(tf!(
                        keys::ERROR_INSTALL_METHOD_VALUE_UNSUPPORTED,
                        "value" => v
                    )));
                }
            };
            args.drain(i..=i + 1);
            continue;
        }
        i += 1;
    }
    Ok(method)
}

fn parse_project_path(args: &mut Vec<String>) -> Result<PathBuf, String> {
    let mut path: Option<PathBuf> = None;
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--path" {
            let v = args
                .get(i + 1)
                .ok_or_else(|| {
                    crate::errors::usage(tf!(keys::ERROR_FLAG_MISSING_VALUE, "flag" => "--path"))
                })?
                .to_string();
            path = Some(PathBuf::from(v));
            args.drain(i..=i + 1);
            continue;
        }
        i += 1;
    }

    let p = match path {
        Some(p) => p,
        None => env::current_dir().map_err(|e| tf!(keys::ERROR_CURRENT_DIR, "error" => e))?,
    };
    Ok(normalize_path(&p))
}

fn normalize_path(path: &Path) -> PathBuf {
    // Best-effort normalization; do not resolve symlinks.
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join(path)
    }
}

fn parse_required_value(args: &mut Vec<String>, flag: &str) -> Result<String, String> {
    let mut value: Option<String> = None;
    let mut i = 0;
    while i < args.len() {
        if args[i] == flag {
            value = Some(
                args.get(i + 1)
                    .ok_or_else(|| {
                        crate::errors::usage(tf!(keys::ERROR_FLAG_MISSING_VALUE, "flag" => flag))
                    })?
                    .to_string(),
            );
            args.drain(i..=i + 1);
            continue;
        }
        i += 1;
    }
    value.ok_or_else(|| crate::errors::usage(tf!(keys::ERROR_MISSING_FLAG, "flag" => flag)))
}

fn quote_path_display(path: &Path) -> String {
    let s = path.display().to_string();
    let escaped = s.replace('\\', "\\\\").replace('"', "\\\"");
    format!("\"{}\"", escaped)
}

// ---- skill ----

fn cmd_skill_list(mut args: Vec<String>) -> Result<(), String> {
    let home = parse_home(&mut args)?;
    if !args.is_empty() {
        return Err(err_unsupported_args_with_help("skill list", &args));
    }
    let home = PrismctlHome::discover(home).map_err(crate::errors::usage)?;

    let installed = skill::list_installed_skills(&home);
    println!("{}", t!(keys::SKILL_BUILTIN_TITLE));
    for s in skill::list_builtin_skills() {
        let is_installed = installed.iter().any(|i| i.name == s);
        if is_installed {
            println!("  - {} (installed)", s);
        } else {
            println!("  - {}", s);
        }
    }

    println!("\n{}", t!(keys::SKILL_INSTALLED_TITLE));
    if installed.is_empty() {
        println!("  {}", t!(keys::SKILL_NONE));
        return Ok(());
    }
    for s in installed {
        if s.description.is_empty() {
            println!("  - {} -> {}", s.name, quote_path_display(&s.path));
        } else {
            println!(
                "  - {}: {} -> {}",
                s.name,
                s.description,
                quote_path_display(&s.path)
            );
        }
    }
    Ok(())
}

fn cmd_skill_install(mut args: Vec<String>) -> Result<(), String> {
    let home = parse_home(&mut args)?;
    let mode = parse_apply_mode(&mut args)?;
    let name = parse_required_value(&mut args, "--name")?;
    if !args.is_empty() {
        return Err(err_unsupported_args_with_help("skill install", &args));
    }

    let home = PrismctlHome::discover(home).map_err(crate::errors::usage)?;
    let cs = skill::plan_install_skill(&home, &name)
        .map_err(|e| crate::errors::usage(localize_skill_error(&e)))?;

    let title = tf!(keys::CHANGESET_PREVIEW_TITLE, "mode" => format!("{:?}", mode));
    println!("{}", title);
    for c in cs.iter() {
        println!("  - {}", crate::icons::render_change(c));
    }
    if mode == ApplyMode::DryRun {
        println!("\n{}", t!(keys::DRY_RUN_HINT_WRITE_SKILL_FILES));
        return Ok(());
    }

    let fs = RealFileSystem;
    let runner = RealCommandRunner;
    cs.apply(mode, &fs, &runner)?;
    println!("\n{}", t!(keys::APPLY_APPLIED));
    Ok(())
}

fn cmd_skill_create(mut args: Vec<String>) -> Result<(), String> {
    let home = parse_home(&mut args)?;
    let mode = parse_apply_mode(&mut args)?;
    let name = parse_required_value(&mut args, "--name")?;
    if !args.is_empty() {
        return Err(err_unsupported_args_with_help("skill create", &args));
    }

    let home = PrismctlHome::discover(home).map_err(crate::errors::usage)?;
    skill::validate_skill_name(&name)
        .map_err(|e| crate::errors::usage(localize_skill_error(&e)))?;
    let cs = skill::plan_create_skill(&home, &name);

    let title = tf!(keys::CHANGESET_PREVIEW_TITLE, "mode" => format!("{:?}", mode));
    println!("{}", title);
    for c in cs.iter() {
        println!("  - {}", crate::icons::render_change(c));
    }
    if mode == ApplyMode::DryRun {
        println!("\n{}", t!(keys::DRY_RUN_HINT_CREATE_SKILL));
        return Ok(());
    }

    let fs = RealFileSystem;
    let runner = RealCommandRunner;
    cs.apply(mode, &fs, &runner)?;
    println!("\n{}", t!(keys::APPLY_APPLIED));
    Ok(())
}

fn cmd_skill_remove(mut args: Vec<String>) -> Result<(), String> {
    let home = parse_home(&mut args)?;
    let mode = parse_apply_mode(&mut args)?;
    let yes = take_flag(&mut args, "--yes");
    let name = parse_required_value(&mut args, "--name")?;
    if !args.is_empty() {
        return Err(err_unsupported_args_with_help("skill remove", &args));
    }

    let home = PrismctlHome::discover(home).map_err(crate::errors::usage)?;
    let cs = skill::plan_remove_skill(&home, &name)
        .map_err(|e| crate::errors::usage(localize_skill_error(&e)))?;

    let title = tf!(keys::CHANGESET_PREVIEW_TITLE, "mode" => format!("{:?}", mode));
    println!("{}", title);
    for c in cs.iter() {
        println!("  - {}", crate::icons::render_change(c));
    }
    if mode == ApplyMode::DryRun {
        println!("\n{}", t!(keys::DRY_RUN_HINT_REMOVE_SKILL));
        return Ok(());
    }
    if !yes {
        return Err(crate::errors::usage(danger_skill_remove_confirmation(
            &name,
        )));
    }

    let fs = RealFileSystem;
    let runner = RealCommandRunner;
    cs.apply(mode, &fs, &runner)?;
    println!("\n{}", t!(keys::APPLY_COMPLETED));
    Ok(())
}

pub(crate) fn danger_skill_remove_confirmation(name: &str) -> String {
    [
        t!(keys::DANGER_TITLE),
        t!(keys::DANGER_SKILL_REMOVE_TYPE),
        tf!(keys::DANGER_SKILL_REMOVE_SCOPE, "name" => name),
        t!(keys::DANGER_SKILL_REMOVE_RISK),
        String::new(),
        t!(keys::DANGER_CONFIRM_NEED_YES),
    ]
    .join("\n")
}

// ---- project ----

fn cmd_project_init(mut args: Vec<String>) -> Result<(), String> {
    let mode = parse_apply_mode(&mut args)?;
    let lang = parse_lang(&mut args)?;
    let project_root = parse_project_path(&mut args)?;
    if !args.is_empty() {
        return Err(err_unsupported_args_with_help("project init", &args));
    }

    let gemini_md_path = project_root.join(".gemini").join("GEMINI.md");
    let existing = fs::read_to_string(&gemini_md_path).unwrap_or_default();

    let cs = prismctl_core::project::plan_project_init(&project_root, lang, &existing);

    let title = tf!(keys::CHANGESET_PREVIEW_TITLE, "mode" => format!("{:?}", mode));
    println!("{}", title);
    for c in cs.iter() {
        println!("  - {}", crate::icons::render_change(c));
    }

    if mode == ApplyMode::DryRun {
        println!("\n{}", t!(keys::DRY_RUN_HINT_WRITE_PROJECT));
        return Ok(());
    }

    let fs = RealFileSystem;
    let runner = RealCommandRunner;
    cs.apply(mode, &fs, &runner)?;
    println!("\n{}", t!(keys::APPLY_APPLIED));
    Ok(())
}

// ---- install/upgrade ----

fn cmd_install_or_upgrade(action: InstallAction, args: &mut Vec<String>) -> Result<(), String> {
    let mode = parse_apply_mode(args)?;
    let method = parse_install_method(args)?;
    let yes = take_flag(args, "--yes");

    let tool = parse_tool(args)?;
    if !args.is_empty() {
        let op = match action {
            InstallAction::Install => "install",
            InstallAction::Upgrade => "upgrade",
        };
        return Err(err_unsupported_args_with_help(op, args));
    }

    let targets = match tool {
        ToolSelection::Codex => vec![ToolInstallTarget::Codex],
        ToolSelection::Claude => vec![ToolInstallTarget::ClaudeCode],
        ToolSelection::Gemini => vec![ToolInstallTarget::GeminiCli],
        ToolSelection::All => vec![
            ToolInstallTarget::Codex,
            ToolInstallTarget::ClaudeCode,
            ToolInstallTarget::GeminiCli,
        ],
    };

    let mut cs = ChangeSet::new();
    for t in targets {
        cs.extend(prismctl_core::installer::plan_install(t, method, action));
    }

    let title = tf!(keys::CHANGESET_PREVIEW_TITLE, "mode" => format!("{:?}", mode));
    println!("{}", title);
    for c in cs.iter() {
        println!("  - {}", crate::icons::render_change(c));
    }

    if mode == ApplyMode::DryRun {
        println!("\n{}", t!(keys::DRY_RUN_HINT_INSTALL_UPGRADE));
        return Ok(());
    }

    if !yes {
        return Err(crate::errors::usage(danger_install_confirmation(action)));
    }

    let fs = RealFileSystem;
    let runner = RealCommandRunner;
    cs.apply(mode, &fs, &runner)?;
    println!("\n{}", t!(keys::APPLY_COMPLETED));
    Ok(())
}

pub(crate) fn danger_install_confirmation(action: InstallAction) -> String {
    let op = match action {
        InstallAction::Install => t!(keys::DANGER_INSTALL_OP_INSTALL),
        InstallAction::Upgrade => t!(keys::DANGER_INSTALL_OP_UPGRADE),
    };
    [
        t!(keys::DANGER_TITLE),
        tf!(keys::DANGER_INSTALL_TYPE, "op" => op),
        t!(keys::DANGER_INSTALL_SCOPE),
        t!(keys::DANGER_INSTALL_RISK),
        String::new(),
        t!(keys::DANGER_CONFIRM_NEED_YES),
    ]
    .join("\n")
}

// ---- codex ----

fn cmd_codex_provider(mut args: Vec<String>) -> Result<(), String> {
    let Some(action) = args.first().cloned() else {
        return Err(err_missing_subcommand_with_help("codex provider"));
    };
    args.remove(0);

    match action.as_str() {
        "set" => cmd_codex_provider_set(args),
        _ => Err(err_unknown_subcommand_with_help("codex provider", &action)),
    }
}

fn cmd_codex_agent(mut args: Vec<String>) -> Result<(), String> {
    let Some(action) = args.first().cloned() else {
        return Err(err_missing_subcommand_with_help("codex agent"));
    };
    args.remove(0);

    match action.as_str() {
        "list" => cmd_codex_agent_list(args),
        "use" => cmd_codex_agent_use(args),
        _ => Err(err_unknown_subcommand_with_help("codex agent", &action)),
    }
}

fn cmd_codex_agent_list(args: Vec<String>) -> Result<(), String> {
    if !args.is_empty() {
        return Err(err_unsupported_args_with_help("codex agent list", &args));
    }

    println!("{}", t!(keys::CODEX_AGENT_LIST_TITLE));
    for name in templates::codex_agent_names() {
        println!("  - {}", name);
    }
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CodexAgentScope {
    User,
    Project,
}

fn parse_codex_agent_scope(args: &mut Vec<String>) -> Result<CodexAgentScope, String> {
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--scope" {
            let v = args
                .get(i + 1)
                .ok_or_else(|| crate::errors::usage(tf!(keys::ERROR_FLAG_MISSING_VALUE, "flag" => "--scope")))?;
            let scope = match v.as_str() {
                "user" => CodexAgentScope::User,
                "project" => CodexAgentScope::Project,
                _ => {
                    return Err(crate::errors::usage(tf!(
                        keys::ERROR_INVALID_CHOICE,
                        "choice" => v
                    )))
                }
            };
            args.drain(i..=i + 1);
            return Ok(scope);
        }
        i += 1;
    }
    Ok(CodexAgentScope::User)
}

fn cmd_codex_agent_use(mut args: Vec<String>) -> Result<(), String> {
    let home = parse_home(&mut args)?;
    let mode = parse_apply_mode(&mut args)?;
    let lang = parse_lang(&mut args)?;
    let yes = take_flag(&mut args, "--yes");
    let scope = parse_codex_agent_scope(&mut args)?;
    let project_path = parse_project_path_opt(&mut args)?;

    let name = parse_required_value(&mut args, "--name")?;
    if !args.is_empty() {
        return Err(err_unsupported_args_with_help("codex agent use", &args));
    }

    let tpl = templates::codex_agent_template(&name, lang)
        .ok_or_else(|| crate::errors::usage(tf!(keys::ERROR_UNKNOWN_AGENT, "name" => &name)))?;

    let home = PrismctlHome::discover(home).map_err(crate::errors::usage)?;

    let (agents_path, backup_base_dir) = match scope {
        CodexAgentScope::User => {
            let codex_root = home.tool_root(Tool::Codex);
            (codex_root.join("AGENTS.md"), codex_root.join("backup").join("prismctl"))
        }
        CodexAgentScope::Project => {
            let root = match project_path {
                Some(p) => normalize_path(&p),
                None => env::current_dir().map_err(|e| tf!(keys::ERROR_CURRENT_DIR, "error" => e))?,
            };
            (
                root.join("AGENTS.md"),
                root.join(".prismctl").join("backup").join("prismctl"),
            )
        }
    };

    let existing = fs::read_to_string(&agents_path).unwrap_or_default();

    let mut cs = ChangeSet::new();
    if let Some(parent) = agents_path.parent() {
        cs.push(Change::CreateDirAll {
            path: parent.to_path_buf(),
        });
    }

    if !existing.trim().is_empty() {
        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| tf!(keys::ERROR_TIMESTAMP, "error" => e))?
            .as_secs();
        let backup_path = backup_base_dir.join(ts.to_string()).join("AGENTS.md");
        if let Some(parent) = backup_path.parent() {
            cs.push(Change::CreateDirAll {
                path: parent.to_path_buf(),
            });
        }
        cs.push(Change::WriteFile {
            path: backup_path,
            bytes: existing.as_bytes().to_vec(),
            overwrite: false,
        });
    }

    cs.push(Change::WriteFile {
        path: agents_path,
        bytes: tpl.as_bytes().to_vec(),
        overwrite: true,
    });

    let title = tf!(keys::CHANGESET_PREVIEW_TITLE, "mode" => format!("{:?}", mode));
    println!("{}", title);
    for c in cs.iter() {
        println!("  - {}", crate::icons::render_change(c));
    }
    println!("{}", tf!(keys::CODEX_AGENT_SWITCHED, "name" => name));

    if mode == ApplyMode::DryRun {
        println!("\n{}", t!(keys::DRY_RUN_HINT_WRITE));
        return Ok(());
    }

    if !yes {
        return Err(crate::errors::usage(danger_codex_agent_use_confirmation()));
    }

    let fs = RealFileSystem;
    let runner = RealCommandRunner;
    cs.apply(mode, &fs, &runner)?;
    println!("\n{}", t!(keys::APPLY_APPLIED));
    Ok(())
}

pub(crate) fn danger_codex_agent_use_confirmation() -> String {
    [
        t!(keys::DANGER_TITLE),
        t!(keys::DANGER_CODEX_AGENT_TYPE),
        t!(keys::DANGER_CODEX_AGENT_SCOPE),
        t!(keys::DANGER_CODEX_AGENT_RISK),
        String::new(),
        t!(keys::DANGER_CONFIRM_NEED_YES),
    ]
    .join("\n")
}

fn cmd_codex_provider_set(mut args: Vec<String>) -> Result<(), String> {
    let home = parse_home(&mut args)?;
    let mode = parse_apply_mode(&mut args)?;

    let mut provider: Option<providers::Provider> = None;
    let mut api_key: Option<String> = None;
    let mut base_url: Option<String> = None;
    let mut model: Option<String> = None;
    let mut wire_api: Option<String> = None;
    let mut set_default = false;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--provider" => {
                let v = args
                    .get(i + 1)
                    .ok_or_else(|| {
                        crate::errors::usage(tf!(
                            keys::ERROR_FLAG_MISSING_VALUE,
                            "flag" => "--provider"
                        ))
                    })?
                    .to_string();
                provider = Some(providers::parse_provider_id(&v).map_err(crate::errors::usage)?);
                args.drain(i..=i + 1);
            }
            "--api-key" => {
                api_key = Some(
                    args.get(i + 1)
                        .ok_or_else(|| {
                            crate::errors::usage(tf!(
                                keys::ERROR_FLAG_MISSING_VALUE,
                                "flag" => "--api-key"
                            ))
                        })?
                        .to_string(),
                );
                args.drain(i..=i + 1);
            }
            "--base-url" => {
                base_url = Some(
                    args.get(i + 1)
                        .ok_or_else(|| {
                            crate::errors::usage(tf!(
                                keys::ERROR_FLAG_MISSING_VALUE,
                                "flag" => "--base-url"
                            ))
                        })?
                        .to_string(),
                );
                args.drain(i..=i + 1);
            }
            "--model" => {
                model = Some(
                    args.get(i + 1)
                        .ok_or_else(|| {
                            crate::errors::usage(tf!(
                                keys::ERROR_FLAG_MISSING_VALUE,
                                "flag" => "--model"
                            ))
                        })?
                        .to_string(),
                );
                args.drain(i..=i + 1);
            }
            "--wire-api" => {
                wire_api = Some(
                    args.get(i + 1)
                        .ok_or_else(|| {
                            crate::errors::usage(tf!(
                                keys::ERROR_FLAG_MISSING_VALUE,
                                "flag" => "--wire-api"
                            ))
                        })?
                        .to_string(),
                );
                args.drain(i..=i + 1);
            }
            "--default" => {
                set_default = true;
                args.remove(i);
            }
            _ => i += 1,
        }
    }

    if api_key.is_none()
        && provider.is_none()
        && base_url.is_none()
        && model.is_none()
        && wire_api.is_none()
        && !set_default
    {
        return Err(crate::errors::usage(t!(
            keys::ERROR_CODEX_PROVIDER_SET_NEEDS_ARGS
        )));
    }
    if !args.is_empty() {
        return Err(err_unsupported_args_with_help("codex provider set", &args));
    }

    let home = PrismctlHome::discover(home).map_err(crate::errors::usage)?;
    let codex_root = home.tool_root(Tool::Codex);
    let config_path = codex_root.join("config.toml");
    let auth_path = codex_root.join("auth.json");

    let existing_toml = fs::read_to_string(&config_path).unwrap_or_default();
    let existing_auth = fs::read_to_string(&auth_path).unwrap_or_default();

    // Default values keep the command usable even when only a subset of args is provided.
    let provider_id = "prismctl".to_string();
    let temp_env_key = "PRISMCTL_CODEX_API_KEY".to_string();

    let resolved = providers::resolve_codex_provider(provider, base_url, wire_api, model);

    let display_name = match provider {
        Some(p) => format!("Prismctl ({})", p.id()),
        None => "Prismctl".to_string(),
    };

    let cfg = CodexProviderConfig {
        provider_id,
        display_name,
        base_url: resolved.base_url,
        wire_api: resolved.wire_api,
        temp_env_key: temp_env_key.clone(),
        requires_openai_auth: false,
        model: resolved.model,
    };

    let next_toml = codex::upsert_codex_provider_in_config_toml(&existing_toml, &cfg, set_default);

    let mut cs = ChangeSet::new();
    cs.push(Change::CreateDirAll {
        path: codex_root.clone(),
    });
    cs.push(Change::WriteFile {
        path: config_path,
        bytes: next_toml.into_bytes(),
        overwrite: true,
    });

    let has_api_key = api_key.is_some();
    if let Some(key_value) = api_key {
        let next_auth =
            codex::upsert_codex_api_key_in_auth_json(&existing_auth, &temp_env_key, &key_value)?;
        cs.push(Change::WriteFile {
            path: auth_path,
            bytes: next_auth.into_bytes(),
            overwrite: true,
        });
    }

    let title = tf!(keys::CHANGESET_PREVIEW_TITLE, "mode" => format!("{:?}", mode));
    println!("{}", title);
    for c in cs.iter() {
        println!("  - {}", crate::icons::render_change(c));
    }
    if set_default {
        println!("{}", t!(keys::CODEX_PROVIDER_DEFAULT_SET));
    }
    if let Some(p) = provider {
        println!("{}", tf!(keys::CODEX_PROVIDER_PRESET, "provider" => p.id()));
    }
    if has_api_key {
        println!("{}", t!(keys::CODEX_AUTH_KEY_HIDDEN));
    }

    if mode == ApplyMode::DryRun {
        println!("\n{}", t!(keys::DRY_RUN_HINT_WRITE));
        return Ok(());
    }

    let fs = RealFileSystem;
    let runner = RealCommandRunner;
    cs.apply(mode, &fs, &runner)?;
    println!("\n{}", t!(keys::APPLY_APPLIED));
    Ok(())
}

// ---- claude ----

fn cmd_claude_env(mut args: Vec<String>) -> Result<(), String> {
    let Some(action) = args.first().cloned() else {
        return Err(err_missing_subcommand_with_help("claude env"));
    };
    args.remove(0);

    match action.as_str() {
        "set" => cmd_claude_env_set(args),
        _ => Err(err_unknown_subcommand_with_help("claude env", &action)),
    }
}

fn cmd_claude_env_set(mut args: Vec<String>) -> Result<(), String> {
    let home = parse_home(&mut args)?;
    let mode = parse_apply_mode(&mut args)?;

    let mut auth_token: Option<String> = None;
    let mut base_url: Option<String> = None;
    let mut model: Option<String> = None;
    let mut haiku_model: Option<String> = None;
    let mut sonnet_model: Option<String> = None;
    let mut opus_model: Option<String> = None;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--auth-token" => {
                auth_token = Some(
                    args.get(i + 1)
                        .ok_or_else(|| {
                            crate::errors::usage(tf!(
                                keys::ERROR_FLAG_MISSING_VALUE,
                                "flag" => "--auth-token"
                            ))
                        })?
                        .to_string(),
                );
                args.drain(i..=i + 1);
            }
            "--base-url" => {
                base_url = Some(
                    args.get(i + 1)
                        .ok_or_else(|| {
                            crate::errors::usage(tf!(
                                keys::ERROR_FLAG_MISSING_VALUE,
                                "flag" => "--base-url"
                            ))
                        })?
                        .to_string(),
                );
                args.drain(i..=i + 1);
            }
            "--model" => {
                model = Some(
                    args.get(i + 1)
                        .ok_or_else(|| {
                            crate::errors::usage(
                                tf!(keys::ERROR_FLAG_MISSING_VALUE, "flag" => "--model"),
                            )
                        })?
                        .to_string(),
                );
                args.drain(i..=i + 1);
            }
            "--haiku-model" => {
                haiku_model = Some(
                    args.get(i + 1)
                        .ok_or_else(|| {
                            crate::errors::usage(tf!(
                                keys::ERROR_FLAG_MISSING_VALUE,
                                "flag" => "--haiku-model"
                            ))
                        })?
                        .to_string(),
                );
                args.drain(i..=i + 1);
            }
            "--sonnet-model" => {
                sonnet_model = Some(
                    args.get(i + 1)
                        .ok_or_else(|| {
                            crate::errors::usage(tf!(
                                keys::ERROR_FLAG_MISSING_VALUE,
                                "flag" => "--sonnet-model"
                            ))
                        })?
                        .to_string(),
                );
                args.drain(i..=i + 1);
            }
            "--opus-model" => {
                opus_model = Some(
                    args.get(i + 1)
                        .ok_or_else(|| {
                            crate::errors::usage(tf!(
                                keys::ERROR_FLAG_MISSING_VALUE,
                                "flag" => "--opus-model"
                            ))
                        })?
                        .to_string(),
                );
                args.drain(i..=i + 1);
            }
            _ => i += 1,
        }
    }

    if auth_token.is_none()
        && base_url.is_none()
        && model.is_none()
        && haiku_model.is_none()
        && sonnet_model.is_none()
        && opus_model.is_none()
    {
        return Err(crate::errors::usage(t!(
            keys::ERROR_CLAUDE_ENV_SET_NEEDS_ARGS
        )));
    }
    if !args.is_empty() {
        return Err(err_unsupported_args_with_help("claude env set", &args));
    }

    let home = PrismctlHome::discover(home).map_err(crate::errors::usage)?;
    let claude_root = home.tool_root(Tool::ClaudeCode);
    let settings_path = claude_root.join("settings.json");
    let existing = fs::read_to_string(&settings_path).unwrap_or_default();

    let has_auth_token = auth_token.is_some();

    let patch = ClaudeEnvPatch {
        auth_token,
        base_url,
        model,
        default_haiku_model: haiku_model,
        default_sonnet_model: sonnet_model,
        default_opus_model: opus_model,
    };
    let next = claude::apply_claude_env_patch_to_settings_json(&existing, &patch)?;

    let mut cs = ChangeSet::new();
    cs.push(Change::CreateDirAll {
        path: claude_root.clone(),
    });
    cs.push(Change::WriteFile {
        path: settings_path,
        bytes: next.into_bytes(),
        overwrite: true,
    });

    let title = tf!(keys::CHANGESET_PREVIEW_TITLE, "mode" => format!("{:?}", mode));
    println!("{}", title);
    for c in cs.iter() {
        println!("  - {}", crate::icons::render_change(c));
    }
    if has_auth_token {
        println!("{}", t!(keys::CLAUDE_AUTH_TOKEN_HIDDEN));
    }

    if mode == ApplyMode::DryRun {
        println!("\n{}", t!(keys::DRY_RUN_HINT_WRITE));
        return Ok(());
    }

    let fs = RealFileSystem;
    let runner = RealCommandRunner;
    cs.apply(mode, &fs, &runner)?;
    println!("\n{}", t!(keys::APPLY_APPLIED));
    Ok(())
}

fn cmd_claude_output_style(mut args: Vec<String>) -> Result<(), String> {
    let Some(action) = args.first().cloned() else {
        return Err(err_missing_subcommand_with_help("claude output-style"));
    };
    args.remove(0);

    match action.as_str() {
        "use" => cmd_claude_output_style_use(args),
        _ => Err(err_unknown_subcommand_with_help(
            "claude output-style",
            &action,
        )),
    }
}

fn cmd_claude_output_style_use(mut args: Vec<String>) -> Result<(), String> {
    let home = parse_home(&mut args)?;
    let mode = parse_apply_mode(&mut args)?;

    let name = parse_required_value(&mut args, "--name")?;
    if !args.is_empty() {
        return Err(err_unsupported_args_with_help(
            "claude output-style use",
            &args,
        ));
    }

    let home = PrismctlHome::discover(home).map_err(crate::errors::usage)?;
    let claude_root = home.tool_root(Tool::ClaudeCode);
    let settings_path = claude_root.join("settings.json");
    let existing = fs::read_to_string(&settings_path).unwrap_or_default();
    let next = claude::set_claude_output_style_in_settings_json(&existing, &name)?;

    let mut cs = ChangeSet::new();
    cs.push(Change::CreateDirAll {
        path: claude_root.clone(),
    });
    cs.push(Change::WriteFile {
        path: settings_path,
        bytes: next.into_bytes(),
        overwrite: true,
    });

    let title = tf!(keys::CHANGESET_PREVIEW_TITLE, "mode" => format!("{:?}", mode));
    println!("{}", title);
    for c in cs.iter() {
        println!("  - {}", crate::icons::render_change(c));
    }
    println!("{}", tf!(keys::CLAUDE_OUTPUT_STYLE_SET, "name" => name));

    if mode == ApplyMode::DryRun {
        println!("\n{}", t!(keys::DRY_RUN_HINT_WRITE));
        return Ok(());
    }

    let fs = RealFileSystem;
    let runner = RealCommandRunner;
    cs.apply(mode, &fs, &runner)?;
    println!("\n{}", t!(keys::APPLY_APPLIED));
    Ok(())
}

// ---- gemini ----

fn cmd_gemini_env(mut args: Vec<String>) -> Result<(), String> {
    let Some(action) = args.first().cloned() else {
        return Err(err_missing_subcommand_with_help("gemini env"));
    };
    args.remove(0);

    match action.as_str() {
        "set" => cmd_gemini_env_set(args),
        _ => Err(err_unknown_subcommand_with_help("gemini env", &action)),
    }
}

fn cmd_gemini_settings(mut args: Vec<String>) -> Result<(), String> {
    let Some(action) = args.first().cloned() else {
        return Err(err_missing_subcommand_with_help("gemini settings"));
    };
    args.remove(0);

    match action.as_str() {
        "set" => cmd_gemini_settings_set(args),
        _ => Err(err_unknown_subcommand_with_help("gemini settings", &action)),
    }
}

fn cmd_gemini_mcp(mut args: Vec<String>) -> Result<(), String> {
    let Some(action) = args.first().cloned() else {
        return Err(err_missing_subcommand_with_help("gemini mcp"));
    };
    args.remove(0);

    match action.as_str() {
        "list" => cmd_gemini_mcp_list(args),
        "builtin" => cmd_gemini_mcp_builtin(args),
        "add" => cmd_gemini_mcp_add(args),
        "remove" => cmd_gemini_mcp_remove(args),
        "enable" => cmd_gemini_mcp_enable(args),
        "disable" => cmd_gemini_mcp_disable(args),
        _ => Err(err_unknown_subcommand_with_help("gemini mcp", &action)),
    }
}

fn parse_gemini_mcp_scope(args: &mut Vec<String>) -> Result<prismctl_core::mcp::GeminiMcpScope, String> {
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--scope" {
            let v = args
                .get(i + 1)
                .ok_or_else(|| crate::errors::usage(tf!(keys::ERROR_FLAG_MISSING_VALUE, "flag" => "--scope")))?;
            let scope = match v.as_str() {
                "project" => prismctl_core::mcp::GeminiMcpScope::Project,
                "user" => prismctl_core::mcp::GeminiMcpScope::User,
                _ => {
                    return Err(crate::errors::usage(tf!(
                        keys::ERROR_INVALID_CHOICE,
                        "choice" => v
                    )))
                }
            };
            args.drain(i..=i + 1);
            return Ok(scope);
        }
        i += 1;
    }
    // Follow Gemini CLI default: project scope.
    Ok(prismctl_core::mcp::GeminiMcpScope::Project)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GeminiScope {
    User,
    Project,
}

fn parse_gemini_scope(args: &mut Vec<String>) -> Result<GeminiScope, String> {
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--scope" {
            let v = args
                .get(i + 1)
                .ok_or_else(|| crate::errors::usage(tf!(keys::ERROR_FLAG_MISSING_VALUE, "flag" => "--scope")))?;
            let scope = match v.as_str() {
                "user" => GeminiScope::User,
                "project" => GeminiScope::Project,
                _ => {
                    return Err(crate::errors::usage(tf!(
                        keys::ERROR_INVALID_CHOICE,
                        "choice" => v
                    )))
                }
            };
            args.drain(i..=i + 1);
            return Ok(scope);
        }
        i += 1;
    }
    Ok(GeminiScope::User)
}

fn cmd_gemini_env_set(mut args: Vec<String>) -> Result<(), String> {
    let home = parse_home(&mut args)?;
    let mode = parse_apply_mode(&mut args)?;
    let scope = parse_gemini_scope(&mut args)?;
    let project_path = parse_project_path_opt(&mut args)?;

    let mut api_key: Option<String> = None;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--api-key" => {
                api_key = Some(
                    args.get(i + 1)
                        .ok_or_else(|| {
                            crate::errors::usage(tf!(
                                keys::ERROR_FLAG_MISSING_VALUE,
                                "flag" => "--api-key"
                            ))
                        })?
                        .to_string(),
                );
                args.drain(i..=i + 1);
            }
            _ => i += 1,
        }
    }

    if api_key.is_none() {
        return Err(crate::errors::usage(t!(
            keys::ERROR_GEMINI_ENV_SET_NEEDS_ARGS
        )));
    }
    if !args.is_empty() {
        return Err(err_unsupported_args_with_help("gemini env set", &args));
    }

    let home = PrismctlHome::discover(home).map_err(crate::errors::usage)?;
    let (gemini_dir, env_path) = match scope {
        GeminiScope::User => {
            let root = home.tool_root(Tool::GeminiCli);
            (root.clone(), root.join(".env"))
        }
        GeminiScope::Project => {
            let root = match project_path {
                Some(p) => normalize_path(&p),
                None => env::current_dir().map_err(|e| tf!(keys::ERROR_CURRENT_DIR, "error" => e))?,
            };
            let dir = root.join(".gemini");
            (dir.clone(), dir.join(".env"))
        }
    };
    let existing = fs::read_to_string(&env_path).unwrap_or_default();
    let start_marker = "# prismctl:start";
    let end_marker = "# prismctl:end";

    let mut kv = parse_env_block(
        &extract_managed_block(&existing, start_marker, end_marker).unwrap_or_default(),
    );
    if let Some(v) = api_key {
        kv.insert("GEMINI_API_KEY".to_string(), v);
    }

    let block = format_env_block(&kv);
    let next = upsert_managed_block(&existing, start_marker, end_marker, &block);
    let mut cs = ChangeSet::new();
    cs.push(Change::CreateDirAll {
        path: gemini_dir,
    });
    cs.push(Change::WriteFile {
        path: env_path,
        bytes: next.into_bytes(),
        overwrite: true,
    });

    let title = tf!(keys::CHANGESET_PREVIEW_TITLE, "mode" => format!("{:?}", mode));
    println!("{}", title);
    for c in cs.iter() {
        println!("  - {}", crate::icons::render_change(c));
    }
    println!("{}", t!(keys::GEMINI_ENV_MANAGED_KEYS));

    if mode == ApplyMode::DryRun {
        println!("\n{}", t!(keys::DRY_RUN_HINT_WRITE));
        return Ok(());
    }

    let fs = RealFileSystem;
    let runner = RealCommandRunner;
    cs.apply(mode, &fs, &runner)?;
    println!("\n{}", t!(keys::APPLY_APPLIED));
    Ok(())
}

fn cmd_gemini_settings_set(mut args: Vec<String>) -> Result<(), String> {
    use prismctl_core::gemini;

    let home = parse_home(&mut args)?;
    let mode = parse_apply_mode(&mut args)?;
    let scope = parse_gemini_scope(&mut args)?;
    let project_path = parse_project_path_opt(&mut args)?;

    let mut model: Option<String> = None;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--model" => {
                model = Some(
                    args.get(i + 1)
                        .ok_or_else(|| {
                            crate::errors::usage(tf!(
                                keys::ERROR_FLAG_MISSING_VALUE,
                                "flag" => "--model"
                            ))
                        })?
                        .to_string(),
                );
                args.drain(i..=i + 1);
            }
            _ => i += 1,
        }
    }

    let Some(model_name) = model else {
        return Err(crate::errors::usage(t!(
            keys::ERROR_GEMINI_SETTINGS_SET_NEEDS_ARGS
        )));
    };
    if !args.is_empty() {
        return Err(err_unsupported_args_with_help("gemini settings set", &args));
    }

    let home = PrismctlHome::discover(home).map_err(crate::errors::usage)?;

    let (settings_dir, settings_path) = match scope {
        GeminiScope::User => {
            let root = home.tool_root(Tool::GeminiCli);
            (root.clone(), root.join("settings.json"))
        }
        GeminiScope::Project => {
            let root = match project_path {
                Some(p) => normalize_path(&p),
                None => env::current_dir().map_err(|e| tf!(keys::ERROR_CURRENT_DIR, "error" => e))?,
            };
            let dir = root.join(".gemini");
            (dir.clone(), dir.join("settings.json"))
        }
    };

    let existing = fs::read_to_string(&settings_path).unwrap_or_default();
    let next = gemini::set_gemini_model_name_in_settings_json(&existing, &model_name)?;

    let mut cs = ChangeSet::new();
    cs.push(Change::CreateDirAll { path: settings_dir });
    cs.push(Change::WriteFile {
        path: settings_path,
        bytes: next.into_bytes(),
        overwrite: true,
    });

    let title = tf!(keys::CHANGESET_PREVIEW_TITLE, "mode" => format!("{:?}", mode));
    println!("{}", title);
    for c in cs.iter() {
        println!("  - {}", crate::icons::render_change(c));
    }

    if mode == ApplyMode::DryRun {
        println!("\n{}", t!(keys::DRY_RUN_HINT_WRITE));
        return Ok(());
    }

    let fs = RealFileSystem;
    let runner = RealCommandRunner;
    cs.apply(mode, &fs, &runner)?;
    println!("\n{}", t!(keys::APPLY_APPLIED));
    Ok(())
}

fn cmd_gemini_mcp_list(mut args: Vec<String>) -> Result<(), String> {
    let home = parse_home(&mut args)?;
    let scope = parse_gemini_mcp_scope(&mut args)?;
    let project_path = parse_project_path_opt(&mut args)?;
    if !args.is_empty() {
        return Err(err_unsupported_args_with_help("gemini mcp list", &args));
    }

    let home = PrismctlHome::discover(home).map_err(crate::errors::usage)?;
    let envs = vec![
        ("HOME".to_string(), home.home_dir().to_string_lossy().to_string()),
        ("USERPROFILE".to_string(), home.home_dir().to_string_lossy().to_string()),
    ];
    let mut cmd = std::process::Command::new("gemini");
    cmd.args(["mcp", "list", "--scope", scope.as_flag_value()])
        .envs(envs);
    if scope == prismctl_core::mcp::GeminiMcpScope::Project {
        if let Some(cwd) = project_path {
            cmd.current_dir(cwd);
        }
    }
    let status = cmd
        .status()
        .map_err(|e| format!("执行命令失败: gemini: {}", e))?;
    if !status.success() {
        return Err(format!("gemini mcp list 失败（exit={}）", status));
    }
    Ok(())
}

fn cmd_gemini_mcp_builtin(args: Vec<String>) -> Result<(), String> {
    if !args.is_empty() {
        return Err(err_unsupported_args_with_help("gemini mcp builtin", &args));
    }
    println!("内置 MCP servers（Gemini CLI）：");
    for s in prismctl_core::mcp::list_builtin_mcp_servers() {
        println!("  - {} ({})", s.id, s.transport.as_flag_value());
        match s.transport {
            prismctl_core::mcp::McpTransport::Stdio => {
                let cmd = s.command.unwrap_or("<missing>");
                let args_joined = s.args.join(" ");
                if args_joined.is_empty() {
                    println!("      command: {}", cmd);
                } else {
                    println!("      command: {} {}", cmd, args_joined);
                }
                if !s.env.is_empty() {
                    let keys = s.env.iter().map(|(k, _)| *k).collect::<Vec<_>>().join(", ");
                    println!("      env: {}", keys);
                }
            }
            _ => {
                if let Some(url) = s.url {
                    println!("      url: {}", url);
                }
                if !s.headers.is_empty() {
                    let keys = s
                        .headers
                        .iter()
                        .map(|(k, _)| *k)
                        .collect::<Vec<_>>()
                        .join(", ");
                    println!("      headers: {}", keys);
                }
            }
        }
    }
    Ok(())
}

fn cmd_gemini_mcp_add(mut args: Vec<String>) -> Result<(), String> {
    let home = parse_home(&mut args)?;
    let mode = parse_apply_mode(&mut args)?;
    let yes = take_flag(&mut args, "--yes");
    let scope = parse_gemini_mcp_scope(&mut args)?;
    let project_path = parse_project_path_opt(&mut args)?;

    let name = parse_required_value(&mut args, "--name")?;
    if !args.is_empty() {
        return Err(err_unsupported_args_with_help("gemini mcp add", &args));
    }

    let home = PrismctlHome::discover(home).map_err(crate::errors::usage)?;
    let cs = prismctl_core::mcp::plan_gemini_mcp_add(&home, scope, &name, project_path)
        .map_err(crate::errors::usage)?;

    let title = tf!(keys::CHANGESET_PREVIEW_TITLE, "mode" => format!("{:?}", mode));
    println!("{}", title);
    for c in cs.iter() {
        println!("  - {}", crate::icons::render_change(c));
    }

    if mode == ApplyMode::DryRun {
        println!("\n{}", t!(keys::DRY_RUN_HINT_WRITE));
        return Ok(());
    }

    if !yes {
        return Err(crate::errors::usage(danger_gemini_mcp_confirmation()));
    }

    let fs = RealFileSystem;
    let runner = RealCommandRunner;
    cs.apply(mode, &fs, &runner)?;
    println!("\n{}", t!(keys::APPLY_APPLIED));
    Ok(())
}

fn cmd_gemini_mcp_remove(mut args: Vec<String>) -> Result<(), String> {
    let home = parse_home(&mut args)?;
    let mode = parse_apply_mode(&mut args)?;
    let yes = take_flag(&mut args, "--yes");
    let scope = parse_gemini_mcp_scope(&mut args)?;
    let project_path = parse_project_path_opt(&mut args)?;

    let name = parse_required_value(&mut args, "--name")?;
    if !args.is_empty() {
        return Err(err_unsupported_args_with_help("gemini mcp remove", &args));
    }

    let home = PrismctlHome::discover(home).map_err(crate::errors::usage)?;
    let cs = prismctl_core::mcp::plan_gemini_mcp_remove(&home, scope, &name, project_path);

    let title = tf!(keys::CHANGESET_PREVIEW_TITLE, "mode" => format!("{:?}", mode));
    println!("{}", title);
    for c in cs.iter() {
        println!("  - {}", crate::icons::render_change(c));
    }

    if mode == ApplyMode::DryRun {
        println!("\n{}", t!(keys::DRY_RUN_HINT_WRITE));
        return Ok(());
    }

    if !yes {
        return Err(crate::errors::usage(danger_gemini_mcp_confirmation()));
    }

    let fs = RealFileSystem;
    let runner = RealCommandRunner;
    cs.apply(mode, &fs, &runner)?;
    println!("\n{}", t!(keys::APPLY_APPLIED));
    Ok(())
}

fn cmd_gemini_mcp_enable(mut args: Vec<String>) -> Result<(), String> {
    let home = parse_home(&mut args)?;
    let mode = parse_apply_mode(&mut args)?;
    let yes = take_flag(&mut args, "--yes");
    let project_path = parse_project_path_opt(&mut args)?;
    let name = parse_required_value(&mut args, "--name")?;
    if !args.is_empty() {
        return Err(err_unsupported_args_with_help("gemini mcp enable", &args));
    }

    let home = PrismctlHome::discover(home).map_err(crate::errors::usage)?;
    let cs = prismctl_core::mcp::plan_gemini_mcp_enable(&home, &name, project_path);

    let title = tf!(keys::CHANGESET_PREVIEW_TITLE, "mode" => format!("{:?}", mode));
    println!("{}", title);
    for c in cs.iter() {
        println!("  - {}", crate::icons::render_change(c));
    }

    if mode == ApplyMode::DryRun {
        println!("\n{}", t!(keys::DRY_RUN_HINT_WRITE));
        return Ok(());
    }

    if !yes {
        return Err(crate::errors::usage(danger_gemini_mcp_confirmation()));
    }

    let fs = RealFileSystem;
    let runner = RealCommandRunner;
    cs.apply(mode, &fs, &runner)?;
    println!("\n{}", t!(keys::APPLY_APPLIED));
    Ok(())
}

fn cmd_gemini_mcp_disable(mut args: Vec<String>) -> Result<(), String> {
    let home = parse_home(&mut args)?;
    let mode = parse_apply_mode(&mut args)?;
    let yes = take_flag(&mut args, "--yes");
    let project_path = parse_project_path_opt(&mut args)?;
    let name = parse_required_value(&mut args, "--name")?;
    if !args.is_empty() {
        return Err(err_unsupported_args_with_help("gemini mcp disable", &args));
    }

    let home = PrismctlHome::discover(home).map_err(crate::errors::usage)?;
    let cs = prismctl_core::mcp::plan_gemini_mcp_disable(&home, &name, project_path);

    let title = tf!(keys::CHANGESET_PREVIEW_TITLE, "mode" => format!("{:?}", mode));
    println!("{}", title);
    for c in cs.iter() {
        println!("  - {}", crate::icons::render_change(c));
    }

    if mode == ApplyMode::DryRun {
        println!("\n{}", t!(keys::DRY_RUN_HINT_WRITE));
        return Ok(());
    }

    if !yes {
        return Err(crate::errors::usage(danger_gemini_mcp_confirmation()));
    }

    let fs = RealFileSystem;
    let runner = RealCommandRunner;
    cs.apply(mode, &fs, &runner)?;
    println!("\n{}", t!(keys::APPLY_APPLIED));
    Ok(())
}

pub(crate) fn danger_gemini_mcp_confirmation() -> String {
    [
        t!(keys::DANGER_TITLE),
        t!(keys::DANGER_GEMINI_MCP_TYPE),
        t!(keys::DANGER_GEMINI_MCP_SCOPE),
        t!(keys::DANGER_GEMINI_MCP_RISK),
        String::new(),
        t!(keys::DANGER_CONFIRM_NEED_YES),
    ]
    .join("\n")
}

fn parse_env_block(block: &str) -> std::collections::HashMap<String, String> {
    let mut out = std::collections::HashMap::new();
    for raw in block.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((k, v)) = line.split_once('=') else {
            continue;
        };
        let key = k.trim().to_string();
        let value = v.trim().trim_matches('"').to_string();
        if !key.is_empty() {
            out.insert(key, value);
        }
    }
    out
}

fn format_env_value(value: &str) -> String {
    let needs_quotes = value
        .chars()
        .any(|c| c.is_whitespace() || c == '#' || c == '"');
    if !needs_quotes {
        return value.to_string();
    }
    let escaped = value.replace('\\', "\\\\").replace('"', "\\\"");
    format!("\"{}\"", escaped)
}

fn format_env_block(kv: &std::collections::HashMap<String, String>) -> String {
    // Prismctl only owns GEMINI_API_KEY for Gemini CLI; keep other keys outside the managed block.
    let mut keys: Vec<&str> = vec!["GEMINI_API_KEY"];
    keys.retain(|k| kv.contains_key(*k));
    let mut out = String::new();
    for k in keys {
        let v = kv.get(k).expect("key exists");
        out.push_str(k);
        out.push('=');
        out.push_str(&format_env_value(v));
        out.push('\n');
    }
    out
}
