//! Shared runtime helpers for errors, configuration, providers, and threads.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use watn::config::{self, load_config, resolve_model, resolve_provider};
use watn::error::exit_code;
use watn::provider::openai_compat::OpenAICompatibleProvider;
use watn::provider::registry::ProviderRegistry;

pub(crate) struct Runtime {
    pub(crate) model: String,
    pub(crate) provider_config: watn::config::types::ProviderConfig,
    pub(crate) api_key: String,
}

pub(crate) fn exit_with_error(error: &watn::error::Error) -> ! {
    let code = exit_code(error);
    eprintln!("{error}");
    std::process::exit(code);
}

pub(crate) fn exit_with_io(error: std::io::Error) -> ! {
    exit_with_error(&watn::error::Error::IoError(error));
}

pub(crate) fn load_config_or_exit() -> watn::config::types::Config {
    match load_config() {
        Ok(config) => config,
        Err(error) => exit_with_error(&error),
    }
}

pub(crate) fn resolve_model_or_exit(
    config: &watn::config::types::Config,
    tier: Option<&str>,
    explicit_model: Option<&str>,
) -> String {
    match resolve_model(config, tier, explicit_model) {
        Ok(model) => model,
        Err(error) => exit_with_error(&error),
    }
}

pub(crate) fn resolve_provider_or_exit(
    config: &watn::config::types::Config,
    provider_name: &str,
) -> watn::config::types::ProviderConfig {
    match resolve_provider(config, provider_name) {
        Ok(provider_config) => provider_config,
        Err(error) => exit_with_error(&error),
    }
}

pub(crate) fn get_api_key_or_exit(
    provider_name: &str,
    provider_config: &watn::config::types::ProviderConfig,
) -> String {
    match config::get_provider_api_key(provider_name, provider_config) {
        Ok(api_key) => api_key,
        Err(error) => exit_with_error(&error),
    }
}

pub(crate) fn resolve_runtime(
    config: &watn::config::types::Config,
    provider_name: &str,
    tier: Option<&str>,
    explicit_model: Option<&str>,
) -> Runtime {
    let model = resolve_model_or_exit(config, tier, explicit_model);
    let provider_config = resolve_provider_or_exit(config, provider_name);
    let api_key = get_api_key_or_exit(provider_name, &provider_config);
    Runtime {
        model,
        provider_config,
        api_key,
    }
}

pub(crate) fn wait_for_scoped_result<T>(
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

pub(crate) fn build_registry(
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
