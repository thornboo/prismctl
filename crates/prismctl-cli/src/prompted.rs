use crate::interactive::utils::{prompt_confirm, prompt_required, prompt_select};
use crate::legacy;
use prismctl_core::installer::InstallAction;
use prismctl_core::templates;
use prismctl_i18n::{keys, t, tf};
use std::io;
use std::io::IsTerminal;

fn is_interactive_tty() -> bool {
    io::stdin().is_terminal() && io::stdout().is_terminal()
}

fn non_tty_completion_error(invocation: &str, missing: &str) -> String {
    crate::errors::usage(
        [
            t!(keys::ERROR_NON_TTY_TITLE),
            String::new(),
            t!(keys::ERROR_NON_TTY_SCOPE),
            tf!(keys::ERROR_NON_TTY_RUNNING, "invocation" => invocation),
            tf!(keys::ERROR_NON_TTY_MISSING, "missing" => missing),
            String::new(),
            t!(keys::ERROR_NON_TTY_SOLUTIONS),
            t!(keys::ERROR_NON_TTY_SOLUTION_TTY),
            t!(keys::ERROR_NON_TTY_SOLUTION_ARGS),
            t!(keys::ERROR_NON_TTY_SOLUTION_HELP),
        ]
        .join("\n"),
    )
}

fn tool_options() -> Vec<String> {
    vec![
        "codex".to_string(),
        "claude".to_string(),
        "gemini".to_string(),
        "all".to_string(),
    ]
}

fn normalize_tool_value(raw: &str) -> Option<&'static str> {
    match raw {
        "codex" => Some("codex"),
        "claude" | "claude-code" => Some("claude"),
        "gemini" | "gemini-cli" => Some("gemini"),
        "all" => Some("all"),
        _ => None,
    }
}

fn tool_flag_is_missing_or_incomplete(args: &[String]) -> bool {
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--tool" {
            if i + 1 >= args.len() {
                return true;
            }
            return normalize_tool_value(args[i + 1].as_str()).is_none();
        }
        i += 1;
    }
    true
}

fn ensure_flag_value(
    args: &mut Vec<String>,
    flag: &str,
    default_index: usize,
    prompt_label: &str,
    normalize: fn(&str) -> Option<&'static str>,
) -> Result<(), String> {
    let Some(i) = args.iter().position(|a| a == flag) else {
        let value = prompt_select(prompt_label, tool_options(), default_index)?;
        args.push(flag.to_string());
        args.push(value);
        return Ok(());
    };

    if i + 1 >= args.len() {
        // `--flag` at tail (missing value).
        args.remove(i);
        let value = prompt_select(prompt_label, tool_options(), default_index)?;
        args.push(flag.to_string());
        args.push(value);
        return Ok(());
    }

    if normalize(args[i + 1].as_str()).is_none() {
        // Invalid value, replace interactively.
        args.drain(i..=i + 1);
        let value = prompt_select(prompt_label, tool_options(), default_index)?;
        args.push(flag.to_string());
        args.push(value);
    }

    Ok(())
}

fn ensure_name_flag(args: &mut Vec<String>, flag: &str, prompt_label: &str) -> Result<(), String> {
    match args.iter().position(|a| a == flag) {
        None => {
            let v = prompt_required(prompt_label)?;
            args.push(flag.to_string());
            args.push(v);
        }
        Some(i) if i + 1 >= args.len() => {
            // `--name` at tail (missing value).
            args.remove(i);
            let v = prompt_required(prompt_label)?;
            args.push(flag.to_string());
            args.push(v);
        }
        Some(_) => {}
    }
    Ok(())
}

fn ensure_yes_confirmation(warning: &str) -> Result<bool, String> {
    println!("{}", warning);
    let prompt = t!(keys::PROMPT_CONFIRM_CONTINUE);
    prompt_confirm(&prompt, false)
}

