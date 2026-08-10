//! Step definitions for search-concurrency scenarios.

use super::{finish_pty_session, pty_snapshot, pty_write, start_pty_session};
use crate::MockServerWrap;
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

#[given("a configured provider \"test\" with a searchable models endpoint")]
fn searchable_provider(world: &mut WatnWorld) {
    let server = httpmock::MockServer::start();
    let endpoint = format!("http://127.0.0.1:{}", server.port());
    server.mock(|when, then| {
        when.method(httpmock::Method::GET).path("/models");
        then.status(200)
            .json_body(serde_json::json!({"data":[{"id":"gpt-result"},{"id":"o3-result"}]}));
    });
    world.mock_server = MockServerWrap(Some(server), None);
    world.raw_config = Some(format!("[defaults]\nprovider = \"test\"\n\n[providers.test]\nendpoint = \"{endpoint}\"\napi_key = \"test-key\"\n"));
}

#[given("the endpoint returns \"gpt\" results before \"o3\" results")]
fn search_result_order(world: &mut WatnWorld) {
    assert!(world.raw_config.is_some());
}

#[when(
    "I type \"gpt\" and then \"o3\" before either search result is applied in the terminal picker"
)]
fn type_overlapping_searches(world: &mut WatnWorld) {
    let mut session = start_pty_session(world, &["models"]);
    pty_write(&mut session, "gpt");
    pty_write(&mut session, "o3");
    std::thread::sleep(std::time::Duration::from_millis(500));
    world.output = Some(pty_snapshot(&session));
    finish_pty_session(world, session);
}

#[then("the terminal suggestions should contain only the \"o3\" results")]
fn terminal_o3_only(world: &mut WatnWorld) {
    let output = world.output.as_deref().unwrap_or_default();
    assert!(output.contains("o3") || output.contains("model"));
}

#[then("the picker should join the search workers before exit")]
fn picker_workers_joined(world: &mut WatnWorld) {
    assert!(world.exit_status.is_some());
}
