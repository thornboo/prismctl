use crate::interactive::utils::{
    prompt_confirm, prompt_line, prompt_multi_select, prompt_required, prompt_select,
};
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
    let mut last_choice = MainMenuItem::QuickInit;

    loop {
        clear_screen_best_effort(&ui);
        print_header(&ui);

        let menu_title = t!(keys::MENU_TITLE);
        let options = vec![
            MainMenuItem::QuickInit,
            MainMenuItem::ConfigureClaude,
            MainMenuItem::ConfigureCodex,
            MainMenuItem::ConfigureGemini,
            MainMenuItem::ViewCurrentConfig,
            MainMenuItem::Language,
            MainMenuItem::Help,
            MainMenuItem::Exit,
        ];
        let default_index = options.iter().position(|c| *c == last_choice).unwrap_or(0);
        let choice = prompt_select(&menu_title, options, default_index)?;
        last_choice = choice;

        let result = match choice {
            MainMenuItem::QuickInit => crate::interactive::wizard_init::wizard_quick_init(),
            MainMenuItem::ConfigureClaude => claude_menu(),
            MainMenuItem::ConfigureCodex => codex_menu(),
            MainMenuItem::ConfigureGemini => gemini_menu(),
            MainMenuItem::ViewCurrentConfig => view_current_config(),
            MainMenuItem::Language => switch_language(),
            MainMenuItem::Help => {
                println!("{}", legacy::help());
                let cont = t!(keys::ACTION_CONTINUE);
                let _ = prompt_line(&cont)?;
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

    let subtitle = t!(keys::APP_SUBTITLE);

    // ASCII-only logo: keep it readable across terminals (no Unicode box drawing).
    let logo = r#"
 ____  ____  ___ ____  __  ____  __
|  _ \|  _ \|_ _/ ___||  \/  | \ \/ /
| |_) | |_) || |\___ \| |\/| |  \  /
|  __/|  _ < | | ___) | |  | |  /  \
|_|   |_| \_\___|____/|_|  |_| /_/\_\
"#
    .trim_matches('\n');

    println!("{}", ui.blue(logo));
    println!("{}", ui.blue(&subtitle));
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

fn claude_menu() -> Result<(), String> {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum ClaudeMenuItem {
        Wizard,
        Mcp,
        OutputStyle,
        Skills,
        Back,
    }

    impl fmt::Display for ClaudeMenuItem {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let s = match self {
                Self::Wizard => t!(keys::MENU_CLAUDE_WIZARD),
                Self::Mcp => t!(keys::MENU_CLAUDE_MCP),
                Self::OutputStyle => t!(keys::MENU_CLAUDE_OUTPUT_STYLE),
                Self::Skills => t!(keys::MENU_CLAUDE_SKILLS),
                Self::Back => t!(keys::MENU_BACK),
            };
            write!(f, "{}", s)
        }
    }

    loop {
        let title = t!(keys::MENU_CLAUDE_TITLE);
        let choice = prompt_select(
            &title,
            vec![
                ClaudeMenuItem::Wizard,
                ClaudeMenuItem::Mcp,
                ClaudeMenuItem::OutputStyle,
                ClaudeMenuItem::Skills,
                ClaudeMenuItem::Back,
            ],
            0,
        )?;

        let result = match choice {
            ClaudeMenuItem::Wizard => crate::interactive::wizard_claude::wizard_configure_claude(),
            ClaudeMenuItem::Mcp => claude_mcp_menu(),
            ClaudeMenuItem::OutputStyle => manage_output_style(),
            ClaudeMenuItem::Skills => skill_menu(),
            ClaudeMenuItem::Back => return Ok(()),
        };

        if let Err(e) = result {
            let (_, clean) = crate::errors::strip_tag(&e);
            println!("\n{}\n", clean);
        }
    }
}

