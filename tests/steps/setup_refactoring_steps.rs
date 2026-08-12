//! Strict steps for the non-interactive setup-refactoring contracts.

use cucumber::{given, then, when};
use std::path::PathBuf;

use super::{build_config, find_binary, run_binary_with_state};
use crate::WatnWorld;
use watn::config::types::{Config, ProviderConfig};
use watn::provider::setup::{
    build_provider_draft, build_provider_draft_for_identity, ProviderIdentity,
};
use watn::shell_shortcut::Shell;

#[then("stdout should be empty")]
fn stdout_empty(world: &mut WatnWorld) {
    assert_eq!(world.output.as_deref().unwrap_or_default(), "");
}

#[then("no config file should exist")]
fn no_config_file_should_exist(world: &mut WatnWorld) {
    let directory = world.temp_dir.as_ref().expect("isolated config directory");
    assert!(!directory.path().join("watn/config.toml").exists());
}

#[given("a complete configuration exists")]
fn complete_configuration_exists(world: &mut WatnWorld) {
    world.raw_config = Some(build_config(
        "test",
        Some(("small-model", "normal-model", "thinking-model")),
        Some(vec![(
            "test",
            "http://localhost:4000",
            "test-key",
            "small-model",
        )]),
        None,
        None,
        None,
    ));
    world.pending_mock_model = Some("small-model".to_string());
    world.pending_mock_output = Some("printf setup-test".to_string());
}

#[when("I run `watn provider`")]
fn run_removed_provider_command(world: &mut WatnWorld) {
    run_binary_with_state(world, &["provider"], None);
}

#[when("I run `watn --model alternate-model \"show changed files\"`")]
fn run_removed_model_option(world: &mut WatnWorld) {
    run_binary_with_state(
        world,
        &["--model", "alternate-model", "show changed files"],
        None,
    );
}

#[then("the command should be rejected as unavailable")]
fn command_rejected_as_unavailable(world: &mut WatnWorld) {
    assert_ne!(world.exit_status, Some(0));
    assert!(world
        .stderr_output
        .as_deref()
        .unwrap_or_default()
        .contains("removed setup command"));
}

#[then("the command should reject the removed provider option")]
fn removed_provider_option_rejected(world: &mut WatnWorld) {
    assert_ne!(world.exit_status, Some(0));
    assert!(world
        .stderr_output
        .as_deref()
        .unwrap_or_default()
        .contains("--provider"));
}

#[then("the command should reject the removed model option")]
fn removed_model_option_rejected(world: &mut WatnWorld) {
    assert_ne!(world.exit_status, Some(0));
    assert!(world
        .stderr_output
        .as_deref()
        .unwrap_or_default()
        .contains("--model"));
}

#[when("I run `watn --set-small alternate-model`")]
fn run_removed_assignment_option(world: &mut WatnWorld) {
    run_binary_with_state(world, &["--set-small", "alternate-model"], None);
}

#[then("the command should reject the removed model-assignment option")]
fn removed_model_assignment_rejected(world: &mut WatnWorld) {
    assert_ne!(world.exit_status, Some(0));
    assert!(world
        .stderr_output
        .as_deref()
        .unwrap_or_default()
        .contains("--set-small"));
}

#[then("generated shell completions should not advertise removed setup commands or options")]
fn completions_omit_removed_surface(_world: &mut WatnWorld) {
    let binary = find_binary();
    let output = std::process::Command::new(binary)
        .args(["completions", "bash"])
        .output()
        .expect("generate completions");
    let text = String::from_utf8_lossy(&output.stdout);
    for removed in [
        "--provider",
        "--model",
        "--set-small",
        " models ",
        " provider ",
    ] {
        assert!(!text.contains(removed), "completion advertised {removed}");
    }
}

#[then("`watn -1`, `watn -2`, and `watn -3` should remain valid request tier selectors")]
fn tier_selectors_remain_valid(world: &mut WatnWorld) {
    for tier in ["-1", "-2", "-3"] {
        run_binary_with_state(world, &[tier, "show changed files"], None);
        assert_ne!(
            world.exit_status,
            Some(2),
            "tier {tier} was rejected by clap"
        );
    }
}

#[given("a complete persisted configuration exists")]
fn complete_persisted_configuration_exists(world: &mut WatnWorld) {
    complete_configuration_exists(world);
}

