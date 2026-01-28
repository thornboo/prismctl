use crate::legacy;
use prismctl_i18n::{current_locale, keys, tf, Locale};
use std::path::PathBuf;

pub fn quick_update(mut args: Vec<String>) -> Result<(), String> {
    let mut silent = false;
    let mut tool: Option<String> = None;
    let mut lang: Option<String> = None;
    let mut home: Option<PathBuf> = None;
    let mut apply = false;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "-s" | "--silent" => {
                silent = true;
                args.remove(i);
            }
            "-t" | "--tool" => {
                tool = Some(
                    args.get(i + 1)
                        .ok_or_else(|| {
                            crate::errors::usage(tf!(
                                keys::ERROR_FLAG_MISSING_VALUE,
                                "flag" => "--tool"
                            ))
                        })?
                        .to_string(),
                );
                args.drain(i..=i + 1);
            }
            "--lang" => {
                lang = Some(
                    args.get(i + 1)
                        .ok_or_else(|| {
                            crate::errors::usage(tf!(
                                keys::ERROR_FLAG_MISSING_VALUE,
                                "flag" => "--lang"
                            ))
                        })?
                        .to_string(),
                );
                args.drain(i..=i + 1);
            }
            "--home" => {
                home = Some(PathBuf::from(
                    args.get(i + 1)
                        .ok_or_else(|| {
                            crate::errors::usage(tf!(
                                keys::ERROR_FLAG_MISSING_VALUE,
                                "flag" => "--home"
                            ))
                        })?
                        .to_string(),
                ));
                args.drain(i..=i + 1);
            }
            "--apply" => {
                apply = true;
                args.remove(i);
            }
            "--dry-run" => {
                apply = false;
                args.remove(i);
            }
            "-h" | "--help" | "help" => {
                return Err(crate::errors::usage(help_quick_update()));
            }
            _ => i += 1,
        }
    }

    if silent {
        apply = true;
    }

    let tool = tool.unwrap_or_else(|| "all".to_string());
    let lang = lang.unwrap_or_else(|| current_locale().as_str().to_string());

    let mut update_args = vec!["--tool".to_string(), tool, "--lang".to_string(), lang];
    if let Some(h) = &home {
        update_args.push("--home".to_string());
        update_args.push(h.display().to_string());
    }
    if apply {
        update_args.push("--apply".to_string());
    }

    legacy::cmd_update(update_args)
}

fn help_quick_update() -> String {
    match current_locale() {
        Locale::ZhCN => [
            "prismctl u（quick update）",
            "",
            "用法：",
            "  prismctl u [-t|--tool <claude|codex|gemini|all>] [--lang <zh-CN|en>] [--home <PATH>] [--dry-run|--apply] [-s|--silent]",
            "",
            "说明：",
            "  - 默认 dry-run；传入 --apply 才会真正写入。",
            "  - --silent 会自动开启 --apply（适合脚本/CI）。",
        ]
        .join("\n"),
        Locale::En => [
            "prismctl u (quick update)",
            "",
            "Usage:",
            "  prismctl u [-t|--tool <claude|codex|gemini|all>] [--lang <zh-CN|en>] [--home <PATH>] [--dry-run|--apply] [-s|--silent]",
            "",
            "Notes:",
            "  - Defaults to dry-run; pass --apply to write.",
            "  - --silent implies --apply (script/CI friendly).",
        ]
        .join("\n"),
    }
}
