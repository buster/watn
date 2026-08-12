//! PTY steps for the reviewed setup-refactoring scenarios.

use cucumber::{given, then, when};

use super::{finish_pty_session, pty_snapshot, pty_write, resize_pty_session, start_pty_session};
use crate::WatnWorld;

#[given("no recognized credential environment variable is set")]
fn no_recognized_credential_environment(world: &mut WatnWorld) {
    for name in [
        "OPENROUTER_API_KEY",
        "WATN_API_KEY",
        "WATN_OPENAI_API_KEY",
        "OPENAI_API_KEY",
    ] {
        world.env_vars.remove(name);
        std::env::remove_var(name);
    }
}

#[given("a setup draft with an active Provider endpoint field")]
fn setup_draft_with_provider_endpoint(world: &mut WatnWorld) {
    let directory = tempfile::tempdir().expect("isolated config directory");
    world.env_vars.insert(
        "XDG_CONFIG_HOME".to_string(),
        directory.path().to_string_lossy().to_string(),
    );
    world.temp_dir = Some(directory);
    world.pending_mock_no_config_file = true;
}

#[when("I render `watn setup` in a wide terminal")]
fn render_setup_wide(world: &mut WatnWorld) {
    render_setup_at_layout(world, 120);
}

#[when("I render `watn setup` in a narrow terminal")]
fn render_setup_narrow(world: &mut WatnWorld) {
    render_setup_at_layout(world, 80);
}

fn render_setup_at_layout(world: &mut WatnWorld, cols: u16) {
    let session = start_pty_session(world, &["setup"]);
    resize_pty_session(&session, cols, 40);
    world.pty_session = Some(session);
}

#[then("the active-setting help should explain what the endpoint is")]
fn help_explains_endpoint(world: &mut WatnWorld) {
    let session = world.pty_session.as_ref().expect("setup PTY session");
    let output = wait_for_fragments(session, &["What", "endpoint"]);
    assert!(output.contains("What"));
}

#[then("the active-setting help should explain what it enables")]
fn help_explains_enablement(world: &mut WatnWorld) {
    let session = world.pty_session.as_ref().expect("setup PTY session");
    let output = wait_for_fragments(session, &["enables"]);
    assert!(output.contains("enables"));
}

#[then("the active-setting help should include a recommendation")]
fn help_includes_recommendation(world: &mut WatnWorld) {
    let session = world.pty_session.as_ref().expect("setup PTY session");
    let output = wait_for_fragments(session, &["Recommendation"]);
    assert!(output.contains("Recommendation"));
}

#[then("the active-setting help should include a requirement or tradeoff")]
fn help_includes_requirement(world: &mut WatnWorld) {
    let session = world.pty_session.as_ref().expect("setup PTY session");
    let output = wait_for_fragments(session, &["Requirement", "tradeoff"]);
    assert!(output.contains("Requirement"));
}

#[then(regex = r##"^the help should appear (beside|below) the settings$"##)]
fn help_placement(world: &mut WatnWorld, placement: String) {
    let session = world.pty_session.as_ref().expect("setup PTY session");
    let output = wait_for_fragments(session, &[&placement]);
    assert!(output.contains(&placement));
}

#[then("the Provider topic should present \"OPENROUTER_API_KEY\" and \"WATN_API_KEY\" as separate choices")]
fn multiple_credential_choices(world: &mut WatnWorld) {
    let session = world.pty_session.as_ref().expect("setup PTY session");
    let output = wait_for_fragments(session, &["OPENROUTER_API_KEY", "WATN_API_KEY"]);
    assert!(
        output.matches("detected").count() >= 2,
        "output: {output:?}"
    );
}

#[then("the Provider topic should not select either detected credential automatically")]
fn multiple_credentials_not_selected(world: &mut WatnWorld) {
    let session = world.pty_session.as_ref().expect("setup PTY session");
    let output = pty_snapshot(session);
    assert!(
        !output.contains("> OPENROUTER_API_KEY"),
        "output: {output:?}"
    );
    assert!(!output.contains("> WATN_API_KEY"), "output: {output:?}");
}

#[then(regex = r##"^the setup terminal should not contain \"([^\"]+)\" or \"([^\"]+)\"$"##)]
fn setup_terminal_has_no_secrets(world: &mut WatnWorld, first: String, second: String) {
    let session = world.pty_session.as_ref().expect("setup PTY session");
    let output = pty_snapshot(session);
    assert!(!output.contains(&first), "setup leaked first secret");
    assert!(!output.contains(&second), "setup leaked second secret");
}