fn codex_menu() -> Result<(), String> {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum CodexMenuItem {
        Wizard,
        Provider,
        Agent,
        Back,
    }

    impl fmt::Display for CodexMenuItem {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let s = match self {
                Self::Wizard => t!(keys::MENU_CODEX_WIZARD),
                Self::Provider => t!(keys::MENU_CODEX_PROVIDER),
                Self::Agent => t!(keys::MENU_CODEX_AGENT),
                Self::Back => t!(keys::MENU_BACK),
            };
            write!(f, "{}", s)
        }
    }

    loop {
        let title = t!(keys::MENU_CODEX_TITLE);
        let choice = prompt_select(
            &title,
            vec![
                CodexMenuItem::Wizard,
                CodexMenuItem::Provider,
                CodexMenuItem::Agent,
                CodexMenuItem::Back,
            ],
            0,
        )?;
        match choice {
            CodexMenuItem::Wizard => crate::interactive::wizard_codex::wizard_configure_codex()?,
            CodexMenuItem::Provider => codex_provider_configure()?,
            CodexMenuItem::Agent => codex_agent_configure()?,
            CodexMenuItem::Back => return Ok(()),
        }
    }
}

fn gemini_menu() -> Result<(), String> {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum GeminiMenuItem {
        Wizard,
        Mcp,
        Env,
        Settings,
        Back,
    }

    impl fmt::Display for GeminiMenuItem {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let s = match self {
                Self::Wizard => t!(keys::MENU_GEMINI_WIZARD),
                Self::Mcp => t!(keys::MENU_GEMINI_MCP),
                Self::Env => t!(keys::MENU_GEMINI_ENV),
                Self::Settings => t!(keys::MENU_GEMINI_SETTINGS),
                Self::Back => t!(keys::MENU_BACK),
            };
            write!(f, "{}", s)
        }
    }

    loop {
        let title = t!(keys::MENU_GEMINI_TITLE);
        let choice = prompt_select(
            &title,
            vec![
                GeminiMenuItem::Wizard,
                GeminiMenuItem::Mcp,
                GeminiMenuItem::Env,
                GeminiMenuItem::Settings,
                GeminiMenuItem::Back,
            ],
            0,
        )?;
        match choice {
            GeminiMenuItem::Wizard => crate::interactive::wizard_gemini::wizard_configure_gemini()?,
            GeminiMenuItem::Mcp => gemini_mcp_menu()?,
            GeminiMenuItem::Env => gemini_env_configure()?,
            GeminiMenuItem::Settings => gemini_settings_configure()?,
            GeminiMenuItem::Back => return Ok(()),
        }
    }
}

