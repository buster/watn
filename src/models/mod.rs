pub mod dialog;
pub mod list;
pub mod picker;

use crate::config::types::ModelTiers;
use crate::config::{resolve_provider, save_config};
use crate::error::Error;
use crate::provider::setup::ModelSetupResult;
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

    if let Some(result) = save_explicit_tiers(&config, &set_small, &set_normal, &set_thinking) {
        return result;
    }

    let provider_name = config.defaults.provider.as_deref().unwrap_or("openrouter");
    let provider_config = match resolve_provider(&config, provider_name) {
        Ok(provider) => provider,
        Err(_) => {
            eprintln!("No provider is configured. Run `watn provider` in a terminal.");
            return ModelSetupResult::Saved;
        }
    };
    let api_key = match crate::config::get_provider_api_key(provider_name, &provider_config) {
        Ok(key) => Some(key),
        Err(error) => return ModelSetupResult::Failed(error),
    };

    if std::io::stdin().is_terminal() {
        return run_interactive_models(&config);
    }
    run_non_interactive_models(&config, &provider_config.endpoint, api_key.as_deref())
}

fn save_explicit_tiers(
    config: &crate::config::types::Config,
    set_small: &Option<String>,
    set_normal: &Option<String>,
    set_thinking: &Option<String>,
) -> Option<ModelSetupResult> {
    let (Some(small), Some(normal), Some(thinking)) = (set_small, set_normal, set_thinking) else {
        return None;
    };
    let mut updated = config.clone();
    updated.tiers.small = Some(small.clone());
    updated.tiers.normal = Some(normal.clone());
    updated.tiers.thinking = Some(thinking.clone());
    if let Err(error) = save_config(&updated) {
        return Some(ModelSetupResult::Failed(error));
    }
    println!(
        "Tiers configured: small={}, normal={}, thinking={}",
        small, normal, thinking
    );
    Some(ModelSetupResult::Saved)
}

fn run_interactive_models(config: &crate::config::types::Config) -> ModelSetupResult {
    match crate::setup::run_with_config(config, crate::setup::SetupEntryPoint::Models) {
        Ok(crate::setup::SetupWizardOutcome::Saved(result)) => {
            let mut updated = config.clone();
            match crate::setup::apply_models_result(&mut updated, &result) {
                Ok(()) => {
                    if result.choices.iter().all(Option::is_some) {
                        println!(
                            "Tiers configured: small={}, normal={}, thinking={}",
                            result.choices[0].as_ref().unwrap().model.id,
                            result.choices[1].as_ref().unwrap().model.id,
                            result.choices[2].as_ref().unwrap().model.id
                        );
                    }
                    ModelSetupResult::Saved
                }
                Err(error) => ModelSetupResult::Failed(error),
            }
        }
        Ok(crate::setup::SetupWizardOutcome::Cancelled(cancellation)) => {
            ModelSetupResult::Cancelled(cancellation)
        }
        Err(error) => ModelSetupResult::Failed(error),
    }
}

fn run_non_interactive_models(
    config: &crate::config::types::Config,
    endpoint: &str,
    api_key: Option<&str>,
) -> ModelSetupResult {
    let models = match fetch_models_page(endpoint, 1, 50, api_key) {
        Ok(models) if !models.is_empty() => models,
        _ => match fetch_models(endpoint, api_key) {
            Ok(models) => models,
            Err(error) => return ModelSetupResult::Failed(error),
        },
    };
    if models.is_empty() {
        return ModelSetupResult::Failed(Error::ConfigError(
            "no models returned from endpoint".to_string(),
        ));
    }

    let small = match select_model(&models, "small") {
        Ok(model) => model.clone(),
        Err(error) => return ModelSetupResult::Failed(error),
    };
    let normal = match select_model(&models, "normal") {
        Ok(model) => model.clone(),
        Err(error) => return ModelSetupResult::Failed(error),
    };
    let thinking = match select_model(&models, "thinking") {
        Ok(model) => model.clone(),
        Err(error) => return ModelSetupResult::Failed(error),
    };
    let mut updated = config.clone();
    let reasoning = updated.tiers.reasoning.clone();
    updated.tiers = ModelTiers {
        small: Some(small.id.clone()),
        normal: Some(normal.id.clone()),
        thinking: Some(thinking.id.clone()),
        reasoning,
    };
    for entry in [&small, &normal, &thinking] {
        crate::setup::capture_catalog_price(&mut updated, entry);
    }

    if let Err(error) = save_config(&updated) {
        return ModelSetupResult::Failed(error);
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
            "${:.2}/1M in, ${:.2}/1M out",
            pricing.input, pricing.output
        ));
    }
    if !entry.supported_features.is_empty() {
        parts.push(format!("[{}]", entry.supported_features.join(", ")));
    }

    parts.join(" ")
}

fn select_model<'a>(
    models: &'a [list::ModelEntry],
    tier: &str,
) -> Result<&'a list::ModelEntry, Error> {
    let selections: Vec<String> = models.iter().map(format_model_entry).collect();

    let selection = select_model_non_interactive(&selections, tier)?;

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
