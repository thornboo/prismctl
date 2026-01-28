use crate::Locale;
use std::collections::HashMap;

pub struct SimpleBackend {
    locale: Locale,
    translations: HashMap<String, String>,
}

impl SimpleBackend {
    pub fn new(locale: Locale) -> Self {
        let content = Self::load_translations(locale);
        Self {
            locale,
            translations: Self::parse_ftl(&content),
        }
    }

    fn load_translations(locale: Locale) -> String {
        match locale {
            Locale::ZhCN => {
                let mut content = String::new();
                content.push_str(include_str!("../i18n/zh-CN/common.ftl"));
                content.push('\n');
                content.push_str(include_str!("../i18n/zh-CN/menu.ftl"));
                content.push('\n');
                content.push_str(include_str!("../i18n/zh-CN/wizard.ftl"));
                content.push('\n');
                content.push_str(include_str!("../i18n/zh-CN/cli.ftl"));
                content
            }
            Locale::En => {
                let mut content = String::new();
                content.push_str(include_str!("../i18n/en/common.ftl"));
                content.push('\n');
                content.push_str(include_str!("../i18n/en/menu.ftl"));
                content.push('\n');
                content.push_str(include_str!("../i18n/en/wizard.ftl"));
                content.push('\n');
                content.push_str(include_str!("../i18n/en/cli.ftl"));
                content
            }
        }
    }

    fn parse_ftl(content: &str) -> HashMap<String, String> {
        content
            .lines()
            .filter_map(|line| {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') {
                    return None;
                }
                let mut parts = line.splitn(2, '=');
                let key = parts.next()?.trim();
                let value = parts.next()?.trim();
                Some((key.to_string(), value.to_string()))
            })
            .collect()
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.translations.get(key).map(|s| s.as_str())
    }

    pub fn format(&self, key: &str, args: &[(&str, &dyn std::fmt::Display)]) -> String {
        let template = self.get(key).unwrap_or(key);
        let mut result = template.to_string();

        for (name, value) in args {
            let placeholder = format!("{{ ${} }}", name);
            result = result.replace(&placeholder, &value.to_string());
        }

        result
    }

    pub fn locale(&self) -> Locale {
        self.locale
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ftl() {
        let content = r#"
# Comment
key1 = value1
key2 = value with spaces

# Another comment
key3 = value { $param }
"#;
        let translations = SimpleBackend::parse_ftl(content);
        assert_eq!(translations.get("key1"), Some(&"value1".to_string()));
        assert_eq!(
            translations.get("key2"),
            Some(&"value with spaces".to_string())
        );
        assert_eq!(
            translations.get("key3"),
            Some(&"value { $param }".to_string())
        );
    }

    #[test]
    fn test_format() {
        let backend = SimpleBackend::new(Locale::En);
        let result = backend.format("error-invalid-choice", &[("choice", &"abc")]);
        assert!(result.contains("abc"));
    }
}
