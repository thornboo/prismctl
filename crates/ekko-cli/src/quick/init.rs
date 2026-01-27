use crate::interactive::utils::{prompt_line, validate_api_key_format};
use crate::legacy;
use ekko_core::providers;
use ekko_i18n::{current_locale, keys, t, tf, Locale};
use std::io;
use std::io::IsTerminal;
use std::path::PathBuf;

fn is_interactive_tty() -> bool {
    io::stdin().is_terminal() && io::stdout().is_terminal()
}

pub fn quick_init(mut args: Vec<String>) -> Result<(), String> {
    // Defaults keep the command usable with minimal flags.
    let mut silent = false;
    let mut provider: Option<String> = None;
    let mut api_key: Option<String> = None;
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
            "-p" | "--provider" => {
                provider = Some(
                    args.get(i + 1)
                        .ok_or_else(|| tf!(keys::ERROR_FLAG_MISSING_VALUE, "flag" => "--provider"))?
                        .to_string(),
                );
                args.drain(i..=i + 1);
            }
            "-k" | "--api-key" => {
                api_key = Some(
                    args.get(i + 1)
                        .ok_or_else(|| tf!(keys::ERROR_FLAG_MISSING_VALUE, "flag" => "--api-key"))?
                        .to_string(),
                );
                args.drain(i..=i + 1);
            }
            "-t" | "--tool" => {
                tool = Some(
                    args.get(i + 1)
                        .ok_or_else(|| tf!(keys::ERROR_FLAG_MISSING_VALUE, "flag" => "--tool"))?
                        .to_string(),
                );
                args.drain(i..=i + 1);
            }
            "--lang" => {
                lang = Some(
                    args.get(i + 1)
                        .ok_or_else(|| tf!(keys::ERROR_FLAG_MISSING_VALUE, "flag" => "--lang"))?
                        .to_string(),
                );
                args.drain(i..=i + 1);
            }
            "--home" => {
                home = Some(PathBuf::from(
                    args.get(i + 1)
                        .ok_or_else(|| tf!(keys::ERROR_FLAG_MISSING_VALUE, "flag" => "--home"))?
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
                return Err(help_quick_init());
            }
            _ => i += 1,
        }
    }

    let tool = tool.unwrap_or_else(|| "all".to_string());
    let lang = lang.unwrap_or_else(|| current_locale().as_str().to_string());

    // Provider selection is mainly for Codex; keep it optional.
    if provider.is_none() && !silent {
        provider = prompt_provider_if_tty()?;
    }

    // API key is only required when the user explicitly asks for provider-based config.
    if api_key.is_none() && !silent {
        api_key = prompt_api_key_if_tty()?;
    }
    if provider.is_some() && api_key.is_none() && !is_interactive_tty() {
        return Err(tf!(keys::ERROR_NON_TTY_MISSING_FLAG, "flag" => "--api-key"));
    }
    if let Some(k) = &api_key {
        validate_api_key_format(k)?;
    }

    // 1) init templates
    let mut init_args = vec![
        "--tool".to_string(),
        tool.clone(),
        "--lang".to_string(),
        lang.clone(),
    ];
    if let Some(h) = &home {
        init_args.push("--home".to_string());
        init_args.push(h.display().to_string());
    }
    if apply {
        init_args.push("--apply".to_string());
    }
    legacy::cmd_init(init_args)?;

    // 2) configure tools (best-effort, based on selected tool and provided flags)
    match tool.as_str() {
        "codex" | "all" => {
            if provider.is_some() || api_key.is_some() {
                let mut codex_args = vec!["provider".to_string(), "set".to_string()];
                if let Some(h) = &home {
                    codex_args.push("--home".to_string());
                    codex_args.push(h.display().to_string());
                }
                if let Some(p) = &provider {
                    // Validate provider early in hybrid mode.
                    providers::parse_provider_id(p)?;
                    codex_args.push("--provider".to_string());
                    codex_args.push(p.clone());
                    codex_args.push("--default".to_string());
                }
                if let Some(k) = &api_key {
                    codex_args.push("--api-key".to_string());
                    codex_args.push(k.clone());
                }
                if apply {
                    codex_args.push("--apply".to_string());
                }
                legacy::cmd_codex(codex_args)?;
            }
        }
        _ => {}
    }

    match tool.as_str() {
        "claude" | "all" => {
            if provider.is_some() || api_key.is_some() {
                let mut claude_args = vec!["env".to_string(), "set".to_string()];
                if let Some(h) = &home {
                    claude_args.push("--home".to_string());
                    claude_args.push(h.display().to_string());
                }
                if let Some(p) = &provider {
                    if let Some(url) = map_provider_to_base_url(p) {
                        claude_args.push("--base-url".to_string());
                        claude_args.push(url.to_string());
                    }
                }
                if let Some(k) = &api_key {
                    claude_args.push("--auth-token".to_string());
                    claude_args.push(k.clone());
                }
                if apply {
                    claude_args.push("--apply".to_string());
                }
                legacy::cmd_claude(claude_args)?;
            }
        }
        _ => {}
    }

    match tool.as_str() {
        "gemini" | "all" => {
            if api_key.is_some() {
                let mut gemini_args = vec!["env".to_string(), "set".to_string()];
                if let Some(h) = &home {
                    gemini_args.push("--home".to_string());
                    gemini_args.push(h.display().to_string());
                }
                if let Some(k) = &api_key {
                    gemini_args.push("--api-key".to_string());
                    gemini_args.push(k.clone());
                }
                if apply {
                    gemini_args.push("--apply".to_string());
                }
                legacy::cmd_gemini(gemini_args)?;
            }
        }
        _ => {}
    }

    Ok(())
}

