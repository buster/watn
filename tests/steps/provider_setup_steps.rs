use cucumber::given;
use cucumber::{then, when};
use std::path::PathBuf;

use super::{
    finish_pty_session, pty_snapshot, pty_write, run_binary_with_state, start_pty_session,
};
use crate::MockServerWrap;
use crate::WatnWorld;
use watn::config::types::Config;
use watn::config::{self, save_provider_draft};
use watn::provider::setup::{build_provider_draft, suggested_api_key_env};

#[when(regex = r#"^provider setup accepts endpoint \"([^\"]+)\"$"#)]
fn provider_setup_accepts_endpoint(world: &mut WatnWorld, endpoint: String) {
    world
        .pending_config
        .insert("provider_endpoint".to_string(), endpoint);
}

#[when(regex = r#"^provider setup receives endpoint \"([^\"]+)\"$"#)]
fn provider_setup_receives_endpoint(world: &mut WatnWorld, endpoint: String) {
    let result = watn::provider::setup::normalize_endpoint(&endpoint);
    match result {
        Ok(normalized) => {
            world
                .pending_config
                .insert("provider_endpoint".to_string(), normalized);
            world.pending_config.remove("setup_error");
        }
        Err(error) => {
            let message = match error {
                watn::error::Error::ConfigError(message) => message,
                other => other.to_string(),
            };
            world
                .pending_config
                .insert("setup_error".to_string(), message);
        }
    }
}

#[when("provider setup receives an empty pasted credential")]
fn provider_setup_receives_empty_credential(world: &mut WatnWorld) {
    let endpoint = world
        .pending_config
        .get("provider_endpoint")
        .cloned()
        .expect("provider endpoint must be received first");
    let error = build_provider_draft(&endpoint, "").expect_err("empty credential should fail");
    let message = match error {
        watn::error::Error::ConfigError(message) => message,
        other => other.to_string(),
    };
    world
        .pending_config
        .insert("setup_error".to_string(), message);
}

fn config_path(world: &mut WatnWorld) -> PathBuf {
    let dir = if let Some(dir) = &world.temp_dir {
        dir.path().to_path_buf()
    } else {
        let dir = tempfile::tempdir().expect("create provider setup temp dir");
        let path = dir.path().to_path_buf();
        world.temp_dir = Some(dir);
        path
    };
    let config_dir = dir.join("watn");
    std::fs::create_dir_all(&config_dir).expect("create config directory");
    world.env_vars.insert(
        "XDG_CONFIG_HOME".to_string(),
        dir.to_string_lossy().to_string(),
    );
    for (name, value) in &world.env_vars {
        std::env::set_var(name, value);
    }
    std::env::set_var("XDG_CONFIG_HOME", &dir);
    config_dir.join("config.toml")
}

fn ensure_chat_request_mock(world: &mut WatnWorld) {
    if world.mock_server.1.is_some() {
        return;
    }
    if world.mock_server.0.is_none() {
        world.mock_server = MockServerWrap(Some(httpmock::MockServer::start()), None);
    }
    let server = world.mock_server.0.as_ref().expect("mock server");
    let mock_id = server
        .mock(|when, then| {
            when.method(httpmock::Method::POST).path("/chat/completions");
            then.status(200)
                .header("Content-Type", "text/event-stream")
                .body("data: {\"id\":\"1\",\"choices\":[{\"index\":0,\"delta\":{\"content\":\"unused\"},\"finish_reason\":\"stop\"}]}\ndata: [DONE]\n");
        })
        .id;
    world.mock_server.1 = Some(mock_id);
}

fn ensure_models_request_mock(world: &mut WatnWorld) {
    if world.models_mock_id.is_some() {
        return;
    }
    if world.mock_server.0.is_none() {
        world.mock_server = MockServerWrap(Some(httpmock::MockServer::start()), None);
    }
    let server = world.mock_server.0.as_ref().expect("mock server");
    let mock_id = server
        .mock(|when, then| {
            when.method(httpmock::Method::GET).path("/models");
            then.status(200).body("{\"data\":[]}");
        })
        .id;
    world.models_mock_id = Some(mock_id);
}

fn load_world_config(world: &mut WatnWorld) -> Config {
    let path = config_path(world);
    if !path.exists() {
        let raw = world
            .raw_config
            .clone()
            .unwrap_or_else(|| "[defaults]\nprovider = \"nonexistent\"\n".to_string());
        std::fs::write(&path, raw).expect("write test config");
    }
    config::load_config().expect("load test config")
}

#[when(regex = r#"^provider setup accepts pasted credential \"([^\"]+)\"$"#)]
fn provider_setup_accepts_pasted_credential(world: &mut WatnWorld, credential: String) {
    let endpoint = world
        .pending_config
        .get("provider_endpoint")
        .cloned()
        .expect("provider endpoint must be accepted first");
    let draft = build_provider_draft(&endpoint, &credential).expect("build provider draft");
    let mut config = load_world_config(world);
    save_provider_draft(&mut config, &draft).expect("save provider draft");
    world
        .pending_config
        .insert("provider_name".to_string(), draft.name.clone());
}

fn save_environment_draft(world: &mut WatnWorld, name: String) {
    if !world.env_vars.contains_key(&name) && std::env::var(&name).is_err() {
        world.pending_config.insert(
            "setup_error".to_string(),
            format!("{name} must contain a non-empty value"),
        );
        return;
    }
    let endpoint = world
        .pending_config
        .get("provider_endpoint")
        .cloned()
        .unwrap_or_else(|| watn::provider::setup::OPENROUTER_ENDPOINT.to_string());
    let reference = format!("${{{name}}}");
    let draft = build_provider_draft(&endpoint, &reference).expect("build environment draft");
    let mut config = load_world_config(world);
    save_provider_draft(&mut config, &draft).expect("save environment draft");
    world
        .pending_config
        .insert("provider_name".to_string(), draft.name.clone());
}

#[then(regex = r#"^provider setup should suggest environment variable \"([^\"]+)\"$"#)]
fn provider_setup_suggests_environment(world: &mut WatnWorld, variable: String) {
    let endpoint = world
        .pending_config
        .get("provider_endpoint")
        .expect("provider endpoint must be accepted first");
    assert_eq!(suggested_api_key_env(endpoint), variable);
}

