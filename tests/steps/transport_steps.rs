use cucumber::{given, then, when};
use httpmock::{Method, MockServer};
use std::fmt;
use watn::config::types::{Config, ProviderConfig};

use crate::WatnWorld;

#[derive(Default)]
pub struct TransportState {
    configured_server: Option<MockServer>,
    configured_mock_id: Option<usize>,
    configured_endpoint: Option<String>,
    competing_server: Option<MockServer>,
    competing_mock_id: Option<usize>,
    competing_endpoint: Option<String>,
    config: Option<Config>,
    readiness: Option<bool>,
}

impl fmt::Debug for TransportState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TransportState")
            .field("configured_endpoint", &self.configured_endpoint)
            .field("competing_endpoint", &self.competing_endpoint)
            .field("readiness", &self.readiness)
            .finish()
    }
}

#[given(regex = r##"^a reachable local configured provider twin returns "([^"]+)" for POST "([^"]+)"$"##)]
fn configured_provider_twin(_world: &mut WatnWorld, _response: String, _path: String) {
    unimplemented!()
}

#[given(regex = r##"^the configured provider has api key "([^"]+)" and default model "([^"]+)"$"##)]
fn configured_provider_credentials(_world: &mut WatnWorld, _api_key: String, _model: String) {
    unimplemented!()
}

#[given(regex = r##"^a separate reachable local competing provider twin returns "([^"]+)" for POST "([^"]+)"$"##)]
fn competing_provider_twin(_world: &mut WatnWorld, _response: String, _path: String) {
    unimplemented!()
}

#[given(regex = r##"^a separate reachable local isolated provider twin returns "([^"]+)" for POST "([^"]+)"$"##)]
fn isolated_provider_twin(_world: &mut WatnWorld, _response: String, _path: String) {
    unimplemented!()
}

#[when("I run the default-feature release binary and the test-support release binary with the override set to the competing twin")]
fn run_release_binaries(_world: &mut WatnWorld) {
    unimplemented!()
}

#[when("I run the test-support debug binary with the override set to the isolated twin")]
fn run_isolated_debug_binary(_world: &mut WatnWorld) {
    unimplemented!()
}

#[when(regex = r##"^I run the test-support debug binary with the override state "([^"]+)"$"##)]
fn run_debug_binary_with_override_state(_world: &mut WatnWorld, _state: String) {
    unimplemented!()
}

#[then(regex = r##"^each binary should exit successfully with output containing "([^"]+)"$"##)]
fn each_binary_output(_world: &mut WatnWorld, _response: String) {
    unimplemented!()
}

#[then(regex = r##"^each binary should request exactly the configured twin base URL plus "([^"]+)"$"##)]
fn each_binary_configured_url(_world: &mut WatnWorld, _path: String) {
    unimplemented!()
}

#[then(regex = r##"^each configured-twin request should be POST path "([^"]+)" exactly once$"##)]
fn each_configured_request(_world: &mut WatnWorld, _path: String) {
    unimplemented!()
}

#[then(regex = r##"^each configured-twin request should have Authorization exactly "([^"]+)"$"##)]
fn each_configured_authorization(_world: &mut WatnWorld, _authorization: String) {
    unimplemented!()
}

#[then(regex = r##"^the competing twin should receive exactly (\d+) requests for path "([^"]+)"$"##)]
fn competing_request_count(_world: &mut WatnWorld, _count: u32, _path: String) {
    unimplemented!()
}

#[then(regex = r##"^the persisted configured endpoint should remain exactly the configured twin base URL plus "([^"]+)"$"##)]
fn persisted_configured_endpoint(_world: &mut WatnWorld, _path: String) {
    unimplemented!()
}

#[then(regex = r##"^the response should contain "([^"]+)"$"##)]
fn response_contains(_world: &mut WatnWorld, _response: String) {
    unimplemented!()
}

#[then(regex = r##"^the isolated twin base URL plus "([^"]+)" should be the exact request endpoint, with path "([^"]+)"$"##)]
fn isolated_request_url(_world: &mut WatnWorld, _base_path: String, _path: String) {
    unimplemented!()
}

#[then(regex = r##"^the isolated-twin request should be POST path "([^"]+)" exactly once$"##)]
fn isolated_request_count(_world: &mut WatnWorld, _path: String) {
    unimplemented!()
}

#[then(regex = r##"^the isolated-twin request should have Authorization exactly "([^"]+)"$"##)]
fn isolated_authorization(_world: &mut WatnWorld, _authorization: String) {
    unimplemented!()
}

#[then(regex = r##"^the configured twin should receive exactly (\d+) requests for path "([^"]+)"$"##)]
fn configured_request_count(_world: &mut WatnWorld, _count: u32, _path: String) {
    unimplemented!()
}

#[then("the persisted TOML should not contain the isolated twin URL")]
fn persisted_toml_excludes_isolated(_world: &mut WatnWorld) {
    unimplemented!()
}

#[given(regex = r##"^a reachable local configured provider record has endpoint path "([^"]+)", api key "([^"]+)", and default model "([^"]+)"$"##)]
fn configured_provider_record(
    world: &mut WatnWorld,
    path: String,
    api_key: String,
    model: String,
) {
    let configured_server = MockServer::start();
    let competing_server = MockServer::start();
    let configured_endpoint = format!("http://127.0.0.1:{}/{}", configured_server.port(), path.trim_start_matches('/'));
    let competing_endpoint = format!("http://127.0.0.1:{}/{}", competing_server.port(), path.trim_start_matches('/'));
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

    world
        .env_vars
        .insert("WATN_TEST_ENDPOINT_OVERRIDE".to_string(), competing_endpoint.clone());
    std::env::set_var("WATN_TEST_ENDPOINT_OVERRIDE", &competing_endpoint);
    world.transport = TransportState {
        configured_server: Some(configured_server),
        configured_mock_id: Some(configured_mock_id),
        configured_endpoint: Some(configured_endpoint),
        competing_server: Some(competing_server),
        competing_mock_id: Some(competing_mock_id),
        competing_endpoint: Some(competing_endpoint),
        config: Some(config),
        readiness: None,
    };
}

#[given(regex = r##"^a separate reachable local competing endpoint is selected by the test routing setting$"##)]
fn competing_endpoint_selected(world: &mut WatnWorld) {
    let endpoint = world
        .transport
        .competing_endpoint
        .as_ref()
        .expect("competing endpoint was not created")
        .clone();
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

#[then(regex = r##"^the configured endpoint in the provider record should remain exactly the configured local endpoint$"##)]
fn configured_record_endpoint(world: &mut WatnWorld) {
    let config = world
        .transport
        .config
        .as_ref()
        .expect("configured provider record was not created");
    let configured_endpoint = world
        .transport
        .configured_endpoint
        .as_ref()
        .expect("configured endpoint was not created");
    assert_eq!(config.providers["configured"].endpoint, *configured_endpoint);
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
    assert_eq!(httpmock::Mock::new(configured_mock_id, configured_server).hits(), count as usize);
    assert_eq!(httpmock::Mock::new(competing_mock_id, competing_server).hits(), count as usize);
}
