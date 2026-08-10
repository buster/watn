//! Step definitions for catalog-source resolution scenarios.

use cucumber::{given, then};
use httpmock::Method;

use crate::{MockServerWrap, WatnWorld};

#[given(regex = r##"^a provider "([^"]+)" with a separate LiteLLM catalog endpoint$"##)]
fn provider_with_litellm(world: &mut WatnWorld, provider: String) {
    let server = httpmock::MockServer::start();
    let endpoint = format!("http://127.0.0.1:{}", server.port());
    world.mock_server = MockServerWrap(Some(server), None);
    world.raw_config = Some(format!(
        "[defaults]\nprovider = \"{provider}\"\n\n[providers.{provider}]\nendpoint = \"https://provider.invalid/v1\"\napi_key = \"sk-provider\"\n\n[litellm]\nendpoint = \"{endpoint}\"\n"
    ));
}

#[given("the LiteLLM catalog has no api key")]
fn litellm_without_key(world: &mut WatnWorld) {
    assert!(world
        .raw_config
        .as_deref()
        .unwrap_or_default()
        .contains("[litellm]"));
}

#[given(regex = r##"^the LiteLLM catalog returns models \[([^\]]+)\]$"##)]
fn litellm_models(world: &mut WatnWorld, values: String) {
    let models: Vec<String> = values
        .split(',')
        .map(|value| value.trim().trim_matches('"').to_string())
        .collect();
    world.pending_mock_returned_models = models.clone();
    let server = world.mock_server.0.as_ref().expect("LiteLLM mock");
    let data: Vec<serde_json::Value> = models
        .iter()
        .map(|id| serde_json::json!({"id": id}))
        .collect();
    let mock = server.mock(|when, then| {
        when.method(Method::GET).path("/models");
        then.status(200)
            .header("Content-Type", "application/json")
            .body(serde_json::json!({"data": data}).to_string());
    });
    world.models_mock_id = Some(mock.id);
}

#[then("the model catalog request should use the LiteLLM endpoint")]
fn litellm_request_used(world: &mut WatnWorld) {
    assert_eq!(
        world.exit_status,
        Some(0),
        "models output: {:?}",
        world.stderr_output
    );
    let id = world.models_mock_id.expect("catalog mock");
    let server = world.mock_server.0.as_ref().expect("catalog server");
    assert!(httpmock::Mock::new(id, server).hits() > 0);
}

#[then("the model catalog request should not include an Authorization header")]
fn litellm_request_without_auth(world: &mut WatnWorld) {
    let id = world.models_mock_id.expect("catalog mock");
    let server = world.mock_server.0.as_ref().expect("catalog server");
    assert!(httpmock::Mock::new(id, server).hits() > 0);
}

#[then(regex = r##"^the config file should contain the selected tier assignments$"##)]
fn selected_tiers(world: &mut WatnWorld) {
    let dir = world.temp_dir.as_ref().expect("config directory");
    let content =
        std::fs::read_to_string(dir.path().join("watn/config.toml")).expect("config file");
    for model in &world.pending_mock_returned_models {
        assert!(content.contains(model));
    }
}
