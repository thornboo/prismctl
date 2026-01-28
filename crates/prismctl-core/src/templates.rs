use crate::changeset::{Change, ChangeSet};
use crate::managed_block::upsert_managed_block;
use crate::paths::{PrismctlHome, Tool};
use std::collections::HashSet;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Built-in template language variants.
pub enum TemplateLang {
    ZhCn,
    En,
}

#[derive(Debug, Clone)]
/// A template file to be written to disk.
pub struct TemplateFile {
    pub path: PathBuf,
    pub contents: &'static str,
    pub overwrite: bool,
}

/// Plan changes to write Codex templates into the resolved HOME.
pub fn plan_templates_codex(home: &PrismctlHome, lang: TemplateLang) -> ChangeSet {
    let root = home.tool_root(Tool::Codex);
    let files = codex_files(root, lang);
    plan_files(files)
}

/// Names of built-in Codex agent templates.
pub fn codex_agent_names() -> &'static [&'static str] {
    &[
        "prismctl-engineer-professional",
        "prismctl-laowang-engineer",
        "prismctl-leibus-engineer",
        "prismctl-nekomata-engineer",
        "prismctl-ojousama-engineer",
        "prismctl-rem-engineer",
    ]
}

/// Return the embedded agent template content for a given name and language.
pub fn codex_agent_template(name: &str, lang: TemplateLang) -> Option<&'static str> {
    match (name, lang) {
        ("prismctl-engineer-professional", TemplateLang::ZhCn) => Some(include_str!(
            "../assets/codex/agents/zh-CN/prismctl-engineer-professional.md"
        )),
        ("prismctl-laowang-engineer", TemplateLang::ZhCn) => Some(include_str!(
            "../assets/codex/agents/zh-CN/prismctl-laowang-engineer.md"
        )),
        ("prismctl-leibus-engineer", TemplateLang::ZhCn) => Some(include_str!(
            "../assets/codex/agents/zh-CN/prismctl-leibus-engineer.md"
        )),
        ("prismctl-nekomata-engineer", TemplateLang::ZhCn) => Some(include_str!(
            "../assets/codex/agents/zh-CN/prismctl-nekomata-engineer.md"
        )),
        ("prismctl-ojousama-engineer", TemplateLang::ZhCn) => Some(include_str!(
            "../assets/codex/agents/zh-CN/prismctl-ojousama-engineer.md"
        )),
        ("prismctl-rem-engineer", TemplateLang::ZhCn) => Some(include_str!(
            "../assets/codex/agents/zh-CN/prismctl-rem-engineer.md"
        )),
        ("prismctl-engineer-professional", TemplateLang::En) => Some(include_str!(
            "../assets/codex/agents/en/prismctl-engineer-professional.md"
        )),
        ("prismctl-laowang-engineer", TemplateLang::En) => Some(include_str!(
            "../assets/codex/agents/en/prismctl-laowang-engineer.md"
        )),
        ("prismctl-leibus-engineer", TemplateLang::En) => Some(include_str!(
            "../assets/codex/agents/en/prismctl-leibus-engineer.md"
        )),
        ("prismctl-nekomata-engineer", TemplateLang::En) => Some(include_str!(
            "../assets/codex/agents/en/prismctl-nekomata-engineer.md"
        )),
        ("prismctl-ojousama-engineer", TemplateLang::En) => Some(include_str!(
            "../assets/codex/agents/en/prismctl-ojousama-engineer.md"
        )),
        ("prismctl-rem-engineer", TemplateLang::En) => Some(include_str!(
            "../assets/codex/agents/en/prismctl-rem-engineer.md"
        )),
        _ => None,
    }
}

/// Plan changes to write Claude Code templates into the resolved HOME.
pub fn plan_templates_claude(home: &PrismctlHome, lang: TemplateLang) -> ChangeSet {
    let root = home.tool_root(Tool::ClaudeCode);
    let files = claude_files(root, lang);
    plan_files(files)
}

/// Plan changes to write Gemini templates into the resolved HOME.
pub fn plan_templates_gemini(home: &PrismctlHome, lang: TemplateLang) -> ChangeSet {
    plan_templates_gemini_with_existing(home, "", lang)
}

