use cucumber::{then, when};
use std::process::Command;

use crate::WatnWorld;

#[when("I run the built `watn completions bash` command")]
fn run_built_bash(world: &mut WatnWorld) {
    execute_built_bash(world);
}

#[then("stdout should contain the authoritative root options and subcommands")]
fn e2e_root_tree(world: &mut WatnWorld) {
    let output = world.output.as_deref().expect("completion stdout");
    for value in [
        "-1",
        "--small",
        "-2",
        "--normal",
        "-3",
        "--thinking",
        "--model",
        "-x",
        "--execute",
        "-v",
        "--verbose",
        "--provider",
        "--set-small",
        "--set-normal",
        "--set-thinking",
        "--help",
        "--version",
        "setup",
        "models",
        "provider",
        "completions",
    ] {
        assert!(
            output.contains(value),
            "missing root completion value {value}"
        );
    }
}

#[then("stdout should contain bash, elvish, fish, powershell, and zsh value suggestions")]
fn selector_suggestions(world: &mut WatnWorld) {
    let output = world.output.as_deref().expect("completion stdout");
    for shell in ["bash", "elvish", "fish", "powershell", "zsh"] {
        assert!(output.contains(shell), "missing shell suggestion {shell}");
    }
}

#[then("a second built Bash generation should be byte-for-byte identical")]
fn second_built_bash(world: &mut WatnWorld) {
    let first = world.output.clone().expect("first Bash completion output");
    execute_built_bash(world);
    assert_eq!(world.output.as_deref(), Some(first.as_str()));
}

fn execute_built_bash(world: &mut WatnWorld) {
    let binary = super::binary_from_env("WATN_TEST_SUPPORT_DEBUG_BIN");
    let mut command = Command::new(binary);
    command.args(["completions", "bash"]);
    command.env_clear();
    command.env("PATH", std::env::var("PATH").expect("PATH"));
    super::apply_env(world, &mut command);
    let output = command.output().expect("run built completion command");
    world.output = Some(String::from_utf8_lossy(&output.stdout).to_string());
    world.stderr_output = Some(String::from_utf8_lossy(&output.stderr).to_string());
    world.exit_status = output.status.code();
}