#[then(regex = r##"^the setup terminal should not contain \"([^\"]+)\"$"##)]
fn setup_terminal_has_no_secret(world: &mut WatnWorld, secret: String) {
    let session = world.pty_session.as_ref().expect("setup PTY session");
    assert!(
        !pty_snapshot(session).contains(&secret),
        "setup leaked secret"
    );
}

#[then(
    regex = r##"^the setup wizard should show topics \"Provider\", \"Model roles\", \"Shell integration\", and \"Review\"$"##
)]
fn setup_topics_visible(world: &mut WatnWorld) {
    let session = world.pty_session.as_ref().expect("setup PTY session");
    let output = wait_for_fragments(
        session,
        &[
            "Provider",
            "Model",
            "roles",
            "Shell",
            "integration",
            "Review",
        ],
    );
    assert!(output.contains("Provider"));
    assert!(output.contains("Review"));
}

#[then(
    regex = r##"^the Provider topic should identify \"OPENROUTER_API_KEY\" as \"Detected from environment\"$"##
)]
fn detected_provider_credential(world: &mut WatnWorld) {
    let session = world.pty_session.as_ref().expect("setup PTY session");
    let output = wait_for_fragments(session, &["OPENROUTER_API_KEY", "Detected"]);
    assert!(output.contains("OPENROUTER_API_KEY"));
    assert!(output.contains("Detected"));
}

#[then(
    regex = r##"^the Provider topic should show required settings \"Endpoint\" and \"Credential source\"$"##
)]
fn provider_required_settings(world: &mut WatnWorld) {
    let session = world.pty_session.as_ref().expect("setup PTY session");
    let output = wait_for_fragments(session, &["1.", "2."]);
    assert!(output.contains("1."), "endpoint step missing: {output:?}");
    assert!(output.contains("2."), "credential step missing: {output:?}");
}

#[when("I accept the detected credential and complete the required model roles")]
fn accept_detected_and_complete_roles(world: &mut WatnWorld) {
    let session = world.pty_session.as_mut().expect("setup PTY session");
    wait_for_fragments(session, &["OPENROUTER_API_KEY"]);
    advance_setup_page(session, "\r", ">> ACTIVE Endpoint");
    advance_setup_page(session, "\r", ">> ACTIVE Credential source");
    advance_setup_page(session, "\r", ">> ACTIVE Credential value");
    pty_write(session, "\r");
    wait_for_catalog_models(session, &["small-model", "normal-model", "thinking-model"]);
    advance_setup_page(session, "\r", "> 2. Balanced / normal");
    advance_setup_page(session, "\r", "> 3. Thinking");
    advance_setup_page(session, "\r", "Completion in Bash");
    advance_setup_page(session, "\r", "Review draft before Finish setup");
    let output = pty_snapshot(session);
    assert!(
        output.contains("Roles"),
        "role checklist missing: {output:?}"
    );
    assert!(
        output.contains("Reasoning"),
        "reasoning panel missing: {output:?}"
    );
    assert!(
        output.contains("small-model"),
        "model list missing: {output:?}"
    );
}

#[then("the Model roles topic should show three roles, the model catalog, and reasoning controls")]
fn model_roles_guidance_visible(world: &mut WatnWorld) {
    let session = world.pty_session.as_ref().expect("setup PTY session");
    let output = wait_for_fragments(session, &["Roles", "Catalog:", "Reasoning"]);
    for model in ["small-model", "normal-model", "thinking-model"] {
        assert!(output.contains(model), "missing model {model}: {output:?}");
    }
    assert!(
        output.contains("Ctrl-R"),
        "reasoning control missing: {output:?}"
    );
}

fn advance_setup_page(session: &mut super::PtySession, key: &str, fragment: &str) {
    pty_write(session, key);
    wait_for_fragments(session, &[fragment]);
    std::thread::sleep(std::time::Duration::from_millis(150));
}

fn wait_for_catalog_models(session: &super::PtySession, models: &[&str]) {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
    loop {
        let output = pty_snapshot(session);
        if models.iter().all(|model| output.contains(model)) {
            return;
        }
        if std::time::Instant::now() >= deadline {
            panic!("catalog models were not rendered: {output:?}");
        }
        std::thread::sleep(std::time::Duration::from_millis(25));
    }
}

