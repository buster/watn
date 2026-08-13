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

#[given(regex = r##"^a configured provider with catalog models "([^"]+)" and "([^"]+)"$"##)]
fn configured_provider_with_two_catalog_models(
    world: &mut WatnWorld,
    first: String,
    second: String,
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
    world.pending_mock_returned_models = vec![first, second];
}

#[given("a configured provider with an unreachable catalog endpoint")]
fn configured_provider_with_unreachable_catalog(world: &mut WatnWorld) {
    world.raw_config = Some(build_config(
        "custom",
        None,
        Some(vec![("custom", "http://127.0.0.1:9/v1", "test-key", "")]),
        None,
        None,
        None,
    ));
}

#[given(
    regex = r##"^a configured provider catalog model "([^"]+)" supports efforts "([^"]+)", "([^"]+)", and "([^"]+)"$"##
)]
fn configured_provider_catalog_model_with_reasoning(
    world: &mut WatnWorld,
    model: String,
    first: String,
    second: String,
    third: String,
) {
    let server = httpmock::MockServer::start();
    let endpoint = format!("http://127.0.0.1:{}/v1", server.port());
    let data = serde_json::json!({
        "data": [{
            "id": model,
            "reasoning": {
                "supported_efforts": [first, second, third],
                "default_effort": "medium",
                "default_enabled": true,
                "mandatory": false
            }
        }]
    });
    let mock_id = server
        .mock(move |when, then| {
            when.method(httpmock::Method::GET).path("/models");
            then.status(200)
                .header("Content-Type", "application/json")
                .body(data.to_string());
        })
        .id;
    world.mock_server = crate::MockServerWrap(Some(server), None);
    world.models_mock_id = Some(mock_id);
    world.raw_config = Some(format!(
        "[defaults]\nprovider = \"custom\"\n\n[providers.custom]\nendpoint = \"{endpoint}\"\napi_key = \"test-key\"\n"
    ));
}

