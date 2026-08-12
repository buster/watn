use cucumber::{then, when};
use std::path::Path;
use std::process::Command;

use crate::WatnWorld;

fn current_buffer(world: &WatnWorld) -> &str {
    world
        .shortcut_output
        .as_deref()
        .unwrap_or_default()
        .split("LINE<<")
        .nth(1)
        .and_then(|value| value.split(">>").next())
        .expect("generated Bash buffer output")
}

#[when(regex = r##"^I run the Bash widget with current input containing \"([^\"]*)\"$"##)]
fn run_bash_widget_with_escaped_input(world: &mut WatnWorld, input: String) {
    let input = input
        .replace("\\n", "\n")
        .replace("\\r", "\r")
        .replace("\\t", "\t");
    super::interactive_shell_shortcut_steps::run_bash_widget(world, input);
}

#[when("I execute the resulting Bash buffer")]
fn execute_resulting_bash_buffer(world: &mut WatnWorld) {
    let buffer = current_buffer(world);

    for path in [
        "/tmp/watn-shortcut-executed",
        "/tmp/watn-shortcut-comment-should-not-run",
    ] {
        let _ = std::fs::remove_file(path);
    }

    let result = Command::new("bash")
        .args(["--noprofile", "--norc", "-c", buffer])
        .output()
        .expect("execute generated Bash buffer");
    world.exit_status = result.status.code();
    world.stderr_output = Some(String::from_utf8_lossy(&result.stderr).to_string());
}

#[then(regex = r##"^the file \"([^\"]*)\" should exist$"##)]
fn file_should_exist(_world: &mut WatnWorld, path: String) {
    assert!(Path::new(&path).is_file(), "expected file to exist: {path}");
}

#[then(regex = r##"^the file \"([^\"]*)\" should not exist$"##)]
fn file_should_not_exist(_world: &mut WatnWorld, path: String) {
    assert!(!Path::new(&path).exists(), "expected file not to exist: {path}");
}

#[then("the preserved request comment should be a single line")]
fn preserved_request_comment_single_line(world: &mut WatnWorld) {
    let buffer = current_buffer(world);
    assert_eq!(
        buffer.matches('\n').count(),
        1,
        "the request comment should contain no embedded line breaks"
    );
    assert!(buffer.starts_with("# "), "the preserved request should be a comment");
}