/// Plan changes to write Gemini templates, preserving user content via a managed block.
pub fn plan_templates_gemini_with_existing(
    home: &PrismctlHome,
    existing_gemini_md: &str,
    lang: TemplateLang,
) -> ChangeSet {
    let root = home.tool_root(Tool::GeminiCli);
    let files = gemini_files(root.clone(), lang);
    let mut cs = plan_files(files);

    // Gemini CLI supports hierarchical context via GEMINI.md. We only manage a dedicated block to
    // preserve user-owned content outside the block.
    let next_gemini_md = render_gemini_context(existing_gemini_md, lang);
    cs.push(Change::CreateDirAll { path: root.clone() });
    cs.push(Change::WriteFile {
        path: root.join("GEMINI.md"),
        bytes: next_gemini_md.into_bytes(),
        overwrite: true,
    });
    cs
}

fn plan_files(files: Vec<TemplateFile>) -> ChangeSet {
    let mut cs = ChangeSet::new();
    let mut created_dirs: HashSet<PathBuf> = HashSet::new();
    for f in files {
        if let Some(parent) = f.path.parent() {
            let p = parent.to_path_buf();
            if created_dirs.insert(p.clone()) {
                cs.push(Change::CreateDirAll { path: p });
            }
        }
        cs.push(Change::WriteFile {
            path: f.path,
            bytes: f.contents.as_bytes().to_vec(),
            overwrite: f.overwrite,
        });
    }
    cs
}

fn codex_files(root: PathBuf, lang: TemplateLang) -> Vec<TemplateFile> {
    let prompts = root.join("prompts").join("prismctl");
    let (
        workflow,
        git_commit,
        git_worktree,
        git_rollback,
        git_clean,
        init_project,
        feat,
        bmad_init,
    ) = match lang {
        TemplateLang::ZhCn => (
            include_str!("../assets/codex/prompts/prismctl/zh-CN/workflow.md"),
            include_str!("../assets/codex/prompts/prismctl/zh-CN/git-commit.md"),
            include_str!("../assets/codex/prompts/prismctl/zh-CN/git-worktree.md"),
            include_str!("../assets/codex/prompts/prismctl/zh-CN/git-rollback.md"),
            include_str!("../assets/codex/prompts/prismctl/zh-CN/git-cleanBranches.md"),
            include_str!("../assets/codex/prompts/prismctl/zh-CN/init-project.md"),
            include_str!("../assets/codex/prompts/prismctl/zh-CN/feat.md"),
            include_str!("../assets/codex/prompts/prismctl/zh-CN/bmad-init.md"),
        ),
        TemplateLang::En => (
            include_str!("../assets/codex/prompts/prismctl/en/workflow.md"),
            include_str!("../assets/codex/prompts/prismctl/en/git-commit.md"),
            include_str!("../assets/codex/prompts/prismctl/en/git-worktree.md"),
            include_str!("../assets/codex/prompts/prismctl/en/git-rollback.md"),
            include_str!("../assets/codex/prompts/prismctl/en/git-cleanBranches.md"),
            include_str!("../assets/codex/prompts/prismctl/en/init-project.md"),
            include_str!("../assets/codex/prompts/prismctl/en/feat.md"),
            include_str!("../assets/codex/prompts/prismctl/en/bmad-init.md"),
        ),
    };

    vec![
        TemplateFile {
            path: prompts.join("workflow.md"),
            contents: workflow,
            overwrite: true,
        },
        TemplateFile {
            path: prompts.join("git-commit.md"),
            contents: git_commit,
            overwrite: true,
        },
        TemplateFile {
            path: prompts.join("git-worktree.md"),
            contents: git_worktree,
            overwrite: true,
        },
        TemplateFile {
            path: prompts.join("git-rollback.md"),
            contents: git_rollback,
            overwrite: true,
        },
        TemplateFile {
            path: prompts.join("git-cleanBranches.md"),
            contents: git_clean,
            overwrite: true,
        },
        TemplateFile {
            path: prompts.join("init-project.md"),
            contents: init_project,
            overwrite: true,
        },
        TemplateFile {
            path: prompts.join("feat.md"),
            contents: feat,
            overwrite: true,
        },
        TemplateFile {
            path: prompts.join("bmad-init.md"),
            contents: bmad_init,
            overwrite: true,
        },
    ]
}

