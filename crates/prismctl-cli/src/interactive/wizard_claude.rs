use crate::interactive::utils::{
    prompt_confirm, prompt_line, prompt_optional, prompt_required, prompt_secret_with_env_default,
    prompt_select, validate_api_key_format, validate_http_url,
};
use crate::legacy;
use prismctl_core::skill;
use prismctl_i18n::{keys, t, tf};

pub fn wizard_configure_claude() -> Result<(), String> {
    println!("\n{}\n", t!(keys::WIZARD_CLAUDE_TITLE));

    let base_url = prompt_provider_base_url()?;
    let auth_token_prompt = t!(keys::WIZARD_CLAUDE_PROMPT_AUTH_TOKEN);
    let auth_token = prompt_secret_with_env_default(
        &auth_token_prompt,
        &[
            "PRISMCTL_CLAUDE_AUTH_TOKEN",
            "ANTHROPIC_AUTH_TOKEN",
            "PRISMCTL_API_KEY",
        ],
    )?;
    if let Some(k) = &auth_token {
        validate_api_key_format(k)?;
    }

    let model_prompt = t!(keys::WIZARD_CLAUDE_PROMPT_MODEL);
    let model = prompt_optional(&model_prompt)?;
    let haiku_prompt = t!(keys::WIZARD_CLAUDE_PROMPT_HAIKU_MODEL);
    let haiku_model = prompt_optional(&haiku_prompt)?;
    let sonnet_prompt = t!(keys::WIZARD_CLAUDE_PROMPT_SONNET_MODEL);
    let sonnet_model = prompt_optional(&sonnet_prompt)?;
    let opus_prompt = t!(keys::WIZARD_CLAUDE_PROMPT_OPUS_MODEL);
    let opus_model = prompt_optional(&opus_prompt)?;

    if auth_token.is_none()
        && base_url.is_none()
        && model.is_none()
        && haiku_model.is_none()
        && sonnet_model.is_none()
        && opus_model.is_none()
    {
        println!("{}\n", t!(keys::WIZARD_NO_CHANGES_CANCELLED));
        return Ok(());
    }

    let mut args = vec!["env".to_string(), "set".to_string()];
    if let Some(v) = auth_token {
        args.push("--auth-token".to_string());
        args.push(v);
    }
    if let Some(v) = base_url {
        args.push("--base-url".to_string());
        args.push(v);
    }
    if let Some(v) = model {
        args.push("--model".to_string());
        args.push(v);
    }
    if let Some(v) = haiku_model {
        args.push("--haiku-model".to_string());
        args.push(v);
    }
    if let Some(v) = sonnet_model {
        args.push("--sonnet-model".to_string());
        args.push(v);
    }
    if let Some(v) = opus_model {
        args.push("--opus-model".to_string());
        args.push(v);
    }

    legacy::cmd_claude(args.clone())?;

    let confirm = t!(keys::ACTION_CONFIRM_APPLY);
    if !prompt_confirm(&confirm, false)? {
        println!();
        return Ok(());
    }

    args.push("--apply".to_string());
    legacy::cmd_claude(args)?;

    // Optional: output style.
    println!("\n{}", t!(keys::WIZARD_CLAUDE_OUTPUT_STYLE_TITLE));
    let p = t!(keys::WIZARD_CLAUDE_OUTPUT_STYLE_CONFIRM);
    if prompt_confirm(&p, false)? {
        let style = prompt_output_style()?;
        legacy::cmd_claude(vec![
            "output-style".to_string(),
            "use".to_string(),
            "--name".to_string(),
            style.clone(),
        ])?;
        let confirm = t!(keys::ACTION_CONFIRM_APPLY);
        if prompt_confirm(&confirm, false)? {
            legacy::cmd_claude(vec![
                "output-style".to_string(),
                "use".to_string(),
                "--name".to_string(),
                style,
                "--apply".to_string(),
            ])?;
        }
    }

    // Optional: install skills.
    println!("\n{}", t!(keys::WIZARD_CLAUDE_SKILLS_TITLE));
    let p = t!(keys::WIZARD_CLAUDE_SKILLS_CONFIRM);
    if prompt_confirm(&p, false)? {
        let skills = prompt_skills_multi_select()?;
        if skills.is_empty() {
            println!("{}\n", t!(keys::WIZARD_CLAUDE_SKILLS_EMPTY_SKIP));
        } else {
            for s in &skills {
                legacy::cmd_skill(vec!["install".to_string(), "--name".to_string(), s.clone()])?;
            }

            let confirm = t!(keys::WIZARD_CLAUDE_SKILLS_CONFIRM_WRITE);
            if prompt_confirm(&confirm, false)? {
                for s in skills {
                    legacy::cmd_skill(vec![
                        "install".to_string(),
                        "--name".to_string(),
                        s,
                        "--apply".to_string(),
                    ])?;
                }
            }
        }
    }

    println!();
    Ok(())
}

