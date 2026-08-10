use clap::Parser;

use watn::config::{self, load_config, resolve_model, resolve_provider};
use watn::error::exit_code;
use watn::output::render;
use std::io::IsTerminal;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use watn::provider::openai_compat::OpenAICompatibleProvider;
use watn::provider::registry::ProviderRegistry;
use watn::provider::{Message, RequestOptions};
use watn::setup::{SetupEntryPoint, SetupWizardOutcome};

#[derive(clap::Parser)]
#[command(name = "watn", version = "0.1.0")]
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

    #[arg(long = "model", conflicts_with_all = ["tier_small", "tier_normal", "tier_thinking"])]
    model: Option<String>,

    #[arg(short = 'x', long = "execute")]
    execute: bool,

    #[arg(short = 'v', long = "verbose")]
    verbose: bool,

    #[arg(long = "provider")]
    provider: Option<String>,

    #[arg(long = "set-small")]
    set_small: Option<String>,

    #[arg(long = "set-normal")]
    set_normal: Option<String>,

    #[arg(long = "set-thinking")]
    set_thinking: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(clap::Subcommand)]
enum Commands {
    Setup,
    Models,
    Provider,
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

    let provider_name = cli.provider.as_deref().unwrap_or(
        config.defaults.provider.as_deref().unwrap_or("openrouter"),
    );

    let explicit_provider = cli.provider.is_some() || std::env::var("WATN_PROVIDER").is_ok();
    if !explicit_provider && !config::provider_ready(&config, provider_name) {
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

    let mut registry = ProviderRegistry::new();
    build_registry(&mut registry, &config, provider_name, &provider_config.endpoint, &api_key);

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
    }).expect("install SIGINT handler");

    let spinner = watn::output::spinner::Spinner::start(&model);
    match provider.chat_completions_streaming(&messages, &options) {
        Ok(response) => {
            spinner.finish();
            let cost = config.pricing.get(&model).map(|p| {
                let input_cost = p.input * response.final_usage.as_ref().map_or(0, |u| u.prompt_tokens) as f64 / 1_000_000.0;
                let output_cost = p.output * response.final_usage.as_ref().map_or(0, |u| u.completion_tokens) as f64 / 1_000_000.0;
                input_cost + output_cost
            });

            let elapsed = response.elapsed_secs;
            let tok_s = if elapsed > 0.0 {
                response.final_usage.as_ref().map_or(0.0, |u| u.completion_tokens as f64) / elapsed
            } else {
                0.0
            };

            let command_text = response.full_content.trim().to_string();

            if cli.verbose {
                if let Some(ref reasoning) = response.reasoning_content {
                    if !reasoning.trim().is_empty() {
                        eprintln!("reasoning: {}", reasoning.trim());
                    }
                }
            }

            render::print_response(&command_text, &response.model, tok_s, cost, elapsed);

            if cli.execute && !command_text.is_empty() {
                if matches!(
                    watn::exec::prompt_and_execute(&command_text),
                    watn::exec::PromptResult::Interrupted
                ) {
                    std::process::exit(130);
                }
            }
        }
        Err(e) => {
            spinner.finish();
            let code = exit_code(&e);
            eprintln!("{}", e);
            std::process::exit(code);
        }
    }

    if interrupted.load(Ordering::SeqCst) {
        std::process::exit(130);
    }
}

fn build_registry(
    registry: &mut ProviderRegistry,
    _config: &watn::config::types::Config,
    active_provider: &str,
    endpoint: &str,
    api_key: &str,
) {
    registry.register(
        active_provider.to_string(),
        Box::new(OpenAICompatibleProvider::new(endpoint.to_string(), api_key.to_string())),
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
    match watn::setup::run_with_config(&config, SetupEntryPoint::Setup) {
        Ok(SetupWizardOutcome::Saved(result)) => {
            if let Err(error) = watn::setup::apply_result(&mut config, &result) {
                eprintln!("{}", error);
                std::process::exit(exit_code(&error));
            }
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

fn exit_setup_cancellation(cancellation: watn::provider::setup::SetupCancellation) -> ! {
    let code = match cancellation {
        watn::provider::setup::SetupCancellation::Escape => 1,
        watn::provider::setup::SetupCancellation::CtrlC => 130,
    };
    std::process::exit(code);
}
