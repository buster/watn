use cucumber::{then, when};

use crate::WatnWorld;
use super::{finish_pty_session, pty_snapshot, pty_wait_for_label, pty_write, start_pty_session};

fn assert_words(output: &str, text: &str) {
    for word in text.split_whitespace() {
        assert!(output.contains(word), "missing {word:?} in setup output: {output:?}");
    }
}

fn latest_page(output: &str, page: &str) -> bool {
    output
        .rfind("Page")
        .map(|index| {
            let current = &output[index..];
            page.split_whitespace().all(|word| current.contains(word))
        })
        .unwrap_or(false)
}

fn wait_for_page(session: &super::PtySession, page: &str) -> String {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(3);
    loop {
        let output = pty_snapshot(session);
        if latest_page(&output, page) {
            return output;
        }
        if std::time::Instant::now() >= deadline {
            panic!("setup page {page:?} was not rendered: {output:?}");
        }
        std::thread::sleep(std::time::Duration::from_millis(25));
    }
}

#[when("I start `watn setup` in a terminal")]
fn start_setup_wizard(world: &mut WatnWorld) {
    let session = start_pty_session(world, &["setup"]);
    world.pty_session = Some(session);
    let session = world.pty_session.as_ref().expect("setup PTY session");
    pty_wait_for_label(session, "Setup");
}

#[when("I start the shared `watn models` wizard in a terminal")]
fn start_shared_models_wizard(world: &mut WatnWorld) {
    let session = start_pty_session(world, &["models"]);
    world.pty_session = Some(session);
    let session = world.pty_session.as_ref().expect("setup PTY session");
    wait_for_page(session, "Small Model");
}

#[then(regex = r#"^the setup wizard should show tabs "([^"]+)", "([^"]+)", "([^"]+)", "([^"]+)", "([^"]+)"$"#)]
fn setup_wizard_tabs(
    _world: &mut WatnWorld,
    _first: String,
    _second: String,
    _third: String,
    _fourth: String,
    _fifth: String,
) {
    let session = _world.pty_session.as_ref().expect("setup PTY session");
    let output = pty_wait_for_label(session, &_first);
    for title in [&_first, &_second, &_third, &_fourth, &_fifth] {
        assert_words(&output, title);
    }
}

#[then(regex = r#"^the setup wizard should show the (URL|API key|Small Model|Middle Model|Large Model) page as active$"#)]
fn setup_wizard_active_page(world: &mut WatnWorld, page: String) {
    let session = world.pty_session.as_ref().expect("setup PTY session");
    let output = wait_for_page(session, &page);
    assert_words(&output, &page);
}

#[then("the setup wizard should explain OpenAI and LiteLLM compatibility")]
fn setup_wizard_compatibility_explanation(world: &mut WatnWorld) {
    let session = world.pty_session.as_ref().expect("setup PTY session");
    let output = pty_snapshot(session);
    for word in ["OpenAI", "LiteLLM", "compatible"] {
        assert_words(&output, word);
    }
}

#[then("the setup wizard should show a visible cursor on the active input")]
fn setup_wizard_visible_cursor(world: &mut WatnWorld) {
    let session = world.pty_session.as_ref().expect("setup PTY session");
    assert!(pty_snapshot(session).contains('█'), "cursor marker missing");
}

#[when("I enter the default endpoint and advance to the API key page")]
fn enter_default_endpoint(world: &mut WatnWorld) {
    let session = world.pty_session.as_mut().expect("setup PTY session");
    pty_write(session, "\r");
    std::thread::sleep(std::time::Duration::from_millis(100));
}

#[when("I advance to the API key page in provider setup")]
fn advance_provider_to_api_key(world: &mut WatnWorld) {
    enter_default_endpoint(world);
}

#[when("choose to store the API key in the configuration")]
fn choose_configuration_storage(world: &mut WatnWorld) {
    let session = world.pty_session.as_mut().expect("setup PTY session");
    pty_write(session, "p");
    std::thread::sleep(std::time::Duration::from_millis(100));
}

#[when(regex = r#"^enter API key "([^"]+)" and advance to Small Model$"#)]
fn enter_api_key(world: &mut WatnWorld, key: String) {
    let session = world.pty_session.as_mut().expect("setup PTY session");
    pty_write(session, &format!("{key}\r"));
    std::thread::sleep(std::time::Duration::from_millis(500));
}

#[when(regex = r#"^choose "([^"]+)" and "([^"]+)" with Enter$"#)]
fn choose_two_models(world: &mut WatnWorld, small: String, middle: String) {
    let session = world.pty_session.as_mut().expect("setup PTY session");
    for model in [small, middle] {
        pty_write(session, &model);
        std::thread::sleep(std::time::Duration::from_millis(400));
        pty_write(session, "\r");
        std::thread::sleep(std::time::Duration::from_millis(200));
    }
}