#[when("I finish setup from Review")]
fn finish_setup_from_review(world: &mut WatnWorld) {
    let session = world.pty_session.as_mut().expect("setup PTY session");
    pty_write(session, "\r");
    let session = world.pty_session.take().expect("setup PTY session");
    let terminal_output = finish_pty_session(world, session);
    world.stderr_output = Some(terminal_output);
    world.output = Some(String::new());
}

#[given("a legacy commented config template exists")]
fn legacy_commented_config(world: &mut WatnWorld) {
    let directory = tempfile::tempdir().expect("isolated config directory");
    let config_directory = directory.path().join("watn");
    std::fs::create_dir_all(&config_directory).expect("config directory");
    std::fs::write(
        config_directory.join("config.toml"),
        "# watn configuration file\n# [defaults]\n# provider = \"openrouter\"\n",
    )
    .expect("commented config");
    world.env_vars.insert(
        "XDG_CONFIG_HOME".to_string(),
        directory.path().to_string_lossy().to_string(),
    );
    world.temp_dir = Some(directory);
    world.raw_config =
        Some("# watn configuration file\n# [defaults]\n# provider = \"openrouter\"\n".to_string());
}

#[then("first-run setup should not start solely because the existing file has no active settings")]
fn existing_comment_file_does_not_start_setup(world: &mut WatnWorld) {
    let session = world.pty_session.as_ref().expect("interactive request PTY");
    let output = pty_snapshot(session);
    assert!(!output.contains("No watn configuration found"));
    assert!(!output.contains("Setup topics"));
}

#[then("the original chat completion request should be sent")]
fn original_chat_request_sent(world: &mut WatnWorld) {
    let session = world.pty_session.take().expect("interactive request PTY");
    finish_pty_session(world, session);
    let mock_id = world
        .pending_config
        .get("implicit_chat_mock")
        .expect("implicit chat mock")
        .parse::<usize>()
        .expect("mock id");
    let server = world.mock_server.0.as_ref().expect("mock server");
    assert!(httpmock::Mock::new(mock_id, server).hits() > 0);
    assert_eq!(world.exit_status, Some(0));
}

#[when(regex = r##"^I choose the Custom provider and enter credential variable \"([^\"]+)\"$"##)]
fn choose_custom_provider_and_credential(world: &mut WatnWorld, variable: String) {
    let session = world.pty_session.as_mut().expect("setup PTY session");
    pty_write(session, "\x1b[B\x1b[B\r");
    pty_write(session, "http://127.0.0.1:1\r");
    pty_write(session, "e");
    pty_write(session, &variable);
    pty_write(session, "\r");
    std::thread::sleep(std::time::Duration::from_millis(250));
}

#[given(
    regex = r##"^a complete config has model roles \"([^\"]+)\", \"([^\"]+)\", and \"([^\"]+)\"$"##
)]
fn complete_config_with_roles(
    world: &mut WatnWorld,
    small: String,
    normal: String,
    thinking: String,
) {
    let server = httpmock::MockServer::start();
    let base = format!("http://127.0.0.1:{}", server.port());
    world.mock_server = crate::MockServerWrap(Some(server), None);
    world
        .env_vars
        .insert("WATN_TEST_ENDPOINT_OVERRIDE".to_string(), base);
    world.raw_config = Some(format!(
        "[defaults]\nprovider = \"custom\"\n\n[providers.custom]\nendpoint = \"http://localhost:4000\"\napi_key = \"test-key\"\n\n[tiers]\nsmall = \"{small}\"\nnormal = \"{normal}\"\nthinking = \"{thinking}\"\n"
    ));
    world.pending_mock_returned_models = vec![
        "new-small".to_string(),
        "new-normal".to_string(),
        "new-thinking".to_string(),
    ];
}

#[given(regex = r##"^the configured provider catalog returns models \[([^\]]+)\]$"##)]
fn configured_catalog_models(world: &mut WatnWorld, models: String) {
    world.pending_mock_returned_models = models
        .split(',')
        .map(|model| model.trim().trim_matches('"').to_string())
        .collect();
}

