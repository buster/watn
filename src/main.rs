use clap::{CommandFactory, Parser};
use clap_complete::generate;
use clap_complete::shells::{Bash, Elvish, Fish, PowerShell, Zsh};

use std::io::{self, IsTerminal};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use watn::config::{self, load_config, resolve_model, resolve_provider};
use watn::error::exit_code;
use watn::output::render;
use watn::provider::openai_compat::OpenAICompatibleProvider;
use watn::provider::registry::ProviderRegistry;
use watn::provider::{Message, RequestOptions, StreamEvent, StreamingResponse};
use watn::setup::{SetupEntryPoint, SetupWizardOutcome};

enum CommandSink {
    Stream(render::StreamRenderer<io::Stdout>),
    Review(watn::review::ReviewBuffer),
}

#[derive(clap::Parser)]
#[command(name = "watn", version = env!("CARGO_PKG_VERSION"))]
#[command(about = "Ask in plain language. Get one command.")]
struct Cli {
    #[arg(
        group = "input",
        num_args = 1..,
        value_name = "QUESTION",
        help = "Natural-language question to turn into a command"
    )]
    question: Vec<String>,

    #[arg(short = '1', long = "small", help = "Use the small/fast model tier")]
    tier_small: bool,

    #[arg(short = '2', long = "normal", help = "Use the balanced model tier")]
    tier_normal: bool,

    #[arg(
        short = '3',
        long = "thinking",
        help = "Use the thinking/reasoning model tier"
    )]
    tier_thinking: bool,

    #[arg(
        long = "model",
        conflicts_with_all = ["tier_small", "tier_normal", "tier_thinking"],
        help = "Use an explicit model instead of a tier"
    )]
    model: Option<String>,

    #[arg(
        short = 'x',
        long = "execute",
        help = "Prompt before executing the generated command"
    )]
    execute: bool,

    #[arg(
        long = "review-panel",
        conflicts_with = "no_review_panel",
        help = "Force the explanatory review surface on for this invocation"
    )]
    review_panel: bool,

    #[arg(
        long = "no-review-panel",
        help = "Disable the explanatory review surface for this invocation"
    )]
    no_review_panel: bool,

    #[arg(
        short = 'v',
        long = "verbose",
        help = "Print provider reasoning to stderr when available"
    )]
    verbose: bool,

    #[arg(long = "provider", help = "Select a configured provider")]
    provider: Option<String>,

    #[arg(
        long = "set-small",
        value_name = "MODEL",
        help = "Set the small-tier model non-interactively"
    )]
    set_small: Option<String>,

    #[arg(
        long = "set-normal",
        value_name = "MODEL",
        help = "Set the normal-tier model non-interactively"
    )]
    set_normal: Option<String>,

    #[arg(
        long = "set-thinking",
        value_name = "MODEL",
        help = "Set the thinking-tier model non-interactively"
    )]
    set_thinking: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(clap::Subcommand)]
enum Commands {
    #[command(
        about = "Configure provider, models, reasoning, and shell integrations interactively"
    )]
    Setup,
    #[command(about = "Configure model tiers and reasoning settings interactively")]
    Models,
    #[command(about = "Configure a provider endpoint and credential")]
    Provider,
    #[command(about = "Configure shell completion and Ctrl-W integrations")]
    Shell,
    #[command(
        about = "Configure provider, models, and shell integrations with a minimal question flow"
    )]
    Quicksetup {
        #[arg(
            long = "url",
            value_name = "URL",
            help = "Prefill the completion endpoint"
        )]
        url: Option<String>,
        #[arg(
            long = "key",
            value_name = "KEY",
            help = "Prefill the credential, literal or ${ENV_VAR}"
        )]
        key: Option<String>,
        #[arg(
            long = "model",
            value_name = "MODEL",
            help = "Prefill the small, normal, and thinking models"
        )]
        model: Option<String>,
        #[arg(
            long = "model-small",
            value_name = "MODEL",
            help = "Prefill the small model"
        )]
        model_small: Option<String>,
        #[arg(
            long = "model-normal",
            value_name = "MODEL",
            help = "Prefill the normal model"
        )]
        model_normal: Option<String>,
        #[arg(
            long = "model-thinking",
            value_name = "MODEL",
            help = "Prefill the thinking model"
        )]
        model_thinking: Option<String>,
    },
    #[command(
        about = "Generate a shell completion script on stdout for the caller to install or source"
    )]
    Completions {
        #[arg(
            value_name = "SHELL",
            value_parser = CompletionShell::parse,
            help = "Supported shell values: bash, elvish, fish, powershell, or zsh"
        )]
        shell: CompletionShell,
    },
    #[command(about = "Explain an existing shell command in the review card")]
    Explain {
        #[arg(
            value_name = "COMMAND",
            help = "The command to explain verbatim; use - to read it from standard input"
        )]
        command: Option<String>,

        #[arg(
            long = "review-panel",
            help = "Accepted for compatibility; explain always opens the explanation card"
        )]
        review_panel: bool,

        #[arg(
            long = "no-review-panel",
            help = "Accepted for compatibility; explain always opens the explanation card"
        )]
        no_review_panel: bool,

        #[arg(
            short = 'v',
            long = "verbose",
            help = "Print the raw provider response to stderr after the card closes"
        )]
        verbose: bool,
    },
}

