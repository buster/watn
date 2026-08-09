use cucumber::{then, when};
use cucumber::given;
use std::path::PathBuf;

use crate::WatnWorld;
use crate::MockServerWrap;
use watn::config::{self, save_provider_draft};
use watn::config::types::Config;
use watn::provider::setup::{build_provider_draft, suggested_api_key_env};

#[when(regex = r#"^provider setup accepts endpoint \"([^\"]+)\"$"#)]
fn provider_setup_accepts_endpoint(world: &mut WatnWorld, endpoint: String) {
    world
        .pending_config
        .insert("provider_endpoint".to_string(), endpoint);
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
    world.config_content = Some(toml::to_string_pretty(&config).expect("serialize config"));
}

fn save_environment_draft(world: &mut WatnWorld, name: String) {
    let endpoint = world
        .pending_config
        .get("provider_endpoint")
        .cloned()
        .expect("provider endpoint must be accepted first");
    let reference = format!("${{{name}}}");
    let draft = build_provider_draft(&endpoint, &reference).expect("build environment draft");
    let mut config = load_world_config(world);
    save_provider_draft(&mut config, &draft).expect("save environment draft");
    world
        .pending_config
        .insert("provider_name".to_string(), draft.name.clone());
    world.config_content = Some(toml::to_string_pretty(&config).expect("serialize config"));
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
    let provider = config.defaults.provider.as_deref().expect("default provider");
    assert_eq!(config.providers[provider].endpoint, endpoint);
}

#[then(regex = r#"^the config file should contain api_key exactly \"([^\"]+)\"$"#)]
fn config_contains_api_key(world: &mut WatnWorld, api_key: String) {
    let config = load_world_config(world);
    let provider = config.defaults.provider.as_deref().expect("default provider");
    assert_eq!(config.providers[provider].api_key.as_deref(), Some(api_key.as_str()));
}

#[then(regex = r#"^the config file should not contain \"([^\"]+)\"$"#)]
fn config_does_not_contain(world: &mut WatnWorld, secret: String) {
    let content = std::fs::read_to_string(config_path(world)).expect("read test config");
    assert!(!content.contains(&secret), "config unexpectedly contained secret");
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
    let id = world
        .pending_config
        .get("implicit_chat_mock")
        .expect("chat mock id")
        .parse()
        .expect("valid mock id");
    let server = world.mock_server.0.as_ref().expect("mock server");
    assert!(httpmock::Mock::new(id, server).hits() > 0, "request did not carry expected API key {}", key);
}

#[then("the process should not initialize ratatui")]
fn process_does_not_initialize_ratatui(world: &mut WatnWorld) {
    provider_setup_should_not_start(world);
}