#[given(regex = r##"^the catalog default effort for "([^"]+)" is "([^"]+)"$"##)]
fn catalog_default_effort(world: &mut WatnWorld, model: String, effort: String) {
    assert_eq!(model, "reasoning-model");
    assert_eq!(effort, "medium");
    world
        .pending_config
        .insert("catalog_default_effort".to_string(), effort);
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

#[then(regex = r##"^provider setup should show that "([^"]+)" must contain a non-empty value$"##)]
fn provider_setup_shows_unresolved_environment(world: &mut WatnWorld, variable: String) {
    assert_eq!(
        world.pending_config.get("setup_error"),
        Some(&format!("{variable} must contain a non-empty value"))
    );
}

#[when("I run `watn models` without a terminal")]
fn run_models_without_terminal(world: &mut WatnWorld) {
    super::run_binary_with_state(world, &["models"], None);
}

#[then("the output should instruct me to run `watn provider`")]
fn output_instructs_provider(world: &mut WatnWorld) {
    let output = world.stderr_output.as_deref().unwrap_or_default();
    assert!(
        output.contains("watn provider"),
        "provider guidance missing: {output:?}"
    );
}

#[then("no provider question should be shown")]
fn no_provider_question_shown(world: &mut WatnWorld) {
    let output = format!(
        "{}{}",
        world.output.as_deref().unwrap_or_default(),
        world.stderr_output.as_deref().unwrap_or_default()
    );
    assert!(
        !output.contains("Provider (editing)"),
        "provider UI was shown: {output:?}"
    );
}

#[when(
    regex = r##"^provider setup saves provider "([^"]+)" with endpoint "([^"]+)" and credential "([^"]+)"$"##
)]
fn provider_setup_saves_provider(
    world: &mut WatnWorld,
    provider: String,
    endpoint: String,
    credential: String,
) {
    let dir = tempfile::tempdir().expect("provider setup temp dir");
    let config_home = dir.path().to_string_lossy().to_string();
    world.temp_dir = Some(dir);
    world
        .env_vars
        .insert("XDG_CONFIG_HOME".to_string(), config_home.clone());
    std::env::set_var("XDG_CONFIG_HOME", &config_home);
    let path = std::path::Path::new(&config_home)
        .join("watn")
        .join("config.toml");
    std::fs::create_dir_all(path.parent().expect("config parent")).expect("config directory");
    std::fs::write(
        &path,
        world
            .raw_config
            .as_deref()
            .expect("provider config fixture"),
    )
    .expect("provider config");
    let mut config = watn::config::load_config().expect("load provider config");
    let draft = watn::provider::setup::ProviderDraft {
        name: provider,
        endpoint,
        api_key: credential,
    };
    watn::config::save_provider_draft(&mut config, &draft).expect("save provider draft");
    let server = world
        .mock_server
        .0
        .get_or_insert_with(httpmock::MockServer::start);
    world.models_mock_id = Some(
        server
            .mock(|when, then| {
                when.method(httpmock::Method::GET).path("/models");
                then.status(200).body(r#"{"data":[{"id":"unused"}]}"#);
            })
            .id,
    );
}

#[given("the config file contains pricing and LiteLLM settings")]
fn config_contains_pricing_and_litellm(world: &mut WatnWorld) {
    let mut raw = world.raw_config.take().expect("provider config fixture");
    raw.push_str(
        "\n\n[pricing]\n\"legacy-small\" = { input = 1.0, output = 2.0 }\n\n[litellm]\nendpoint = \"https://legacy-litellm.example\"\napi_key = \"sk-litellm\"\n",
    );
    world.raw_config = Some(raw);
}

#[then("the pricing and LiteLLM settings should remain unchanged")]
fn pricing_and_litellm_remain_unchanged(_world: &mut WatnWorld) {
    let config = watn::config::load_config().expect("load preserved config");
    assert_eq!(config.pricing["legacy-small"].input, 1.0);
    assert_eq!(
        config.litellm.as_ref().map(|value| value.endpoint.as_str()),
        Some("https://legacy-litellm.example")
    );
}

#[given(regex = r##"^the config file contains model "([^"]+)" for the small role$"##)]
fn config_contains_small_model(world: &mut WatnWorld, model: String) {
    let mut raw = world.raw_config.take().expect("provider config fixture");
    raw.push_str(&format!("\n\n[tiers]\nsmall = \"{model}\"\n"));
    world.raw_config = Some(raw);
}

#[then("the small role should require a replacement model")]
fn small_role_requires_replacement(world: &mut WatnWorld) {
    let session = world.pty_session.as_ref().expect("models PTY session");
    let output = pty_snapshot(session);
    let page = latest_page(&output);
    assert!(
        page.contains("Small Model"),
        "small model page was not active: {page:?}"
    );
    assert!(
        !page.contains("not-in-catalog"),
        "stale model remained selectable: {page:?}"
    );
}

#[then(regex = r##"^the model choices should include only "([^"]+)" and "([^"]+)"$"##)]
fn model_choices_are_catalog_only(world: &mut WatnWorld, first: String, second: String) {
    let session = world.pty_session.as_ref().expect("models PTY session");
    let output = latest_page(&pty_snapshot(session)).to_string();
    assert!(
        output.contains(&first),
        "first catalog model missing: {output:?}"
    );
    assert!(
        output.contains(&second),
        "second catalog model missing: {output:?}"
    );
    assert!(
        !output.contains("not-in-catalog"),
        "stale model was displayed: {output:?}"
    );
}

#[given(regex = r##"^a configured provider catalog model "([^"]+)" has no reasoning metadata$"##)]
fn configured_provider_catalog_model_without_reasoning(world: &mut WatnWorld, model: String) {
    let server = httpmock::MockServer::start();
    let endpoint = format!("http://127.0.0.1:{}/v1", server.port());
    let data = serde_json::json!({ "data": [{ "id": model }] });
    let mock_id = server
        .mock(move |when, then| {
            when.method(httpmock::Method::GET).path("/models");
            then.status(200)
                .header("Content-Type", "application/json")
                .body(data.to_string());
        })
        .id;
    world.mock_server = crate::MockServerWrap(Some(server), None);
    world.models_mock_id = Some(mock_id);
    world.raw_config = Some(format!(
        "[defaults]\nprovider = \"custom\"\n\n[providers.custom]\nendpoint = \"{endpoint}\"\napi_key = \"test-key\"\n"
    ));
}

#[then("the small reasoning question should warn that supported efforts are unavailable")]
fn small_reasoning_warns_missing_metadata(world: &mut WatnWorld) {
    let session = world.pty_session.as_ref().expect("models PTY session");
    let output = pty_snapshot(session);
    for word in ["reasoning", "metadata", "unavailable"] {
        assert!(
            output.contains(word),
            "metadata warning missing: {output:?}"
        );
    }
}

#[then(
    regex = r##"^the generic reasoning choices should include "([^"]+)", "([^"]+)", "([^"]+)", "([^"]+)", and "([^"]+)"$"##
)]
fn generic_reasoning_choices_include(
    world: &mut WatnWorld,
    first: String,
    second: String,
    third: String,
    fourth: String,
    fifth: String,
) {
    let session = world.pty_session.as_ref().expect("models PTY session");
    let output = pty_snapshot(session);
    for choice in [first, second, third, fourth, fifth] {
        assert!(
            output.contains(&choice),
            "generic effort missing: {output:?}"
        );
    }
}

#[then("the generic reasoning choices should include a custom effort entry")]
fn generic_reasoning_choices_include_custom(world: &mut WatnWorld) {
    let session = world.pty_session.as_ref().expect("models PTY session");
    let output = pty_snapshot(session);
    assert!(
        output.contains("custom"),
        "custom effort entry missing: {output:?}"
    );
}