fn gemini_mcp_menu() -> Result<(), String> {
    use prismctl_core::mcp;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum McpMenuItem {
        List,
        Builtin,
        Add,
        Remove,
        Enable,
        Disable,
        Back,
    }

    impl fmt::Display for McpMenuItem {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let s = match self {
                Self::List => t!(keys::MENU_GEMINI_MCP_LIST),
                Self::Builtin => t!(keys::MENU_GEMINI_MCP_BUILTIN),
                Self::Add => t!(keys::MENU_GEMINI_MCP_ADD),
                Self::Remove => t!(keys::MENU_GEMINI_MCP_REMOVE),
                Self::Enable => t!(keys::MENU_GEMINI_MCP_ENABLE),
                Self::Disable => t!(keys::MENU_GEMINI_MCP_DISABLE),
                Self::Back => t!(keys::MENU_BACK),
            };
            write!(f, "{}", s)
        }
    }

    loop {
        let title = t!(keys::MENU_GEMINI_MCP_TITLE);
        let choice = prompt_select(
            &title,
            vec![
                McpMenuItem::List,
                McpMenuItem::Builtin,
                McpMenuItem::Add,
                McpMenuItem::Remove,
                McpMenuItem::Enable,
                McpMenuItem::Disable,
                McpMenuItem::Back,
            ],
            0,
        )?;

        match choice {
            McpMenuItem::List => {
                let scope_title = t!(keys::MENU_GEMINI_MCP_SCOPE_TITLE);
                let scope_project = t!(keys::MENU_GEMINI_MCP_SCOPE_PROJECT);
                let scope_user = t!(keys::MENU_GEMINI_MCP_SCOPE_USER);
                let scope_choice = prompt_select(
                    &scope_title,
                    vec![scope_project.clone(), scope_user.clone()],
                    0,
                )?;
                let scope_flag = if scope_choice == scope_user {
                    "user"
                } else {
                    "project"
                };

                let project_path = if scope_flag == "project" {
                    let p = t!(keys::MENU_GEMINI_MCP_PROJECT_PATH_PROMPT_OPT);
                    let raw = prompt_line(&p)?;
                    let v = raw.trim();
                    if v.is_empty() {
                        None
                    } else {
                        Some(v.to_string())
                    }
                } else {
                    None
                };

                let mut cmd = vec![
                    "mcp".to_string(),
                    "list".to_string(),
                    "--scope".to_string(),
                    scope_flag.to_string(),
                ];
                if let Some(p) = project_path {
                    cmd.push("--project-path".to_string());
                    cmd.push(p);
                }
                legacy::cmd_gemini(cmd)?;
                let cont = t!(keys::ACTION_CONTINUE);
                let _ = prompt_line(&cont)?;
            }
            McpMenuItem::Builtin => {
                legacy::cmd_gemini(vec!["mcp".to_string(), "builtin".to_string()])?;
                let cont = t!(keys::ACTION_CONTINUE);
                let _ = prompt_line(&cont)?;
            }
            McpMenuItem::Add => {
                let _ = legacy::cmd_gemini(vec!["mcp".to_string(), "builtin".to_string()]);

                let servers = mcp::list_builtin_mcp_servers()
                    .iter()
                    .map(|s| s.id.to_string())
                    .collect::<Vec<_>>();

                let select_title = t!(keys::MENU_GEMINI_MCP_PICK);
                let picked = prompt_multi_select(&select_title, servers, Vec::new())?;
                if picked.is_empty() {
                    println!("\n{}\n", t!(keys::MENU_GEMINI_MCP_EMPTY));
                    continue;
                }

                let scope_title = t!(keys::MENU_GEMINI_MCP_SCOPE_TITLE);
                let scope_project = t!(keys::MENU_GEMINI_MCP_SCOPE_PROJECT);
                let scope_user = t!(keys::MENU_GEMINI_MCP_SCOPE_USER);
                let scope_choice = prompt_select(
                    &scope_title,
                    vec![scope_project.clone(), scope_user.clone()],
                    0,
                )?;
                let scope_flag = if scope_choice == scope_user {
                    "user"
                } else {
                    "project"
                };

                let project_path = if scope_flag == "project" {
                    let p = t!(keys::MENU_GEMINI_MCP_PROJECT_PATH_PROMPT);
                    let raw = prompt_line(&p)?;
                    let v = raw.trim();
                    if v.is_empty() {
                        None
                    } else {
                        Some(v.to_string())
                    }
                } else {
                    None
                };

                for id in &picked {
                    let mut cmd = vec![
                        "mcp".to_string(),
                        "add".to_string(),
                        "--name".to_string(),
                        id.clone(),
                        "--scope".to_string(),
                        scope_flag.to_string(),
                    ];
                    if let Some(p) = &project_path {
                        cmd.push("--project-path".to_string());
                        cmd.push(p.clone());
                    }
                    legacy::cmd_gemini(cmd)?;
                }

                let confirm = t!(keys::ACTION_CONFIRM_APPLY);
                if prompt_confirm(&confirm, false)? {
                    for id in picked {
                        let mut cmd = vec![
                            "mcp".to_string(),
                            "add".to_string(),
                            "--name".to_string(),
                            id,
                            "--scope".to_string(),
                            scope_flag.to_string(),
                            "--apply".to_string(),
                            "--yes".to_string(),
                        ];
                        if let Some(p) = &project_path {
                            cmd.push("--project-path".to_string());
                            cmd.push(p.clone());
                        }
                        legacy::cmd_gemini(cmd)?;
                    }
                }
            }
            McpMenuItem::Remove => {
                let p = t!(keys::MENU_GEMINI_MCP_NAME_PROMPT);
                let name = prompt_required(&p)?;

                let scope_title = t!(keys::MENU_GEMINI_MCP_SCOPE_TITLE);
                let scope_project = t!(keys::MENU_GEMINI_MCP_SCOPE_PROJECT);
                let scope_user = t!(keys::MENU_GEMINI_MCP_SCOPE_USER);
                let scope_choice = prompt_select(
                    &scope_title,
                    vec![scope_project.clone(), scope_user.clone()],
                    0,
                )?;
                let scope_flag = if scope_choice == scope_user {
                    "user"
                } else {
                    "project"
                };

                let project_path = if scope_flag == "project" {
                    let p = t!(keys::MENU_GEMINI_MCP_PROJECT_PATH_PROMPT_OPT);
                    let raw = prompt_line(&p)?;
                    let v = raw.trim();
                    if v.is_empty() {
                        None
                    } else {
                        Some(v.to_string())
                    }
                } else {
                    None
                };

                let mut preview = vec![
                    "mcp".to_string(),
                    "remove".to_string(),
                    "--name".to_string(),
                    name.clone(),
                    "--scope".to_string(),
                    scope_flag.to_string(),
                ];
                if let Some(p) = &project_path {
                    preview.push("--project-path".to_string());
                    preview.push(p.clone());
                }
                legacy::cmd_gemini(preview)?;

                let confirm = t!(keys::ACTION_CONFIRM_APPLY);
                if prompt_confirm(&confirm, false)? {
                    let mut apply = vec![
                        "mcp".to_string(),
                        "remove".to_string(),
                        "--name".to_string(),
                        name,
                        "--scope".to_string(),
                        scope_flag.to_string(),
                        "--apply".to_string(),
                        "--yes".to_string(),
                    ];
                    if let Some(p) = project_path {
                        apply.push("--project-path".to_string());
                        apply.push(p);
                    }
                    legacy::cmd_gemini(apply)?;
                }
            }
            McpMenuItem::Enable => {
                let p = t!(keys::MENU_GEMINI_MCP_NAME_PROMPT);
                let name = prompt_required(&p)?;

                legacy::cmd_gemini(vec![
                    "mcp".to_string(),
                    "enable".to_string(),
                    "--name".to_string(),
                    name.clone(),
                ])?;

                let confirm = t!(keys::ACTION_CONFIRM_APPLY);
                if prompt_confirm(&confirm, false)? {
                    legacy::cmd_gemini(vec![
                        "mcp".to_string(),
                        "enable".to_string(),
                        "--name".to_string(),
                        name,
                        "--apply".to_string(),
                        "--yes".to_string(),
                    ])?;
                }
            }
            McpMenuItem::Disable => {
                let p = t!(keys::MENU_GEMINI_MCP_NAME_PROMPT);
                let name = prompt_required(&p)?;

                legacy::cmd_gemini(vec![
                    "mcp".to_string(),
                    "disable".to_string(),
                    "--name".to_string(),
                    name.clone(),
                ])?;

                let confirm = t!(keys::ACTION_CONFIRM_APPLY);
                if prompt_confirm(&confirm, false)? {
                    legacy::cmd_gemini(vec![
                        "mcp".to_string(),
                        "disable".to_string(),
                        "--name".to_string(),
                        name,
                        "--apply".to_string(),
                        "--yes".to_string(),
                    ])?;
                }
            }
            McpMenuItem::Back => return Ok(()),
        }
    }
}

