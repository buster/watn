use cucumber::{then, when};
use std::io::ErrorKind;
use std::path::Path;
use std::process::Command;

use super::{
    ensure_test_env, find_binary, finish_pty_session, pty_wait_for_label, pty_write,
    start_pty_command,
};
use crate::WatnWorld;

const E2E_REVIEW_COMMAND: &str = "printf 'accepted' > /tmp/watn-shortcut-should-not-run";

fn captured_bash_line(world: &WatnWorld) -> &str {
    world
        .shortcut_output
        .as_deref()
        .unwrap_or_default()
        .split("LINE<<")
        .nth(1)
        .and_then(|value| value.split(">>").next())
        .expect("Bash process line output")
}

fn captured_bash_history(world: &WatnWorld) -> &str {
    super::interactive_shell_shortcut_steps::current_history(world)
}

fn assert_shell_syntax(shell: &str, name: &str, path: &Path, required: bool) {
    let result = Command::new(shell)
        .args(["-n", path.to_str().expect("UTF-8 shell target path")])
        .output();
    let result = match result {
        Ok(result) => result,
        Err(error) if !required && error.kind() == ErrorKind::NotFound => return,
        Err(error) => panic!("run {name} syntax check: {error}"),
    };
    assert!(
        result.status.success(),
        "{name} rejected generated configuration: {}",
        String::from_utf8_lossy(&result.stderr)
    );
}

#[then("the generated Bash configuration should pass a Bash syntax check")]
fn bash_syntax_check(world: &mut WatnWorld) {
    let path = world.shortcut_targets.get("bash").expect("Bash target");
    assert_shell_syntax("bash", "Bash", path, true);
}

#[then("the generated Zsh configuration should pass a Zsh syntax check")]
fn zsh_syntax_check(world: &mut WatnWorld) {
    let path = world.shortcut_targets.get("zsh").expect("Zsh target");
    assert_shell_syntax("zsh", "Zsh", path, std::env::var_os("CI").is_some());
}

#[then("the generated Fish configuration should pass a Fish syntax check")]
fn fish_syntax_check(world: &mut WatnWorld) {
    let path = world.shortcut_targets.get("fish").expect("Fish target");
    assert_shell_syntax("fish", "Fish", path, std::env::var_os("CI").is_some());
}

#[when(
    regex = r##"^I run the generated Bash widget through Bash with current input \"([^\"]*)\"$"##
)]
fn run_generated_bash(world: &mut WatnWorld, input: String) {
    super::interactive_shell_shortcut_steps::run_bash_widget(world, input);
}

#[then(regex = r##"^the Bash process command line should be exactly \"([^\"]*)\"$"##)]
fn bash_process_line(world: &mut WatnWorld, expected: String) {
    let line = captured_bash_line(world);
    assert_eq!(line, expected.replace("\\n", "\n"));
    let temp = world.temp_dir.as_ref().expect("Bash E2E temp dir");
    let log = std::fs::read_to_string(temp.path().join("watn-invocations.log"))
        .expect("Bash E2E invocation log");
    assert_eq!(log, "find all images\n");
}

#[then("the Bash process should not execute the replacement text")]
fn bash_process_no_eval(_world: &mut WatnWorld) {
    assert!(!std::path::Path::new("/tmp/watn-shortcut-should-not-run").exists());
}

#[then(
    regex = r##"^the Bash process should record the request \"([^\"]*)\" in the shell history$"##
)]
fn bash_process_records_request(world: &mut WatnWorld, comment: String) {
    let history = captured_bash_history(world);
    assert!(
        history.contains(&comment),
        "the shell history should contain the request comment {comment:?}, got: {history:?}"
    );
}
fn configure_e2e_review(world: &mut WatnWorld) {
    world.review.e2e = true;
    let response = serde_json::json!({
        "review_version": 1,
        "command": E2E_REVIEW_COMMAND,
        "stages": [
            {
                "stage_text": E2E_REVIEW_COMMAND,
                "purpose": "Write the accepted marker without executing it."
            }
        ],
        "purpose_status": "ready"
    })
    .to_string();
    world.pending_mock_output = Some(response);
    world.pending_mock_model = Some("test-model".to_string());
    world.pending_mock_usage = Some(false);
    world.temp_dir = None;
    world.raw_config = None;
    ensure_test_env(world);

    let home = world
        .temp_dir
        .as_ref()
        .expect("e2e review temp dir")
        .path()
        .join("home");
    std::fs::create_dir_all(&home).expect("create e2e review home");
    let target = home.join(".bashrc");
    let environment = watn::shell_shortcut::ShellEnvironment {
        home,
        xdg_config_home: None,
        shell: Some("/bin/bash".to_string()),
    };
    let report = watn::shell_shortcut::install_with_environment(
        &[watn::shell_shortcut::Shell::Bash],
        &environment,
    );
    assert!(report.is_success(), "e2e shortcut install: {report:?}");
    world.shortcut_targets = std::collections::HashMap::from([("bash".to_string(), target)]);

    let binary = find_binary();
    let bin_dir = world
        .temp_dir
        .as_ref()
        .expect("e2e review temp dir")
        .path()
        .join("bin");
    std::fs::create_dir_all(&bin_dir).expect("create e2e bin dir");
    let watn_link = bin_dir.join("watn");
    #[cfg(unix)]
    std::os::unix::fs::symlink(&binary, &watn_link).expect("link the real watn binary");
    let current_path = std::env::var("PATH").unwrap_or_default();
    world.path_override = Some(format!("{}:{}", bin_dir.display(), current_path));

    let _ = std::fs::remove_file("/tmp/watn-shortcut-should-not-run");
}

