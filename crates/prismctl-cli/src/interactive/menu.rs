use crate::interactive::utils::{prompt_confirm, prompt_line, prompt_required, prompt_select};
use crate::interactive::{style::UiStyle, utils::install_signal_handlers};
use crate::legacy;
use prismctl_i18n::{keys, t};
use std::fmt;
use std::io;
use std::io::IsTerminal;
use std::io::Write;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MainMenuItem {
    QuickInit,
    ConfigureClaude,
    ConfigureCodex,
    ConfigureGemini,
    ManageOutputStyle,
    ManageSkills,
    ViewCurrentConfig,
    Language,
    Help,
    Exit,
}

impl fmt::Display for MainMenuItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::QuickInit => t!(keys::MENU_QUICK_INIT),
            Self::ConfigureClaude => t!(keys::MENU_CONFIGURE_CLAUDE),
            Self::ConfigureCodex => t!(keys::MENU_CONFIGURE_CODEX),
            Self::ConfigureGemini => t!(keys::MENU_CONFIGURE_GEMINI),
            Self::ManageOutputStyle => t!(keys::MENU_MANAGE_OUTPUT_STYLE),
            Self::ManageSkills => t!(keys::MENU_MANAGE_SKILLS),
            Self::ViewCurrentConfig => t!(keys::MENU_VIEW_CONFIG),
            Self::Language => t!(keys::MENU_LANGUAGE),
            Self::Help => t!(keys::MENU_HELP),
            Self::Exit => t!(keys::MENU_EXIT),
        };
        write!(f, "{}", s)
    }
}

pub fn run() -> Result<(), String> {
    install_signal_handlers();
    let ui = UiStyle::detect();

    loop {
        clear_screen_best_effort(&ui);
        print_header(&ui);

        let menu_title = t!(keys::MENU_TITLE);
        let choice = prompt_select(
            &menu_title,
            vec![
                MainMenuItem::QuickInit,
                MainMenuItem::ConfigureClaude,
                MainMenuItem::ConfigureCodex,
                MainMenuItem::ConfigureGemini,
                MainMenuItem::ManageOutputStyle,
                MainMenuItem::ManageSkills,
                MainMenuItem::ViewCurrentConfig,
                MainMenuItem::Language,
                MainMenuItem::Help,
                MainMenuItem::Exit,
            ],
            0,
        )?;

        let result = match choice {
            MainMenuItem::QuickInit => crate::interactive::wizard_init::wizard_quick_init(),
            MainMenuItem::ConfigureClaude => {
                crate::interactive::wizard_claude::wizard_configure_claude()
            }
            MainMenuItem::ConfigureCodex => {
                crate::interactive::wizard_codex::wizard_configure_codex()
            }
            MainMenuItem::ConfigureGemini => {
                crate::interactive::wizard_gemini::wizard_configure_gemini()
            }
            MainMenuItem::ManageOutputStyle => manage_output_style(),
            MainMenuItem::ManageSkills => skill_menu(),
            MainMenuItem::ViewCurrentConfig => view_current_config(),
            MainMenuItem::Language => switch_language(),
            MainMenuItem::Help => {
                println!("{}", legacy::help());
                Ok(())
            }
            MainMenuItem::Exit => return Ok(()),
        };

        if let Err(e) = result {
            let (_, clean) = crate::errors::strip_tag(&e);
            let prefix = ui.red(ui.err());
            println!("\n{} {}\n", prefix, clean);
            let cont = t!(keys::ACTION_CONTINUE);
            let _ = prompt_line(&cont)?;
        }
    }
}

fn print_header(ui: &UiStyle) {
    let version = env!("CARGO_PKG_VERSION");
    let repo = env!("CARGO_PKG_REPOSITORY");

    // ASCII-only banner (keeps it readable across terminals).
    // Keep it compact: ~80 cols.
    let title = "PRISMCTL";
    let subtitle = t!(keys::APP_SUBTITLE);
    let w = 78usize;
    println!("{}", ui.blue(&format!("+{}+", "-".repeat(w))));
    println!("{}", ui.blue(&format!("| {:<w$} |", "", w = w)));
    println!(
        "{}",
        ui.blue(&format!(
            "|  {:<w$} |",
            format!("{}  {}", title, subtitle),
            w = w - 2
        ))
    );
    println!("{}", ui.blue(&format!("| {:<w$} |", "", w = w)));
    println!("{}", ui.blue(&format!("+{}+", "-".repeat(w))));
    println!("{}", ui.blue(&format!("Version: {} | {}", version, repo)));
    println!("{}", ui.blue(&t!(keys::MENU_TIP_NON_TTY)));
    println!();
}

fn clear_screen_best_effort(ui: &UiStyle) {
    // Keep it safe: only clear when stdout is a real terminal and TERM is not dumb.
    // (Clearing in pipes makes logs unreadable.)
    let _ = ui; // keep signature consistent if we later add style-aware clearing.
    if !io::stdout().is_terminal() {
        return;
    }
    if std::env::var("TERM").ok().as_deref() == Some("dumb") {
        return;
    }
    let mut out = io::stdout();
    let _ = write!(out, "\x1b[2J\x1b[H");
    let _ = out.flush();
}