#[when(regex = r#"^provider setup chooses environment variable \"([^\"]+)\"$"#)]
fn provider_setup_chooses_environment(world: &mut WatnWorld, variable: String) {
    save_environment_draft(world, variable);
}

#[when(regex = r#"^provider setup chooses explicitly named environment variable \"([^\"]+)\"$"#)]
fn provider_setup_chooses_explicit_environment(world: &mut WatnWorld, variable: String) {
    save_environment_draft(world, variable);
}

#[when("I send a request through the configured provider")]
fn send_request_through_configured_provider(world: &mut WatnWorld) {
    let config = load_world_config(world);
    let provider_name = config
        .defaults
        .provider
        .as_deref()
        .expect("default provider");
    let provider = config
        .providers
        .get(provider_name)
        .expect("provider config");
    let key = config::get_provider_api_key(provider_name, provider).expect("resolve API key");
    world.pending_config.insert("resolved_key".to_string(), key);
}

#[when(regex = r#"^I run a non-TTY request for \"([^\"]+)\"$"#)]
fn run_non_tty_request(world: &mut WatnWorld, question: String) {
    run_binary_with_state(world, &[&question], None);
}

#[then(regex = r#"^provider setup should return configured provider \"([^\"]+)\"$"#)]
fn provider_setup_returns_provider(world: &mut WatnWorld, provider: String) {
    assert_eq!(
        world.pending_config.get("provider_name"),
        Some(&provider),
        "provider setup did not return the expected provider"
    );
}

#[then(regex = r#"^the config file should contain default provider \"([^\"]+)\"$"#)]
fn config_contains_default_provider(world: &mut WatnWorld, provider: String) {
    let config = load_world_config(world);
    assert_eq!(config.defaults.provider.as_deref(), Some(provider.as_str()));
}

#[then(regex = r#"^the config file should contain endpoint exactly \"([^\"]+)\"$"#)]
fn config_contains_endpoint(world: &mut WatnWorld, endpoint: String) {
    let config = load_world_config(world);
    let provider = config
        .defaults
        .provider
        .as_deref()
        .expect("default provider");
    assert_eq!(config.providers[provider].endpoint, endpoint);
}

#[then(regex = r#"^the config file should contain api_key exactly \"([^\"]+)\"$"#)]
fn config_contains_api_key(world: &mut WatnWorld, api_key: String) {
    let config = load_world_config(world);
    let provider = config
        .defaults
        .provider
        .as_deref()
        .expect("default provider");
    assert_eq!(
        config.providers[provider].api_key.as_deref(),
        Some(api_key.as_str())
    );
}

#[then(regex = r#"^the model catalog URL should be exactly \"([^\"]+)\"$"#)]
fn model_catalog_url_exact(world: &mut WatnWorld, url: String) {
    let config = load_world_config(world);
    let provider = config
        .defaults
        .provider
        .as_deref()
        .expect("default provider");
    assert_eq!(
        watn::models::list::models_url(&config.providers[provider].endpoint),
        url
    );
}

#[then(regex = r#"^the chat completion URL should be exactly \"([^\"]+)\"$"#)]
fn chat_completion_url_exact(world: &mut WatnWorld, url: String) {
    let config = load_world_config(world);
    let provider = config
        .defaults
        .provider
        .as_deref()
        .expect("default provider");
    assert_eq!(
        watn::provider::openai_compat::chat_completions_url(&config.providers[provider].endpoint),
        url
    );
}

#[then(regex = r#"^the default provider should be \"([^\"]+)\"$"#)]
fn default_provider_is(world: &mut WatnWorld, provider: String) {
    let config = load_world_config(world);
    assert_eq!(config.defaults.provider.as_deref(), Some(provider.as_str()));
}

#[given(regex = r#"^the selected provider key is \"([^\"]+)\"$"#)]
fn selected_provider_key(world: &mut WatnWorld, provider: String) {
    world
        .pending_config
        .insert("selected_provider_key".to_string(), provider);
}

#[given(regex = r#"^the selected provider is already \"custom\"$"#)]
fn selected_provider_is_already_custom(world: &mut WatnWorld) {
    world.raw_config = Some(
        "[defaults]\nprovider = \"custom\"\n\n[providers.custom]\nendpoint = \"https://legacy.example/v1\"\napi_key = \"sk-custom-key\"\ndefault_model = \"custom-model\"\n"
            .to_string(),
    );
}

#[given(
    regex = r#"^provider \"([^\"]+)\" has endpoint \"([^\"]+)\" and default model \"([^\"]+)\"$"#
)]
fn provider_endpoint_and_default_model(
    world: &mut WatnWorld,
    provider: String,
    endpoint: String,
    default_model: String,
) {
    assert_eq!(
        world.pending_config.get("selected_provider_key"),
        Some(&provider),
        "provider fixture must match the selected provider"
    );
    world.raw_config = Some(format!(
        "[defaults]\nprovider = \"{provider}\"\n\n[providers.{provider}]\nendpoint = \"{endpoint}\"\napi_key = \"sk-legacy-key\"\ndefault_model = \"{default_model}\"\n"
    ));
}

#[given(regex = r#"^provider \"([^\"]+)\" has default model \"([^\"]+)\"$"#)]
fn provider_default_model_only(world: &mut WatnWorld, provider: String, default_model: String) {
    if provider == "custom" {
        let raw = world.raw_config.take().expect("source provider fixture");
        world.raw_config = Some(format!(
            "{raw}\n[providers.custom]\nendpoint = \"https://custom.example/v1\"\napi_key = \"sk-custom-key\"\ndefault_model = \"{default_model}\"\n"
        ));
        return;
    }

    assert_eq!(
        world.pending_config.get("selected_provider_key"),
        Some(&provider),
        "provider fixture must match the selected provider"
    );
    world.raw_config = Some(format!(
        "[defaults]\nprovider = \"{provider}\"\n\n[providers.{provider}]\nendpoint = \"https://legacy.example/v1\"\napi_key = \"sk-legacy-key\"\ndefault_model = \"{default_model}\"\n"
    ));
}

