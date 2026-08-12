use cucumber::{then, when};
use std::collections::HashMap;
use std::path::Path;
use std::process::Command;

use cucumber::given;

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
    assert!(
        !Path::new(&path).exists(),
        "expected file not to exist: {path}"
    );
}

#[then("the preserved request comment should be a single line")]
fn preserved_request_comment_single_line(world: &mut WatnWorld) {
    let buffer = current_buffer(world);
    assert_eq!(
        buffer.matches('\n').count(),
        1,
        "the request comment should contain no embedded line breaks"
    );
    assert!(
        buffer.starts_with("# "),
        "the preserved request should be a comment"
    );
}

#[given("an installed Zsh and Fish shortcut")]
fn installed_zsh_and_fish_shortcut(world: &mut WatnWorld) {
    let temp = tempfile::tempdir().expect("create Zsh and Fish temp dir");
    let home = temp.path().join("home");
    let fish_config = home.join(".config/fish");
    std::fs::create_dir_all(&fish_config).expect("create Fish config directory");
    let environment = watn::shell_shortcut::ShellEnvironment {
        home: home.clone(),
        xdg_config_home: Some(home.join(".config")),
        shell: Some("/bin/bash".to_string()),
    };
    let report = watn::shell_shortcut::install_with_environment(
        &[
            watn::shell_shortcut::Shell::Zsh,
            watn::shell_shortcut::Shell::Fish,
        ],
        &environment,
    );
    assert!(
        report.is_success(),
        "Zsh and Fish fixture report: {report:?}"
    );
    world.temp_dir = Some(temp);
    world.shortcut_targets = HashMap::from([
        ("zsh".to_string(), home.join(".zshrc")),
        ("fish".to_string(), fish_config.join("config.fish")),
    ]);
}

#[then("the Zsh configuration should keep the request above the generated command")]
fn zsh_request_comment(world: &mut WatnWorld) {
    let content = std::fs::read_to_string(world.shortcut_targets.get("zsh").unwrap())
        .expect("read Zsh target");
    assert!(content.contains("comment=${question//$'\\n'/ }"));
    assert!(content.contains("BUFFER=\"# $comment\"$'\\n'\"$result\""));
}

#[then("the Fish configuration should keep the request above the generated command")]
fn fish_request_comment(world: &mut WatnWorld) {
    let content = std::fs::read_to_string(world.shortcut_targets.get("fish").unwrap())
        .expect("read Fish target");
    assert!(content.contains("set -l comment (string replace -a '\\n' ' ' -- \"$question\")"));
    assert!(content
        .contains("set -l buffer (printf '%s\\n%s' \"# $comment\" \"$result\" | string collect)"));
    assert!(content.contains("commandline -r -- \"$buffer\""));
}
