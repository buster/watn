use cucumber::{given, then, when};

use crate::WatnWorld;

fn prepare_explain_pty(world: &mut WatnWorld, script: &str) {
    let _ = std::fs::remove_file("/tmp/watn-explain-should-not-run");
    let _ = std::fs::remove_file("/tmp/watn-explain-enter-should-not-run");
    super::ensure_test_env(world);
    let binary = super::find_binary();
    let dir = world
        .temp_dir
        .as_ref()
        .expect("explain temp dir")
        .path()
        .to_path_buf();
    let out = dir.join("explain-stdout.txt");
    let _ = std::fs::remove_file(&out);
    world
        .env_vars
        .insert("WATN_BIN".to_string(), binary.display().to_string());
    world
        .env_vars
        .insert("WATN_OUT".to_string(), out.display().to_string());
    world.review.stdout_path = Some(out);
    let session = super::start_pty_command(world, "sh", &["-c", script]);
    world.pty_session = Some(session);
}

fn close_explain_card(world: &mut WatnWorld, key: &str) -> String {
    {
        let session = world.pty_session.as_ref().expect("explain pty session");
        super::pty_wait_for_label(session, "esc close");
    }
    let mut session = world.pty_session.take().expect("explain pty session");
    super::pty_write(&mut session, key);
    let transcript = super::finish_pty_session(world, session);
    if let Some(path) = &world.review.stdout_path {
        world.review.command_output = std::fs::read_to_string(path).unwrap_or_default();
    }
    transcript
}

