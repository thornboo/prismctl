use inquire::error::InquireError;
use inquire::validator::Validation;
use inquire::{Confirm, MultiSelect, Password, Select, Text};
use prismctl_i18n::{keys, t, tf};
use std::env;
use std::sync::Once;

#[cfg(unix)]
static SIGINT_INSTALLED: Once = Once::new();

fn select_page_size(item_count: usize) -> usize {
    // Try to show all options when the terminal is tall enough, otherwise fall back to scrolling.
    // Keep it heuristic-based: callers may have printed headers already.
    if item_count <= 1 {
        return item_count.max(1);
    }

    let Ok((_cols, rows)) = crossterm::terminal::size() else {
        return item_count;
    };

    let rows = rows as usize;
    // Reserve some rows for already-rendered content (banner), the prompt line, and help hints.
    // This is intentionally conservative to avoid overflowing small terminals.
    let available = rows.saturating_sub(12);
    let min_page = if item_count < 3 { item_count } else { 3 };
    available.max(min_page).min(item_count)
}

pub fn install_signal_handlers() {
    #[cfg(unix)]
    {
        SIGINT_INSTALLED.call_once(|| unsafe {
            // Best-effort Ctrl+C handling without external deps.
            // We only need to prevent process termination so `read_line` can return `Interrupted`.
            extern "C" fn handle_sigint(_: i32) {}

            extern "C" {
                fn signal(sig: i32, handler: extern "C" fn(i32)) -> extern "C" fn(i32);
            }

            const SIGINT: i32 = 2;
            let _ = signal(SIGINT, handle_sigint);
        });
    }
}

fn map_inquire_error(e: InquireError) -> String {
    match e {
        InquireError::OperationCanceled => t!(keys::ERROR_CANCELLED_ESC),
        InquireError::OperationInterrupted => t!(keys::ERROR_CANCELLED),
        other => tf!(keys::ERROR_INTERACTIVE_INPUT, "error" => other),
    }
}

pub fn prompt_line(prompt: &str) -> Result<String, String> {
    let out = Text::new(prompt)
        .prompt()
        .map_err(map_inquire_error)?
        .trim()
        .to_string();
    if out.eq_ignore_ascii_case("esc") {
        return Err(t!(keys::ERROR_CANCELLED_ESC));
    }
    Ok(out)
}

pub fn prompt_optional(prompt: &str) -> Result<Option<String>, String> {
    let v = prompt_line(prompt)?;
    if v.is_empty() {
        Ok(None)
    } else {
        Ok(Some(v))
    }
}

pub fn prompt_required(prompt: &str) -> Result<String, String> {
    Text::new(prompt)
        .with_validator(|input: &str| {
            if input.trim().is_empty() {
                Ok(Validation::Invalid(t!(keys::ERROR_EMPTY_INPUT).into()))
            } else {
                Ok(Validation::Valid)
            }
        })
        .prompt()
        .map_err(map_inquire_error)
        .map(|s| s.trim().to_string())
}

pub fn prompt_confirm(prompt: &str, default: bool) -> Result<bool, String> {
    let help = t!(keys::INQUIRE_HELP_CONFIRM);
    Confirm::new(prompt)
        .with_help_message(&help)
        .with_default(default)
        .prompt()
        .map_err(map_inquire_error)
}

pub fn prompt_select<T>(prompt: &str, options: Vec<T>, default_index: usize) -> Result<T, String>
where
    T: Clone + std::fmt::Display,
{
    let help = t!(keys::INQUIRE_HELP_SELECT);
    let page_size = select_page_size(options.len());
    Select::new(prompt, options)
        .with_help_message(&help)
        .with_starting_cursor(default_index)
        .with_page_size(page_size)
        .prompt()
        .map_err(map_inquire_error)
}

pub fn prompt_multi_select(
    prompt: &str,
    options: Vec<String>,
    default_indexes: Vec<usize>,
) -> Result<Vec<String>, String> {
    let help = t!(keys::INQUIRE_HELP_MULTI_SELECT);
    let page_size = select_page_size(options.len());
    MultiSelect::new(prompt, options)
        .with_help_message(&help)
        .with_default(&default_indexes)
        .with_page_size(page_size)
        .prompt()
        .map_err(map_inquire_error)
}

pub fn prompt_secret_with_env_default(
    prompt: &str,
    env_keys: &[&str],
) -> Result<Option<String>, String> {
    let mut detected: Option<String> = None;
    for k in env_keys {
        if let Ok(v) = env::var(k) {
            if !v.trim().is_empty() {
                detected = Some(v);
                break;
            }
        }
    }

    if detected.is_some() {
        println!("{}", t!(keys::INFO_ENV_DEFAULT_DETECTED));
    }

    let v = Password::new(prompt)
        .without_confirmation()
        .prompt()
        .map_err(map_inquire_error)?
        .trim()
        .to_string();

    if v == "-" {
        return Ok(None);
    }
    if v.is_empty() {
        return Ok(detected);
    }
    Ok(Some(v))
}

pub fn prompt_lang_selection() -> Result<String, String> {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum LangChoice {
        ZhCN,
        En,
    }

    impl std::fmt::Display for LangChoice {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let s = match self {
                Self::ZhCN => t!(keys::LANGUAGE_ZH_CN),
                Self::En => t!(keys::LANGUAGE_EN),
            };
            write!(f, "{}", s)
        }
    }

    let title = t!(keys::LANGUAGE_TITLE);
    let choice = prompt_select(&title, vec![LangChoice::ZhCN, LangChoice::En], 0)?;
    Ok(match choice {
        LangChoice::ZhCN => "zh-CN",
        LangChoice::En => "en",
    }
    .to_string())
}

pub fn validate_http_url(url: &str) -> Result<(), String> {
    let u = url.trim();
    if u.is_empty() {
        return Err(t!(keys::ERROR_URL_EMPTY));
    }
    if u.starts_with("http://") || u.starts_with("https://") {
        return Ok(());
    }
    Err(tf!(keys::ERROR_URL_INVALID, "url" => u))
}

pub fn validate_api_key_format(key: &str) -> Result<(), String> {
    let k = key.trim();
    if k.is_empty() {
        return Err(t!(keys::ERROR_API_KEY_EMPTY));
    }
    if k.chars().any(|c| c.is_whitespace()) {
        return Err(t!(keys::ERROR_API_KEY_WHITESPACE));
    }
    if k.len() < 8 {
        return Err(t!(keys::ERROR_API_KEY_TOO_SHORT));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_http_urls() {
        assert!(validate_http_url("https://example.com/v1").is_ok());
        assert!(validate_http_url("http://localhost:11434/v1").is_ok());
        assert!(validate_http_url("example.com").is_err());
        assert!(validate_http_url("").is_err());
    }

    #[test]
    fn validates_api_key_format() {
        assert!(validate_api_key_format("sk-12345678").is_ok());
        assert!(validate_api_key_format("  ").is_err());
        assert!(validate_api_key_format("short").is_err());
        assert!(validate_api_key_format("has space").is_err());
    }
}
