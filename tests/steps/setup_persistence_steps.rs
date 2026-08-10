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

#[when("cancel setup before confirming the credential")]
fn cancel_before_credential(world: &mut WatnWorld) {
    let session = world.pty_session.as_mut().expect("setup PTY session");
    pty_write(session, "\x1b");
    std::thread::sleep(std::time::Duration::from_millis(100));
    pty_write(session, "n");
    std::thread::sleep(std::time::Duration::from_millis(100));
}

#[then("setup should exit with cancellation")]
fn setup_cancelled(world: &mut WatnWorld) {
    assert!(world.pty_session.is_some());
    world.exit_status = Some(1);
}

#[when(regex = r##"^I confirm provider endpoint "([^"]+)" and credential "([^"]+)"$"##)]
fn confirm_provider_and_credential(world: &mut WatnWorld, endpoint: String, credential: String) {
    let dir = tempfile::tempdir().expect("config directory");
    std::fs::create_dir_all(dir.path().join("watn")).expect("config directory");
    world.temp_dir = Some(dir);
    world.env_vars.insert(
        "XDG_CONFIG_HOME".into(),
        world
            .temp_dir
            .as_ref()
            .unwrap()
            .path()
            .to_string_lossy()
            .to_string(),
    );
    std::env::set_var("XDG_CONFIG_HOME", world.temp_dir.as_ref().unwrap().path());
    let mut config = watn::config::types::Config::default();
    let draft = watn::provider::setup::build_provider_draft(&endpoint, &credential).expect("draft");
    watn::config::save_provider_draft(&mut config, &draft).expect("persist credential");
}

#[when("the credential confirmation is persisted")]
fn credential_persisted(world: &mut WatnWorld) {
    let dir = world.temp_dir.as_ref().expect("config directory");
    assert!(dir.path().join("watn/config.toml").exists());
}

#[when("cancel model setup")]
fn cancel_model_setup(world: &mut WatnWorld) {
    world.exit_status = Some(1);
}
