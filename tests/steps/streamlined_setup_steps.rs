use cucumber::{given, then, when};

use super::{build_config, pty_snapshot, pty_write};
use crate::WatnWorld;

fn latest_page(output: &str) -> &str {
    output
        .rfind("Page")
        .map(|index| &output[index..])
        .unwrap_or(output)
}

fn wait_for_active_page(session: &super::PtySession, title: &str) -> String {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(3);
    loop {
        let output = pty_snapshot(session);
        if latest_page(&output).contains(title) {
            return output;
        }
        if std::time::Instant::now() >= deadline {
            panic!("active setup page {title:?} was not rendered: {output:?}");
        }
        std::thread::sleep(std::time::Duration::from_millis(25));
    }
}

#[given(
    regex = r##"^a configured provider with catalog models "([^"]+)", "([^"]+)", and "([^"]+)"$"##
)]
fn configured_provider_with_catalog_models(
    world: &mut WatnWorld,
    first: String,
    second: String,
    third: String,
) {
    world.raw_config = Some(build_config(
        "custom",
        None,
        Some(vec![("custom", "http://mock", "test-key", "")]),
        None,
        None,
        None,
    ));
    world.pending_mock_model = Some("test-model".to_string());
    world.pending_mock_output = Some("output".to_string());
    world.pending_mock_returned_models = vec![first, second, third];
}

#[when("advance to the small model question")]
fn advance_to_small_model_question(world: &mut WatnWorld) {
    let session = world.pty_session.as_mut().expect("setup PTY session");
    for _ in 0..4 {
        pty_write(session, "\r");
        std::thread::sleep(std::time::Duration::from_millis(150));
    }
    wait_for_active_page(session, "Small Model");
}

#[then("the setup coordinator should show the provider question first")]
fn setup_coordinator_provider_question(_world: &mut WatnWorld) {
    unimplemented!()
}

#[then("the small model question should not contain the reasoning choices")]
fn small_model_question_has_no_reasoning_choices(world: &mut WatnWorld) {
    let session = world.pty_session.as_ref().expect("setup PTY session");
    let output = pty_snapshot(session);
    let page = latest_page(&output);
    assert!(
        page.contains("Small Model"),
        "small model page was not active: {page:?}"
    );
    assert!(
        !page.contains("Choices:"),
        "reasoning choices leaked into model page: {page:?}"
    );
}

#[when(regex = r##"^I choose model "([^"]+)" for the small role$"##)]
fn choose_small_model(world: &mut WatnWorld, model: String) {
    let session = world.pty_session.as_mut().expect("setup PTY session");
    pty_write(session, &model);
    std::thread::sleep(std::time::Duration::from_millis(500));
    pty_write(session, "\r");
    wait_for_active_page(session, "Small Reasoning");
}

#[then(regex = r##"^the small reasoning question should identify model "([^"]+)"$"##)]
fn small_reasoning_identifies_model(world: &mut WatnWorld, model: String) {
    let session = world.pty_session.as_ref().expect("setup PTY session");
    let output = pty_snapshot(session);
    let page = latest_page(&output);
    assert!(
        page.contains("Small Reasoning"),
        "small reasoning page was not active: {page:?}"
    );
    assert!(
        page.contains(&format!("Model: {model}")),
        "selected model missing: {page:?}"
    );
}

#[when(regex = r##"^I choose reasoning "([^"]+)" for the small role$"##)]
fn choose_small_reasoning(world: &mut WatnWorld, effort: String) {
    assert_eq!(effort, "low", "this scenario drives the low effort option");
    let session = world.pty_session.as_mut().expect("setup PTY session");
    pty_write(session, "\x1b[B\r");
    wait_for_active_page(session, "Normal Model");
}

#[then("the normal model question should be active")]
fn normal_model_question_active(world: &mut WatnWorld) {
    let session = world.pty_session.as_ref().expect("setup PTY session");
    let output = pty_snapshot(session);
    let page = latest_page(&output);
    assert!(
        page.contains("Normal Model"),
        "normal model page was not active: {page:?}"
    );
}

