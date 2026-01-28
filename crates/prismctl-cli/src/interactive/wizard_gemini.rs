use crate::interactive::utils::{
    prompt_confirm, prompt_line, prompt_multi_select, prompt_optional, prompt_secret_with_env_default,
    prompt_select, validate_api_key_format,
};
use crate::legacy;
use prismctl_core::mcp;
use prismctl_i18n::{keys, t};

pub fn wizard_configure_gemini() -> Result<(), String> {
    println!("\n{}\n", t!(keys::WIZARD_GEMINI_TITLE));

    let api_key_prompt = t!(keys::WIZARD_GEMINI_PROMPT_API_KEY);
    let api_key =
        prompt_secret_with_env_default(&api_key_prompt, &["GEMINI_API_KEY", "PRISMCTL_API_KEY"])?;
    if let Some(k) = &api_key {
        validate_api_key_format(k)?;
    }
    let model_prompt = t!(keys::WIZARD_GEMINI_PROMPT_MODEL);
    let model = prompt_optional(&model_prompt)?;

    if api_key.is_none() && model.is_none() {
        println!("{}\n", t!(keys::WIZARD_NO_CHANGES_CANCELLED));
        return Ok(());
    }

    // API key scope (user vs project).
    let (env_scope_flag, env_project_path) = if api_key.is_some() {
        let title = t!(keys::MENU_GEMINI_ENV_SCOPE_TITLE);
        let user = t!(keys::MENU_GEMINI_ENV_SCOPE_USER);
        let project = t!(keys::MENU_GEMINI_ENV_SCOPE_PROJECT);
        let choice = crate::interactive::utils::prompt_select(&title, vec![user.clone(), project.clone()], 0)?;
        if choice == project {
            let p = t!(keys::MENU_GEMINI_ENV_PROJECT_PATH_PROMPT);
            let raw = crate::interactive::utils::prompt_line(&p)?;
            let v = raw.trim();
            ("project", if v.is_empty() { None } else { Some(v.to_string()) })
        } else {
            ("user", None)
        }
    } else {
        ("user", None)
    };

    // settings.json scope (user vs project).
    let (settings_scope_flag, settings_project_path) = if model.is_some() {
        let title = t!(keys::MENU_GEMINI_SETTINGS_SCOPE_TITLE);
        let user = t!(keys::MENU_GEMINI_SETTINGS_SCOPE_USER);
        let project = t!(keys::MENU_GEMINI_SETTINGS_SCOPE_PROJECT);
        let choice = crate::interactive::utils::prompt_select(&title, vec![user.clone(), project.clone()], 0)?;
        if choice == project {
            let p = t!(keys::MENU_GEMINI_SETTINGS_PROJECT_PATH_PROMPT);
            let raw = crate::interactive::utils::prompt_line(&p)?;
            let v = raw.trim();
            ("project", if v.is_empty() { None } else { Some(v.to_string()) })
        } else {
            ("user", None)
        }
    } else {
        ("user", None)
    };

    // Preview.
    if let Some(v) = &api_key {
        let mut cmd = vec![
            "env".to_string(),
            "set".to_string(),
            "--scope".to_string(),
            env_scope_flag.to_string(),
            "--api-key".to_string(),
            v.clone(),
        ];
        if let Some(p) = &env_project_path {
            cmd.push("--project-path".to_string());
            cmd.push(p.clone());
        }
        legacy::cmd_gemini(cmd)?;
    }
    if let Some(v) = &model {
        let mut cmd = vec![
            "settings".to_string(),
            "set".to_string(),
            "--scope".to_string(),
            settings_scope_flag.to_string(),
            "--model".to_string(),
            v.clone(),
        ];
        if let Some(p) = &settings_project_path {
            cmd.push("--project-path".to_string());
            cmd.push(p.clone());
        }
        legacy::cmd_gemini(cmd)?;
    }

    let confirm = t!(keys::ACTION_CONFIRM_APPLY);
    if !prompt_confirm(&confirm, false)? {
        println!();
        return Ok(());
    }

    if let Some(v) = api_key {
        let mut cmd = vec![
            "env".to_string(),
            "set".to_string(),
            "--scope".to_string(),
            env_scope_flag.to_string(),
            "--api-key".to_string(),
            v,
            "--apply".to_string(),
        ];
        if let Some(p) = env_project_path {
            cmd.push("--project-path".to_string());
            cmd.push(p);
        }
        legacy::cmd_gemini(cmd)?;
    }
    if let Some(v) = model {
        let mut cmd = vec![
            "settings".to_string(),
            "set".to_string(),
            "--scope".to_string(),
            settings_scope_flag.to_string(),
            "--model".to_string(),
            v,
            "--apply".to_string(),
        ];
        if let Some(p) = settings_project_path {
            cmd.push("--project-path".to_string());
            cmd.push(p);
        }
        legacy::cmd_gemini(cmd)?;
    }

    // Optional: configure MCP servers (delegates to `gemini mcp add`).
    println!("\n{}", t!(keys::WIZARD_GEMINI_MCP_TITLE));
    let p = t!(keys::WIZARD_GEMINI_MCP_CONFIRM);
    if prompt_confirm(&p, false)? {
        let _ = legacy::cmd_gemini(vec!["mcp".to_string(), "builtin".to_string()]);

        let options = mcp::list_builtin_mcp_servers()
            .iter()
            .map(|s| s.id.to_string())
            .collect::<Vec<_>>();
        let title = t!(keys::WIZARD_GEMINI_MCP_SELECT);
        let selected = prompt_multi_select(&title, options, Vec::new())?;
        if selected.is_empty() {
            println!("{}\n", t!(keys::WIZARD_GEMINI_MCP_EMPTY_SKIP));
        } else {
            let scope_title = t!(keys::WIZARD_GEMINI_MCP_SCOPE_TITLE);
            let scope_project = t!(keys::WIZARD_GEMINI_MCP_SCOPE_PROJECT);
            let scope_user = t!(keys::WIZARD_GEMINI_MCP_SCOPE_USER);
            let scope_choice =
                prompt_select(&scope_title, vec![scope_project.clone(), scope_user.clone()], 0)?;
            let scope_flag = if scope_choice == scope_user { "user" } else { "project" };

            let project_path = if scope_flag == "project" {
                let p = t!(keys::WIZARD_GEMINI_MCP_PROJECT_PATH_PROMPT);
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

            // Preview planned changes (dry-run).
            for id in &selected {
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

            let confirm = t!(keys::WIZARD_GEMINI_MCP_CONFIRM_WRITE);
            if prompt_confirm(&confirm, false)? {
                for id in selected {
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
    }

    println!();
    Ok(())
}