#[cucumber::given("the candidate has a visible command flow with model-written stage purposes")]
fn e2e_visible_command_flow(world: &mut WatnWorld) {
    configure_e2e_review(world);
}

pub(crate) fn invoke_review_widget_pty(world: &mut WatnWorld) {
    let target = world
        .shortcut_targets
        .get("bash")
        .expect("e2e Bash shortcut target")
        .display()
        .to_string();
    world
        .env_vars
        .insert("WATN_SHORTCUT_FILE".to_string(), target);
    world
        .env_vars
        .insert("WATN_INPUT".to_string(), world.review.intent.clone());
    let script = r#"
source "$WATN_SHORTCUT_FILE"
READLINE_LINE="$WATN_INPUT"
READLINE_POINT=0
_watn_widget
printf 'LINE<<%s>>\n' "$READLINE_LINE"
printf 'HIST<<%s>>\n' "$(history)"
"#;
    let session = start_pty_command(world, "bash", &["--noprofile", "--norc", "-c", script]);
    world.pty_session = Some(session);
}

fn marker_value_first(output: &str, marker: &str) -> String {
    let start = output.find(marker).expect("marker start") + marker.len();
    let rest = &output[start..];
    let end = rest.find(">>").expect("marker end");
    rest[..end].to_string()
}

fn marker_value_last(output: &str, marker: &str) -> String {
    let start = output.find(marker).expect("marker start") + marker.len();
    let rest = &output[start..];
    let end = rest.rfind(">>").expect("marker end");
    rest[..end].to_string()
}

#[when("I accept the selected candidate in the review surface")]
fn e2e_accept_candidate(world: &mut WatnWorld) {
    let session = world.pty_session.as_mut().expect("e2e bash PTY session");
    pty_wait_for_label(session, "accept ·");
    pty_write(session, "\r");
    let output = pty_wait_for_label(session, "HIST<<");
    let line = marker_value_first(&output, "LINE<<");
    let history = marker_value_last(&output, "HIST<<");
    world.review.bash_command_line = line;
    world.review.bash_history = history.lines().map(str::to_string).collect();
    let session = world.pty_session.take().expect("e2e bash PTY session");
    let _ = finish_pty_session(world, session);
}

#[then("the Bash command line should contain the accepted candidate")]
fn e2e_bash_line_contains_candidate(world: &mut WatnWorld) {
    assert!(
        world.review.bash_command_line.contains(E2E_REVIEW_COMMAND),
        "Bash command line {:?} should contain the accepted candidate",
        world.review.bash_command_line
    );
}

#[then(expr = "the Bash history should contain the original request comment {string}")]
fn e2e_bash_history_contains_comment(world: &mut WatnWorld, comment: String) {
    let history = world.review.bash_history.join("\n");
    assert!(
        history.contains(&comment),
        "Bash history {history:?} should contain {comment:?}"
    );
}

#[then("the accepted candidate should not have executed")]
fn e2e_candidate_not_executed(_world: &mut WatnWorld) {
    assert!(
        !std::path::Path::new("/tmp/watn-shortcut-should-not-run").exists(),
        "the accepted candidate must not execute"
    );
}

#[cucumber::given(expr = "the current Bash command line is {string}")]
fn e2e_current_command_line(world: &mut WatnWorld, line: String) {
    configure_e2e_review(world);
    world.review.bash_command_line = line;
}

