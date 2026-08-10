use cucumber::{then, when};

use crate::WatnWorld;
use super::{finish_pty_session, pty_snapshot, pty_wait_for_label, pty_write};

fn assert_label(output: &str, label: &str) {
    for word in label.split_whitespace() {
        assert!(output.contains(word), "missing {word:?} in provider setup output: {output:?}");
    }
}

#[then(regex = r#"^the provider setup should show a bordered \"([^\"]+)\" panel$"#)]
fn provider_setup_bordered_panel(world: &mut WatnWorld, title: String) {
    let session = world.pty_session.as_ref().expect("provider PTY session");
    let output = pty_wait_for_label(session, &title);
    assert!(output.contains('┌'), "provider setup is not bordered: {output:?}");
    assert_label(&output, &title);
}

#[then("provider setup should show a selectable credential source list")]
fn provider_setup_credential_source_list(world: &mut WatnWorld) {
    let session = world.pty_session.as_ref().expect("provider PTY session");
    let output = pty_snapshot(session);
    assert_label(&output, "Credential source");
    assert_label(&output, "Paste credential");
    assert_label(&output, "Environment variable");
}

#[then("provider setup should show provider details in aligned rows")]
fn provider_setup_details_table(world: &mut WatnWorld) {
    let session = world.pty_session.as_ref().expect("provider PTY session");
    let output = pty_snapshot(session);
    assert_label(&output, "Provider details");
    for label in ["Field", "Endpoint", "Credential", "Value"] {
        assert_label(&output, label);
    }
}

#[then("provider setup should show setup guidance as a paragraph")]
fn provider_setup_guidance_paragraph(world: &mut WatnWorld) {
    let session = world.pty_session.as_ref().expect("provider PTY session");
    let output = pty_snapshot(session);
    assert_label(&output, "Guidance");
    assert_label(&output, "OpenAI-compatible endpoint");
}

#[when("I enter an invalid endpoint in provider setup")]
fn enter_invalid_provider_endpoint(world: &mut WatnWorld) {
    let session = world.pty_session.as_mut().expect("provider PTY session");
    pty_write(session, "\x15not a URL\r");
    std::thread::sleep(std::time::Duration::from_millis(100));
}

#[then(regex = r#"^provider setup should show validation message \"([^\"]+)\"$"#)]
fn provider_setup_validation_message(world: &mut WatnWorld, message: String) {
    let session = world.pty_session.as_ref().expect("provider PTY session");
    let output = pty_wait_for_label(session, &message);
    assert_label(&output, &message);
}

#[when(regex = r#"^I restore the default endpoint and enter pasted credential \"([^\"]+)\" in provider setup$"#)]
fn restore_endpoint_and_enter_credential(world: &mut WatnWorld, credential: String) {
    let session = world.pty_session.as_mut().expect("provider PTY session");
    pty_write(session, "\x15https://openrouter.ai/api/v1\r");
    std::thread::sleep(std::time::Duration::from_millis(100));
    pty_write(session, "\r");
    std::thread::sleep(std::time::Duration::from_millis(100));
    pty_write(session, &credential);
    world.pending_config.insert("layout_credential".to_string(), credential);
    std::thread::sleep(std::time::Duration::from_millis(100));
}

#[then("provider setup should mask pasted credentials")]
fn provider_setup_masks_credentials(world: &mut WatnWorld) {
    let session = world.pty_session.as_ref().expect("provider PTY session");
    let credential = world
        .pending_config
        .get("layout_credential")
        .expect("layout credential")
        .clone();
    let output = pty_wait_for_label(session, "Value");
    let masked_count = output.chars().filter(|character| *character == '*').count();
    assert!(
        masked_count >= credential.chars().count(),
        "masked credential missing: {output:?}"
    );
    assert!(!output.contains(&credential), "credential leaked in provider output: {output:?}");

    let session = world.pty_session.take().expect("provider PTY session");
    let mut session = session;
    pty_write(&mut session, "\x1b");
    finish_pty_session(world, session);
}
