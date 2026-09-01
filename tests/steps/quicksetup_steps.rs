use std::path::PathBuf;

use cucumber::{given, then, when};

use crate::WatnWorld;

use super::{
    finish_pty_session, pty_wait_for_label, pty_write, run_binary_with_state,
    start_pty_session,
};

const STUB_SHELLS: [&str; 3] = ["bash", "zsh", "fish"];

/// Isolate every quicksetup scenario from the runner's real configuration:
/// fresh temp dir for `HOME` and `XDG_CONFIG_HOME`, and a `PATH` containing
/// only the scenario's stub shell directory. Idempotent: reuses the existing
/// temp dir on later calls within the same scenario.
fn isolate_quicksetup_env(world: &mut WatnWorld) -> PathBuf {
    if world.temp_dir.is_none() {
        world.temp_dir = Some(tempfile::tempdir().expect("create quicksetup temp dir"));
    }
    let dir = world
        .temp_dir
        .as_ref()
        .expect("quicksetup temp dir")
        .path()
        .to_path_buf();

    let bin_dir = dir.join("bin");
    std::fs::create_dir_all(&bin_dir).expect("create stub bin dir");

    world
        .env_vars
        .insert("HOME".to_string(), dir.to_string_lossy().to_string());
    world.env_vars.insert(
        "XDG_CONFIG_HOME".to_string(),
        dir.to_string_lossy().to_string(),
    );
    // Replace (not prepend) the child's PATH: the real PATH still contains
    // real shell binaries that availability detection would otherwise find.
    // Kept out of env_vars because WatnWorld::drop removes those keys from
    // the runner process. The watn binary is spawned by absolute path, so a
    // minimal PATH is safe.
    world.path_override = Some(bin_dir.to_string_lossy().to_string());

    assert!(
        world
            .env_vars
            .get("HOME")
            .is_some_and(|home| PathBuf::from(home).starts_with(&dir)),
        "quicksetup HOME isolation guard failed"
    );
    assert!(
        world
            .env_vars
            .get("XDG_CONFIG_HOME")
            .is_some_and(|xdg| PathBuf::from(xdg).starts_with(&dir)),
        "quicksetup XDG_CONFIG_HOME isolation guard failed"
    );
    dir
}

fn create_shell_stub(world: &mut WatnWorld, name: &str) {
    let dir = isolate_quicksetup_env(world);
    let stub = dir.join("bin").join(name);
    std::fs::write(&stub, b"").expect("write shell stub");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&stub, std::fs::Permissions::from_mode(0o755))
            .expect("make shell stub executable");
    }
}

fn quicksetup_config_path(world: &WatnWorld) -> PathBuf {
    let dir = world.temp_dir.as_ref().expect("quicksetup temp dir");
    dir.path().join("watn").join("config.toml")
}

fn quicksetup_config_content(world: &WatnWorld) -> String {
    std::fs::read_to_string(quicksetup_config_path(world)).expect("quicksetup config file")
}

/// `ensure_test_env` only writes the fixture config when it creates the temp
/// dir itself; quicksetup scenarios pre-create it for HOME/PATH isolation, so
/// materialize the fixture config here before spawning the binary.
fn ensure_quicksetup_fixture_config(world: &mut WatnWorld) {
    if let Some(raw) = world.raw_config.clone() {
        let path = quicksetup_config_path(world);
        if !path.exists() {
            std::fs::create_dir_all(path.parent().expect("config parent"))
                .expect("create config dir");
            std::fs::write(&path, raw).expect("write fixture config");
        }
    }
}

#[given("no watn configuration exists")]
fn no_watn_configuration(world: &mut WatnWorld) {
    isolate_quicksetup_env(world);
    world.raw_config = None;
    world.pending_mock_no_config_file = true;
}

#[given(regex = r#"^an existing watn configuration contains provider \"([^\"]+)\" with credential \"([^\"]+)\"$"#)]
fn existing_watn_configuration(world: &mut WatnWorld, provider: String, credential: String) {
    isolate_quicksetup_env(world);
    world.raw_config = Some(format!(
        "[defaults]\nprovider = \"{provider}\"\n\n[providers.{provider}]\nendpoint = \"https://legacy.example/v1\"\napi_key = \"{credential}\"\n"
    ));
    ensure_quicksetup_fixture_config(world);
}

