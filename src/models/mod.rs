pub mod list;

use dialoguer::Select;

use crate::config::{resolve_provider, save_config};
use crate::config::types::ModelTiers;
use list::fetch_models;
use std::io::IsTerminal;

pub fn run_models(
    set_small: Option<String>,
    set_normal: Option<String>,
    set_thinking: Option<String>,
) {
    let config = crate::config::load_config().unwrap_or_default();

    if let (Some(small), Some(normal), Some(thinking)) =
        (&set_small, &set_normal, &set_thinking)
    {
        let mut updated = config.clone();
        updated.tiers.small = Some(small.clone());
        updated.tiers.normal = Some(normal.clone());
        updated.tiers.thinking = Some(thinking.clone());
        if let Err(e) = save_config(&updated) {
            eprintln!("error: failed to save config: {}", e);
            std::process::exit(1);
        }
        println!(
            "Tiers configured: small={}, normal={}, thinking={}",
            small, normal, thinking
        );
        return;
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
            return;
        }
    };

    let endpoint = provider_config.endpoint.clone();
    let api_key = match crate::config::get_provider_api_key(provider_name, &provider_config) {
        Ok(k) => Some(k),
        Err(_) => None,
    };

    let models = match fetch_models(&endpoint, api_key.as_deref()) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("error: failed to fetch models: {}", e);
            std::process::exit(1);
        }
    };

    if models.is_empty() {
        eprintln!("error: no models returned from endpoint");
        std::process::exit(1);
    }

    let small = select_model(&models, "small");
    let normal = select_model(&models, "normal");
    let thinking = select_model(&models, "thinking");

    let mut updated = config.clone();
    updated.tiers = ModelTiers {
        small: Some(small.id.clone()),
        normal: Some(normal.id.clone()),
        thinking: Some(thinking.id.clone()),
    };

    if let Err(e) = save_config(&updated) {
        eprintln!("error: failed to save config: {}", e);
        std::process::exit(1);
    }

    println!(
        "Tiers configured: small={}, normal={}, thinking={}",
        small.id, normal.id, thinking.id
    );
}

fn format_model_entry(entry: &list::ModelEntry, _index: usize) -> String {
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

fn select_model<'a>(models: &'a [list::ModelEntry], tier: &str) -> &'a list::ModelEntry {
    let selections: Vec<String> = models
        .iter()
        .enumerate()
        .map(|(i, m)| format_model_entry(m, i))
        .collect();

    let prompt = format!("Select a model for the {} tier:", tier);

    let selection = if std::io::stdin().is_terminal() {
        Select::new()
            .with_prompt(&prompt)
            .items(&selections)
            .default(0)
            .interact()
            .unwrap_or_else(|_| {
                eprintln!("error: failed to read selection");
                std::process::exit(1);
            })
    } else {
        select_model_non_interactive(&selections, tier)
    };

    &models[selection]
}

fn select_model_non_interactive(selections: &[String], tier: &str) -> usize {
    eprintln!("{}", selections.join("\n"));
    eprintln!();
    eprint!("Enter index for {} tier: ", tier);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap_or_else(|_| {
        eprintln!("error: failed to read input");
        std::process::exit(1);
    });
    let index: usize = input.trim().parse().unwrap_or_else(|_| {
        eprintln!("error: invalid index");
        std::process::exit(1);
    });
    if index >= selections.len() {
        eprintln!("error: index out of range");
        std::process::exit(1);
    }
    index
}