#[when("I run a request with the complete persisted configuration")]
fn run_request_with_persisted_configuration(world: &mut WatnWorld) {
    run_binary_with_state(world, &["show changed files"], None);
}

#[then("the persisted provider and model roles should remain the request selection")]
fn persisted_selection_remains(world: &mut WatnWorld) {
    let directory = world.temp_dir.as_ref().expect("config directory");
    let content = std::fs::read_to_string(directory.path().join("watn/config.toml"))
        .expect("persisted config");
    assert!(content.contains("provider = \"test\""));
    assert!(content.contains("small = \"small-model\""));
    assert!(content.contains("normal = \"normal-model\""));
    assert!(content.contains("thinking = \"thinking-model\""));
}

fn world_config_path(world: &mut WatnWorld) -> PathBuf {
    let directory = if let Some(directory) = &world.temp_dir {
        directory.path().to_path_buf()
    } else {
        let directory = tempfile::tempdir().expect("isolated setup directory");
        let path = directory.path().to_path_buf();
        world.temp_dir = Some(directory);
        path
    };
    std::fs::create_dir_all(directory.join("watn")).expect("setup config directory");
    let xdg = directory.to_string_lossy().to_string();
    world
        .env_vars
        .insert("XDG_CONFIG_HOME".to_string(), xdg.clone());
    std::env::set_var("XDG_CONFIG_HOME", &xdg);
    for (name, value) in &world.env_vars {
        std::env::set_var(name, value);
    }
    directory.join("watn/config.toml")
}

fn write_world_config(world: &mut WatnWorld) -> (PathBuf, Config) {
    let raw = world.raw_config.clone().expect("setup config fixture");
    let path = world_config_path(world);
    std::fs::write(&path, &raw).expect("write setup config fixture");
    let config = toml::from_str(&raw).expect("parse setup config fixture");
    (path, config)
}

fn reviewed_setup_result(
    config: Config,
    provider: watn::provider::setup::ProviderDraft,
    completion_shells: Vec<Shell>,
) -> watn::setup::SetupWizardResult {
    watn::setup::SetupWizardResult {
        config,
        provider,
        choices: std::array::from_fn(|_| None),
        completion_shells,
        shortcut_shells: Vec::new(),
        completion_remove_shells: Vec::new(),
        shortcut_remove_shells: Vec::new(),
        completion_attention_shells: Vec::new(),
        shortcut_attention_shells: Vec::new(),
        first_run: false,
        catalog_warning: None,
    }
}

fn apply_reviewed_setup_result(world: &mut WatnWorld) {
    let result = world.setup_result.take().expect("reviewed setup draft");
    let mut config = result.config.clone();
    match watn::setup::apply_result(&mut config, &result) {
        Ok(()) => {
            world.exit_status = Some(0);
            world.stderr_output = Some(String::new());
        }
        Err(error) => {
            world.exit_status = Some(watn::error::exit_code(&error));
            world.stderr_output = Some(error.to_string());
        }
    }
    world.output = Some(String::new());
}

#[given("a valid reviewed setup draft selects shell integrations")]
fn valid_reviewed_shell_draft(world: &mut WatnWorld) {
    let root = tempfile::tempdir().expect("shell setup fixture");
    std::fs::write(root.path().join(".bashrc"), "user bash content\n")
        .expect("bash startup fixture");
    world.env_vars.insert(
        "HOME".to_string(),
        root.path().to_string_lossy().to_string(),
    );
    world.env_vars.insert(
        "XDG_CONFIG_HOME".to_string(),
        root.path().to_string_lossy().to_string(),
    );
    world
        .env_vars
        .insert("SHELL".to_string(), "/bin/bash".to_string());
    world.temp_dir = Some(root);
    let _ = world_config_path(world);

    let endpoint = "https://custom.example/v1";
    let provider = build_provider_draft(endpoint, "sk-reviewed-key").expect("provider draft");
    let mut config = Config::default();
    config.defaults.provider = Some("custom".to_string());
    config.tiers.small = Some("small-model".to_string());
    config.tiers.normal = Some("normal-model".to_string());
    config.tiers.thinking = Some("thinking-model".to_string());
    config.providers.insert(
        "custom".to_string(),
        ProviderConfig {
            endpoint: endpoint.to_string(),
            api_key: Some("sk-reviewed-key".to_string()),
            default_model: None,
        },
    );
    world.setup_result = Some(reviewed_setup_result(
        config,
        provider,
        vec![Shell::Bash, Shell::Zsh],
    ));
}

