use crate::interactive::utils::{
    prompt_confirm, prompt_lang_selection, prompt_optional, prompt_required,
    prompt_secret_with_env_default, prompt_select, validate_api_key_format, validate_http_url,
};
use crate::legacy;
use prismctl_core::providers;
use prismctl_core::templates;
use prismctl_i18n::{keys, t};

pub fn wizard_configure_codex() -> Result<(), String> {
    println!("\n{}\n", t!(keys::WIZARD_CODEX_TITLE));

    let provider_id = prompt_provider_id()?;
    let (base_url, wire_api, model) = if provider_id == "custom" {
        let prompt = t!(keys::WIZARD_CODEX_PROMPT_BASE_URL);
        let base_url = prompt_optional(&prompt)?;
        if let Some(u) = &base_url {
            validate_http_url(u)?;
        }
        (
            base_url,
            {
                let prompt = t!(keys::WIZARD_CODEX_PROMPT_WIRE_API);
                prompt_optional(&prompt)?
            },
            {
                let prompt = t!(keys::WIZARD_CODEX_PROMPT_MODEL);
                prompt_optional(&prompt)?
            },
        )
    } else {
        // Preset values are resolved inside `cmd_codex_provider_set` when passing `--provider`.
        (None, None, None)
    };

    let api_key_prompt = t!(keys::WIZARD_CODEX_PROMPT_API_KEY);
    let api_key = prompt_secret_with_env_default(
        &api_key_prompt,
        &["PRISMCTL_CODEX_API_KEY", "PRISMCTL_API_KEY"],
    )?;
    if let Some(k) = &api_key {
        validate_api_key_format(k)?;
    }

    let set_default_prompt = t!(keys::WIZARD_CODEX_SET_DEFAULT);
    let set_default = prompt_confirm(&set_default_prompt, true)?;

    let mut args = vec!["provider".to_string(), "set".to_string()];
    if provider_id != "custom" {
        // Validate early to give a friendly error message.
        providers::parse_provider_id(&provider_id)?;
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

    // Preview.
    legacy::cmd_codex(args.clone())?;

    let confirm = t!(keys::ACTION_CONFIRM_APPLY);
    if !prompt_confirm(&confirm, false)? {
        println!();
        return Ok(());
    }

    args.push("--apply".to_string());
    legacy::cmd_codex(args)?;

    // Optional: pick and apply agent.
    println!("\n{}", t!(keys::WIZARD_CODEX_SWITCH_AGENT_TITLE));
    let switch_prompt = t!(keys::WIZARD_CODEX_SWITCH_AGENT);
    if prompt_confirm(&switch_prompt, false)? {
        let mut options = templates::codex_agent_names()
            .iter()
            .map(|s| (*s).to_string())
            .collect::<Vec<_>>();
        let custom = t!(keys::WIZARD_CODEX_AGENT_CUSTOM);
        options.push(custom.clone());

        let agent_title = t!(keys::WIZARD_CODEX_AGENT_TITLE);
        let agent = prompt_select(&agent_title, options, 0)?;
        let agent = if agent == custom {
            let prompt = t!(keys::WIZARD_CODEX_AGENT_NAME);
            prompt_required(&prompt)?
        } else {
            agent
        };
        let lang = prompt_lang_selection()?;

        legacy::cmd_codex(vec![
            "agent".to_string(),
            "use".to_string(),
            "--name".to_string(),
            agent.clone(),
            "--lang".to_string(),
            lang.clone(),
        ])?;

        let confirm = t!(keys::WIZARD_CODEX_CONFIRM_OVERWRITE_AGENTS);
        if prompt_confirm(&confirm, false)? {
            legacy::cmd_codex(vec![
                "agent".to_string(),
                "use".to_string(),
                "--name".to_string(),
                agent,
                "--lang".to_string(),
                lang,
                "--apply".to_string(),
                "--yes".to_string(),
            ])?;
        }
    }

    println!();
    Ok(())
}

fn prompt_provider_id() -> Result<String, String> {
    let ids = providers::list_provider_ids();
    let mut options = ids.iter().map(|id| (*id).to_string()).collect::<Vec<_>>();
    let custom = t!(keys::WIZARD_CODEX_PROVIDER_CUSTOM);
    options.push(custom.clone());

    let default_idx = ids.iter().position(|id| *id == "openrouter").unwrap_or(0);

    let title = t!(keys::WIZARD_CODEX_PROVIDER_TITLE);
    let choice = prompt_select(&title, options, default_idx)?;
    if choice == custom {
        return Ok("custom".to_string());
    }
    Ok(choice)
}
