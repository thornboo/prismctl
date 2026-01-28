use std::env;
use std::io;
use std::io::IsTerminal;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct UiStyle {
    pub use_color: bool,
}

#[allow(dead_code)]
impl UiStyle {
    pub fn detect() -> Self {
        let use_color = io::stdout().is_terminal()
            && env::var_os("NO_COLOR").is_none()
            && env::var("TERM").ok().as_deref() != Some("dumb");

        Self { use_color }
    }

    pub fn ok(&self) -> &'static str {
        "OK"
    }
    pub fn warn(&self) -> &'static str {
        "WARN"
    }
    pub fn info(&self) -> &'static str {
        "INFO"
    }
    pub fn err(&self) -> &'static str {
        "ERR"
    }

    pub fn green(&self, s: &str) -> String {
        self.colorize("32", s)
    }

    pub fn yellow(&self, s: &str) -> String {
        self.colorize("33", s)
    }

    pub fn blue(&self, s: &str) -> String {
        self.colorize("34", s)
    }

    pub fn red(&self, s: &str) -> String {
        self.colorize("31", s)
    }

    fn colorize(&self, code: &str, s: &str) -> String {
        if !self.use_color {
            return s.to_string();
        }
        format!("\u{001b}[{}m{}\u{001b}[0m", code, s)
    }
}