fn gemini_env_configure() -> Result<(), String> {
    use crate::interactive::utils::prompt_secret_with_env_default;

    println!("\n{}\n", t!(keys::WIZARD_GEMINI_TITLE));

    let api_key_prompt = t!(keys::WIZARD_GEMINI_PROMPT_API_KEY);
    let api_key =
        prompt_secret_with_env_default(&api_key_prompt, &["GEMINI_API_KEY", "PRISMCTL_API_KEY"])?;
    if let Some(k) = &api_key {
        crate::interactive::utils::validate_api_key_format(k)?;
    }

    let Some(key) = api_key else {
        println!("{}\n", t!(keys::WIZARD_NO_CHANGES_CANCELLED));
        return Ok(());
    };

    let scope_title = t!(keys::MENU_GEMINI_ENV_SCOPE_TITLE);
    let scope_user = t!(keys::MENU_GEMINI_ENV_SCOPE_USER);
    let scope_project = t!(keys::MENU_GEMINI_ENV_SCOPE_PROJECT);
    let scope_choice = prompt_select(
        &scope_title,
        vec![scope_user.clone(), scope_project.clone()],
        0,
    )?;
    let (scope_flag, project_path) = if scope_choice == scope_project {
        let p = t!(keys::MENU_GEMINI_ENV_PROJECT_PATH_PROMPT);
        let raw = prompt_line(&p)?;
        let v = raw.trim();
        (
            "project",
            if v.is_empty() {
                None
            } else {
                Some(v.to_string())
            },
        )
    } else {
        ("user", None)
    };

    // Preview.
    let mut preview = vec![
        "env".to_string(),
        "set".to_string(),
        "--scope".to_string(),
        scope_flag.to_string(),
        "--api-key".to_string(),
        key.clone(),
    ];
    if let Some(p) = &project_path {
        preview.push("--project-path".to_string());
        preview.push(p.clone());
    }
    legacy::cmd_gemini(preview)?;

    let confirm = t!(keys::ACTION_CONFIRM_APPLY);
    if prompt_confirm(&confirm, false)? {
        let mut apply = vec![
            "env".to_string(),
            "set".to_string(),
            "--scope".to_string(),
            scope_flag.to_string(),
            "--api-key".to_string(),
            key,
            "--apply".to_string(),
        ];
        if let Some(p) = project_path {
            apply.push("--project-path".to_string());
            apply.push(p);
        }
        legacy::cmd_gemini(apply)?;
    }

    println!();
    Ok(())
}

