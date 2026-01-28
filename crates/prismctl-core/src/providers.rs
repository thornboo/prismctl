#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Supported provider presets for Codex configuration.
pub enum Provider {
    OpenRouter,
    DeepSeek,
    Ollama,
    Volcengine,
    SiliconFlow,
}

impl Provider {
    pub fn id(&self) -> &'static str {
        match self {
            Provider::OpenRouter => "openrouter",
            Provider::DeepSeek => "deepseek",
            Provider::Ollama => "ollama",
            Provider::Volcengine => "volcengine",
            Provider::SiliconFlow => "siliconflow",
        }
    }
}

pub fn parse_provider_id(id: &str) -> Result<Provider, String> {
    match id.trim().to_ascii_lowercase().as_str() {
        "openrouter" => Ok(Provider::OpenRouter),
        "deepseek" => Ok(Provider::DeepSeek),
        "ollama" => Ok(Provider::Ollama),
        "volcengine" => Ok(Provider::Volcengine),
        "siliconflow" => Ok(Provider::SiliconFlow),
        _ => Err(format!(
            "未知 provider: {}（可用: {}）",
            id,
            list_provider_ids().join(", ")
        )),
    }
}

/// List all supported provider IDs.
pub fn list_provider_ids() -> Vec<&'static str> {
    vec![
        "openrouter",
        "deepseek",
        "ollama",
        "volcengine",
        "siliconflow",
    ]
}

pub struct CodexProviderPreset {
    pub base_url: &'static str,
    pub wire_api: &'static str,
    pub default_model: &'static str,
}

/// Return the Codex preset values for a provider.
pub fn codex_preset(provider: Provider) -> CodexProviderPreset {
    match provider {
        Provider::OpenRouter => CodexProviderPreset {
            base_url: "https://openrouter.ai/api/v1",
            wire_api: "chat",
            default_model: "google/gemini-2.5-pro-preview",
        },
        Provider::DeepSeek => CodexProviderPreset {
            base_url: "https://api.deepseek.com/v1",
            wire_api: "chat",
            default_model: "deepseek-chat",
        },
        Provider::Ollama => CodexProviderPreset {
            base_url: "http://localhost:11434/v1",
            wire_api: "chat",
            default_model: "qwen2.5-coder:latest",
        },
        Provider::Volcengine => CodexProviderPreset {
            base_url: "https://ark.cn-beijing.volces.com/api/v3",
            wire_api: "chat",
            default_model: "deepseek-v3-250324",
        },
        Provider::SiliconFlow => CodexProviderPreset {
            base_url: "https://api.siliconflow.cn/v1",
            wire_api: "chat",
            default_model: "moonshotai/Kimi-K2-Instruct",
        },
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedCodexProvider {
    pub base_url: String,
    pub wire_api: String,
    pub model: String,
}

/// Resolve a Codex provider config using an optional preset plus optional explicit overrides.
pub fn resolve_codex_provider(
    provider: Option<Provider>,
    base_url: Option<String>,
    wire_api: Option<String>,
    model: Option<String>,
) -> ResolvedCodexProvider {
    let preset = provider.map(codex_preset);

    let base_url = base_url
        .or_else(|| preset.as_ref().map(|p| p.base_url.to_string()))
        .unwrap_or_else(|| "https://api.openai.com/v1".to_string());
    let wire_api = wire_api
        .or_else(|| preset.as_ref().map(|p| p.wire_api.to_string()))
        .unwrap_or_else(|| "openai".to_string());
    let model = model
        .or_else(|| preset.as_ref().map(|p| p.default_model.to_string()))
        .unwrap_or_else(|| "gpt-5".to_string());

    ResolvedCodexProvider {
        base_url,
        wire_api,
        model,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_provider_id_accepts_known_ids() {
        assert_eq!(
            parse_provider_id("openrouter").unwrap(),
            Provider::OpenRouter
        );
        assert_eq!(parse_provider_id("DeepSeek").unwrap(), Provider::DeepSeek);
        assert_eq!(parse_provider_id("OLLAMA").unwrap(), Provider::Ollama);
    }

    #[test]
    fn resolve_codex_provider_prefers_explicit_values_over_preset() {
        let resolved = resolve_codex_provider(
            Some(Provider::OpenRouter),
            Some("https://example.com/v1".into()),
            Some("openai".into()),
            Some("gpt-5".into()),
        );
        assert_eq!(resolved.base_url, "https://example.com/v1");
        assert_eq!(resolved.wire_api, "openai");
        assert_eq!(resolved.model, "gpt-5");
    }

    #[test]
    fn resolve_codex_provider_uses_preset_when_fields_missing() {
        let resolved = resolve_codex_provider(Some(Provider::DeepSeek), None, None, None);
        assert_eq!(resolved.base_url, "https://api.deepseek.com/v1");
        assert_eq!(resolved.wire_api, "chat");
        assert_eq!(resolved.model, "deepseek-chat");
    }
}
