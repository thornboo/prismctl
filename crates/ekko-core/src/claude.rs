use crate::json_text;

#[derive(Debug, Default, Clone)]
/// A patch describing which Claude Code environment variables should be upserted.
pub struct ClaudeEnvPatch {
    pub auth_token: Option<String>,
    pub base_url: Option<String>,
    pub model: Option<String>,
    pub default_haiku_model: Option<String>,
    pub default_sonnet_model: Option<String>,
    pub default_opus_model: Option<String>,
}

/// Apply a `ClaudeEnvPatch` to the given `settings.json` content (as text).
pub fn apply_claude_env_patch_to_settings_json(
    content: &str,
    patch: &ClaudeEnvPatch,
) -> Result<String, String> {
    let mut out = content.to_string();

    if let Some(v) = &patch.auth_token {
        out = json_text::upsert_string_map_entry(&out, "env", "ANTHROPIC_AUTH_TOKEN", v)?;
    }
    if let Some(v) = &patch.base_url {
        out = json_text::upsert_string_map_entry(&out, "env", "ANTHROPIC_BASE_URL", v)?;
    }
    if let Some(v) = &patch.model {
        out = json_text::upsert_string_map_entry(&out, "env", "ANTHROPIC_MODEL", v)?;
    }
    if let Some(v) = &patch.default_haiku_model {
        out = json_text::upsert_string_map_entry(&out, "env", "ANTHROPIC_DEFAULT_HAIKU_MODEL", v)?;
    }
    if let Some(v) = &patch.default_sonnet_model {
        out = json_text::upsert_string_map_entry(&out, "env", "ANTHROPIC_DEFAULT_SONNET_MODEL", v)?;
    }
    if let Some(v) = &patch.default_opus_model {
        out = json_text::upsert_string_map_entry(&out, "env", "ANTHROPIC_DEFAULT_OPUS_MODEL", v)?;
    }

    Ok(out)
}

/// Set `outputStyle` in Claude Code `settings.json` content.
pub fn set_claude_output_style_in_settings_json(
    content: &str,
    name: &str,
) -> Result<String, String> {
    json_text::upsert_string_property(content, "outputStyle", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn env_patch_sets_keys() {
        let patch = ClaudeEnvPatch {
            auth_token: Some("t".into()),
            base_url: Some("u".into()),
            model: Some("m".into()),
            default_haiku_model: None,
            default_sonnet_model: None,
            default_opus_model: None,
        };
        let out = apply_claude_env_patch_to_settings_json("", &patch).expect("ok");
        assert!(out.contains("\"ANTHROPIC_AUTH_TOKEN\": \"t\""));
        assert!(out.contains("\"ANTHROPIC_BASE_URL\": \"u\""));
        assert!(out.contains("\"ANTHROPIC_MODEL\": \"m\""));
    }

    #[test]
    fn output_style_sets_value() {
        let out =
            set_claude_output_style_in_settings_json("{}", "engineer-professional").expect("ok");
        assert!(out.contains("\"outputStyle\": \"engineer-professional\""));
    }
}
