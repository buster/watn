use std::path::PathBuf;

use cucumber::{given, then, when};

use crate::WatnWorld;

use super::{
    finish_pty_session, pty_snapshot, pty_wait_for_label, pty_write, run_binary_with_state,
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
    // Replace (not prepend) PATH: the real PATH still contains real shell
    // binaries that availability detection would otherwise find. The watn
    // binary is spawned by absolute path, so a minimal PATH is safe.
    world
        .env_vars
        .insert("PATH".to_string(), bin_dir.to_string_lossy().to_string());

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

#[given("the final configuration write cannot complete")]
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
    let session = start_pty_session(world, &["quicksetup"]);
    world.pty_session = Some(session);
}

#[when(regex = r#"^I run a request for \"([^\"]+)\" without a terminal$"#)]
fn run_request_without_terminal(world: &mut WatnWorld, question: String) {
    isolate_quicksetup_env(world);
    run_binary_with_state(world, &[question.as_str()], None);
}

#[when("I run `watn quicksetup` without a terminal")]
fn run_quicksetup_without_terminal(world: &mut WatnWorld) {
    isolate_quicksetup_env(world);
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
fn announce_no_configuration(_world: &mut WatnWorld) {
    unimplemented!("announce assertion")
}

#[then("the quick setup should ask for the completion endpoint")]
fn asks_for_endpoint(_world: &mut WatnWorld) {
    unimplemented!("endpoint question assertion")
}

#[then(regex = r#"^the endpoint question should suggest \"([^\"]+)\"$"#)]
fn endpoint_suggestion(_world: &mut WatnWorld, _suggestion: String) {
    unimplemented!("endpoint suggestion assertion")
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
fn accept_prefilled_normal_model(_world: &mut WatnWorld) {
    unimplemented!("accept normal model")
}

#[when("I accept the pre-filled thinking model")]
fn accept_prefilled_thinking_model(_world: &mut WatnWorld) {
    unimplemented!("accept thinking model")
}

#[when("I accept the suggested endpoint, credential, and models")]
fn accept_suggestions_through_models(_world: &mut WatnWorld) {
    unimplemented!("accept all text suggestions")
}

#[when("I keep the pre-selected shell integrations and confirm")]
fn keep_preselected_shells(_world: &mut WatnWorld) {
    unimplemented!("confirm shell list")
}

#[when("I deselect all shell integrations and confirm")]
fn deselect_all_shells(_world: &mut WatnWorld) {
    unimplemented!("deselect shell list")
}

#[when("I complete the quick setup with the suggested answers and no shell integrations")]
fn complete_no_shells(_world: &mut WatnWorld) {
    unimplemented!("complete flow without shells")
}

#[when("I complete the quick setup with the suggested answers and shell integrations selected")]
fn complete_with_shells(_world: &mut WatnWorld) {
    unimplemented!("complete flow with shells")
}

#[when("I abort the quick setup with Ctrl-C")]
fn abort_quicksetup(_world: &mut WatnWorld) {
    unimplemented!("abort flow")
}

#[then("quick setup should exit successfully")]
fn quicksetup_exit_success(_world: &mut WatnWorld) {
    unimplemented!("exit status assertion")
}

#[then("quick setup should report a configuration error")]
fn quicksetup_config_error(_world: &mut WatnWorld) {
    unimplemented!("config error assertion")
}

#[then("quick setup should report a nonzero result")]
fn quicksetup_nonzero_result(_world: &mut WatnWorld) {
    unimplemented!("nonzero result assertion")
}

#[then("the output should state the configuration file location")]
fn output_states_config_location(_world: &mut WatnWorld) {
    unimplemented!("config location assertion")
}

#[then(regex = r#"^the output should state that the configuration can be changed with `watn setup`$"#)]
fn output_states_setup_hint(_world: &mut WatnWorld) {
    unimplemented!("watn setup hint assertion")
}

#[then(regex = r#"^the config file should contain small model \"([^\"]+)\"$"#)]
fn config_contains_small_model(_world: &mut WatnWorld, _model: String) {
    unimplemented!("small model config assertion")
}

#[then(regex = r#"^the config file should contain normal model \"([^\"]+)\"$"#)]
fn config_contains_normal_model(_world: &mut WatnWorld, _model: String) {
    unimplemented!("normal model config assertion")
}

#[then(regex = r#"^the config file should contain thinking model \"([^\"]+)\"$"#)]
fn config_contains_thinking_model(_world: &mut WatnWorld, _model: String) {
    unimplemented!("thinking model config assertion")
}

#[then(regex = r#"^the config file should contain small model \"([^\"]+)\" without reasoning$"#)]
fn config_contains_small_model_without_reasoning(_world: &mut WatnWorld, _model: String) {
    unimplemented!("small model without reasoning assertion")
}

#[then(regex = r#"^the config file should contain normal model \"([^\"]+)\" without reasoning$"#)]
fn config_contains_normal_model_without_reasoning(_world: &mut WatnWorld, _model: String) {
    unimplemented!("normal model without reasoning assertion")
}

#[then(regex = r#"^the config file should contain thinking model \"([^\"]+)\" without reasoning$"#)]
fn config_contains_thinking_model_without_reasoning(_world: &mut WatnWorld, _model: String) {
    unimplemented!("thinking model without reasoning assertion")
}

#[then(regex = r#"^the config file should contain credential \"([^\"]+)\"$"#)]
fn config_contains_credential(_world: &mut WatnWorld, _credential: String) {
    unimplemented!("credential config assertion")
}

#[then("no reasoning question should have been shown")]
fn no_reasoning_question(_world: &mut WatnWorld) {
    unimplemented!("no reasoning question assertion")
}

#[then("Bash should contain a Watn-managed Ctrl-W block")]
fn bash_has_ctrlw_block(_world: &mut WatnWorld) {
    unimplemented!("Bash Ctrl-W block assertion")
}

#[then("Zsh should contain a Watn-managed completion block")]
fn zsh_has_completion_block(_world: &mut WatnWorld) {
    unimplemented!("Zsh completion block assertion")
}

#[then("Fish should contain a Watn-managed completion block")]
fn fish_has_completion_block(_world: &mut WatnWorld) {
    unimplemented!("Fish completion block assertion")
}

#[then("Fish should contain a Watn-managed Ctrl-W block")]
fn fish_has_ctrlw_block(_world: &mut WatnWorld) {
    unimplemented!("Fish Ctrl-W block assertion")
}

#[then("no shell target file should change")]
fn no_shell_target_changes(_world: &mut WatnWorld) {
    unimplemented!("shell target unchanged assertion")
}

#[then(regex = r#"^the shell integration list should mark ([A-Za-z]+) as (selected|not selected)$"#)]
fn shell_list_marks(_world: &mut WatnWorld, _shell: String, _state: String) {
    unimplemented!("shell list marking assertion")
}
