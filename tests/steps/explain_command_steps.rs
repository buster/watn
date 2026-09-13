use cucumber::{then, when};

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