#[when("I run `watn explain` with this single argument in a terminal, then press Enter:")]
fn run_explain_then_enter(world: &mut WatnWorld, step: &cucumber::gherkin::Step) {
    let command = step
        .docstring
        .as_deref()
        .expect("command docstring")
        .trim()
        .to_string();
    world
        .env_vars
        .insert("WATN_COMMAND".to_string(), command);
    prepare_explain_pty(world, r#""$WATN_BIN" explain "$WATN_COMMAND" > "$WATN_OUT""#);
    close_explain_card(world, "\r");
}

#[then("the command-output channel should contain no command")]
fn command_output_contains_no_command(world: &mut WatnWorld) {
    assert!(
        world.review.command_output.trim().is_empty(),
        "command-output channel received: {:?}",
        world.review.command_output
    );
}

fn card_plain_text(world: &WatnWorld) -> String {
    let rendered = world.output.clone().unwrap_or_default();
    let plain = watn::review::sanitize_terminal_text(&rendered);
    let plain = plain.replace(['│', '┌', '┐', '└', '┘', '─'], " ");
    plain.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn assert_card_contains(world: &WatnWorld, needle: &str) {
    let plain = card_plain_text(world);
    let collapsed = needle.split_whitespace().collect::<Vec<_>>().join(" ");
    assert!(
        plain.contains(&collapsed),
        "explanation card should show {needle:?}, got:\n{plain}"
    );
}

#[when("I run `watn explain` with this single argument:")]
fn run_explain_single_argument(world: &mut WatnWorld, step: &cucumber::gherkin::Step) {
    let command = step
        .docstring
        .as_deref()
        .expect("command docstring")
        .trim()
        .to_string();
    world
        .env_vars
        .insert("WATN_COMMAND".to_string(), command);
    prepare_explain_pty(world, r#""$WATN_BIN" explain "$WATN_COMMAND" > "$WATN_OUT""#);
    close_explain_card(world, "\x1b");
}

#[then("the explanation card should show the stage:")]
fn card_shows_stage(world: &mut WatnWorld, step: &cucumber::gherkin::Step) {
    let needle = step
        .docstring
        .as_deref()
        .expect("stage docstring")
        .trim()
        .to_string();
    assert_card_contains(world, &needle);
}

#[then("the explanation card should show purpose-unavailable")]
fn card_shows_purpose_unavailable(world: &mut WatnWorld) {
    assert_card_contains(world, "purpose-unavailable");
}

#[when("I run `watn explain --` with this single argument:")]
fn run_explain_terminator(world: &mut WatnWorld, step: &cucumber::gherkin::Step) {
    let command = step
        .docstring
        .as_deref()
        .expect("command docstring")
        .trim()
        .to_string();
    world
        .env_vars
        .insert("WATN_COMMAND".to_string(), command);
    prepare_explain_pty(world, r#""$WATN_BIN" explain -- "$WATN_COMMAND" > "$WATN_OUT""#);
    close_explain_card(world, "\x1b");
}

fn write_explain_stdin(world: &mut WatnWorld, contents: &str) -> String {
    let dir = world
        .temp_dir
        .as_ref()
        .expect("explain temp dir")
        .path()
        .to_path_buf();
    let path = dir.join("explain-stdin.txt");
    std::fs::write(&path, contents).expect("write explain stdin");
    path.display().to_string()
}

/// Strip terminal escape sequences without collapsing line breaks, so stage
/// boundaries can be asserted on the rendered rows.
fn strip_ansi_keep_lines(value: &str) -> String {
    let mut plain = String::with_capacity(value.len());
    let mut chars = value.chars().peekable();
    while let Some(character) = chars.next() {
        if character != '\u{1b}' {
            plain.push(character);
            continue;
        }
        match chars.peek() {
            Some('[') => {
                chars.next();
                for escaped in chars.by_ref() {
                    if ('@'..='~').contains(&escaped) {
                        break;
                    }
                }
            }
            Some(']') => {
                chars.next();
                while let Some(escaped) = chars.next() {
                    if escaped == '\u{7}' {
                        break;
                    }
                    if escaped == '\u{1b}' && chars.peek() == Some(&'\\') {
                        chars.next();
                        break;
                    }
                }
            }
            _ => {}
        }
    }
    plain
}

#[when("I run `watn explain -` in a terminal with this command on standard input:")]
fn run_explain_stdin_marker(world: &mut WatnWorld, step: &cucumber::gherkin::Step) {
    let command = step
        .docstring
        .as_deref()
        .expect("command docstring")
        .trim()
        .to_string();
    let stdin_path = write_explain_stdin(world, &command);
    world
        .env_vars
        .insert("WATN_STDIN".to_string(), stdin_path);
    prepare_explain_pty(world, r#""$WATN_BIN" explain - < "$WATN_STDIN" > "$WATN_OUT""#);
    close_explain_card(world, "\x1b");
}

#[then(expr = "the explanation card should show the stages {string} and {string} as separate stages")]
fn card_shows_stages_separately(world: &mut WatnWorld, first: String, second: String) {
    let rendered = match world.pty_session.as_ref() {
        Some(session) => super::pty_snapshot(session),
        None => world.output.clone().unwrap_or_default(),
    };
    let plain = strip_ansi_keep_lines(&rendered);
    let lines: Vec<&str> = plain
        .lines()
        .map(|line| line.trim_end_matches('\r'))
        .collect();
    assert!(
        lines.iter().any(|line| line.contains(&first)),
        "the card should show stage {first:?} on its own line, got:\n{plain}"
    );
    assert!(
        lines.iter().any(|line| line.contains(&second)),
        "the card should show stage {second:?} on its own line, got:\n{plain}"
    );
    assert!(
        !lines
            .iter()
            .any(|line| line.contains(&first) && line.contains(&second)),
        "stages {first:?} and {second:?} must not be joined on one line, got:\n{plain}"
    );
}

#[when(expr = "I run `watn explain` in a terminal with the command {string} piped on standard input")]
fn run_explain_piped_command(world: &mut WatnWorld, command: String) {
    let stdin_path = write_explain_stdin(world, &command);
    world
        .env_vars
        .insert("WATN_STDIN".to_string(), stdin_path);
    prepare_explain_pty(world, r#""$WATN_BIN" explain < "$WATN_STDIN" > "$WATN_OUT""#);
    close_explain_card(world, "\x1b");
}

#[when(expr = "I run watn explain with the argument {string} in a terminal with the command {string} on standard input")]
fn run_explain_argument_over_stdin(world: &mut WatnWorld, argument: String, stdin_command: String) {
    world
        .env_vars
        .insert("WATN_ARG".to_string(), argument);
    let stdin_path = write_explain_stdin(world, &stdin_command);
    world
        .env_vars
        .insert("WATN_STDIN".to_string(), stdin_path);
    prepare_explain_pty(
        world,
        r#""$WATN_BIN" explain "$WATN_ARG" < "$WATN_STDIN" > "$WATN_OUT""#,
    );
    close_explain_card(world, "\x1b");
}

#[then(expr = "the explanation card should not show the text {string}")]
fn card_does_not_show_text(world: &mut WatnWorld, needle: String) {
    let plain = card_plain_text(world);
    let collapsed = needle.split_whitespace().collect::<Vec<_>>().join(" ");
    assert!(
        !plain.contains(&collapsed),
        "explanation card should not show {needle:?}, got:\n{plain}"
    );
}

#[when("I run `watn explain` with an empty argument")]
fn run_explain_empty_argument(world: &mut WatnWorld) {
    super::run_binary_with_state(world, &["explain", ""], None);
}

#[when("I run `watn explain` with empty standard input")]
fn run_explain_empty_stdin(world: &mut WatnWorld) {
    super::run_binary_with_state(world, &["explain"], Some(""));
}

#[then("watn should report a usage error")]
fn watn_reports_usage_error(world: &mut WatnWorld) {
    assert_eq!(
        world.exit_status,
        Some(2),
        "watn should exit with a usage error, got {:?}; stderr: {:?}",
        world.exit_status,
        world.stderr_output
    );
    assert!(
        !world
            .stderr_output
            .as_deref()
            .unwrap_or_default()
            .trim()
            .is_empty(),
        "a usage error must be reported on stderr"
    );
}

#[then("no explanation card should open")]
fn no_explanation_card(world: &mut WatnWorld) {
    let output = world.output.as_deref().unwrap_or_default();
    assert!(
        !output.contains("esc close"),
        "no explanation card should open, got:\n{output}"
    );
}

#[when(expr = "I run watn explain with the arguments {string} and {string}")]
fn run_explain_two_arguments(world: &mut WatnWorld, first: String, second: String) {
    super::run_binary_with_state(world, &["explain", &first, &second], None);
}

#[given("a configured provider without a usable credential")]
fn configured_provider_without_credential(world: &mut WatnWorld) {
    let server = httpmock::MockServer::start();
    let port = server.port();
    let mock = server.mock(|when, then| {
        when.method(httpmock::Method::POST)
            .path("/chat/completions");
        then.status(200)
            .header("Content-Type", "text/event-stream")
            .body("data: {\"id\":\"1\",\"choices\":[{\"index\":0,\"delta\":{\"content\":\"output\"},\"finish_reason\":\"stop\"}]}\ndata: [DONE]\n");
    });
    let mock_id = mock.id;
    drop(mock);
    world.mock_server = crate::MockServerWrap(Some(server), Some(mock_id));
    world.raw_config = Some(format!(
        "[defaults]\nprovider = \"test\"\nmodel = \"test-model\"\n\n[providers.test]\nendpoint = \"http://127.0.0.1:{port}\"\n"
    ));
}

#[then("no provider request should have been made")]
fn no_provider_request(world: &mut WatnWorld) {
    let server = world.mock_server.0.as_ref().expect("mock server");
    let id = world.mock_server.1.expect("registered provider mock");
    assert_eq!(
        httpmock::Mock::new(id, server).calls(),
        0,
        "the provider must not be contacted when no credential is usable"
    );
}

#[given("a configured provider whose explanation request fails")]
fn configured_provider_request_fails(world: &mut WatnWorld) {
    world.pending_mock_auth_fail = true;
    world.pending_mock_model = Some("test-model".to_string());
    world.pending_mock_output = Some("output".to_string());
}

#[given("a configured provider whose explanation does not cover the command")]
fn configured_provider_not_covering(world: &mut WatnWorld) {
    world.pending_mock_model = Some("test-model".to_string());
    world.pending_mock_output = Some(
        r#"{"review_version":1,"command":"git log --oneline","stages":[{"stage_text":"unrelated stage","purpose":"not covering"}],"purpose_status":"ready"}"#
            .to_string(),
    );
    world.pending_mock_usage = Some(false);
}
