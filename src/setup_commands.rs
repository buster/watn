//! Setup, models, provider, shell, and quick-setup command handlers.

use std::io::IsTerminal;

use watn::config::{self, load_config};
use watn::error::exit_code;
use watn::setup::{SetupEntryPoint, SetupWizardOutcome};

use crate::runtime::exit_with_error;

pub(crate) fn ensure_ready(
    config: &mut watn::config::types::Config,
    provider_name: &str,
    explicit_selection: bool,
) -> bool {
    if explicit_selection {
        return true;
    }
    if config::provider_ready(config, provider_name) && config::model_roles_ready(config) {
        return true;
    }
    if !std::io::stdin().is_terminal() {
        watn::provider::setup::print_setup_guidance();
        std::process::exit(1);
    }
    if !config::config_file_exists() {
        if let Err(error) = watn::quicksetup::run() {
            exit_with_error(&error);
        }
        return false;
    }

    match watn::setup::run_with_config(config, SetupEntryPoint::Setup) {
        Ok(SetupWizardOutcome::Saved(result)) => {
            if let Err(error) = watn::setup::apply_result(config, &result) {
                exit_with_error(&error);
            }
            false
        }
        Ok(SetupWizardOutcome::Cancelled(cancellation)) => exit_setup_cancellation(cancellation),
        Err(error) => exit_with_error(&error),
    }
}

pub(crate) fn run_provider_setup_command() {
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

pub(crate) fn run_setup_command() {
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

pub(crate) fn run_shell_setup_command() {
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

pub(crate) fn run_models_command(
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

pub(crate) fn run_quicksetup_command(defaults: watn::quicksetup::QuickSetupDefaults) {
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

pub(crate) fn exit_setup_cancellation(cancellation: watn::provider::setup::SetupCancellation) -> ! {
    let code = match cancellation {
        watn::provider::setup::SetupCancellation::Escape => 1,
        watn::provider::setup::SetupCancellation::CtrlC => 130,
    };
    std::process::exit(code);
}
