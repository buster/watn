use cucumber::{given, then, when};
use httpmock::{Method, MockServer};
use std::fmt;
use std::path::PathBuf;
use std::process::Command;

use watn::config::types::{Config, ProviderConfig};

use super::binary_from_env;
use crate::WatnWorld;

#[derive(Debug)]
struct Invocation {
    binary: PathBuf,
    status: Option<i32>,
    stdout: String,
    stderr: String,
}

#[derive(Default)]
pub struct TransportState {
    configured_server: Option<MockServer>,
    configured_mock_id: Option<usize>,
    configured_endpoint: Option<String>,
    configured_path: Option<String>,
    configured_response: Option<String>,
    competing_server: Option<MockServer>,
    competing_mock_id: Option<usize>,
    competing_endpoint: Option<String>,
    isolated_server: Option<MockServer>,
    isolated_mock_id: Option<usize>,
    isolated_endpoint: Option<String>,
    config: Option<Config>,
    config_path: Option<PathBuf>,
    api_key: Option<String>,
    invocations: Vec<Invocation>,
    readiness: Option<bool>,
}

impl fmt::Debug for TransportState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TransportState")
            .field("configured_endpoint", &self.configured_endpoint)
            .field("competing_endpoint", &self.competing_endpoint)
            .field("isolated_endpoint", &self.isolated_endpoint)
            .field("readiness", &self.readiness)
            .finish()
    }
}

fn register_chat_mock(server: &MockServer, path: &str, api_key: &str, response: &str) -> usize {
    let path = path.to_string();
    let authorization = format!("Bearer {api_key}");
    let content = serde_json::to_string(response).expect("serialize mock response");
    let body = format!(
        "data: {{\"id\":\"1\",\"model\":\"test-model\",\"choices\":[{{\"index\":0,\"delta\":{{\"content\":{content}}},\"finish_reason\":\"stop\"}}]}}\ndata: [DONE]\n"
    );
    server
        .mock(move |when, then| {
            let mut when = when.method(Method::POST);
            when = when.path(path);
            when.header("Authorization", authorization);
            then.status(200)
                .header("Content-Type", "text/event-stream")
                .body(body);
        })
        .id
}

fn endpoint_path(path: &str) -> String {
    path.strip_suffix("/chat/completions")
        .unwrap_or("/v1")
        .to_string()
}

fn configured_endpoint(world: &WatnWorld) -> &str {
    world
        .transport
        .configured_endpoint
        .as_deref()
        .expect("configured endpoint was not created")
}

fn configured_server(world: &WatnWorld) -> &MockServer {
    world
        .transport
        .configured_server
        .as_ref()
        .expect("configured provider twin was not created")
}

fn configured_mock_hits(world: &WatnWorld) -> usize {
    let mock_id = world
        .transport
        .configured_mock_id
        .expect("configured provider mock was not created");
    httpmock::Mock::new(mock_id, configured_server(world)).hits()
}

#[given(
    regex = r##"^a reachable local configured provider twin returns "([^"]+)" for POST "([^"]+)"$"##
)]
fn configured_provider_twin(world: &mut WatnWorld, response: String, path: String) {
    let server = MockServer::start();
    let endpoint = server.url(endpoint_path(&path));
    world.transport.configured_server = Some(server);
    world.transport.configured_endpoint = Some(endpoint);
    world.transport.configured_path = Some(path);
    world.transport.configured_response = Some(response);
}

