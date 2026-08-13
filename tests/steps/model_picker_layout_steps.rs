use cucumber::{then, when};

use crate::WatnWorld;

use super::{finish_pty_session, pty_snapshot, pty_wait_for_label, pty_write, start_pty_session};

fn wait_for_page(session: &super::PtySession, title: &str) {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(3);
    loop {
        let output = pty_snapshot(session);
        if output.rfind("Page").is_some_and(|index| {
            let current = &output[index..];
            title.split_whitespace().all(|word| current.contains(word))
        }) {
            return;
        }
        if std::time::Instant::now() >= deadline {
            panic!("model picker page {title:?} was not rendered: {output:?}");
        }
        std::thread::sleep(std::time::Duration::from_millis(25));
    }
}

fn assert_label(output: &str, label: &str) {
    for word in label.split_whitespace() {
        assert!(
            output.contains(word),
            "missing {word:?} in model picker output: {output:?}"
        );
    }
}

#[when("I start `watn models` in a terminal")]
fn start_models_in_terminal(world: &mut WatnWorld) {
    let session = start_pty_session(world, &["models"]);
    world.pty_session = Some(session);
    let session = world
        .pty_session
        .as_ref()
        .expect("model picker PTY session");
    pty_wait_for_label(session, "Setup");
}

#[then(regex = r#"^the model picker should show a bordered \"([^\"]+)\" panel$"#)]
fn model_picker_bordered_panel(world: &mut WatnWorld, title: String) {
    let session = world
        .pty_session
        .as_ref()
        .expect("model picker PTY session");
    let output = pty_wait_for_label(session, "Setup");
    assert!(
        output.contains('┌'),
        "model picker is not bordered: {output:?}"
    );
    assert_label(
        &output,
        if title == "Model picker" {
            "Setup"
        } else {
            title.as_str()
        },
    );
}

#[then("the model picker should show the visible Small and Normal model tabs")]
fn model_picker_tier_tabs(world: &mut WatnWorld) {
    let session = world
        .pty_session
        .as_ref()
        .expect("model picker PTY session");
    let output = pty_snapshot(session);
    assert_label(&output, "Setup pages");
    for tier in ["Small Model", "Normal Model"] {
        assert_label(&output, tier);
    }
}

#[then("the model picker should show models in aligned columns")]
fn model_picker_table_columns(world: &mut WatnWorld) {
    let session = world
        .pty_session
        .as_ref()
        .expect("model picker PTY session");
    let output = pty_snapshot(session);
    assert_label(&output, "Model");
    for column in ["Model", "Context", "Pricing", "Features"] {
        assert_label(&output, column);
    }
}

#[then("the model picker should show a scrollbar for the model list")]
fn model_picker_scrollbar(world: &mut WatnWorld) {
    let session = world
        .pty_session
        .as_ref()
        .expect("model picker PTY session");
    let output = pty_snapshot(session);
    assert!(
        output.contains('#'),
        "model picker scrollbar is missing: {output:?}"
    );
}

#[when("I move to the next model and advance to the normal tier")]
fn move_to_next_model_and_advance(world: &mut WatnWorld) {
    let session = world
        .pty_session
        .as_mut()
        .expect("model picker PTY session");
    pty_write(session, "\x1b[B");
    wait_for_page(session, "Small Model");
    pty_write(session, "\r");
    wait_for_page(session, "Small Reasoning");
    pty_write(session, "\r");
    wait_for_page(session, "Normal Model");
}

#[then(regex = r#"^the model picker should show the active tier \"([^\"]+)\"$"#)]
fn model_picker_active_tier(world: &mut WatnWorld, tier: String) {
    let session = world
        .pty_session
        .as_ref()
        .expect("model picker PTY session");
    let expected = match tier.as_str() {
        "small" => "Small Model",
        "normal" => "Normal Model",
        "thinking" => "Thinking Model",
        other => other,
    };
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(3);
    let output = loop {
        let output = pty_snapshot(session);
        if let Some(index) = output.rfind("Page") {
            let current_frame = &output[index..];
            if expected
                .split_whitespace()
                .all(|word| current_frame.contains(word))
            {
                break output;
            }
        }
        if std::time::Instant::now() >= deadline {
            panic!("active tier {tier:?} was not rendered: {output:?}");
        }
        std::thread::sleep(std::time::Duration::from_millis(25));
    };
    assert_label(&output, expected);
}

#[then("the model picker should keep the selected row visible")]
fn model_picker_selected_row(world: &mut WatnWorld) {
    let session = world
        .pty_session
        .as_ref()
        .expect("model picker PTY session");
    let output = pty_snapshot(session);
    assert!(
        output.contains("model-02"),
        "selected row was not rendered: {output:?}"
    );

    let session = world.pty_session.take().expect("model picker PTY session");
    let mut session = session;
    pty_write(&mut session, "\x03");
    finish_pty_session(world, session);
}
