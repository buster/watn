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

    if let (Some(small), Some(normal), Some(thinking)) = (&set_small, &set_normal, &set_thinking) {
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

    let provider_name = config.defaults.provider.as_deref().unwrap_or("openrouter");

    let provider_config = match resolve_provider(&config, provider_name) {
        Ok(p) => p,
        Err(_) => {
            eprintln!("No provider is configured. Run `watn provider` in a terminal.");
            return ModelSetupResult::Saved;
        }
    };

    let (endpoint, api_key) = if let Some(catalog) = &config.litellm {
        let key = match catalog.api_key.as_deref() {
            Some(source) => match crate::config::expand_api_key(source) {
                Ok(key) => Some(key),
                Err(error) => return ModelSetupResult::Failed(error),
            },
            None => None,
        };
        (catalog.endpoint.clone(), key)
    } else {
        let key = match crate::config::get_provider_api_key(provider_name, &provider_config) {
            Ok(key) => Some(key),
            Err(error) => return ModelSetupResult::Failed(error),
        };
        (provider_config.endpoint.clone(), key)
    };

    if std::io::stdin().is_terminal() {
        return match crate::setup::run_with_config(&config, crate::setup::SetupEntryPoint::Models) {
            Ok(crate::setup::SetupWizardOutcome::Saved(result)) => {
                let mut updated = config.clone();
                match crate::setup::apply_result(&mut updated, &result) {
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
        };
    }

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
            pricing.input, 1, pricing.output, 1
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
