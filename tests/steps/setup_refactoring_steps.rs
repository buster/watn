//! Strict steps for the non-interactive setup-refactoring contracts.

use cucumber::{given, then, when};

use super::{build_config, find_binary, run_binary_with_state};
use crate::WatnWorld;

#[then("stdout should be empty")]
fn stdout_empty(world: &mut WatnWorld) {
    assert_eq!(world.output.as_deref().unwrap_or_default(), "");
}

#[then("no config file should exist")]
fn no_config_file_should_exist(world: &mut WatnWorld) {
    let directory = world.temp_dir.as_ref().expect("isolated config directory");
    assert!(!directory.path().join("watn/config.toml").exists());
}

#[given("a complete configuration exists")]
fn complete_configuration_exists(world: &mut WatnWorld) {
    world.raw_config = Some(build_config(
        "test",
        Some(("small-model", "normal-model", "thinking-model")),
        Some(vec![(
            "test",
            "http://localhost:4000",
            "test-key",
            "small-model",
        )]),
        None,
        None,
        None,
    ));
    world.pending_mock_model = Some("small-model".to_string());
    world.pending_mock_output = Some("printf setup-test".to_string());
}

#[when("I run `watn provider`")]
fn run_removed_provider_command(world: &mut WatnWorld) {
    run_binary_with_state(world, &["provider"], None);
}

#[when("I run `watn --model alternate-model \"show changed files\"`")]
fn run_removed_model_option(world: &mut WatnWorld) {
    run_binary_with_state(
        world,
        &["--model", "alternate-model", "show changed files"],
        None,
    );
}

#[then("the command should be rejected as unavailable")]
fn command_rejected_as_unavailable(world: &mut WatnWorld) {
    assert_ne!(world.exit_status, Some(0));
    assert!(world
        .stderr_output
        .as_deref()
        .unwrap_or_default()
        .contains("removed setup command"));
}

#[then("the command should reject the removed provider option")]
fn removed_provider_option_rejected(world: &mut WatnWorld) {
    assert_ne!(world.exit_status, Some(0));
    assert!(world
        .stderr_output
        .as_deref()
        .unwrap_or_default()
        .contains("--provider"));
}

#[then("the command should reject the removed model option")]
fn removed_model_option_rejected(world: &mut WatnWorld) {
    assert_ne!(world.exit_status, Some(0));
    assert!(world
        .stderr_output
        .as_deref()
        .unwrap_or_default()
        .contains("--model"));
}

#[when("I run `watn --set-small alternate-model`")]
fn run_removed_assignment_option(world: &mut WatnWorld) {
    run_binary_with_state(world, &["--set-small", "alternate-model"], None);
}

#[then("the command should reject the removed model-assignment option")]
fn removed_model_assignment_rejected(world: &mut WatnWorld) {
    assert_ne!(world.exit_status, Some(0));
    assert!(world
        .stderr_output
        .as_deref()
        .unwrap_or_default()
        .contains("--set-small"));
}

#[then("generated shell completions should not advertise removed setup commands or options")]
fn completions_omit_removed_surface(_world: &mut WatnWorld) {
    let binary = find_binary();
    let output = std::process::Command::new(binary)
        .args(["completions", "bash"])
        .output()
        .expect("generate completions");
    let text = String::from_utf8_lossy(&output.stdout);
    for removed in [
        "--provider",
        "--model",
        "--set-small",
        " models ",
        " provider ",
    ] {
        assert!(!text.contains(removed), "completion advertised {removed}");
    }
}

#[then("`watn -1`, `watn -2`, and `watn -3` should remain valid request tier selectors")]
fn tier_selectors_remain_valid(world: &mut WatnWorld) {
    for tier in ["-1", "-2", "-3"] {
        run_binary_with_state(world, &[tier, "show changed files"], None);
        assert_ne!(
            world.exit_status,
            Some(2),
            "tier {tier} was rejected by clap"
        );
    }
}

#[given("a complete persisted configuration exists")]
fn complete_persisted_configuration_exists(world: &mut WatnWorld) {
    complete_configuration_exists(world);
}

#[when("I run a request with the complete persisted configuration")]
fn run_request_with_persisted_configuration(world: &mut WatnWorld) {
    run_binary_with_state(world, &["show changed files"], None);
}

#[then("the persisted provider and model roles should remain the request selection")]
fn persisted_selection_remains(world: &mut WatnWorld) {
    let directory = world.temp_dir.as_ref().expect("config directory");
    let content = std::fs::read_to_string(directory.path().join("watn/config.toml"))
        .expect("persisted config");
    assert!(content.contains("provider = \"test\""));
    assert!(content.contains("small = \"small-model\""));
    assert!(content.contains("normal = \"normal-model\""));
    assert!(content.contains("thinking = \"thinking-model\""));
}
