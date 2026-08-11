use cucumber::{then, when};
use std::process::Command;

use crate::WatnWorld;

#[then("the generated Bash configuration should pass a Bash syntax check")]
fn bash_syntax_check(world: &mut WatnWorld) {
    let path = world.shortcut_targets.get("bash").expect("Bash target");
    let result = Command::new("bash")
        .args(["-n", path.to_str().expect("UTF-8 Bash target path")])
        .output()
        .expect("run Bash syntax check");
    assert!(
        result.status.success(),
        "Bash rejected generated configuration: {}",
        String::from_utf8_lossy(&result.stderr)
    );
}

#[then("the generated Fish configuration should pass a Fish syntax check")]
fn fish_syntax_check(world: &mut WatnWorld) {
    let path = world.shortcut_targets.get("fish").expect("Fish target");
    let result = Command::new("fish")
        .args(["-n", path.to_str().expect("UTF-8 Fish target path")])
        .output()
        .expect("run Fish syntax check");
    assert!(
        result.status.success(),
        "Fish rejected generated configuration: {}",
        String::from_utf8_lossy(&result.stderr)
    );
}

#[when(
    regex = r##"^I run the generated Bash widget through Bash with current input \"([^\"]*)\"$"##
)]
fn run_generated_bash(world: &mut WatnWorld, input: String) {
    super::interactive_shell_shortcut_steps::run_bash_widget(world, input);
}

#[then(regex = r##"^the Bash process command line should contain \"([^\"]*)\"$"##)]
fn bash_process_line(world: &mut WatnWorld, expected: String) {
    let output = world.shortcut_output.as_deref().unwrap_or_default();
    let line = output
        .split("LINE<<")
        .nth(1)
        .and_then(|value| value.split(">>").next())
        .expect("Bash process line output");
    assert_eq!(line, expected);
}

#[then("the Bash process should not execute the replacement text")]
fn bash_process_no_eval(_world: &mut WatnWorld) {
    assert!(!std::path::Path::new("/tmp/watn-shortcut-should-not-run").exists());
}