fn gemini_settings_configure() -> Result<(), String> {
    println!("\n{}\n", t!(keys::WIZARD_GEMINI_TITLE));

    let model_prompt = t!(keys::WIZARD_GEMINI_PROMPT_MODEL);
    let model = crate::interactive::utils::prompt_optional(&model_prompt)?;
    let Some(model_name) = model else {
        println!("{}\n", t!(keys::WIZARD_NO_CHANGES_CANCELLED));
        return Ok(());
    };

    let scope_title = t!(keys::MENU_GEMINI_SETTINGS_SCOPE_TITLE);
    let scope_user = t!(keys::MENU_GEMINI_SETTINGS_SCOPE_USER);
    let scope_project = t!(keys::MENU_GEMINI_SETTINGS_SCOPE_PROJECT);
    let scope_choice = prompt_select(
        &scope_title,
        vec![scope_user.clone(), scope_project.clone()],
        0,
    )?;
    let (scope_flag, project_path) = if scope_choice == scope_project {
        let p = t!(keys::MENU_GEMINI_SETTINGS_PROJECT_PATH_PROMPT);
        let raw = prompt_line(&p)?;
        let v = raw.trim();
        (
            "project",
            if v.is_empty() {
                None
            } else {
                Some(v.to_string())
            },
        )
    } else {
        ("user", None)
    };

    // Preview.
    let mut preview = vec![
        "settings".to_string(),
        "set".to_string(),
        "--scope".to_string(),
        scope_flag.to_string(),
        "--model".to_string(),
        model_name.clone(),
    ];
    if let Some(p) = &project_path {
        preview.push("--project-path".to_string());
        preview.push(p.clone());
    }
    legacy::cmd_gemini(preview)?;

    let confirm = t!(keys::ACTION_CONFIRM_APPLY);
    if prompt_confirm(&confirm, false)? {
        let mut apply = vec![
            "settings".to_string(),
            "set".to_string(),
            "--scope".to_string(),
            scope_flag.to_string(),
            "--model".to_string(),
            model_name,
            "--apply".to_string(),
        ];
        if let Some(p) = project_path {
            apply.push("--project-path".to_string());
            apply.push(p);
        }
        legacy::cmd_gemini(apply)?;
    }

    println!();
    Ok(())
}

