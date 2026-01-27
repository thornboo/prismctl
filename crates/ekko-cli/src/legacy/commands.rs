use ekko_core::changeset::{ApplyMode, Change, ChangeSet, RealCommandRunner, RealFileSystem};
use ekko_core::claude::{self, ClaudeEnvPatch};
use ekko_core::codex::{self, CodexProviderConfig};
use ekko_core::installer::{InstallAction, InstallMethod, ToolInstallTarget};
use ekko_core::managed_block::{extract_managed_block, upsert_managed_block};
use ekko_core::paths::{EkkoHome, Tool};
use ekko_core::providers;
use ekko_core::skill;
use ekko_core::templates::{self, TemplateLang};
use ekko_i18n::{keys, t, tf};
use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

pub fn help() -> String {
    match ekko_i18n::current_locale() {
        ekko_i18n::Locale::En => help_en(),
        ekko_i18n::Locale::ZhCN => help_zh_cn(),
    }
}

fn help_zh_cn() -> String {
    [
        "Ekko (early)\n",
        "用法:",
        "  ekko                              # 交互式菜单（仅 TTY；非 TTY 将报错）",
        "  ekko config                       # 交互式菜单（仅 TTY；非 TTY 将报错）",
        "  ekko config <CMD> [ARGS...]        # 镜像入口：等价于 `ekko <CMD> [ARGS...]`\n",
        "  ekko doctor [--home <PATH>]",
        "  ekko d                            # doctor 的短命令（等价于 ekko doctor）",
        "  ekko init --tool <codex|claude|gemini|all> [--home <PATH>] [--lang <zh-CN|en>] [--dry-run|--apply]\n",
        "  ekko i [-t|--tool <TOOL>] [-p|--provider <NAME>] [-k|--api-key <KEY>] [--lang <zh-CN|en>] [--home <PATH>] [--dry-run|--apply] [-s|--silent]",
        "  ekko update --tool <codex|claude|gemini|all> [--home <PATH>] [--lang <zh-CN|en>] [--dry-run|--apply]\n",
        "  ekko u [-t|--tool <TOOL>] [--lang <zh-CN|en>] [--home <PATH>] [--dry-run|--apply] [-s|--silent]",
        "  ekko install --tool <codex|claude|gemini|all> [--install-method <auto|npm|brew>] [--dry-run|--apply] [--yes]",
        "  ekko upgrade --tool <codex|claude|gemini|all> [--install-method <auto|npm|brew>] [--dry-run|--apply] [--yes]\n",
        "  ekko skill list [--home <PATH>]",
        "  ekko skill install --name <VALUE> [--home <PATH>] [--dry-run|--apply]",
        "  ekko skill create --name <VALUE> [--home <PATH>] [--dry-run|--apply]",
        "  ekko skill remove --name <VALUE> [--home <PATH>] [--dry-run|--apply] [--yes]\n",
        "  ekko codex agent list",
        "  ekko codex agent use --name <VALUE> [--home <PATH>] [--lang <zh-CN|en>] [--dry-run|--apply] [--yes]\n",
        "  ekko codex provider set [--home <PATH>] [--dry-run|--apply] [--provider <VALUE>] [--api-key <VALUE>] [--base-url <VALUE>] [--model <VALUE>] [--wire-api <VALUE>] [--default]\n",
        "  ekko claude env set [--home <PATH>] [--dry-run|--apply] [--auth-token <VALUE>] [--base-url <VALUE>] [--model <VALUE>] [--haiku-model <VALUE>] [--sonnet-model <VALUE>] [--opus-model <VALUE>]\n",
        "  ekko claude output-style use --name <VALUE> [--home <PATH>] [--dry-run|--apply]\n",
        "  ekko gemini env set [--home <PATH>] [--dry-run|--apply] [--api-key <VALUE>] [--base-url <VALUE>] [--model <VALUE>]\n",
        "  ekko project init [--path <PATH>] [--lang <zh-CN|en>] [--dry-run|--apply]\n",
        "行为:",
        "  - 默认 dry-run：仅打印将执行的变更，不会写入任何文件。",
        "  - 只有传入 --apply 才会真正落盘。",
        "  - --home 或 EKKO_HOME 可将所有读写重定向到沙箱 HOME，避免破坏真实配置。\n",
        "  - --verbose 会在报错时附加调试上下文（cmd/args）。\n",
    ]
    .join("\n")
}

