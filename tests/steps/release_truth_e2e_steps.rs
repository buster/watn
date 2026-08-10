use cucumber::when;
use std::path::PathBuf;
use std::process::Command;

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