fn manage_output_style() -> Result<(), String> {
    println!("\n{}", t!(keys::OUTPUT_STYLE_TITLE));
    println!("{}\n", t!(keys::OUTPUT_STYLE_NOTE));

    let prompt = t!(keys::OUTPUT_STYLE_PROMPT_NAME);
    let name = prompt_required(&prompt)?;

    legacy::cmd_claude(vec![
        "output-style".to_string(),
        "use".to_string(),
        "--name".to_string(),
        name.clone(),
    ])?;

    let confirm = t!(keys::ACTION_CONFIRM_APPLY);
    if prompt_confirm(&confirm, false)? {
        legacy::cmd_claude(vec![
            "output-style".to_string(),
            "use".to_string(),
            "--name".to_string(),
            name,
            "--apply".to_string(),
        ])?;
    }

    println!();
    Ok(())
}

fn view_current_config() -> Result<(), String> {
    println!();
    legacy::cmd_doctor(Vec::new())?;
    println!();
    legacy::cmd_skill(vec!["list".to_string()])?;
    println!();
    Ok(())
}

fn skill_menu() -> Result<(), String> {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum SkillMenuItem {
        List,
        Install,
        Create,
        Remove,
        Back,
    }

    impl fmt::Display for SkillMenuItem {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let s = match self {
                Self::List => t!(keys::SKILL_MENU_LIST),
                Self::Install => t!(keys::SKILL_MENU_INSTALL),
                Self::Create => t!(keys::SKILL_MENU_CREATE),
                Self::Remove => t!(keys::SKILL_MENU_REMOVE),
                Self::Back => t!(keys::SKILL_MENU_BACK),
            };
            write!(f, "{}", s)
        }
    }

    loop {
        let title = t!(keys::SKILL_MENU_TITLE);
        let choice = prompt_select(
            &title,
            vec![
                SkillMenuItem::List,
                SkillMenuItem::Install,
                SkillMenuItem::Create,
                SkillMenuItem::Remove,
                SkillMenuItem::Back,
            ],
            0,
        )?;

        let result = match choice {
            SkillMenuItem::List => legacy::cmd_skill(vec!["list".to_string()]),
            SkillMenuItem::Install => skill_install(),
            SkillMenuItem::Create => skill_create(),
            SkillMenuItem::Remove => skill_remove(),
            SkillMenuItem::Back => return Ok(()),
        };

        if let Err(e) = result {
            let (_, clean) = crate::errors::strip_tag(&e);
            println!("\n{}\n", clean);
        }
    }
}

fn skill_install() -> Result<(), String> {
    let prompt = t!(keys::WIZARD_PROMPT_NAME);
    let name = prompt_required(&prompt)?;

    legacy::cmd_skill(vec![
        "install".to_string(),
        "--name".to_string(),
        name.clone(),
    ])?;

    let confirm = t!(keys::ACTION_CONFIRM_APPLY);
    if prompt_confirm(&confirm, false)? {
        legacy::cmd_skill(vec![
            "install".to_string(),
            "--name".to_string(),
            name,
            "--apply".to_string(),
        ])?;
    }

    println!();
    Ok(())
}

fn skill_create() -> Result<(), String> {
    let prompt = t!(keys::WIZARD_PROMPT_NEW_NAME);
    let name = prompt_required(&prompt)?;

    legacy::cmd_skill(vec![
        "create".to_string(),
        "--name".to_string(),
        name.clone(),
    ])?;

    let confirm = t!(keys::ACTION_CONFIRM_CREATE);
    if prompt_confirm(&confirm, false)? {
        legacy::cmd_skill(vec![
            "create".to_string(),
            "--name".to_string(),
            name,
            "--apply".to_string(),
        ])?;
    }

    println!();
    Ok(())
}

fn skill_remove() -> Result<(), String> {
    let prompt = t!(keys::WIZARD_PROMPT_REMOVE_NAME);
    let name = prompt_required(&prompt)?;

    legacy::cmd_skill(vec![
        "remove".to_string(),
        "--name".to_string(),
        name.clone(),
    ])?;

    let confirm = t!(keys::WIZARD_CONFIRM_REMOVE_SHORT);
    if prompt_confirm(&confirm, false)? {
        legacy::cmd_skill(vec![
            "remove".to_string(),
            "--name".to_string(),
            name,
            "--apply".to_string(),
            "--yes".to_string(),
        ])?;
    }

    println!();
    Ok(())
}

fn switch_language() -> Result<(), String> {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum LangItem {
        ZhCN,
        En,
        Back,
    }

    impl fmt::Display for LangItem {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let s = match self {
                Self::ZhCN => t!(keys::LANGUAGE_ZH_CN),
                Self::En => t!(keys::LANGUAGE_EN),
                Self::Back => t!(keys::LANGUAGE_BACK),
            };
            write!(f, "{}", s)
        }
    }

    let title = t!(keys::LANGUAGE_TITLE);
    let default = match prismctl_i18n::current_locale() {
        prismctl_i18n::Locale::ZhCN => 0,
        prismctl_i18n::Locale::En => 1,
    };
    let choice = prompt_select(
        &title,
        vec![LangItem::ZhCN, LangItem::En, LangItem::Back],
        default,
    )?;

    let locale = match choice {
        LangItem::ZhCN => prismctl_i18n::Locale::ZhCN,
        LangItem::En => prismctl_i18n::Locale::En,
        LangItem::Back => return Ok(()),
    };

    prismctl_i18n::set_locale(locale);
    crate::app_config::save_locale(locale)?;

    println!("\n{}\n", t!(keys::LANGUAGE_CHANGED));
    let cont = t!(keys::ACTION_CONTINUE);
    let _ = prompt_line(&cont)?;
    Ok(())
}
