use std::time::{Duration, Instant};

use cucumber::{given, then, when};
use httpmock::Method;

use super::{pty_snapshot, pty_wait_for_label, pty_write, start_pty_session};
use crate::MockServerWrap;
use crate::WatnWorld;

const SCREEN_WIDTH: usize = 120;
const SCREEN_HEIGHT: usize = 40;

struct Screen {
    cells: [[char; SCREEN_WIDTH]; SCREEN_HEIGHT],
    cursor_x: usize,
    cursor_y: usize,
}

impl Screen {
    fn new() -> Self {
        Self {
            cells: [[' '; SCREEN_WIDTH]; SCREEN_HEIGHT],
            cursor_x: 0,
            cursor_y: 0,
        }
    }

    fn put(&mut self, character: char) {
        if self.cursor_y < SCREEN_HEIGHT && self.cursor_x < SCREEN_WIDTH {
            self.cells[self.cursor_y][self.cursor_x] = character;
        }
        self.cursor_x = (self.cursor_x + 1).min(SCREEN_WIDTH.saturating_sub(1));
    }

    fn text(&self) -> String {
        self.cells
            .iter()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

fn parse_screen(output: &str) -> Screen {
    let mut screen = Screen::new();
    let mut characters = output.chars().peekable();
    while let Some(character) = characters.next() {
        if character == '\x1b' {
            match characters.next() {
                Some('[') => {
                    let mut params = String::new();
                    for next in characters.by_ref() {
                        if ('@'..='~').contains(&next) {
                            let values = params
                                .split(';')
                                .map(|value| {
                                    value.trim_start_matches('?').parse::<usize>().unwrap_or(0)
                                })
                                .collect::<Vec<_>>();
                            let first = values
                                .first()
                                .copied()
                                .filter(|value| *value != 0)
                                .unwrap_or(1);
                            match next {
                                'A' => screen.cursor_y = screen.cursor_y.saturating_sub(first),
                                'B' => {
                                    screen.cursor_y =
                                        (screen.cursor_y + first).min(SCREEN_HEIGHT - 1)
                                }
                                'C' => {
                                    screen.cursor_x =
                                        (screen.cursor_x + first).min(SCREEN_WIDTH - 1)
                                }
                                'D' => screen.cursor_x = screen.cursor_x.saturating_sub(first),
                                'G' => screen.cursor_x = (first - 1).min(SCREEN_WIDTH - 1),
                                'd' => screen.cursor_y = (first - 1).min(SCREEN_HEIGHT - 1),
                                'H' | 'f' => {
                                    screen.cursor_y = values
                                        .first()
                                        .copied()
                                        .unwrap_or(1)
                                        .saturating_sub(1)
                                        .min(SCREEN_HEIGHT - 1);
                                    screen.cursor_x = values
                                        .get(1)
                                        .copied()
                                        .unwrap_or(1)
                                        .saturating_sub(1)
                                        .min(SCREEN_WIDTH - 1);
                                }
                                'J' if values.first().copied().unwrap_or(0) == 2 => {
                                    screen = Screen::new();
                                }
                                'K' => {
                                    for x in screen.cursor_x..SCREEN_WIDTH {
                                        screen.cells[screen.cursor_y][x] = ' ';
                                    }
                                }
                                _ => {}
                            }
                            break;
                        }
                        params.push(next);
                    }
                }
                Some(']') => {
                    for next in characters.by_ref() {
                        if next == '\x07' {
                            break;
                        }
                    }
                }
                _ => {}
            }
        } else {
            match character {
                '\r' => screen.cursor_x = 0,
                '\n' => screen.cursor_y = (screen.cursor_y + 1).min(SCREEN_HEIGHT - 1),
                character if !character.is_control() => screen.put(character),
                _ => {}
            }
        }
    }
    screen
}

#[given(
    regex = r##"^a provider with a complete model catalog containing "([^"]+)", "([^"]+)", and "([^"]+)"$"##
)]
fn complete_catalog(world: &mut WatnWorld, _first: String, _second: String, _third: String) {
    world.mock_server = MockServerWrap(Some(httpmock::MockServer::start()), None);
    let server = world.mock_server.0.as_ref().expect("catalog server");
    let base_url = format!("http://127.0.0.1:{}", server.port());
    let models = [_first, _second, _third];
    let data = models
        .iter()
        .map(|id| serde_json::json!({ "id": id }))
        .collect::<Vec<_>>();
    let catalog = server.mock(|when, then| {
        when.method(Method::GET)
            .path("/models")
            .query_param("page", "1")
            .query_param("limit", "50");
        then.status(200)
            .header("Content-Type", "application/json")
            .json_body(serde_json::json!({ "data": data }));
    });
    let search = server.mock(|when, then| {
        when.method(Method::GET)
            .path("/models")
            .query_param("search", "gpt");
        then.status(200)
            .header("Content-Type", "application/json")
            .json_body(serde_json::json!({ "data": [{ "id": "gpt-4o-mini" }] }));
    });
    world.models_mock_id = Some(catalog.id);
    world.search_mock_ids = vec![search.id];
    world.raw_config = Some(format!(
        "[defaults]\nprovider = \"test\"\n\n[providers.test]\nendpoint = \"{base_url}\"\napi_key = \"test-key\"\n"
    ));
}

