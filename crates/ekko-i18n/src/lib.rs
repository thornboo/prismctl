mod simple;

use std::sync::OnceLock;
use std::sync::RwLock;

// Include generated keys.
include!(concat!(env!("OUT_DIR"), "/generated_keys.rs"));

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Locale {
    ZhCN,
    En,
}

impl Locale {
    pub fn detect() -> Self {
        // 1) EKKO_LANG
        if let Ok(lang) = std::env::var("EKKO_LANG") {
            return Self::from_str(&lang);
        }

        // 2) LANG
        if let Ok(lang) = std::env::var("LANG") {
            if lang.to_ascii_lowercase().starts_with("zh") {
                return Self::ZhCN;
            }
        }

        // 3) default
        Self::En
    }

    fn from_str(s: &str) -> Self {
        match s.trim().to_ascii_lowercase().as_str() {
            "zh" | "zh-cn" | "zh_cn" | "chinese" => Self::ZhCN,
            "en" | "en-us" | "en_us" | "english" => Self::En,
            _ => Self::En,
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s.trim().to_ascii_lowercase().as_str() {
            "zh" | "zh-cn" | "zh_cn" | "chinese" => Some(Self::ZhCN),
            "en" | "en-us" | "en_us" | "english" => Some(Self::En),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ZhCN => "zh-CN",
            Self::En => "en",
        }
    }
}

pub struct I18n {
    backend: simple::SimpleBackend,
}

impl I18n {
    pub fn new() -> Self {
        Self::with_locale(Locale::detect())
    }

    pub fn with_locale(locale: Locale) -> Self {
        Self {
            backend: simple::SimpleBackend::new(locale),
        }
    }

    pub fn t(&self, key: &str) -> String {
        self.backend
            .get(key)
            .map(|s| s.to_string())
            .unwrap_or_else(|| key.to_string())
    }

    pub fn tf(&self, key: &str, args: &[(&str, &dyn std::fmt::Display)]) -> String {
        self.backend.format(key, args)
    }

    pub fn locale(&self) -> Locale {
        self.backend.locale()
    }
}

impl Default for I18n {
    fn default() -> Self {
        Self::new()
    }
}

static I18N: OnceLock<RwLock<I18n>> = OnceLock::new();

pub fn i18n() -> &'static RwLock<I18n> {
    I18N.get_or_init(|| RwLock::new(I18n::new()))
}

pub fn set_locale(locale: Locale) {
    let lock = i18n();
    if let Ok(mut g) = lock.write() {
        *g = I18n::with_locale(locale);
    }
}

pub fn current_locale() -> Locale {
    i18n().read().map(|g| g.locale()).unwrap_or(Locale::En)
}

#[macro_export]
macro_rules! t {
    ($key:expr) => {
        $crate::i18n()
            .read()
            .map(|g| g.t($key))
            .unwrap_or_else(|_| $key.to_string())
    };
}

#[macro_export]
macro_rules! tf {
    ($key:expr, $($name:expr => $value:expr),* $(,)?) => {{
        let args: &[(&str, &dyn std::fmt::Display)] = &[$(($name, &$value)),*];
        $crate::i18n()
            .read()
            .map(|g| g.tf($key, args))
            .unwrap_or_else(|_| $key.to_string())
    }};
}
