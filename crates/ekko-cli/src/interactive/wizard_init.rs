use crate::interactive;
use crate::interactive::utils::{prompt_confirm, prompt_lang_selection, prompt_multi_select};
use crate::legacy;
use ekko_i18n::{keys, t};

pub fn wizard_quick_init() -> Result<(), String> {
    println!("\n{}\n", t!(keys::WIZARD_QUICK_INIT_TITLE));
    println!("{}", t!(keys::WIZARD_QUICK_INIT_DESC));
    println!("  - {}", t!(keys::WIZARD_QUICK_INIT_DESC_INIT));
    println!("  - {}\n", t!(keys::WIZARD_QUICK_INIT_DESC_CONFIG));

    let tools = prompt_tools_multi_select()?;
    let lang = prompt_lang_selection()?;

    // Step 1: init templates (dry-run) for selected tools.
    let init_tools: Vec<String> = if tools.len() == 3 {
        vec!["all".to_string()]
    } else {
        tools.clone()
    };
    for tool in &init_tools {
        legacy::cmd_init(vec![
            "--tool".to_string(),
            tool.clone(),
            "--lang".to_string(),
            lang.clone(),
        ])?;
    }

    let confirm = t!(keys::WIZARD_QUICK_INIT_CONFIRM_WRITE);
    if prompt_confirm(&confirm, false)? {
        for tool in &init_tools {
            legacy::cmd_init(vec![
                "--tool".to_string(),
                tool.clone(),
                "--lang".to_string(),
                lang.clone(),
                "--apply".to_string(),
            ])?;
        }
    }

    // Step 2: optional per-tool configuration.
    println!("\n{}", t!(keys::WIZARD_QUICK_INIT_CONTINUE_TITLE));
    let cont = t!(keys::WIZARD_QUICK_INIT_CONTINUE);
    if !prompt_confirm(&cont, false)? {
        println!();
        return Ok(());
    }

    for tool in tools {
        match tool.as_str() {
            "claude" => {
                let p = t!(keys::WIZARD_QUICK_INIT_CONFIGURE_CLAUDE);
                if prompt_confirm(&p, true)? {
                    interactive::wizard_claude::wizard_configure_claude()?;
                }
            }
            "codex" => {
                let p = t!(keys::WIZARD_QUICK_INIT_CONFIGURE_CODEX);
                if prompt_confirm(&p, true)? {
                    interactive::wizard_codex::wizard_configure_codex()?;
                }
            }
            "gemini" => {
                let p = t!(keys::WIZARD_QUICK_INIT_CONFIGURE_GEMINI);
                if prompt_confirm(&p, true)? {
                    interactive::wizard_gemini::wizard_configure_gemini()?;
                }
            }
            _ => {}
        }
    }

    println!();
    Ok(())
}

fn prompt_tools_multi_select() -> Result<Vec<String>, String> {
    let options = vec![
        "claude".to_string(),
        "codex".to_string(),
        "gemini".to_string(),
    ];
    let prompt = t!(keys::WIZARD_QUICK_INIT_SELECT_TOOLS);
    let selected = prompt_multi_select(&prompt, options, vec![0, 1, 2])?;
    if selected.is_empty() {
        return Ok(vec![
            "claude".to_string(),
            "codex".to_string(),
            "gemini".to_string(),
        ]);
    }
    Ok(selected)
}