#[given(regex = r##"^the configured provider has api key "([^"]+)" and default model "([^"]+)"$"##)]
fn configured_provider_credentials(world: &mut WatnWorld, api_key: String, model: String) {
    let endpoint = configured_endpoint(world).to_string();
    let path = world
        .transport
        .configured_path
        .as_deref()
        .expect("configured provider path was not created")
        .to_string();
    let response = world
        .transport
        .configured_response
        .as_deref()
        .expect("configured provider response was not created")
        .to_string();
    let mock_id = register_chat_mock(configured_server(world), &path, &api_key, &response);
    world.transport.configured_mock_id = Some(mock_id);
    world.transport.api_key = Some(api_key.clone());

    let mut config = Config::default();
    config.defaults.provider = Some("configured".to_string());
    config.providers.insert(
        "configured".to_string(),
        ProviderConfig {
            endpoint,
            api_key: Some(api_key),
            default_model: Some(model),
        },
    );
    let temp_dir = tempfile::tempdir().expect("create transport config directory");
    let config_dir = temp_dir.path().join("watn");
    std::fs::create_dir_all(&config_dir).expect("create watn config directory");
    let config_path = config_dir.join("config.toml");
    let content = toml::to_string_pretty(&config).expect("serialize transport config");
    std::fs::write(&config_path, content).expect("write transport config");
    world.env_vars.insert(
        "XDG_CONFIG_HOME".to_string(),
        temp_dir.path().display().to_string(),
    );
    world.temp_dir = Some(temp_dir);
    world.transport.config = Some(config);
    world.transport.config_path = Some(config_path);
}

#[given(
    regex = r##"^a separate reachable local competing provider twin returns "([^"]+)" for POST "([^"]+)"$"##
)]
fn competing_provider_twin(world: &mut WatnWorld, response: String, path: String) {
    let server = MockServer::start();
    let endpoint = server.url(endpoint_path(&path));
    let api_key = world
        .transport
        .api_key
        .as_deref()
        .expect("configured credentials were not created")
        .to_string();
    let mock_id = register_chat_mock(&server, &path, &api_key, &response);
    world.transport.competing_server = Some(server);
    world.transport.competing_endpoint = Some(endpoint);
    world.transport.competing_mock_id = Some(mock_id);
}

#[given(
    regex = r##"^a separate reachable local isolated provider twin returns "([^"]+)" for POST "([^"]+)"$"##
)]
fn isolated_provider_twin(world: &mut WatnWorld, response: String, path: String) {
    let server = MockServer::start();
    let endpoint = server.url(endpoint_path(&path));
    let api_key = world
        .transport
        .api_key
        .as_deref()
        .expect("configured credentials were not created")
        .to_string();
    let mock_id = register_chat_mock(&server, &path, &api_key, &response);
    world.transport.isolated_server = Some(server);
    world.transport.isolated_endpoint = Some(endpoint);
    world.transport.isolated_mock_id = Some(mock_id);
}

fn run_transport_binary(world: &mut WatnWorld, binary_env: &str, override_endpoint: Option<&str>) {
    let binary = binary_from_env(binary_env);
    let config_home = world
        .env_vars
        .get("XDG_CONFIG_HOME")
        .expect("transport config home was not created")
        .clone();
    let mut command = Command::new(&binary);
    command.arg("hello");
    for name in [
        "XDG_CONFIG_HOME",
        "WATN_PROVIDER",
        "WATN_MODEL",
        "WATN_OPENAI_API_KEY",
        "OPENROUTER_API_KEY",
        "WATN_API_KEY",
        "WATN_TEST_ENDPOINT_OVERRIDE",
    ] {
        command.env_remove(name);
    }
    command.env("XDG_CONFIG_HOME", config_home);
    if let Some(endpoint) = override_endpoint {
        command.env("WATN_TEST_ENDPOINT_OVERRIDE", endpoint);
    }
    let output = command.output().expect("run transport binary");
    world.transport.invocations.push(Invocation {
        binary,
        status: output.status.code(),
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
    });
}

#[when("I run the default-feature release binary and the test-support release binary with the override set to the competing twin")]
fn run_release_binaries(world: &mut WatnWorld) {
    let endpoint = world
        .transport
        .competing_endpoint
        .as_deref()
        .expect("competing provider twin was not created")
        .to_string();
    run_transport_binary(world, "WATN_DEFAULT_RELEASE_BIN", Some(&endpoint));
    run_transport_binary(world, "WATN_TEST_SUPPORT_RELEASE_BIN", Some(&endpoint));
}

