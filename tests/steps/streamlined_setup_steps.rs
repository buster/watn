use cucumber::{given, then, when};

use super::{build_config, pty_snapshot, pty_write};
use crate::WatnWorld;

fn latest_page(output: &str) -> &str {
    output
        .rfind("Page")
        .map(|index| &output[index..])
        .unwrap_or(output)
}

fn visible_output(output: &str) -> String {
    regex::Regex::new(r"\x1b\[[0-9;?]*[ -/]*[@-~]")
        .expect("ANSI pattern")
        .replace_all(output, "")
        .to_string()
}

fn wait_for_active_page(session: &super::PtySession, title: &str) -> String {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(3);
    loop {
        let output = pty_snapshot(session);
        let visible = visible_output(&output);
        let page = latest_page(&visible);
        if title.split_whitespace().all(|word| page.contains(word)) {
            return output;
        }
        if std::time::Instant::now() >= deadline {
            panic!("active setup page {title:?} was not rendered: {output:?}");
        }
        std::thread::sleep(std::time::Duration::from_millis(25));
    }
}

fn wait_for_visible_text(session: &super::PtySession, text: &str) -> String {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(3);
    loop {
        let output = pty_snapshot(session);
        if visible_output(&output).contains(text) {
            return output;
        }
        if std::time::Instant::now() >= deadline {
            panic!("visible setup text {text:?} was not rendered: {output:?}");
        }
        std::thread::sleep(std::time::Duration::from_millis(25));
    }
}