#[given(regex = r##"^the ephemeral catalog returns models \[([^\]]+)\] for \"([^\"]+)\"$"##)]
fn ephemeral_catalog_returns_models(world: &mut WatnWorld, models: String, path: String) {
    assert_eq!(path, "/models");
    let ids = models
        .split(',')
        .map(|model| model.trim().trim_matches('"').to_string())
        .collect::<Vec<_>>();
    let server = httpmock::MockServer::start();
    let base = format!("http://127.0.0.1:{}", server.port());
    let mock_id = server
        .mock(|when, then| {
            when.method(httpmock::Method::GET).path("/models");
            let data = ids
                .iter()
                .map(|id| serde_json::json!({"id": id}))
                .collect::<Vec<_>>();
            then.status(200)
                .header("Content-Type", "application/json")
                .body(serde_json::json!({"data": data}).to_string());
        })
        .id;
    world.mock_server = crate::MockServerWrap(Some(server), None);
    world.models_mock_id = Some(mock_id);
    world
        .env_vars
        .insert("WATN_TEST_ENDPOINT_OVERRIDE".to_string(), base);
}

#[given("the model catalog transport fails for \"/models\"")]
fn catalog_transport_fails(world: &mut WatnWorld) {
    world.env_vars.insert(
        "WATN_TEST_ENDPOINT_OVERRIDE".to_string(),
        "http://127.0.0.1:1".to_string(),
    );
}

