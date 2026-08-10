//! Step definitions for credential-source and credential-fallback scenarios.

use cucumber::{given, then};
use httpmock::Method;

use crate::WatnWorld;

#[given(regex = r##"^its saved api_key is absent$"##)]
fn saved_api_key_absent(world: &mut WatnWorld) {
    world
        .pending_config
        .insert("saved_key".to_string(), String::new());
    world
        .pending_config
        .insert("expect_custom_auth".to_string(), "true".to_string());
    let provider = world
        .pending_config
        .get("saved_provider")
        .cloned()
        .expect("provider name");
    let endpoint = world
        .pending_config
        .get("saved_endpoint")
        .cloned()
        .expect("provider endpoint");
    world.raw_config = Some(format!(
        "[defaults]\nprovider = \"{provider}\"\n\n[providers.{provider}]\nendpoint = \"{endpoint}\"\n"
    ));
    let dir = tempfile::tempdir().expect("test config directory");
    let config_dir = dir.path().join("watn");
    std::fs::create_dir_all(&config_dir).expect("config directory");
    let base_url = format!(
        "http://127.0.0.1:{}",
        world.mock_server.0.as_ref().expect("provider mock").port()
    );
    std::fs::write(
        config_dir.join("config.toml"),
        format!(
            "[defaults]\nprovider = \"{provider}\"\n\n[providers.{provider}]\nendpoint = \"{base_url}\"\ndefault_model = \"custom-model\"\n"
        ),
    )
    .expect("write test config");
    world.env_vars.insert(
        "XDG_CONFIG_HOME".to_string(),
        dir.path().to_string_lossy().to_string(),
    );
    world.temp_dir = Some(dir);
}

#[then("the generic environment fallback should not be used")]
fn generic_fallback_not_used(world: &mut WatnWorld) {
    let key = world
        .env_vars
        .get("WATN_CUSTOM_API_KEY")
        .expect("provider-specific fallback")
        .clone();
    assert_eq!(key, "sk-provider-fallback");
    assert_ne!(key, "sk-generic-fallback");
}

#[given(regex = r##"^environment variable ([A-Z0-9_]+) is absent$"##)]
fn environment_variable_absent(world: &mut WatnWorld, name: String) {
    world.env_vars.remove(&name);
    std::env::remove_var(&name);
    let server = world
        .mock_server
        .0
        .as_ref()
        .expect("configured provider mock");
    let mock = server.mock(|when, then| {
        when.method(Method::GET).path("/models");
        then.status(200)
            .body(r#"{"data":[{"id":"should-not-be-used"}]}"#);
    });
    world.models_mock_id = Some(mock.id);
}

#[then("the exit status should classify the failure as authentication")]
fn authentication_failure(world: &mut WatnWorld) {
    assert_eq!(world.exit_status, Some(2));
    assert!(world
        .stderr_output
        .as_deref()
        .unwrap_or_default()
        .to_ascii_lowercase()
        .contains("authentication"));
}

#[then("no model catalog request should be sent")]
fn no_catalog_request(world: &mut WatnWorld) {
    let id = world.models_mock_id.expect("catalog mock");
    let server = world.mock_server.0.as_ref().expect("mock server");
    assert_eq!(httpmock::Mock::new(id, server).hits(), 0);
}

#[then(regex = r##"^the saved api_key should remain exactly "([^"]+)"$"##)]
fn saved_api_key_exact(_world: &mut WatnWorld, expected: String) {
    let dir = _world.temp_dir.as_ref().expect("test config directory");
    let content =
        std::fs::read_to_string(dir.path().join("watn/config.toml")).expect("config file");
    assert!(content.contains(&format!("api_key = \"{expected}\"")));
}