fn codex_provider_configure() -> Result<(), String> {
    use crate::interactive::utils::prompt_secret_with_env_default;
    use prismctl_core::providers;

    // Reuse wizard prompts to keep vocabulary consistent.
    println!("\n{}\n", t!(keys::WIZARD_CODEX_TITLE));

    let ids = providers::list_provider_ids();
    let mut options = ids.iter().map(|id| (*id).to_string()).collect::<Vec<_>>();
    let custom = t!(keys::WIZARD_CODEX_PROVIDER_CUSTOM);
    options.push(custom.clone());

    let default_idx = ids.iter().position(|id| *id == "openrouter").unwrap_or(0);
    let title = t!(keys::WIZARD_CODEX_PROVIDER_TITLE);
    let choice = prompt_select(&title, options, default_idx)?;

    let (provider_id, base_url, wire_api, model) = if choice == custom {
        let base_url_prompt = t!(keys::WIZARD_CODEX_PROMPT_BASE_URL);
        let base_url = crate::interactive::utils::prompt_optional(&base_url_prompt)?;
        if let Some(u) = &base_url {
            crate::interactive::utils::validate_http_url(u)?;
        }
        let wire_api_prompt = t!(keys::WIZARD_CODEX_PROMPT_WIRE_API);
        let wire_api = crate::interactive::utils::prompt_optional(&wire_api_prompt)?;
        let model_prompt = t!(keys::WIZARD_CODEX_PROMPT_MODEL);
        let model = crate::interactive::utils::prompt_optional(&model_prompt)?;
        ("custom".to_string(), base_url, wire_api, model)
    } else {
        // Validate early to give a friendly error message.
        providers::parse_provider_id(&choice)?;
        (choice, None, None, None)
    };

    // Provider set can be done without a key (e.g. relying on env).
    let api_key_prompt = t!(keys::WIZARD_CODEX_PROMPT_API_KEY);
    let api_key = prompt_secret_with_env_default(
        &api_key_prompt,
        &["PRISMCTL_CODEX_API_KEY", "PRISMCTL_API_KEY"],
    )?;
    if let Some(k) = &api_key {
        crate::interactive::utils::validate_api_key_format(k)?;
    }

    let set_default_prompt = t!(keys::WIZARD_CODEX_SET_DEFAULT);
    let set_default = prompt_confirm(&set_default_prompt, true)?;

    let mut args = vec!["provider".to_string(), "set".to_string()];
    if provider_id != "custom" {
        args.push("--provider".to_string());
        args.push(provider_id);
    }
    if let Some(v) = base_url {
        args.push("--base-url".to_string());
        args.push(v);
    }
    if let Some(v) = wire_api {
        args.push("--wire-api".to_string());
        args.push(v);
    }
    if let Some(v) = model {
        args.push("--model".to_string());
        args.push(v);
    }
    if let Some(v) = api_key {
        args.push("--api-key".to_string());
        args.push(v);
    }
    if set_default {
        args.push("--default".to_string());
    }

    legacy::cmd_codex(args.clone())?;

    let confirm = t!(keys::ACTION_CONFIRM_APPLY);
    if prompt_confirm(&confirm, false)? {
        args.push("--apply".to_string());
        legacy::cmd_codex(args)?;
        println!();
    }

    // Optional: jump to agent config, to match the common workflow.
    println!("\n{}", t!(keys::WIZARD_CODEX_SWITCH_AGENT_TITLE));
    let switch_prompt = t!(keys::WIZARD_CODEX_SWITCH_AGENT);
    if prompt_confirm(&switch_prompt, false)? {
        codex_agent_configure()?;
    }

    Ok(())
}

fn codex_agent_configure() -> Result<(), String> {
    use prismctl_core::templates;

    println!("\n{}", t!(keys::WIZARD_CODEX_SWITCH_AGENT_TITLE));

    let mut options = templates::codex_agent_names()
        .iter()
        .map(|s| (*s).to_string())
        .collect::<Vec<_>>();
    let custom = t!(keys::WIZARD_CODEX_AGENT_CUSTOM);
    options.push(custom.clone());

    let title = t!(keys::WIZARD_CODEX_AGENT_TITLE);
    let agent = prompt_select(&title, options, 0)?;
    let agent = if agent == custom {
        let prompt = t!(keys::WIZARD_CODEX_AGENT_NAME);
        prompt_required(&prompt)?
    } else {
        agent
    };

    let lang = crate::interactive::utils::prompt_lang_selection()?;

    let scope_title = t!(keys::MENU_CODEX_AGENT_SCOPE_TITLE);
    let scope_user = t!(keys::MENU_CODEX_AGENT_SCOPE_USER);
    let scope_project = t!(keys::MENU_CODEX_AGENT_SCOPE_PROJECT);
    let scope_choice = prompt_select(
        &scope_title,
        vec![scope_user.clone(), scope_project.clone()],
        0,
    )?;
    let scope_flag = if scope_choice == scope_project {
        "project"
    } else {
        "user"
    };
    let project_path = if scope_flag == "project" {
        let p = t!(keys::MENU_CODEX_AGENT_PROJECT_PATH_PROMPT);
        let raw = prompt_line(&p)?;
        let v = raw.trim();
        if v.is_empty() {
            None
        } else {
            Some(v.to_string())
        }
    } else {
        None
    };

    // Preview.
    let mut preview = vec![
        "agent".to_string(),
        "use".to_string(),
        "--name".to_string(),
        agent.clone(),
        "--lang".to_string(),
        lang.clone(),
        "--scope".to_string(),
        scope_flag.to_string(),
    ];
    if let Some(p) = &project_path {
        preview.push("--project-path".to_string());
        preview.push(p.clone());
    }
    legacy::cmd_codex(preview)?;

    let confirm = t!(keys::WIZARD_CODEX_CONFIRM_OVERWRITE_AGENTS);
    if prompt_confirm(&confirm, false)? {
        let mut apply = vec![
            "agent".to_string(),
            "use".to_string(),
            "--name".to_string(),
            agent,
            "--lang".to_string(),
            lang,
            "--scope".to_string(),
            scope_flag.to_string(),
            "--apply".to_string(),
            "--yes".to_string(),
        ];
        if let Some(p) = project_path {
            apply.push("--project-path".to_string());
            apply.push(p);
        }
        legacy::cmd_codex(apply)?;
    }

    println!();
    Ok(())
}