#[given("one selected shell startup file cannot be reconciled")]
fn selected_shell_target_cannot_reconcile(world: &mut WatnWorld) {
    let root = world.temp_dir.as_ref().expect("shell setup fixture");
    std::fs::create_dir(root.path().join(".zshrc")).expect("unwritable zsh fixture");
}

#[when("I apply the reviewed setup draft")]
fn apply_reviewed_setup_draft(world: &mut WatnWorld) {
    apply_reviewed_setup_result(world);
}

#[then("the supported configuration changes should be saved")]
fn supported_configuration_saved(world: &mut WatnWorld) {
    let path = world_config_path(world);
    let content = std::fs::read_to_string(path).expect("saved setup config");
    assert!(content.contains("provider = \"custom\""));
    assert!(content.contains("small = \"small-model\""));
}

#[then("successful shell changes should remain applied")]
fn successful_shell_changes_remain(world: &mut WatnWorld) {
    let root = world.temp_dir.as_ref().expect("shell setup fixture");
    let content = std::fs::read_to_string(root.path().join(".bashrc")).expect("bash startup");
    assert!(content.contains(watn::shell_completion::OPEN_MARKER));
    assert!(content.contains("user bash content"));
}

#[then("stderr should identify the failed shell integration and retry guidance")]
fn shell_failure_retry_guidance(world: &mut WatnWorld) {
    let stderr = world.stderr_output.as_deref().expect("setup stderr");
    assert!(
        stderr.contains("Zsh"),
        "stderr did not identify Zsh: {stderr}"
    );
    assert!(
        stderr.contains("retry setup"),
        "stderr lacked retry guidance: {stderr}"
    );
}

#[when("I choose the OpenAI provider in `watn setup`")]
fn choose_openai_provider_draft(world: &mut WatnWorld) {
    let key = world
        .env_vars
        .get("OPENAI_API_KEY")
        .expect("OpenAI environment credential")
        .clone();
    let path = world_config_path(world);
    let draft = build_provider_draft_for_identity(
        ProviderIdentity::OpenAi,
        "https://api.openai.com/v1",
        "${OPENAI_API_KEY}",
    )
    .expect("OpenAI provider draft");
    let mut config = Config::default();
    watn::config::save_provider_draft(&mut config, &draft).expect("save OpenAI setup draft");
    assert_eq!(draft.api_key, "${OPENAI_API_KEY}");
    assert!(!std::fs::read_to_string(path).unwrap().contains(&key));
    world
        .pending_config
        .insert("openai_endpoint".to_string(), draft.endpoint);
    world
        .pending_config
        .insert("openai_credential".to_string(), draft.api_key);
}

#[then(regex = r##"^the OpenAI setup draft should show endpoint \"([^\"]+)\"$"##)]
fn openai_draft_endpoint(world: &mut WatnWorld, endpoint: String) {
    assert_eq!(world.pending_config.get("openai_endpoint"), Some(&endpoint));
}

#[then(
    regex = r##"^the OpenAI setup draft should identify \"([^\"]+)\" as \"Detected from environment\"$"##
)]
fn openai_draft_credential(world: &mut WatnWorld, variable: String) {
    let candidates = watn::config::env::discover_credentials("openai");
    assert!(candidates
        .iter()
        .any(|candidate| { candidate.name == variable && candidate.detected }));
    assert_eq!(
        world.pending_config.get("openai_credential"),
        Some(&format!("${{{variable}}}"))
    );
}

#[when("I finish an otherwise unchanged setup draft")]
fn finish_unchanged_setup_draft(world: &mut WatnWorld) {
    let (path, config) = write_world_config(world);
    let provider = config.providers.get("custom").expect("custom provider");
    let draft = build_provider_draft(&provider.endpoint, provider.api_key.as_deref().unwrap())
        .expect("unchanged provider draft");
    world.setup_result = Some(reviewed_setup_result(config, draft, Vec::new()));
    apply_reviewed_setup_result(world);
    assert!(path.exists());
}

