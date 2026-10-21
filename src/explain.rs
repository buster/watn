//! The explain command and its explanation card.

use std::io::IsTerminal;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use watn::error::exit_code;
use watn::output::render;
use watn::provider::registry::ProviderRegistry;
use watn::provider::{Message, RequestOptions, StreamingResponse};

use crate::review_flow::surface_amount;
use crate::runtime::{
    build_registry, get_api_key_or_exit, load_config_or_exit, resolve_model_or_exit,
    resolve_provider_or_exit,
};
use crate::setup_commands::ensure_ready;

pub(crate) fn run_explanation_card(
    candidate: watn::review::ReviewCandidate,
    context: watn::review::ReviewContext,
) -> Result<(), String> {
    let size = crossterm::terminal::size().unwrap_or((80, 24));
    let layout = watn::review::InlineLayout::for_dimensions(size.0, size.1);
    let terminal = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open("/dev/tty")
        .map_err(|error| error.to_string())
        .and_then(|tty| {
            watn::review::ControllingTerminal::open(tty, layout).map_err(|error| error.to_string())
        })?;
    let mut state = watn::review::ReviewPanelState::new(context, candidate);
    state.explain_only = true;
    let mut panel = watn::review::InlineReviewPanel::with_color(
        terminal,
        state,
        watn::review::color_terminal_supports_card(),
    );
    panel.render().map_err(|error| error.to_string())?;
    loop {
        let key = match crossterm::event::read() {
            Ok(crossterm::event::Event::Key(key)) => key,
            Ok(_) => continue,
            Err(error) => return Err(error.to_string()),
        };
        match panel.handle_key(key) {
            Ok(watn::review::PanelOutcome::Continue) => {}
            Ok(_) => {
                let _ = panel.finish();
                return Ok(());
            }
            Err(error) => {
                let _ = panel.finish();
                return Err(error.to_string());
            }
        }
    }
}

fn explain_system_prompt() -> String {
    format!(
        "You are a command explanation engine. The user provides one existing shell command. Explain it without changing it and without executing it.\n\
         Respond with exactly one JSON object and nothing else.\n\
         Schema: {{\"review_version\":1,\"command\":\"...\",\"stages\":[{{\"stage_text\":\"...\",\"purpose\":\"...\"}}],\"purpose_status\":\"ready\"}}\n\
         Rules:\n\
         - command must repeat the user's command exactly, character for character. If the command contains line breaks, encode them as \\n inside the JSON string.\n\
         - Split the command into stages at top-level pipes, ||, &&, ; and newline boundaries. stage_text must be the exact text of each stage as it appears in the command.\n\
         - purpose is plain text explaining why the stage is present and what it contributes; never evaluate or execute anything.\n\
         - purpose_status must be exactly ready and every stage must have a non-empty purpose.\n\
         - Do not wrap the response in a markdown code fence and do not add prose before or after the object.\n\
         Operating System: {} ({}). Shell: {}.",
        std::env::consts::OS,
        std::env::consts::ARCH,
        std::env::var("SHELL").unwrap_or_else(|_| "sh".to_string()),
    )
}

/// Resolve the explained command from the positional argument or standard
/// input. The bytes are never re-split, joined, evaluated, or trimmed beyond
/// one trailing standard-input newline.
fn resolve_explain_command(command: Option<String>) -> Result<String, String> {
    match command {
        Some(value) if value != "-" => {
            if value.is_empty() {
                Err("Usage: watn explain <command>".to_string())
            } else {
                Ok(value)
            }
        }
        Some(_) => {
            let command = read_stdin_command()?;
            if command.is_empty() {
                Err("Usage: watn explain <command>".to_string())
            } else {
                Ok(command)
            }
        }
        None => {
            if std::io::stdin().is_terminal() {
                return Err("Usage: watn explain <command>".to_string());
            }
            let command = read_stdin_command()?;
            if command.is_empty() {
                Err("Usage: watn explain <command>".to_string())
            } else {
                Ok(command)
            }
        }
    }
}

