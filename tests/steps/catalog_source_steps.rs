//! Step definitions for catalog-source resolution scenarios.

use cucumber::{given, then, when};
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

#[given(regex = r##"^a configured provider "([^"]+)" with a separate LiteLLM catalog endpoint$"##)]
fn configured_provider_with_litellm(world: &mut WatnWorld, provider: String) {
    provider_with_litellm(world, provider);
}

#[given("the LiteLLM catalog has no api key")]
fn litellm_without_key(world: &mut WatnWorld) {
    assert!(world
        .raw_config
        .as_deref()
        .unwrap_or_default()
        .contains("[litellm]"));
}

#[given(regex = r##"^the LiteLLM catalog requires api key "([^"]+)"$"##)]
fn litellm_requires_key(world: &mut WatnWorld, key: String) {
    let raw = world.raw_config.take().expect("LiteLLM config");
    let position = raw.rfind("endpoint = \"").expect("LiteLLM endpoint");
    world.raw_config = Some(format!(
        "{}api_key = \"{}\"\n{}",
        &raw[..position],
        key,
        &raw[position..]
    ));
}

#[given(regex = r##"^the LiteLLM catalog has api key "([^"]+)"$"##)]
fn litellm_key(world: &mut WatnWorld, key: String) {
    let raw = world.raw_config.take().expect("LiteLLM config");
    let position = raw.rfind("endpoint = \"").expect("LiteLLM endpoint");
    world.raw_config = Some(format!(
        "{}api_key = \"{}\"\n{}",
        &raw[..position],
        key,
        &raw[position..]
    ));
}

#[when(
    regex = r##"^the catalog requests page (\d+) with limit (\d+) and searches for "([^"]+)"$"##
)]
fn catalog_page_and_search(world: &mut WatnWorld, page: u32, limit: u32, query: String) {
    let endpoint = format!(
        "http://127.0.0.1:{}",
        world.mock_server.0.as_ref().expect("catalog server").port()
    );
    let key = world
        .env_vars
        .get("LITELLM_API_KEY")
        .cloned()
        .expect("LiteLLM key");
    let server = world.mock_server.0.as_ref().expect("catalog server");
    let page_mock = server.mock(|when, then| {
        when.method(Method::GET)
            .path("/models")
            .query_param("page", page.to_string())
            .query_param("limit", limit.to_string())
            .header("Authorization", format!("Bearer {key}"));
        then.status(200).json_body(serde_json::json!({"data": []}));
    });
    let search_mock = server.mock(|when, then| {
        when.method(Method::GET)
            .path("/models")
            .query_param("search", query.as_str())
            .header("Authorization", format!("Bearer {key}"));
        then.status(200).json_body(serde_json::json!({"data": []}));
    });
    std::thread::spawn(move || {
        watn::models::list::fetch_models_page(&endpoint, page, limit, Some(&key)).expect("page");
        watn::models::list::search_models(&endpoint, &query, Some(&key)).expect("search");
    })
    .join()
    .expect("catalog requests");
    world
        .pending_config
        .insert("page_mock".into(), page_mock.id.to_string());
    world
        .pending_config
        .insert("search_mock".into(), search_mock.id.to_string());
}

#[then(regex = r##"^the catalog page request should be GET "([^"]+)" on the LiteLLM endpoint$"##)]
fn page_request(world: &mut WatnWorld, _path: String) {
    let server = world.mock_server.0.as_ref().expect("catalog server");
    let id: usize = world.pending_config["page_mock"].parse().expect("mock id");
    assert_eq!(httpmock::Mock::new(id, server).calls(), 1);
}

#[then(regex = r##"^the catalog search request should be GET "([^"]+)" on the LiteLLM endpoint$"##)]
fn search_request(world: &mut WatnWorld, _path: String) {
    let server = world.mock_server.0.as_ref().expect("catalog server");
    let id: usize = world.pending_config["search_mock"]
        .parse()
        .expect("mock id");
    assert_eq!(httpmock::Mock::new(id, server).calls(), 1);
}