fn ensure_codex_agent_name(args: &mut Vec<String>) -> Result<(), String> {
    let idx = args.iter().position(|a| a == "--name");

    let needs_prompt = match idx {
        None => true,
        Some(i) => i + 1 >= args.len(),
    };

    if !needs_prompt {
        return Ok(());
    }

    if let Some(i) = idx {
        // `--name` at tail (missing value).
        args.remove(i);
    }

    let mut options = templates::codex_agent_names()
        .iter()
        .map(|s| (*s).to_string())
        .collect::<Vec<_>>();
    let custom = t!(keys::PROMPT_CODEX_AGENT_CUSTOM);
    options.push(custom.clone());

    let title = t!(keys::PROMPT_CODEX_AGENT_FLAG);
    let choice = prompt_select(&title, options, 0)?;
    let name = if choice == custom {
        let p = t!(keys::PROMPT_CODEX_AGENT_NAME);
        prompt_required(&p)?
    } else {
        choice
    };
    args.push("--name".to_string());
    args.push(name);
    Ok(())
}

pub fn cmd_init(mut args: Vec<String>) -> Result<(), String> {
    if !is_interactive_tty() {
        if tool_flag_is_missing_or_incomplete(&args) {
            return Err(non_tty_completion_error(
                "prismctl init",
                &t!(keys::ERROR_TOOL_FLAG_INVALID),
            ));
        }
        return legacy::cmd_init(args);
    }
    ensure_flag_value(
        &mut args,
        "--tool",
        3,
        &t!(keys::PROMPT_TOOL_INIT),
        normalize_tool_value,
    )?;
    legacy::cmd_init(args)
}

pub fn cmd_update(mut args: Vec<String>) -> Result<(), String> {
    if !is_interactive_tty() {
        if tool_flag_is_missing_or_incomplete(&args) {
            return Err(non_tty_completion_error(
                "prismctl update",
                &t!(keys::ERROR_TOOL_FLAG_INVALID),
            ));
        }
        return legacy::cmd_update(args);
    }
    ensure_flag_value(
        &mut args,
        "--tool",
        3,
        &t!(keys::PROMPT_TOOL_UPDATE),
        normalize_tool_value,
    )?;
    legacy::cmd_update(args)
}

pub fn cmd_install(mut args: Vec<String>) -> Result<(), String> {
    if !is_interactive_tty() {
        if tool_flag_is_missing_or_incomplete(&args) {
            return Err(non_tty_completion_error(
                "prismctl install",
                &t!(keys::ERROR_TOOL_FLAG_INVALID),
            ));
        }
        return legacy::cmd_install(args);
    }

    ensure_flag_value(
        &mut args,
        "--tool",
        3,
        &t!(keys::PROMPT_TOOL_INSTALL),
        normalize_tool_value,
    )?;

    if args.iter().any(|a| a == "--apply") && !args.iter().any(|a| a == "--yes") {
        let warning = legacy::danger_install_confirmation(InstallAction::Install);
        if ensure_yes_confirmation(&warning)? {
            args.push("--yes".to_string());
        } else {
            return Err(t!(keys::ACTION_CANCEL));
        }
    }

    legacy::cmd_install(args)
}

pub fn cmd_upgrade(mut args: Vec<String>) -> Result<(), String> {
    if !is_interactive_tty() {
        if tool_flag_is_missing_or_incomplete(&args) {
            return Err(non_tty_completion_error(
                "prismctl upgrade",
                &t!(keys::ERROR_TOOL_FLAG_INVALID),
            ));
        }
        return legacy::cmd_upgrade(args);
    }

    ensure_flag_value(
        &mut args,
        "--tool",
        3,
        &t!(keys::PROMPT_TOOL_UPGRADE),
        normalize_tool_value,
    )?;

    if args.iter().any(|a| a == "--apply") && !args.iter().any(|a| a == "--yes") {
        let warning = legacy::danger_install_confirmation(InstallAction::Upgrade);
        if ensure_yes_confirmation(&warning)? {
            args.push("--yes".to_string());
        } else {
            return Err(t!(keys::ACTION_CANCEL));
        }
    }

    legacy::cmd_upgrade(args)
}

