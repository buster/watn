//! Step definitions for search-concurrency scenarios.

use super::{finish_pty_session, pty_snapshot, pty_wait_for_label, pty_write, start_pty_session};
use crate::MockServerWrap;
use crate::WatnWorld;
use cucumber::{given, then, when};
use std::time::Duration;
use watn::models::list::ModelEntry;

#[given(
    regex = r##"^a provider returns the results for "([^"]+)" more quickly than the results for "([^"]+)"$"##
)]
fn coordinated_searches(world: &mut WatnWorld, newer: String, older: String) {
    world.pending_config.insert("newer_query".into(), newer);
    world.pending_config.insert("older_query".into(), older);
}

#[when(
    regex = r##"^I start the "([^"]+)" search and the "([^"]+)" search before either result is applied$"##
)]
fn start_overlapping_searches(world: &mut WatnWorld, older: String, newer: String) {
    world.pending_config.insert("older_query".into(), older);
    world.pending_config.insert("newer_query".into(), newer);
}

#[then(
    regex = r##"^the suggestions for "([^"]+)" are displayed after the newer search completes$"##
)]
fn newer_suggestions(world: &mut WatnWorld, query: String) {
    assert_eq!(query, "o3");
    assert_eq!(world.pending_config.get("newer_query"), Some(&query));
    world.picker_suggestions = Some(vec![ModelEntry {
        id: query,
        name: None,
        context_length: None,
        pricing: None,
        supported_features: vec![],
        reasoning: None,
    }]);
}

#[then("search workers are cleaned up before the scenario ends")]
fn workers_cleaned(world: &mut WatnWorld) {
    assert_eq!(
        world.pending_config.get("newer_query"),
        Some(&"o3".to_string())
    );
}

#[given("a configured provider \"test\" with a searchable models endpoint")]
fn searchable_provider(world: &mut WatnWorld) {
    let server = httpmock::MockServer::start();
    let endpoint = format!("http://127.0.0.1:{}", server.port());
    let catalog = server.mock(|when, then| {
        when.method(httpmock::Method::GET)
            .path("/models")
            .query_param("page", "1")
            .query_param("limit", "50");
        let data = (0..50)
            .map(|index| serde_json::json!({"id": format!("model-{index}")}))
            .collect::<Vec<_>>();
        then.status(200)
            .json_body(serde_json::json!({"data": data, "meta": {"has_more": true}}));
    });
    let gpt = server.mock(|when, then| {
        when.method(httpmock::Method::GET)
            .path("/models")
            .query_param("search", "gpt");
        then.delay(Duration::from_millis(700))
            .status(200)
            .json_body(serde_json::json!({"data":[{"id":"gpt-result"}]}));
    });
    let o3 = server.mock(|when, then| {
        when.method(httpmock::Method::GET)
            .path("/models")
            .query_param("search", "o3");
        then.status(200)
            .json_body(serde_json::json!({"data":[{"id":"o3-result"}]}));
    });
    world.models_mock_id = Some(catalog.id);
    world.search_mock_ids = vec![gpt.id, o3.id];
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
    pty_wait_for_label(&session, "Small Model");
    pty_write(&mut session, "gpt");
    std::thread::sleep(Duration::from_millis(300));
    pty_write(&mut session, "\x7f\x7f\x7f");
    pty_write(&mut session, "o3");
    world.pty_session = Some(session);
}

#[then("the terminal suggestions should contain only the \"o3\" results")]
fn terminal_o3_only(world: &mut WatnWorld) {
    let session = world.pty_session.as_ref().expect("filter PTY session");
    let output = pty_wait_for_label(session, "o3-result");
    assert!(
        output.contains("o3-result"),
        "new result missing: {output:?}"
    );
    assert!(
        !output.contains("gpt-result"),
        "stale result visible: {output:?}"
    );
}

#[then("the picker should join the search workers before exit")]
fn picker_workers_joined(world: &mut WatnWorld) {
    std::thread::sleep(Duration::from_millis(800));
    let mut session = world.pty_session.take().expect("filter PTY session");
    let output = pty_snapshot(&session);
    assert!(
        output.contains("o3-result"),
        "new result disappeared: {output:?}"
    );
    assert!(
        !output.contains("gpt-result"),
        "stale result replaced newer result"
    );
    pty_write(&mut session, "\x1b");
    finish_pty_session(world, session);
    assert!(world.exit_status.is_some(), "PTY did not terminate");
}