#[given("the catalog can be loaded in one response")]
fn complete_catalog_response(_world: &mut WatnWorld) {
    assert!(_world.models_mock_id.is_some());
}

#[given("a provider with a catalog larger than one response")]
fn incomplete_catalog(world: &mut WatnWorld) {
    world.mock_server = MockServerWrap(Some(httpmock::MockServer::start()), None);
    let server = world.mock_server.0.as_ref().expect("catalog server");
    let base_url = format!("http://127.0.0.1:{}", server.port());
    let data = (0..50)
        .map(|index| serde_json::json!({ "id": format!("model-{index}") }))
        .collect::<Vec<_>>();
    let catalog = server.mock(|when, then| {
        when.method(Method::GET)
            .path("/models")
            .query_param("page", "1")
            .query_param("limit", "50");
        then.status(200)
            .header("Content-Type", "application/json")
            .json_body(serde_json::json!({ "data": data }));
    });
    let search = server.mock(|when, then| {
        when.method(Method::GET)
            .path("/models")
            .query_param("search", "o3");
        then.status(200)
            .header("Content-Type", "application/json")
            .json_body(serde_json::json!({ "data": [{ "id": "o3-pro" }] }));
    });
    world.models_mock_id = Some(catalog.id);
    world.search_mock_ids = vec![search.id];
    world.raw_config = Some(format!(
        "[defaults]\nprovider = \"test\"\n\n[providers.test]\nendpoint = \"{base_url}\"\napi_key = \"test-key\"\n"
    ));
}

#[given(regex = r##"^the provider search returns "([^"]+)" for the query "([^"]+)"$"##)]
fn provider_search_result(world: &mut WatnWorld, model: String, query: String) {
    assert_eq!(model, "o3-pro");
    assert_eq!(query, "o3");
    assert!(!world.search_mock_ids.is_empty());
}

#[given(
    regex = r##"^a provider returns the result for "([^"]+)" after the result for "([^"]+)"$"##
)]
fn ordered_search_results(world: &mut WatnWorld, older: String, newer: String) {
    assert_eq!(older, "gpt");
    assert_eq!(newer, "o3");
    world.mock_server = MockServerWrap(Some(httpmock::MockServer::start()), None);
    let server = world.mock_server.0.as_ref().expect("catalog server");
    let base_url = format!("http://127.0.0.1:{}", server.port());
    let data = (0..50)
        .map(|index| serde_json::json!({ "id": format!("model-{index}") }))
        .collect::<Vec<_>>();
    let catalog = server.mock(|when, then| {
        when.method(Method::GET)
            .path("/models")
            .query_param("page", "1")
            .query_param("limit", "50");
        then.status(200)
            .header("Content-Type", "application/json")
            .json_body(serde_json::json!({ "data": data }));
    });
    let older_mock = server.mock(|when, then| {
        when.method(Method::GET)
            .path("/models")
            .query_param("search", "gpt");
        then.delay(Duration::from_millis(700))
            .status(200)
            .header("Content-Type", "application/json")
            .json_body(serde_json::json!({ "data": [{ "id": "gpt-result" }] }));
    });
    let newer_mock = server.mock(|when, then| {
        when.method(Method::GET)
            .path("/models")
            .query_param("search", "o3");
        then.status(200)
            .header("Content-Type", "application/json")
            .json_body(serde_json::json!({ "data": [{ "id": "o3-result" }] }));
    });
    world.models_mock_id = Some(catalog.id);
    world.search_mock_ids = vec![older_mock.id, newer_mock.id];
    world.raw_config = Some(format!(
        "[defaults]\nprovider = \"test\"\n\n[providers.test]\nendpoint = \"{base_url}\"\napi_key = \"test-key\"\n"
    ));
}

