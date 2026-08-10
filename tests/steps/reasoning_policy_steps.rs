//! Step definitions for reasoning-policy scenarios.

use crate::MockServerWrap;
use crate::WatnWorld;
use cucumber::{given, then, when};
use watn::models::dialog::resolve_reasoning_default;
use watn::models::list::ModelReasoning;

fn metadata(world: &WatnWorld) -> ModelReasoning {
    ModelReasoning {
        default_effort: world.pending_config.get("default_effort").cloned(),
        default_enabled: world
            .pending_config
            .get("default_enabled")
            .map(|v| v == "true")
            .unwrap_or(true),
        mandatory: world
            .pending_config
            .get("mandatory")
            .map(|v| v == "true")
            .unwrap_or(false),
        supported_efforts: world
            .pending_config
            .get("supported")
            .map(|v| v.split(',').map(str::to_string).collect())
            .unwrap_or_default(),
    }
}

#[given(
    regex = r##"^model reasoning metadata has default effort "([^"]+)", default enabled (true|false), and supported efforts "([^"]+)", "([^"]+)"$"##
)]
fn reasoning_metadata(
    world: &mut WatnWorld,
    effort: String,
    enabled: String,
    first: String,
    second: String,
) {
    world.pending_config.insert("default_effort".into(), effort);
    world
        .pending_config
        .insert("default_enabled".into(), enabled);
    world
        .pending_config
        .insert("supported".into(), format!("{first},{second}"));
}

#[when("I resolve the model reasoning default")]
fn resolve_reasoning(world: &mut WatnWorld) {
    let result = resolve_reasoning_default(&metadata(world), None)
        .map(|value| value.as_str().to_string())
        .map_err(|error| error.to_string());
    world.pending_config.insert(
        "resolved_reasoning".into(),
        result.unwrap_or_else(|error| format!("error:{error}")),
    );
}

#[given(
    regex = r##"^model reasoning metadata is mandatory with supported efforts "([^"]+)", "([^"]+)"$"##
)]
fn mandatory_metadata(world: &mut WatnWorld, first: String, second: String) {
    world
        .pending_config
        .insert("mandatory".into(), "true".into());
    world
        .pending_config
        .insert("supported".into(), format!("{first},{second}"));
}

#[given(regex = r##"^model reasoning metadata is mandatory with supported efforts "([^"]+)"$"##)]
fn mandatory_invalid_metadata(world: &mut WatnWorld, only: String) {
    world
        .pending_config
        .insert("mandatory".into(), "true".into());
    world.pending_config.insert("supported".into(), only);
}

#[given("no existing non-off reasoning value is configured")]
fn no_existing_reasoning(world: &mut WatnWorld) {
    world.pending_config.remove("existing_reasoning");
}

#[then("the resolver should return a reasoning policy error")]
fn reasoning_policy_error(world: &mut WatnWorld) {
    assert!(world
        .pending_config
        .get("resolved_reasoning")
        .is_some_and(|value| value.starts_with("error:")));
}

#[then("the config file should not contain an empty reasoning value")]
fn no_empty_reasoning(world: &mut WatnWorld) {
    let dir = world.temp_dir.as_ref().expect("config directory");
    let content = std::fs::read_to_string(dir.path().join("watn/config.toml")).expect("config");
    assert!(!content.contains("reasoning = \"\""));
    assert!(!content.contains("reasoning = ''"));
}

#[given("a configured provider \"test\" with models that have no reasoning metadata")]
fn models_without_reasoning(world: &mut WatnWorld) {
    let server = httpmock::MockServer::start();
    let endpoint = format!("http://127.0.0.1:{}", server.port());
    server.mock(|when, then| {
        when.method(httpmock::Method::GET).path("/models");
        then.status(200)
            .json_body(serde_json::json!({"data":[{"id":"new-model"}]}));
    });
    world.mock_server = MockServerWrap(Some(server), None);
    world.pending_mock_returned_models = vec!["new-model".into()];
    world.raw_config = Some(format!(
        "[defaults]\nprovider = \"test\"\n\n[providers.test]\nendpoint = \"{endpoint}\"\napi_key = \"test-key\"\n"
    ));
}

#[given(regex = r##"^the existing small tier reasoning is "([^"]+)"$"##)]
fn existing_small_reasoning(world: &mut WatnWorld, effort: String) {
    let raw = world.raw_config.take().expect("provider config");
    world.raw_config = Some(format!(
        "{raw}\n[tiers]\nsmall = \"old-model\"\n[tiers.reasoning]\nsmall = \"{effort}\"\n"
    ));
}

#[when("I select a new model for the small tier through the non-TTY model assignment")]
fn select_new_small_model(world: &mut WatnWorld) {
    crate::steps::run_binary_with_state(world, &["models"], Some("0\n0\n0\n"));
}

#[then(regex = r##"^the saved small tier reasoning should remain "([^"]+)"$"##)]
fn saved_small_reasoning(world: &mut WatnWorld, expected: String) {
    let dir = world.temp_dir.as_ref().expect("config directory");
    let content = std::fs::read_to_string(dir.path().join("watn/config.toml")).expect("config");
    assert!(content.contains(&format!("small = \"{expected}\"")));
}

#[then("the new small-tier model should be persisted")]
fn new_small_model(world: &mut WatnWorld) {
    let dir = world.temp_dir.as_ref().expect("config directory");
    let content = std::fs::read_to_string(dir.path().join("watn/config.toml")).expect("config");
    assert!(content.contains("small = \"new-model\""));
}

#[then(regex = r##"^the selected reasoning should be "([^"]+)"$"##)]
fn selected_reasoning(world: &mut WatnWorld, expected: String) {
    assert_eq!(
        world.pending_config.get("resolved_reasoning"),
        Some(&expected)
    );
}

#[then(
    regex = r##"^the selected reasoning should be the first valid supported effort "([^"]+)"$"##
)]
fn first_supported_reasoning(world: &mut WatnWorld, expected: String) {
    selected_reasoning(world, expected);
}