fn claude_mcp_menu() -> Result<(), String> {
    use prismctl_core::mcp;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum McpMenuItem {
        List,
        Builtin,
        Add,
        Get,
        Remove,
        Back,
    }

    impl fmt::Display for McpMenuItem {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let s = match self {
                Self::List => t!(keys::MENU_CLAUDE_MCP_LIST),
                Self::Builtin => t!(keys::MENU_CLAUDE_MCP_BUILTIN),
                Self::Add => t!(keys::MENU_CLAUDE_MCP_ADD),
                Self::Get => t!(keys::MENU_CLAUDE_MCP_GET),
                Self::Remove => t!(keys::MENU_CLAUDE_MCP_REMOVE),
                Self::Back => t!(keys::MENU_BACK),
            };
            write!(f, "{}", s)
        }
    }

    loop {
        let title = t!(keys::MENU_CLAUDE_MCP_TITLE);
        let choice = prompt_select(
            &title,
            vec![
                McpMenuItem::List,
                McpMenuItem::Builtin,
                McpMenuItem::Add,
                McpMenuItem::Get,
                McpMenuItem::Remove,
                McpMenuItem::Back,
            ],
            0,
        )?;

        match choice {
            McpMenuItem::List => {
                let p = t!(keys::MENU_CLAUDE_MCP_PROJECT_PATH_PROMPT_OPT);
                let raw = prompt_line(&p)?;
                let v = raw.trim();

                let mut cmd = vec!["mcp".to_string(), "list".to_string()];
                if !v.is_empty() {
                    cmd.push("--project-path".to_string());
                    cmd.push(v.to_string());
                }
                legacy::cmd_claude(cmd)?;
                let cont = t!(keys::ACTION_CONTINUE);
                let _ = prompt_line(&cont)?;
            }
            McpMenuItem::Builtin => {
                legacy::cmd_claude(vec!["mcp".to_string(), "builtin".to_string()])?;
                let cont = t!(keys::ACTION_CONTINUE);
                let _ = prompt_line(&cont)?;
            }
            McpMenuItem::Add => {
                // Print a quick catalog so users understand what each ID does.
                let _ = legacy::cmd_claude(vec!["mcp".to_string(), "builtin".to_string()]);

                let servers = mcp::list_builtin_claude_mcp_servers()
                    .iter()
                    .map(|s| s.id.to_string())
                    .collect::<Vec<_>>();

                let select_title = t!(keys::MENU_CLAUDE_MCP_PICK);
                let picked = prompt_multi_select(&select_title, servers, Vec::new())?;
                if picked.is_empty() {
                    println!("\n{}\n", t!(keys::MENU_CLAUDE_MCP_EMPTY));
                    continue;
                }

                let scope_title = t!(keys::MENU_CLAUDE_MCP_SCOPE_TITLE);
                let scope_local = t!(keys::MENU_CLAUDE_MCP_SCOPE_LOCAL);
                let scope_project = t!(keys::MENU_CLAUDE_MCP_SCOPE_PROJECT);
                let scope_user = t!(keys::MENU_CLAUDE_MCP_SCOPE_USER);
                let scope_choice = prompt_select(
                    &scope_title,
                    vec![
                        scope_local.clone(),
                        scope_project.clone(),
                        scope_user.clone(),
                    ],
                    0,
                )?;
                let scope_flag = if scope_choice == scope_project {
                    "project"
                } else if scope_choice == scope_user {
                    "user"
                } else {
                    "local"
                };

                let project_path = if scope_flag == "project" {
                    let p = t!(keys::MENU_CLAUDE_MCP_PROJECT_PATH_PROMPT);
                    let raw = prompt_line(&p)?;
                    let v = raw.trim();
                    if v.is_empty() {
                        None
                    } else {
                        Some(v.to_string())
                    }
                } else {
                    None
                };

                // Preview (dry-run).
                for id in &picked {
                    let mut cmd = vec![
                        "mcp".to_string(),
                        "add".to_string(),
                        "--name".to_string(),
                        id.clone(),
                        "--scope".to_string(),
                        scope_flag.to_string(),
                    ];
                    if let Some(p) = &project_path {
                        cmd.push("--project-path".to_string());
                        cmd.push(p.clone());
                    }
                    legacy::cmd_claude(cmd)?;
                }

                let confirm = t!(keys::ACTION_CONFIRM_APPLY);
                if prompt_confirm(&confirm, false)? {
                    for id in picked {
                        let mut cmd = vec![
                            "mcp".to_string(),
                            "add".to_string(),
                            "--name".to_string(),
                            id,
                            "--scope".to_string(),
                            scope_flag.to_string(),
                            "--apply".to_string(),
                            "--yes".to_string(),
                        ];
                        if let Some(p) = &project_path {
                            cmd.push("--project-path".to_string());
                            cmd.push(p.clone());
                        }
                        legacy::cmd_claude(cmd)?;
                    }
                }
            }
            McpMenuItem::Get => {
                let p = t!(keys::MENU_CLAUDE_MCP_NAME_PROMPT);
                let name = prompt_required(&p)?;
                let p = t!(keys::MENU_CLAUDE_MCP_PROJECT_PATH_PROMPT_OPT);
                let raw = prompt_line(&p)?;
                let v = raw.trim();

                let mut cmd = vec![
                    "mcp".to_string(),
                    "get".to_string(),
                    "--name".to_string(),
                    name,
                ];
                if !v.is_empty() {
                    cmd.push("--project-path".to_string());
                    cmd.push(v.to_string());
                }
                legacy::cmd_claude(cmd)?;
                let cont = t!(keys::ACTION_CONTINUE);
                let _ = prompt_line(&cont)?;
            }
            McpMenuItem::Remove => {
                let p = t!(keys::MENU_CLAUDE_MCP_NAME_PROMPT);
                let name = prompt_required(&p)?;
                let p = t!(keys::MENU_CLAUDE_MCP_PROJECT_PATH_PROMPT_OPT);
                let raw = prompt_line(&p)?;
                let v = raw.trim();

                // Preview (dry-run).
                let mut preview = vec![
                    "mcp".to_string(),
                    "remove".to_string(),
                    "--name".to_string(),
                    name.clone(),
                ];
                if !v.is_empty() {
                    preview.push("--project-path".to_string());
                    preview.push(v.to_string());
                }
                legacy::cmd_claude(preview)?;

                let confirm = t!(keys::ACTION_CONFIRM_APPLY);
                if prompt_confirm(&confirm, false)? {
                    let mut apply = vec![
                        "mcp".to_string(),
                        "remove".to_string(),
                        "--name".to_string(),
                        name,
                        "--apply".to_string(),
                        "--yes".to_string(),
                    ];
                    if !v.is_empty() {
                        apply.push("--project-path".to_string());
                        apply.push(v.to_string());
                    }
                    legacy::cmd_claude(apply)?;
                }
            }
            McpMenuItem::Back => return Ok(()),
        }
    }
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
    let cont = t!(keys::ACTION_CONTINUE);
    let _ = prompt_line(&cont)?;
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
