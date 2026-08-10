//! Step definitions for credential-source and credential-fallback scenarios.

use cucumber::{given, then};
use httpmock::Method;

use crate::WatnWorld;

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
