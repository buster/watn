pub mod dialog;
pub mod list;
pub mod picker;

use dialoguer::Select;

use crate::config::{resolve_provider, save_config};
use crate::config::types::ModelTiers;
use crate::error::Error;
use crate::models::dialog::{ReasoningStrength, SettingsDialog};
use crate::provider::setup::{ModelSetupResult, SetupCancellation};
use list::{fetch_models, fetch_models_page};
use std::io::IsTerminal;

pub fn run_models_result(
    set_small: Option<String>,
    set_normal: Option<String>,
    set_thinking: Option<String>,
) -> ModelSetupResult {
    let config = match crate::config::load_config() {
        Ok(config) => config,
        Err(error) => return ModelSetupResult::Failed(error),
    };

    if let (Some(small), Some(normal), Some(thinking)) =
        (&set_small, &set_normal, &set_thinking)
    {
        let mut updated = config.clone();
        updated.tiers.small = Some(small.clone());
        updated.tiers.normal = Some(normal.clone());
        updated.tiers.thinking = Some(thinking.clone());
        if let Err(e) = save_config(&updated) {
            return ModelSetupResult::Failed(e);
        }
        println!(
            "Tiers configured: small={}, normal={}, thinking={}",
            small, normal, thinking
        );
        return ModelSetupResult::Saved;
    }

    let provider_name = config
        .defaults
        .provider
        .as_deref()
        .unwrap_or("openrouter");

    let provider_config = match resolve_provider(&config, provider_name) {
        Ok(p) => p,
        Err(_) => {
            println!("No provider endpoint configured.");
            println!("To configure providers manually, edit ~/.config/watn/config.toml");
            println!("See the configuration guide for details.");
            return ModelSetupResult::Saved;
        }
    };

    let endpoint = provider_config.endpoint.clone();
    let api_key = match crate::config::get_provider_api_key(provider_name, &provider_config) {
        Ok(k) => Some(k),
        Err(_) => None,
    };

    let models = match fetch_models_page(&endpoint, 1, 50, api_key.as_deref()) {
        Ok(m) if !m.is_empty() => m,
        _ => match fetch_models(&endpoint, api_key.as_deref()) {
            Ok(m) => m,
            Err(e) => return ModelSetupResult::Failed(e),
        },
    };

    if models.is_empty() {
        return ModelSetupResult::Failed(Error::ConfigError(
            "no models returned from endpoint".to_string(),
        ));
    }

    let small;
    let normal;
    let thinking;
    let reasoning: [String; 3];

    if std::io::stdin().is_terminal() {
        // Keyboard-driven dialog (ratatui) covering all three levels in a
        // guided sequence, each with a model pick + reasoning strength.
        let parse = |r: &Option<String>| {
            r.as_deref()
                .and_then(ReasoningStrength::parse)
                .unwrap_or(ReasoningStrength::Off)
        };
        let initial = [
            parse(&config.tiers.reasoning.small),
            parse(&config.tiers.reasoning.normal),
            parse(&config.tiers.reasoning.thinking),
        ];
        let dialog = SettingsDialog::new(endpoint, api_key.clone(), models.clone(), initial);
        match dialog.run() {
            Ok(choices) => {
                small = choices[0].model.clone();
                normal = choices[1].model.clone();
                thinking = choices[2].model.clone();
                reasoning = [
                    choices[0].reasoning.as_str().to_string(),
                    choices[1].reasoning.as_str().to_string(),
                    choices[2].reasoning.as_str().to_string(),
                ];
            }
            Err(error) if error.to_string().contains("interrupted") => {
                return ModelSetupResult::Cancelled(SetupCancellation::CtrlC);
            }
            Err(error) => return ModelSetupResult::Failed(error),
        }
    } else {
        small = match select_model(&models, "small") {
            Ok(model) => model.clone(),
            Err(error) => return ModelSetupResult::Failed(error),
        };
        normal = match select_model(&models, "normal") {
            Ok(model) => model.clone(),
            Err(error) => return ModelSetupResult::Failed(error),
        };
        thinking = match select_model(&models, "thinking") {
            Ok(model) => model.clone(),
            Err(error) => return ModelSetupResult::Failed(error),
        };
        reasoning = Default::default();
    }

    let mut updated = config.clone();
    updated.tiers.reasoning = crate::config::types::TierReasoning {
        small: Some(reasoning[0].clone()),
        normal: Some(reasoning[1].clone()),
        thinking: Some(reasoning[2].clone()),
    };
    updated.tiers = ModelTiers {
        small: Some(small.id.clone()),
        normal: Some(normal.id.clone()),
        thinking: Some(thinking.id.clone()),
        reasoning: updated.tiers.reasoning.clone(),
    };

    if let Err(e) = save_config(&updated) {
        return ModelSetupResult::Failed(e);
    }

    println!(
        "Tiers configured: small={}, normal={}, thinking={}",
        small.id, normal.id, thinking.id
    );
    ModelSetupResult::Saved
}

pub fn format_model_entry(entry: &list::ModelEntry) -> String {
    let mut parts = vec![format!("{}", entry.id)];

    if let Some(ref name) = entry.name {
        parts.push(format!("({})", name));
    }
    if let Some(ctx) = entry.context_length {
        parts.push(format!("{}K ctx", ctx / 1000));
    }
    if let Some(ref pricing) = entry.pricing {
        parts.push(format!(
            "${:.2}/{}K in, ${:.2}/{}K out",
            pricing.input,
            1,
            pricing.output,
            1
        ));
    }
    if !entry.supported_features.is_empty() {
        parts.push(format!("[{}]", entry.supported_features.join(", ")));
    }

    parts.join(" ")
}

fn select_model<'a>(models: &'a [list::ModelEntry], tier: &str) -> Result<&'a list::ModelEntry, Error> {
    let selections: Vec<String> = models
        .iter()
        .map(format_model_entry)
        .collect();

    let prompt = format!("Select a model for the {} tier:", tier);

    let selection = if std::io::stdin().is_terminal() {
        Select::new()
            .with_prompt(&prompt)
            .items(&selections)
            .default(0)
            .interact()
            .map_err(|_| Error::IoError(std::io::Error::other("failed to read selection")))?
    } else {
        select_model_non_interactive(&selections, tier)?
    };

    Ok(&models[selection])
}

fn select_model_non_interactive(selections: &[String], tier: &str) -> Result<usize, Error> {
    eprintln!("{}", selections.join("\n"));
    eprintln!();
    eprint!("Enter index for {} tier: ", tier);
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .map_err(|_| Error::IoError(std::io::Error::other("failed to read input")))?;
    let index: usize = input
        .trim()
        .parse()
        .map_err(|_| Error::ConfigError("invalid index".to_string()))?;
    if index >= selections.len() {
        return Err(Error::ConfigError("index out of range".to_string()));
    }
    Ok(index)
}