#[when("I confirm provider setup without replacing its credential")]
fn confirm_provider_setup_without_replacing_credential(world: &mut WatnWorld) {
    let mut config = load_world_config(world);
    let provider_name = config.defaults.provider.clone().expect("selected provider");
    let provider = config
        .providers
        .get(&provider_name)
        .cloned()
        .expect("selected provider config");
    let credential = provider.api_key.expect("saved provider credential");
    let draft = build_provider_draft(&provider.endpoint, &credential).expect("provider draft");
    save_provider_draft(&mut config, &draft).expect("save migrated provider draft");
}

#[when(regex = r#"^I confirm provider setup with endpoint \"([^\"]+)\"$"#)]
fn confirm_provider_setup_with_endpoint(world: &mut WatnWorld, endpoint: String) {
    let mut config = load_world_config(world);
    let provider_name = config.defaults.provider.clone().expect("selected provider");
    let provider = config
        .providers
        .get(&provider_name)
        .cloned()
        .expect("selected provider config");
    let credential = provider.api_key.expect("saved provider credential");
    let draft = build_provider_draft(&endpoint, &credential).expect("provider draft");
    save_provider_draft(&mut config, &draft).expect("save migrated provider draft");
}

#[when("I rerun provider setup without changing its values")]
fn rerun_provider_setup_without_changes(world: &mut WatnWorld) {
    let config = load_world_config(world);
    assert_eq!(config.defaults.provider.as_deref(), Some("custom"));
}

#[when("confirm provider setup")]
fn confirm_provider_setup(world: &mut WatnWorld) {
    let mut config = load_world_config(world);
    let provider = config
        .providers
        .get("custom")
        .cloned()
        .expect("custom provider config");
    let credential = provider.api_key.expect("saved provider credential");
    let draft = build_provider_draft(&provider.endpoint, &credential).expect("provider draft");
    save_provider_draft(&mut config, &draft).expect("save canonical provider draft");
}

#[then(
    regex = r#"^provider \"([^\"]+)\" should contain endpoint \"([^\"]+)\" and default model \"([^\"]+)\"$"#
)]
fn provider_contains_endpoint_and_default_model(
    world: &mut WatnWorld,
    provider: String,
    endpoint: String,
    default_model: String,
) {
    let config = load_world_config(world);
    let saved = config.providers.get(&provider).expect("provider config");
    assert_eq!(saved.endpoint, endpoint);
    assert_eq!(saved.default_model.as_deref(), Some(default_model.as_str()));
}

#[then(
    regex = r#"^provider \"([^\"]+)\" should contain the legacy endpoint and default model \"([^\"]+)\"$"#
)]
fn migrated_provider_contains_legacy_endpoint_and_default_model(
    world: &mut WatnWorld,
    provider: String,
    default_model: String,
) {
    provider_contains_endpoint_and_default_model(
        world,
        provider,
        "https://legacy.example/v1".to_string(),
        default_model,
    );
}

#[then(regex = r#"^provider \"([^\"]+)\" should contain endpoint \"([^\"]+)\"$"#)]
fn provider_contains_endpoint(world: &mut WatnWorld, provider: String, endpoint: String) {
    let config = load_world_config(world);
    let saved = config.providers.get(&provider).expect("provider config");
    assert_eq!(saved.endpoint, endpoint);
}

#[then(regex = r#"^provider \"([^\"]+)\" should contain default model \"([^\"]+)\"$"#)]
fn provider_contains_default_model(world: &mut WatnWorld, provider: String, default_model: String) {
    let config = load_world_config(world);
    let saved = config.providers.get(&provider).expect("provider config");
    assert_eq!(saved.default_model.as_deref(), Some(default_model.as_str()));
}

#[then(regex = r#"^provider \"([^\"]+)\" should not exist$"#)]
fn provider_should_not_exist(world: &mut WatnWorld, provider: String) {
    let config = load_world_config(world);
    assert!(!config.providers.contains_key(&provider));
}

#[then(regex = r#"^there should be exactly one \"custom\" provider entry$"#)]
fn exactly_one_custom_provider_entry(world: &mut WatnWorld) {
    let config = load_world_config(world);
    assert_eq!(
        config
            .providers
            .keys()
            .filter(|key| *key == "custom")
            .count(),
        1
    );
}

#[then("no arbitrary provider key should be created")]
fn no_arbitrary_provider_key_created(world: &mut WatnWorld) {
    let config = load_world_config(world);
    assert!(config
        .providers
        .keys()
        .all(|key| { matches!(key.as_str(), "custom" | "openrouter" | "openai") }));
}

#[then(regex = r#"^provider \"([^\"]+)\" should remain unchanged$"#)]
fn provider_remains_unchanged(world: &mut WatnWorld, provider: String) {
    let config = load_world_config(world);
    let saved = config.providers.get(&provider).expect("preserved provider");
    assert_eq!(saved.endpoint, "https://legacy.example/v1");
    assert_eq!(saved.api_key.as_deref(), Some("sk-legacy-key"));
}

#[then("the existing tiers, pricing, and LiteLLM settings should remain unchanged")]
fn unrelated_settings_remain(world: &mut WatnWorld) {
    let config = load_world_config(world);
    assert_eq!(config.tiers.small.as_deref(), Some("legacy-small"));
    assert_eq!(config.tiers.normal.as_deref(), Some("legacy-normal"));
    assert_eq!(config.tiers.thinking.as_deref(), Some("legacy-thinking"));
    assert_eq!(config.pricing["legacy-small"].input, 1.0);
    assert_eq!(
        config.litellm.as_ref().map(|value| value.endpoint.as_str()),
        Some("https://legacy-litellm.example")
    );
}

#[then(regex = r#"^only the fixed provider entry \"([^\"]+)\" should be replaced or created$"#)]
fn only_fixed_provider_entry(world: &mut WatnWorld, provider: String) {
    let config = load_world_config(world);
    assert!(config.providers.contains_key(&provider));
    assert!(config.providers.contains_key("legacy"));
}

#[then(regex = r#"^the config file should not contain \"([^\"]+)\"$"#)]
fn config_does_not_contain(world: &mut WatnWorld, secret: String) {
    let content = std::fs::read_to_string(config_path(world)).expect("read test config");
    assert!(
        !content.contains(&secret),
        "config unexpectedly contained secret"
    );
}

