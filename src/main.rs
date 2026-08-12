use clap::{CommandFactory, Parser};
use clap_complete::generate;
use clap_complete::shells::{Bash, Elvish, Fish, PowerShell, Zsh};

use std::io::{self, IsTerminal};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use watn::config::{self, read_config, resolve_model, resolve_provider};
use watn::error::exit_code;
use watn::output::render;
use watn::provider::openai_compat::OpenAICompatibleProvider;
use watn::provider::registry::ProviderRegistry;
use watn::provider::{Message, RequestOptions, StreamEvent};
use watn::setup::SetupWizardOutcome;

#[derive(clap::Parser)]
#[command(name = "watn", version = env!("CARGO_PKG_VERSION"))]
#[command(about = "Ask in plain language. Get one command.")]
struct Cli {
    #[arg(group = "input", num_args = 1..)]
    question: Vec<String>,

    #[arg(short = '1', long = "small")]
    tier_small: bool,

    #[arg(short = '2', long = "normal")]
    tier_normal: bool,

    #[arg(short = '3', long = "thinking")]
    tier_thinking: bool,

    #[arg(short = 'x', long = "execute")]
    execute: bool,

    #[arg(short = 'v', long = "verbose")]
    verbose: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(clap::Subcommand)]
enum Commands {
    Setup,
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

    if cli.command.is_none()
        && cli
            .question
            .first()
            .is_some_and(|value| matches!(value.as_str(), "provider" | "models"))
    {
        eprintln!(
            "error: removed setup command; use `watn setup` for provider and model configuration"
        );
        std::process::exit(2);
    }

    if let Some(command) = &cli.command {
        match command {
            Commands::Setup => run_setup_command(),
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

    let persisted = match read_config() {
        Ok(c) => c,
        Err(e) => {
            let code = exit_code(&e);
            eprintln!("{}", e);
            std::process::exit(code);
        }
    };

    if !persisted.exists {
        if !std::io::stdin().is_terminal() {
            watn::provider::setup::print_setup_guidance();
            std::process::exit(1);
        }
        match watn::setup::run_with_persisted_config(
            &persisted,
            watn::setup::SetupEntryPoint::Setup,
        ) {
            Ok(SetupWizardOutcome::Saved(result)) => {
                let mut config = persisted.config.clone();
                if let Err(error) = watn::setup::apply_result(&mut config, &result) {
                    eprintln!("{}", error);
                    std::process::exit(exit_code(&error));
                }
                eprintln!("Setup complete. Retry your command.");
                return;
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

    let mut config = persisted.config;

    let provider_name = config.defaults.provider.as_deref().unwrap_or("openrouter");

    if !config::provider_ready(&config, provider_name) {
        if !std::io::stdin().is_terminal() {
            watn::provider::setup::print_setup_guidance();
            std::process::exit(1);
        }
        match watn::setup::run_with_config(&config, watn::setup::SetupEntryPoint::Setup) {
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
    let model = match resolve_model(&config, tier, None) {
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

    let mut registry = ProviderRegistry::new();
    build_registry(
        &mut registry,
        provider_name,
        &provider_config.endpoint,
        &api_key,
    );

    let provider = registry.get(provider_name).unwrap();

    let system_prompt = format!(
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
    );

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

    let interrupted = Arc::new(AtomicBool::new(false));
    let int_flag = interrupted.clone();
    ctrlc::set_handler(move || {
        int_flag.store(true, Ordering::SeqCst);
    })
    .expect("install SIGINT handler");

    let mut spinner = Some(watn::output::spinner::Spinner::start(&model));
    let mut output = render::StreamRenderer::new(io::stdout());
    let stream_result = {
        let mut emit_content = |event: StreamEvent| -> Result<(), watn::error::Error> {
            match event {
                StreamEvent::Content(content) if !content.is_empty() => {
                    if !output.has_content() {
                        if let Some(active_spinner) = spinner.take() {
                            active_spinner.finish();
                        }
                    }

                    output
                        .write_content(&content)
                        .map_err(watn::error::Error::IoError)?;
                }
                StreamEvent::Content(_) => {}
            }
            Ok(())
        };

        provider.chat_completions_streaming(&messages, &options, &mut emit_content)
    };

    match stream_result {
        Ok(response) => {
            if let Some(active_spinner) = spinner.take() {
                active_spinner.finish();
            }

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
            if output.has_content() {
                let _ = output.finish_partial();
            }
            let code = exit_code(&e);
            eprintln!("{}", e);
            std::process::exit(code);
        }
    }

    if interrupted.load(Ordering::SeqCst) {
        std::process::exit(130);
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
) {
    registry.register(
        active_provider.to_string(),
        Box::new(OpenAICompatibleProvider::new(
            endpoint.to_string(),
            api_key.to_string(),
        )),
    );
}

fn run_setup_command() {
    if !std::io::stdin().is_terminal() {
        watn::provider::setup::print_setup_guidance();
        std::process::exit(1);
    }
    let persisted = match read_config() {
        Ok(config) => config,
        Err(error) => {
            eprintln!("{}", error);
            std::process::exit(exit_code(&error));
        }
    };
    let mut config = persisted.config.clone();
    match watn::setup::run_with_persisted_config(&persisted, watn::setup::SetupEntryPoint::Setup) {
        Ok(SetupWizardOutcome::Saved(result)) => {
            if let Err(error) = watn::setup::apply_result(&mut config, &result) {
                eprintln!("{}", error);
                std::process::exit(exit_code(&error));
            }
            eprintln!("Setup complete.");
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

fn exit_setup_cancellation(cancellation: watn::provider::setup::SetupCancellation) -> ! {
    let code = match cancellation {
        watn::provider::setup::SetupCancellation::Escape => 1,
        watn::provider::setup::SetupCancellation::CtrlC => 130,
    };
    std::process::exit(code);
}
