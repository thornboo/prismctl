use ekko_i18n::{keys, t, tf, Locale};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const CONFIG_DIR: &str = ".ekko";
const CONFIG_FILE: &str = "config.toml";

pub fn apply_saved_locale_best_effort() {
    // Explicit env always wins; don't override it with config.
    if env::var_os("EKKO_LANG").is_some() {
        return;
    }

    if let Some(locale) = load_locale_best_effort() {
        ekko_i18n::set_locale(locale);
    }
}

pub fn save_locale(locale: Locale) -> Result<(), String> {
    let path = config_path()?;
    ensure_parent_dir(&path)?;

    let existing = fs::read_to_string(&path).unwrap_or_default();
    let content = if existing.trim().is_empty() {
        render_config(locale)
    } else {
        upsert_cli_lang(&existing, locale)
    };
    fs::write(&path, content.as_bytes())
        .map_err(|e| tf!(keys::ERROR_CONFIG_WRITE, "path" => path.display(), "error" => e))
}

fn config_path() -> Result<PathBuf, String> {
    let home = discover_home_dir()?;
    Ok(home.join(CONFIG_DIR).join(CONFIG_FILE))
}

fn ensure_parent_dir(path: &Path) -> Result<(), String> {
    let Some(dir) = path.parent() else {
        return Ok(());
    };

    fs::create_dir_all(dir)
        .map_err(|e| tf!(keys::ERROR_CONFIG_DIR_CREATE, "path" => dir.display(), "error" => e))
}

fn discover_home_dir() -> Result<PathBuf, String> {
    if let Some(home) = env::var_os("EKKO_HOME") {
        return Ok(PathBuf::from(home));
    }
    if let Some(home) = env::var_os("HOME") {
        return Ok(PathBuf::from(home));
    }
    if let Some(home) = env::var_os("USERPROFILE") {
        return Ok(PathBuf::from(home));
    }

    Err(t!(keys::ERROR_HOME_NOT_FOUND))
}

fn load_locale_best_effort() -> Option<Locale> {
    let path = config_path().ok()?;
    let content = fs::read_to_string(path).ok()?;
    parse_locale_from_toml(&content)
}

fn parse_locale_from_toml(content: &str) -> Option<Locale> {
    // Minimal TOML parser for:
    // - `[cli]\nlang = "..."` (preferred)
    // - legacy: `lang = "..."` at top-level (backward compatible)
    let mut in_cli = false;
    let mut saw_any_section = false;
    let mut top_level: Option<Locale> = None;
    let mut cli_level: Option<Locale> = None;

    for raw in content.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if line.starts_with('[') && line.ends_with(']') {
            saw_any_section = true;
            let section = line.trim_matches(&['[', ']'][..]).trim();
            in_cli = section == "cli";
            continue;
        }

        let Some((k, v)) = line.split_once('=') else {
            continue;
        };
        if k.trim() != "lang" {
            continue;
        }

        let v = v.trim();
        let v = v
            .strip_prefix('"')
            .and_then(|s| s.strip_suffix('"'))
            .or_else(|| v.strip_prefix('\'').and_then(|s| s.strip_suffix('\'')))
            .unwrap_or(v);

        let parsed = Locale::parse(v);
        if saw_any_section {
            if in_cli {
                cli_level = parsed;
            }
        } else {
            top_level = parsed;
        }
    }

    cli_level.or(top_level)
}

fn render_config(locale: Locale) -> String {
    // Keep it simple and hand-editable.
    format!(
        "# Ekko CLI config\n\
         #\n\
         # This file is safe to edit manually.\n\
         \n\
         [cli]\n\
         lang = \"{}\"\n",
        locale.as_str()
    )
}

fn upsert_cli_lang(existing: &str, locale: Locale) -> String {
    let mut out: Vec<String> = Vec::new();

    let mut in_cli = false;
    let mut cli_found = false;
    let mut cli_lang_written = false;

    for raw in existing.lines() {
        let line = raw.to_string();
        let trimmed = raw.trim();

        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            if in_cli && cli_found && !cli_lang_written {
                out.push(format!("lang = \"{}\"", locale.as_str()));
                cli_lang_written = true;
            }

            let section = trimmed.trim_matches(&['[', ']'][..]).trim();
            in_cli = section == "cli";
            if in_cli {
                cli_found = true;
            }

            out.push(line);
            continue;
        }

        if in_cli && trimmed.contains('=') {
            let key = trimmed.split('=').next().unwrap_or("").trim();
            if key == "lang" {
                out.push(format!("lang = \"{}\"", locale.as_str()));
                cli_lang_written = true;
                continue;
            }
        }

        out.push(line);
    }

    if in_cli && cli_found && !cli_lang_written {
        out.push(format!("lang = \"{}\"", locale.as_str()));
    }

    if !cli_found {
        // Preserve original file; just append a new `[cli]` section.
        if !out.last().map(|l| l.is_empty()).unwrap_or(false) {
            out.push(String::new());
        }
        out.push("[cli]".to_string());
        out.push(format!("lang = \"{}\"", locale.as_str()));
    }

    let mut s = out.join("\n");
    if !s.ends_with('\n') {
        s.push('\n');
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_lang_from_minimal_toml() {
        // preferred
        assert_eq!(
            parse_locale_from_toml("[cli]\nlang = \"en\""),
            Some(Locale::En)
        );
        assert_eq!(
            parse_locale_from_toml("[cli]\nlang=\"zh-CN\""),
            Some(Locale::ZhCN)
        );
        assert_eq!(
            parse_locale_from_toml("[cli]\nlang = 'zh'"),
            Some(Locale::ZhCN)
        );

        // backward compatible
        assert_eq!(parse_locale_from_toml("lang = \"en\""), Some(Locale::En));
        assert_eq!(parse_locale_from_toml("lang=\"zh-CN\""), Some(Locale::ZhCN));
        assert_eq!(parse_locale_from_toml("lang = 'zh'"), Some(Locale::ZhCN));
        assert_eq!(
            parse_locale_from_toml("# c\nlang = \"en\""),
            Some(Locale::En)
        );
        assert_eq!(
            parse_locale_from_toml("x = 1\nlang = \"en\""),
            Some(Locale::En)
        );

        // prefer `[cli]` when present
        assert_eq!(
            parse_locale_from_toml("[other]\nlang = \"zh-CN\"\n[cli]\nlang=\"en\""),
            Some(Locale::En)
        );

        assert_eq!(parse_locale_from_toml("lang = \"unknown\""), None);
    }

    #[test]
    fn upserts_cli_lang_preserving_other_content() {
        let updated = upsert_cli_lang("[cli]\nlang = \"en\"\n", Locale::ZhCN);
        assert!(updated.contains("[cli]"));
        assert!(updated.contains("lang = \"zh-CN\""));

        let updated = upsert_cli_lang("[other]\nx=1\n", Locale::En);
        assert!(updated.contains("[other]"));
        assert!(updated.contains("x=1"));
        assert!(updated.contains("[cli]"));
        assert!(updated.contains("lang = \"en\""));
    }
}
