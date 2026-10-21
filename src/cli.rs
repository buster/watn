//! CLI types, command dispatch, and completion generation.

use clap::CommandFactory;
use clap_complete::generate;
use clap_complete::shells::{Bash, Elvish, Fish, PowerShell, Zsh};
use std::io;

use watn::error::exit_code;

use crate::explain::run_explain_command;
use crate::setup_commands::{
    run_models_command, run_provider_setup_command, run_quicksetup_command, run_setup_command,
    run_shell_setup_command,
};

#[derive(clap::Parser)]
#[command(name = "watn", version = env!("CARGO_PKG_VERSION"))]
#[command(about = "Ask in plain language. Get one command.")]
pub(crate) struct Cli {
    #[arg(
        group = "input",
        num_args = 1..,
        value_name = "QUESTION",
        help = "Natural-language question to turn into a command"
    )]
    pub(crate) question: Vec<String>,

    #[arg(short = '1', long = "small", help = "Use the small/fast model tier")]
    pub(crate) tier_small: bool,

    #[arg(short = '2', long = "normal", help = "Use the balanced model tier")]
    pub(crate) tier_normal: bool,

    #[arg(
        short = '3',
        long = "thinking",
        help = "Use the thinking/reasoning model tier"
    )]
    pub(crate) tier_thinking: bool,

    #[arg(
        long = "model",
        conflicts_with_all = ["tier_small", "tier_normal", "tier_thinking"],
        help = "Use an explicit model instead of a tier"
    )]
    pub(crate) model: Option<String>,

    #[arg(
        short = 'x',
        long = "execute",
        help = "Prompt before executing the generated command"
    )]
    pub(crate) execute: bool,

    #[arg(
        long = "review-panel",
        conflicts_with = "no_review_panel",
        help = "Force the explanatory review surface on for this invocation"
    )]
    pub(crate) review_panel: bool,

    #[arg(
        long = "no-review-panel",
        help = "Disable the explanatory review surface for this invocation"
    )]
    pub(crate) no_review_panel: bool,

    #[arg(
        short = 'v',
        long = "verbose",
        help = "Print provider reasoning to stderr when available"
    )]
    pub(crate) verbose: bool,

    #[arg(long = "provider", help = "Select a configured provider")]
    pub(crate) provider: Option<String>,

    #[arg(
        long = "set-small",
        value_name = "MODEL",
        help = "Set the small-tier model non-interactively"
    )]
    pub(crate) set_small: Option<String>,

    #[arg(
        long = "set-normal",
        value_name = "MODEL",
        help = "Set the normal-tier model non-interactively"
    )]
    pub(crate) set_normal: Option<String>,

    #[arg(
        long = "set-thinking",
        value_name = "MODEL",
        help = "Set the thinking-tier model non-interactively"
    )]
    pub(crate) set_thinking: Option<String>,

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
    pub(crate) fn tier(&self) -> Option<&str> {
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
pub(crate) fn apply_review_switch(enabled: bool) -> ! {
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

pub(crate) fn dispatch_command(cli: &Cli) -> bool {
    let Some(command) = &cli.command else {
        return false;
    };

    match command {
        Commands::Setup => run_setup_command(),
        Commands::Models => {
            run_models_command(
                cli.set_small.clone(),
                cli.set_normal.clone(),
                cli.set_thinking.clone(),
            );
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

    true
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
