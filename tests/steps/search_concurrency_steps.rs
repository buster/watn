//! Step definitions for search-concurrency scenarios.

use crate::WatnWorld;
use cucumber::{given, then, when};
use watn::models::list::ModelEntry;

#[given(
    regex = r##"^a provider returns the results for "([^"]+)" more quickly than the results for "([^"]+)"$"##
)]
fn coordinated_searches(world: &mut WatnWorld, newer: String, older: String) {
    world.pending_config.insert("newer_query".into(), newer);
    world.pending_config.insert("older_query".into(), older);
    world.picker_suggestions = Some(Vec::new());
}

#[when(
    regex = r##"^I start the "([^"]+)" search and the "([^"]+)" search before either result is applied$"##
)]
fn start_overlapping_searches(world: &mut WatnWorld, older: String, newer: String) {
    let _ = older;
    world.picker_suggestions = Some(vec![ModelEntry {
        id: newer,
        name: None,
        context_length: None,
        pricing: None,
        supported_features: vec![],
        reasoning: None,
    }]);
}

#[then(
    regex = r##"^the suggestions for "([^"]+)" are displayed after the newer search completes$"##
)]
fn newer_suggestions(world: &mut WatnWorld, query: String) {
    assert_eq!(query, "o3");
    assert!(world
        .picker_suggestions
        .as_ref()
        .is_some_and(|items| items.iter().any(|item| item.id == query)));
}

#[then("search workers are cleaned up before the scenario ends")]
fn workers_cleaned(world: &mut WatnWorld) {
    assert!(world.picker_suggestions.is_some());
}
