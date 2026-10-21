//! The question flow: input resolution, provider setup, streaming, and output.

use std::io::{self, IsTerminal};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use watn::output::render;
use watn::provider::registry::ProviderRegistry;
use watn::provider::{Message, RequestOptions, StreamEvent, StreamingResponse};

use crate::cli::{apply_review_switch, Cli};
use crate::explain::run_explanation_card;
use crate::review_flow::{review_system_prompt, run_review_path, surface_amount};
use crate::runtime::{
    build_registry, exit_with_error, exit_with_io, load_config_or_exit, resolve_runtime,
    wait_for_scoped_result,
};
use crate::setup_commands::ensure_ready;

enum CommandSink {
    Stream(render::StreamRenderer<io::Stdout>),
    Review(watn::review::ReviewBuffer),
}

struct QuestionContext<'a> {
    config: &'a watn::config::types::Config,
    provider_name: &'a str,
    provider_config: &'a watn::config::types::ProviderConfig,
    api_key: &'a str,
    model: &'a str,
    tier: &'a str,
    intent: &'a str,
    interrupt: &'a Arc<AtomicBool>,
    use_color: bool,
    execute: bool,
    verbose: bool,
}

fn resolve_question(cli: &Cli) -> String {
    let mut pending_stdin_question: Option<String> = None;
    if cli.question.is_empty() && (cli.review_panel || cli.no_review_panel) {
        if std::io::stdin().is_terminal() {
            apply_review_switch(cli.review_panel);
        }
        let mut buf = String::new();
        std::io::Read::read_to_string(&mut std::io::stdin(), &mut buf).unwrap_or_default();
        if buf.trim().is_empty() {
            apply_review_switch(cli.review_panel);
        }
        pending_stdin_question = Some(buf.trim().to_string());
    }

    match &cli.question {
        q if !q.is_empty() => q.join(" "),
        _ => match pending_stdin_question {
            Some(question) => question,
            None if !std::io::stdin().is_terminal() => {
                let mut buf = String::new();
                std::io::Read::read_to_string(&mut std::io::stdin(), &mut buf).unwrap_or_default();
                if buf.trim().is_empty() {
                    eprintln!("Usage: watn <question>");
                    eprintln!("   or: echo \"question\" | watn");
                    std::process::exit(1);
                }
                buf.trim().to_string()
            }
            None => {
                eprintln!("Usage: watn <question>");
                eprintln!("   or: echo \"question\" | watn");
                std::process::exit(1);
            }
        },
    }
}

fn persist_review_override(cli: &Cli) -> watn::config::types::ReviewPanelOverride {
    let review_override = match watn::config::types::ReviewPanelOverride::from_flags(
        cli.review_panel,
        cli.no_review_panel,
    ) {
        Ok(review_override) => review_override,
        Err(message) => {
            eprintln!("{message}");
            std::process::exit(2);
        }
    };
    if review_override != watn::config::types::ReviewPanelOverride::Unset {
        let enabled = matches!(
            review_override,
            watn::config::types::ReviewPanelOverride::Enabled
        );
        if let Err(error) = watn::config::persist_review_panel(enabled) {
            eprintln!("warning: could not persist the review panel setting: {error}");
        }
    }
    review_override
}

fn build_messages(question: &str, review_enabled: bool) -> Vec<Message> {
    let system_prompt = if review_enabled {
        review_system_prompt()
    } else {
        format!(
            "You are a direct answer engine. Output ONLY the requested information.\n\
             Operating System: {} ({}). Shell: {}.\n\
             \n\
             For commands: Output executable syntax only. No explanations, no comments.\n\
             For questions: Output the answer only. No context, no elaboration.\n\
             \n\
             Rules:\n\
             - If asked for a command, provide ONLY the command\n\
             - If asked a question, provide ONLY the answer\n\
             - Never include markdown formatting or code blocks\n\
             - Never add explanatory text before or after\n\
             - Assume output will be piped or executed directly\n\
             - For multi-step commands, use && or ; to chain them\n\
             - Make commands robust and handle edge cases silently",
            std::env::consts::OS,
            std::env::consts::ARCH,
            std::env::var("SHELL").unwrap_or_else(|_| "sh".to_string()),
        )
    };

    vec![
        Message {
            role: "system".to_string(),
            content: system_prompt,
        },
        Message {
            role: "user".to_string(),
            content: question.to_string(),
        },
    ]
}

fn install_interrupt_handler(interrupt: &Arc<AtomicBool>) {
    let int_flag = Arc::clone(interrupt);
    ctrlc::set_handler(move || {
        int_flag.store(true, Ordering::SeqCst);
    })
    .expect("install SIGINT handler");
}

