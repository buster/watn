//! The review surface's streaming, parsing, and terminal loop.

use std::io;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use watn::output::render;
use watn::provider::registry::ProviderRegistry;
use watn::provider::{Message, RequestOptions, StreamingResponse};

pub(crate) fn review_system_prompt() -> String {
    format!(
        "You are a command review engine. Respond with exactly one JSON object and nothing else.\n\
         Schema: {{\"review_version\":1,\"command\":\"...\",\"stages\":[{{\"stage_text\":\"...\",\"purpose\":\"...\"}}],\"purpose_status\":\"ready\"}}\n\
         Rules:\n\
         - command is the complete executable command for the request.\n\
         - Split the command into stages at top-level pipes, && and ; boundaries. stage_text must be the exact text of each stage.\n\
         - purpose is plain text explaining the stage; never evaluate or execute anything.\n\
         - purpose_status must be exactly one of ready, loading, or purpose-unavailable.\n\
         - Never put a line break or tab inside command; write it as a single line.\n\
         - Do not wrap the response in a markdown code fence and do not add prose before or after the object.\n\
         Operating System: {} ({}). Shell: {}.",
        std::env::consts::OS,
        std::env::consts::ARCH,
        std::env::var("SHELL").unwrap_or_else(|_| "sh".to_string()),
    )
}

/// Report the unusable-response capture after the review surface has closed.
/// A write failure only warns; the review outcome is unchanged.
fn print_capture_diagnostics(path: &Option<std::path::PathBuf>, error: &Option<String>) {
    if let Some(error) = error {
        eprintln!("warning: could not save the unusable provider response: {error}");
    }
    if let Some(path) = path {
        eprintln!(
            "provider response was not usable; raw response saved to {}",
            path.display()
        );
    }
}

/// The billed amount to show at the Model label of a surface: only when the
/// provider reported usage and a recorded price applies to the request.
pub(crate) fn surface_amount(
    response: &StreamingResponse,
    requested_model: &str,
    config: &watn::config::types::Config,
) -> Option<String> {
    let billed = watn::amount::billed_amount(
        response.final_usage.as_ref(),
        watn::amount::recorded_price(&config.pricing, &response.model, requested_model),
    )?;
    billed.usage_reported().then(|| billed.cents_text())
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn run_review_path(
    registry: &ProviderRegistry,
    provider_name: &str,
    endpoint: &str,
    catalog_endpoint: Option<&str>,
    api_key: Option<&str>,
    interrupt: &Arc<AtomicBool>,
    response: &StreamingResponse,
    buffer: &watn::review::ReviewBuffer,
    intent: &str,
    tier: &str,
    provider: &str,
    model: &str,
    execute: bool,
    verbose: bool,
    color: bool,
    config: &watn::config::types::Config,
) -> ! {
    let (candidate, capture_path, capture_error) = parse_review_candidate(response, buffer);
    let context = watn::review::ReviewContext {
        intent: intent.to_string(),
        tier: tier.to_string(),
        provider: provider.to_string(),
        model: model.to_string(),
        amount: surface_amount(response, model, config),
    };
    let panel = open_review_panel(context, candidate, color);
    let review_messages = vec![
        Message {
            role: "system".to_string(),
            content: review_system_prompt(),
        },
        Message {
            role: "user".to_string(),
            content: intent.to_string(),
        },
    ];

    let result = review_event_loop(ReviewLoop {
        panel,
        registry,
        provider_name,
        endpoint,
        catalog_endpoint,
        api_key,
        interrupt,
        review_messages: &review_messages,
        config,
        color,
        catalog_receiver: None,
        catalog_request: 0,
        capture_path,
        capture_error,
        accepted_model: model.to_string(),
        accepted_response: response.clone(),
    });

    finish_accepted_review(
        result.accepted,
        &result.accepted_response,
        &result.accepted_model,
        &result.capture_path,
        &result.capture_error,
        verbose,
        execute,
        config,
    );
}

struct ReviewLoop<'a> {
    panel: watn::review::InlineReviewPanel<std::fs::File>,
    registry: &'a ProviderRegistry,
    provider_name: &'a str,
    endpoint: &'a str,
    catalog_endpoint: Option<&'a str>,
    api_key: Option<&'a str>,
    interrupt: &'a Arc<AtomicBool>,
    review_messages: &'a [Message],
    config: &'a watn::config::types::Config,
    color: bool,
    catalog_receiver: Option<std::sync::mpsc::Receiver<(u64, Vec<String>)>>,
    catalog_request: u64,
    capture_path: Option<std::path::PathBuf>,
    capture_error: Option<String>,
    accepted_model: String,
    accepted_response: StreamingResponse,
}