#[when(
    regex = r##"^I type "([^"]+)" and then replace it with "([^"]+)" before either result is applied$"##
)]
fn type_replaced_filter(world: &mut WatnWorld, older: String, newer: String) {
    assert_eq!(older, "gpt");
    assert_eq!(newer, "o3");
    let mut session = start_pty_session(world, &["models"]);
    pty_wait_for_label(&session, "Small Model");
    pty_write(&mut session, &older);
    std::thread::sleep(Duration::from_millis(300));
    pty_write(&mut session, "\x7f\x7f\x7f");
    pty_write(&mut session, &newer);
    world.pty_session = Some(session);
}

#[then(regex = r##"^the suggestions should show only the results for "([^"]+)"$"##)]
fn only_newer_suggestions(world: &mut WatnWorld, query: String) {
    assert_eq!(query, "o3");
    let current = wait_for_screen(world, &["Filter: o3", "o3-result"]);
    assert!(!current.contains("gpt-result"), "stale result was visible");
}

#[then(regex = r##"^a later result for "([^"]+)" should not replace them$"##)]
fn stale_suggestions_ignored(world: &mut WatnWorld, query: String) {
    assert_eq!(query, "gpt");
    std::thread::sleep(Duration::from_millis(800));
    let current = parse_screen(&pty_snapshot(
        world.pty_session.as_ref().expect("filter PTY session"),
    ))
    .text();
    assert!(current.contains("Filter: o3"));
    assert!(current.contains("o3-result"));
    assert!(!current.contains("gpt-result"));
}

#[given(
    regex = r##"^a configured provider with an incomplete model catalog containing "([^"]+)", "([^"]+)", and "([^"]+)"$"##
)]
fn configured_incomplete_catalog(
    world: &mut WatnWorld,
    first: String,
    second: String,
    third: String,
) {
    world.mock_server = MockServerWrap(Some(httpmock::MockServer::start()), None);
    let server = world.mock_server.0.as_ref().expect("catalog server");
    let base_url = format!("http://127.0.0.1:{}", server.port());
    let mut ids = vec![first, second, third];
    ids.extend((0..47).map(|index| format!("model-{index}")));
    let data = ids
        .iter()
        .map(|id| serde_json::json!({ "id": id }))
        .collect::<Vec<_>>();
    let catalog = server.mock(|when, then| {
        when.method(Method::GET)
            .path("/models")
            .query_param("page", "1")
            .query_param("limit", "50");
        then.status(200)
            .header("Content-Type", "application/json")
            .json_body(serde_json::json!({
                "data": data,
                "meta": {"has_more": true}
            }));
    });
    let gpt = server.mock(|when, then| {
        when.method(Method::GET)
            .path("/models")
            .query_param("search", "gpt");
        then.delay(Duration::from_millis(1000))
            .status(200)
            .header("Content-Type", "application/json")
            .json_body(serde_json::json!({ "data": [{ "id": "gpt-result" }] }));
    });
    let o3 = server.mock(|when, then| {
        when.method(Method::GET)
            .path("/models")
            .query_param("search", "o3");
        then.status(200)
            .header("Content-Type", "application/json")
            .json_body(serde_json::json!({ "data": [{ "id": "o3-pro" }] }));
    });
    world.models_mock_id = Some(catalog.id);
    world.search_mock_ids = vec![gpt.id, o3.id];
    world.raw_config = Some(format!(
        "[defaults]\nprovider = \"test\"\n\n[providers.test]\nendpoint = \"{base_url}\"\napi_key = \"test-key\"\n"
    ));
}

#[given("the provider delays a model search response")]
fn delayed_model_search(world: &mut WatnWorld) {
    assert_eq!(world.search_mock_ids.len(), 2);
}

#[when("I start the setup wizard in a terminal")]
fn start_filter_setup(world: &mut WatnWorld) {
    let session = start_pty_session(world, &["models"]);
    pty_wait_for_label(&session, "Small Model");
    world.pty_session = Some(session);
}

#[when(regex = r##"^I replace the filter with "([^"]+)" before the delayed response arrives$"##)]
fn replace_delayed_filter(world: &mut WatnWorld, query: String) {
    assert_eq!(query, "o3");
    std::thread::sleep(Duration::from_millis(300));
    let mut session = world.pty_session.take().expect("filter PTY session");
    pty_write(&mut session, "\x7f\x7f\x7f");
    pty_write(&mut session, &query);
    world.pty_session = Some(session);
}

