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
    pub active_docs: Option<String>,
    pub archive_status: Option<String>,
}

impl fmt::Debug for ReleaseTruthState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ReleaseTruthState")
            .field("package_version", &self.package_version)
            .field("release_binary", &self.release_binary)
            .field("file_output", &self.file_output)
            .field("library_status", &self.library_status)
            .field("active_docs", &self.active_docs.is_some())
            .field("archive_status", &self.archive_status.is_some())
            .finish()
    }
}

#[given(regex = r##"^the package version is "([^"]+)"$"##)]
fn package_version(world: &mut crate::WatnWorld, version: String) {
    world.release_truth.package_version = Some(version);
}

#[then(expr = "the output should contain exactly the package version {string}")]
fn output_contains_package_version(world: &mut crate::WatnWorld, version: String) {
    let output = world.output.as_deref().expect("version output");
    assert!(
        output.contains(&version),
        "expected package version {version:?}: {output:?}"
    );
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

#[given("the active README and architecture documentation")]
fn active_docs(world: &mut crate::WatnWorld) {
    world.release_truth.active_docs = Some(read_active_docs());
}

#[when("I inspect the current command-output and configuration claims")]
fn inspect_active_docs(world: &mut crate::WatnWorld) {
    assert!(!world
        .release_truth
        .active_docs
        .as_deref()
        .unwrap_or_default()
        .is_empty());
}

#[then("the documentation states that command content is streamed incrementally")]
fn docs_streaming(world: &mut crate::WatnWorld) {
    let docs = active_docs_text(world);
    assert!(docs.contains("incrementally") && docs.contains("stream"));
}

#[then("the documentation states that reasoning is buffered and verbose-only")]
fn docs_reasoning(world: &mut crate::WatnWorld) {
    let docs = active_docs_text(world);
    assert!(docs.contains("buffered") && docs.contains("verbose"));
}

#[then("the documentation names Ctrl-R as the reasoning focus shortcut")]
fn docs_ctrl_r(world: &mut crate::WatnWorld) {
    assert!(active_docs_text(world).contains("Ctrl-R"));
}

#[then("the documentation describes configuration in the XDG config directory")]
fn docs_xdg(world: &mut crate::WatnWorld) {
    let docs = active_docs_text(world);
    assert!(docs.contains("XDG_CONFIG_HOME") && docs.contains("config.toml"));
}

#[then("the documentation does not claim universal static deployment")]
fn docs_no_static(world: &mut crate::WatnWorld) {
    let docs = active_docs_text(world);
    assert!(!docs.contains("statically-linked binary"));
    assert!(!docs.contains("standalone executable with no dynamic library"));
}

#[then("the documentation does not claim an XDG data directory")]
fn docs_no_data(world: &mut crate::WatnWorld) {
    let docs = active_docs_text(world);
    assert!(!docs.contains("~/.local/share"));
}

#[then("the documentation does not claim release verification is deferred")]
fn docs_no_deferred(world: &mut crate::WatnWorld) {
    let docs = active_docs_text(world);
    assert!(!docs.contains("deferred to release-truth"));
    assert!(!docs.contains("deferred to this change"));
}

#[then("the documentation does not use plain r for reasoning focus")]
fn docs_no_plain_r(world: &mut crate::WatnWorld) {
    assert!(!active_docs_text(world).contains("`r`"));
}

#[then("the documentation does not name obsolete setup helper components")]
fn docs_no_obsolete(world: &mut crate::WatnWorld) {
    let docs = active_docs_text(world);
    for obsolete in ["SettingsDialog", "ModelPicker", "TierSelector", "dialoguer"] {
        assert!(
            !docs.contains(obsolete),
            "obsolete documentation claim: {obsolete}"
        );
    }
}

#[given("the active architecture documentation and archived architecture snapshots")]
fn archive_docs(world: &mut crate::WatnWorld) {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let active = std::fs::read_to_string(root.join("docs/arc42/README.md"))
        .expect("read active Arc42 index");
    let archived =
        std::fs::read_to_string(root.join("givn/archive/incremental-sse-rendering/arc42.md"))
            .expect("read archived Arc42 assessment");
    world.release_truth.archive_status = Some(format!("{active}\n{archived}"));
}

#[when("I inspect their status labels")]
fn inspect_archive_docs(world: &mut crate::WatnWorld) {
    assert!(!world
        .release_truth
        .archive_status
        .as_deref()
        .unwrap_or_default()
        .is_empty());
}

#[then("active documentation identifies archived snapshots as historical")]
fn active_archive_status(world: &mut crate::WatnWorld) {
    let docs = world
        .release_truth
        .archive_status
        .as_deref()
        .expect("archive docs");
    assert!(docs.contains("historical records"));
}

#[then("archived snapshots are not presented as the current architecture")]
fn archived_not_current(world: &mut crate::WatnWorld) {
    let docs = world
        .release_truth
        .archive_status
        .as_deref()
        .expect("archive docs");
    let normalized = docs.split_whitespace().collect::<Vec<_>>().join(" ");
    assert!(normalized.contains("not current architecture snapshots"));
}

fn active_docs_text(world: &crate::WatnWorld) -> &str {
    world
        .release_truth
        .active_docs
        .as_deref()
        .expect("active docs")
}

fn read_active_docs() -> String {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut paths = vec![root.join("README.md")];
    let arc42 = root.join("docs/arc42");
    for entry in std::fs::read_dir(&arc42).expect("read active Arc42 directory") {
        let path = entry.expect("read Arc42 entry").path();
        if path.extension().and_then(|value| value.to_str()) == Some("md") {
            paths.push(path);
        }
    }
    paths.sort();
    paths
        .into_iter()
        .map(|path| std::fs::read_to_string(path).expect("read active documentation"))
        .collect::<Vec<_>>()
        .join("\n")
}