struct ReviewLoopResult {
    accepted: watn::review::ReviewCandidate,
    accepted_response: StreamingResponse,
    accepted_model: String,
    capture_path: Option<std::path::PathBuf>,
    capture_error: Option<String>,
}

fn review_event_loop(mut session: ReviewLoop<'_>) -> ReviewLoopResult {
    let accepted = loop {
        poll_catalog(&mut session);
        let Some(key) = wait_for_review_key() else {
            continue;
        };
        if let Some(candidate) = handle_review_key(&mut session, key) {
            break candidate;
        }
    };

    if let Err(error) = session.panel.finish() {
        eprintln!("review unavailable: {error}");
        std::process::exit(1);
    }

    ReviewLoopResult {
        accepted,
        accepted_response: session.accepted_response,
        accepted_model: session.accepted_model,
        capture_path: session.capture_path,
        capture_error: session.capture_error,
    }
}

fn poll_catalog(session: &mut ReviewLoop<'_>) {
    let received = session
        .catalog_receiver
        .as_ref()
        .map(std::sync::mpsc::Receiver::try_recv);
    match received {
        Some(Ok((request_id, models))) => {
            session.panel.state.set_catalog(request_id, models);
            session.catalog_receiver = None;
            if let Err(error) = session.panel.render() {
                eprintln!("review unavailable: {error}");
                std::process::exit(1);
            }
        }
        Some(Err(std::sync::mpsc::TryRecvError::Disconnected)) => {
            session.catalog_receiver = None;
        }
        Some(Err(std::sync::mpsc::TryRecvError::Empty)) | None => {}
    }
}

fn wait_for_review_key() -> Option<crossterm::event::KeyEvent> {
    let ready = match crossterm::event::poll(std::time::Duration::from_millis(50)) {
        Ok(ready) => ready,
        Err(error) => {
            eprintln!("review unavailable: {error}");
            std::process::exit(1);
        }
    };
    if !ready {
        return None;
    }
    match crossterm::event::read() {
        Ok(crossterm::event::Event::Key(key)) => Some(key),
        Ok(_) => None,
        Err(error) => {
            eprintln!("review unavailable: {error}");
            std::process::exit(1);
        }
    }
}

fn handle_review_key(
    session: &mut ReviewLoop<'_>,
    key: crossterm::event::KeyEvent,
) -> Option<watn::review::ReviewCandidate> {
    match session.panel.handle_key(key) {
        Ok(watn::review::PanelOutcome::Accepted(candidate)) => return Some(candidate),
        Ok(watn::review::PanelOutcome::Cancelled) => {
            let _ = session.panel.finish();
            print_capture_diagnostics(&session.capture_path, &session.capture_error);
            std::process::exit(0);
        }
        Ok(watn::review::PanelOutcome::DisableReviewPermanently) => {
            let command = session.panel.state.candidate().command.clone();
            match watn::config::persist_review_panel(false) {
                Ok(()) => {
                    let _ = session.panel.finish();
                    eprintln!("{}", watn::review::disable_hint(session.color));
                    print_capture_diagnostics(&session.capture_path, &session.capture_error);
                    println!("{command}");
                    std::process::exit(0);
                }
                Err(error) => {
                    eprintln!("review unavailable: {error}");
                    let _ = session.panel.render();
                }
            };
        }
        Ok(watn::review::PanelOutcome::RejectRequested) => {
            session.catalog_receiver = Some(start_catalog_load(
                &mut session.panel,
                session.config,
                session.endpoint,
                session.catalog_endpoint,
                session.api_key,
                &mut session.catalog_request,
            ));
        }
        Ok(watn::review::PanelOutcome::RegenerateWith { tier, model }) => {
            regenerate_review(
                &mut session.panel,
                session.registry,
                session.provider_name,
                session.review_messages,
                session.config,
                &tier,
                &model,
                session.interrupt,
                &mut session.capture_path,
                &mut session.capture_error,
                &mut session.accepted_model,
                &mut session.accepted_response,
            );
        }
        Ok(_) => {}
        Err(error) => {
            eprintln!("review unavailable: {error}");
            std::process::exit(1);
        }
    }
    None
}

