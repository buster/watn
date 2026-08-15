//! Shared fixture and subprocess helpers for consolidation steps.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use tempfile::TempDir;

use crate::WatnWorld;

pub(crate) fn setup_fixture(world: &mut WatnWorld, failing_archive: bool) {
    let temp = TempDir::new().expect("create consolidation fixture");
    let root = temp.path();
    let givn_bin = givn_binary();
    let init = Command::new(&givn_bin)
        .args(["init", "--no-addons"])
        .current_dir(root)
        .output()
        .expect("start givn init");
    assert!(init.status.success(), "givn init failed: {:?}", init);

    let specs_dir = root.join("givn/specs/fixture");
    fs::create_dir_all(&specs_dir).expect("create fixture permanent specs");
    fs::write(
        specs_dir.join("fixture.feature"),
        "Feature: fixture\n\n  Scenario: Obsolete behavior\n    Given a fixture behavior\n    When the old behavior runs\n    Then the old result appears\n",
    )
    .expect("write fixture permanent spec");

    let change = root.join("givn/changes/fixture-consolidation");
    let delta_dir = change.join("specs/fixture");
    fs::create_dir_all(&delta_dir).expect("create fixture delta");
    fs::write(
        delta_dir.join("fixture.feature"),
        "@givn.delta @fixture\nFeature: fixture\n\n  @givn.removed\n  Scenario: Obsolete behavior\n    Given a placeholder\n\n  @givn.added\n  Scenario: Canonical retained behavior\n    Given a fixture behavior\n    When the old behavior runs\n    Then the old result appears\n",
    )
    .expect("write fixture delta");
    fs::write(change.join("proposal.md"), "# fixture\n").expect("write proposal");
    fs::write(change.join("design.md"), "# fixture\n").expect("write design");
    fs::write(change.join("design-review.md"), "DESIGN-REVIEW: PASS\n")
        .expect("write design review");
    fs::write(
        change.join("tasks.md"),
        "- [x] COMMIT: `aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa`\n",
    )
    .expect("write tasks");
    fs::write(
        change.join("review.md"),
        "## Overlap dispositions\n\n| Scenario A | Scenario B | Disposition |\n|---|---|---|\n| `Obsolete behavior` | `Canonical retained behavior` | merge |\n\nREVIEW: PASS\n",
    )
    .expect("write review");
    let verify = if failing_archive { "false" } else { "true" };
    fs::write(
        root.join("givn/commands.yaml"),
        format!("verify:\n  command: \"{verify}\"\n  e2e_command: \"true\"\n"),
    )
    .expect("write fixture commands");

    world.consolidation_before_tree = Some(snapshot_tree(&root.join("givn/specs")));
    world.temp_dir = Some(temp);
}

pub(crate) fn invoke_givn(world: &mut WatnWorld, args: &[&str]) {
    let output = Command::new(givn_binary())
        .args(args)
        .current_dir(fixture_root(world))
        .output()
        .expect("start givn subprocess");
    world.output = Some(String::from_utf8_lossy(&output.stdout).into_owned());
    world.stderr_output = Some(String::from_utf8_lossy(&output.stderr).into_owned());
    world.exit_status = output.status.code();
}

pub(crate) fn fixture_stdout(world: &WatnWorld) -> &str {
    world.output.as_deref().unwrap_or_default()
}

fn givn_binary() -> PathBuf {
    if let Some(path) = std::env::var_os("GIVN_BIN").map(PathBuf::from) {
        assert!(
            path.is_absolute(),
            "GIVN_BIN must be an absolute executable path"
        );
        assert!(
            path.is_file(),
            "GIVN_BIN does not point to a file: {}",
            path.display()
        );
        path
    } else {
        PathBuf::from("givn")
    }
}

pub(crate) fn fixture_root(world: &WatnWorld) -> &Path {
    world
        .temp_dir
        .as_ref()
        .expect("consolidation fixture")
        .path()
}

pub(crate) fn fixture_specs(world: &WatnWorld) -> PathBuf {
    fixture_root(world).join("givn/specs")
}

pub(crate) fn snapshot_tree(root: &Path) -> Vec<(String, Vec<u8>)> {
    let mut entries = Vec::new();
    collect_files(root, &mut entries);
    entries.sort_by(|left, right| left.0.cmp(&right.0));
    entries
}

fn collect_files(root: &Path, output: &mut Vec<(String, Vec<u8>)>) {
    let Ok(entries) = fs::read_dir(root) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_files(&path, output);
        } else if let Ok(bytes) = fs::read(&path) {
            output.push((
                path.strip_prefix(root)
                    .unwrap_or(&path)
                    .to_string_lossy()
                    .into_owned(),
                bytes,
            ));
        }
    }
}

pub(crate) fn feature_text(root: PathBuf) -> String {
    snapshot_tree(&root)
        .into_iter()
        .filter(|(path, _)| path.ends_with(".feature"))
        .filter_map(|(_, bytes)| String::from_utf8(bytes).ok())
        .collect::<Vec<_>>()
        .join("\n")
}

pub(crate) fn scenario_titles(root: PathBuf) -> Vec<String> {
    feature_text(root)
        .lines()
        .filter_map(|line| line.trim().strip_prefix("Scenario: ").map(str::to_owned))
        .collect()
}

pub(crate) fn duplicate_titles(root: PathBuf) -> Vec<String> {
    let mut counts = std::collections::HashMap::new();
    for title in scenario_titles(root) {
        *counts.entry(title).or_insert(0usize) += 1;
    }
    counts
        .into_iter()
        .filter_map(|(title, count)| (count > 1).then_some(title))
        .collect()
}