#[given(regex = r##"^the provider credential is the literal "([^"]+)"$"##)]
fn provider_credential_literal(world: &mut WatnWorld, credential: String) {
    let raw = world.raw_config.take().expect("provider config fixture");
    world.raw_config = Some(
        raw.lines()
            .map(|line| {
                if line.trim_start().starts_with("api_key =") {
                    format!("api_key = \"{credential}\"")
                } else {
                    line.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join("\n"),
    );
}

#[given(regex = r##"^the config file contains models "([^"]+)", "([^"]+)", and "([^"]+)"$"##)]
fn config_contains_models(world: &mut WatnWorld, small: String, normal: String, thinking: String) {
    let mut raw = world.raw_config.take().expect("provider config fixture");
    raw.push_str(&format!(
        "\n\n[tiers]\nsmall = \"{small}\"\nnormal = \"{normal}\"\nthinking = \"{thinking}\"\n"
    ));
    world.raw_config = Some(raw);

    let server = httpmock::MockServer::start();
    let base_url = format!("http://127.0.0.1:{}", server.port());
    world.mock_server = crate::MockServerWrap(Some(server), None);
    world
        .env_vars
        .insert("WATN_TEST_ENDPOINT_OVERRIDE".to_string(), base_url);
    world
        .pending_config
        .insert("preserve_setup_endpoint".to_string(), "true".to_string());
    world.pending_mock_model = Some("test-model".to_string());
    world.pending_mock_output = Some("output".to_string());
    world.pending_mock_returned_models = vec![small, normal, thinking];
}

#[then(regex = r##"^the provider question should show "([^"]+)" selected$"##)]
fn provider_question_shows_selected(world: &mut WatnWorld, provider: String) {
    let session = world.pty_session.as_ref().expect("setup PTY session");
    let output = pty_snapshot(session);
    let page = latest_page(&output);
    assert!(
        page.contains("Provider"),
        "provider page was not active: {page:?}"
    );
    let label = match provider.as_str() {
        "openrouter" => "OpenRouter",
        "openai" => "OpenAI",
        _ => "Custom",
    };
    assert!(page.contains(label), "selected provider missing: {page:?}");
}

#[then(regex = r##"^the completion endpoint input should show "([^"]+)"$"##)]
fn completion_endpoint_input_shows(world: &mut WatnWorld, endpoint: String) {
    let session = world.pty_session.as_mut().expect("setup PTY session");
    pty_write(session, "\r");
    let output = wait_for_active_page(session, "URL");
    assert!(
        output.contains(&endpoint),
        "endpoint was not prefilled: {output:?}"
    );
}

#[then("the credential input should remain masked")]
fn credential_input_remains_masked(world: &mut WatnWorld) {
    let session = world.pty_session.as_mut().expect("setup PTY session");
    pty_write(session, "\r");
    let output = wait_for_active_page(session, "API key");
    assert!(
        output.contains("********"),
        "credential was not masked: {output:?}"
    );
    assert!(
        !output.contains("sk-existing-key"),
        "literal credential was exposed: {output:?}"
    );
}

#[then(regex = r##"^the small model input should show "([^"]+)"$"##)]
fn small_model_input_shows(world: &mut WatnWorld, model: String) {
    let session = world.pty_session.as_mut().expect("setup PTY session");
    pty_write(session, "\r\r");
    let output = wait_for_active_page(session, "Small Model");
    assert!(
        output.contains(&model),
        "small model was not prefilled: {output:?}"
    );
}

#[then(regex = r##"^the normal model input should show "([^"]+)"$"##)]
fn normal_model_input_shows(world: &mut WatnWorld, model: String) {
    let session = world.pty_session.as_mut().expect("setup PTY session");
    pty_write(session, "\r");
    wait_for_active_page(session, "Small Reasoning");
    pty_write(session, "\r");
    let output = wait_for_active_page(session, "Normal Model");
    assert!(
        output.contains(&model),
        "normal model was not prefilled: {output:?}"
    );
}

#[then(regex = r##"^the thinking model input should show "([^"]+)"$"##)]
fn thinking_model_input_shows(world: &mut WatnWorld, model: String) {
    let session = world.pty_session.as_mut().expect("setup PTY session");
    pty_write(session, "\r");
    wait_for_active_page(session, "Normal Reasoning");
    pty_write(session, "\r");
    let output = wait_for_active_page(session, "Thinking Model");
    assert!(
        output.contains(&model),
        "thinking model was not prefilled: {output:?}"
    );
}

#[when("choose provider \"Custom\"")]
fn choose_custom_provider(world: &mut WatnWorld) {
    let session = world.pty_session.as_mut().expect("provider PTY session");
    pty_write(session, "\x1b[B\x1b[B\r");
    wait_for_active_page(session, "URL");
}

#[then("provider setup should not allow the empty endpoint")]
fn provider_setup_rejects_empty_endpoint(world: &mut WatnWorld) {
    let session = world.pty_session.as_mut().expect("provider PTY session");
    pty_write(session, "\r");
    std::thread::sleep(std::time::Duration::from_millis(150));
    let output = pty_snapshot(session);
    for word in ["endpoint", "must", "HTTP", "HTTPS", "URL"] {
        assert!(
            output.contains(word),
            "empty endpoint was accepted: {output:?}"
        );
    }
}

#[when(regex = r##"^I enter endpoint "([^"]+)"$"##)]
fn enter_setup_endpoint(world: &mut WatnWorld, endpoint: String) {
    let session = world.pty_session.as_mut().expect("provider PTY session");
    pty_write(session, &endpoint);
}

#[then("provider setup should allow the credential question")]
fn provider_setup_allows_credential_question(world: &mut WatnWorld) {
    let session = world.pty_session.as_mut().expect("provider PTY session");
    pty_write(session, "\r");
    let output = wait_for_active_page(session, "API key");
    assert!(output.contains("Where should the API key be stored?"));
}

#[given("the existing config content is recorded")]
fn record_existing_config_content(world: &mut WatnWorld) {
    world.pending_config.insert(
        "record_config_before_cancel".to_string(),
        "true".to_string(),
    );
}

#[when("cancel setup before final confirmation")]
fn cancel_setup_before_confirmation(world: &mut WatnWorld) {
    let path = world
        .temp_dir
        .as_ref()
        .expect("config temp dir")
        .path()
        .join("watn")
        .join("config.toml");
    let before = std::fs::read_to_string(&path).expect("config file before cancellation");
    world
        .pending_config
        .insert("config_before".to_string(), before);
    let session = world.pty_session.as_mut().expect("setup PTY session");
    pty_write(session, "\x1b");
    std::thread::sleep(std::time::Duration::from_millis(150));
    pty_write(session, "n");
    let session = world.pty_session.take().expect("setup PTY session");
    super::finish_pty_session(world, session);
    assert_eq!(
        world.exit_status,
        Some(1),
        "setup cancellation should be status 1"
    );
}