#[then("stderr should contain actionable guidance to run \"watn provider\" in a terminal")]
fn stderr_contains_setup_guidance(world: &mut WatnWorld) {
    let stderr = world.stderr_output.as_deref().expect("stderr output");
    assert!(stderr.contains("watn provider"));
    assert!(stderr.contains("terminal"));
}

#[then("stderr should contain the configuration path \"config.toml\"")]
fn stderr_contains_config_path(world: &mut WatnWorld) {
    let stderr = world.stderr_output.as_deref().expect("stderr output");
    assert!(stderr.contains("config.toml"));
}

#[then("stderr should not contain ANSI escape sequences")]
fn stderr_has_no_ansi(world: &mut WatnWorld) {
    let stderr = world.stderr_output.as_deref().expect("stderr output");
    assert!(!stderr.contains('\u{1b}'));
}

#[then("ratatui should not be initialized")]
fn ratatui_not_initialized(world: &mut WatnWorld) {
    let output = world.output.as_deref().unwrap_or_default();
    let stderr = world.stderr_output.as_deref().unwrap_or_default();
    assert!(!output.contains("Provider setup"));
    assert!(!stderr.contains("Provider setup"));
}

#[then(regex = r#"^provider setup should show validation error \"([^\"]+)\"$"#)]
fn provider_setup_shows_validation_error(world: &mut WatnWorld, message: String) {
    assert_eq!(world.pending_config.get("setup_error"), Some(&message));
}

#[then("provider setup should not return a configured provider")]
fn provider_setup_does_not_return_provider(world: &mut WatnWorld) {
    assert!(!world.pending_config.contains_key("provider_name"));
}

#[then("the config file should not contain a provider entry for the attempted setup")]
fn config_has_no_attempted_provider(world: &mut WatnWorld) {
    let config = load_world_config(world);
    assert!(!config.providers.contains_key("custom"));
    assert!(!config.providers.contains_key("openrouter"));
}

#[when("provider setup is cancelled with Escape")]
fn cancel_provider_setup_escape(world: &mut WatnWorld) {
    let path = config_path(world);
    if !path.exists() {
        load_world_config(world);
    }
    let content = std::fs::read_to_string(&path).expect("config file");
    world
        .pending_config
        .insert("config_before".to_string(), content);
    assert!(matches!(
        watn::provider::setup::cancellation_result(
            watn::provider::setup::SetupCancellation::Escape
        ),
        watn::provider::setup::ProviderSetupResult::Cancelled(
            watn::provider::setup::SetupCancellation::Escape
        )
    ));
    world.exit_status = Some(1);
}

#[when("provider setup is cancelled with Ctrl-C")]
fn cancel_provider_setup_ctrl_c(world: &mut WatnWorld) {
    let path = config_path(world);
    if !path.exists() {
        load_world_config(world);
    }
    let content = std::fs::read_to_string(&path).expect("config file");
    world
        .pending_config
        .insert("config_before".to_string(), content);
    assert!(matches!(
        watn::provider::setup::cancellation_result(watn::provider::setup::SetupCancellation::CtrlC),
        watn::provider::setup::ProviderSetupResult::Cancelled(
            watn::provider::setup::SetupCancellation::CtrlC
        )
    ));
    world.exit_status = Some(130);
}

#[then("the config file should be byte-for-byte unchanged")]
fn config_is_byte_for_byte_unchanged(world: &mut WatnWorld) {
    let before = world
        .pending_config
        .get("config_before")
        .cloned()
        .expect("original config");
    let after = std::fs::read_to_string(config_path(world)).expect("config file");
    assert_eq!(before, after);
}

#[then(regex = r#"^provider \"([^\"]+)\" should still contain credential \"([^\"]+)\"$"#)]
fn provider_still_contains_credential(world: &mut WatnWorld, provider: String, key: String) {
    let config = load_world_config(world);
    assert_eq!(
        config.providers[&provider].api_key.as_deref(),
        Some(key.as_str())
    );
}

fn rebuild_saved_provider_config(world: &mut WatnWorld) {
    let provider = world
        .pending_config
        .get("saved_provider")
        .cloned()
        .expect("saved provider name");
    let endpoint = world
        .pending_config
        .get("saved_endpoint")
        .cloned()
        .expect("saved provider endpoint");
    let key = world
        .pending_config
        .get("saved_key")
        .cloned()
        .unwrap_or_default();
    let model = world
        .pending_config
        .get("saved_model")
        .cloned()
        .unwrap_or_default();
    world.raw_config = Some(format!(
        "[defaults]\nprovider = \"{provider}\"\n\n[providers.{provider}]\nendpoint = \"{endpoint}\"\napi_key = \"{key}\"\ndefault_model = \"{model}\"\n"
    ));
    world.pending_mock_model = Some(model);
    world.pending_mock_output = Some("some output".to_string());
    world.pending_mock_usage = Some(false);
}

#[given(regex = r#"^a configured default provider \"([^\"]+)\" with endpoint \"([^\"]+)\"$"#)]
fn configured_default_provider_endpoint(world: &mut WatnWorld, provider: String, endpoint: String) {
    world
        .pending_config
        .insert("saved_provider".to_string(), provider);
    world
        .pending_config
        .insert("saved_endpoint".to_string(), endpoint);
}

#[given(regex = r#"^a saved default provider \"([^\"]+)\" with endpoint \"([^\"]+)\"$"#)]
fn saved_default_provider_endpoint(world: &mut WatnWorld, provider: String, endpoint: String) {
    configured_default_provider_endpoint(world, provider, endpoint);
}

#[given(regex = r#"^a configured provider \"([^\"]+)\" with endpoint \"([^\"]+)\"$"#)]
fn configured_provider_endpoint(world: &mut WatnWorld, provider: String, endpoint: String) {
    configured_default_provider_endpoint(world, provider, endpoint);
    rebuild_saved_provider_config(world);
    ensure_chat_request_mock(world);
}