#[when("I cancel the review surface")]
fn e2e_cancel_review(world: &mut WatnWorld) {
    let session = world.pty_session.as_mut().expect("e2e bash PTY session");
    pty_wait_for_label(session, "accept ·");
    pty_write(session, "\x1b");
    let output = pty_wait_for_label(session, "HIST<<");
    let line = marker_value_first(&output, "LINE<<");
    let history = marker_value_last(&output, "HIST<<");
    world.review.bash_command_line = line;
    world.review.bash_history = history.lines().map(str::to_string).collect();
    let session = world.pty_session.take().expect("e2e bash PTY session");
    let _ = finish_pty_session(world, session);
}

#[then(expr = "the Bash command line should remain {string}")]
fn e2e_bash_line_remains(world: &mut WatnWorld, line: String) {
    assert_eq!(
        world.review.bash_command_line.trim_end(),
        line,
        "cancellation must leave the Bash command line unchanged"
    );
}

#[then(expr = "the Bash history should not contain a new request comment for {string}")]
fn e2e_history_no_comment(world: &mut WatnWorld, request: String) {
    let history = world.review.bash_history.join("\n");
    assert!(
        !history.contains(&format!("# {request}")),
        "cancellation must not record a request comment, history: {history:?}"
    );
}

pub(crate) fn prepare_e2e_direct(world: &mut WatnWorld, output: String) {
    world.review.e2e = true;
    world.pending_mock_output = Some(output);
    world.pending_mock_model = Some("test-model".to_string());
    world.pending_mock_usage = Some(false);
    world.temp_dir = None;
    world.raw_config = None;
    ensure_test_env(world);
    let binary = find_binary();
    world
        .env_vars
        .insert("WATN_BIN".to_string(), binary.display().to_string());
}

fn start_direct_review(world: &mut WatnWorld, script: &str) {
    let out = world
        .temp_dir
        .as_ref()
        .expect("e2e direct temp dir")
        .path()
        .join("stdout.txt");
    world
        .env_vars
        .insert("WATN_OUT".to_string(), out.display().to_string());
    world.review.stdout_path = Some(out);
    let session = start_pty_command(world, "sh", &["-c", script]);
    world.pty_session = Some(session);
}

#[when(expr = "I ask interactively for {string}")]
fn e2e_ask_interactively(world: &mut WatnWorld, question: String) {
    let command = world.review.candidate_command.clone();
    prepare_e2e_direct(world, command);
    world.env_vars.insert("WATN_QUESTION".to_string(), question);
    start_direct_review(world, r#""$WATN_BIN" "$WATN_QUESTION" > "$WATN_OUT""#);
}

pub(crate) fn run_eligible_x_review(world: &mut WatnWorld, question: String) {
    let command = world.review.candidate_command.clone();
    prepare_e2e_direct(world, command);
    world.env_vars.insert("WATN_QUESTION".to_string(), question);
    start_direct_review(world, r#""$WATN_BIN" -x "$WATN_QUESTION" > "$WATN_OUT""#);
}

#[when("I accept the candidate in the review surface")]
fn e2e_accept_direct_candidate(world: &mut WatnWorld) {
    let session = world.pty_session.as_mut().expect("e2e direct PTY session");
    pty_wait_for_label(session, "accept ·");
    pty_write(session, "\r");
    let session = world.pty_session.take().expect("e2e direct PTY session");
    let _ = finish_pty_session(world, session);
    if let Some(path) = &world.review.stdout_path {
        world.review.command_output = std::fs::read_to_string(path).unwrap_or_default();
    }
}

#[then(expr = "normal command output should contain only {string}")]
fn e2e_normal_output_only(world: &mut WatnWorld, expected: String) {
    assert_eq!(
        world.review.command_output.trim_end(),
        expected,
        "the command-output channel must contain only the accepted candidate"
    );
}

#[then("the review surface should not appear in normal command output")]
fn e2e_normal_output_no_review(world: &mut WatnWorld) {
    let output = &world.review.command_output;
    assert!(
        !output.contains("Review |") && !output.contains("accept") && !output.contains("purpose"),
        "review surface text leaked into the command-output channel: {output:?}"
    );
}

#[then(expr = "{string} should be printed once")]
fn e2e_printed_once(world: &mut WatnWorld, text: String) {
    assert_eq!(
        world.review.command_output.matches(&text).count(),
        1,
        "expected exactly one execution of the accepted candidate, output: {:?}",
        world.review.command_output
    );
}

#[then("no second execution confirmation should be shown")]
fn e2e_no_second_confirmation(world: &mut WatnWorld) {
    let terminal = world.output.as_deref().unwrap_or_default();
    assert!(
        !terminal.contains("Execute now?"),
        "eligible review -x must not show a second confirmation: {terminal:?}"
    );
}