fn read_stdin_command() -> Result<String, String> {
    let mut buffer = String::new();
    std::io::Read::read_to_string(&mut std::io::stdin(), &mut buffer)
        .map_err(|error| format!("cannot read the command from standard input: {error}"))?;
    if let Some(stripped) = buffer.strip_suffix("\r\n") {
        buffer = stripped.to_string();
    } else if let Some(stripped) = buffer.strip_suffix('\n') {
        buffer = stripped.to_string();
    }
    Ok(buffer)
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn run_explain_command(
    command: Option<String>,
    execute: bool,
    provider: Option<&str>,
    explicit_model: Option<&str>,
    tier: Option<&str>,
    verbose: bool,
) -> ! {
    if execute {
        eprintln!("explain never executes a command; remove -x");
        std::process::exit(2);
    }

    let command = match resolve_explain_command(command) {
        Ok(command) => command,
        Err(message) => {
            eprintln!("{message}");
            std::process::exit(2);
        }
    };

    if !watn::review::explanation_terminal_is_usable() {
        eprintln!("explain requires a terminal for the explanation card");
        std::process::exit(1);
    }

    let mut config = load_config_or_exit();
    let provider_name = provider
        .unwrap_or(config.defaults.provider.as_deref().unwrap_or("openrouter"))
        .to_string();
    let tier = tier.unwrap_or("1");
    let explicit_selection =
        provider.is_some() || explicit_model.is_some() || std::env::var("WATN_PROVIDER").is_ok();
    if !ensure_ready(&mut config, &provider_name, explicit_selection) {
        print_explain_rerun_hint();
        std::process::exit(0);
    }

    let model = resolve_model_or_exit(&config, Some(tier), explicit_model);
    let provider_config = resolve_provider_or_exit(&config, &provider_name);
    let api_key = get_api_key_or_exit(&provider_name, &provider_config);

    let mut context = watn::review::ReviewContext {
        intent: "explain command".to_string(),
        tier: tier.to_string(),
        provider: provider_name.clone(),
        model: model.clone(),
        amount: None,
    };

    let interrupt = Arc::new(AtomicBool::new(false));
    let int_flag = Arc::clone(&interrupt);
    let _ = ctrlc::set_handler(move || {
        int_flag.store(true, Ordering::SeqCst);
    });
    let mut registry = ProviderRegistry::new();
    build_registry(
        &mut registry,
        &provider_name,
        &provider_config.endpoint,
        &api_key,
        Arc::clone(&interrupt),
    );

    let ExplanationResult {
        candidate,
        failure_status,
        raw_response,
    } = request_explanation(
        &registry,
        &provider_name,
        &command,
        &config,
        &model,
        tier,
        &interrupt,
    );

    if interrupt.load(Ordering::SeqCst) {
        std::process::exit(130);
    }

    context.amount = raw_response
        .as_ref()
        .and_then(|response| surface_amount(response, &model, &config));

    if let Err(error) = run_explanation_card(candidate, context) {
        eprintln!("explain unavailable: {error}");
        std::process::exit(1);
    }
    if verbose {
        if let Some(response) = &raw_response {
            let _ = render::print_raw_response(&response.full_content);
        }
    }
    std::process::exit(failure_status.unwrap_or(0));
}

struct ExplanationResult {
    candidate: watn::review::ReviewCandidate,
    failure_status: Option<i32>,
    raw_response: Option<StreamingResponse>,
}

fn request_explanation(
    registry: &ProviderRegistry,
    provider_name: &str,
    command: &str,
    config: &watn::config::types::Config,
    model: &str,
    tier: &str,
    interrupt: &Arc<AtomicBool>,
) -> ExplanationResult {
    let messages = vec![
        Message {
            role: "system".to_string(),
            content: explain_system_prompt(),
        },
        Message {
            role: "user".to_string(),
            content: command.to_string(),
        },
    ];
    let options = RequestOptions {
        model: model.to_string(),
        reasoning_effort: config.tiers.reasoning.effort(Some(tier)),
    };
    let spinner = Some(watn::output::spinner::Spinner::start(model));
    let provider = registry
        .get(provider_name)
        .expect("the active provider is registered");
    let outcome = watn::review::session::explain_command_candidate(
        provider, command, &messages, &options, interrupt, spinner,
    );

    match outcome {
        Ok((watn::review::ExplanationOutcome::Ready(candidate), response)) => ExplanationResult {
            candidate,
            failure_status: None,
            raw_response: Some(response),
        },
        Ok((watn::review::ExplanationOutcome::Unusable { candidate, reason }, response)) => {
            let mut message = format!(
                "explain response was not usable: {}",
                reason.explain_reason()
            );
            match watn::review::capture_unusable_response(&response.full_content) {
                Ok(path) => {
                    message.push_str(&format!("; raw response saved to {}", path.display()))
                }
                Err(error) => message.push_str(&format!(
                    "; warning: could not save the unusable provider response: {error}"
                )),
            }
            eprintln!("{message}");
            ExplanationResult {
                candidate,
                failure_status: None,
                raw_response: Some(response),
            }
        }
        Err(error) => {
            if matches!(error, watn::error::Error::Interrupted) {
                std::process::exit(130);
            }
            eprintln!("explain request failed: {error}");
            ExplanationResult {
                candidate: watn::review::ReviewCandidate::from_command(command),
                failure_status: Some(exit_code(&error)),
                raw_response: None,
            }
        }
    }
}

/// After a successful delegated setup, the original command is not resumed;
/// the developer reruns `watn explain` with the configuration in place.
fn print_explain_rerun_hint() {
    eprintln!("setup complete; rerun `watn explain` with the command");
}
