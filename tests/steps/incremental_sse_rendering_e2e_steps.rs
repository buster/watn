use cucumber::{given, then, when};
use std::thread;
use std::time::{Duration, Instant};

use crate::WatnWorld;

use super::{finish_pty_session, pty_snapshot, start_pty_session};

#[given(
    regex = r##"^a streaming provider flushes content "([^"]+)" and delays content "([^"]+)" while keeping the connection open$"##
)]
fn delayed_provider(world: &mut WatnWorld, first: String, second: String) {
    super::incremental_sse_rendering_steps::configure_delayed_content(world, first, second);
}

#[when(regex = r##"^I start the delayed streaming command `watn "([^"]*)"` in a terminal$"##)]
fn start_delayed_stream(world: &mut WatnWorld, question: String) {
    let session = start_pty_session(world, &[&question]);
    world.pty_session = Some(session);
}

#[then("the progress indicator is visible before the first streamed content")]
fn progress_before_content(world: &mut WatnWorld) {
    wait_for_terminal_text(world, "Asking");
    let output = pty_snapshot(world.pty_session.as_ref().expect("streaming PTY session"));
    assert!(
        !output.contains("printf first"),
        "content arrived before progress observation"
    );
}

#[then(
    regex = r##"^the first streamed content "([^"]+)" is visible before the provider releases the delayed event$"##
)]
fn first_content_before_release(world: &mut WatnWorld, first: String) {
    wait_for_terminal_text(world, &first);
    let output = pty_snapshot(world.pty_session.as_ref().expect("streaming PTY session"));
    assert!(output.contains(&first));
    assert!(
        !output.contains("printf second"),
        "delayed content arrived before release"
    );
}

#[then("the terminal shows spinner cleanup after the first streamed content")]
fn spinner_cleanup_after_content(world: &mut WatnWorld) {
    let output = pty_snapshot(world.pty_session.as_ref().expect("streaming PTY session"));
    assert!(
        output.contains("\x1b[2K"),
        "expected terminal clear-line evidence, got {output:?}"
    );
}

#[when("I release the delayed event and wait for watn to exit")]
fn release_delayed_event(world: &mut WatnWorld) {
    super::incremental_sse_rendering_steps::release_stream(world);
    if let Some(session) = world.pty_session.take() {
        finish_pty_session(world, session);
    }
}

#[then(regex = r##"^the terminal generated command line "([^"]+)" appears exactly once$"##)]
fn terminal_generated_command_once(world: &mut WatnWorld, command: String) {
    let output = world
        .output
        .as_deref()
        .expect("terminal output was not captured");
    assert_eq!(
        output.match_indices(&command).count(),
        1,
        "expected one generated command in terminal output: {output:?}"
    );
}

fn wait_for_terminal_text(world: &WatnWorld, text: &str) {
    let deadline = Instant::now() + Duration::from_secs(3);
    loop {
        let output = pty_snapshot(world.pty_session.as_ref().expect("streaming PTY session"));
        if output.contains(text) {
            return;
        }
        assert!(
            Instant::now() < deadline,
            "terminal did not contain {text:?}: {output:?}"
        );
        thread::sleep(Duration::from_millis(25));
    }
}
