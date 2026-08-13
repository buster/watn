use cucumber::{given, then, when};

use super::{build_config, pty_snapshot, pty_write};
use crate::WatnWorld;

fn latest_page(output: &str) -> &str {
    output
        .rfind("Page")
        .map(|index| &output[index..])
        .unwrap_or(output)
}

fn wait_for_active_page(session: &super::PtySession, title: &str) -> String {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(3);
    loop {
        let output = pty_snapshot(session);
        if latest_page(&output).contains(title) {
            return output;
        }
        if std::time::Instant::now() >= deadline {
            panic!("active setup page {title:?} was not rendered: {output:?}");
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

#[when("advance to the small model question")]
fn advance_to_small_model_question(world: &mut WatnWorld) {
    let session = world.pty_session.as_mut().expect("setup PTY session");
    for _ in 0..3 {
        pty_write(session, "\r");
        std::thread::sleep(std::time::Duration::from_millis(150));
    }
    wait_for_active_page(session, "Small Model");
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
