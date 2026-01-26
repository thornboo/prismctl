use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApplyMode {
    DryRun,
    Apply,
}

#[derive(Debug, Clone)]
pub enum Change {
    CreateDirAll {
        path: PathBuf,
    },
    RemoveDirAll {
        path: PathBuf,
    },
    WriteFile {
        path: PathBuf,
        bytes: Vec<u8>,
        overwrite: bool,
    },
    RunCommand {
        program: String,
        args: Vec<String>,
    },
}

impl fmt::Display for Change {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Change::CreateDirAll { path } => write!(f, "mkdir -p {}", quote_path(path)),
            Change::RemoveDirAll { path } => write!(f, "rm -rf {}", quote_path(path)),
            Change::WriteFile {
                path, overwrite, ..
            } => {
                if *overwrite {
                    write!(f, "write {}", quote_path(path))
                } else {
                    write!(f, "write-if-missing {}", quote_path(path))
                }
            }
            Change::RunCommand { program, args } => {
                let rendered_args = args
                    .iter()
                    .map(|a| quote_arg(a))
                    .collect::<Vec<_>>()
                    .join(" ");
                if rendered_args.is_empty() {
                    write!(f, "run {}", quote_arg(program))
                } else {
                    write!(f, "run {} {}", quote_arg(program), rendered_args)
                }
            }
        }
    }
}

fn quote_path(path: &Path) -> String {
    // Always quote paths to avoid whitespace issues and align with repo conventions.
    let s = path.display().to_string();
    let escaped = s.replace('\\', "\\\\").replace('"', "\\\"");
    format!("\"{}\"", escaped)
}

fn quote_arg(arg: &str) -> String {
    // Minimal shell-style quoting for display only.
    let needs_quotes = arg
        .chars()
        .any(|c| c.is_whitespace() || c == '"' || c == '\'');
    if !needs_quotes {
        return arg.to_string();
    }
    let escaped = arg.replace('\\', "\\\\").replace('"', "\\\"");
    format!("\"{}\"", escaped)
}

#[derive(Debug, Default, Clone)]
pub struct ChangeSet {
    changes: Vec<Change>,
}

impl ChangeSet {
    pub fn new() -> Self {
        Self {
            changes: Vec::new(),
        }
    }

    pub fn push(&mut self, change: Change) {
        self.changes.push(change);
    }

    pub fn is_empty(&self) -> bool {
        self.changes.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Change> {
        self.changes.iter()
    }

    pub fn extend(&mut self, other: ChangeSet) {
        self.changes.extend(other.changes);
    }

    pub fn apply(
        &self,
        mode: ApplyMode,
        fs: &dyn FileSystem,
        runner: &dyn CommandRunner,
    ) -> Result<(), String> {
        for change in &self.changes {
            match mode {
                ApplyMode::DryRun => {
                    // Intentionally do nothing other than allow the caller to print `Change`.
                    let _ = change;
                }
                ApplyMode::Apply => {
                    match change {
                        Change::CreateDirAll { path } => fs
                            .create_dir_all(path)
                            .map_err(|e| format!("创建目录失败: {}: {}", path.display(), e))?,
                        Change::RemoveDirAll { path } => fs
                            .remove_dir_all(path)
                            .map_err(|e| format!("删除目录失败: {}: {}", path.display(), e))?,
                        Change::WriteFile {
                            path,
                            bytes,
                            overwrite,
                        } => fs
                            .write_file(path, bytes, *overwrite)
                            .map_err(|e| format!("写入文件失败: {}: {}", path.display(), e))?,
                        Change::RunCommand { program, args } => runner
                            .run(program, args)
                            .map(|_| ())
                            .map_err(|e| format!("执行命令失败: {}: {}", program, e))?,
                    }
                }
            }
        }

        Ok(())
    }
}

pub trait FileSystem {
    fn create_dir_all(&self, path: &Path) -> io::Result<()>;
    fn remove_dir_all(&self, path: &Path) -> io::Result<()>;
    fn write_file(&self, path: &Path, bytes: &[u8], overwrite: bool) -> io::Result<()>;
    fn path_exists(&self, path: &Path) -> bool;
}

pub struct RealFileSystem;

impl FileSystem for RealFileSystem {
    fn create_dir_all(&self, path: &Path) -> io::Result<()> {
        fs::create_dir_all(path)
    }

    fn remove_dir_all(&self, path: &Path) -> io::Result<()> {
        if !path.exists() {
            return Ok(());
        }
        fs::remove_dir_all(path)
    }

