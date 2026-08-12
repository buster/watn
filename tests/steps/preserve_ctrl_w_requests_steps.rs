use cucumber::{then, when};
use std::path::Path;
use std::process::Command;

use crate::WatnWorld;

#[when("I execute the resulting Bash buffer")]
fn execute_resulting_bash_buffer(world: &mut WatnWorld) {
    let output = world.shortcut_output.as_deref().unwrap_or_default();
    let buffer = output
        .split("LINE<<")
        .nth(1)
        .and_then(|value| value.split(">>").next())
        .expect("generated Bash buffer output");

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
