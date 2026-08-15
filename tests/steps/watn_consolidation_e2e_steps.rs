//! E2E CLI steps for the givn-driven Watn specification consolidation.

use cucumber::{given, then, when};

use crate::WatnWorld;
use super::watn_consolidation_support::{duplicate_titles, feature_text, fixture_specs, fixture_stdout, invoke_givn, setup_fixture};

#[given("an isolated watn consolidation fixture with dispositions for every overlap finding")]
fn isolated_fixture(world: &mut WatnWorld) {
    setup_fixture(world, false);
}

#[when("the maintainer invokes the fixture review command")]
fn run_fixture_review(world: &mut WatnWorld) {
    invoke_givn(world, &["check", "review", "--change", "fixture-consolidation"]);
}

#[when("the maintainer invokes the fixture archive command")]
fn run_fixture_archive(world: &mut WatnWorld) {
    invoke_givn(world, &["archive", "--change", "fixture-consolidation"]);
}

#[then("the fixture command exits 0")]
fn command_exits_zero(world: &mut WatnWorld) {
    assert_eq!(world.exit_status, Some(0), "stderr: {:?}", world.stderr_output);
}

#[then(regex = r##"^fixture stdout contains "([^"]+)"$"##)]
fn stdout_contains(world: &mut WatnWorld, text: String) {
    let output = fixture_stdout(world);
    assert!(output.contains(&text), "stdout missing {text:?}: {output:?}");
}

#[then("the fixture permanent specification tree contains no duplicate scenario titles")]
fn no_duplicate_titles(world: &mut WatnWorld) {
    let duplicates = duplicate_titles(fixture_specs(world));
    assert!(duplicates.is_empty(), "duplicate titles: {duplicates:?}");
}

#[then(regex = r##"^the fixture permanent specification tree contains "([^"]+)"$"##)]
fn permanent_tree_contains(world: &mut WatnWorld, text: String) {
    let content = feature_text(fixture_specs(world));
    assert!(content.contains(&text), "tree missing {text:?}: {content:?}");
}

#[then(regex = r##"^the fixture permanent specification tree does not contain "([^"]+)"$"##)]
fn permanent_tree_does_not_contain(world: &mut WatnWorld, text: String) {
    let content = feature_text(fixture_specs(world));
    assert!(!content.contains(&text), "tree contains {text:?}: {content:?}");
}