    fn write_file(&self, path: &Path, bytes: &[u8], overwrite: bool) -> io::Result<()> {
        if !overwrite && path.exists() {
            return Ok(());
        }

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(path, bytes)
    }

    fn path_exists(&self, path: &Path) -> bool {
        path.exists()
    }
}

pub trait CommandRunner {
    fn run(&self, program: &str, args: &[String]) -> io::Result<ExitStatus>;
}

pub struct RealCommandRunner;

impl CommandRunner for RealCommandRunner {
    fn run(&self, program: &str, args: &[String]) -> io::Result<ExitStatus> {
        Command::new(program).args(args).status()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    struct RecordingFileSystem {
        created_dirs: std::sync::Mutex<HashSet<PathBuf>>,
        removed_dirs: std::sync::Mutex<HashSet<PathBuf>>,
        written_files: std::sync::Mutex<HashSet<PathBuf>>,
    }

    impl RecordingFileSystem {
        fn new() -> Self {
            Self {
                created_dirs: std::sync::Mutex::new(HashSet::new()),
                removed_dirs: std::sync::Mutex::new(HashSet::new()),
                written_files: std::sync::Mutex::new(HashSet::new()),
            }
        }
    }

    impl FileSystem for RecordingFileSystem {
        fn create_dir_all(&self, path: &Path) -> io::Result<()> {
            self.created_dirs
                .lock()
                .expect("lock")
                .insert(path.to_path_buf());
            Ok(())
        }

        fn remove_dir_all(&self, path: &Path) -> io::Result<()> {
            self.removed_dirs
                .lock()
                .expect("lock")
                .insert(path.to_path_buf());
            Ok(())
        }

        fn write_file(&self, path: &Path, _bytes: &[u8], _overwrite: bool) -> io::Result<()> {
            self.written_files
                .lock()
                .expect("lock")
                .insert(path.to_path_buf());
            Ok(())
        }

        fn path_exists(&self, _path: &Path) -> bool {
            false
        }
    }

    struct NoopRunner;
    impl CommandRunner for NoopRunner {
        fn run(&self, _program: &str, _args: &[String]) -> io::Result<ExitStatus> {
            Err(io::Error::other(
                "runner should not be invoked in these tests",
            ))
        }
    }

    fn unique_path() -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time")
            .as_nanos();
        std::env::temp_dir().join(format!("ekko-test-{}", nanos))
    }

    #[test]
    fn dry_run_does_not_touch_filesystem() {
        let mut cs = ChangeSet::new();
        cs.push(Change::CreateDirAll {
            path: unique_path(),
        });
        cs.push(Change::RemoveDirAll {
            path: unique_path(),
        });
        cs.push(Change::WriteFile {
            path: unique_path().join("a.txt"),
            bytes: b"hello".to_vec(),
            overwrite: true,
        });

        let fs = RecordingFileSystem::new();
        let runner = NoopRunner;

        cs.apply(ApplyMode::DryRun, &fs, &runner).expect("apply");

        assert!(fs.created_dirs.lock().expect("lock").is_empty());
        assert!(fs.removed_dirs.lock().expect("lock").is_empty());
        assert!(fs.written_files.lock().expect("lock").is_empty());
    }

    #[test]
    fn apply_writes_only_where_told() {
        let root = unique_path();
        let out_dir = root.join("out");
        let out_file = out_dir.join("a.txt");

        let mut cs = ChangeSet::new();
        cs.push(Change::CreateDirAll {
            path: out_dir.clone(),
        });
        cs.push(Change::WriteFile {
            path: out_file.clone(),
            bytes: b"ok\n".to_vec(),
            overwrite: true,
        });

        let real_fs = RealFileSystem;
        let runner = NoopRunner;

        cs.apply(ApplyMode::Apply, &real_fs, &runner)
            .expect("apply");
        assert!(out_file.exists());

        // Best-effort cleanup.
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn apply_removes_directory_tree() {
        let root = unique_path();
        let nested = root.join("a/b");
        fs::create_dir_all(&nested).expect("mkdir");
        fs::write(root.join("a/b/c.txt"), b"ok\n").expect("write");

        let mut cs = ChangeSet::new();
        cs.push(Change::RemoveDirAll { path: root.clone() });

        let real_fs = RealFileSystem;
        let runner = NoopRunner;
        cs.apply(ApplyMode::Apply, &real_fs, &runner)
            .expect("apply");

        assert!(!root.exists());
    }
}
