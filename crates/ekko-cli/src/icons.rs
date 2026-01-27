use devicons::icon_for_file;
use devicons::{FileIcon, Theme};
use ekko_core::changeset::Change;
use std::env;
use std::io;
use std::io::IsTerminal;

fn use_icons() -> bool {
    if !io::stdout().is_terminal() {
        return false;
    }
    if env::var("TERM").ok().as_deref() == Some("dumb") {
        return false;
    }
    if env::var_os("EKKO_NO_ICONS").is_some() || env::var_os("NO_ICONS").is_some() {
        return false;
    }
    true
}

pub fn render_change(change: &Change) -> String {
    if !use_icons() {
        return change.to_string();
    }

    let icon: Option<FileIcon> = match change {
        Change::CreateDirAll { path } | Change::RemoveDirAll { path } => {
            // `devicons` only treats directories specially when the path ends with '/' or exists.
            // For planned changes, the directory may not exist yet, so we force a trailing slash.
            let pseudo = format!("{}/", path.display());
            Some(icon_for_file(pseudo.as_str(), &Some(Theme::Dark)))
        }
        Change::WriteFile { path, .. } => Some(icon_for_file(path, &Some(Theme::Dark))),
        Change::RunCommand { .. } => None,
    };

    match icon {
        Some(ic) => format!("{} {}", ic.icon, change),
        None => change.to_string(),
    }
}