#[allow(clippy::too_many_arguments)]
fn stream_response(
    registry: &ProviderRegistry,
    provider_name: &str,
    messages: Vec<Message>,
    options: RequestOptions,
    interrupt: &Arc<AtomicBool>,
    spinner: Option<watn::output::spinner::Spinner>,
    sink: CommandSink,
    review_enabled: bool,
) -> (
    Result<StreamingResponse, watn::error::Error>,
    Option<watn::output::spinner::Spinner>,
    CommandSink,
) {
    if review_enabled {
        let provider = registry
            .get(provider_name)
            .expect("the active provider is registered");
        let generation = watn::review::session::generate_candidate(
            provider, &messages, &options, interrupt, spinner,
        );
        match generation {
            Ok(generation) => (
                Ok(generation.response),
                None,
                CommandSink::Review(generation.buffer),
            ),
            Err(error) => (
                Err(error),
                None,
                CommandSink::Review(watn::review::ReviewBuffer::new()),
            ),
        }
    } else {
        stream_plain(
            registry,
            provider_name,
            messages,
            options,
            interrupt,
            spinner,
            sink,
        )
    }
}

fn stream_plain(
    registry: &ProviderRegistry,
    provider_name: &str,
    messages: Vec<Message>,
    options: RequestOptions,
    interrupt: &Arc<AtomicBool>,
    spinner: Option<watn::output::spinner::Spinner>,
    sink: CommandSink,
) -> (
    Result<StreamingResponse, watn::error::Error>,
    Option<watn::output::spinner::Spinner>,
    CommandSink,
) {
    let worker_provider_name = provider_name.to_string();
    let worker_messages = messages;
    let worker_options = options;
    std::thread::scope(|scope| {
        let stream_handle = scope.spawn(|| {
            let mut spinner = spinner;
            let mut sink = sink;
            let provider = registry.get(&worker_provider_name).unwrap();
            let result = {
                let mut emit_content = |event: StreamEvent| -> Result<(), watn::error::Error> {
                    match event {
                        StreamEvent::Content(content) if !content.is_empty() => {
                            if let Some(active_spinner) = spinner.take() {
                                active_spinner.finish();
                            }

                            match &mut sink {
                                CommandSink::Stream(output) => {
                                    output
                                        .write_content(&content)
                                        .map_err(watn::error::Error::IoError)?;
                                }
                                CommandSink::Review(buffer) => buffer.receive(&content),
                            }
                        }
                        StreamEvent::Content(_) => {}
                    }
                    Ok(())
                };

                provider.chat_completions_streaming(
                    &worker_messages,
                    &worker_options,
                    &mut emit_content,
                )
            };
            (result, spinner, sink)
        });
        wait_for_scoped_result(stream_handle, interrupt.as_ref())
    })
}

fn finish_stream(
    response: StreamingResponse,
    spinner: Option<watn::output::spinner::Spinner>,
    sink: CommandSink,
    registry: &ProviderRegistry,
    context: &QuestionContext<'_>,
) {
    match sink {
        CommandSink::Stream(mut output) => {
            if let Some(active_spinner) = spinner {
                active_spinner.finish();
            }

            if let Err(error) = output.complete() {
                exit_with_io(error);
            }

            if context.verbose {
                if let Some(ref reasoning) = response.reasoning_content {
                    if !reasoning.trim().is_empty() {
                        if let Err(error) = render::print_reasoning(reasoning) {
                            exit_with_io(error);
                        }
                    }
                }
            }

            let cost = watn::amount::billed_amount(
                response.final_usage.as_ref(),
                watn::amount::recorded_price(
                    &context.config.pricing,
                    &response.model,
                    context.model,
                ),
            )
            .map(|amount| amount.usd());

            let elapsed = response.elapsed_secs;
            let tok_s = if elapsed > 0.0 {
                response
                    .final_usage
                    .as_ref()
                    .map_or(0.0, |usage| usage.completion_tokens as f64)
                    / elapsed
            } else {
                0.0
            };

            let command_text = response.full_content.trim().to_string();

            if let Err(error) = render::print_metadata(&response.model, tok_s, cost, elapsed) {
                exit_with_io(error);
            }

            if context.execute && !command_text.is_empty() {
                run_execution_prompt(&command_text, &response, context);
            }
        }
        CommandSink::Review(mut buffer) => {
            buffer.complete();
            run_review_path(
                registry,
                context.provider_name,
                &context.provider_config.endpoint,
                context.provider_config.catalog_endpoint.as_deref(),
                Some(context.api_key),
                context.interrupt,
                &response,
                &buffer,
                context.intent,
                context.tier,
                context.provider_name,
                context.model,
                context.execute,
                context.verbose,
                context.use_color,
                context.config,
            );
        }
    }
}