#[when(regex = r#"^I type "([^"]+)" on the Large Model page$"#)]
fn type_large_model(world: &mut WatnWorld, model: String) {
    let session = world.pty_session.as_mut().expect("setup PTY session");
    pty_write(session, &model);
    std::thread::sleep(std::time::Duration::from_millis(500));
}

#[when("I confirm the Large Model selection with Enter")]
fn confirm_large_model(world: &mut WatnWorld) {
    let session = world.pty_session.as_mut().expect("setup PTY session");
    pty_write(session, "\r");
    std::thread::sleep(std::time::Duration::from_millis(300));
    let session = world.pty_session.take().expect("setup PTY session");
    finish_pty_session(world, session);
}

#[then("setup should exit successfully")]
fn setup_exits_successfully(world: &mut WatnWorld) {
    assert_eq!(world.exit_status, Some(0), "setup output: {:?}", world.output);
}

#[then(regex = r#"^the setup wizard should show the URL and API key tabs$"#)]
fn setup_wizard_provider_tabs(world: &mut WatnWorld) {
    let session = world.pty_session.as_ref().expect("setup PTY session");
    let output = pty_snapshot(session);
    assert_words(&output, "URL");
    assert_words(&output, "API key");
}

#[then("the setup wizard should show model choices in a table")]
fn setup_wizard_model_table(world: &mut WatnWorld) {
    let session = world.pty_session.as_ref().expect("setup PTY session");
    let output = pty_snapshot(session);
    for column in ["Model", "Context", "Pricing", "Features"] {
        assert_words(&output, column);
    }
}

#[then("the setup wizard should show model-specific reasoning options")]
fn setup_wizard_reasoning_options(world: &mut WatnWorld) {
    let session = world.pty_session.as_ref().expect("setup PTY session");
    let output = pty_snapshot(session);
    assert_words(&output, "Supported:");
    assert_words(&output, "low");
}

#[when("I choose the second model and advance with Enter")]
fn choose_second_model(world: &mut WatnWorld) {
    let session = world.pty_session.as_mut().expect("setup PTY session");
    pty_write(session, "\x1b[B\r");
    std::thread::sleep(std::time::Duration::from_millis(200));
}

#[when("press Escape in the setup wizard")]
fn press_escape_in_setup_wizard(world: &mut WatnWorld) {
    let dir = world.temp_dir.as_ref().expect("config temp dir");
    let path = dir.path().join("watn").join("config.toml");
    let content = std::fs::read_to_string(&path).expect("config file");
    world.pending_config.insert("config_before".to_string(), content);
    let session = world.pty_session.as_mut().expect("setup PTY session");
    pty_write(session, "\x1b");
    std::thread::sleep(std::time::Duration::from_millis(150));
}

#[then("the setup wizard should ask whether to save current settings")]
fn setup_wizard_save_prompt(world: &mut WatnWorld) {
    let session = world.pty_session.as_ref().expect("setup PTY session");
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(3);
    loop {
        let output = pty_snapshot(session);
        if output.contains("Sav") && output.contains("Discard") {
            assert_words(&output, "settings");
            return;
        }
        if std::time::Instant::now() >= deadline {
            panic!("save prompt was not rendered: {output:?}");
        }
        std::thread::sleep(std::time::Duration::from_millis(25));
    }
}

#[when("I choose to discard current setup")]
fn discard_current_setup(world: &mut WatnWorld) {
    let session = world.pty_session.as_mut().expect("setup PTY session");
    pty_write(session, "n");
    std::thread::sleep(std::time::Duration::from_millis(100));
    let session = world.pty_session.take().expect("setup PTY session");
    finish_pty_session(world, session);
    assert_eq!(world.exit_status, Some(1), "discard should cancel setup");
}

#[then(regex = r#"^the config file should contain small tier "([^"]+)", middle tier "([^"]+)", and large tier "([^"]+)"$"#)]
fn config_contains_wizard_tiers(world: &mut WatnWorld, small: String, middle: String, large: String) {
    let dir = world.temp_dir.as_ref().expect("config temp dir");
    let path = dir.path().join("watn").join("config.toml");
    let raw = std::fs::read_to_string(&path).expect("read wizard config");
    let config: watn::config::types::Config = toml::from_str(&raw).expect("parse wizard config");
    assert_eq!(config.tiers.small.as_deref(), Some(small.as_str()));
    assert_eq!(config.tiers.normal.as_deref(), Some(middle.as_str()));
    assert_eq!(config.tiers.thinking.as_deref(), Some(large.as_str()));
}