pub fn cmd_skill(mut args: Vec<String>) -> Result<(), String> {
    if args.is_empty() {
        if !is_interactive_tty() {
            return Err(non_tty_completion_error(
                "prismctl skill",
                &t!(keys::ERROR_MISSING_SUBCOMMAND),
            ));
        }
        let title = t!(keys::PROMPT_SKILL_SUBCOMMAND);
        let choice = prompt_select(&title, vec!["list", "install", "create", "remove"], 0)?;
        args.push(choice.to_string());
    }

    let sub = args[0].clone();
    match sub.as_str() {
        "install" | "create" | "remove" => {
            if !is_interactive_tty() {
                // 让 legacy 输出原始错误（包含 help / 危险操作提示）。
                return legacy::cmd_skill(args);
            }
            let p = t!(keys::PROMPT_SKILL_NAME_FLAG);
            ensure_name_flag(&mut args, "--name", &p)?;
            if sub == "remove"
                && args.iter().any(|a| a == "--apply")
                && !args.iter().any(|a| a == "--yes")
            {
                let name = args
                    .iter()
                    .position(|a| a == "--name")
                    .and_then(|i| args.get(i + 1))
                    .cloned()
                    .unwrap_or_default();
                let warning = legacy::danger_skill_remove_confirmation(&name);
                if ensure_yes_confirmation(&warning)? {
                    args.push("--yes".to_string());
                } else {
                    return Err(t!(keys::ACTION_CANCEL));
                }
            }
            legacy::cmd_skill(args)
        }
        _ => legacy::cmd_skill(args),
    }
}

pub fn cmd_codex(mut args: Vec<String>) -> Result<(), String> {
    if args.is_empty() {
        if !is_interactive_tty() {
            return Err(non_tty_completion_error(
                "prismctl codex",
                &t!(keys::ERROR_MISSING_SUBCOMMAND),
            ));
        }
        let title = t!(keys::PROMPT_CODEX_SUBCOMMAND);
        let choice = prompt_select(&title, vec!["provider", "agent"], 0)?;
        args.push(choice.to_string());
    }

    // Parameter completion for: `prismctl codex agent use --name <...> [--apply] [--yes]`
    if args.first().map(|s| s.as_str()) == Some("agent")
        && args.get(1).map(|s| s.as_str()) == Some("use")
    {
        if !is_interactive_tty() {
            return legacy::cmd_codex(args);
        }

        ensure_codex_agent_name(&mut args)?;

        if args.iter().any(|a| a == "--apply") && !args.iter().any(|a| a == "--yes") {
            let warning = legacy::danger_codex_agent_use_confirmation();
            if ensure_yes_confirmation(&warning)? {
                args.push("--yes".to_string());
            } else {
                return Err(t!(keys::ACTION_CANCEL));
            }
        }
    }

    legacy::cmd_codex(args)
}

pub fn cmd_claude(args: Vec<String>) -> Result<(), String> {
    // 当前 claude 子命令没有必填参数；保持行为不变。
    legacy::cmd_claude(args)
}

pub fn cmd_gemini(args: Vec<String>) -> Result<(), String> {
    // 当前 gemini 子命令没有必填参数；保持行为不变。
    legacy::cmd_gemini(args)
}

pub fn cmd_project(args: Vec<String>) -> Result<(), String> {
    // project init 已有默认 path=current_dir；这里仅做缺少子命令时的 TTY 友好补全。
    if args.is_empty() {
        if !is_interactive_tty() {
            return Err(non_tty_completion_error(
                "prismctl project",
                &t!(keys::ERROR_MISSING_SUBCOMMAND),
            ));
        }
        let title = t!(keys::PROMPT_PROJECT_SUBCOMMAND);
        let choice = prompt_select(&title, vec!["init"], 0)?;
        return legacy::cmd_project(vec![choice.to_string()]);
    }
    legacy::cmd_project(args)
}