#[when("I run the test-support debug binary with the override set to the isolated twin")]
fn run_isolated_debug_binary(world: &mut WatnWorld) {
    let endpoint = world
        .transport
        .isolated_endpoint
        .as_deref()
        .expect("isolated provider twin was not created")
        .to_string();
    run_transport_binary(world, "WATN_TEST_SUPPORT_DEBUG_BIN", Some(&endpoint));
}

#[when(regex = r##"^I run the test-support debug binary with the override state "([^"]+)"$"##)]
fn run_debug_binary_with_override_state(world: &mut WatnWorld, state: String) {
    match state.as_str() {
        "missing" => run_transport_binary(world, "WATN_TEST_SUPPORT_DEBUG_BIN", None),
        "whitespace" => run_transport_binary(world, "WATN_TEST_SUPPORT_DEBUG_BIN", Some("   ")),
        other => panic!("unknown transport override state: {other}"),
    }
}

#[then(regex = r##"^each binary should exit successfully with output containing "([^"]+)"$"##)]
fn each_binary_output(world: &mut WatnWorld, response: String) {
    assert_eq!(world.transport.invocations.len(), 2);
    for invocation in &world.transport.invocations {
        assert_eq!(
            invocation.status,
            Some(0),
            "{}: {}",
            invocation.binary.display(),
            invocation.stderr
        );
        assert!(
            invocation.stdout.contains(&response),
            "stdout: {}",
            invocation.stdout
        );
    }
}

#[then(
    regex = r##"^each binary should request exactly the configured twin base URL plus "([^"]+)"$"##
)]
fn each_binary_configured_url(world: &mut WatnWorld, path: String) {
    let expected = configured_server(world).url(&path);
    let endpoint = configured_endpoint(world);
    let suffix = path.strip_prefix("/v1").unwrap_or(&path);
    assert_eq!(expected, format!("{endpoint}{suffix}"));
    assert_eq!(configured_mock_hits(world), 2);
}

#[then(regex = r##"^each configured-twin request should be POST path "([^"]+)" exactly once$"##)]
fn each_configured_request(world: &mut WatnWorld, path: String) {
    assert_eq!(Some(path), world.transport.configured_path.clone());
    assert_eq!(configured_mock_hits(world), 2);
}

#[then(regex = r##"^each configured-twin request should have Authorization exactly "([^"]+)"$"##)]
fn each_configured_authorization(world: &mut WatnWorld, authorization: String) {
    assert_eq!(
        authorization,
        format!("Bearer {}", world.transport.api_key.as_deref().unwrap())
    );
    assert_eq!(configured_mock_hits(world), 2);
}

#[then(
    regex = r##"^the competing twin should receive exactly (\d+) requests for path "([^"]+)"$"##
)]
fn competing_request_count(world: &mut WatnWorld, count: u32, path: String) {
    let server = world
        .transport
        .competing_server
        .as_ref()
        .expect("competing provider twin was not created");
    let mock_id = world
        .transport
        .competing_mock_id
        .expect("competing provider mock was not created");
    assert_eq!(
        server.url(&path),
        format!(
            "{}{}",
            world.transport.competing_endpoint.as_deref().unwrap(),
            "/chat/completions"
        )
    );
    assert_eq!(httpmock::Mock::new(mock_id, server).hits(), count as usize);
}

#[then(
    regex = r##"^the persisted configured endpoint should remain exactly the configured twin base URL plus "([^"]+)"$"##
)]
fn persisted_configured_endpoint(world: &mut WatnWorld, path: String) {
    let endpoint = configured_endpoint(world);
    let config_path = world
        .transport
        .config_path
        .as_ref()
        .expect("transport config was not written");
    let raw = std::fs::read_to_string(config_path).expect("read transport config");
    assert!(
        raw.contains(endpoint),
        "configured endpoint missing from TOML"
    );
    assert_eq!(configured_server(world).url(&path), endpoint);
}

