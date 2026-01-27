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

    let content = render_config(locale);
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
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Minimal TOML parser for a single `lang = "..."` key.
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

        return Locale::parse(v);
    }
    None
}

fn render_config(locale: Locale) -> String {
    // Keep it simple and hand-editable.
    format!(
        "# Ekko CLI config\n\
         #\n\
         # This file is safe to edit manually.\n\
         lang = \"{}\"\n",
        locale.as_str()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_lang_from_minimal_toml() {
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
        assert_eq!(parse_locale_from_toml("lang = \"unknown\""), None);
    }
}
