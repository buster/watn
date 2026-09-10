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

type StreamOutcome = (
    Result<StreamingResponse, watn::error::Error>,
    Option<watn::output::spinner::Spinner>,
    CommandSink,
);

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
    Quicksetup,
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
            Commands::Quicksetup => run_quicksetup_command(),
            Commands::Completions { shell } => run_completions(shell),
        }
        return;
    }

    let question = match &cli.question {
        q if !q.is_empty() => q.join(" "),
        _ => {
            if !std::io::stdin().is_terminal() {
                let mut buf = String::new();
                std::io::Read::read_to_string(&mut std::io::stdin(), &mut buf).unwrap_or_default();
                if buf.trim().is_empty() {
                    eprintln!("Usage: watn <question>");
                    eprintln!("   or: echo \"question\" | watn");
                    std::process::exit(1);
                }
                buf.trim().to_string()
            } else {
                eprintln!("Usage: watn <question>");
                eprintln!("   or: echo \"question\" | watn");
                std::process::exit(1);
            }
        }
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
    let review_eligible = watn::review::controlling_terminal_is_usable();
    let review_enabled =
        watn::review::resolve_review_enabled(&config, review_override, review_eligible);

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
        max_tokens: None,
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

    let (stream_result, mut spinner, mut sink) = {
        let worker_provider_name = provider_name.to_string();
        let worker_messages = messages;
        let worker_options = options;
        let stream_handle = std::thread::spawn(move || {
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
        wait_for_stream_result(stream_handle, &interrupt)
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
                        &response,
                        &buffer,
                        &intent,
                        tier.unwrap_or("1"),
                        provider_name,
                        &model,
                        cli.execute,
                        cli.verbose,
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

            if cli.execute
                && !command_text.is_empty()
                && matches!(
                    watn::exec::prompt_and_execute(&command_text),
                    watn::exec::PromptResult::Interrupted
                )
            {
                std::process::exit(130);
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

#[allow(clippy::too_many_arguments)]
fn run_review_path(
    response: &StreamingResponse,
    buffer: &watn::review::ReviewBuffer,
    intent: &str,
    tier: &str,
    provider: &str,
    model: &str,
    execute: bool,
    verbose: bool,
    config: &watn::config::types::Config,
) -> ! {
    let raw = buffer.candidate().unwrap_or_default();
    let candidate = match watn::review::candidate_from_provider_response(raw) {
        Some(candidate) => candidate,
        None => {
            eprintln!("review unavailable: no complete command candidate");
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
    let mut panel = watn::review::InlineReviewPanel::new(
        terminal,
        watn::review::ReviewPanelState::new(context, candidate.clone()),
    );
    if let Err(error) = panel.render() {
        eprintln!("review unavailable: {error}");
        std::process::exit(1);
    }

    let accepted = loop {
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
                std::process::exit(0);
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
        if verbose {
            if let Some(reasoning) = &response.reasoning_content {
                if !reasoning.trim().is_empty() {
                    let _ = render::print_reasoning(reasoning);
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
        let _ = render::print_metadata(&response.model, tok_s, cost, elapsed);

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

fn wait_for_stream_result(
    handle: std::thread::JoinHandle<StreamOutcome>,
    interrupt: &AtomicBool,
) -> StreamOutcome {
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
                drop(handle);
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

fn run_quicksetup_command() {
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
    if let Err(error) = watn::quicksetup::run() {
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
