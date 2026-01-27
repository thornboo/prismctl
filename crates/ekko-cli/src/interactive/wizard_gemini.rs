use crate::interactive::utils::{
    prompt_confirm, prompt_optional, prompt_secret_with_env_default, validate_api_key_format,
    validate_http_url,
};
use crate::legacy;
use ekko_i18n::{keys, t};

pub fn wizard_configure_gemini() -> Result<(), String> {
    println!("\n{}\n", t!(keys::WIZARD_GEMINI_TITLE));

    let api_key_prompt = t!(keys::WIZARD_GEMINI_PROMPT_API_KEY);
    let api_key =
        prompt_secret_with_env_default(&api_key_prompt, &["GEMINI_API_KEY", "EKKO_API_KEY"])?;
    if let Some(k) = &api_key {
        validate_api_key_format(k)?;
    }
    let base_url_prompt = t!(keys::WIZARD_GEMINI_PROMPT_BASE_URL);
    let base_url = prompt_optional(&base_url_prompt)?;
    if let Some(u) = &base_url {
        validate_http_url(u)?;
    }
    let model_prompt = t!(keys::WIZARD_GEMINI_PROMPT_MODEL);
    let model = prompt_optional(&model_prompt)?;

    if api_key.is_none() && base_url.is_none() && model.is_none() {
        println!("{}\n", t!(keys::WIZARD_NO_CHANGES_CANCELLED));
        return Ok(());
    }

    let mut args = vec!["env".to_string(), "set".to_string()];
    if let Some(v) = api_key {
        args.push("--api-key".to_string());
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

    legacy::cmd_gemini(args.clone())?;

    let confirm = t!(keys::ACTION_CONFIRM_APPLY);
    if !prompt_confirm(&confirm, false)? {
        println!();
        return Ok(());
    }

    args.push("--apply".to_string());
    legacy::cmd_gemini(args)?;

    println!();
    Ok(())
}
