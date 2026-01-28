use crate::changeset::{Change, ChangeSet, FileSystem};
use crate::paths::{PrismctlHome, Tool};
use std::fs;
use std::path::PathBuf;

const ERR_SKILL_NAME_EMPTY: &str = "PRISMCTL_SKILL_NAME_EMPTY";
const ERR_SKILL_NAME_DOT_PREFIX: &str = "PRISMCTL_SKILL_NAME_DOT_PREFIX";
const ERR_SKILL_NAME_HAS_SEPARATOR: &str = "PRISMCTL_SKILL_NAME_HAS_SEPARATOR";
const ERR_SKILL_NAME_INVALID_CHARS: &str = "PRISMCTL_SKILL_NAME_INVALID_CHARS";
const ERR_SKILL_UNKNOWN_BUILTIN_PREFIX: &str = "PRISMCTL_SKILL_UNKNOWN_BUILTIN:";

#[derive(Debug, Clone, PartialEq, Eq)]
/// A Claude Code skill installed on disk.
pub struct Skill {
    pub name: String,
    pub description: String,
    pub path: PathBuf,
}

/// List names of built-in skills embedded in the Prismctl binary.
pub fn list_builtin_skills() -> Vec<&'static str> {
    vec!["explain-code", "codebase-visualizer", "pr-summary"]
}

/// List currently installed skills under the resolved HOME.
pub fn list_installed_skills(home: &PrismctlHome) -> Vec<Skill> {
    let skills_root = claude_skills_root(home);
    let Ok(entries) = fs::read_dir(&skills_root) else {
        return Vec::new();
    };

    let mut out = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let name = match path.file_name().and_then(|s| s.to_str()) {
            Some(s) => s.to_string(),
            None => continue,
        };
        let skill_md = path.join("SKILL.md");
        let content = fs::read_to_string(&skill_md).unwrap_or_default();
        let (parsed_name, parsed_desc) = parse_skill_frontmatter(&content);
        out.push(Skill {
            name: parsed_name.unwrap_or(name),
            description: parsed_desc.unwrap_or_default(),
            path,
        });
    }

    out.sort_by(|a, b| a.name.cmp(&b.name));
    out
}

/// Plan changes to install a built-in skill into `~/.claude/skills/<name>/`.
pub fn plan_install_skill(home: &PrismctlHome, name: &str) -> Result<ChangeSet, String> {
    validate_skill_name(name)?;
    let Some(files) = builtin_skill_files(name) else {
        return Err(format!("{}{}", ERR_SKILL_UNKNOWN_BUILTIN_PREFIX, name));
    };

    let skills_root = claude_skills_root(home);
    let skill_dir = skills_root.join(name);

    let mut cs = ChangeSet::new();
    cs.push(Change::CreateDirAll {
        path: skill_dir.clone(),
    });
    for f in files {
        cs.push(Change::WriteFile {
            path: skill_dir.join(f.rel_path),
            bytes: f.contents.as_bytes().to_vec(),
            overwrite: true,
        });
    }
    Ok(cs)
}

/// Plan changes to create a new skill skeleton (write-if-missing).
pub fn plan_create_skill(home: &PrismctlHome, name: &str) -> ChangeSet {
    if validate_skill_name(name).is_err() {
        return ChangeSet::new();
    }

    let skills_root = claude_skills_root(home);
    let skill_dir = skills_root.join(name);
    let skill_md = skill_dir.join("SKILL.md");

    let mut cs = ChangeSet::new();
    cs.push(Change::CreateDirAll { path: skill_dir });
    cs.push(Change::WriteFile {
        path: skill_md,
        bytes: default_skill_md(name).into_bytes(),
        overwrite: false,
    });
    cs
}

/// Plan changes to remove a skill directory recursively.
pub fn plan_remove_skill(home: &PrismctlHome, name: &str) -> Result<ChangeSet, String> {
    validate_skill_name(name)?;
    let skills_root = claude_skills_root(home);
    let skill_dir = skills_root.join(name);
    let mut cs = ChangeSet::new();
    cs.push(Change::RemoveDirAll { path: skill_dir });
    Ok(cs)
}