fn prompt_provider_if_tty() -> Result<Option<String>, String> {
    if !is_interactive_tty() {
        return Ok(None);
    }
    println!("\n{}", t!(keys::QUICK_PROVIDER_PRESETS_TITLE));
    for (i, id) in providers::list_provider_ids().iter().enumerate() {
        println!("  {}) {}", i + 1, id);
    }
    println!("  {}", t!(keys::QUICK_SKIP));
    let prompt = format!("{} ", t!(keys::QUICK_PROMPT));
    let choice = prompt_line(&prompt)?;
    if choice.trim().is_empty() || choice.trim() == "0" {
        return Ok(None);
    }
    let idx: usize = choice
        .parse()
        .map_err(|_| tf!(keys::ERROR_INVALID_CHOICE, "choice" => choice))?;
    let ids = providers::list_provider_ids();
    let Some(id) = ids.get(idx - 1) else {
        return Err(tf!(keys::ERROR_INVALID_CHOICE, "choice" => choice));
    };
    Ok(Some((*id).to_string()))
}

fn prompt_api_key_if_tty() -> Result<Option<String>, String> {
    if is_interactive_tty() {
        let prompt = t!(keys::QUICK_API_KEY_PROMPT);
        let v = prompt_line(&prompt)?;
        if v.trim().is_empty() {
            Ok(None)
        } else {
            Ok(Some(v))
        }
    } else {
        Ok(None)
    }
}

fn help_quick_init() -> String {
    match current_locale() {
        Locale::ZhCN => [
            "ekko i（quick init）",
            "",
            "用法：",
            "  ekko i [-t|--tool <claude|codex|gemini|all>] [-p|--provider <NAME>] [-k|--api-key <KEY>] [--lang <zh-CN|en>] [--home <PATH>] [--dry-run|--apply] [-s|--silent]",
            "",
            "说明：",
            "  - 默认 dry-run；传入 --apply 才会真正写入。",
            "  - TTY 下缺少关键参数会提示补全；非 TTY 下缺少则直接报错。",
        ]
        .join("\n"),
        Locale::En => [
            "ekko i (quick init)",
            "",
            "Usage:",
            "  ekko i [-t|--tool <claude|codex|gemini|all>] [-p|--provider <NAME>] [-k|--api-key <KEY>] [--lang <zh-CN|en>] [--home <PATH>] [--dry-run|--apply] [-s|--silent]",
            "",
            "Notes:",
            "  - Defaults to dry-run; pass --apply to write.",
            "  - In TTY, missing key flags will be prompted; in non-TTY they error out.",
        ]
        .join("\n"),
    }
}

fn map_provider_to_base_url(provider: &str) -> Option<&'static str> {
    match provider.trim().to_ascii_lowercase().as_str() {
        "openrouter" => Some("https://openrouter.ai/api/v1"),
        "deepseek" => Some("https://api.deepseek.com/v1"),
        "ollama" => Some("http://localhost:11434/v1"),
        "volcengine" => Some("https://ark.cn-beijing.volces.com/api/v3"),
        "siliconflow" => Some("https://api.siliconflow.cn/v1"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_known_provider_to_base_url() {
        assert_eq!(
            map_provider_to_base_url("openrouter"),
            Some("https://openrouter.ai/api/v1")
        );
        assert_eq!(
            map_provider_to_base_url("DeepSeek"),
            Some("https://api.deepseek.com/v1")
        );
        assert_eq!(
            map_provider_to_base_url("ollama"),
            Some("http://localhost:11434/v1")
        );
        assert_eq!(map_provider_to_base_url("unknown"), None);
    }
}