#[given("the model catalog response is delayed")]
fn delayed_model_catalog_response(world: &mut WatnWorld) {
    let root = world.temp_dir.as_ref().expect("isolated config directory");
    let bash = root.path().join(".bashrc");
    let zsh = root.path().join(".zshrc");
    let fish = root.path().join("fish/config.fish");
    std::fs::create_dir_all(fish.parent().expect("fish parent")).expect("fish directory");
    std::fs::write(&bash, "shell before\n").expect("bash startup file");
    std::fs::write(&zsh, "zsh before\n").expect("zsh startup file");
    std::fs::write(&fish, "fish before\n").expect("fish startup file");
    for (key, path) in [
        ("bash_before", &bash),
        ("zsh_before", &zsh),
        ("fish_before", &fish),
    ] {
        world.pending_config.insert(
            key.to_string(),
            std::fs::read_to_string(path).expect("shell snapshot"),
        );
    }
    let home = root.path().to_string_lossy().to_string();
    world.env_vars.insert("HOME".to_string(), home);
    world
        .env_vars
        .insert("SHELL".to_string(), "/bin/bash".to_string());
    world
        .env_vars
        .insert("OPENROUTER_API_KEY".to_string(), "sk-delay-key".to_string());
    let server = httpmock::MockServer::start();
    let base = format!("http://127.0.0.1:{}", server.port());
    let mock_id = server
        .mock(|when, then| {
            when.method(httpmock::Method::GET).path("/models");
            then.delay(std::time::Duration::from_secs(3))
                .status(200)
                .header("Content-Type", "application/json")
                .body(r#"{"data":[{"id":"delayed-small"},{"id":"delayed-normal"},{"id":"delayed-thinking"}]}"#);
        })
        .id;
    world.mock_server = crate::MockServerWrap(Some(server), None);
    world.models_mock_id = Some(mock_id);
    world
        .env_vars
        .insert("WATN_TEST_ENDPOINT_OVERRIDE".to_string(), base);
}

#[when("I start `watn setup` in a terminal and press Ctrl-C during discovery")]
fn start_setup_and_interrupt_discovery(world: &mut WatnWorld) {
    let mut session = start_pty_session(world, &["setup"]);
    wait_for_fragments(&session, &["Provider"]);
    pty_write(&mut session, "\r");
    wait_for_fragments(&session, &[">> ACTIVE Endpoint"]);
    pty_write(&mut session, "\r");
    wait_for_fragments(&session, &["Credential source"]);
    pty_write(&mut session, "\r");
    wait_for_fragments(&session, &["Credential value"]);
    pty_write(&mut session, "\r");
    wait_for_fragments(&session, &["Catalog:"]);
    pty_write(&mut session, "\x03");
    finish_pty_session(world, session);
}

#[then("no shell startup file should be changed")]
fn no_shell_startup_file_changed(world: &mut WatnWorld) {
    let root = world.temp_dir.as_ref().expect("isolated config directory");
    for (key, path) in [
        ("bash_before", root.path().join(".bashrc")),
        ("zsh_before", root.path().join(".zshrc")),
        ("fish_before", root.path().join("fish/config.fish")),
    ] {
        let before = world.pending_config.get(key).expect("shell snapshot");
        let after = std::fs::read_to_string(path).expect("shell startup file");
        assert_eq!(before, &after, "startup file changed: {key}");
    }
}

#[when("I provide a valid custom endpoint and credential source")]
fn provide_custom_endpoint_and_credential(world: &mut WatnWorld) {
    let session = world.pty_session.as_mut().expect("setup PTY session");
    pty_write(session, "\x1b[B\x1b[B\r");
    pty_write(session, "http://127.0.0.1:1\r");
    pty_write(session, "px");
    pty_write(session, "\r");
    std::thread::sleep(std::time::Duration::from_millis(250));
}

#[when(regex = r##"^I enter manual model IDs \"([^\"]+)\", \"([^\"]+)\", and \"([^\"]+)\"$"##)]
fn enter_manual_model_ids(world: &mut WatnWorld, small: String, normal: String, thinking: String) {
    let session = world.pty_session.as_mut().expect("setup PTY session");
    for model in [small, normal, thinking] {
        pty_write(session, &model);
        pty_write(session, "\r");
        std::thread::sleep(std::time::Duration::from_millis(75));
    }
    pty_write(session, "\r");
    wait_for_fragments(session, &["Review", "Unverified"]);
}

#[then("each manual model role should show reasoning \"off\"")]
fn manual_roles_reasoning_off(world: &mut WatnWorld) {
    let session = world.pty_session.as_ref().expect("setup PTY session");
    let output = wait_for_fragments(session, &["Reasoning", "off"]);
    assert!(
        output.matches("Reasoning").count() >= 3,
        "output: {output:?}"
    );
}

#[then("Review should show an unverified catalog warning")]
fn review_unverified_warning(world: &mut WatnWorld) {
    let session = world.pty_session.as_ref().expect("setup PTY session");
    let output = wait_for_fragments(session, &["Unverified", "catalog"]);
    assert!(output.contains("Unverified"));
}

#[then("Finish setup should be available")]
fn finish_setup_available(world: &mut WatnWorld) {
    let session = world.pty_session.as_ref().expect("setup PTY session");
    let output = wait_for_fragments(session, &["Finish", "available"]);
    assert!(!output.contains("blocked"));
}

#[when("I finish setup")]
fn finish_setup(world: &mut WatnWorld) {
    let session = world.pty_session.as_mut().expect("setup PTY session");
    pty_write(session, "\r");
    let session = world.pty_session.take().expect("setup PTY session");
    finish_pty_session(world, session);
    assert_eq!(
        world.exit_status,
        Some(0),
        "setup output: {:?}",
        world.output
    );
}

#[then("the config file should contain the three manual model roles")]
fn config_contains_manual_roles(world: &mut WatnWorld) {
    let directory = world.temp_dir.as_ref().expect("config directory");
    let content =
        std::fs::read_to_string(directory.path().join("watn/config.toml")).expect("config file");
    for model in ["manual-small", "manual-normal", "manual-thinking"] {
        assert!(content.contains(model), "missing {model}: {content}");
    }
}

#[then("the config file should contain reasoning \"off\" for each manual role")]
fn config_manual_roles_reasoning_off(world: &mut WatnWorld) {
    let directory = world.temp_dir.as_ref().expect("config directory");
    let content =
        std::fs::read_to_string(directory.path().join("watn/config.toml")).expect("config file");
    assert_eq!(content.matches("= \"off\"").count(), 3, "config: {content}");
}

#[when("I complete Provider and Model roles in `watn setup`")]
fn complete_provider_and_model_roles(world: &mut WatnWorld) {
    world
        .env_vars
        .insert("OPENROUTER_API_KEY".to_string(), "pconfig-key".to_string());
    if world.pty_session.is_none() {
        let session = start_pty_session(world, &["setup"]);
        world.pty_session = Some(session);
    }
    {
        let session = world.pty_session.as_ref().expect("setup PTY session");
        wait_for_fragments(session, &["Provider", "openrouter"]);
    }
    accept_detected_and_complete_roles(world);
}

#[then("Review should summarize the endpoint, credential source, model roles, reasoning, shell changes, and warnings")]
fn review_summarizes_draft(world: &mut WatnWorld) {
    let session = world.pty_session.as_ref().expect("setup PTY session");
    let output = wait_for_fragments(
        session,
        &[
            "Provider",
            "Credential",
            "Small",
            "Reasoning",
            "Shell",
            "Warning",
        ],
    );
    assert!(output.contains("small-model"));
}

#[when("I discard setup from Review")]
fn discard_setup_from_review(world: &mut WatnWorld) {
    let session = world.pty_session.as_mut().expect("setup PTY session");
    pty_write(session, "\x1b d");
    let session = world.pty_session.take().expect("setup PTY session");
    finish_pty_session(world, session);
    assert_eq!(world.exit_status, Some(1));
}

#[given("the config contains known tiers, reasoning, pricing, and LiteLLM settings")]
fn config_contains_known_setup_settings(world: &mut WatnWorld) {
    let raw = world
        .raw_config
        .take()
        .expect("existing provider config")
        .replace(
            "provider = \"custom\"",
            "provider = \"custom\"\nmodel = \"legacy-default\"",
        )
        .replace(
            "api_key = \"sk-old-key\"",
            "api_key = \"sk-old-key\"\ndefault_model = \"legacy-provider-default\"",
        );
    world.raw_config = Some(format!(
        "{raw}\n[tiers]\nsmall = \"legacy-small\"\nnormal = \"legacy-normal\"\nthinking = \"legacy-thinking\"\n\n[tiers.reasoning]\nsmall = \"off\"\nnormal = \"medium\"\nthinking = \"high\"\n\n[pricing]\n\"legacy-small\" = {{ input = 1.0, output = 2.0 }}\n\n[litellm]\nendpoint = \"https://legacy-litellm.example\"\napi_key = \"sk-litellm\"\n"
    ));
}

#[then("the Provider topic should prefill supported saved values as \"Loaded from config\"")]
fn provider_prefills_loaded_values(world: &mut WatnWorld) {
    let session = world.pty_session.as_ref().expect("setup PTY session");
    let output = wait_for_fragments(session, &["Loaded", "config"]);
    assert!(output.contains("Loaded"));
}

#[when("I edit the draft and cancel setup")]
fn edit_and_cancel_setup(world: &mut WatnWorld) {
    let directory = world.temp_dir.as_ref().expect("config directory");
    let path = directory.path().join("watn/config.toml");
    let before = std::fs::read_to_string(&path).expect("existing config");
    world
        .pending_config
        .insert("before_bytes".to_string(), before);
    let session = world.pty_session.as_mut().expect("setup PTY session");
    pty_write(session, "x\x1bd");
    let session = world.pty_session.take().expect("setup PTY session");
    finish_pty_session(world, session);
    assert_eq!(world.exit_status, Some(1));
}

#[given("shell startup files contain user content and existing watn completion and shortcut marker blocks")]
fn shell_marker_fixture(world: &mut WatnWorld) {
    let root = tempfile::tempdir().expect("shell fixture directory");
    let config_path = root.path().join("watn/config.toml");
    std::fs::create_dir_all(config_path.parent().expect("config parent")).expect("config parent");
    let raw = "[defaults]\nprovider = \"custom\"\n\n[providers.custom]\nendpoint = \"http://127.0.0.1:1\"\napi_key = \"test-key\"\n\n[tiers]\nsmall = \"small-model\"\nnormal = \"normal-model\"\nthinking = \"thinking-model\"\n";
    std::fs::write(&config_path, raw).expect("setup config");
    let bash = format!(
        "user before\n{}\nold completion\n{}\nuser after\n",
        watn::shell_completion::OPEN_MARKER,
        watn::shell_completion::CLOSE_MARKER
    );
    let zsh = format!(
        "zsh before\n{}\nold shortcut\n{}\nzsh after\n",
        watn::shell_shortcut::OPEN_MARKER,
        watn::shell_shortcut::CLOSE_MARKER
    );
    std::fs::write(root.path().join(".bashrc"), &bash).expect("bash fixture");
    std::fs::write(root.path().join(".zshrc"), &zsh).expect("zsh fixture");
    std::fs::write(root.path().join(".bashrc.before"), &bash).expect("bash snapshot");
    std::fs::write(root.path().join(".zshrc.before"), &zsh).expect("zsh snapshot");
    world.env_vars.insert(
        "HOME".to_string(),
        root.path().to_string_lossy().to_string(),
    );
    world.env_vars.insert(
        "XDG_CONFIG_HOME".to_string(),
        root.path().to_string_lossy().to_string(),
    );
    world
        .env_vars
        .insert("SHELL".to_string(), "/bin/bash".to_string());
    world.temp_dir = Some(root);
    world.raw_config = Some(raw.to_string());
}

#[then("Shell integration should derive its selections from the marker blocks")]
fn shell_selection_from_markers(world: &mut WatnWorld) {
    let root = world.temp_dir.as_ref().expect("shell fixture");
    let environment = watn::shell_shortcut::ShellEnvironment {
        home: root.path().to_path_buf(),
        xdg_config_home: Some(root.path().to_path_buf()),
        shell: Some("/bin/bash".to_string()),
    };
    assert_eq!(
        watn::shell_completion::marker_state(watn::shell_shortcut::Shell::Bash, &environment)
            .expect("completion marker state"),
        watn::shell_shortcut::BlockState::Present
    );
    assert_eq!(
        watn::shell_shortcut::marker_state(watn::shell_shortcut::Shell::Zsh, &environment)
            .expect("shortcut marker state"),
        watn::shell_shortcut::BlockState::Present
    );
}

#[when("I uncheck the existing completion block and check a missing shortcut block")]
fn toggle_shell_marker_intents(world: &mut WatnWorld) {
    let session = world.pty_session.as_mut().expect("setup PTY session");
    pty_write(session, "\r");
    std::thread::sleep(std::time::Duration::from_millis(150));
    pty_write(session, "\r");
    wait_for_fragments(session, &["Catalog:"]);
    for _ in 0..3 {
        pty_write(session, "\r");
        std::thread::sleep(std::time::Duration::from_millis(150));
    }
    wait_for_fragments(session, &["Completion", "Bash"]);
    pty_write(session, " \x1b[B\x1b[B\x1b[B ");
    pty_write(session, "\r");
    wait_for_fragments(session, &["Review"]);
}

#[then("the completion marker block should be removed")]
fn completion_marker_removed(world: &mut WatnWorld) {
    let root = world.temp_dir.as_ref().expect("shell fixture");
    let content = std::fs::read_to_string(root.path().join(".bashrc")).expect("bash file");
    assert!(!content.contains(watn::shell_completion::OPEN_MARKER));
}

#[then("the shortcut marker block should be installed")]
fn shortcut_marker_installed(world: &mut WatnWorld) {
    let root = world.temp_dir.as_ref().expect("shell fixture");
    let content = std::fs::read_to_string(root.path().join(".bashrc")).expect("bash file");
    assert!(content.contains(watn::shell_shortcut::OPEN_MARKER));
}

#[then("unrelated shell startup-file content should be unchanged")]
fn unrelated_shell_content_unchanged(world: &mut WatnWorld) {
    let root = world.temp_dir.as_ref().expect("shell fixture");
    let bash = std::fs::read_to_string(root.path().join(".bashrc")).expect("bash file");
    let zsh = std::fs::read_to_string(root.path().join(".zshrc")).expect("zsh file");
    assert!(bash.contains("user before"));
    assert!(bash.contains("user after"));
    assert!(zsh.contains("zsh before"));
    assert!(zsh.contains("zsh after"));
}

#[then("the config file should not contain shell integration state")]
fn config_has_no_shell_state(world: &mut WatnWorld) {
    let root = world.temp_dir.as_ref().expect("shell fixture");
    let content = std::fs::read_to_string(root.path().join("watn/config.toml")).expect("config");
    assert!(!content.contains("completion_shell"));
    assert!(!content.contains("shortcut_shell"));
}

#[when("I change the Provider endpoint")]
fn change_provider_endpoint(world: &mut WatnWorld) {
    let session = world.pty_session.as_mut().expect("setup PTY session");
    pty_write(session, "\x1b[Z\x1b[B\r\r\r");
    wait_for_fragments(session, &["Catalog:"]);
}

#[then("the Model roles topic should show Small / fast, Balanced / normal, and Thinking together")]
fn model_roles_together(world: &mut WatnWorld) {
    let session = world.pty_session.as_ref().expect("setup PTY session");
    let output = wait_for_fragments(session, &["Small", "Balanced", "Thinking"]);
    assert!(output.contains("Small"));
    assert!(output.contains("Balanced"));
    assert!(output.contains("Thinking"));
}

#[then("the existing model roles should be marked \"Needs attention\"")]
fn existing_roles_need_attention(world: &mut WatnWorld) {
    let session = world.pty_session.as_ref().expect("setup PTY session");
    let output = wait_for_fragments(session, &["Needs", "attention"]);
    assert!(output.contains("Needs"));
    assert!(output.contains("attention"));
}

#[then("Finish setup should be unavailable until the model roles are reviewed")]
fn finish_unavailable_roles(world: &mut WatnWorld) {
    let session = world.pty_session.as_ref().expect("setup PTY session");
    let output = wait_for_fragments(session, &["Finish", "unavailable"]);
    assert!(output.contains("unavailable"));
}

#[when("I select or explicitly retain each model role")]
fn retain_each_model_role(world: &mut WatnWorld) {
    let session = world.pty_session.as_mut().expect("setup PTY session");
    for _ in 0..3 {
        pty_write(session, "\r");
        std::thread::sleep(std::time::Duration::from_millis(200));
    }
    pty_write(session, "\x1b[Z");
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(3);
    loop {
        let output = pty_snapshot(session);
        if output.matches("Manual").count() >= 3 {
            return;
        }
        if std::time::Instant::now() >= deadline {
            panic!("roles were not confirmed: {output:?}");
        }
        std::thread::sleep(std::time::Duration::from_millis(25));
    }
}

#[then("the Model roles topic should be complete")]
fn model_roles_complete(world: &mut WatnWorld) {
    let session = world.pty_session.as_ref().expect("setup PTY session");
    let output = pty_snapshot(session);
    assert!(output.matches("Manual").count() >= 3, "output: {output:?}");
}

#[when("I complete the required model roles and finish setup")]
fn complete_manual_roles_and_finish(world: &mut WatnWorld) {
    let session = world.pty_session.as_mut().expect("setup PTY session");
    for model in ["manual-small", "manual-normal", "manual-thinking"] {
        pty_write(session, model);
        pty_write(session, "\r");
        std::thread::sleep(std::time::Duration::from_millis(75));
    }
    pty_write(session, "\r");
    wait_for_fragments(session, &["Review", "Finish"]);
    pty_write(session, "\r");
    let session = world.pty_session.take().expect("setup PTY session");
    finish_pty_session(world, session);
    assert_eq!(
        world.exit_status,
        Some(0),
        "setup output: {:?}",
        world.output
    );
}

#[then(
    regex = r##"^the Provider topic should show the OpenRouter endpoint as \"Recommended default\"$"##
)]
fn provider_endpoint_recommended(world: &mut WatnWorld) {
    let session = world.pty_session.as_ref().expect("setup PTY session");
    let output = wait_for_fragments(session, &["Provider", "openrouter"]);
    assert!(output.contains("https://openrouter.ai/api/v1"));
    assert!(output.contains("Recommended"));
    assert!(output.contains("default"));
}