fn parse_review_candidate(
    response: &StreamingResponse,
    buffer: &watn::review::ReviewBuffer,
) -> (
    watn::review::ReviewCandidate,
    Option<std::path::PathBuf>,
    Option<String>,
) {
    let raw = buffer.candidate().unwrap_or_default();
    let mut capture_path: Option<std::path::PathBuf> = None;
    let mut capture_error: Option<String> = None;
    let candidate = match watn::review::candidate_from_provider_response_with_finish(
        raw,
        response.finish_reason.as_deref(),
    ) {
        Some(candidate) => {
            if candidate.purpose_status == watn::review::PurposeStatus::Unavailable {
                match watn::review::capture_unusable_response(raw) {
                    Ok(path) => capture_path = Some(path),
                    Err(error) => capture_error = Some(error.to_string()),
                }
            }
            candidate
        }
        None => {
            eprintln!("review unavailable: no complete command candidate");
            match watn::review::capture_unusable_response(raw) {
                Ok(path) => eprintln!("raw provider response saved to {}", path.display()),
                Err(error) => {
                    eprintln!("warning: could not save the unusable provider response: {error}")
                }
            }
            std::process::exit(1);
        }
    };
    (candidate, capture_path, capture_error)
}

fn open_review_panel(
    context: watn::review::ReviewContext,
    candidate: watn::review::ReviewCandidate,
    color: bool,
) -> watn::review::InlineReviewPanel<std::fs::File> {
    let size = crossterm::terminal::size().unwrap_or((80, 24));
    let layout = watn::review::InlineLayout::for_dimensions(size.0, size.1);
    let terminal = match std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open("/dev/tty")
        .map_err(|error| error.to_string())
        .and_then(|tty| {
            watn::review::ControllingTerminal::open(tty, layout).map_err(|error| error.to_string())
        }) {
        Ok(terminal) => terminal,
        Err(error) => {
            eprintln!("review unavailable: {error}");
            std::process::exit(1);
        }
    };
    let mut panel = watn::review::InlineReviewPanel::with_color(
        terminal,
        watn::review::ReviewPanelState::new(context, candidate),
        color,
    );
    if let Err(error) = panel.render() {
        eprintln!("review unavailable: {error}");
        std::process::exit(1);
    }
    panel
}

fn start_catalog_load(
    panel: &mut watn::review::InlineReviewPanel<std::fs::File>,
    config: &watn::config::types::Config,
    endpoint: &str,
    catalog_endpoint: Option<&str>,
    api_key: Option<&str>,
    catalog_request: &mut u64,
) -> std::sync::mpsc::Receiver<(u64, Vec<String>)> {
    let tiers = watn::review::session::chooser_tiers(config);
    panel.state.open_model_chooser(tiers, Vec::new());
    *catalog_request += 1;
    let request_id = *catalog_request;
    panel.state.begin_catalog_load(request_id);
    if let Err(error) = panel.render() {
        eprintln!("review unavailable: {error}");
        std::process::exit(1);
    }
    let (sender, receiver) = std::sync::mpsc::channel();
    let endpoint = endpoint.to_string();
    let catalog_endpoint = catalog_endpoint.map(str::to_string);
    let api_key = api_key.map(str::to_string);
    std::thread::spawn(move || {
        let models = watn::review::session::fetch_catalog(
            &endpoint,
            catalog_endpoint.as_deref(),
            api_key.as_deref(),
        );
        let _ = sender.send((request_id, models));
    });
    receiver
}

