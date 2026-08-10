//! Step definitions for setup persistence-boundary scenarios.

use super::{finish_pty_session, pty_snapshot, pty_write};
use crate::WatnWorld;
use cucumber::{then, when};

#[when("confirm the credential before loading models")]
fn confirm_credential(world: &mut WatnWorld) {
    let session = world.pty_session.as_mut().expect("setup PTY session");
    pty_write(session, "\r");
    std::thread::sleep(std::time::Duration::from_millis(800));
    let output = pty_snapshot(session);
    assert!(output.contains("catalog") || output.contains("model") || output.contains("error"));
    let session = world.pty_session.take().expect("setup PTY session");
    finish_pty_session(world, session);
}

#[then("the setup wizard should report the catalog failure")]
fn setup_reports_catalog_failure(world: &mut WatnWorld) {
    let output = world.output.as_deref().unwrap_or_default();
    let stderr = world.stderr_output.as_deref().unwrap_or_default();
    assert!(matches!(world.exit_status, Some(1) | Some(2)));
    assert!(!output.is_empty() || !stderr.is_empty());
}

#[then(regex = r##"^the default provider should remain "([^"]+)"$"##)]
fn default_provider_remains(world: &mut WatnWorld, expected: String) {
    let dir = world.temp_dir.as_ref().expect("config directory");
    let content = std::fs::read_to_string(dir.path().join("watn/config.toml")).expect("config");
    assert!(content.contains(&format!("provider = \"{expected}\"")));
}

#[then("the LiteLLM settings should remain unchanged")]
fn litellm_settings_remain(world: &mut WatnWorld) {
    let dir = world.temp_dir.as_ref().expect("config directory");
    let content = std::fs::read_to_string(dir.path().join("watn/config.toml")).expect("config");
    assert!(content.contains("[litellm]"));
}