fn prompt_provider_base_url() -> Result<Option<String>, String> {
    let title = t!(keys::WIZARD_CLAUDE_PROVIDER_TITLE);
    let custom = t!(keys::WIZARD_CLAUDE_PROVIDER_CUSTOM);
    let skip = t!(keys::WIZARD_CLAUDE_PROVIDER_SKIP);
    let choice = prompt_select(
        &title,
        vec![
            "openrouter".to_string(),
            "deepseek".to_string(),
            "ollama".to_string(),
            custom.clone(),
            skip.clone(),
        ],
        0,
    )?;

    if choice == custom {
        let p = t!(keys::WIZARD_CLAUDE_PROMPT_BASE_URL);
        let v = prompt_required(&p)?;
        validate_http_url(&v)?;
        return Ok(Some(v));
    }

    if choice == skip {
        return Ok(None);
    }

    match choice.as_str() {
        "openrouter" => Ok(Some("https://openrouter.ai/api/v1".to_string())),
        "deepseek" => Ok(Some("https://api.deepseek.com/v1".to_string())),
        "ollama" => Ok(Some("http://localhost:11434/v1".to_string())),
        _ => Err(tf!(keys::ERROR_INVALID_CHOICE, "choice" => choice)),
    }
}

fn prompt_output_style() -> Result<String, String> {
    let styles = [
        "prismctl-engineer-professional",
        "prismctl-laowang-engineer",
        "prismctl-leibus-engineer",
        "prismctl-nekomata-engineer",
        "prismctl-ojousama-engineer",
        "prismctl-rem-engineer",
    ];
    let mut options = styles.iter().map(|s| (*s).to_string()).collect::<Vec<_>>();
    let custom = t!(keys::WIZARD_CLAUDE_OUTPUT_STYLE_CUSTOM);
    options.push(custom.clone());

    let title = t!(keys::WIZARD_CLAUDE_OUTPUT_STYLE_SELECT);
    let choice = prompt_select(&title, options, 0)?;
    if choice == custom {
        let p = t!(keys::WIZARD_CLAUDE_OUTPUT_STYLE_NAME);
        return prompt_required(&p);
    }
    Ok(choice)
}

fn prompt_skills_multi_select() -> Result<Vec<String>, String> {
    let builtin = skill::list_builtin_skills();
    println!("\n{}", t!(keys::WIZARD_CLAUDE_SKILLS_BUILTIN_TITLE));
    for s in &builtin {
        println!("  - {}", s);
    }
    println!("{}", t!(keys::WIZARD_CLAUDE_SKILLS_INSTRUCTION));
    println!("{}", t!(keys::WIZARD_CLAUDE_SKILLS_EMPTY));
    let raw = prompt_line("> ")?;
    if raw.trim().is_empty() {
        return Ok(Vec::new());
    }
    let out = raw
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
    Ok(out)
}