#[then(regex = r##"^the response should contain "([^"]+)"$"##)]
fn response_contains(world: &mut WatnWorld, response: String) {
    let invocation = world
        .transport
        .invocations
        .last()
        .expect("no transport invocation was recorded");
    assert_eq!(invocation.status, Some(0), "{}", invocation.stderr);
    assert!(
        invocation.stdout.contains(&response),
        "stdout: {}",
        invocation.stdout
    );
}

#[then(
    regex = r##"^the isolated twin base URL plus "([^"]+)" should be the exact request endpoint, with path "([^"]+)"$"##
)]
fn isolated_request_url(world: &mut WatnWorld, base_path: String, path: String) {
    let server = world
        .transport
        .isolated_server
        .as_ref()
        .expect("isolated provider twin was not created");
    let endpoint = world
        .transport
        .isolated_endpoint
        .as_deref()
        .expect("isolated endpoint was not created");
    assert_eq!(
        server.url(format!("{base_path}{path}")),
        format!("{endpoint}{path}")
    );
}

#[then(regex = r##"^the isolated-twin request should be POST path "([^"]+)" exactly once$"##)]
fn isolated_request_count(world: &mut WatnWorld, path: String) {
    let server = world
        .transport
        .isolated_server
        .as_ref()
        .expect("isolated provider twin was not created");
    let mock_id = world
        .transport
        .isolated_mock_id
        .expect("isolated provider mock was not created");
    assert_eq!(
        server.url(&path),
        format!(
            "{}{}",
            world.transport.isolated_endpoint.as_deref().unwrap(),
            "/chat/completions"
        )
    );
    assert_eq!(httpmock::Mock::new(mock_id, server).hits(), 1);
}

#[then(regex = r##"^the isolated-twin request should have Authorization exactly "([^"]+)"$"##)]
fn isolated_authorization(world: &mut WatnWorld, authorization: String) {
    assert_eq!(
        authorization,
        format!("Bearer {}", world.transport.api_key.as_deref().unwrap())
    );
    let server = world
        .transport
        .isolated_server
        .as_ref()
        .expect("isolated provider twin was not created");
    let mock_id = world
        .transport
        .isolated_mock_id
        .expect("isolated provider mock was not created");
    assert_eq!(httpmock::Mock::new(mock_id, server).hits(), 1);
}

#[then(
    regex = r##"^the configured twin should receive exactly (\d+) requests for path "([^"]+)"$"##
)]
fn configured_request_count(world: &mut WatnWorld, count: u32, path: String) {
    assert_eq!(Some(path), world.transport.configured_path.clone());
    assert_eq!(configured_mock_hits(world), count as usize);
}

#[then("the persisted TOML should not contain the isolated twin URL")]
fn persisted_toml_excludes_isolated(world: &mut WatnWorld) {
    let config_path = world
        .transport
        .config_path
        .as_ref()
        .expect("transport config was not written");
    let isolated_endpoint = world
        .transport
        .isolated_endpoint
        .as_deref()
        .expect("isolated endpoint was not created");
    let raw = std::fs::read_to_string(config_path).expect("read transport config");
    assert!(!raw.contains(isolated_endpoint));
}

