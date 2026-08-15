//! Regular rollback steps for the givn-driven Watn specification consolidation.

use cucumber::{given, then, when};

use crate::WatnWorld;
use super::watn_consolidation_support::{invoke_givn, setup_fixture, fixture_root, snapshot_tree};

#[given("an isolated watn consolidation fixture with a failing archive hook")]
fn failing_fixture(world: &mut WatnWorld) {
    setup_fixture(world, true);
}

#[when("the maintainer runs `givn archive --change fixture-consolidation`")]
fn run_fixture_archive(world: &mut WatnWorld) {
    invoke_givn(world, &["archive", "--change", "fixture-consolidation"]);
}

#[then("the fixture command fails")]
fn command_fails(world: &mut WatnWorld) {
    assert_ne!(world.exit_status, Some(0), "stdout: {:?}", world.output);
}

#[then("the fixture permanent specification tree remains unchanged")]
fn permanent_tree_unchanged(world: &mut WatnWorld) {
    let before = world
        .consolidation_before_tree
        .as_ref()
        .expect("fixture snapshot");
    let after = snapshot_tree(&fixture_root(world).join("givn/specs"));
    assert_eq!(after, *before, "archive changed the fixture after failure");
}