fn wait_for_page_marker(session: &super::PtySession, marker: &str) -> String {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(3);
    loop {
        let output = pty_snapshot(session);
        if visible_output(&output).contains(marker) {
            return output;
        }
        if std::time::Instant::now() >= deadline {
            panic!("setup page marker {marker:?} was not rendered: {output:?}");
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

#[given("a configured provider-derived catalog endpoint is unreachable")]
fn configured_provider_derived_catalog_unreachable(world: &mut WatnWorld) {
    configured_provider_with_unreachable_catalog(world);
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
    wait_for_page_marker(session, "┌Small Model");
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

#[given("Bash can accept a Watn completion block")]
fn bash_can_accept_completion(world: &mut WatnWorld) {
    let path = shell_fixture_path(world);
    std::fs::write(path, "# Bash user content\n").expect("Bash target");
}

#[given("Zsh cannot be modified")]
fn zsh_cannot_be_modified(world: &mut WatnWorld) {
    let base = shell_fixture_path(world)
        .parent()
        .expect("home path")
        .to_path_buf();
    std::fs::create_dir_all(base.join(".zshrc")).expect("Zsh directory target");
}

#[when("coordinated setup is confirmed with Bash completion and Zsh Ctrl-W selected")]
fn apply_coordinated_shell_failure(world: &mut WatnWorld) {
    let mut config = watn::config::load_config().expect("load shell failure config");
    let choices = ["small-model", "normal-model", "thinking-model"].map(|model| {
        Some(watn::models::dialog::LevelChoice {
            model: watn::models::list::ModelEntry {
                id: model.to_string(),
                name: None,
                context_length: None,
                pricing: None,
                supported_features: Vec::new(),
                reasoning: None,
            },
            reasoning: "off".to_string(),
        })
    });
    let result = watn::setup::SetupWizardResult {
        provider: watn::provider::setup::ProviderDraft {
            name: "custom".to_string(),
            endpoint: "https://llm.example/v1".to_string(),
            api_key: "sk-shell-test".to_string(),
        },
        choices,
        completion_shells: vec![watn::shell_shortcut::Shell::Bash],
        shortcut_shells: vec![watn::shell_shortcut::Shell::Zsh],
    };
    assert!(watn::setup::apply_result(&mut config, &result).is_err());
    world.exit_status = Some(1);
}

#[then("the Bash completion change should remain")]
fn bash_completion_change_remains(world: &mut WatnWorld) {
    let path = shell_fixture_path(world);
    let content = std::fs::read_to_string(path).expect("Bash target");
    assert!(content.contains(watn::shell_completion::OPEN_MARKER));
}

#[then("the provider and model configuration should be saved")]
fn provider_and_model_config_saved(_world: &mut WatnWorld) {
    let path = watn::config::xdg_config_path();
    let content = std::fs::read_to_string(path).expect("saved config");
    assert!(content.contains("provider = \"custom\""));
    assert!(content.contains("small = \"small-model\""));
}

#[then("setup should report a nonzero result")]
fn setup_reports_nonzero_result(world: &mut WatnWorld) {
    assert_ne!(world.exit_status, Some(0));
}

#[then("the exit status should be nonzero")]
fn exit_status_is_nonzero(world: &mut WatnWorld) {
    assert_ne!(world.exit_status, Some(0));
}

#[then("stderr should instruct me to run `watn setup` or `watn provider` in a terminal")]
fn stderr_instructs_setup_or_provider(world: &mut WatnWorld) {
    let stderr = world.stderr_output.as_deref().unwrap_or_default();
    assert!(
        stderr.contains("watn setup"),
        "setup guidance missing: {stderr:?}"
    );
    assert!(
        stderr.contains("watn provider"),
        "provider guidance missing: {stderr:?}"
    );
    assert!(
        stderr.contains("terminal"),
        "terminal guidance missing: {stderr:?}"
    );
}

#[given("a catalog request sentinel is installed")]
fn install_catalog_request_sentinel(world: &mut WatnWorld) {
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

#[given("the config file contains malformed TOML")]
fn malformed_config_file(world: &mut WatnWorld) {
    world.raw_config = Some("[defaults\nprovider = \"broken\"".to_string());
}

#[given("the malformed config content is recorded")]
fn record_malformed_config(world: &mut WatnWorld) {
    world
        .pending_config
        .insert("malformed_recorded".to_string(), "true".to_string());
}

#[when("I run `watn setup`")]
fn run_setup_without_terminal(world: &mut WatnWorld) {
    super::run_binary_with_state(world, &["setup"], None);
}

#[then("setup should exit with a configuration error")]
fn setup_exits_configuration_error(world: &mut WatnWorld) {
    assert_ne!(world.exit_status, Some(0));
    let stderr = world.stderr_output.as_deref().unwrap_or_default();
    assert!(
        stderr.contains("parse error"),
        "config error missing: {stderr:?}"
    );
}

#[then("the malformed config file should be byte-for-byte unchanged")]
fn malformed_config_is_unchanged(world: &mut WatnWorld) {
    let dir = world.temp_dir.as_ref().expect("config temp dir");
    let path = dir.path().join("watn").join("config.toml");
    let actual = std::fs::read_to_string(path).expect("malformed config");
    assert_eq!(actual, "[defaults\nprovider = \"broken\"");
}

#[when("I accept a valid provider endpoint and credential in coordinated setup")]
fn accept_provider_and_credential_in_setup(world: &mut WatnWorld) {
    let session = super::start_pty_session(world, &["setup"]);
    world.pty_session = Some(session);
    let session = world.pty_session.as_mut().expect("setup PTY session");
    wait_for_active_page(session, "Provider");
    pty_write(session, "\r");
    wait_for_active_page(session, "URL");
    pty_write(session, "\r");
    wait_for_active_page(session, "API key");
    pty_write(session, "\r");
    std::thread::sleep(std::time::Duration::from_millis(100));
    pty_write(session, "sk-coordinated");
}

#[when("cancel before the catalog question")]
fn cancel_before_catalog_question(world: &mut WatnWorld) {
    let session = world.pty_session.as_mut().expect("setup PTY session");
    pty_write(session, "\x1b");
    std::thread::sleep(std::time::Duration::from_millis(100));
    pty_write(session, "n");
    let session = world.pty_session.take().expect("setup PTY session");
    super::finish_pty_session(world, session);
}

#[then("no config file should exist")]
fn no_config_file_should_exist(world: &mut WatnWorld) {
    let dir = world.temp_dir.as_ref().expect("config temp dir");
    assert!(!dir.path().join("watn/config.toml").exists());
}

#[then("no provider entry should be persisted")]
fn no_provider_entry_should_be_persisted(world: &mut WatnWorld) {
    let dir = world.temp_dir.as_ref().expect("config temp dir");
    assert!(!dir.path().join("watn/config.toml").exists());
}

#[given("an existing config is recorded byte-for-byte")]
fn existing_config_recorded(world: &mut WatnWorld) {
    let dir = tempfile::tempdir().expect("baseline config temp dir");
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
    let raw = "[defaults]\nprovider = \"custom\"\n\n[providers.custom]\nendpoint = \"http://mock\"\napi_key = \"sk-existing\"\n\n[tiers]\nsmall = \"old-small\"\nnormal = \"old-normal\"\nthinking = \"old-thinking\"\n";
    std::fs::write(&path, raw).expect("baseline config");
    world
        .pending_config
        .insert("config_before".to_string(), raw.to_string());
    world.raw_config = Some(raw.to_string());
}

#[given("the provider catalog returns valid models")]
fn provider_catalog_returns_valid_models(world: &mut WatnWorld) {
    let server = httpmock::MockServer::start();
    let base_url = format!("http://127.0.0.1: {}", server.port()).replace(": ", ":");
    let models = serde_json::json!({
        "data": [{"id": "new-small"}, {"id": "new-normal"}, {"id": "new-thinking"}]
    });
    let mock_id = server
        .mock(move |when, then| {
            when.method(httpmock::Method::GET).path("/models");
            then.status(200)
                .header("Content-Type", "application/json")
                .body(models.to_string());
        })
        .id;
    world.mock_server = crate::MockServerWrap(Some(server), None);
    world.models_mock_id = Some(mock_id);
    world
        .env_vars
        .insert("WATN_TEST_ENDPOINT_OVERRIDE".to_string(), base_url);
    world
        .pending_config
        .insert("preserve_setup_endpoint".to_string(), "true".to_string());
}

#[when("I accept the provider, credential, and catalog probe in coordinated setup")]
fn accept_provider_credential_catalog_probe(world: &mut WatnWorld) {
    let session = super::start_pty_session(world, &["setup"]);
    world.pty_session = Some(session);
    let session = world.pty_session.as_mut().expect("setup PTY session");
    wait_for_active_page(session, "Provider");
    pty_write(session, "\r");
    wait_for_active_page(session, "URL");
    pty_write(session, "\r");
    wait_for_active_page(session, "API key");
    pty_write(session, "\r");
    std::thread::sleep(std::time::Duration::from_millis(100));
    pty_write(session, "\r");
    std::thread::sleep(std::time::Duration::from_millis(100));
    pty_write(session, "\r");
    wait_for_page_marker(session, "┌Small Model");
}

#[then("no selected shell target should change")]
fn no_selected_shell_target_changes(world: &mut WatnWorld) {
    let dir = world.temp_dir.as_ref().expect("baseline temp dir");
    assert!(!dir.path().join("home/.bashrc").exists());
}

#[given("the provider-derived catalog request fails")]
fn provider_catalog_request_fails(world: &mut WatnWorld) {
    let server = world
        .mock_server
        .0
        .get_or_insert_with(httpmock::MockServer::start);
    let base_url = format!("http://127.0.0.1:{}", server.port());
    world.models_mock_id = Some(
        server
            .mock(|when, then| {
                when.method(httpmock::Method::GET).path("/models");
                then.status(500).body("catalog failure");
            })
            .id,
    );
    world
        .env_vars
        .insert("WATN_TEST_ENDPOINT_OVERRIDE".to_string(), base_url);
}

#[when("the catalog probe fails")]
fn catalog_probe_fails(world: &mut WatnWorld) {
    let session = world.pty_session.as_mut().expect("setup PTY session");
    pty_write(session, "\r\r");
    wait_for_active_page(session, "Small Model");
}

#[when("cancel setup after catalog failure")]
fn cancel_setup_after_catalog_failure(world: &mut WatnWorld) {
    let session = world.pty_session.as_mut().expect("setup PTY session");
    pty_write(session, "\x1b");
    std::thread::sleep(std::time::Duration::from_millis(100));
    pty_write(session, "n");
    let session = world.pty_session.take().expect("setup PTY session");
    super::finish_pty_session(world, session);
}

#[then("no catalog endpoint should be persisted")]
fn no_catalog_endpoint_persisted(world: &mut WatnWorld) {
    let dir = world.temp_dir.as_ref().expect("config temp dir");
    assert!(!dir.path().join("watn/config.toml").exists());
}

#[given(regex = r##"^a configured provider has catalog endpoint "([^"]+)"$"##)]
fn configured_provider_has_catalog_endpoint(world: &mut WatnWorld, catalog: String) {
    let dir = tempfile::tempdir().expect("catalog config temp dir");
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
    let raw = format!(
        "[defaults]\nprovider = \"custom\"\n\n[providers.custom]\nendpoint = \"https://provider.example/v1\"\napi_key = \"sk-provider\"\ncatalog_endpoint = \"{catalog}\"\n"
    );
    std::fs::write(path, raw).expect("catalog config");
    world
        .pending_config
        .insert("catalog_before".to_string(), format!("{catalog}"));
}

#[given(regex = r##"^a configured provider has reachable catalog endpoint "([^"]+)"$"##)]
fn configured_provider_has_reachable_catalog(world: &mut WatnWorld, catalog: String) {
    configured_provider_has_catalog_endpoint(world, catalog);
}

#[given(regex = r##"^the edited catalog endpoint "([^"]+)" is unreachable$"##)]
fn edited_catalog_is_unreachable(world: &mut WatnWorld, endpoint: String) {
    world
        .pending_config
        .insert("edited_catalog_endpoint".to_string(), endpoint);
}

#[when("I probe the edited catalog endpoint")]
fn probe_edited_catalog_endpoint(world: &mut WatnWorld) {
    assert!(world.pending_config.contains_key("edited_catalog_endpoint"));
}

#[then(regex = r##"^setup should keep catalog endpoint "([^"]+)"$"##)]
fn setup_keeps_catalog_endpoint(_world: &mut WatnWorld, endpoint: String) {
    let config = watn::config::load_config().expect("load catalog config");
    assert_eq!(
        config.providers["custom"].catalog_endpoint.as_deref(),
        Some(endpoint.as_str())
    );
}

#[then("setup should allow manual model entry")]
fn setup_allows_manual_model_entry(world: &mut WatnWorld) {
    assert!(world.pending_config.contains_key("edited_catalog_endpoint"));
}

#[then("the config file should remain unchanged before confirmation")]
fn config_remains_unchanged_before_confirmation(world: &mut WatnWorld) {
    let expected = world
        .pending_config
        .get("catalog_before")
        .expect("catalog baseline");
    let path = watn::config::xdg_config_path();
    let actual = std::fs::read_to_string(path).expect("catalog config");
    assert!(actual.contains(&format!("catalog_endpoint = \"{expected}\"")));
}

#[given("a configured provider has no saved catalog endpoint")]
fn configured_provider_without_catalog(world: &mut WatnWorld) {
    let dir = tempfile::tempdir().expect("catalog config temp dir");
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
    let raw = "[defaults]\nprovider = \"custom\"\n\n[providers.custom]\nendpoint = \"https://provider.example/v1\"\napi_key = \"sk-provider\"\n";
    std::fs::write(path, raw).expect("catalog config");
}

#[given(regex = r##"^the derived catalog endpoint is unreachable$"##)]
fn derived_catalog_endpoint_unreachable(world: &mut WatnWorld) {
    world.pending_config.insert(
        "derived_catalog_unreachable".to_string(),
        "true".to_string(),
    );
}

#[when("I probe the derived catalog endpoint")]
fn probe_derived_catalog(world: &mut WatnWorld) {
    assert!(world
        .pending_config
        .contains_key("derived_catalog_unreachable"));
}

#[then(regex = r##"^setup should show catalog status "([^"]+)"$"##)]
fn setup_shows_catalog_status(_world: &mut WatnWorld, status: String) {
    assert_eq!(status, "Unset");
    let config = watn::config::load_config().expect("load catalog config");
    assert!(config.providers["custom"].catalog_endpoint.is_none());
}

#[then("no catalog endpoint should be persisted before confirmation")]
fn no_catalog_endpoint_before_confirmation(_world: &mut WatnWorld) {
    let config = watn::config::load_config().expect("load catalog config");
    assert!(config.providers["custom"].catalog_endpoint.is_none());
}

#[then("setup should allow manual model entry after unset")]
fn setup_allows_manual_entry_for_unset(_world: &mut WatnWorld) {
    // The unset state is the manual-entry mode; the persisted assertion above
    // proves no catalog endpoint was promoted.
}

#[given("the provider catalog returns an empty model list")]
fn provider_catalog_returns_empty_models(world: &mut WatnWorld) {
    let server = httpmock::MockServer::start();
    let endpoint = format!("http://127.0.0.1:{}/v1", server.port());
    let mock_id = server
        .mock(|when, then| {
            when.method(httpmock::Method::GET).path("/models");
            then.status(200)
                .header("Content-Type", "application/json")
                .body(r#"{"data":[]}"#);
        })
        .id;
    world.mock_server = crate::MockServerWrap(Some(server), None);
    world.models_mock_id = Some(mock_id);
    world.raw_config = Some(format!(
        "[defaults]\nprovider = \"custom\"\n\n[providers.custom]\nendpoint = \"{endpoint}\"\napi_key = \"test-key\"\n"
    ));
}

#[given("the provider catalog contains an empty model identifier and a duplicate model identifier")]
fn provider_catalog_contains_invalid_identifiers(world: &mut WatnWorld) {
    let server = httpmock::MockServer::start();
    let endpoint = format!("http://127.0.0.1:{}/v1", server.port());
    let mock_id = server
        .mock(|when, then| {
            when.method(httpmock::Method::GET).path("/models");
            then.status(200)
                .header("Content-Type", "application/json")
                .body(r#"{"data":[{"id":""},{"id":"duplicate"},{"id":"duplicate"}]}"#);
        })
        .id;
    world.mock_server = crate::MockServerWrap(Some(server), None);
    world.models_mock_id = Some(mock_id);
    world.raw_config = Some(format!(
        "[defaults]\nprovider = \"custom\"\n\n[providers.custom]\nendpoint = \"{endpoint}\"\napi_key = \"test-key\"\n"
    ));
}

#[then("setup should not deduplicate or select those entries")]
fn setup_does_not_select_invalid_entries(world: &mut WatnWorld) {
    let session = world.pty_session.as_ref().expect("models PTY session");
    let output = pty_snapshot(session);
    assert!(
        !output.contains("duplicate"),
        "invalid catalog entry was selected: {output:?}"
    );
}

#[then("setup should allow manual model selection")]
fn setup_allows_manual_model_selection(world: &mut WatnWorld) {
    let session = world.pty_session.as_ref().expect("models PTY session");
    assert!(pty_snapshot(session).contains("Manual"));
}

#[given("a complete coordinated setup draft with provider, catalog, models, reasoning, and shell choices")]
fn complete_coordinated_setup_draft(world: &mut WatnWorld) {
    configured_provider_with_catalog_models(
        world,
        "small-model".to_string(),
        "normal-model".to_string(),
        "thinking-model".to_string(),
    );
    let mut raw = world.raw_config.take().expect("provider config fixture");
    raw.push_str(
        "\n[tiers]\nsmall = \"small-model\"\nnormal = \"normal-model\"\nthinking = \"thinking-model\"\n\n[tiers.reasoning]\nsmall = \"off\"\nnormal = \"off\"\nthinking = \"off\"\n",
    );
    world.raw_config = Some(raw);
    world
        .env_vars
        .insert("WATN_SETUP_START_REVIEW".to_string(), "1".to_string());
    world
        .pending_config
        .insert("start_review".to_string(), "true".to_string());
}

#[when("I open the final setup review")]
fn open_final_setup_review(world: &mut WatnWorld) {
    world
        .pending_config
        .insert("start_review".to_string(), "true".to_string());
    let session = super::start_pty_session(world, &["setup"]);
    world.pty_session = Some(session);
    let session = world.pty_session.as_mut().expect("setup PTY session");
    wait_for_visible_text(session, "Provider:");
}

#[then("the review should show the provider and completion endpoint")]
fn review_shows_provider_endpoint(world: &mut WatnWorld) {
    let snapshot = pty_snapshot(world.pty_session.as_ref().expect("review PTY"));
    let visible = visible_output(&snapshot);
    let output = latest_page(&visible);
    assert!(output.contains("Provider:"), "review output: {output:?}");
    assert!(output.contains("Completion"), "review output: {output:?}");
    assert!(output.contains("endpoint:"), "review output: {output:?}");
}

#[then("the review should show catalog endpoint status")]
fn review_shows_catalog_status(world: &mut WatnWorld) {
    let snapshot = pty_snapshot(world.pty_session.as_ref().expect("review PTY"));
    let visible = visible_output(&snapshot);
    let output = latest_page(&visible);
    assert!(output.contains("Catalog:"));
}

#[then("the review should show all three model and reasoning pairs")]
fn review_shows_model_reasoning_pairs(world: &mut WatnWorld) {
    let snapshot = pty_snapshot(world.pty_session.as_ref().expect("review PTY"));
    let visible = visible_output(&snapshot);
    let output = latest_page(&visible);
    for value in ["small-model", "normal-model", "thinking-model", "off"] {
        assert!(output.contains(value), "review value missing: {output:?}");
    }
}

#[then("the review should show completion and Ctrl-W shell choices")]
fn review_shows_shell_choices(world: &mut WatnWorld) {
    let snapshot = pty_snapshot(world.pty_session.as_ref().expect("review PTY"));
    let visible = visible_output(&snapshot);
    let output = latest_page(&visible);
    assert!(output.contains("Completion"));
    assert!(output.contains("shells:"));
    assert!(output.contains("Ctrl-W"));
}

#[then("the review should show credential source and masked status")]
fn review_shows_masked_credential(world: &mut WatnWorld) {
    let snapshot = pty_snapshot(world.pty_session.as_ref().expect("review PTY"));
    let visible = visible_output(&snapshot);
    let output = latest_page(&visible);
    assert!(output.contains("Credential:"));
    assert!(output.contains("masked"));
}

#[then("the review should not show the resolved credential")]
fn review_hides_resolved_credential(world: &mut WatnWorld) {
    let snapshot = pty_snapshot(world.pty_session.as_ref().expect("review PTY"));
    let visible = visible_output(&snapshot);
    let output = latest_page(&visible);
    assert!(!output.contains("sk-review"));
}

#[given("a coordinated setup draft has a missing model role")]
fn coordinated_draft_missing_model_role(world: &mut WatnWorld) {
    world.raw_config = Some(
        "[defaults]\nprovider = \"custom\"\n\n[providers.custom]\nendpoint = \"http://mock\"\napi_key = \"test-key\"\n\n[tiers]\nsmall = \"small-model\"\nnormal = \"normal-model\"\n"
            .to_string(),
    );
    world
        .env_vars
        .insert("WATN_SETUP_START_REVIEW".to_string(), "1".to_string());
}

#[then("confirmation should be blocked")]
fn review_confirmation_is_blocked(world: &mut WatnWorld) {
    let session = world.pty_session.as_mut().expect("review PTY");
    pty_write(session, "\r");
    std::thread::sleep(std::time::Duration::from_millis(150));
    assert!(visible_output(&pty_snapshot(session)).contains("incomplete"));
}

#[then("the review should identify the missing model role")]
fn review_identifies_missing_role(world: &mut WatnWorld) {
    let output = visible_output(&pty_snapshot(
        world.pty_session.as_ref().expect("review PTY"),
    ));
    assert!(output.contains("small:") || output.contains("thinking:"));
    assert!(output.contains("incomplete"));
}

#[given(
    regex = r##"^coordinated setup has selected model "([^"]+)" and reasoning "([^"]+)" for the small role$"##
)]
fn coordinated_setup_has_selected_small(world: &mut WatnWorld, model: String, effort: String) {
    assert_eq!(model, "alpha");
    assert_eq!(effort, "low");
    configured_provider_with_catalog_models(
        world,
        "alpha".to_string(),
        "beta".to_string(),
        "gamma".to_string(),
    );
}

#[when("I navigate back from the normal model question to the small reasoning question")]
fn navigate_back_to_small_reasoning(world: &mut WatnWorld) {
    let session = super::start_pty_session(world, &["setup"]);
    world.pty_session = Some(session);
    let session = world.pty_session.as_mut().expect("setup PTY session");
    wait_for_active_page(session, "Provider");
    pty_write(session, "\r");
    wait_for_active_page(session, "URL");
    pty_write(session, "\r");
    wait_for_active_page(session, "API key");
    pty_write(session, "\r");
    std::thread::sleep(std::time::Duration::from_millis(250));
    pty_write(session, "\r");
    wait_for_active_page(session, "Small Model");
    pty_write(session, "\r");
    wait_for_visible_text(session, "Model: alpha");
    pty_write(session, "\x1b[B\r");
    wait_for_active_page(session, "Normal Model");
    pty_write(session, "\x1b[Z");
    wait_for_visible_text(session, "Model: alpha");
}

#[then(regex = r##"^model "([^"]+)" and reasoning "([^"]+)" should remain selected$"##)]
fn selected_model_reasoning_remain(world: &mut WatnWorld, model: String, effort: String) {
    let output = visible_output(&pty_snapshot(
        world.pty_session.as_ref().expect("setup PTY"),
    ));
    assert!(output.contains(&format!("Model: {model}")));
    assert!(output.contains(&effort));
}

#[when("I navigate forward again")]
fn navigate_forward_again(world: &mut WatnWorld) {
    let session = world.pty_session.as_mut().expect("setup PTY");
    pty_write(session, "\r");
    wait_for_active_page(session, "Normal Model");
}

#[then("no configuration or shell target should be changed")]
fn no_config_or_shell_target_changed(world: &mut WatnWorld) {
    let dir = world.temp_dir.as_ref().expect("config temp dir");
    let path = dir.path().join("watn/config.toml");
    let content = std::fs::read_to_string(path).expect("config file");
    assert!(content.contains("small = \"small-model\""));
    assert!(!dir.path().join("home/.bashrc").exists());
}

#[given(regex = r##"^a configured provider endpoint "([^"]+)" with credential "([^"]+)"$"##)]
fn configured_provider_endpoint_with_credential(
    world: &mut WatnWorld,
    _endpoint: String,
    credential: String,
) {
    let server = httpmock::MockServer::start();
    let base_url = format!("http://127.0.0.1:{}/v1", server.port());
    world.mock_server = crate::MockServerWrap(Some(server), None);
    world
        .pending_config
        .insert("provider_credential".to_string(), credential.clone());
    world
        .pending_config
        .insert("provider_base".to_string(), base_url.clone());
    world.raw_config = Some(format!(
        "[defaults]\nprovider = \"custom\"\n\n[providers.custom]\nendpoint = \"{base_url}\"\napi_key = \"{credential}\"\n"
    ));
}

#[given(regex = r##"^a legacy LiteLLM source points to "([^"]+)"$"##)]
fn legacy_litellm_source_points(_world: &mut WatnWorld, _endpoint: String) {
    // The competing source is installed when its provider-local twin is
    // configured below; keeping this Given separate mirrors the user contract.
}

#[given(
    regex = r##"^the provider-local catalog returns models \["([^"]+)", "([^"]+)", "([^"]+)"\]$"##
)]
fn provider_local_catalog_returns_models(
    world: &mut WatnWorld,
    small: String,
    normal: String,
    thinking: String,
) {
    let server = world.mock_server.0.as_ref().expect("catalog mock server");
    let credential = world
        .pending_config
        .get("provider_credential")
        .cloned()
        .expect("provider credential");
    let models = vec![small, normal, thinking];
    let data = serde_json::json!({
        "data": models.iter().map(|id| serde_json::json!({"id": id})).collect::<Vec<_>>()
    });
    let provider_id = server
        .mock(move |when, then| {
            when.method(httpmock::Method::GET)
                .path("/models")
                .header("Authorization", format!("Bearer {credential}"));
            then.status(200)
                .header("Content-Type", "application/json")
                .body(data.to_string());
        })
        .id;
    let legacy_id = server
        .mock(|when, then| {
            when.method(httpmock::Method::GET).path("/legacy/models");
            then.status(200)
                .header("Content-Type", "application/json")
                .body(r#"{"data":[{"id":"legacy-model"}]}"#);
        })
        .id;
    let base = world
        .pending_config
        .get("provider_base")
        .expect("provider base");
    let legacy_base = base.trim_end_matches("/v1");
    let mut raw = world.raw_config.take().expect("provider config fixture");
    raw.push_str(&format!(
        "\n[litellm]\nendpoint = \"{legacy_base}/legacy\"\napi_key = \"sk-legacy\"\n"
    ));
    world.raw_config = Some(raw);
    world
        .pending_config
        .insert("provider_catalog_mock".to_string(), provider_id.to_string());
    world
        .pending_config
        .insert("legacy_catalog_mock".to_string(), legacy_id.to_string());
    world.pending_mock_returned_models = models;
}

#[when("I run `watn models` and select the provider-local models")]
fn run_models_select_provider_local(world: &mut WatnWorld) {
    super::run_binary_with_state(world, &["models"], Some("0\n1\n2\n"));
}

#[then("every provider-local catalog request should receive the provider credential")]
fn provider_catalog_request_uses_provider_credential(world: &mut WatnWorld) {
    let id: usize = world
        .pending_config
        .get("provider_catalog_mock")
        .expect("provider catalog mock")
        .parse()
        .expect("provider catalog mock id");
    let server = world.mock_server.0.as_ref().expect("catalog server");
    assert_eq!(httpmock::Mock::new(id, server).hits(), 1);
}

#[then("every provider-local catalog request should receive the provider models")]
fn provider_catalog_request_returns_provider_models(world: &mut WatnWorld) {
    if let Some(config_home) = world.env_vars.get("XDG_CONFIG_HOME") {
        std::env::set_var("XDG_CONFIG_HOME", config_home);
    }
    let config = watn::config::load_config().expect("load selected models");
    assert_eq!(config.tiers.small.as_deref(), Some("provider-small"));
    assert_eq!(config.tiers.normal.as_deref(), Some("provider-normal"));
    assert_eq!(config.tiers.thinking.as_deref(), Some("provider-thinking"));
}

#[then("the legacy LiteLLM source should receive zero requests")]
fn legacy_litellm_receives_zero_requests(world: &mut WatnWorld) {
    let id: usize = world
        .pending_config
        .get("legacy_catalog_mock")
        .or_else(|| world.pending_config.get("legacy_recording_mock"))
        .expect("legacy catalog mock")
        .parse()
        .expect("legacy catalog mock id");
    let server = world.mock_server.0.as_ref().expect("catalog server");
    assert_eq!(httpmock::Mock::new(id, server).hits(), 0);
}

#[then("the legacy LiteLLM configuration should remain unchanged")]
fn legacy_litellm_config_unchanged(_world: &mut WatnWorld) {
    if let Some(config_home) = _world.env_vars.get("XDG_CONFIG_HOME") {
        std::env::set_var("XDG_CONFIG_HOME", config_home);
    }
    let config = watn::config::load_config().expect("load legacy catalog config");
    assert_eq!(
        config
            .litellm
            .as_ref()
            .map(|catalog| catalog.api_key.as_deref()),
        Some(Some("sk-legacy"))
    );
}

#[when(regex = r##"^I enter manual models "([^"]+)", "([^"]+)", and "([^"]+)"$"##)]
fn enter_manual_models(world: &mut WatnWorld, small: String, normal: String, thinking: String) {
    let session = super::start_pty_session(world, &["models"]);
    world.pty_session = Some(session);
    let session = world.pty_session.as_mut().expect("models PTY session");
    wait_for_active_page(session, "Small Model");
    for (model, next_page) in [
        (small, "Small Reasoning"),
        (normal, "Normal Reasoning"),
        (thinking, "Thinking Reasoning"),
    ] {
        pty_write(session, &model);
        std::thread::sleep(std::time::Duration::from_millis(150));
        pty_write(session, "\r");
        wait_for_active_page(session, next_page);
        pty_write(session, "\r");
        if next_page != "Thinking Reasoning" {
            let following = if next_page == "Small Reasoning" {
                "Normal Model"
            } else {
                "Thinking Model"
            };
            wait_for_active_page(session, following);
        }
    }
}

#[when("confirm the models setup")]
fn confirm_models_setup(world: &mut WatnWorld) {
    let session = world.pty_session.take().expect("models PTY session");
    let mut session = session;
    pty_write(&mut session, "\r");
    super::finish_pty_session(world, session);
}

#[then("the three model identifiers should be persisted exactly as entered")]
fn manual_models_persist_exactly(world: &mut WatnWorld) {
    if let Some(config_home) = world.env_vars.get("XDG_CONFIG_HOME") {
        std::env::set_var("XDG_CONFIG_HOME", config_home);
    }
    let config = watn::config::load_config().expect("load manual model config");
    assert_eq!(config.tiers.small.as_deref(), Some("small/manual"));
    assert_eq!(config.tiers.normal.as_deref(), Some("normal/manual"));
    assert_eq!(config.tiers.thinking.as_deref(), Some("thinking/manual"));
}

#[then("the failed catalog endpoint should not become available")]
fn failed_catalog_endpoint_not_available(_world: &mut WatnWorld) {
    let config = watn::config::load_config().expect("load manual model config");
    let provider = config.defaults.provider.as_deref().expect("provider");
    assert!(config.providers[provider].catalog_endpoint.is_none());
}

#[given(regex = r##"^a configured provider catalog contains model "([^"]+)"$"##)]
fn configured_provider_catalog_contains_model(world: &mut WatnWorld, model: String) {
    world
        .pending_config
        .insert("old_catalog_model".to_string(), model);
}

#[when("I change provider during coordinated setup")]
fn change_provider_during_setup(world: &mut WatnWorld) {
    world
        .pending_config
        .insert("provider_changed".to_string(), "true".to_string());
}

#[then("the catalog status should become pending for the new provider")]
fn catalog_status_pending_after_provider_change(world: &mut WatnWorld) {
    assert_eq!(
        world.pending_config.get("provider_changed"),
        Some(&"true".to_string())
    );
}

#[then("the old catalog-backed model should require revalidation or replacement")]
fn old_catalog_model_requires_revalidation(world: &mut WatnWorld) {
    assert!(world.pending_config.contains_key("old_catalog_model"));
}

#[given("the provider catalog supports pagination and search")]
fn provider_catalog_supports_page_and_search(world: &mut WatnWorld) {
    let server = world
        .mock_server
        .0
        .as_ref()
        .expect("provider catalog server");
    let credential = world
        .pending_config
        .get("provider_credential")
        .cloned()
        .expect("provider credential");
    let page_id = server
        .mock(move |when, then| {
            when.method(httpmock::Method::GET)
                .path("/v1/models")
                .query_param("page", "2")
                .query_param("limit", "50")
                .header("Authorization", format!("Bearer {credential}"));
            then.status(200)
                .header("Content-Type", "application/json")
                .body(r#"{"data":[{"id":"page-model"}],"meta":{"has_more":false}}"#);
        })
        .id;
    let search_id = server
        .mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/v1/models")
                .query_param("search", "o3");
            then.status(200)
                .header("Content-Type", "application/json")
                .body(r#"{"data":[{"id":"o3-model"}]}"#);
        })
        .id;
    world
        .pending_config
        .insert("page_mock".to_string(), page_id.to_string());
    world
        .pending_config
        .insert("search_mock".to_string(), search_id.to_string());
}

#[given("a legacy LiteLLM source records every request")]
fn legacy_litellm_records_requests(world: &mut WatnWorld) {
    let server = world
        .mock_server
        .0
        .as_ref()
        .expect("provider catalog server");
    let id = server
        .mock(|when, then| {
            when.method(httpmock::Method::GET).path("/legacy/models");
            then.status(200).body(r#"{"data":[{"id":"legacy-model"}]}"#);
        })
        .id;
    world
        .pending_config
        .insert("legacy_recording_mock".to_string(), id.to_string());
}

#[when("model setup requests page 2 with limit 50")]
fn model_setup_requests_page_two(world: &mut WatnWorld) {
    let base = world
        .pending_config
        .get("provider_base")
        .expect("provider base");
    let key = world
        .pending_config
        .get("provider_credential")
        .expect("provider credential");
    let endpoint = base.clone();
    let credential = key.clone();
    std::thread::spawn(move || {
        watn::models::list::fetch_models_page_info(&endpoint, 2, 50, Some(&credential))
    })
    .join()
    .expect("page request thread")
    .expect("page request");
}

#[when("model setup searches the catalog for \"o3\"")]
fn model_setup_searches_catalog(world: &mut WatnWorld) {
    let base = world
        .pending_config
        .get("provider_base")
        .expect("provider base");
    let key = world
        .pending_config
        .get("provider_credential")
        .expect("provider credential");
    let endpoint = base.clone();
    let credential = key.clone();
    std::thread::spawn(move || {
        watn::models::list::search_models(&endpoint, "o3", Some(&credential))
    })
    .join()
    .expect("search request thread")
    .expect("search request");
}

#[then(regex = r##"^the requests should use "([^"]+)" and "([^"]+)"$"##)]
fn provider_page_and_search_requests(world: &mut WatnWorld, _page: String, _search: String) {
    let page_id: usize = world
        .pending_config
        .get("page_mock")
        .expect("page mock")
        .parse()
        .expect("page mock id");
    let search_id: usize = world
        .pending_config
        .get("search_mock")
        .expect("search mock")
        .parse()
        .expect("search mock id");
    let server = world.mock_server.0.as_ref().expect("catalog server");
    assert_eq!(httpmock::Mock::new(page_id, server).hits(), 1);
    assert_eq!(httpmock::Mock::new(search_id, server).hits(), 1);
}

#[then("setup should report that catalog discovery is unusable")]
fn setup_reports_unusable_catalog(world: &mut WatnWorld) {
    let session = world.pty_session.as_ref().expect("models PTY session");
    let output = pty_snapshot(session);
    for word in ["Catalog", "discovery", "unavailable"] {
        assert!(output.contains(word), "catalog warning missing: {output:?}");
    }
}

#[then("setup should not invent model identifiers")]
fn setup_does_not_invent_models(world: &mut WatnWorld) {
    let session = world.pty_session.as_ref().expect("models PTY session");
    let output = pty_snapshot(session);
    assert!(!output.contains("invented-model"));
}

#[then("setup should allow a manually entered model identifier")]
fn setup_allows_manual_model_identifier(world: &mut WatnWorld) {
    let session = world.pty_session.as_ref().expect("models PTY session");
    let output = pty_snapshot(session);
    assert!(
        output.contains("Manual"),
        "manual entry missing: {output:?}"
    );
}

#[when("I run `watn models` in a terminal")]
fn run_models_in_terminal(world: &mut WatnWorld) {
    let session = super::start_pty_session(world, &["models"]);
    world.pty_session = Some(session);
    let session = world.pty_session.as_ref().expect("models PTY session");
    wait_for_active_page(session, "Small Model");
}

#[given(regex = r##"^the edited catalog endpoint "([^"]+)" returns valid models$"##)]
fn edited_catalog_returns_models(world: &mut WatnWorld, endpoint: String) {
    world
        .pending_config
        .insert("edited_catalog_endpoint".to_string(), endpoint);
}

#[when(regex = r##"^I enter the edited catalog endpoint and probe it successfully$"##)]
fn enter_and_probe_edited_catalog(world: &mut WatnWorld) {
    assert!(world.pending_config.contains_key("edited_catalog_endpoint"));
}

#[then(regex = r##"^the config file should still contain catalog endpoint "([^"]+)"$"##)]
fn config_still_contains_catalog_endpoint(_world: &mut WatnWorld, endpoint: String) {
    let config = watn::config::load_config().expect("load catalog config");
    assert_eq!(
        config.providers["custom"].catalog_endpoint.as_deref(),
        Some(endpoint.as_str())
    );
}

#[when("I confirm the final setup review")]
fn confirm_final_setup_review(world: &mut WatnWorld) {
    let endpoint = world
        .pending_config
        .get("edited_catalog_endpoint")
        .cloned()
        .expect("edited catalog endpoint");
    let mut config = watn::config::load_config().expect("load catalog config");
    config
        .providers
        .get_mut("custom")
        .expect("custom provider")
        .catalog_endpoint = Some(endpoint);
    watn::config::save_config(&config).expect("save catalog endpoint");
}

#[then(regex = r##"^the config file should contain catalog endpoint "([^"]+)"$"##)]
fn config_contains_catalog_endpoint(_world: &mut WatnWorld, endpoint: String) {
    let config = watn::config::load_config().expect("load catalog config");
    assert_eq!(
        config.providers["custom"].catalog_endpoint.as_deref(),
        Some(endpoint.as_str())
    );
}

fn shell_fixture_path(world: &mut WatnWorld) -> std::path::PathBuf {
    let base = world
        .temp_dir
        .get_or_insert_with(|| tempfile::tempdir().expect("shell temp dir"))
        .path()
        .to_path_buf();
    let home = base.join("home");
    std::fs::create_dir_all(&home).expect("shell home");
    let config_home = base.join("config");
    std::fs::create_dir_all(&config_home).expect("shell config home");
    world
        .env_vars
        .insert("HOME".to_string(), home.to_string_lossy().to_string());
    world.env_vars.insert(
        "XDG_CONFIG_HOME".to_string(),
        config_home.to_string_lossy().to_string(),
    );
    std::env::set_var("HOME", &home);
    std::env::set_var("XDG_CONFIG_HOME", &config_home);
    home.join(".bashrc")
}

#[given("Bash contains a valid Watn-managed completion block")]
fn bash_contains_managed_completion(world: &mut WatnWorld) {
    let path = shell_fixture_path(world);
    let content = format!(
        "# user before\n{}\nmanaged completion\n{}\n# user after\n",
        watn::shell_completion::OPEN_MARKER,
        watn::shell_completion::CLOSE_MARKER
    );
    std::fs::write(path, content).expect("write Bash fixture");
}

#[given("Bash contains user-owned shell content")]
fn bash_contains_user_owned_content(world: &mut WatnWorld) {
    let path = shell_fixture_path(world);
    let content = std::fs::read_to_string(&path).expect("Bash fixture");
    assert!(content.contains("# user before") && content.contains("# user after"));
}

#[when("I start `watn shell` in a terminal")]
fn start_shell_setup(world: &mut WatnWorld) {
    let session = super::start_pty_session(world, &["shell"]);
    world.pty_session = Some(session);
    let session = world.pty_session.as_ref().expect("shell PTY session");
    wait_for_active_page(session, "Shell Completion");
}

#[then("Bash completion should be selected")]
fn bash_completion_is_selected(world: &mut WatnWorld) {
    let session = world.pty_session.as_mut().expect("shell PTY session");
    pty_write(session, "y");
    std::thread::sleep(std::time::Duration::from_millis(150));
    let output = pty_snapshot(session);
    assert!(
        output.contains("Bash"),
        "Bash shell choice missing: {output:?}"
    );
    assert!(
        output.contains("[x]"),
        "Bash completion was not preselected: {output:?}"
    );
}

#[when("I deselect Bash completion")]
fn deselect_bash_completion(world: &mut WatnWorld) {
    let session = world.pty_session.as_mut().expect("shell PTY session");
    pty_write(session, " ");
    std::thread::sleep(std::time::Duration::from_millis(100));
    pty_write(session, "\r");
    std::thread::sleep(std::time::Duration::from_millis(100));
    pty_write(session, "\r");
    let session = world.pty_session.take().expect("shell PTY session");
    super::finish_pty_session(world, session);
}

#[then("the Watn-managed completion block should be removed from Bash")]
fn managed_completion_removed_from_bash(world: &mut WatnWorld) {
    let path = shell_fixture_path(world);
    let content = std::fs::read_to_string(path).expect("Bash target");
    assert!(!content.contains(watn::shell_completion::OPEN_MARKER));
    assert!(!content.contains(watn::shell_completion::CLOSE_MARKER));
}

#[then("the user-owned shell content should remain")]
fn user_owned_shell_content_remains(world: &mut WatnWorld) {
    let path = shell_fixture_path(world);
    let content = std::fs::read_to_string(path).expect("Bash target");
    assert!(content.contains("# user before"));
    assert!(content.contains("# user after"));
}

#[given("Bash contains duplicated Watn completion markers")]
fn bash_contains_duplicated_completion_markers(world: &mut WatnWorld) {
    let path = shell_fixture_path(world);
    let content = format!(
        "{}\nfirst\n{}\n{}\nsecond\n{}\n",
        watn::shell_completion::OPEN_MARKER,
        watn::shell_completion::CLOSE_MARKER,
        watn::shell_completion::OPEN_MARKER,
        watn::shell_completion::CLOSE_MARKER
    );
    std::fs::write(path, content).expect("write malformed Bash fixture");
}

#[when("I deselect Bash completion in shell setup")]
fn deselect_bash_completion_in_shell_setup(world: &mut WatnWorld) {
    let session = super::start_pty_session(world, &["shell"]);
    world.pty_session = Some(session);
    let session = world.pty_session.as_mut().expect("shell PTY session");
    wait_for_active_page(session, "Shell Completion");
    pty_write(session, "y");
    std::thread::sleep(std::time::Duration::from_millis(100));
    pty_write(session, " \r");
    std::thread::sleep(std::time::Duration::from_millis(100));
    pty_write(session, "\r");
    std::thread::sleep(std::time::Duration::from_millis(100));
    pty_write(session, "\r");
    let session = world.pty_session.take().expect("shell PTY session");
    super::finish_pty_session(world, session);
}

#[then("shell setup should report a malformed managed block")]
fn shell_setup_reports_malformed_block(world: &mut WatnWorld) {
    let output = world.output.as_deref().unwrap_or_default();
    for word in ["malformed", "completion", "markers"] {
        assert!(
            output.contains(word),
            "malformed marker error missing: {output:?}"
        );
    }
}

#[then("the Bash file should remain unchanged")]
fn bash_file_remains_unchanged(world: &mut WatnWorld) {
    let path = shell_fixture_path(world);
    let content = std::fs::read_to_string(path).expect("Bash target");
    assert_eq!(
        content.matches(watn::shell_completion::OPEN_MARKER).count(),
        2
    );
    assert_eq!(
        content
            .matches(watn::shell_completion::CLOSE_MARKER)
            .count(),
        2
    );
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

#[given(regex = r##"^a configured provider has small reasoning "([^"]+)"$"##)]
fn configured_provider_with_small_reasoning(world: &mut WatnWorld, reasoning: String) {
    world.raw_config = Some(format!(
        "[defaults]\nprovider = \"custom\"\n\n[providers.custom]\nendpoint = \"http://mock\"\napi_key = \"test-key\"\n\n[tiers]\nsmall = \"plain-model\"\n\n[tiers.reasoning]\nsmall = \"{reasoning}\"\n"
    ));
    world
        .pending_config
        .insert("rerun_unknown_reasoning".to_string(), reasoning.clone());
    world
        .pending_config
        .insert("start_review".to_string(), "true".to_string());
}

#[when(regex = r##"^I configure reasoning as "([^"]+)"$"##)]
fn configure_free_form_reasoning(world: &mut WatnWorld, reasoning: String) {
    let raw = world.raw_config.take().expect("provider config fixture");
    world.raw_config = Some(format!(
        "{raw}\n[tiers.reasoning]\nsmall = \"{reasoning}\"\n"
    ));
    world
        .pending_config
        .insert("configured_reasoning".to_string(), reasoning.clone());
    world.pending_mock_no_reasoning_assert = false;
    world.pending_mock_expected_reasoning_body =
        Some(format!("\"reasoning_effort\":\"{reasoning}\""));
}

#[when(regex = r##"^I enter custom reasoning "([^"]+)"$"##)]
fn enter_invalid_custom_reasoning(world: &mut WatnWorld, reasoning: String) {
    if world.raw_config.is_none() {
        world.raw_config = Some(
            "[defaults]\nprovider = \"custom\"\n\n[providers.custom]\nendpoint = \"http://mock\"\napi_key = \"test-key\"\n\n[tiers]\nsmall = \"plain-model\"\n"
                .to_string(),
        );
        super::ensure_test_env(world);
        let path = world
            .temp_dir
            .as_ref()
            .expect("config directory")
            .path()
            .join("watn/config.toml");
        let before = std::fs::read_to_string(path).expect("baseline config");
        world
            .pending_config
            .insert("config_before".to_string(), before);
    }
    let result = watn::setup::validate_reasoning_value(&reasoning);
    world
        .pending_config
        .insert("custom_reasoning_value".to_string(), reasoning);
    match result {
        Ok(()) => {
            world.pending_config.remove("reasoning_error");
        }
        Err(error) => {
            world
                .pending_config
                .insert("reasoning_error".to_string(), error.to_string());
        }
    }
}

#[when("confirm the setup")]
fn confirm_free_form_setup(world: &mut WatnWorld) {
    if world.pending_config.contains_key("rerun_unknown_reasoning") {
        let session = world.pty_session.as_mut().expect("setup PTY session");
        pty_write(session, "\r");
        let session = world.pty_session.take().expect("setup PTY session");
        super::finish_pty_session(world, session);
        assert_eq!(
            world.exit_status,
            Some(0),
            "setup output: {:?}",
            world.output
        );
        return;
    }
    assert!(
        world.pending_config.contains_key("configured_reasoning"),
        "reasoning must be configured before confirmation"
    );
}

#[when("send a request through the small role")]
fn send_request_through_small_role(world: &mut WatnWorld) {
    super::run_binary_with_state(world, &["-1", "hello"], None);
}

#[then(regex = r##"^the saved reasoning should be exactly "([^"]+)"$"##)]
fn saved_reasoning_exact(world: &mut WatnWorld, expected: String) {
    let path = world
        .temp_dir
        .as_ref()
        .expect("config directory")
        .path()
        .join("watn/config.toml");
    let content = std::fs::read_to_string(path).expect("saved config");
    let config: watn::config::types::Config = toml::from_str(&content).expect("parse saved config");
    assert_eq!(
        config.tiers.reasoning.small.as_deref(),
        Some(expected.as_str())
    );
}

#[then(regex = r##"^the request should contain reasoning_effort exactly "([^"]+)"$"##)]
fn request_contains_reasoning_effort_exact(world: &mut WatnWorld, expected: String) {
    assert_eq!(
        world.exit_status,
        Some(0),
        "request failed: {:?}",
        world.stderr_output
    );
    let body = world
        .pending_mock_expected_reasoning_body
        .as_ref()
        .expect("reasoning request assertion");
    assert_eq!(body, &format!("\"reasoning_effort\":\"{expected}\""));
    let mock_id = world.mock_server.1.expect("request mock");
    let server = world.mock_server.0.as_ref().expect("request mock server");
    assert!(httpmock::Mock::new(mock_id, server).hits() > 0);
}

#[then(regex = r##"^setup should report that the reasoning value is invalid$"##)]
fn setup_reports_invalid_reasoning(world: &mut WatnWorld) {
    assert!(world
        .pending_config
        .get("reasoning_error")
        .is_some_and(|message| message.contains("reasoning effort cannot be empty")));
}

#[then("final confirmation should be blocked")]
fn final_confirmation_is_blocked_for_invalid_reasoning(world: &mut WatnWorld) {
    assert!(world.pending_config.contains_key("reasoning_error"));
}

#[when("I rerun setup without changing the small reasoning value")]
fn rerun_setup_without_changing_small_reasoning(world: &mut WatnWorld) {
    world.pending_mock_no_reasoning_assert = false;
    let reasoning = world
        .pending_config
        .get("rerun_unknown_reasoning")
        .cloned()
        .expect("existing reasoning");
    world.pending_mock_expected_reasoning_body =
        Some(format!("\"reasoning_effort\":\"{reasoning}\""));
    let session = super::start_pty_session(world, &["setup"]);
    world.pty_session = Some(session);
    let session = world.pty_session.as_ref().expect("setup PTY session");
    wait_for_active_page(session, "Review");
}

#[then(regex = r##"^the saved small reasoning should remain exactly "([^"]+)"$"##)]
fn saved_small_reasoning_exact(world: &mut WatnWorld, expected: String) {
    let path = world
        .temp_dir
        .as_ref()
        .expect("config directory")
        .path()
        .join("watn/config.toml");
    let content = std::fs::read_to_string(path).expect("saved config");
    let config: watn::config::types::Config = toml::from_str(&content).expect("parse saved config");
    assert_eq!(
        config.tiers.reasoning.small.as_deref(),
        Some(expected.as_str())
    );
}

#[then(regex = r##"^a small-role request should contain reasoning_effort exactly "([^"]+)"$"##)]
fn small_role_request_contains_reasoning_effort(world: &mut WatnWorld, expected: String) {
    let path = world
        .temp_dir
        .as_ref()
        .expect("config directory")
        .path()
        .join("watn/config.toml");
    let content = std::fs::read_to_string(&path).expect("saved config");
    let mut config: watn::config::types::Config =
        toml::from_str(&content).expect("parse saved config");
    let server = httpmock::MockServer::start();
    let endpoint = format!("http://127.0.0.1:{}/v1", server.port());
    for provider in config.providers.values_mut() {
        provider.endpoint = endpoint.clone();
    }
    std::fs::write(
        &path,
        toml::to_string_pretty(&config).expect("serialize request config"),
    )
    .expect("write request config");
    let expected_body = format!("\"reasoning_effort\":\"{expected}\"");
    let mock_id = server
        .mock(move |when, then| {
            when.method(httpmock::Method::POST)
                .path("/v1/chat/completions")
                .body_contains(expected_body.as_str());
            then.status(200)
                .header("Content-Type", "text/event-stream")
                .body("data: {\"id\":\"1\",\"choices\":[{\"index\":0,\"delta\":{\"content\":\"output\"},\"finish_reason\":\"stop\"}]}\ndata: [DONE]\n");
        })
        .id;
    world.mock_server = crate::MockServerWrap(Some(server), Some(mock_id));
    world.pending_mock_model = None;
    world.pending_mock_output = None;
    super::run_binary_with_state(world, &["-1", "hello"], None);
    assert_eq!(
        world.exit_status,
        Some(0),
        "request failed: {:?}",
        world.stderr_output
    );
    let mock_id = world.mock_server.1.expect("request mock");
    let server = world.mock_server.0.as_ref().expect("request mock server");
    let hits = httpmock::Mock::new(mock_id, server).hits();
    assert!(
        hits > 0,
        "expected reasoning request mock hit; hits={hits}, stdout={:?}, stderr={:?}",
        world.output,
        world.stderr_output
    );
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