fn help_en() -> String {
    [
        "Ekko (early)\n",
        "Usage:",
        "  ekko                              # interactive menu (TTY only; non-TTY will error)",
        "  ekko config                       # interactive menu (TTY only; non-TTY will error)",
        "  ekko config <CMD> [ARGS...]        # mirror entry: same as `ekko <CMD> [ARGS...]`\n",
        "  ekko doctor [--home <PATH>]",
        "  ekko d                            # short for doctor\n",
        "  ekko init --tool <codex|claude|gemini|all> [--home <PATH>] [--lang <zh-CN|en>] [--dry-run|--apply]\n",
        "  ekko i [-t|--tool <TOOL>] [-p|--provider <NAME>] [-k|--api-key <KEY>] [--lang <zh-CN|en>] [--home <PATH>] [--dry-run|--apply] [-s|--silent]",
        "  ekko update --tool <codex|claude|gemini|all> [--home <PATH>] [--lang <zh-CN|en>] [--dry-run|--apply]\n",
        "  ekko u [-t|--tool <TOOL>] [--lang <zh-CN|en>] [--home <PATH>] [--dry-run|--apply] [-s|--silent]",
        "  ekko install --tool <codex|claude|gemini|all> [--install-method <auto|npm|brew>] [--dry-run|--apply] [--yes]",
        "  ekko upgrade --tool <codex|claude|gemini|all> [--install-method <auto|npm|brew>] [--dry-run|--apply] [--yes]\n",
        "  ekko skill list [--home <PATH>]",
        "  ekko skill install --name <VALUE> [--home <PATH>] [--dry-run|--apply]",
        "  ekko skill create --name <VALUE> [--home <PATH>] [--dry-run|--apply]",
        "  ekko skill remove --name <VALUE> [--home <PATH>] [--dry-run|--apply] [--yes]\n",
        "  ekko codex agent list",
        "  ekko codex agent use --name <VALUE> [--home <PATH>] [--lang <zh-CN|en>] [--dry-run|--apply] [--yes]\n",
        "  ekko codex provider set [--home <PATH>] [--dry-run|--apply] [--provider <VALUE>] [--api-key <VALUE>] [--base-url <VALUE>] [--model <VALUE>] [--wire-api <VALUE>] [--default]\n",
        "  ekko claude env set [--home <PATH>] [--dry-run|--apply] [--auth-token <VALUE>] [--base-url <VALUE>] [--model <VALUE>] [--haiku-model <VALUE>] [--sonnet-model <VALUE>] [--opus-model <VALUE>]\n",
        "  ekko claude output-style use --name <VALUE> [--home <PATH>] [--dry-run|--apply]\n",
        "  ekko gemini env set [--home <PATH>] [--dry-run|--apply] [--api-key <VALUE>] [--base-url <VALUE>] [--model <VALUE>]\n",
        "  ekko project init [--path <PATH>] [--lang <zh-CN|en>] [--dry-run|--apply]\n",
        "Behavior:",
        "  - Default is dry-run: prints planned changes without writing files.",
        "  - Only `--apply` writes changes to disk.",
        "  - `--home` or `EKKO_HOME` redirects all I/O to a sandbox home.\n",
        "  - `--verbose` adds cmd/args context on errors.\n",
    ]
    .join("\n")
}

pub fn cmd_doctor(mut args: Vec<String>) -> Result<(), String> {
    let home = parse_home(&mut args)?;
    if !args.is_empty() {
        return Err(err_unsupported_args_with_help("doctor", &args));
    }

    let home = EkkoHome::discover(home)?;
    let codex = home.tool_root(Tool::Codex);
    let claude = home.tool_root(Tool::ClaudeCode);
    let gemini = home.tool_root(Tool::GeminiCli);
    let claude_settings = home.claude_settings_path();

    println!("Ekko HOME: {}", home.home_dir().display());
    println!("Codex root: {}", codex.display());
    println!("Claude root: {}", claude.display());
    println!("Claude settings: {}", claude_settings.display());
    println!("Gemini root: {}", gemini.display());

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

    let home = EkkoHome::discover(home)?;
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

    let home = EkkoHome::discover(home)?;
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
        _ => Err(err_unknown_subcommand_with_help("claude", &sub)),
    }
}