/// Check whether a skill exists under `~/.claude/skills/` using the injected filesystem.
pub fn skill_exists(fs: &dyn FileSystem, home: &PrismctlHome, name: &str) -> bool {
    if validate_skill_name(name).is_err() {
        return false;
    }
    fs.path_exists(&claude_skills_root(home).join(name))
}

fn claude_skills_root(home: &PrismctlHome) -> PathBuf {
    home.tool_root(Tool::ClaudeCode).join("skills")
}

fn default_skill_md(name: &str) -> String {
    format!(
        r#"---
name: {name}
description: TODO: describe what this skill does
---

Describe how this skill should behave.
"#
    )
}

struct BuiltinSkillFile {
    rel_path: &'static str,
    contents: &'static str,
}

fn builtin_skill_files(name: &str) -> Option<Vec<BuiltinSkillFile>> {
    match name {
        "explain-code" => Some(vec![BuiltinSkillFile {
            rel_path: "SKILL.md",
            contents: include_str!("../assets/skills/explain-code/SKILL.md"),
        }]),
        "codebase-visualizer" => Some(vec![
            BuiltinSkillFile {
                rel_path: "SKILL.md",
                contents: include_str!("../assets/skills/codebase-visualizer/SKILL.md"),
            },
            BuiltinSkillFile {
                rel_path: "scripts/visualize.py",
                contents: include_str!("../assets/skills/codebase-visualizer/scripts/visualize.py"),
            },
        ]),
        "pr-summary" => Some(vec![BuiltinSkillFile {
            rel_path: "SKILL.md",
            contents: include_str!("../assets/skills/pr-summary/SKILL.md"),
        }]),
        _ => None,
    }
}

fn parse_skill_frontmatter(content: &str) -> (Option<String>, Option<String>) {
    let mut lines = content.lines();
    if lines.next() != Some("---") {
        return (None, None);
    }

    let mut name: Option<String> = None;
    let mut desc: Option<String> = None;
    for line in lines {
        if line.trim() == "---" {
            break;
        }
        let trimmed = line.trim();
        if let Some(v) = trimmed.strip_prefix("name:") {
            name = Some(v.trim().to_string());
        } else if let Some(v) = trimmed.strip_prefix("description:") {
            desc = Some(v.trim().to_string());
        }
    }
    (name, desc)
}

pub fn validate_skill_name(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err(ERR_SKILL_NAME_EMPTY.to_string());
    }
    if name.starts_with('.') {
        return Err(ERR_SKILL_NAME_DOT_PREFIX.to_string());
    }
    if name.contains('/') || name.contains('\\') {
        return Err(ERR_SKILL_NAME_HAS_SEPARATOR.to_string());
    }
    if name
        .chars()
        .any(|c| !(c.is_ascii_alphanumeric() || c == '-' || c == '_'))
    {
        return Err(ERR_SKILL_NAME_INVALID_CHARS.to_string());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::changeset::{ApplyMode, CommandRunner, RealFileSystem};
    use std::io;
    use std::process::ExitStatus;
    use std::time::{SystemTime, UNIX_EPOCH};

    struct NoopRunner;
    impl CommandRunner for NoopRunner {
        fn run(&self, _program: &str, _args: &[String]) -> io::Result<ExitStatus> {
            Err(io::Error::other("noop"))
        }
    }

    fn unique_home() -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time")
            .as_nanos();
        std::env::temp_dir().join(format!("prismctl-skill-home-{}", nanos))
    }

    #[test]
    fn parse_frontmatter_reads_name_and_description() {
        let content = "---\nname: a\ndescription: b\n---\n";
        let (n, d) = parse_skill_frontmatter(content);
        assert_eq!(n.as_deref(), Some("a"));
        assert_eq!(d.as_deref(), Some("b"));
    }

    #[test]
    fn install_skill_writes_into_claude_skills_dir() {
        let home_dir = unique_home();
        let home = PrismctlHome::discover(Some(home_dir.clone())).expect("home");

        let cs = plan_install_skill(&home, "explain-code").expect("plan");
        let fs = RealFileSystem;
        let runner = NoopRunner;
        cs.apply(ApplyMode::Apply, &fs, &runner).expect("apply");

        let installed = home
            .tool_root(Tool::ClaudeCode)
            .join("skills")
            .join("explain-code")
            .join("SKILL.md");
        assert!(installed.exists());

        let _ = fs::remove_dir_all(&home_dir);
    }
}