#[given(
    regex = r##"^a reachable local configured provider record has endpoint path "([^"]+)", api key "([^"]+)", and default model "([^"]+)"$"##
)]
fn configured_provider_record(world: &mut WatnWorld, path: String, api_key: String, model: String) {
    let configured_server = MockServer::start();
    let competing_server = MockServer::start();
    let configured_endpoint = configured_server.url(&path);
    let competing_endpoint = competing_server.url(&path);
    let configured_mock_id = configured_server
        .mock(|when, then| {
            when.method(Method::POST).path("/v1/chat/completions");
            then.status(200);
        })
        .id;
    let competing_mock_id = competing_server
        .mock(|when, then| {
            when.method(Method::POST).path("/v1/chat/completions");
            then.status(200);
        })
        .id;
    let mut config = Config::default();
    config.defaults.provider = Some("configured".to_string());
    config.providers.insert(
        "configured".to_string(),
        ProviderConfig {
            endpoint: configured_endpoint.clone(),
            api_key: Some(api_key),
            default_model: Some(model),
        },
    );
    world.env_vars.insert(
        "WATN_TEST_ENDPOINT_OVERRIDE".to_string(),
        competing_endpoint.clone(),
    );
    std::env::set_var("WATN_TEST_ENDPOINT_OVERRIDE", competing_endpoint.clone());
    world.transport = TransportState {
        configured_server: Some(configured_server),
        configured_mock_id: Some(configured_mock_id),
        configured_endpoint: Some(configured_endpoint),
        configured_path: Some("/v1/chat/completions".to_string()),
        configured_response: None,
        competing_server: Some(competing_server),
        competing_mock_id: Some(competing_mock_id),
        competing_endpoint: Some(competing_endpoint),
        isolated_server: None,
        isolated_mock_id: None,
        isolated_endpoint: None,
        config: Some(config),
        config_path: None,
        api_key: None,
        invocations: Vec::new(),
        readiness: None,
    };
}

#[given(
    regex = r##"^a separate reachable local competing endpoint is selected by the test routing setting$"##
)]
fn competing_endpoint_selected(world: &mut WatnWorld) {
    let endpoint = world
        .transport
        .competing_endpoint
        .as_deref()
        .expect("competing endpoint was not created")
        .to_string();
    world
        .env_vars
        .insert("WATN_TEST_ENDPOINT_OVERRIDE".to_string(), endpoint.clone());
    std::env::set_var("WATN_TEST_ENDPOINT_OVERRIDE", endpoint);
}

#[when("I evaluate provider readiness with the test routing setting present without starting an HTTP request")]
fn evaluate_readiness(world: &mut WatnWorld) {
    let config = world
        .transport
        .config
        .as_ref()
        .expect("configured provider record was not created");
    world.transport.readiness = Some(watn::config::provider_ready(config, "configured"));
}

#[then("provider readiness should be ready")]
fn readiness_should_be_ready(world: &mut WatnWorld) {
    assert_eq!(world.transport.readiness, Some(true));
}

#[then(
    regex = r##"^the configured endpoint in the provider record should remain exactly the configured local endpoint$"##
)]
fn configured_record_endpoint(world: &mut WatnWorld) {
    let config = world
        .transport
        .config
        .as_ref()
        .expect("configured provider record was not created");
    let endpoint = world
        .transport
        .configured_endpoint
        .as_deref()
        .expect("configured endpoint was not created");
    assert_eq!(config.providers["configured"].endpoint, endpoint);
}

#[then(regex = r##"^both local endpoints should have received exactly (\d+) requests$"##)]
fn both_endpoints_request_count(world: &mut WatnWorld, count: u32) {
    let configured_server = world
        .transport
        .configured_server
        .as_ref()
        .expect("configured server was not created");
    let configured_mock_id = world
        .transport
        .configured_mock_id
        .expect("configured mock was not created");
    let competing_server = world
        .transport
        .competing_server
        .as_ref()
        .expect("competing server was not created");
    let competing_mock_id = world
        .transport
        .competing_mock_id
        .expect("competing mock was not created");
    assert_eq!(
        httpmock::Mock::new(configured_mock_id, configured_server).hits(),
        count as usize
    );
    assert_eq!(
        httpmock::Mock::new(competing_mock_id, competing_server).hits(),
        count as usize
    );
}