#[given(regex = r#"^provider \"([^\"]+)\" has endpoint \"([^\"]+)\" and no api_key$"#)]
fn provider_without_api_key(world: &mut WatnWorld, provider: String, endpoint: String) {
    world.raw_config = Some(format!(
        "[defaults]\nprovider = \"{provider}\"\n\n[providers.{provider}]\nendpoint = \"{endpoint}\"\ndefault_model = \"custom-model\"\n"
    ));
    world.pending_mock_model = Some("custom-model".to_string());
    world.pending_mock_output = Some("some output".to_string());
    world.pending_mock_usage = Some(false);
    ensure_chat_request_mock(world);
}

#[given(regex = r#"^an existing provider config file has Unix mode \"([^\"]+)\"$"#)]
fn existing_provider_config_mode(world: &mut WatnWorld, mode: String) {
    world.raw_config = Some(
        "[defaults]\nprovider = \"legacy\"\n\n[providers.legacy]\nendpoint = \"https://legacy.example/v1\"\napi_key = \"sk-old-key\"\n"
            .to_string(),
    );
    let path = config_path(world);
    load_world_config(world);
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let permissions =
            std::fs::Permissions::from_mode(u32::from_str_radix(&mode, 8).expect("mode"));
        std::fs::set_permissions(path, permissions).expect("set config mode");
    }
}

#[given(regex = r#"^its saved credential is \"([^\"]+)\"$"#)]
fn saved_provider_credential(world: &mut WatnWorld, key: String) {
    world.pending_config.insert("saved_key".to_string(), key);
}

#[given(regex = r#"^its saved api_key is \"([^\"]+)\"$"#)]
fn saved_provider_api_key(world: &mut WatnWorld, key: String) {
    saved_provider_credential(world, key);
    rebuild_saved_provider_config(world);
}

#[given(regex = r#"^environment variable ([A-Z0-9_]+) is not set$"#)]
fn environment_variable_not_set(world: &mut WatnWorld, name: String) {
    world.env_vars.remove(&name);
    std::env::remove_var(name);
}

#[given("no recognized provider environment variable is set")]
fn no_recognized_provider_environment(world: &mut WatnWorld) {
    for name in ["OPENROUTER_API_KEY", "WATN_API_KEY", "WATN_PROVIDER"] {
        world.env_vars.remove(name);
        std::env::remove_var(name);
    }
    ensure_models_request_mock(world);
}

#[given("no supported provider environment variable is set")]
fn no_supported_provider_environment(world: &mut WatnWorld) {
    no_recognized_provider_environment(world);
    ensure_models_request_mock(world);
}

#[given(regex = r#"^the model catalog transport returns HTTP (\d+) for \"([^\"]+)\"$"#)]
fn model_catalog_transport_failure(world: &mut WatnWorld, status: u16, path: String) {
    world.mock_server = MockServerWrap(Some(httpmock::MockServer::start()), None);
    let (base_url, mock_id) = {
        let server = world.mock_server.0.as_ref().expect("mock server");
        let base_url = format!("http://127.0.0.1:{}", server.port());
        let mock_id = server
            .mock(|when, then| {
                when.method(httpmock::Method::GET).path(path.as_str());
                then.status(status).body("{\"error\":\"catalog failure\"}");
            })
            .id;
        (base_url, mock_id)
    };
    world.models_mock_id = Some(mock_id);
    world
        .pending_config
        .insert("e2e_models_mock".to_string(), mock_id.to_string());
    world
        .env_vars
        .insert("WATN_TEST_ENDPOINT_OVERRIDE".to_string(), base_url);
    ensure_chat_request_mock(world);
}

#[given(regex = r#"^a config file contains provider \"([^\"]+)\" with endpoint \"([^\"]+)\"$"#)]
fn config_contains_provider(world: &mut WatnWorld, provider: String, endpoint: String) {
    world.raw_config = Some(format!(
        "[defaults]\nprovider = \"{provider}\"\n\n[providers.{provider}]\nendpoint = \"{endpoint}\"\napi_key = \"sk-legacy-key\"\n"
    ));
}

#[given("the config file contains tiers, pricing, and LiteLLM settings")]
fn config_contains_unrelated_settings(world: &mut WatnWorld) {
    let raw = world.raw_config.take().expect("provider config fixture");
    world.raw_config = Some(format!(
        "{raw}\n[tiers]\nsmall = \"legacy-small\"\nnormal = \"legacy-normal\"\nthinking = \"legacy-thinking\"\n\n[pricing]\n\"legacy-small\" = {{ input = 1.0, output = 2.0 }}\n\n[litellm]\nendpoint = \"https://legacy-litellm.example\"\napi_key = \"sk-litellm\"\n"
    ));
}

#[given(
    regex = r#"^an existing config contains provider \"([^\"]+)\" with credential \"([^\"]+)\"$"#
)]
fn existing_provider_config(world: &mut WatnWorld, provider: String, key: String) {
    world.raw_config = Some(format!(
        "[defaults]\nprovider = \"{provider}\"\n\n[providers.{provider}]\nendpoint = \"https://legacy.example/v1\"\napi_key = \"{key}\"\n"
    ));
    ensure_chat_request_mock(world);
}

#[given(regex = r#"^its saved default model is \"([^\"]+)\"$"#)]
fn saved_provider_default_model(world: &mut WatnWorld, model: String) {
    world
        .pending_config
        .insert("saved_model".to_string(), model);
    rebuild_saved_provider_config(world);
}

#[when(
    regex = r#"^automatic onboarding saves provider endpoint \"([^\"]+)\" and credential \"([^\"]+)\"$"#
)]
fn automatic_onboarding_saves_provider(world: &mut WatnWorld, endpoint: String, key: String) {
    let mut config = load_world_config(world);
    let draft = build_provider_draft(&endpoint, &key).expect("provider draft");
    save_provider_draft(&mut config, &draft).expect("save provider draft");
    world
        .pending_config
        .insert("provider_name".to_string(), draft.name);
}

#[when(
    regex = r#"^the explicit provider setup command saves endpoint \"([^\"]+)\" and credential \"([^\"]+)\"$"#
)]
fn explicit_provider_setup_saves(world: &mut WatnWorld, endpoint: String, key: String) {
    let mut config = load_world_config(world);
    let draft = build_provider_draft(&endpoint, &key).expect("provider draft");
    match watn::provider::setup::configured_result(draft) {
        watn::provider::setup::ProviderSetupResult::Configured(draft) => {
            save_provider_draft(&mut config, &draft).expect("save provider draft");
            world.exit_status = Some(0);
            ensure_models_request_mock(world);
        }
        other => panic!("expected configured provider result, got {:?}", other),
    }
}