pub fn cmd_gemini(mut args: Vec<String>) -> Result<(), String> {
    let Some(sub) = args.first().cloned() else {
        return Err(err_missing_subcommand_with_help("gemini"));
    };
    args.remove(0);

    match sub.as_str() {
        "env" => cmd_gemini_env(args),
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
    let home = EkkoHome::discover(home)?;

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

    let home = EkkoHome::discover(home)?;
    let cs = skill::plan_install_skill(&home, &name)?;

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

    let home = EkkoHome::discover(home)?;
    skill::validate_skill_name(&name)?;
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

    let home = EkkoHome::discover(home)?;
    let cs = skill::plan_remove_skill(&home, &name)?;

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
        return Err(danger_skill_remove_confirmation(&name));
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

    let cs = ekko_core::project::plan_project_init(&project_root, lang, &existing);

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
        cs.extend(ekko_core::installer::plan_install(t, method, action));
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
        return Err(danger_install_confirmation(action));
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

fn cmd_codex_agent_use(mut args: Vec<String>) -> Result<(), String> {
    let home = parse_home(&mut args)?;
    let mode = parse_apply_mode(&mut args)?;
    let lang = parse_lang(&mut args)?;
    let yes = take_flag(&mut args, "--yes");

    let name = parse_required_value(&mut args, "--name")?;
    if !args.is_empty() {
        return Err(err_unsupported_args_with_help("codex agent use", &args));
    }

    let tpl = templates::codex_agent_template(&name, lang)
        .ok_or_else(|| crate::errors::usage(tf!(keys::ERROR_UNKNOWN_AGENT, "name" => &name)))?;

    let home = EkkoHome::discover(home)?;
    let codex_root = home.tool_root(Tool::Codex);
    let agents_path = codex_root.join("AGENTS.md");

    let existing = fs::read_to_string(&agents_path).unwrap_or_default();

    let mut cs = ChangeSet::new();
    cs.push(Change::CreateDirAll {
        path: codex_root.clone(),
    });

    if !existing.trim().is_empty() {
        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| tf!(keys::ERROR_TIMESTAMP, "error" => e))?
            .as_secs();
        let backup_path = codex_root
            .join("backup")
            .join("ekko")
            .join(ts.to_string())
            .join("AGENTS.md");
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
        return Err(danger_codex_agent_use_confirmation());
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

    let home = EkkoHome::discover(home)?;
    let codex_root = home.tool_root(Tool::Codex);
    let config_path = codex_root.join("config.toml");
    let auth_path = codex_root.join("auth.json");

    let existing_toml = fs::read_to_string(&config_path).unwrap_or_default();
    let existing_auth = fs::read_to_string(&auth_path).unwrap_or_default();

    // Default values keep the command usable even when only a subset of args is provided.
    let provider_id = "ekko".to_string();
    let temp_env_key = "EKKO_CODEX_API_KEY".to_string();

    let resolved = providers::resolve_codex_provider(provider, base_url, wire_api, model);

    let display_name = match provider {
        Some(p) => format!("Ekko ({})", p.id()),
        None => "Ekko".to_string(),
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

    let home = EkkoHome::discover(home)?;
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

    let home = EkkoHome::discover(home)?;
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

fn cmd_gemini_env_set(mut args: Vec<String>) -> Result<(), String> {
    let home = parse_home(&mut args)?;
    let mode = parse_apply_mode(&mut args)?;

    let mut api_key: Option<String> = None;
    let mut base_url: Option<String> = None;
    let mut model: Option<String> = None;

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
            _ => i += 1,
        }
    }

    if api_key.is_none() && base_url.is_none() && model.is_none() {
        return Err(crate::errors::usage(t!(
            keys::ERROR_GEMINI_ENV_SET_NEEDS_ARGS
        )));
    }
    if !args.is_empty() {
        return Err(err_unsupported_args_with_help("gemini env set", &args));
    }

    let home = EkkoHome::discover(home)?;
    let gemini_root = home.tool_root(Tool::GeminiCli);
    let env_path = gemini_root.join(".env");
    let existing = fs::read_to_string(&env_path).unwrap_or_default();
    let start_marker = "# ekko:start";
    let end_marker = "# ekko:end";

    let mut kv = parse_env_block(
        &extract_managed_block(&existing, start_marker, end_marker).unwrap_or_default(),
    );
    if let Some(v) = api_key {
        kv.insert("GEMINI_API_KEY".to_string(), v);
    }
    if let Some(v) = base_url {
        kv.insert("GOOGLE_GEMINI_BASE_URL".to_string(), v);
    }
    if let Some(v) = model {
        kv.insert("GEMINI_MODEL".to_string(), v);
    }

    let block = format_env_block(&kv);
    let next = upsert_managed_block(&existing, start_marker, end_marker, &block);
    let mut cs = ChangeSet::new();
    cs.push(Change::CreateDirAll {
        path: gemini_root.clone(),
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
    let mut keys: Vec<&str> = vec!["GOOGLE_GEMINI_BASE_URL", "GEMINI_API_KEY", "GEMINI_MODEL"];
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
