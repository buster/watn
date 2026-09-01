use std::io::{self, BufRead, Write};

use crate::config::{self, types::Config};
use crate::error::Error;
use crate::provider::setup::{
    build_provider_draft, normalize_endpoint, suggested_api_key_env, OPENROUTER_ENDPOINT,
};
use crate::shell_completion;
use crate::shell_shortcut::{self, shells_available_on_path, Shell, ShellEnvironment};

const OPENROUTER_SUGGESTED_SMALL_MODEL: &str = "google/gemma-4-flash";

fn prompt(question: &str, suggestion: Option<&str>) -> String {
    match suggestion {
        Some(suggestion) => print!("{question} [{suggestion}]: "),
        None => print!("{question}: "),
    }
    io::stdout().flush().expect("flush prompt");
    let mut line = String::new();
    io::stdin()
        .lock()
        .read_line(&mut line)
        .expect("read answer");
    line.trim().to_string()
}

fn resolve_answer(answer: String, suggestion: Option<&str>) -> Option<String> {
    if answer.is_empty() {
        return suggestion.map(str::to_string);
    }
    Some(answer)
}

fn ask_required(question: &str, suggestion: Option<&str>) -> String {
    loop {
        let answer = prompt(question, suggestion);
        match resolve_answer(answer, suggestion) {
            Some(value) => return value,
            None => println!("error: a value is required"),
        }
    }
}

fn ask_endpoint() -> String {
    loop {
        let answer = prompt("Completion endpoint", Some(OPENROUTER_ENDPOINT));
        let endpoint = resolve_answer(answer, Some(OPENROUTER_ENDPOINT))
            .unwrap_or_else(|| OPENROUTER_ENDPOINT.to_string());
        match normalize_endpoint(&endpoint) {
            Ok(normalized) => return normalized,
            Err(error) => println!("error: {error}"),
        }
    }
}

fn ask_credential(endpoint: &str) -> String {
    let name = suggested_api_key_env(endpoint);
    let suggestion = std::env::var(name)
        .ok()
        .filter(|value| !value.is_empty())
        .map(|_| format!("${{{name}}}"));
    ask_required("API key", suggestion.as_deref())
}

fn ask_model(question: &str, suggestion: Option<&str>) -> String {
    ask_required(question, suggestion)
}

fn render_shell_list(selected: &[bool; 3]) {
    println!("Shell integrations (type shell names to toggle, Enter to confirm):");
    for (index, shell) in Shell::ALL.into_iter().enumerate() {
        let marker = if selected[index] { "[x]" } else { "[ ]" };
        println!("  {marker} {}", shell.name());
    }
}

fn ask_shells(preselected: [bool; 3]) -> Vec<Shell> {
    let mut selected = preselected;
    render_shell_list(&selected);
    loop {
        let answer = prompt("Shell integrations", None);
        if answer.is_empty() {
            return selected_shells(&selected);
        }
        let mut unknown = false;
        for token in answer.split_whitespace() {
            match Shell::ALL
                .into_iter()
                .find(|shell| shell.lowercase_name() == token.to_ascii_lowercase())
            {
                Some(shell) => {
                    let index = Shell::ALL
                        .into_iter()
                        .position(|candidate| candidate == shell)
                        .expect("shell index");
                    selected[index] = !selected[index];
                }
                None => {
                    println!("error: unknown shell '{token}'");
                    unknown = true;
                }
            }
        }
        if unknown {
            continue;
        }
        render_shell_list(&selected);
    }
}

fn selected_shells(selected: &[bool; 3]) -> Vec<Shell> {
    Shell::ALL
        .into_iter()
        .enumerate()
        .filter_map(|(index, shell)| selected[index].then_some(shell))
        .collect()
}

fn save_configuration(endpoint: &str, credential: &str, models: [&str; 3]) -> Result<(), Error> {
    let draft = build_provider_draft(endpoint, credential)?;
    let mut config: Config = config::load_config()?;
    config::update_provider_draft(&mut config, &draft);
    config.tiers.small = Some(models[0].to_string());
    config.tiers.normal = Some(models[1].to_string());
    config.tiers.thinking = Some(models[2].to_string());
    config::save_config(&config)
}

fn install_shell_integrations(shells: &[Shell]) -> Result<(), Error> {
    if shells.is_empty() {
        return Ok(());
    }
    let environment = ShellEnvironment::from_process();
    let completion = shell_completion::install_with_environment(shells, &environment);
    let shortcut = shell_shortcut::install_with_environment(shells, &environment);
    let failure = completion
        .aggregate_error()
        .map(Err::<(), Error>)
        .or_else(|| shortcut.aggregate_error().map(Err));
    match failure {
        Some(Err(error)) => {
            println!(
                "Configuration saved to {}. Shell integration failed: {}",
                config::xdg_config_path().display(),
                error
            );
            Err(error)
        }
        _ => Ok(()),
    }
}

/// Run the plain-line quick setup. Asks for the endpoint, credential, three
/// model strengths, and shell integrations; saves the configuration only at
/// the final confirm and then installs the chosen shell integrations.
/// Interrupts (Ctrl-C) terminate the process before anything is written.
pub fn run() -> Result<(), Error> {
    println!("No configuration file found — starting quick setup.");

    let endpoint = ask_endpoint();
    let credential = ask_credential(&endpoint);
    let small_suggestion =
        (endpoint == OPENROUTER_ENDPOINT).then_some(OPENROUTER_SUGGESTED_SMALL_MODEL);
    let small = ask_model("Small model", small_suggestion);
    let normal = ask_model("Normal model", Some(&small));
    let thinking = ask_model("Thinking model", Some(&small));
    let shells = ask_shells(shells_available_on_path());

    save_configuration(&endpoint, &credential, [&small, &normal, &thinking])?;
    let install_result = install_shell_integrations(&shells);

    println!(
        "Configuration written to {}.",
        config::xdg_config_path().display()
    );
    println!("Run `watn setup` to change the configuration later.");

    install_result
}
