//! Step definitions for reasoning-policy scenarios.

use crate::WatnWorld;
use cucumber::{given, then, when};
use watn::models::dialog::resolve_reasoning_default;
use watn::models::list::ModelReasoning;

fn metadata(world: &WatnWorld) -> ModelReasoning {
    ModelReasoning {
        default_effort: world.pending_config.get("default_effort").cloned(),
        default_enabled: world
            .pending_config
            .get("default_enabled")
            .map(|v| v == "true")
            .unwrap_or(true),
        mandatory: world
            .pending_config
            .get("mandatory")
            .map(|v| v == "true")
            .unwrap_or(false),
        supported_efforts: world
            .pending_config
            .get("supported")
            .map(|v| v.split(',').map(str::to_string).collect())
            .unwrap_or_default(),
    }
}

#[given(
    regex = r##"^model reasoning metadata has default effort "([^"]+)", default enabled (true|false), and supported efforts "([^"]+)", "([^"]+)"$"##
)]
fn reasoning_metadata(
    world: &mut WatnWorld,
    effort: String,
    enabled: String,
    first: String,
    second: String,
) {
    world.pending_config.insert("default_effort".into(), effort);
    world
        .pending_config
        .insert("default_enabled".into(), enabled);
    world
        .pending_config
        .insert("supported".into(), format!("{first},{second}"));
}

#[when("I resolve the model reasoning default")]
fn resolve_reasoning(world: &mut WatnWorld) {
    let result = resolve_reasoning_default(&metadata(world), None)
        .map(|value| value.as_str().to_string())
        .map_err(|error| error.to_string());
    world.pending_config.insert(
        "resolved_reasoning".into(),
        result.unwrap_or_else(|error| format!("error:{error}")),
    );
}

#[then(regex = r##"^the selected reasoning should be "([^"]+)"$"##)]
fn selected_reasoning(world: &mut WatnWorld, expected: String) {
    assert_eq!(
        world.pending_config.get("resolved_reasoning"),
        Some(&expected)
    );
}
