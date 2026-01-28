use std::env;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Supported external tools Prismctl can manage.
pub enum Tool {
    Codex,
    ClaudeCode,
    GeminiCli,
}

#[derive(Debug, Clone)]
/// Represents the "logical HOME" Prismctl reads from and writes to.
///
/// This can be redirected via `--home` or `PRISMCTL_HOME` to safely operate in a sandbox.
pub struct PrismctlHome {
    home_dir: PathBuf,
}

impl PrismctlHome {
    /// Create an `PrismctlHome` from an explicit directory.
    pub fn new(home_dir: PathBuf) -> Self {
        Self { home_dir }
    }

    /// Resolve the effective HOME directory for Prismctl.
    ///
    /// Priority: CLI `--home` > `PRISMCTL_HOME` > `HOME` > `USERPROFILE`.
    pub fn discover(cli_home: Option<PathBuf>) -> Result<Self, String> {
        if let Some(home) = cli_home {
            return Ok(Self::new(home));
        }

        if let Some(home) = env::var_os("PRISMCTL_HOME") {
            return Ok(Self::new(PathBuf::from(home)));
        }

        if let Some(home) = env::var_os("HOME") {
            return Ok(Self::new(PathBuf::from(home)));
        }

        if let Some(home) = env::var_os("USERPROFILE") {
            return Ok(Self::new(PathBuf::from(home)));
        }

        Err("无法确定 HOME：请传入 --home 或设置 PRISMCTL_HOME".to_string())
    }

    /// Return the resolved HOME directory used by Prismctl.
    pub fn home_dir(&self) -> &Path {
        &self.home_dir
    }

    /// Return the tool-specific root directory under the resolved HOME.
    pub fn tool_root(&self, tool: Tool) -> PathBuf {
        match tool {
            Tool::Codex => self.home_dir.join(".codex"),
            Tool::ClaudeCode => self.home_dir.join(".claude"),
            Tool::GeminiCli => self.home_dir.join(".gemini"),
        }
    }

    /// Path to Claude Code's `settings.json` under the resolved HOME.
    pub fn claude_settings_path(&self) -> PathBuf {
        self.tool_root(Tool::ClaudeCode).join("settings.json")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tool_roots_are_under_home() {
        let home = PrismctlHome::new(PathBuf::from("/tmp/prismctl-home"));
        assert_eq!(
            home.tool_root(Tool::Codex),
            PathBuf::from("/tmp/prismctl-home/.codex")
        );
        assert_eq!(
            home.tool_root(Tool::ClaudeCode),
            PathBuf::from("/tmp/prismctl-home/.claude")
        );
        assert_eq!(
            home.tool_root(Tool::GeminiCli),
            PathBuf::from("/tmp/prismctl-home/.gemini")
        );
        assert_eq!(
            home.claude_settings_path(),
            PathBuf::from("/tmp/prismctl-home/.claude/settings.json")
        );
    }
}