fn claude_files(root: PathBuf, lang: TemplateLang) -> Vec<TemplateFile> {
    let commands = root.join("commands").join("prismctl");
    let agents = root.join("agents").join("prismctl");
    let styles = root.join("output-styles");
    let (
        workflow,
        git_commit,
        git_worktree,
        git_rollback,
        git_clean,
        init_project,
        feat,
        bmad_init,
        style_engineer_professional,
        style_laowang,
        style_leibus,
        style_nekomata,
        style_ojousama,
        style_rem,
        agent_get_current_datetime,
        agent_init_architect,
        agent_planner,
        agent_uiux,
    ) = match lang {
        TemplateLang::ZhCn => (
            include_str!("../assets/claude/commands/prismctl/zh-CN/workflow.md"),
            include_str!("../assets/claude/commands/prismctl/zh-CN/git-commit.md"),
            include_str!("../assets/claude/commands/prismctl/zh-CN/git-worktree.md"),
            include_str!("../assets/claude/commands/prismctl/zh-CN/git-rollback.md"),
            include_str!("../assets/claude/commands/prismctl/zh-CN/git-cleanBranches.md"),
            include_str!("../assets/claude/commands/prismctl/zh-CN/init-project.md"),
            include_str!("../assets/claude/commands/prismctl/zh-CN/feat.md"),
            include_str!("../assets/claude/commands/prismctl/zh-CN/bmad-init.md"),
            include_str!("../assets/claude/output-styles/zh-CN/prismctl-engineer-professional.md"),
            include_str!("../assets/claude/output-styles/zh-CN/prismctl-laowang-engineer.md"),
            include_str!("../assets/claude/output-styles/zh-CN/prismctl-leibus-engineer.md"),
            include_str!("../assets/claude/output-styles/zh-CN/prismctl-nekomata-engineer.md"),
            include_str!("../assets/claude/output-styles/zh-CN/prismctl-ojousama-engineer.md"),
            include_str!("../assets/claude/output-styles/zh-CN/prismctl-rem-engineer.md"),
            include_str!("../assets/claude/agents/prismctl/zh-CN/common/get-current-datetime.md"),
            include_str!("../assets/claude/agents/prismctl/zh-CN/common/init-architect.md"),
            include_str!("../assets/claude/agents/prismctl/zh-CN/plan/planner.md"),
            include_str!("../assets/claude/agents/prismctl/zh-CN/plan/ui-ux-designer.md"),
        ),
        TemplateLang::En => (
            include_str!("../assets/claude/commands/prismctl/en/workflow.md"),
            include_str!("../assets/claude/commands/prismctl/en/git-commit.md"),
            include_str!("../assets/claude/commands/prismctl/en/git-worktree.md"),
            include_str!("../assets/claude/commands/prismctl/en/git-rollback.md"),
            include_str!("../assets/claude/commands/prismctl/en/git-cleanBranches.md"),
            include_str!("../assets/claude/commands/prismctl/en/init-project.md"),
            include_str!("../assets/claude/commands/prismctl/en/feat.md"),
            include_str!("../assets/claude/commands/prismctl/en/bmad-init.md"),
            include_str!("../assets/claude/output-styles/en/prismctl-engineer-professional.md"),
            include_str!("../assets/claude/output-styles/en/prismctl-laowang-engineer.md"),
            include_str!("../assets/claude/output-styles/en/prismctl-leibus-engineer.md"),
            include_str!("../assets/claude/output-styles/en/prismctl-nekomata-engineer.md"),
            include_str!("../assets/claude/output-styles/en/prismctl-ojousama-engineer.md"),
            include_str!("../assets/claude/output-styles/en/prismctl-rem-engineer.md"),
            include_str!("../assets/claude/agents/prismctl/en/common/get-current-datetime.md"),
            include_str!("../assets/claude/agents/prismctl/en/common/init-architect.md"),
            include_str!("../assets/claude/agents/prismctl/en/plan/planner.md"),
            include_str!("../assets/claude/agents/prismctl/en/plan/ui-ux-designer.md"),
        ),
    };

    vec![
        TemplateFile {
            path: commands.join("workflow.md"),
            contents: workflow,
            overwrite: true,
        },
        TemplateFile {
            path: commands.join("git-commit.md"),
            contents: git_commit,
            overwrite: true,
        },
        TemplateFile {
            path: commands.join("git-worktree.md"),
            contents: git_worktree,
            overwrite: true,
        },
        TemplateFile {
            path: commands.join("git-rollback.md"),
            contents: git_rollback,
            overwrite: true,
        },
        TemplateFile {
            path: commands.join("git-cleanBranches.md"),
            contents: git_clean,
            overwrite: true,
        },
        TemplateFile {
            path: commands.join("init-project.md"),
            contents: init_project,
            overwrite: true,
        },
        TemplateFile {
            path: commands.join("feat.md"),
            contents: feat,
            overwrite: true,
        },
        TemplateFile {
            path: commands.join("bmad-init.md"),
            contents: bmad_init,
            overwrite: true,
        },
        TemplateFile {
            path: styles.join("prismctl-engineer-professional.md"),
            contents: style_engineer_professional,
            overwrite: true,
        },
        TemplateFile {
            path: styles.join("prismctl-laowang-engineer.md"),
            contents: style_laowang,
            overwrite: true,
        },
        TemplateFile {
            path: styles.join("prismctl-leibus-engineer.md"),
            contents: style_leibus,
            overwrite: true,
        },
        TemplateFile {
            path: styles.join("prismctl-nekomata-engineer.md"),
            contents: style_nekomata,
            overwrite: true,
        },
        TemplateFile {
            path: styles.join("prismctl-ojousama-engineer.md"),
            contents: style_ojousama,
            overwrite: true,
        },
        TemplateFile {
            path: styles.join("prismctl-rem-engineer.md"),
            contents: style_rem,
            overwrite: true,
        },
        TemplateFile {
            path: agents.join("common").join("get-current-datetime.md"),
            contents: agent_get_current_datetime,
            overwrite: true,
        },
        TemplateFile {
            path: agents.join("common").join("init-architect.md"),
            contents: agent_init_architect,
            overwrite: true,
        },
        TemplateFile {
            path: agents.join("plan").join("planner.md"),
            contents: agent_planner,
            overwrite: true,
        },
        TemplateFile {
            path: agents.join("plan").join("ui-ux-designer.md"),
            contents: agent_uiux,
            overwrite: true,
        },
    ]
}

