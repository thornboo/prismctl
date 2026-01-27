mod app_config;
mod icons;
mod interactive;
mod legacy;
mod prompted;
mod quick;

use ekko_i18n::{keys, t, tf};
use std::env;
use std::io;
use std::io::IsTerminal;

fn main() {
    match run() {
        Ok(()) => {}
        Err(f) => {
            eprintln!("{}", f.message);
            std::process::exit(f.code);
        }
    }
}

struct CliFailure {
    message: String,
    code: i32,
}

fn run() -> Result<(), CliFailure> {
    app_config::apply_saved_locale_best_effort();

    let (verbose, args) = take_global_flag(env::args().skip(1).collect(), "--verbose");
    if args.is_empty() {
        return enter_interactive_or_fail("ekko");
    }

    let cmd = args[0].as_str();
    let rest = args[1..].to_vec();

    if is_help_cmd(cmd) {
        println!("{}", legacy::help());
        return Ok(());
    }

    // `ekko config <...>` mirrors all commands: `ekko config init ...` ≡ `ekko init ...`.
    if cmd == "config" {
        if rest.is_empty() {
            return enter_interactive_or_fail("ekko config");
        }
        if is_help_cmd(rest[0].as_str()) {
            println!("{}", legacy::help());
            return Ok(());
        }
        return dispatch_command(rest[0].as_str(), rest[1..].to_vec(), verbose);
    }

    dispatch_command(cmd, rest, verbose)
}

fn dispatch_command(cmd: &str, args: Vec<String>, verbose: bool) -> Result<(), CliFailure> {
    let args_dbg = if verbose {
        Some(format!("{:?}", args))
    } else {
        None
    };

    let res = match cmd {
        // Short aliases.
        "d" => legacy::cmd_doctor(args),
        "i" => quick::init::quick_init(args),
        "u" => quick::update::quick_update(args),

        // Legacy commands.
        "doctor" => legacy::cmd_doctor(args),
        "init" => prompted::cmd_init(args),
        "update" => prompted::cmd_update(args),
        "project" => prompted::cmd_project(args),
        "skill" => prompted::cmd_skill(args),
        "install" => prompted::cmd_install(args),
        "upgrade" => prompted::cmd_upgrade(args),
        "codex" => prompted::cmd_codex(args),
        "claude" => prompted::cmd_claude(args),
        "gemini" => prompted::cmd_gemini(args),

        _ => {
            return Err(CliFailure {
                message: format!(
                    "{}\n\n{}",
                    tf!(keys::ERROR_UNKNOWN_COMMAND, "cmd" => cmd),
                    legacy::help()
                ),
                code: 2,
            });
        }
    };

    res.map_err(|message| CliFailure {
        code: classify_error_code(&message),
        message: if verbose {
            format!(
                "{}\n\n[verbose] cmd={} args={}",
                message,
                cmd,
                args_dbg.unwrap_or_else(|| "[]".to_string())
            )
        } else {
            message
        },
    })
}

fn is_help_cmd(cmd: &str) -> bool {
    matches!(cmd, "-h" | "--help" | "help")
}

fn enter_interactive_or_fail(invocation: &str) -> Result<(), CliFailure> {
    if is_interactive_tty() {
        return interactive::menu::run().map_err(|message| CliFailure { message, code: 1 });
    }

    Err(CliFailure {
        message: non_tty_interactive_error(invocation),
        code: 2,
    })
}

fn is_interactive_tty() -> bool {
    io::stdin().is_terminal() && io::stdout().is_terminal()
}

fn non_tty_interactive_error(invocation: &str) -> String {
    let lines: Vec<String> = vec![
        t!(keys::ERROR_NON_TTY_INTERACTIVE_TITLE),
        String::new(),
        t!(keys::ERROR_NON_TTY_SCOPE),
        tf!(keys::ERROR_NON_TTY_RUNNING, "invocation" => invocation),
        String::new(),
        t!(keys::ERROR_NON_TTY_SOLUTIONS),
        t!(keys::ERROR_NON_TTY_INTERACTIVE_SOLUTION_MENU),
        t!(keys::ERROR_NON_TTY_INTERACTIVE_SOLUTION_SUBCMD),
        t!(keys::ERROR_NON_TTY_SOLUTION_HELP),
    ];
    lines.join("\n")
}

fn classify_error_code(message: &str) -> i32 {
    // Best-effort: keep 2 for user-facing usage errors; 1 for operational/runtime errors.
    let usage_markers = [
        "用法:",
        "Usage:",
        "不支持的参数",
        "unsupported argument",
        "Unsupported argument",
        "缺少参数",
        "Missing argument",
        "missing argument",
        "缺少子命令",
        "Missing subcommand",
        "missing subcommand",
        "缺少必填参数",
        "Missing required",
        "missing required",
        "未知命令",
        "Unknown command",
        "unknown command",
        "不支持的 --",
        "Unsupported --",
        "unsupported --",
        "Missing/invalid",
        "missing/invalid",
    ];
    if usage_markers.iter().any(|m| message.contains(m)) {
        return 2;
    }
    1
}

fn take_global_flag(mut args: Vec<String>, flag: &str) -> (bool, Vec<String>) {
    let mut found = false;
    let mut i = 0;
    while i < args.len() {
        if args[i] == flag {
            found = true;
            args.remove(i);
            continue;
        }
        i += 1;
    }
    (found, args)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_usage_errors_as_2() {
        assert_eq!(classify_error_code("未知命令: x"), 2);
        assert_eq!(classify_error_code("Unknown command: x"), 2);
        assert_eq!(classify_error_code("缺少参数 --tool"), 2);
        assert_eq!(classify_error_code("Missing argument --tool"), 2);
        assert_eq!(classify_error_code("不支持的参数: [\"--x\"]"), 2);
        assert_eq!(classify_error_code("Unsupported argument: [\"--x\"]"), 2);
    }

    #[test]
    fn classifies_runtime_errors_as_1() {
        assert_eq!(
            classify_error_code("写入文件失败: /x: permission denied"),
            1
        );
    }

    #[test]
    fn takes_global_verbose_flag() {
        let (v, rest) = take_global_flag(vec!["--verbose".into(), "doctor".into()], "--verbose");
        assert!(v);
        assert_eq!(rest, vec!["doctor".to_string()]);
    }
}
