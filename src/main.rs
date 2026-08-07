use clap::Parser;

use config::{load_config, resolve_endpoint, resolve_model, resolve_provider};
use error::exit_code;
use output::render;
use std::io::IsTerminal;
use provider::openai_compat::OpenAICompatProvider;
use provider::{Message, Provider, RequestOptions};

mod config;
mod error;
mod exec;
mod models;
mod output;
mod provider;

#[derive(clap::Parser)]
#[command(name = "watn", version = "0.1.0")]
#[command(about = "Ask in plain language. Get one command.")]
struct Cli {
    #[arg(group = "input")]
    question: Option<String>,

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
    Models,
}

impl Cli {
    fn tier(&self) -> Option<&str> {
        if self.tier_small || (self.question.is_some() && !self.tier_normal && !self.tier_thinking) {
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

    if let Some(_) = &cli.command {
        models::run_models(cli.set_small, cli.set_normal, cli.set_thinking);
        return;
    }

    let question = match &cli.question {
        Some(q) if !q.is_empty() => q.clone(),
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

    let config = match load_config() {
        Ok(c) => c,
        Err(e) => {
            let code = exit_code(&e);
            eprintln!("{}", e);
            std::process::exit(code);
        }
    };

    let provider_name = cli.provider.as_deref().unwrap_or(
        config.defaults.provider.as_deref().unwrap_or("openai"),
    );

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

    let endpoint = resolve_endpoint(provider_name, &provider_config);

    let api_key = match config::get_provider_api_key(provider_name, &provider_config) {
        Ok(k) => k,
        Err(e) => {
            let code = exit_code(&e);
            eprintln!("{}", e);
            std::process::exit(code);
        }
    };

    let proc = OpenAICompatProvider::new(endpoint, api_key);

    let messages = vec![Message {
        role: "user".to_string(),
        content: question,
    }];

    match proc.chat_completions_streaming(&messages, &RequestOptions {
        model: model.clone(),
        streaming: true,
        temperature: None,
        max_tokens: None,
    }) {
        Ok(response) => {
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

            render::print_response(&command_text, &response.model, tok_s, cost);

            if cli.execute && !command_text.is_empty() {
                exec::prompt_and_execute(&command_text);
            }
        }
        Err(e) => {
            let code = exit_code(&e);
            eprintln!("{}", e);
            std::process::exit(code);
        }
    }
}