#[then(
    regex = r##"^the Provider topic should show \"OPENROUTER_API_KEY\" as \"Recommended default\" and missing$"##
)]
fn provider_credential_recommended_missing(world: &mut WatnWorld) {
    let session = world.pty_session.as_ref().expect("setup PTY session");
    let output = wait_for_fragments(session, &["OPENROUTER_API_KEY", "Recommended"]);
    assert!(output.contains("OPENROUTER_API_KEY"), "output: {output:?}");
    assert!(output.contains("Recommended"), "output: {output:?}");
    assert!(output.contains("not"), "output: {output:?}");
    assert!(output.contains("found"), "output: {output:?}");
}

#[then("Finish setup should be unavailable until a credential source is supplied")]
fn finish_unavailable_without_credential(world: &mut WatnWorld) {
    let session = world.pty_session.as_ref().expect("setup PTY session");
    let output = wait_for_fragments(session, &["Finish", "unavailable"]);
    assert!(output.contains("Finish"));
    assert!(output.contains("unavailable"));
}

fn wait_for_fragments(session: &super::PtySession, fragments: &[&str]) -> String {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(3);
    loop {
        let output = pty_snapshot(session);
        if fragments.iter().all(|fragment| output.contains(fragment)) {
            return output;
        }
        if std::time::Instant::now() >= deadline {
            panic!("missing fragments {fragments:?} in PTY output: {output:?}");
        }
        std::thread::sleep(std::time::Duration::from_millis(25));
    }
}

#[then("no config file should exist before Finish")]
fn no_config_before_finish(world: &mut WatnWorld) {
    let directory = world.temp_dir.as_ref().expect("isolated config directory");
    assert!(!directory.path().join("watn/config.toml").exists());
}
