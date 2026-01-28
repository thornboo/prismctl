use crate::changeset::{Change, ChangeSet};
use std::env;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// How Prismctl should install/upgrade external CLI tools.
pub enum InstallMethod {
    Auto,
    Npm,
    Brew,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Whether the plan is an initial install or an upgrade.
pub enum InstallAction {
    Install,
    Upgrade,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// External tool to install/upgrade.
pub enum ToolInstallTarget {
    Codex,
    ClaudeCode,
    GeminiCli,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BrewPkgKind {
    Formula,
    Cask,
}

fn npm_package_name(tool: ToolInstallTarget) -> &'static str {
    match tool {
        ToolInstallTarget::Codex => "@openai/codex",
        ToolInstallTarget::ClaudeCode => "@anthropic-ai/claude-code",
        ToolInstallTarget::GeminiCli => "@google/gemini-cli",
    }
}

fn brew_package(tool: ToolInstallTarget) -> (&'static str, BrewPkgKind) {
    match tool {
        ToolInstallTarget::Codex => ("codex", BrewPkgKind::Cask),
        ToolInstallTarget::ClaudeCode => ("claude-code", BrewPkgKind::Cask),
        ToolInstallTarget::GeminiCli => ("gemini-cli", BrewPkgKind::Formula),
    }
}

/// Plan an install/upgrade action using either npm or brew, depending on `method`.
pub fn plan_install(
    tool: ToolInstallTarget,
    method: InstallMethod,
    action: InstallAction,
) -> ChangeSet {
    let resolved = resolve_method(method);
    match resolved {
        InstallMethod::Npm => plan_npm(tool, action),
        InstallMethod::Brew => plan_brew(tool, action),
        InstallMethod::Auto => unreachable!("auto resolved"),
    }
}

fn resolve_method(method: InstallMethod) -> InstallMethod {
    if method != InstallMethod::Auto {
        return method;
    }

    let os = env::consts::OS;
    // KISS: macOS prefers brew; everything else defaults to npm.
    if os == "macos" {
        InstallMethod::Brew
    } else {
        InstallMethod::Npm
    }
}

fn plan_npm(tool: ToolInstallTarget, action: InstallAction) -> ChangeSet {
    let pkg = npm_package_name(tool);
    let mut cs = ChangeSet::new();
    let args: Vec<String> = match action {
        InstallAction::Install | InstallAction::Upgrade => vec![
            "install".to_string(),
            "-g".to_string(),
            format!("{}@latest", pkg),
        ],
    };
    cs.push(Change::RunCommand {
        program: "npm".to_string(),
        args,
        cwd: None,
        env: Vec::new(),
    });
    cs
}

fn plan_brew(tool: ToolInstallTarget, action: InstallAction) -> ChangeSet {
    let (name, kind) = brew_package(tool);
    let mut cs = ChangeSet::new();

    let (verb, mut args) = match action {
        InstallAction::Install => ("install", Vec::new()),
        InstallAction::Upgrade => ("upgrade", Vec::new()),
    };

    args.push(verb.to_string());
    match kind {
        BrewPkgKind::Cask => args.push("--cask".to_string()),
        BrewPkgKind::Formula => {}
    }
    args.push(name.to_string());

    cs.push(Change::RunCommand {
        program: "brew".to_string(),
        args,
        cwd: None,
        env: Vec::new(),
    });
    cs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plan_contains_one_command() {
        let cs = plan_install(
            ToolInstallTarget::Codex,
            InstallMethod::Npm,
            InstallAction::Install,
        );
        assert_eq!(cs.iter().count(), 1);
    }
}