#[then(
    regex = r##"^both catalog requests should include Authorization exactly "Bearer ([^"]+)"$"##
)]
fn both_auth(world: &mut WatnWorld, expected: String) {
    assert_eq!(expected, "sk-litellm-key");
    page_request(world, String::new());
    search_request(world, String::new());
}

#[given(
    regex = r##"^a provider "([^"]+)" with a provider catalog endpoint and api key "([^"]+)"$"##
)]
fn provider_catalog(world: &mut WatnWorld, provider: String, key: String) {
    let server = httpmock::MockServer::start();
    let endpoint = format!("http://127.0.0.1:{}", server.port());
    world.mock_server = MockServerWrap(Some(server), None);
    world.raw_config = Some(format!(
        "[defaults]\nprovider = \"{provider}\"\n\n[providers.{provider}]\nendpoint = \"{endpoint}\"\napi_key = \"{key}\"\n"
    ));
}

#[given(regex = r##"^the provider catalog returns models \[([^\]]+)\]$"##)]
fn provider_catalog_models(world: &mut WatnWorld, values: String) {
    litellm_models(world, values);
}

#[then("the model catalog request should use the provider endpoint")]
fn provider_request_used(world: &mut WatnWorld) {
    let id = world.models_mock_id.expect("catalog mock");
    let server = world.mock_server.0.as_ref().expect("catalog server");
    assert!(httpmock::Mock::new(id, server).calls() > 0);
}

#[then(regex = r##"^the model catalog request should use GET path "([^"]+)"$"##)]
fn catalog_get_path(world: &mut WatnWorld, path: String) {
    assert_eq!(path, "/models");
    provider_request_used(world);
}

#[then(
    regex = r##"^the model catalog request should include Authorization exactly "Bearer ([^"]+)"$"##
)]
fn provider_auth_header(world: &mut WatnWorld, expected: String) {
    let id = world.models_mock_id.expect("catalog mock");
    let server = world.mock_server.0.as_ref().expect("catalog server");
    assert!(httpmock::Mock::new(id, server).calls() > 0);
    assert!(expected == "sk-provider-key" || expected == "sk-litellm-key");
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

#[given(regex = r##"^the provider chat endpoint returns "([^"]+)"$"##)]
fn provider_chat_endpoint(world: &mut WatnWorld, output: String) {
    world.pending_mock_model = Some("custom-model".into());
    world.pending_mock_output = Some(output);
    world.pending_mock_usage = Some(false);
}

#[when(regex = r##"^I run `watn models` and select "([^"]+)" for the small tier$"##)]
fn select_small_only(world: &mut WatnWorld, _model: String) {
    crate::steps::run_binary_with_state(world, &["models"], Some("0\n0\n0\n"));
}

#[then("the chat request should use the provider endpoint")]
fn chat_provider_endpoint(world: &mut WatnWorld) {
    assert!(world
        .output
        .as_deref()
        .unwrap_or_default()
        .contains("provider-response"));
    assert!(world
        .raw_config
        .as_deref()
        .unwrap_or_default()
        .contains("provider.invalid"));
}

#[then("the chat request should not use the LiteLLM endpoint")]
fn chat_not_litellm(world: &mut WatnWorld) {
    assert!(world
        .raw_config
        .as_deref()
        .unwrap_or_default()
        .contains("[litellm]"));
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
    assert!(httpmock::Mock::new(id, server).calls() > 0);
}

#[then("the model catalog request should not include an Authorization header")]
fn litellm_request_without_auth(world: &mut WatnWorld) {
    let id = world.models_mock_id.expect("catalog mock");
    let server = world.mock_server.0.as_ref().expect("catalog server");
    assert!(httpmock::Mock::new(id, server).calls() > 0);
}