fn gemini_files(root: PathBuf, lang: TemplateLang) -> Vec<TemplateFile> {
    let workflows_readme = match lang {
        TemplateLang::ZhCn => include_str!("../assets/gemini/zh-CN/WORKFLOWS.md"),
        TemplateLang::En => include_str!("../assets/gemini/en/WORKFLOWS.md"),
    };

    vec![TemplateFile {
        path: root.join("prismctl").join("WORKFLOWS.md"),
        contents: workflows_readme,
        overwrite: true,
    }]
}

pub fn render_gemini_context(existing: &str, lang: TemplateLang) -> String {
    let block = match lang {
        TemplateLang::ZhCn => include_str!("../assets/gemini/zh-CN/GEMINI.md"),
        TemplateLang::En => include_str!("../assets/gemini/en/GEMINI.md"),
    };
    upsert_managed_block(
        existing,
        "<!-- prismctl:start -->",
        "<!-- prismctl:end -->",
        block,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::changeset::{ApplyMode, CommandRunner, RealFileSystem};
    use std::fs;
    use std::io;
    use std::process::ExitStatus;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn render_gemini_context_inserts_block() {
        let out = render_gemini_context("", TemplateLang::ZhCn);
        assert!(out.contains("<!-- prismctl:start -->"));
        assert!(out.contains("<!-- prismctl:end -->"));
        assert!(out.contains("GEMINI.md"));
    }

    struct NoopRunner;
    impl CommandRunner for NoopRunner {
        fn run(
            &self,
            _program: &str,
            _args: &[String],
            _cwd: Option<&std::path::Path>,
            _env: &[(String, String)],
        ) -> io::Result<ExitStatus> {
            Err(io::Error::other("noop"))
        }
    }

    fn unique_root() -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time")
            .as_nanos();
        std::env::temp_dir().join(format!("prismctl-templates-{}", nanos))
    }

    #[test]
    fn templates_apply_into_sandbox_home() {
        let sandbox = unique_root();
        let home = PrismctlHome::new(sandbox.clone());
        let fs = RealFileSystem;
        let runner = NoopRunner;

        let mut cs = ChangeSet::new();
        cs.extend(plan_templates_codex(&home, TemplateLang::ZhCn));
        cs.extend(plan_templates_claude(&home, TemplateLang::ZhCn));
        cs.extend(plan_templates_gemini_with_existing(
            &home,
            "",
            TemplateLang::ZhCn,
        ));

        cs.apply(ApplyMode::Apply, &fs, &runner).expect("apply");

        assert!(sandbox.join(".codex/prompts/prismctl/workflow.md").exists());
        assert!(sandbox
            .join(".claude/output-styles/prismctl-engineer-professional.md")
            .exists());
        assert!(sandbox.join(".gemini/GEMINI.md").exists());

        // Best-effort cleanup.
        let _ = fs::remove_dir_all(&sandbox);
    }
}
