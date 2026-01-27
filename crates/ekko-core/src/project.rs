use crate::changeset::{Change, ChangeSet};
use crate::managed_block::upsert_managed_block;
use crate::templates::TemplateLang;
use std::path::Path;

/// Plan project initialization: create `.ekko/plan/` directories and manage `.gemini/GEMINI.md`.
pub fn plan_project_init(
    project_root: &Path,
    lang: TemplateLang,
    existing_project_gemini_md: &str,
) -> ChangeSet {
    let mut cs = ChangeSet::new();

    let ekko_plan_current = project_root.join(".ekko").join("plan").join("current");
    let ekko_plan_history = project_root.join(".ekko").join("plan").join("history");
    let ekko_plan_readme = project_root.join(".ekko").join("plan").join("README.md");
    let gemini_dir = project_root.join(".gemini");

    cs.push(Change::CreateDirAll {
        path: ekko_plan_current,
    });
    cs.push(Change::CreateDirAll {
        path: ekko_plan_history,
    });
    cs.push(Change::WriteFile {
        path: ekko_plan_readme,
        bytes: render_ekko_plan_readme(lang).as_bytes().to_vec(),
        overwrite: true,
    });
    cs.push(Change::CreateDirAll {
        path: gemini_dir.clone(),
    });

    let next = render_project_gemini_md(existing_project_gemini_md, lang);
    cs.push(Change::WriteFile {
        path: gemini_dir.join("GEMINI.md"),
        bytes: next.into_bytes(),
        overwrite: true,
    });

    cs
}

/// Render and upsert the Ekko-managed block for a project-level `.gemini/GEMINI.md`.
pub fn render_project_gemini_md(existing: &str, lang: TemplateLang) -> String {
    let block = match lang {
        TemplateLang::ZhCn => include_str!("../assets/gemini/zh-CN/PROJECT_GEMINI.md"),
        TemplateLang::En => include_str!("../assets/gemini/en/PROJECT_GEMINI.md"),
    };
    upsert_managed_block(existing, "<!-- ekko:start -->", "<!-- ekko:end -->", block)
}

fn render_ekko_plan_readme(lang: TemplateLang) -> &'static str {
    match lang {
        TemplateLang::ZhCn => include_str!("../assets/project/zh-CN/EKKO_PLAN_README.md"),
        TemplateLang::En => include_str!("../assets/project/en/EKKO_PLAN_README.md"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::changeset::{ApplyMode, CommandRunner, RealFileSystem};
    use std::fs;
    use std::io;
    use std::path::PathBuf;
    use std::process::ExitStatus;
    use std::time::{SystemTime, UNIX_EPOCH};

    struct NoopRunner;
    impl CommandRunner for NoopRunner {
        fn run(&self, _program: &str, _args: &[String]) -> io::Result<ExitStatus> {
            Err(io::Error::other("noop"))
        }
    }

    fn unique_root() -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time")
            .as_nanos();
        std::env::temp_dir().join(format!("ekko-project-{}", nanos))
    }

    #[test]
    fn render_project_gemini_md_inserts_block() {
        let out = render_project_gemini_md("", TemplateLang::ZhCn);
        assert!(out.contains("<!-- ekko:start -->"));
        assert!(out.contains("<!-- ekko:end -->"));
    }

    #[test]
    fn project_init_writes_expected_paths() {
        let root = unique_root();
        let fs = RealFileSystem;
        let runner = NoopRunner;

        let cs = plan_project_init(&root, TemplateLang::En, "");
        cs.apply(ApplyMode::Apply, &fs, &runner).expect("apply");

        assert!(root.join(".ekko/plan/current").exists());
        assert!(root.join(".ekko/plan/history").exists());
        assert!(root.join(".ekko/plan/README.md").exists());
        assert!(root.join(".gemini/GEMINI.md").exists());

        let _ = fs::remove_dir_all(&root);
    }
}