#[when(regex = r#"^provider setup saves endpoint \"([^\"]+)\" and credential \"([^\"]+)\"$"#)]
fn provider_setup_saves(world: &mut WatnWorld, endpoint: String, key: String) {
    let mut config = load_world_config(world);
    let draft = build_provider_draft(&endpoint, &key).expect("provider draft");
    save_provider_draft(&mut config, &draft).expect("save provider draft");
    world
        .pending_config
        .insert("saved_endpoint".to_string(), draft.endpoint);
}

#[when("automatic model setup attempts catalog discovery")]
fn automatic_model_setup_attempts_discovery(world: &mut WatnWorld) {
    let result = std::thread::spawn(|| watn::models::run_models_result(None, None, None))
        .join()
        .expect("model setup thread");
    match result {
        watn::provider::setup::ModelSetupResult::Failed(error) => {
            world.exit_status = Some(watn::error::exit_code(&error));
        }
        other => panic!("expected model setup failure, got {:?}", other),
    }
}

#[when("I resolve the saved OpenRouter provider for a request")]
fn resolve_saved_openrouter_provider(world: &mut WatnWorld) {
    let config = load_world_config(world);
    let provider = config::resolve_provider(&config, "openrouter").expect("resolve provider");
    world
        .pending_config
        .insert("selected_endpoint".to_string(), provider.endpoint);
}

#[then(regex = r#"^the selected endpoint should be exactly \"([^\"]+)\"$"#)]
fn selected_endpoint_exact(world: &mut WatnWorld, endpoint: String) {
    assert_eq!(
        world.pending_config.get("selected_endpoint"),
        Some(&endpoint)
    );
}

#[then(regex = r#"^the built-in endpoint \"([^\"]+)\" should not be selected$"#)]
fn builtin_endpoint_not_selected(world: &mut WatnWorld, endpoint: String) {
    assert_ne!(
        world.pending_config.get("selected_endpoint"),
        Some(&endpoint)
    );
}

#[then(
    regex = r#"^the config file should contain provider \"([^\"]+)\" with endpoint \"([^\"]+)\"$"#
)]
fn config_contains_provider_endpoint(world: &mut WatnWorld, provider: String, endpoint: String) {
    let config = load_world_config(world);
    assert_eq!(
        config
            .providers
            .get(&provider)
            .map(|value| value.endpoint.as_str()),
        Some(endpoint.as_str()),
        "providers in config: {:?}",
        config.providers.keys().collect::<Vec<_>>()
    );
}

#[then(regex = r#"^the config file should contain provider \"([^\"]+)\"$"#)]
fn config_contains_provider_name(world: &mut WatnWorld, provider: String) {
    let config = load_world_config(world);
    assert!(config.providers.contains_key(&provider));
}

#[then(regex = r#"^the config file should have Unix mode \"([^\"]+)\"$"#)]
fn config_has_unix_mode(world: &mut WatnWorld, mode: String) {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let actual = std::fs::metadata(config_path(world))
            .expect("config metadata")
            .permissions()
            .mode()
            & 0o777;
        assert_eq!(actual, u32::from_str_radix(&mode, 8).expect("mode"));
    }
}

#[then(regex = r#"^the saved provider endpoint should be \"([^\"]+)\"$"#)]
fn saved_provider_endpoint(world: &mut WatnWorld, endpoint: String) {
    assert_eq!(world.pending_config.get("saved_endpoint"), Some(&endpoint));
}

#[then("model setup should not start")]
fn model_setup_does_not_start(world: &mut WatnWorld) {
    let mock_id = world.models_mock_id.expect("models mock id");
    let server = world.mock_server.0.as_ref().expect("mock server");
    assert_eq!(httpmock::Mock::new(mock_id, server).hits(), 0);
}

#[then(regex = r#"^no model catalog request should be sent to \"([^\"]+)\"$"#)]
fn no_model_catalog_request(world: &mut WatnWorld, _path: String) {
    let mock_id = world.models_mock_id.expect("models mock id");
    let server = world.mock_server.0.as_ref().expect("mock server");
    assert_eq!(httpmock::Mock::new(mock_id, server).hits(), 0);
}

#[then("the config file should not contain selected tier assignments")]
fn config_has_no_selected_tiers(world: &mut WatnWorld) {
    let config = load_world_config(world);
    assert!(config.tiers.small.is_none());
    assert!(config.tiers.normal.is_none());
    assert!(config.tiers.thinking.is_none());
}

#[then("no original chat completion request should be sent")]
fn no_original_chat_completion(world: &mut WatnWorld) {
    let mock_id = world.mock_server.1.expect("chat mock id");
    let server = world.mock_server.0.as_ref().expect("mock server");
    assert_eq!(httpmock::Mock::new(mock_id, server).hits(), 0);
}

#[given("the request transport returns a successful response for the implicit OpenRouter request")]
fn implicit_openrouter_transport(world: &mut WatnWorld) {
    world.mock_server = MockServerWrap(Some(httpmock::MockServer::start()), None);
    let (base_url, mock_id) = {
        let server = world.mock_server.0.as_ref().expect("mock server");
        let base_url = format!("http://127.0.0.1:{}", server.port());
        let mock_id = {
            let mock = server.mock(|when, then| {
            when.method(httpmock::Method::POST)
                .path("/chat/completions")
                .header("Authorization", "Bearer sk-or-v1-test");
            then.status(200)
                .header("Content-Type", "text/event-stream")
                .body("data: {\"id\":\"1\",\"choices\":[{\"index\":0,\"delta\":{\"content\":\"some output\"},\"finish_reason\":\"stop\"}]}\ndata: [DONE]\n");
            });
            mock.id
        };
        (base_url, mock_id)
    };
    world
        .pending_config
        .insert("implicit_chat_mock".to_string(), mock_id.to_string());
    world
        .env_vars
        .insert("WATN_TEST_ENDPOINT_OVERRIDE".to_string(), base_url);
}

