mod app_config;
mod errors;
mod icons;
mod interactive;
mod legacy;
mod prompted;
mod quick;

use ekko_i18n::{keys, t, tf, Locale};
use errors::ErrorKind;
use std::env;
use std::io;
use std::io::IsTerminal;

fn main() {
    match run() {
        Ok(()) => {}
        Err(f) => {
            let (_, clean) = errors::strip_tag(&f.message);
            eprintln!("{}", clean);
            std::process::exit(f.code);
        }
    }
}

#[derive(Debug)]
struct CliFailure {
    message: String,
    code: i32,
}

fn run() -> Result<(), CliFailure> {
    app_config::apply_saved_locale_best_effort();

    let mut args: Vec<String> = env::args().skip(1).collect();

    // Global `--lang` only applies when placed before the command, to avoid stealing
    // subcommand flags like `ekko init ... --lang en` (template language).
    let lang_override = take_leading_kv_flag(&mut args, "--lang")?;
    if env::var_os("EKKO_LANG").is_none() {
        if let Some(lang) = lang_override {
            let locale = Locale::parse(&lang).ok_or_else(|| CliFailure {
                message: errors::usage(format!(
                    "{}\n\n{}",
                    t!(keys::ERROR_LANG_FLAG_INVALID),
                    legacy::help()
                )),
                code: 2,
            })?;
            ekko_i18n::set_locale(locale);
        }
    }

    let (verbose, args) = take_global_flag(args, "--verbose");
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
                message: errors::usage(format!(
                    "{}\n\n{}",
                    tf!(keys::ERROR_UNKNOWN_COMMAND, "cmd" => cmd),
                    legacy::help()
                )),
                code: 2,
            });
        }
    };

    res.map_err(|message| {
        let (kind, clean) = errors::strip_tag(&message);
        let code = match kind {
            Some(ErrorKind::Usage) => 2,
            Some(ErrorKind::Runtime) => 1,
            None => classify_error_code(clean),
        };
        let message = if verbose {
            format!(
                "{}\n\n[verbose] cmd={} args={}",
                clean,
                cmd,
                args_dbg.unwrap_or_else(|| "[]".to_string())
            )
        } else {
            clean.to_string()
        };

        CliFailure { code, message }
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
        message: errors::usage(non_tty_interactive_error(invocation)),
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
    // Fallback only: prefer explicit tags via `crate::errors::usage/runtime`.
    // Heuristic: if help text is present, it's almost always a usage error.
    if message.contains("Usage:") || message.contains("用法:") {
        2
    } else {
        1
    }
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

fn take_leading_kv_flag(args: &mut Vec<String>, flag: &str) -> Result<Option<String>, CliFailure> {
    if args.first().map(|s| s.as_str()) != Some(flag) {
        return Ok(None);
    }
    if args.len() < 2 {
        return Err(CliFailure {
            message: errors::usage(format!(
                "{}\n\n{}",
                t!(keys::ERROR_LANG_FLAG_INVALID),
                legacy::help()
            )),
            code: 2,
        });
    }
    let value = args.remove(1);
    args.remove(0);
    Ok(Some(value))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_usage_errors_as_2() {
        assert_eq!(classify_error_code("Usage:\n  ekko ..."), 2);
        assert_eq!(classify_error_code("用法:\n  ekko ..."), 2);
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

    #[test]
    fn takes_leading_lang_flag_only() {
        let mut args = vec!["--lang".into(), "en".into(), "--help".into()];
        let lang = take_leading_kv_flag(&mut args, "--lang").unwrap();
        assert_eq!(lang.as_deref(), Some("en"));
        assert_eq!(args, vec!["--help".to_string()]);

        let mut args = vec!["init".into(), "--lang".into(), "en".into()];
        let lang = take_leading_kv_flag(&mut args, "--lang").unwrap();
        assert_eq!(lang, None);
        assert_eq!(
            args,
            vec!["init".to_string(), "--lang".to_string(), "en".to_string()]
        );
    }
}
