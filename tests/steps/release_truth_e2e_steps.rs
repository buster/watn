use cucumber::{given, then, when};
use std::path::PathBuf;
use std::process::Command;

#[given(
    "a release range with three revisions of one change that share the release note \"Explain an existing shell command without executing it\""
)]
fn three_shared_revisions(world: &mut crate::WatnWorld) {
    let fixture = crate::steps::release_truth_steps::changelog_fixture();
    fixture.commit("feat(use-shell): Explain an existing shell command without executing it");
    fixture.commit("feat(use-shell): Explain an existing shell command without executing it");
    fixture.commit("feat(use-shell): Explain an existing shell command without executing it");
    world.release_truth.changelog_fixture = Some(fixture);
}

#[given("a documentation revision in the same range")]
fn documentation_revision(world: &mut crate::WatnWorld) {
    let fixture = world
        .release_truth
        .changelog_fixture
        .as_ref()
        .expect("a changelog fixture is required first");
    fixture.commit("docs(use-shell): Plan the explanation card");
}

#[then(
    "the changelog lists \"Explain an existing shell command without executing it\" exactly once"
)]
fn changelog_lists_note_once(world: &mut crate::WatnWorld) {
    let output = changelog_output(world);
    let occurrences = output
        .matches("Explain an existing shell command without executing it")
        .count();
    assert_eq!(
        occurrences, 1,
        "expected the release note exactly once: {output}"
    );
}

#[then("the changelog lists no documentation entry")]
fn changelog_lists_no_documentation(world: &mut crate::WatnWorld) {
    let output = changelog_output(world);
    assert!(
        !output.contains("Plan the explanation card"),
        "changelog contains the documentation revision: {output}"
    );
}

fn changelog_output(world: &crate::WatnWorld) -> &str {
    world
        .release_truth
        .changelog_output
        .as_deref()
        .expect("changelog output")
}

#[when("I run the release binary with `--version`")]
fn run_release_version(world: &mut crate::WatnWorld) {
    let build = Command::new("cargo")
        .args(["build", "--release"])
        .status()
        .expect("build release binary");
    assert!(build.success(), "release build failed with {build}");
    let binary = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/release/watn");
    let output = Command::new(binary)
        .arg("--version")
        .output()
        .expect("run release version command");
    world.exit_status = output.status.code();
    world.output = Some(String::from_utf8_lossy(&output.stdout).to_string());
    world.stderr_output = Some(String::from_utf8_lossy(&output.stderr).to_string());
}