#[derive(Clone, Debug)]
enum CompletionShell {
    Bash,
    Elvish,
    Fish,
    PowerShell,
    Zsh,
}

impl CompletionShell {
    fn parse(input: &str) -> Result<Self, String> {
        match input {
            "bash" => Ok(Self::Bash),
            "elvish" => Ok(Self::Elvish),
            "fish" => Ok(Self::Fish),
            "powershell" => Ok(Self::PowerShell),
            "zsh" => Ok(Self::Zsh),
            _ => Err(format!(
                "unsupported shell '{input}'; choose bash, elvish, fish, powershell, or zsh"
            )),
        }
    }
}

impl Cli {
    fn tier(&self) -> Option<&str> {
        if self.tier_small
            || (!self.question.is_empty() && !self.tier_normal && !self.tier_thinking)
        {
            Some("1")
        } else if self.tier_normal {
            Some("2")
        } else if self.tier_thinking {
            Some("3")
        } else {
            None
        }
    }
}

/// A review-panel switch without a question only persists the setting; a clean
/// machine keeps its first-run onboarding path untouched.
fn apply_review_switch(enabled: bool) -> ! {
    if !watn::config::config_file_exists() {
        eprintln!(
            "review surface {} by default; no configuration to update",
            if enabled { "enabled" } else { "disabled" }
        );
        std::process::exit(0);
    }
    match watn::config::persist_review_panel(enabled) {
        Ok(()) => {
            eprintln!(
                "review surface {}",
                if enabled { "enabled" } else { "disabled" }
            );
            std::process::exit(0);
        }
        Err(error) => {
            let code = exit_code(&error);
            eprintln!("{error}");
            std::process::exit(code);
        }
    }
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = &cli.command {
        match command {
            Commands::Setup => run_setup_command(),
            Commands::Models => {
                run_models_command(cli.set_small, cli.set_normal, cli.set_thinking);
            }
            Commands::Provider => run_provider_setup_command(),
            Commands::Shell => run_shell_setup_command(),
            Commands::Quicksetup {
                url,
                key,
                model,
                model_small,
                model_normal,
                model_thinking,
            } => run_quicksetup_command(watn::quicksetup::QuickSetupDefaults {
                url: url.clone(),
                key: key.clone(),
                model: model.clone(),
                model_small: model_small.clone(),
                model_normal: model_normal.clone(),
                model_thinking: model_thinking.clone(),
            }),
            Commands::Completions { shell } => run_completions(shell),
            Commands::Explain {
                command, verbose, ..
            } => run_explain_command(
                command.clone(),
                cli.execute,
                cli.provider.as_deref(),
                cli.model.as_deref(),
                cli.tier(),
                *verbose || cli.verbose,
            ),
        }
        return;
    }

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

    let question = match &cli.question {
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
    };

    let mut config = match load_config() {
        Ok(c) => c,
        Err(e) => {
            let code = exit_code(&e);
            eprintln!("{}", e);
            std::process::exit(code);
        }
    };

    let provider_name = cli
        .provider
        .as_deref()
        .unwrap_or(config.defaults.provider.as_deref().unwrap_or("openrouter"));

    let explicit_provider = cli.provider.is_some() || std::env::var("WATN_PROVIDER").is_ok();
    let explicit_model = cli.model.is_some();
    if !explicit_provider
        && !explicit_model
        && (!config::provider_ready(&config, provider_name) || !config::model_roles_ready(&config))
    {
        if !std::io::stdin().is_terminal() {
            watn::provider::setup::print_setup_guidance();
            std::process::exit(1);
        }
        if !config::config_file_exists() {
            if let Err(error) = watn::quicksetup::run() {
                eprintln!("{}", error);
                std::process::exit(exit_code(&error));
            }
            return;
        }
        match watn::setup::run_with_config(&config, SetupEntryPoint::Setup) {
            Ok(SetupWizardOutcome::Saved(result)) => {
                if let Err(error) = watn::setup::apply_result(&mut config, &result) {
                    eprintln!("{}", error);
                    std::process::exit(exit_code(&error));
                }
                return;
            }
            Ok(SetupWizardOutcome::Cancelled(cancellation)) => {
                exit_setup_cancellation(cancellation);
            }
            Err(e) => {
                let code = exit_code(&e);
                eprintln!("{}", e);
                std::process::exit(code);
            }
        }
    }

    let tier = cli.tier();
    let model = match resolve_model(&config, tier, cli.model.as_deref()) {
        Ok(m) => m,
        Err(e) => {
            let code = exit_code(&e);
            eprintln!("{}", e);
            std::process::exit(code);
        }
    };

    let provider_config = match resolve_provider(&config, provider_name) {
        Ok(p) => p,
        Err(e) => {
            let code = exit_code(&e);
            eprintln!("{}", e);
            std::process::exit(code);
        }
    };

    let api_key = match config::get_provider_api_key(provider_name, &provider_config) {
        Ok(k) => k,
        Err(e) => {
            let code = exit_code(&e);
            eprintln!("{}", e);
            std::process::exit(code);
        }
    };

    let interrupt = Arc::new(AtomicBool::new(false));

    let review_override = match watn::config::types::ReviewPanelOverride::from_flags(
        cli.review_panel,
        cli.no_review_panel,
    ) {
        Ok(override_value) => override_value,
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

    let review_eligible = watn::review::controlling_terminal_is_usable();
    let review_enabled =
        watn::review::resolve_review_enabled(&config, review_override, review_eligible);

    let use_color = watn::review::color_terminal_supports_card();

    let mut registry = ProviderRegistry::new();
    build_registry(
        &mut registry,
        provider_name,
        &provider_config.endpoint,
        &api_key,
        Arc::clone(&interrupt),
    );

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

    let intent = question.clone();
    let messages = vec![
        Message {
            role: "system".to_string(),
            content: system_prompt,
        },
        Message {
            role: "user".to_string(),
            content: question,
        },
    ];

    let reasoning_effort = config.tiers.reasoning.effort(tier);

    let options = RequestOptions {
        model: model.clone(),
        temperature: None,
        max_tokens: review_enabled.then_some(4096),
        reasoning_effort,
    };

    let int_flag = Arc::clone(&interrupt);
    ctrlc::set_handler(move || {
        int_flag.store(true, Ordering::SeqCst);
    })
    .expect("install SIGINT handler");

    let mut spinner = Some(watn::output::spinner::Spinner::start(&model));
    let mut sink = if review_enabled {
        CommandSink::Review(watn::review::ReviewBuffer::new())
    } else {
        CommandSink::Stream(render::StreamRenderer::new(io::stdout()))
    };

    let (stream_result, mut spinner, mut sink) = if review_enabled {
        let provider = registry
            .get(provider_name)
            .expect("the active provider is registered");
        let generation = watn::review::session::generate_candidate(
            provider,
            &messages,
            &options,
            &interrupt,
            spinner.take(),
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
        let worker_provider_name = provider_name.to_string();
        let worker_messages = messages;
        let worker_options = options;
        std::thread::scope(|scope| {
            let stream_handle = scope.spawn(|| {
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
            wait_for_scoped_result(stream_handle, &interrupt)
        })
    };

    match stream_result {
        Ok(response) => {
            if let Some(active_spinner) = spinner.take() {
                active_spinner.finish();
            }

            let mut output = match sink {
                CommandSink::Stream(output) => output,
                CommandSink::Review(mut buffer) => {
                    buffer.complete();
                    run_review_path(
                        &registry,
                        provider_name,
                        &provider_config.endpoint,
                        provider_config.catalog_endpoint.as_deref(),
                        Some(api_key.as_str()),
                        &interrupt,
                        &response,
                        &buffer,
                        &intent,
                        tier.unwrap_or("1"),
                        provider_name,
                        &model,
                        cli.execute,
                        cli.verbose,
                        use_color,
                        &config,
                    );
                }
            };

            if let Err(error) = output.complete() {
                let error = watn::error::Error::IoError(error);
                eprintln!("{}", error);
                std::process::exit(exit_code(&error));
            }

            if cli.verbose {
                if let Some(ref reasoning) = response.reasoning_content {
                    if !reasoning.trim().is_empty() {
                        if let Err(error) = render::print_reasoning(reasoning) {
                            let error = watn::error::Error::IoError(error);
                            eprintln!("{}", error);
                            std::process::exit(exit_code(&error));
                        }
                    }
                }
            }

            let cost = config.pricing.get(&response.model).map(|p| {
                let input_cost = p.input
                    * response.final_usage.as_ref().map_or(0, |u| u.prompt_tokens) as f64
                    / 1_000_000.0;
                let output_cost = p.output
                    * response
                        .final_usage
                        .as_ref()
                        .map_or(0, |u| u.completion_tokens) as f64
                    / 1_000_000.0;
                input_cost + output_cost
            });

            let elapsed = response.elapsed_secs;
            let tok_s = if elapsed > 0.0 {
                response
                    .final_usage
                    .as_ref()
                    .map_or(0.0, |u| u.completion_tokens as f64)
                    / elapsed
            } else {
                0.0
            };

            let command_text = response.full_content.trim().to_string();

            if let Err(error) = render::print_metadata(&response.model, tok_s, cost, elapsed) {
                let error = watn::error::Error::IoError(error);
                eprintln!("{}", error);
                std::process::exit(exit_code(&error));
            }

            if cli.execute && !command_text.is_empty() {
                let explain_allowed = watn::review::controlling_terminal_is_usable();
                loop {
                    match watn::exec::prompt_for_execution(&command_text, explain_allowed) {
                        watn::exec::PromptResult::Execute => watn::exec::execute(&command_text),
                        watn::exec::PromptResult::Explain => {
                            let candidate =
                                watn::review::ReviewCandidate::from_command(&command_text);
                            let context = watn::review::ReviewContext {
                                intent: "explain command".to_string(),
                                tier: tier.unwrap_or("1").to_string(),
                                provider: provider_name.to_string(),
                                model: model.clone(),
                            };
                            if let Err(error) = run_explanation_card(candidate, context) {
                                eprintln!("explanation unavailable: {error}");
                            }
                        }
                        watn::exec::PromptResult::Cancelled => break,
                        watn::exec::PromptResult::Interrupted => std::process::exit(130),
                    }
                }
            }
        }
        Err(e) => {
            if let Some(active_spinner) = spinner.take() {
                active_spinner.finish();
            }
            if let CommandSink::Stream(output) = &mut sink {
                if output.has_content() {
                    let _ = output.finish_partial();
                }
            }
            if matches!(e, watn::error::Error::Interrupted) {
                std::process::exit(130);
            }
            let code = exit_code(&e);
            eprintln!("{}", e);
            std::process::exit(code);
        }
    }

    if interrupt.load(Ordering::SeqCst) {
        std::process::exit(130);
    }
}

fn review_system_prompt() -> String {
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

fn run_explanation_card(
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
fn run_explain_command(
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

    let mut config = match load_config() {
        Ok(config) => config,
        Err(error) => {
            let code = exit_code(&error);
            eprintln!("{error}");
            std::process::exit(code);
        }
    };

    let provider_name = provider
        .unwrap_or(config.defaults.provider.as_deref().unwrap_or("openrouter"))
        .to_string();
    let tier = tier.unwrap_or("1");
    let explicit_selection =
        provider.is_some() || explicit_model.is_some() || std::env::var("WATN_PROVIDER").is_ok();

    if !explicit_selection
        && (!config::provider_ready(&config, &provider_name) || !config::model_roles_ready(&config))
    {
        if !std::io::stdin().is_terminal() {
            watn::provider::setup::print_setup_guidance();
            std::process::exit(1);
        }
        if !config::config_file_exists() {
            if let Err(error) = watn::quicksetup::run() {
                eprintln!("{error}");
                std::process::exit(exit_code(&error));
            }
            print_explain_rerun_hint();
            std::process::exit(0);
        }
        match watn::setup::run_with_config(&config, SetupEntryPoint::Setup) {
            Ok(SetupWizardOutcome::Saved(result)) => {
                if let Err(error) = watn::setup::apply_result(&mut config, &result) {
                    eprintln!("{error}");
                    std::process::exit(exit_code(&error));
                }
                print_explain_rerun_hint();
                std::process::exit(0);
            }
            Ok(SetupWizardOutcome::Cancelled(cancellation)) => {
                exit_setup_cancellation(cancellation);
            }
            Err(error) => {
                let code = exit_code(&error);
                eprintln!("{error}");
                std::process::exit(code);
            }
        }
    }

    let model = match resolve_model(&config, Some(tier), explicit_model) {
        Ok(model) => model,
        Err(error) => {
            let code = exit_code(&error);
            eprintln!("{error}");
            std::process::exit(code);
        }
    };

    let provider_config = match resolve_provider(&config, &provider_name) {
        Ok(provider_config) => provider_config,
        Err(error) => {
            let code = exit_code(&error);
            eprintln!("{error}");
            std::process::exit(code);
        }
    };

    let api_key = match config::get_provider_api_key(&provider_name, &provider_config) {
        Ok(api_key) => api_key,
        Err(error) => {
            let code = exit_code(&error);
            eprintln!("{error}");
            std::process::exit(code);
        }
    };

    let context = watn::review::ReviewContext {
        intent: "explain command".to_string(),
        tier: tier.to_string(),
        provider: provider_name.clone(),
        model: model.clone(),
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
    let messages = vec![
        Message {
            role: "system".to_string(),
            content: explain_system_prompt(),
        },
        Message {
            role: "user".to_string(),
            content: command.clone(),
        },
    ];
    let options = RequestOptions {
        model: model.clone(),
        temperature: None,
        max_tokens: Some(4096),
        reasoning_effort: config.tiers.reasoning.effort(Some(tier)),
    };
    let spinner = Some(watn::output::spinner::Spinner::start(&model));
    let provider = registry
        .get(&provider_name)
        .expect("the active provider is registered");
    let outcome = watn::review::session::explain_command_candidate(
        provider, &command, &messages, &options, &interrupt, spinner,
    );

    let (candidate, failure_status, raw_response) = match outcome {
        Ok((watn::review::ExplanationOutcome::Ready(candidate), response)) => {
            (candidate, None, Some(response))
        }
        Ok((watn::review::ExplanationOutcome::Unusable { candidate, reason }, response)) => {
            let mut message =
                format!("explain response was not usable: {}", reason.explain_reason());
            match watn::review::capture_unusable_response(&response.full_content) {
                Ok(path) => {
                    message.push_str(&format!("; raw response saved to {}", path.display()))
                }
                Err(error) => message.push_str(&format!(
                    "; warning: could not save the unusable provider response: {error}"
                )),
            }
            eprintln!("{message}");
            (candidate, None, Some(response))
        }
        Err(error) => {
            if matches!(error, watn::error::Error::Interrupted) {
                std::process::exit(130);
            }
            eprintln!("explain request failed: {error}");
            (
                watn::review::ReviewCandidate::from_command(&command),
                Some(exit_code(&error)),
                None,
            )
        }
    };

    if interrupt.load(Ordering::SeqCst) {
        std::process::exit(130);
    }

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

/// After a successful delegated setup, the original command is not resumed;
/// the developer reruns `watn explain` with the configuration in place.
fn print_explain_rerun_hint() {
    eprintln!("setup complete; rerun `watn explain` with the command");
}

/// Report the unusable-response capture after the review surface has closed.
/// A write failure only warns; the review outcome is unchanged.
fn print_capture_diagnostics(
    path: &Option<std::path::PathBuf>,
    error: &Option<String>,
) {
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

#[allow(clippy::too_many_arguments)]
fn run_review_path(
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
                Err(error) => eprintln!(
                    "warning: could not save the unusable provider response: {error}"
                ),
            }
            std::process::exit(1);
        }
    };

    let context = watn::review::ReviewContext {
        intent: intent.to_string(),
        tier: tier.to_string(),
        provider: provider.to_string(),
        model: model.to_string(),
    };
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
        watn::review::ReviewPanelState::new(context, candidate.clone()),
        color,
    );
    if let Err(error) = panel.render() {
        eprintln!("review unavailable: {error}");
        std::process::exit(1);
    }

    let mut accepted_response = response.clone();
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
    let mut catalog_receiver: Option<std::sync::mpsc::Receiver<(u64, Vec<String>)>> = None;
    let mut catalog_request: u64 = 0;

    let accepted = loop {
        if let Some(receiver) = &catalog_receiver {
            match receiver.try_recv() {
                Ok((request_id, models)) => {
                    panel.state.set_catalog(request_id, models);
                    catalog_receiver = None;
                    if let Err(error) = panel.render() {
                        eprintln!("review unavailable: {error}");
                        std::process::exit(1);
                    }
                }
                Err(std::sync::mpsc::TryRecvError::Disconnected) => catalog_receiver = None,
                Err(std::sync::mpsc::TryRecvError::Empty) => {}
            }
        }
        let ready = match crossterm::event::poll(std::time::Duration::from_millis(50)) {
            Ok(ready) => ready,
            Err(error) => {
                eprintln!("review unavailable: {error}");
                std::process::exit(1);
            }
        };
        if !ready {
            continue;
        }
        let key = match crossterm::event::read() {
            Ok(crossterm::event::Event::Key(key)) => key,
            Ok(_) => continue,
            Err(error) => {
                eprintln!("review unavailable: {error}");
                std::process::exit(1);
            }
        };
        match panel.handle_key(key) {
            Ok(watn::review::PanelOutcome::Accepted(candidate)) => break candidate,
            Ok(watn::review::PanelOutcome::Cancelled) => {
                let _ = panel.finish();
                print_capture_diagnostics(&capture_path, &capture_error);
                std::process::exit(0);
            }
            Ok(watn::review::PanelOutcome::DisableReviewPermanently) => {
                let command = panel.state.candidate().command.clone();
                match watn::config::persist_review_panel(false) {
                    Ok(()) => {
                        let _ = panel.finish();
                        eprintln!("{}", watn::review::disable_hint(color));
                        print_capture_diagnostics(&capture_path, &capture_error);
                        println!("{command}");
                        std::process::exit(0);
                    }
                    Err(error) => {
                        eprintln!("review unavailable: {error}");
                        let _ = panel.render();
                    }
                }
            }
            Ok(watn::review::PanelOutcome::RejectRequested) => {
                let tiers = watn::review::session::chooser_tiers(config);
                panel.state.open_model_chooser(tiers, Vec::new());
                catalog_request += 1;
                let request_id = catalog_request;
                panel.state.begin_catalog_load(request_id);
                if let Err(error) = panel.render() {
                    eprintln!("review unavailable: {error}");
                    std::process::exit(1);
                }
                let (sender, receiver) = std::sync::mpsc::channel();
                catalog_receiver = Some(receiver);
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
            }
            Ok(watn::review::PanelOutcome::RegenerateWith { tier, model }) => {
                let provider = match registry.get(provider_name) {
                    Ok(provider) => provider,
                    Err(error) => {
                        panel.state.apply_regeneration_failure(error.to_string());
                        let _ = panel.render();
                        continue;
                    }
                };
                let options = RequestOptions {
                    model: model.clone(),
                    temperature: None,
                    max_tokens: Some(4096),
                    reasoning_effort: config.tiers.reasoning.effort(Some(&tier)),
                };
                panel
                    .state
                    .begin_operation(watn::review::ReviewOperation::Regeneration);
                let spinner = Some(watn::output::spinner::Spinner::start(&model));
                match watn::review::session::generate_candidate(
                    provider,
                    &review_messages,
                    &options,
                    interrupt,
                    spinner,
                ) {
                    Ok(generation) => {
                        match watn::review::session::parse_generated_candidate(&generation) {
                            Some(candidate) => {
                                if candidate.purpose_status == watn::review::PurposeStatus::Unavailable
                                {
                                    let mut regeneration_buffer = generation.buffer.clone();
                                    regeneration_buffer.complete();
                                    if let Some(regeneration_raw) = regeneration_buffer.candidate() {
                                        match watn::review::capture_unusable_response(regeneration_raw)
                                        {
                                            Ok(path) => capture_path = Some(path),
                                            Err(error) => capture_error = Some(error.to_string()),
                                        }
                                    }
                                }
                                panel.state.apply_regeneration(
                                    tier.clone(),
                                    model.clone(),
                                    candidate,
                                );
                                accepted_response = generation.response;
                            }
                            None => panel
                                .state
                                .apply_regeneration_failure("no complete command candidate"),
                        }
                    }
                    Err(error) => panel.state.apply_regeneration_failure(error.to_string()),
                }
                panel.state.interrupt_operation();
                if let Err(error) = panel.render() {
                    eprintln!("review unavailable: {error}");
                    std::process::exit(1);
                }
            }
            Ok(_) => {}
            Err(error) => {
                eprintln!("review unavailable: {error}");
                std::process::exit(1);
            }
        }
    };
    if let Err(error) = panel.finish() {
        eprintln!("review unavailable: {error}");
        std::process::exit(1);
    }

    {
        use std::io::Write as _;
        print_capture_diagnostics(&capture_path, &capture_error);
        if verbose {
            if let Some(reasoning) = &accepted_response.reasoning_content {
                if !reasoning.trim().is_empty() {
                    let _ = render::print_reasoning(reasoning);
                }
            }
            let _ = render::print_raw_response(&accepted_response.full_content);
        }

        let cost = config.pricing.get(&accepted_response.model).map(|p| {
            let input_cost = p.input
                * accepted_response
                    .final_usage
                    .as_ref()
                    .map_or(0, |u| u.prompt_tokens) as f64
                / 1_000_000.0;
            let output_cost = p.output
                * accepted_response
                    .final_usage
                    .as_ref()
                    .map_or(0, |u| u.completion_tokens) as f64
                / 1_000_000.0;
            input_cost + output_cost
        });
        let elapsed = accepted_response.elapsed_secs;
        let tok_s = if elapsed > 0.0 {
            accepted_response
                .final_usage
                .as_ref()
                .map_or(0.0, |u| u.completion_tokens as f64)
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
    }

    std::process::exit(0);
}

fn wait_for_scoped_result<T>(
    handle: std::thread::ScopedJoinHandle<'_, T>,
    interrupt: &AtomicBool,
) -> T {
    const GRACE: std::time::Duration = std::time::Duration::from_millis(500);
    const POLL: std::time::Duration = std::time::Duration::from_millis(20);

    loop {
        if handle.is_finished() {
            return handle.join().expect("stream worker panicked");
        }
        if interrupt.load(Ordering::SeqCst) {
            let deadline = std::time::Instant::now() + GRACE;
            while !handle.is_finished() && std::time::Instant::now() < deadline {
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
            if !handle.is_finished() {
                std::process::exit(130);
            }
            return handle.join().expect("stream worker panicked");
        }
        std::thread::sleep(POLL);
    }
}

fn run_completions(shell: &CompletionShell) -> ! {
    let mut command = Cli::command().mut_subcommand("completions", |subcommand| {
        subcommand.mut_arg("shell", |argument| {
            argument.value_parser(["bash", "elvish", "fish", "powershell", "zsh"])
        })
    });
    match shell {
        CompletionShell::Bash => generate(Bash, &mut command, "watn", &mut io::stdout()),
        CompletionShell::Elvish => generate(Elvish, &mut command, "watn", &mut io::stdout()),
        CompletionShell::Fish => generate(Fish, &mut command, "watn", &mut io::stdout()),
        CompletionShell::PowerShell => {
            generate(PowerShell, &mut command, "watn", &mut io::stdout())
        }
        CompletionShell::Zsh => generate(Zsh, &mut command, "watn", &mut io::stdout()),
    }
    std::process::exit(0)
}

fn build_registry(
    registry: &mut ProviderRegistry,
    active_provider: &str,
    endpoint: &str,
    api_key: &str,
    interrupt: Arc<AtomicBool>,
) {
    registry.register(
        active_provider.to_string(),
        Box::new(OpenAICompatibleProvider::new(
            endpoint.to_string(),
            api_key.to_string(),
            interrupt,
        )),
    );
}

fn run_provider_setup_command() {
    if !std::io::stdin().is_terminal() {
        watn::provider::setup::print_setup_guidance();
        std::process::exit(1);
    }

    let mut config = match load_config() {
        Ok(config) => config,
        Err(error) => {
            eprintln!("{}", error);
            std::process::exit(exit_code(&error));
        }
    };
    match watn::setup::run_with_config(&config, SetupEntryPoint::Provider) {
        Ok(SetupWizardOutcome::Saved(result)) => {
            if let Err(error) = watn::setup::apply_result(&mut config, &result) {
                eprintln!("{}", error);
                std::process::exit(exit_code(&error));
            }
            println!("Provider configured: {}", result.provider.name);
        }
        Ok(SetupWizardOutcome::Cancelled(cancellation)) => {
            exit_setup_cancellation(cancellation);
        }
        Err(e) => {
            let code = exit_code(&e);
            eprintln!("{}", e);
            std::process::exit(code);
        }
    }
}

fn run_setup_command() {
    let mut config = match load_config() {
        Ok(config) => config,
        Err(error) => {
            eprintln!("{}", error);
            std::process::exit(exit_code(&error));
        }
    };
    if !std::io::stdin().is_terminal() {
        watn::provider::setup::print_setup_guidance();
        std::process::exit(1);
    }
    match watn::setup::run_with_config(&config, SetupEntryPoint::Setup) {
        Ok(SetupWizardOutcome::Saved(result)) => {
            if let Err(error) = watn::setup::apply_result(&mut config, &result) {
                eprintln!("{}", error);
                std::process::exit(exit_code(&error));
            }
            println!("Setup complete");
        }
        Ok(SetupWizardOutcome::Cancelled(cancellation)) => {
            exit_setup_cancellation(cancellation);
        }
        Err(error) => {
            eprintln!("{}", error);
            std::process::exit(exit_code(&error));
        }
    }
}

fn run_shell_setup_command() {
    if !std::io::stdin().is_terminal() {
        eprintln!("Run `watn shell` in a terminal to configure shell integrations.");
        std::process::exit(1);
    }
    let config = match load_config() {
        Ok(config) => config,
        Err(error) => {
            eprintln!("{}", error);
            std::process::exit(exit_code(&error));
        }
    };
    match watn::setup::run_with_config(&config, SetupEntryPoint::Shell) {
        Ok(SetupWizardOutcome::Saved(result)) => {
            if let Err(error) = watn::setup::apply_shell_result(&result) {
                eprintln!("{}", error);
                std::process::exit(exit_code(&error));
            }
            println!("Shell setup complete");
        }
        Ok(SetupWizardOutcome::Cancelled(cancellation)) => {
            exit_setup_cancellation(cancellation);
        }
        Err(error) => {
            eprintln!("{}", error);
            std::process::exit(exit_code(&error));
        }
    }
}

fn run_models_command(
    set_small: Option<String>,
    set_normal: Option<String>,
    set_thinking: Option<String>,
) {
    match watn::models::run_models_result(set_small, set_normal, set_thinking) {
        watn::provider::setup::ModelSetupResult::Saved => {}
        watn::provider::setup::ModelSetupResult::Cancelled(cancellation) => {
            exit_setup_cancellation(cancellation);
        }
        watn::provider::setup::ModelSetupResult::Failed(error) => {
            eprintln!("error: failed to configure models: {}", error);
            std::process::exit(exit_code(&error));
        }
    }
}

fn run_quicksetup_command(defaults: watn::quicksetup::QuickSetupDefaults) {
    if !std::io::stdin().is_terminal() {
        eprintln!("Run `watn quicksetup` in a terminal to configure watn.");
        std::process::exit(1);
    }
    let _config = match load_config() {
        Ok(config) => config,
        Err(error) => {
            eprintln!("{}", error);
            std::process::exit(exit_code(&error));
        }
    };
    if let Err(error) = watn::quicksetup::run_with_defaults(defaults) {
        eprintln!("{}", error);
        std::process::exit(exit_code(&error));
    }
}

fn exit_setup_cancellation(cancellation: watn::provider::setup::SetupCancellation) -> ! {
    let code = match cancellation {
        watn::provider::setup::SetupCancellation::Escape => 1,
        watn::provider::setup::SetupCancellation::CtrlC => 130,
    };
    std::process::exit(code);
}