#[allow(clippy::too_many_arguments)]
fn regenerate_review(
    panel: &mut watn::review::InlineReviewPanel<std::fs::File>,
    registry: &ProviderRegistry,
    provider_name: &str,
    review_messages: &[Message],
    config: &watn::config::types::Config,
    tier: &str,
    model: &str,
    interrupt: &Arc<AtomicBool>,
    capture_path: &mut Option<std::path::PathBuf>,
    capture_error: &mut Option<String>,
    accepted_model: &mut String,
    accepted_response: &mut StreamingResponse,
) {
    let provider = match registry.get(provider_name) {
        Ok(provider) => provider,
        Err(error) => {
            panel.state.apply_regeneration_failure(error.to_string());
            let _ = panel.render();
            return;
        }
    };
    let options = RequestOptions {
        model: model.to_string(),
        reasoning_effort: config.tiers.reasoning.effort(Some(tier)),
    };
    panel
        .state
        .begin_operation(watn::review::ReviewOperation::Regeneration);
    let spinner = Some(watn::output::spinner::Spinner::start(model));
    match watn::review::session::generate_candidate(
        provider,
        review_messages,
        &options,
        interrupt,
        spinner,
    ) {
        Ok(generation) => match watn::review::session::parse_generated_candidate(&generation) {
            Some(candidate) => {
                if candidate.purpose_status == watn::review::PurposeStatus::Unavailable {
                    let mut regeneration_buffer = generation.buffer.clone();
                    regeneration_buffer.complete();
                    if let Some(regeneration_raw) = regeneration_buffer.candidate() {
                        match watn::review::capture_unusable_response(regeneration_raw) {
                            Ok(path) => *capture_path = Some(path),
                            Err(error) => *capture_error = Some(error.to_string()),
                        }
                    }
                }
                panel.state.apply_regeneration(
                    tier.to_string(),
                    model.to_string(),
                    candidate,
                    surface_amount(&generation.response, model, config),
                );
                *accepted_model = model.to_string();
                *accepted_response = generation.response;
            }
            None => panel
                .state
                .apply_regeneration_failure("no complete command candidate"),
        },
        Err(error) => panel.state.apply_regeneration_failure(error.to_string()),
    }
    panel.state.interrupt_operation();
    if let Err(error) = panel.render() {
        eprintln!("review unavailable: {error}");
        std::process::exit(1);
    }
}

#[allow(clippy::too_many_arguments)]
fn finish_accepted_review(
    accepted: watn::review::ReviewCandidate,
    accepted_response: &StreamingResponse,
    accepted_model: &str,
    capture_path: &Option<std::path::PathBuf>,
    capture_error: &Option<String>,
    verbose: bool,
    execute: bool,
    config: &watn::config::types::Config,
) -> ! {
    use std::io::Write as _;
    print_capture_diagnostics(capture_path, capture_error);
    if verbose {
        if let Some(reasoning) = &accepted_response.reasoning_content {
            if !reasoning.trim().is_empty() {
                let _ = render::print_reasoning(reasoning);
            }
        }
        let _ = render::print_raw_response(&accepted_response.full_content);
    }

    let cost = watn::amount::billed_amount(
        accepted_response.final_usage.as_ref(),
        watn::amount::recorded_price(&config.pricing, &accepted_response.model, accepted_model),
    )
    .map(|amount| amount.usd());
    let elapsed = accepted_response.elapsed_secs;
    let tok_s = if elapsed > 0.0 {
        accepted_response
            .final_usage
            .as_ref()
            .map_or(0.0, |usage| usage.completion_tokens as f64)
            / elapsed
    } else {
        0.0
    };
    let _ = render::print_metadata(&accepted_response.model, tok_s, cost, elapsed);

    // Eligible `-x`: final acceptance is the sole execution authorization.
    // The accepted candidate is not printed to the command-output channel
    // on this path; the execution output is the observable result.
    if execute {
        watn::exec::execute(&accepted.command);
    }

    let mut stdout = io::stdout();
    if writeln!(stdout, "{}", accepted.command).is_err() || stdout.flush().is_err() {
        std::process::exit(1);
    }

    std::process::exit(0);
}