#[then(regex = r##"^the terminal should keep showing the current filter "([^"]+)"$"##)]
fn terminal_filter_visible(world: &mut WatnWorld, query: String) {
    let current = wait_for_current_render(world, &format!("Filter: {query}"));
    assert!(current.contains(&format!("Filter: {query}")));
}

#[then(regex = r##"^the terminal should show the matching "([^"]+)" suggestion$"##)]
fn terminal_suggestion_visible(world: &mut WatnWorld, model: String) {
    let current = wait_for_screen(world, &["Filter: o3", model.as_str()]);
    assert!(current.contains(&model), "missing {model:?}: {current:?}");
}

#[when(regex = r##"^I replace the filter with "([^"]+)"$"##)]
fn replace_filter(world: &mut WatnWorld, query: String) {
    assert_eq!(query, "gpt");
    let mut session = world.pty_session.take().expect("filter PTY session");
    pty_write(&mut session, "\x7f\x7f");
    pty_write(&mut session, &query);
    world.pty_session = Some(session);
}

#[then(regex = r##"^the terminal should show the current filter "([^"]+)"$"##)]
fn terminal_filter_after_change(world: &mut WatnWorld, query: String) {
    let current = wait_for_current_render(world, &format!("Filter: {query}"));
    assert!(current.contains(&format!("Filter: {query}")));
}

#[when(regex = r##"^I type "([^"]+)" into the active model filter$"##)]
fn type_model_filter(world: &mut WatnWorld, query: String) {
    let mut session = world
        .pty_session
        .take()
        .unwrap_or_else(|| start_pty_session(world, &["models"]));
    pty_wait_for_label(&session, "Small Model");
    pty_write(&mut session, &query);
    world.pty_session = Some(session);
}

#[then(regex = r##"^the model filter should show "([^"]+)"$"##)]
fn model_filter_visible(world: &mut WatnWorld, query: String) {
    let current = wait_for_current_render(world, &format!("Filter: {query}"));
    assert!(current.contains(&format!("Filter: {query}")));
}

#[then(regex = r##"^the suggestions should contain "([^"]+)" and "([^"]+)"$"##)]
fn suggestions_contain(world: &mut WatnWorld, first: String, second: String) {
    let current = wait_for_current_render(world, "Filter: gpt");
    assert!(current.contains(&first), "missing {first:?}: {current:?}");
    assert!(current.contains(&second), "missing {second:?}: {current:?}");
}

#[then(regex = r##"^the suggestions should not contain "([^"]+)"$"##)]
fn suggestions_exclude(world: &mut WatnWorld, model: String) {
    let current = wait_for_current_render(world, "Filter: gpt");
    assert!(
        !current.contains(&model),
        "unexpected {model:?}: {current:?}"
    );
}

#[then("the provider should not receive a search request")]
fn no_provider_search(world: &mut WatnWorld) {
    let server = world.mock_server.0.as_ref().expect("catalog server");
    for id in &world.search_mock_ids {
        assert_eq!(httpmock::Mock::new(*id, server).hits(), 0);
    }
}

#[then(regex = r##"^the suggestions should contain "([^"]+)"$"##)]
fn one_suggestion(world: &mut WatnWorld, model: String) {
    let current = wait_for_screen(world, &["Filter: o3", model.as_str()]);
    assert!(current.contains(&model), "missing {model:?}: {current:?}");
}

#[then(regex = r##"^the provider should receive a search request for "([^"]+)"$"##)]
fn provider_received_search(world: &mut WatnWorld, query: String) {
    assert_eq!(query, "o3");
    let server = world.mock_server.0.as_ref().expect("catalog server");
    assert!(
        world
            .search_mock_ids
            .iter()
            .any(|id| httpmock::Mock::new(*id, server).hits() > 0),
        "provider did not receive a search request"
    );
}

fn wait_for_current_render(world: &WatnWorld, marker: &str) -> String {
    wait_for_screen(world, &[marker])
}

fn wait_for_screen(world: &WatnWorld, markers: &[&str]) -> String {
    let session = world.pty_session.as_ref().expect("filter PTY session");
    let deadline = Instant::now() + Duration::from_secs(3);
    loop {
        let rendered = parse_screen(&pty_snapshot(session)).text();
        if markers.iter().all(|marker| rendered.contains(marker)) {
            return rendered;
        }
        if Instant::now() >= deadline {
            panic!("screen markers {markers:?} were not rendered: {rendered:?}");
        }
        std::thread::sleep(Duration::from_millis(25));
    }
}