#[given("an existing openrouter configuration without a credential")]
fn existing_openrouter_without_credential(world: &mut WatnWorld) {
    isolate_quicksetup_env(world);
    world.raw_config = Some("[defaults]\nprovider = \"openrouter\"\n".to_string());
    ensure_quicksetup_fixture_config(world);
    // Trigger the shared harness chat-completion mock so the
    // "no original chat completion request" assertion has a sentinel.
    world.pending_mock_model = Some("test-model".to_string());
    world.pending_mock_output = Some("some output".to_string());
    world.pending_mock_usage = Some(false);
}

#[given("bash, zsh, and fish are available on the path")]
fn all_shells_available(world: &mut WatnWorld) {
    for name in STUB_SHELLS {
        create_shell_stub(world, name);
    }
}

#[given("bash and zsh are available on the path but fish is not")]
fn bash_and_zsh_available(world: &mut WatnWorld) {
    for name in ["bash", "zsh"] {
        create_shell_stub(world, name);
    }
}

#[given("provider requests are captured by a sentinel")]
fn capture_provider_requests(world: &mut WatnWorld) {
    isolate_quicksetup_env(world);
    let server = world
        .mock_server
        .0
        .get_or_insert_with(httpmock::MockServer::start);
    let base_url = format!("http://127.0.0.1:{}", server.port());
    world.models_mock_id = Some(
        server
            .mock(|when, then| {
                when.method(httpmock::Method::GET).path("/models");
                then.status(200).body(r#"{"data":[{"id":"unused"}]}"#);
            })
            .id,
    );
    world.mock_server.1 = Some(
        server
            .mock(|when, then| {
                when.method(httpmock::Method::POST).path("/chat/completions");
                then.status(200)
                    .header("Content-Type", "text/event-stream")
                    .body("data: {\"id\":\"1\",\"choices\":[{\"index\":0,\"delta\":{\"content\":\"unused\"},\"finish_reason\":\"stop\"}]}\ndata: [DONE]\n");
            })
            .id,
    );
    world
        .env_vars
        .insert("WATN_TEST_ENDPOINT_OVERRIDE".to_string(), base_url);
}

#[given("the configuration write is forced to fail")]
fn fail_config_write(world: &mut WatnWorld) {
    isolate_quicksetup_env(world);
    world
        .env_vars
        .insert("WATN_TEST_FAIL_CONFIG_WRITE".to_string(), "1".to_string());
}

#[given("the fish target path cannot be written")]
fn fish_target_blocked(world: &mut WatnWorld) {
    let dir = isolate_quicksetup_env(world);
    let fish_config = dir.join("fish").join("config.fish");
    std::fs::create_dir_all(&fish_config).expect("create fish config.fish directory blocker");
}

#[when("I start `watn quicksetup` in a terminal")]
fn start_quicksetup_in_terminal(world: &mut WatnWorld) {
    isolate_quicksetup_env(world);
    ensure_quicksetup_fixture_config(world);
    let session = start_pty_session(world, &["quicksetup"]);
    world.pty_session = Some(session);
}

#[when(regex = r#"^I run a request for \"([^\"]+)\" without a terminal$"#)]
fn run_request_without_terminal(world: &mut WatnWorld, question: String) {
    isolate_quicksetup_env(world);
    ensure_quicksetup_fixture_config(world);
    run_binary_with_state(world, &[question.as_str()], None);
}

#[when("I run `watn quicksetup` without a terminal")]
fn run_quicksetup_without_terminal(world: &mut WatnWorld) {
    isolate_quicksetup_env(world);
    ensure_quicksetup_fixture_config(world);
    run_binary_with_state(world, &["quicksetup"], None);
}

#[then("the output should instruct me to run `watn quicksetup` in a terminal")]
fn guidance_mentions_quicksetup(world: &mut WatnWorld) {
    let output = format!(
        "{}{}",
        world.output.as_deref().unwrap_or_default(),
        world.stderr_output.as_deref().unwrap_or_default()
    );
    assert!(
        output.contains("watn quicksetup"),
        "quicksetup guidance missing: {output:?}"
    );
    assert!(
        output.contains("terminal"),
        "terminal guidance missing: {output:?}"
    );
}

#[then("the output should not mention the quick setup")]
fn output_does_not_mention_quicksetup(world: &mut WatnWorld) {
    let output = format!(
        "{}{}",
        world.output.as_deref().unwrap_or_default(),
        world.stderr_output.as_deref().unwrap_or_default()
    );
    assert!(
        !output.to_ascii_lowercase().contains("quick setup"),
        "unexpected quick setup mention: {output:?}"
    );
}

// ---------------------------------------------------------------------------
// Interactive flow steps — implemented scenario by scenario.
// ---------------------------------------------------------------------------

#[then("the quick setup should announce that no configuration was found")]
fn announce_no_configuration(world: &mut WatnWorld) {
    let session = world.pty_session.as_ref().expect("quicksetup PTY session");
    let output = pty_wait_for_label(session, "Completion endpoint");
    assert!(
        output.to_ascii_lowercase().contains("no configuration"),
        "announcement missing: {output:?}"
    );
    assert!(
        output.to_ascii_lowercase().contains("quick setup"),
        "quick setup announcement missing: {output:?}"
    );
}

#[then("the quick setup should ask for the completion endpoint")]
fn asks_for_endpoint(world: &mut WatnWorld) {
    let session = world.pty_session.as_ref().expect("quicksetup PTY session");
    let output = pty_wait_for_label(session, "Completion endpoint");
    assert!(
        output.contains("Completion endpoint"),
        "endpoint question missing: {output:?}"
    );
}

#[then(regex = r#"^the endpoint question should suggest \"([^\"]+)\"$"#)]
fn endpoint_suggestion(world: &mut WatnWorld, suggestion: String) {
    let session = world.pty_session.as_ref().expect("quicksetup PTY session");
    let output = pty_wait_for_label(session, "Completion endpoint");
    assert!(
        output.contains(&format!("[{suggestion}]")),
        "endpoint suggestion {suggestion:?} missing: {output:?}"
    );
}

#[when("I accept the suggested endpoint")]
fn accept_suggested_endpoint(world: &mut WatnWorld) {
    let session = world.pty_session.as_mut().expect("quicksetup PTY session");
    pty_wait_for_label(session, "Completion endpoint");
    pty_write(session, "\r");
}

#[when(regex = r#"^I answer the endpoint with \"([^\"]+)\"$"#)]
fn answer_endpoint(world: &mut WatnWorld, endpoint: String) {
    let session = world.pty_session.as_mut().expect("quicksetup PTY session");
    pty_wait_for_label(session, "Completion endpoint");
    pty_write(session, &format!("{endpoint}\r"));
}

#[when("I answer the endpoint with an invalid value")]
fn answer_endpoint_invalid(world: &mut WatnWorld) {
    let session = world.pty_session.as_mut().expect("quicksetup PTY session");
    pty_wait_for_label(session, "Completion endpoint");
    pty_write(session, "not-a-valid-url\r");
}

#[then("quick setup should still ask for the endpoint")]
fn still_asks_endpoint(world: &mut WatnWorld) {
    let session = world.pty_session.as_ref().expect("quicksetup PTY session");
    let output = pty_wait_for_label(session, "endpoint must be an HTTP or HTTPS URL");
    assert!(
        output.matches("Completion endpoint").count() >= 2,
        "endpoint question was not re-asked: {output:?}"
    );
}

#[when("I accept the suggested credential reference")]
fn accept_suggested_credential(world: &mut WatnWorld) {
    let session = world.pty_session.as_mut().expect("quicksetup PTY session");
    pty_wait_for_label(session, "API key");
    pty_write(session, "\r");
}

#[when(regex = r#"^I answer the credential with \"([^\"]+)\"$"#)]
fn answer_credential(world: &mut WatnWorld, credential: String) {
    let session = world.pty_session.as_mut().expect("quicksetup PTY session");
    pty_wait_for_label(session, "API key");
    pty_write(session, &format!("{credential}\r"));
}

#[then(regex = r#"^the credential question should suggest \"([^\"]+)\"$"#)]
fn credential_suggestion(world: &mut WatnWorld, suggestion: String) {
    let session = world.pty_session.as_ref().expect("quicksetup PTY session");
    let output = pty_wait_for_label(session, "API key");
    assert!(
        output.contains(&format!("[{suggestion}]")),
        "credential suggestion {suggestion:?} missing: {output:?}"
    );
}

#[when("I accept the suggested small model")]
fn accept_suggested_small_model(world: &mut WatnWorld) {
    let session = world.pty_session.as_mut().expect("quicksetup PTY session");
    pty_wait_for_label(session, "Small model");
    pty_write(session, "\r");
}

#[when(regex = r#"^I answer the small model with \"([^\"]+)\"$"#)]
fn answer_small_model(world: &mut WatnWorld, model: String) {
    let session = world.pty_session.as_mut().expect("quicksetup PTY session");
    pty_wait_for_label(session, "Small model");
    pty_write(session, &format!("{model}\r"));
}

#[then("the small model question should show no suggestion")]
fn small_model_no_suggestion(world: &mut WatnWorld) {
    let session = world.pty_session.as_ref().expect("quicksetup PTY session");
    let output = pty_wait_for_label(session, "Small model");
    assert!(
        !output.contains("Small model ["),
        "unexpected small model suggestion: {output:?}"
    );
}

#[when("I answer the small model question with an empty input")]
fn answer_small_model_empty(world: &mut WatnWorld) {
    let session = world.pty_session.as_mut().expect("quicksetup PTY session");
    pty_wait_for_label(session, "Small model");
    pty_write(session, "\r");
}

#[then("quick setup should still ask for the small model")]
fn still_asks_small_model(world: &mut WatnWorld) {
    let session = world.pty_session.as_ref().expect("quicksetup PTY session");
    let output = pty_wait_for_label(session, "value is required");
    assert!(
        output.matches("Small model").count() >= 2,
        "small model question was not re-asked: {output:?}"
    );
}

#[when("I accept the pre-filled normal model")]
fn accept_prefilled_normal_model(world: &mut WatnWorld) {
    let session = world.pty_session.as_mut().expect("quicksetup PTY session");
    pty_wait_for_label(session, "Normal model");
    pty_write(session, "\r");
}

#[when("I accept the pre-filled thinking model")]
fn accept_prefilled_thinking_model(world: &mut WatnWorld) {
    let session = world.pty_session.as_mut().expect("quicksetup PTY session");
    pty_wait_for_label(session, "Thinking model");
    pty_write(session, "\r");
}

#[when("I accept the suggested endpoint, credential, and models")]
fn accept_suggestions_through_models(world: &mut WatnWorld) {
    answer_all_suggestions(world);
}

#[then(regex = r#"^the shell integration list should mark ([A-Za-z]+) as (selected|not selected)$"#)]
fn shell_list_marks(world: &mut WatnWorld, shell: String, state: String) {
    let session = world.pty_session.as_ref().expect("quicksetup PTY session");
    let output = pty_wait_for_label(session, "Shell integrations");
    let name = shell.to_ascii_lowercase();
    let marker = match state.as_str() {
        "selected" => "[x]",
        _ => "[ ]",
    };
    let rendered = format!("{marker} {}", capitalize(&name));
    assert!(
        output.contains(&rendered),
        "shell list does not show {rendered:?}: {output:?}"
    );
}

fn capitalize(name: &str) -> String {
    let mut chars = name.chars();
    match chars.next() {
        Some(first) => first.to_ascii_uppercase().to_string() + chars.as_str(),
        None => String::new(),
    }
}

#[when("I keep the pre-selected shell integrations and confirm")]
fn keep_preselected_shells(world: &mut WatnWorld) {
    let session = world.pty_session.as_mut().expect("quicksetup PTY session");
    pty_wait_for_label(session, "Shell integrations");
    std::thread::sleep(std::time::Duration::from_millis(100));
    pty_write(session, "\r");
    let session = world.pty_session.take().expect("quicksetup PTY session");
    finish_pty_session(world, session);
}

#[when("I deselect all shell integrations and confirm")]
fn deselect_all_shells(world: &mut WatnWorld) {
    let session = world.pty_session.as_mut().expect("quicksetup PTY session");
    pty_wait_for_label(session, "Shell integrations");
    std::thread::sleep(std::time::Duration::from_millis(100));
    pty_write(session, "bash zsh fish\r");
    std::thread::sleep(std::time::Duration::from_millis(150));
    pty_write(session, "\r");
    let session = world.pty_session.take().expect("quicksetup PTY session");
    finish_pty_session(world, session);
}

fn answer_all_suggestions(world: &mut WatnWorld) {
    let session = world.pty_session.as_mut().expect("quicksetup PTY session");
    pty_wait_for_label(session, "Completion endpoint");
    pty_write(session, "\r");
    pty_wait_for_label(session, "API key");
    pty_write(session, "\r");
    pty_wait_for_label(session, "Small model");
    pty_write(session, "\r");
    pty_wait_for_label(session, "Normal model");
    pty_write(session, "\r");
    pty_wait_for_label(session, "Thinking model");
    pty_write(session, "\r");
    pty_wait_for_label(session, "Shell integrations");
}

#[when("I complete the quick setup with the suggested answers and no shell integrations")]
fn complete_no_shells(world: &mut WatnWorld) {
    start_quicksetup_in_terminal(world);
    answer_all_suggestions(world);
    let session = world.pty_session.as_mut().expect("quicksetup PTY session");
    pty_write(session, "\r");
    let session = world.pty_session.take().expect("quicksetup PTY session");
    finish_pty_session(world, session);
}

#[when("I complete the quick setup with the suggested answers and shell integrations selected")]
fn complete_with_shells(world: &mut WatnWorld) {
    start_quicksetup_in_terminal(world);
    answer_all_suggestions(world);
    let session = world.pty_session.as_mut().expect("quicksetup PTY session");
    std::thread::sleep(std::time::Duration::from_millis(100));
    pty_write(session, "bash zsh fish\r");
    std::thread::sleep(std::time::Duration::from_millis(100));
    pty_write(session, "\r");
    let session = world.pty_session.take().expect("quicksetup PTY session");
    finish_pty_session(world, session);
}

#[when("I abort the quick setup with Ctrl-C")]
fn abort_quicksetup(world: &mut WatnWorld) {
    let path = quicksetup_config_path(world);
    // Record the baseline only when a configuration already exists (explicit
    // run); on a first run there is nothing to record.
    if let Ok(content) = std::fs::read_to_string(&path) {
        world
            .pending_config
            .insert("config_before".to_string(), content);
    }
    let session = world.pty_session.as_mut().expect("quicksetup PTY session");
    std::thread::sleep(std::time::Duration::from_millis(100));
    pty_write(session, "\x03");
    let session = world.pty_session.take().expect("quicksetup PTY session");
    finish_pty_session(world, session);
}

#[then("quick setup should exit successfully")]
fn quicksetup_exit_success(world: &mut WatnWorld) {
    assert_eq!(world.exit_status, Some(0), "quicksetup should exit 0");
}

#[then("quick setup should report a configuration error")]
fn quicksetup_config_error(world: &mut WatnWorld) {
    assert_ne!(world.exit_status, Some(0), "quicksetup should exit nonzero");
    // The PTY harness merges stderr into the captured output stream.
    let output = world.output.as_deref().unwrap_or_default();
    assert!(
        output.contains("cannot write config"),
        "config write error missing: {output:?}"
    );
}

#[then("the output should state the configuration file location")]
fn output_states_config_location(world: &mut WatnWorld) {
    let output = world.output.as_deref().unwrap_or_default();
    let path = quicksetup_config_path(world);
    let file_name = path
        .file_name()
        .expect("config file name")
        .to_string_lossy()
        .to_string();
    assert!(
        output.contains("Configuration written to") && output.contains(&file_name),
        "config location missing: {output:?}"
    );
}

#[then(regex = r#"^the output should state that the configuration can be changed with `watn setup`$"#)]
fn output_states_setup_hint(world: &mut WatnWorld) {
    let output = world.output.as_deref().unwrap_or_default();
    assert!(
        output.contains("watn setup"),
        "watn setup hint missing: {output:?}"
    );
}

#[then("quick setup should report a nonzero result")]
fn quicksetup_nonzero_result(world: &mut WatnWorld) {
    assert_ne!(world.exit_status, Some(0), "quicksetup should exit nonzero");
}

#[then(regex = r#"^the config file should contain small model \"([^\"]+)\"$"#)]
fn config_contains_small_model(world: &mut WatnWorld, model: String) {
    let content = quicksetup_config_content(world);
    assert!(
        content.contains(&format!("small = \"{model}\"")),
        "small model {model:?} missing: {content:?}"
    );
}

#[then(regex = r#"^the config file should contain normal model \"([^\"]+)\"$"#)]
fn config_contains_normal_model(world: &mut WatnWorld, model: String) {
    let content = quicksetup_config_content(world);
    assert!(
        content.contains(&format!("normal = \"{model}\"")),
        "normal model {model:?} missing: {content:?}"
    );
}

#[then(regex = r#"^the config file should contain thinking model \"([^\"]+)\"$"#)]
fn config_contains_thinking_model(world: &mut WatnWorld, model: String) {
    let content = quicksetup_config_content(world);
    assert!(
        content.contains(&format!("thinking = \"{model}\"")),
        "thinking model {model:?} missing: {content:?}"
    );
}

#[then(regex = r#"^the config file should contain small model \"([^\"]+)\" without reasoning$"#)]
fn config_contains_small_model_without_reasoning(world: &mut WatnWorld, model: String) {
    let content = quicksetup_config_content(world);
    assert!(
        content.contains(&format!("small = \"{model}\"")),
        "small model {model:?} missing: {content:?}"
    );
    assert!(
        !content.contains("reasoning"),
        "unexpected reasoning setting: {content:?}"
    );
}

#[then(regex = r#"^the config file should contain normal model \"([^\"]+)\" without reasoning$"#)]
fn config_contains_normal_model_without_reasoning(world: &mut WatnWorld, model: String) {
    let content = quicksetup_config_content(world);
    assert!(
        content.contains(&format!("normal = \"{model}\"")),
        "normal model {model:?} missing: {content:?}"
    );
}

#[then(regex = r#"^the config file should contain thinking model \"([^\"]+)\" without reasoning$"#)]
fn config_contains_thinking_model_without_reasoning(world: &mut WatnWorld, model: String) {
    let content = quicksetup_config_content(world);
    assert!(
        content.contains(&format!("thinking = \"{model}\"")),
        "thinking model {model:?} missing: {content:?}"
    );
    assert!(
        !content.contains("reasoning"),
        "unexpected reasoning setting: {content:?}"
    );
}

#[then(regex = r#"^the config file should contain credential \"([^\"]+)\"$"#)]
fn config_contains_credential(world: &mut WatnWorld, credential: String) {
    let content = quicksetup_config_content(world);
    assert!(
        content.contains(&format!("api_key = \"{credential}\"")),
        "credential {credential:?} missing: {content:?}"
    );
}

#[then("no reasoning question should have been shown")]
fn no_reasoning_question(world: &mut WatnWorld) {
    let output = world.output.as_deref().unwrap_or_default();
    assert!(
        !output.to_ascii_lowercase().contains("reasoning"),
        "reasoning question was shown: {output:?}"
    );
}

#[then("Bash should contain a Watn-managed Ctrl-W block")]
fn bash_has_ctrlw_block(world: &mut WatnWorld) {
    let dir = world.temp_dir.as_ref().expect("quicksetup temp dir");
    let content = std::fs::read_to_string(dir.path().join(".bashrc")).expect("Bash target");
    assert!(
        content.contains(watn::shell_shortcut::OPEN_MARKER),
        "Bash Ctrl-W block missing: {content:?}"
    );
}

#[then("Zsh should contain a Watn-managed completion block")]
fn zsh_has_completion_block(world: &mut WatnWorld) {
    let dir = world.temp_dir.as_ref().expect("quicksetup temp dir");
    let content = std::fs::read_to_string(dir.path().join(".zshrc")).expect("Zsh target");
    assert!(
        content.contains(watn::shell_completion::OPEN_MARKER),
        "Zsh completion block missing: {content:?}"
    );
}

#[then("Fish should contain a Watn-managed completion block")]
fn fish_has_completion_block(world: &mut WatnWorld) {
    let dir = world.temp_dir.as_ref().expect("quicksetup temp dir");
    let content =
        std::fs::read_to_string(dir.path().join("fish").join("config.fish")).expect("Fish target");
    assert!(
        content.contains(watn::shell_completion::OPEN_MARKER),
        "Fish completion block missing: {content:?}"
    );
}

#[then("Fish should contain a Watn-managed Ctrl-W block")]
fn fish_has_ctrlw_block(world: &mut WatnWorld) {
    let dir = world.temp_dir.as_ref().expect("quicksetup temp dir");
    let content =
        std::fs::read_to_string(dir.path().join("fish").join("config.fish")).expect("Fish target");
    assert!(
        content.contains(watn::shell_shortcut::OPEN_MARKER),
        "Fish Ctrl-W block missing: {content:?}"
    );
}

#[then("no shell target file should change")]
fn no_shell_target_changes(world: &mut WatnWorld) {
    let dir = world.temp_dir.as_ref().expect("quicksetup temp dir");
    let home = dir.path();
    for target in [
        home.join(".bashrc"),
        home.join(".zshrc"),
        home.join("fish").join("config.fish"),
    ] {
        assert!(
            !target.exists(),
            "shell target unexpectedly created: {}",
            target.display()
        );
    }
}