fn run_execution_prompt(
    command_text: &str,
    response: &StreamingResponse,
    context: &QuestionContext<'_>,
) {
    let explain_allowed = watn::review::controlling_terminal_is_usable();
    loop {
        match watn::exec::prompt_for_execution(command_text, explain_allowed) {
            watn::exec::PromptResult::Execute => watn::exec::execute(command_text),
            watn::exec::PromptResult::Explain => {
                let candidate = watn::review::ReviewCandidate::from_command(command_text);
                let review_context = watn::review::ReviewContext {
                    intent: "explain command".to_string(),
                    tier: context.tier.to_string(),
                    provider: context.provider_name.to_string(),
                    model: context.model.to_string(),
                    amount: surface_amount(response, context.model, context.config),
                };
                if let Err(error) = run_explanation_card(candidate, review_context) {
                    eprintln!("explanation unavailable: {error}");
                }
            }
            watn::exec::PromptResult::Cancelled => break,
            watn::exec::PromptResult::Interrupted => std::process::exit(130),
        }
    }
}

fn handle_stream_error(
    error: watn::error::Error,
    spinner: Option<watn::output::spinner::Spinner>,
    sink: &mut CommandSink,
) -> ! {
    if let Some(active_spinner) = spinner {
        active_spinner.finish();
    }
    if let CommandSink::Stream(output) = sink {
        if output.has_content() {
            let _ = output.finish_partial();
        }
    }
    if matches!(error, watn::error::Error::Interrupted) {
        std::process::exit(130);
    }
    exit_with_error(&error);
}

pub(crate) fn run_question(cli: &Cli) {
    let question = resolve_question(cli);

    let mut config = load_config_or_exit();
    let provider_name = cli.provider.clone().unwrap_or_else(|| {
        config
            .defaults
            .provider
            .clone()
            .unwrap_or_else(|| "openrouter".to_string())
    });
    let explicit_selection =
        cli.provider.is_some() || cli.model.is_some() || std::env::var("WATN_PROVIDER").is_ok();
    if !ensure_ready(&mut config, &provider_name, explicit_selection) {
        return;
    }

    let tier = cli.tier();
    let runtime = resolve_runtime(&config, &provider_name, cli.tier(), cli.model.as_deref());

    let interrupt = Arc::new(AtomicBool::new(false));

    let review_override = persist_review_override(cli);
    let review_eligible = watn::review::controlling_terminal_is_usable();
    let review_enabled =
        watn::review::resolve_review_enabled(&config, review_override, review_eligible);

    let use_color = watn::review::color_terminal_supports_card();

    let mut registry = ProviderRegistry::new();
    build_registry(
        &mut registry,
        &provider_name,
        &runtime.provider_config.endpoint,
        &runtime.api_key,
        Arc::clone(&interrupt),
    );

    let intent = question.clone();
    let messages = build_messages(&question, review_enabled);

    let reasoning_effort = config.tiers.reasoning.effort(tier);
    let options = RequestOptions {
        model: runtime.model.clone(),
        reasoning_effort,
    };

    install_interrupt_handler(&interrupt);

    let spinner = Some(watn::output::spinner::Spinner::start(&runtime.model));
    let sink = if review_enabled {
        CommandSink::Review(watn::review::ReviewBuffer::new())
    } else {
        CommandSink::Stream(render::StreamRenderer::new(io::stdout()))
    };

    let (stream_result, spinner, mut sink) = stream_response(
        &registry,
        &provider_name,
        messages,
        options,
        &interrupt,
        spinner,
        sink,
        review_enabled,
    );

    let context = QuestionContext {
        config: &config,
        provider_name: &provider_name,
        provider_config: &runtime.provider_config,
        api_key: &runtime.api_key,
        model: &runtime.model,
        tier: tier.unwrap_or("1"),
        intent: &intent,
        interrupt: &interrupt,
        use_color,
        execute: cli.execute,
        verbose: cli.verbose,
    };

    match stream_result {
        Ok(response) => finish_stream(response, spinner, sink, &registry, &context),
        Err(error) => handle_stream_error(error, spinner, &mut sink),
    }

    if interrupt.load(Ordering::SeqCst) {
        std::process::exit(130);
    }
}
