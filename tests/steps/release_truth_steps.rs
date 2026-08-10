use cucumber::{given, then, when};
use std::fmt;
use std::path::PathBuf;
use std::process::Command;

#[derive(Default)]
pub struct ReleaseTruthState {
    pub package_version: Option<String>,
    pub release_binary: Option<PathBuf>,
    pub file_output: Option<String>,
    pub library_output: Option<String>,
    pub library_status: Option<bool>,
}

impl fmt::Debug for ReleaseTruthState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ReleaseTruthState")
            .field("package_version", &self.package_version)
            .field("release_binary", &self.release_binary)
            .field("file_output", &self.file_output)
            .field("library_status", &self.library_status)
            .finish()
    }
}

#[given(regex = r##"^the package version is "([^"]+)"$"##)]
fn package_version(world: &mut crate::WatnWorld, version: String) {
    world.release_truth.package_version = Some(version);
}

#[given("a release binary has been built for the current host")]
fn release_binary(world: &mut crate::WatnWorld) {
    let status = Command::new("cargo")
        .args(["build", "--release"])
        .status()
        .expect("build release artifact");
    assert!(status.success(), "release build failed with {status}");
    let binary = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/release/watn");
    assert!(
        binary.is_file(),
        "release binary is missing: {}",
        binary.display()
    );
    world.release_truth.release_binary = Some(binary);
}

#[when("I inspect the release artifact's file type and runtime libraries")]
fn inspect_release(world: &mut crate::WatnWorld) {
    let binary = world
        .release_truth
        .release_binary
        .as_ref()
        .expect("release binary");
    let file = Command::new("file")
        .arg(binary)
        .output()
        .expect("inspect release file type");
    assert!(file.status.success(), "file inspection failed");
    world.release_truth.file_output = Some(String::from_utf8_lossy(&file.stdout).to_string());

    let (tool, args): (&str, Vec<String>) = if cfg!(target_os = "linux") {
        ("ldd", vec![binary.display().to_string()])
    } else if cfg!(target_os = "macos") {
        (
            "otool",
            vec!["-L".to_string(), binary.display().to_string()],
        )
    } else {
        panic!("release library inspection is unsupported on this host");
    };
    let libraries = Command::new(tool)
        .args(args)
        .output()
        .expect("inspect release runtime libraries");
    world.release_truth.library_status = Some(libraries.status.success());
    world.release_truth.library_output =
        Some(String::from_utf8_lossy(&libraries.stdout).to_string());
}

#[then("it is identified as a dynamically linked executable for the current host")]
fn dynamic_release(world: &mut crate::WatnWorld) {
    let file = world
        .release_truth
        .file_output
        .as_deref()
        .expect("file output");
    let libraries = world
        .release_truth
        .library_output
        .as_deref()
        .expect("library output");
    if cfg!(target_os = "linux") {
        assert!(
            file.contains("dynamically linked"),
            "unexpected file output: {file}"
        );
        assert!(!libraries.contains("not a dynamic executable"));
    } else {
        assert!(file.contains("Mach-O"), "unexpected file output: {file}");
        assert!(
            libraries.contains(".dylib"),
            "unexpected library output: {libraries}"
        );
    }
}

#[then(regex = r##"^the runtime library inspection succeeds.*$"##)]
fn libraries_reported(world: &mut crate::WatnWorld) {
    assert_eq!(world.release_truth.library_status, Some(true));
    let libraries = world
        .release_truth
        .library_output
        .as_deref()
        .expect("library output");
    assert!(
        libraries.lines().any(|line| !line.trim().is_empty()),
        "runtime library output is empty"
    );
}

#[then("the deployment documentation states that requirements depend on the target")]
fn deployment_docs(_world: &mut crate::WatnWorld) {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("docs/arc42/07-deployment-view.md");
    let docs = std::fs::read_to_string(path).expect("read deployment documentation");
    assert!(docs.contains("target-dependent"));
}