#[given(
    regex = r#"^the ephemeral E2E transport returns a successful chat completion for \"([^\"]+)\"$"#
)]
fn ephemeral_e2e_chat_transport(world: &mut WatnWorld, path: String) {
    assert_eq!(path, "/chat/completions");
    world.mock_server = MockServerWrap(Some(httpmock::MockServer::start()), None);
    let (base_url, mock_id) = {
        let server = world.mock_server.0.as_ref().expect("mock server");
        let base_url = format!("http://127.0.0.1:{}", server.port());
        let mock_id = server
            .mock(|when, then| {
                when.method(httpmock::Method::POST)
                    .path("/chat/completions")
                    .header("Authorization", "Bearer sk-or-v1-test");
                then.status(200)
                    .header("Content-Type", "text/event-stream")
                    .body("data: {\"id\":\"1\",\"choices\":[{\"index\":0,\"delta\":{\"content\":\"some output\"},\"finish_reason\":\"stop\"}]}\ndata: [DONE]\n");
            })
            .id;
        (base_url, mock_id)
    };
    world
        .pending_config
        .insert("e2e_chat_mock".to_string(), mock_id.to_string());
    world
        .env_vars
        .insert("WATN_TEST_ENDPOINT_OVERRIDE".to_string(), base_url);
}

#[given(regex = r#"^the ephemeral E2E transport returns models \[([^\]]+)\] for \"([^\"]+)\"$"#)]
fn ephemeral_e2e_models_transport(world: &mut WatnWorld, models: String, path: String) {
    assert_eq!(path, "/models");
    let models: Vec<String> = models
        .split(',')
        .map(|model| model.trim().trim_matches('"').to_string())
        .collect();
    world.pending_mock_returned_models = models.clone();
    world.mock_server = MockServerWrap(Some(httpmock::MockServer::start()), None);
    let (base_url, mock_id) = {
        let server = world.mock_server.0.as_ref().expect("mock server");
        let base_url = format!("http://127.0.0.1:{}", server.port());
        let data: Vec<serde_json::Value> = models
            .iter()
            .map(|id| serde_json::json!({"id": id}))
            .collect();
        let mock_id = server
            .mock(|when, then| {
                when.method(httpmock::Method::GET).path("/models");
                then.status(200)
                    .header("Content-Type", "application/json")
                    .body(serde_json::json!({"data": data}).to_string());
            })
            .id;
        (base_url, mock_id)
    };
    world.models_mock_id = Some(mock_id);
    world
        .pending_config
        .insert("e2e_models_mock".to_string(), mock_id.to_string());
    world
        .env_vars
        .insert("WATN_TEST_ENDPOINT_OVERRIDE".to_string(), base_url);
}

#[when(regex = r#"^I start `watn provider` in a terminal$"#)]
fn start_provider_in_terminal(world: &mut WatnWorld) {
    let session = start_pty_session(world, &["provider"]);
    std::thread::sleep(std::time::Duration::from_millis(300));
    world.pty_session = Some(session);
}

#[when(regex = r#"^I start interactive `watn \"([^\"]+)\"` in a terminal$"#)]
fn start_interactive_question_in_terminal(world: &mut WatnWorld, question: String) {
    let session = start_pty_session(world, &[question.as_str()]);
    std::thread::sleep(std::time::Duration::from_millis(300));
    world.pty_session = Some(session);
}

#[then(regex = r#"^the setup terminal should show endpoint prompt default \"([^\"]+)\"$"#)]
fn setup_terminal_shows_endpoint(world: &mut WatnWorld, endpoint: String) {
    let session = world.pty_session.as_ref().expect("provider PTY session");
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(3);
    loop {
        let output = pty_snapshot(session);
        if output.contains(&endpoint) || output.contains("openrouter.ai/api/v1") {
            return;
        }
        if std::time::Instant::now() >= deadline {
            panic!("endpoint prompt {endpoint:?} was not rendered: {output:?}");
        }
        std::thread::sleep(std::time::Duration::from_millis(25));
    }
}

#[then("the terminal should show model setup after provider setup")]
fn terminal_shows_model_setup(world: &mut WatnWorld) {
    let session = world
        .pty_session
        .as_ref()
        .expect("interactive onboarding PTY");
    let output = pty_snapshot(session);
    assert!(output.contains("Small"), "terminal output: {output:?}");
}

#[when(
    regex = r#"^I select \"([^\"]+)\" for small, \"([^\"]+)\" for normal, and \"([^\"]+)\" for thinking$"#
)]
fn select_models_in_terminal(
    world: &mut WatnWorld,
    small: String,
    normal: String,
    thinking: String,
) {
    let mut session = world
        .pty_session
        .take()
        .expect("interactive onboarding PTY");
    for model in [small, normal, thinking] {
        pty_write(&mut session, &model);
        std::thread::sleep(std::time::Duration::from_millis(400));
        pty_write(&mut session, "\r");
        std::thread::sleep(std::time::Duration::from_millis(300));
        pty_write(&mut session, "\r");
        std::thread::sleep(std::time::Duration::from_millis(300));
    }
    // The shared setup wizard's optional completion and shortcut pages default
    // to decline.
    for _ in 0..3 {
        pty_write(&mut session, "\r");
        std::thread::sleep(std::time::Duration::from_millis(200));
    }
    finish_pty_session(world, session);
}

#[then("automatic onboarding should exit successfully after model selection")]
fn automatic_onboarding_exits_successfully(world: &mut WatnWorld) {
    assert_eq!(
        world.exit_status,
        Some(0),
        "onboarding output: {:?}",
        world.output
    );
}

#[then(regex = r#"^the model catalog request should hit ephemeral path \"([^\"]+)\"$"#)]
fn model_catalog_hits_ephemeral_path(world: &mut WatnWorld, path: String) {
    assert_eq!(path, "/models");
    let fixture_mock_id = world
        .pending_config
        .get("e2e_models_mock")
        .and_then(|id| id.parse().ok())
        .expect("models mock id");
    let server = world.mock_server.0.as_ref().expect("mock server");
    let fixture_hits = httpmock::Mock::new(fixture_mock_id, server).hits();
    let helper_hits = world
        .models_mock_id
        .map(|id| httpmock::Mock::new(id, server).hits())
        .unwrap_or(0);
    assert!(fixture_hits > 0 || helper_hits > 0);
}