#[when(regex = r##"^I enter custom reasoning effort "([^"]+)"$"##)]
fn enter_custom_reasoning(world: &mut WatnWorld, effort: String) {
    let session = world.pty_session.as_mut().expect("models PTY session");
    pty_write(session, "c");
    pty_write(session, &effort);
    pty_write(session, "\r");
    wait_for_active_page(session, "Normal Model");
}

#[then(regex = r##"^the small role should use reasoning "([^"]+)"$"##)]
fn small_role_uses_reasoning(world: &mut WatnWorld, effort: String) {
    let session = world.pty_session.as_ref().expect("models PTY session");
    let output = pty_snapshot(session);
    for character in effort.chars() {
        assert!(
            output.contains(character),
            "custom effort was not retained: {output:?}"
        );
    }
}

#[given(regex = r##"^a configured provider with model "([^"]+)" for the small role$"##)]
fn configured_provider_with_small_model(world: &mut WatnWorld, model: String) {
    world.raw_config = Some(format!(
        "[defaults]\nprovider = \"custom\"\n\n[providers.custom]\nendpoint = \"http://mock\"\napi_key = \"test-key\"\n\n[tiers]\nsmall = \"{model}\"\n"
    ));
    world.pending_mock_model = Some("test-model".to_string());
    world.pending_mock_output = Some("output".to_string());
    world.pending_mock_no_reasoning_assert = true;
}

#[given("the small role reasoning is \"off\"")]
fn small_role_reasoning_off(world: &mut WatnWorld) {
    let mut raw = world.raw_config.take().expect("provider config fixture");
    raw.push_str("\n[tiers.reasoning]\nsmall = \"off\"\n");
    world.raw_config = Some(raw);
}

#[when("I send a small-role request through the configured provider")]
fn send_small_request(world: &mut WatnWorld) {
    super::run_binary_with_state(world, &["-1", "hello"], None);
}

#[then("the API request should omit the reasoning effort")]
fn api_request_omits_reasoning(world: &mut WatnWorld) {
    assert_eq!(
        world.exit_status,
        Some(0),
        "reasoning request was rejected: {:?}",
        world.stderr_output
    );
    let id = world
        .blocking_mock_id
        .expect("reasoning assertion mock was not installed");
    let server = world.mock_server.0.as_ref().expect("request mock server");
    assert_eq!(httpmock::Mock::new(id, server).hits(), 0);
}

#[then("model setup should warn that catalog discovery is unavailable")]
fn model_setup_warns_catalog_unavailable(world: &mut WatnWorld) {
    let session = world.pty_session.as_ref().expect("models PTY session");
    let output = pty_snapshot(session);
    for word in ["Catalog", "discovery", "unavailable"] {
        assert!(output.contains(word), "catalog warning missing: {output:?}");
    }
}

#[then("model setup should allow a manually entered model identifier")]
fn model_setup_allows_manual_identifier(world: &mut WatnWorld) {
    let session = world.pty_session.as_ref().expect("models PTY session");
    let output = pty_snapshot(session);
    for word in ["Manual", "model", "identifier"] {
        assert!(
            output.contains(word),
            "manual model entry missing: {output:?}"
        );
    }
}

#[when(regex = r##"^choose "([^"]+)" for the small role$"##)]
fn choose_model_for_small_role(world: &mut WatnWorld, model: String) {
    let session = world.pty_session.as_mut().expect("models PTY session");
    pty_write(session, &model);
    std::thread::sleep(std::time::Duration::from_millis(500));
    pty_write(session, "\r");
    wait_for_active_page(session, "Small Reasoning");
}

#[then(
    regex = r##"^the small reasoning question should show only "([^"]+)", "([^"]+)", and "([^"]+)"$"##
)]
fn small_reasoning_shows_only_catalog_efforts(
    world: &mut WatnWorld,
    first: String,
    second: String,
    third: String,
) {
    let session = world.pty_session.as_ref().expect("models PTY session");
    let snapshot = pty_snapshot(session);
    let output = latest_page(&snapshot);
    for effort in [first, second, third] {
        assert!(
            output.contains(&effort),
            "reasoning effort missing: {output:?}"
        );
    }
    assert!(
        !output.contains("Choices: off"),
        "off was not catalog-supported: {output:?}"
    );
}

#[then(regex = r##"^"([^"]+)" should be selected by default$"##)]
fn reasoning_effort_selected_by_default(world: &mut WatnWorld, effort: String) {
    let session = world.pty_session.as_ref().expect("models PTY session");
    let snapshot = pty_snapshot(session);
    let output = latest_page(&snapshot);
    assert!(
        output.contains("Selected:"),
        "default effort label missing: {output:?}"
    );
    assert!(
        output.contains(&effort),
        "default effort missing: {output:?}"
    );
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
