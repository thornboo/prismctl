use crate::json_text;
use crate::toml_text::{self, TomlScalar};

#[derive(Debug, Clone)]
/// A fully-resolved Codex provider configuration to be written into `config.toml`.
pub struct CodexProviderConfig {
    pub provider_id: String, // e.g. "ekko"
    pub display_name: String,
    pub base_url: String,
    pub wire_api: String,
    pub temp_env_key: String, // key name inside auth.json
    pub requires_openai_auth: bool,
    pub model: String,
}

/// Upsert an Ekko provider entry into Codex `config.toml` content (as text).
pub fn upsert_codex_provider_in_config_toml(
    content: &str,
    cfg: &CodexProviderConfig,
    set_default_provider: bool,
) -> String {
    let mut out = content.to_string();
    if set_default_provider {
        out = toml_text::upsert_root_key(
            &out,
            "model_provider",
            TomlScalar::Str(cfg.provider_id.clone()),
        );
    }

    let header = format!("model_providers.{}", cfg.provider_id);
    let kv = vec![
        (
            "name".to_string(),
            TomlScalar::Str(cfg.display_name.clone()),
        ),
        (
            "base_url".to_string(),
            TomlScalar::Str(cfg.base_url.clone()),
        ),
        (
            "wire_api".to_string(),
            TomlScalar::Str(cfg.wire_api.clone()),
        ),
        (
            "temp_env_key".to_string(),
            TomlScalar::Str(cfg.temp_env_key.clone()),
        ),
        (
            "requires_openai_auth".to_string(),
            TomlScalar::Bool(cfg.requires_openai_auth),
        ),
        ("model".to_string(), TomlScalar::Str(cfg.model.clone())),
    ];
    toml_text::upsert_table_kv(&out, &header, &kv)
}

/// Upsert an API key into Codex `auth.json` content.
pub fn upsert_codex_api_key_in_auth_json(
    content: &str,
    temp_env_key: &str,
    api_key: &str,
) -> Result<String, String> {
    json_text::upsert_string_property(content, temp_env_key, api_key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inserts_provider_table() {
        let cfg = CodexProviderConfig {
            provider_id: "ekko".into(),
            display_name: "Ekko".into(),
            base_url: "https://example.com".into(),
            wire_api: "openai".into(),
            temp_env_key: "EKKO_CODEX_API_KEY".into(),
            requires_openai_auth: false,
            model: "gpt-5".into(),
        };

        let out = upsert_codex_provider_in_config_toml("", &cfg, true);
        assert!(out.contains("model_provider = \"ekko\""));
        assert!(out.contains("[model_providers.ekko]"));
        assert!(out.contains("base_url = \"https://example.com\""));
    }

    #[test]
    fn upserts_auth_key() {
        let out =
            upsert_codex_api_key_in_auth_json("{}", "EKKO_CODEX_API_KEY", "sk-x").expect("ok");
        assert!(out.contains("\"EKKO_CODEX_API_KEY\": \"sk-x\""));
    }
}