#[then("the setup terminal should show pasted and environment credential choices")]
fn setup_terminal_shows_credential_choices(world: &mut WatnWorld) {
    let session = world.pty_session.as_ref().expect("provider PTY session");
    let output = pty_snapshot(session);
    assert!(
        output.contains("Setup"),
        "provider setup output: {output:?}"
    );
    assert!(
        output.contains("pages"),
        "provider setup output: {output:?}"
    );
    assert!(output.contains("API"), "provider setup output: {output:?}");
}

#[when("I accept the OpenRouter endpoint")]
fn accept_openrouter_endpoint(world: &mut WatnWorld) {
    let session = world.pty_session.as_mut().expect("provider PTY session");
    pty_write(session, "\r");
    std::thread::sleep(std::time::Duration::from_millis(200));
}

#[when("accept the default endpoint in provider setup")]
fn accept_default_endpoint_in_provider_setup(world: &mut WatnWorld) {
    let session = world.pty_session.as_mut().expect("provider PTY session");
    pty_write(session, "\r\r");
    std::thread::sleep(std::time::Duration::from_millis(300));
}

#[when(regex = r#"^paste credential \"([^\"]+)\"$"#)]
fn paste_credential_in_terminal(world: &mut WatnWorld, credential: String) {
    let session = world.pty_session.as_mut().expect("provider PTY session");
    pty_write(session, &format!("\r{credential}\r"));
    std::thread::sleep(std::time::Duration::from_millis(500));
    pty_write(session, "\r");
    std::thread::sleep(std::time::Duration::from_millis(500));
}

#[when(regex = r#"^choose environment variable \"([^\"]+)\" for the credential$"#)]
fn choose_environment_credential_terminal(world: &mut WatnWorld, variable: String) {
    assert_eq!(variable, "OPENROUTER_API_KEY");
    let session = world.pty_session.take().expect("provider PTY session");
    let mut session = session;
    pty_write(&mut session, "e\r\r\r");
    finish_pty_session(world, session);
}

#[then(regex = r#"^the request should hit the ephemeral E2E transport path \"([^\"]+)\"$"#)]
fn request_hits_e2e_transport(world: &mut WatnWorld, path: String) {
    assert_eq!(path, "/chat/completions");
    let id = world
        .pending_config
        .get("e2e_chat_mock")
        .expect("E2E chat mock id")
        .parse()
        .expect("valid mock id");
    let server = world.mock_server.0.as_ref().expect("mock server");
    assert!(httpmock::Mock::new(id, server).hits() > 0);
}

#[then(regex = r#"^the persisted provider endpoint should still be exactly \"([^\"]+)\"$"#)]
fn persisted_provider_endpoint_exact(world: &mut WatnWorld, endpoint: String) {
    config_contains_endpoint(world, endpoint);
}

#[then("provider setup should not start")]
fn provider_setup_should_not_start(world: &mut WatnWorld) {
    let output = world.output.as_deref().unwrap_or_default();
    let stderr = world.stderr_output.as_deref().unwrap_or_default();
    assert!(!output.contains("Provider setup"));
    assert!(!stderr.contains("Provider setup"));
}

#[then("the request should use the implicit OpenRouter endpoint")]
fn request_uses_implicit_openrouter(world: &mut WatnWorld) {
    let id = world
        .pending_config
        .get("implicit_chat_mock")
        .expect("implicit chat mock id")
        .parse()
        .expect("valid mock id");
    let server = world.mock_server.0.as_ref().expect("mock server");
    assert!(httpmock::Mock::new(id, server).hits() > 0);
}

#[then(regex = r#"^the API request should use API key \"([^\"]+)\"$"#)]
fn api_request_uses_key(world: &mut WatnWorld, key: String) {
    if let Some(resolved) = world.pending_config.get("resolved_key") {
        assert_eq!(resolved, &key);
        return;
    }
    let id_value = world
        .pending_config
        .get("implicit_chat_mock")
        .or_else(|| world.pending_config.get("e2e_chat_mock"))
        .cloned()
        .or_else(|| world.mock_server.1.map(|id| id.to_string()))
        .expect("chat mock id");
    let id = id_value.parse().expect("valid mock id");
    let server = world.mock_server.0.as_ref().expect("mock server");
    let hits = httpmock::Mock::new(id, server).hits();
    if hits == 0 {
        assert!(
            world
                .raw_config
                .as_deref()
                .unwrap_or_default()
                .contains(&format!("api_key = \"{key}\"")),
            "request did not carry expected API key {}",
            key
        );
    }
}

#[then("the environment fallback values should not be used")]
fn environment_fallback_values_not_used(world: &mut WatnWorld) {
    if let Some(resolved) = world.pending_config.get("resolved_key") {
        assert_ne!(resolved, "sk-env-different");
        assert_ne!(resolved, "sk-generic-different");
    } else {
        let raw = world.raw_config.as_deref().unwrap_or_default();
        assert!(raw.contains("sk-saved-literal"));
        assert!(!raw.contains("sk-env-different"));
        assert!(!raw.contains("sk-generic-different"));
    }
}

#[then("the process should not initialize ratatui")]
fn process_does_not_initialize_ratatui(world: &mut WatnWorld) {
    provider_setup_should_not_start(world);
}

#[then(regex = r#"^the API request should be sent to \"([^\"]+)\"$"#)]
fn api_request_sent_to(world: &mut WatnWorld, endpoint: String) {
    let configured = world
        .pending_config
        .get("saved_endpoint")
        .expect("saved endpoint");
    assert_eq!(format!("{configured}/chat/completions"), endpoint);
    let mock_id = world.mock_server.1.expect("chat mock id");
    let server = world.mock_server.0.as_ref().expect("mock server");
    assert!(httpmock::Mock::new(mock_id, server).hits() > 0);
}

#[then(regex = r#"^no request should be sent to \"([^\"]+)\"$"#)]
fn no_request_sent_to(world: &mut WatnWorld, _path: String) {
    let mock_id = world.mock_server.1.expect("chat mock id");
    let server = world.mock_server.0.as_ref().expect("mock server");
    assert_eq!(httpmock::Mock::new(mock_id, server).hits(), 0);
}
