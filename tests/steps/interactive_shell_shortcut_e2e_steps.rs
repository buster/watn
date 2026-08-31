use cucumber::{then, when};
use std::io::ErrorKind;
use std::path::Path;
use std::process::Command;

use crate::WatnWorld;

fn captured_bash_line(world: &WatnWorld) -> &str {
    world
        .shortcut_output
        .as_deref()
        .unwrap_or_default()
        .split("LINE<<")
        .nth(1)
        .and_then(|value| value.split(">>").next())
        .expect("Bash process line output")
}

fn captured_bash_history(world: &WatnWorld) -> &str {
    super::interactive_shell_shortcut_steps::current_history(world)
}

fn assert_shell_syntax(shell: &str, name: &str, path: &Path, required: bool) {
    let result = Command::new(shell)
        .args(["-n", path.to_str().expect("UTF-8 shell target path")])
        .output();
    let result = match result {
        Ok(result) => result,
        Err(error) if !required && error.kind() == ErrorKind::NotFound => return,
        Err(error) => panic!("run {name} syntax check: {error}"),
    };
    assert!(
        result.status.success(),
        "{name} rejected generated configuration: {}",
        String::from_utf8_lossy(&result.stderr)
    );
}

#[then("the generated Bash configuration should pass a Bash syntax check")]
fn bash_syntax_check(world: &mut WatnWorld) {
    let path = world.shortcut_targets.get("bash").expect("Bash target");
    assert_shell_syntax("bash", "Bash", path, true);
}

#[then("the generated Zsh configuration should pass a Zsh syntax check")]
fn zsh_syntax_check(world: &mut WatnWorld) {
    let path = world.shortcut_targets.get("zsh").expect("Zsh target");
    assert_shell_syntax("zsh", "Zsh", path, std::env::var_os("CI").is_some());
}

#[then("the generated Fish configuration should pass a Fish syntax check")]
fn fish_syntax_check(world: &mut WatnWorld) {
    let path = world.shortcut_targets.get("fish").expect("Fish target");
    assert_shell_syntax("fish", "Fish", path, std::env::var_os("CI").is_some());
}

#[when(
    regex = r##"^I run the generated Bash widget through Bash with current input \"([^\"]*)\"$"##
)]
fn run_generated_bash(world: &mut WatnWorld, input: String) {
    super::interactive_shell_shortcut_steps::run_bash_widget(world, input);
}

#[then(regex = r##"^the Bash process command line should be exactly \"([^\"]*)\"$"##)]
fn bash_process_line(world: &mut WatnWorld, expected: String) {
    let line = captured_bash_line(world);
    assert_eq!(line, expected.replace("\\n", "\n"));
    let temp = world.temp_dir.as_ref().expect("Bash E2E temp dir");
    let log = std::fs::read_to_string(temp.path().join("watn-invocations.log"))
        .expect("Bash E2E invocation log");
    assert_eq!(log, "find all images\n");
}

#[then("the Bash process should not execute the replacement text")]
fn bash_process_no_eval(_world: &mut WatnWorld) {
    assert!(!std::path::Path::new("/tmp/watn-shortcut-should-not-run").exists());
}

#[then(
    regex = r##"^the Bash process should record the request \"([^\"]*)\" in the shell history$"##
)]
fn bash_process_records_request(world: &mut WatnWorld, comment: String) {
    let history = captured_bash_history(world);
    assert!(
        history.contains(&comment),
        "the shell history should contain the request comment {comment:?}, got: {history:?}"
    );
}