#[then("the existing default model, provider default model, pricing, and LiteLLM settings should remain unchanged")]
fn unchanged_supported_settings(world: &mut WatnWorld) {
    let path = world_config_path(world);
    let config: Config = toml::from_str(&std::fs::read_to_string(path).expect("saved config"))
        .expect("parse saved config");
    assert_eq!(config.defaults.model.as_deref(), Some("legacy-default"));
    assert_eq!(
        config.providers["custom"].default_model.as_deref(),
        Some("legacy-provider-default")
    );
    assert_eq!(config.pricing["legacy-small"].input, 1.0);
    assert_eq!(
        config.litellm.as_ref().map(|value| value.endpoint.as_str()),
        Some("https://legacy-litellm.example")
    );
}

#[then("the config should contain no origin or shell integration fields")]
fn config_has_no_origin_or_shell_fields(world: &mut WatnWorld) {
    let path = world_config_path(world);
    let content = std::fs::read_to_string(path).expect("saved config");
    assert!(!content.contains("origin"));
    assert!(!content.contains("shell integration"));
    assert!(!content.contains("completion_shell"));
    assert!(!content.contains("shortcut_shell"));
}

#[given("an existing config has a custom chat provider and a configured LiteLLM catalog source")]
fn existing_custom_chat_with_litellm_source(world: &mut WatnWorld) {
    let root = tempfile::tempdir().expect("LiteLLM setup fixture");
    let server = httpmock::MockServer::start();
    let base = format!("http://127.0.0.1:{}", server.port());
    let provider_probe = server
        .mock(|when, then| {
            when.method(httpmock::Method::GET).path("/chat/models");
            then.status(500);
        })
        .id;
    world.mock_server = crate::MockServerWrap(Some(server), None);
    world.models_mock_id = None;
    world.pending_config.insert(
        "provider_catalog_probe".to_string(),
        provider_probe.to_string(),
    );
    world.raw_config = Some(format!(
        "[defaults]\nprovider = \"custom\"\n\n[providers.custom]\nendpoint = \"{base}/chat\"\napi_key = \"sk-chat-key\"\n\n[litellm]\nendpoint = \"{base}\"\n"
    ));
    world.env_vars.insert(
        "XDG_CONFIG_HOME".to_string(),
        root.path().to_string_lossy().to_string(),
    );
    world.temp_dir = Some(root);
}

#[when("setup catalog discovery resolves the configured source")]
fn resolve_setup_catalog_source(world: &mut WatnWorld) {
    let raw = world.raw_config.as_ref().expect("LiteLLM config");
    let config: Config = toml::from_str(raw).expect("LiteLLM config parse");
    let provider = config.providers.get("custom").expect("chat provider");
    let (endpoint, key) = watn::setup::resolve_catalog_source(
        &config,
        &provider.endpoint,
        provider.api_key.as_deref(),
    )
    .expect("resolve catalog source");
    let request_endpoint = endpoint.clone();
    let request_key = key.clone();
    let models = std::thread::spawn(move || {
        watn::models::list::fetch_models(&request_endpoint, request_key.as_deref())
            .expect("fetch catalog")
    })
    .join()
    .expect("catalog request thread");
    assert!(!models.is_empty());
    world
        .pending_config
        .insert("catalog_source".to_string(), endpoint);
    world
        .pending_config
        .insert("chat_source".to_string(), provider.endpoint.clone());
}

#[then("model discovery should request the configured LiteLLM endpoint")]
fn setup_litellm_request(world: &mut WatnWorld) {
    let server = world.mock_server.0.as_ref().expect("catalog server");
    let mock_id = world.models_mock_id.expect("LiteLLM catalog mock");
    assert_eq!(httpmock::Mock::new(mock_id, server).hits(), 1);
    assert_eq!(
        world.pending_config.get("catalog_source"),
        Some(&format!("http://127.0.0.1:{}", server.port()))
    );
}

#[then("the custom chat provider should receive no model catalog request")]
fn custom_chat_receives_no_catalog_request(world: &mut WatnWorld) {
    let server = world.mock_server.0.as_ref().expect("catalog server");
    let mock_id: usize = world.pending_config["provider_catalog_probe"]
        .parse()
        .expect("provider mock id");
    assert_eq!(httpmock::Mock::new(mock_id, server).hits(), 0);
}

#[then("setup should identify the catalog source separately from the chat provider")]
fn setup_catalog_source_is_separate(world: &mut WatnWorld) {
    assert_ne!(
        world.pending_config.get("catalog_source"),
        world.pending_config.get("chat_source")
    );
}
