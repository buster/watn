use cucumber::{given, then, when};

use crate::WatnWorld;

fn explain_purposes(command: &str) -> Vec<String> {
    watn::review::derive_command_flow(command)
        .stages
        .iter()
        .enumerate()
        .map(|(index, _)| format!("Explains stage {}", index + 1))
        .collect()
}

fn wait_for_card_text(world: &WatnWorld, needle: &str) {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(3);
    loop {
        if super::explain_command_steps::card_plain_text(world).contains(needle) {
            return;
        }
        if std::time::Instant::now() >= deadline {
            panic!(
                "the explanation card did not show {needle:?}; card:\n{}",
                super::explain_command_steps::card_plain_text(world)
            );
        }
        std::thread::sleep(std::time::Duration::from_millis(25));
    }
}

#[given("a configured provider whose explanation covers the command:")]
fn configured_provider_covering(world: &mut WatnWorld, step: &cucumber::gherkin::Step) {
    let command = step
        .docstring
        .as_deref()
        .expect("command docstring")
        .trim()
        .to_string();
    let stages: Vec<serde_json::Value> = watn::review::derive_command_flow(&command)
        .stages
        .iter()
        .enumerate()
        .map(|(index, stage)| {
            serde_json::json!({
                "stage_text": &stage.stage_text,
                "purpose": format!("Explains stage {}", index + 1),
            })
        })
        .collect();
    let payload = serde_json::json!({
        "review_version": 1,
        "command": command,
        "stages": stages,
        "purpose_status": "ready",
    })
    .to_string();
    world.pending_mock_model = Some("test-model".to_string());
    world.pending_mock_output = Some(payload);
    world.pending_mock_usage = Some(false);
    world.env_vars.insert("WATN_COMMAND".to_string(), command);
}

#[when("I run `watn explain` with that command as one argument in a terminal")]
fn run_explain_that_command(world: &mut WatnWorld) {
    super::explain_command_steps::prepare_explain_pty(
        world,
        r#""$WATN_BIN" explain "$WATN_COMMAND" > "$WATN_OUT""#,
    );
    let session = world.pty_session.as_ref().expect("explain pty session");
    super::pty_wait_for_label(session, "esc close");
}

#[then("the explanation card should show each model-written stage purpose")]
fn card_shows_each_purpose(world: &mut WatnWorld) {
    let command = world
        .env_vars
        .get("WATN_COMMAND")
        .expect("e2e explain command")
        .clone();
    let purposes = explain_purposes(&command);
    assert!(!purposes.is_empty(), "the explained command needs stages");
    wait_for_card_text(world, &purposes[0]);
    for purpose in purposes.iter().skip(1) {
        {
            let session = world
                .pty_session
                .as_mut()
                .expect("live explain pty session");
            super::pty_write(session, "\x1b[C");
        }
        wait_for_card_text(world, purpose);
    }
}

#[then(expr = "the explanation card should show the close hint {string}")]
fn card_shows_close_hint(world: &mut WatnWorld, hint: String) {
    let plain = super::explain_command_steps::card_plain_text(world);
    let collapsed = hint.split_whitespace().collect::<Vec<_>>().join(" ");
    assert!(
        plain.contains(&collapsed),
        "the explanation card should show the close hint {hint:?}, got:\n{plain}"
    );
}

#[then(expr = "the explanation card should name {string}")]
fn card_names(world: &mut WatnWorld, name: String) {
    let plain = super::explain_command_steps::card_plain_text(world);
    assert!(
        plain.contains(&name),
        "the explanation card should name {name:?}, got:\n{plain}"
    );
}

#[then("the explanation card should not show internal identifiers")]
fn card_hides_internal_identifiers(world: &mut WatnWorld) {
    let plain = super::explain_command_steps::card_plain_text(world);
    assert!(
        !plain.contains("explain_only"),
        "the explanation card must not show internal identifiers, got:\n{plain}"
    );
}

#[when("I close the explanation card")]
fn close_explanation_card(world: &mut WatnWorld) {
    let session = world.pty_session.as_ref().expect("live explain pty session");
    super::pty_wait_for_label(session, "esc close");
    let mut session = world.pty_session.take().expect("live explain pty session");
    super::pty_write(&mut session, "\x1b");
    let transcript = super::finish_pty_session(world, session);
    if let Some(path) = &world.review.stdout_path {
        world.review.command_output = std::fs::read_to_string(path).unwrap_or_default();
    }
    let plain = watn::review::sanitize_terminal_text(&transcript);
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(
        "givn/changes/explain-command/evidence/visual/developer-explains-an-existing-command-in-the-review-card",
    );
    std::fs::create_dir_all(&dir).expect("create evidence directory");
    std::fs::write(dir.join("transcript.txt"), plain).expect("write evidence transcript");
}
