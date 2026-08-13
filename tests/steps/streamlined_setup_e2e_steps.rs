use cucumber::{then, when};

use super::{finish_pty_session, pty_snapshot, pty_write};
use crate::WatnWorld;

fn visible_output(output: &str) -> String {
    regex::Regex::new(r"\x1b\[[0-9;?]*[ -/]*[@-~]")
        .expect("ANSI pattern")
        .replace_all(output, "")
        .to_string()
}

fn wait_for_page(session: &super::PtySession, title: &str) {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
    loop {
        let output = visible_output(&pty_snapshot(session));
        let page = output.rfind("Page").map(|index| &output[index..]);
        if page.is_some_and(|page| title.split_whitespace().all(|word| page.contains(word))) {
            return;
        }
        if std::time::Instant::now() >= deadline {
            panic!("E2E page {title:?} was not rendered: {output:?}");
        }
        std::thread::sleep(std::time::Duration::from_millis(25));
    }
}

#[then("provider setup should show provider choices \"OpenRouter\", \"OpenAI\", and \"Custom\"")]
fn provider_choices_are_visible(world: &mut WatnWorld) {
    let session = world.pty_session.as_ref().expect("provider PTY session");
    let output = visible_output(&pty_snapshot(session));
    for choice in ["OpenRouter", "OpenAI", "Custom"] {
        assert!(
            output.contains(choice),
            "provider choice missing: {output:?}"
        );
    }
}

#[when(regex = r##"^I choose provider "OpenAI"$"##)]
fn choose_openai_provider(world: &mut WatnWorld) {
    let session = world.pty_session.as_mut().expect("provider PTY session");
    pty_write(session, "\x1b[A\r");
    wait_for_page(session, "URL");
}

#[when(regex = r##"^choose environment variable "([^\"]+)"$"##)]
fn choose_e2e_environment_variable(world: &mut WatnWorld, variable: String) {
    let session = world.pty_session.as_mut().expect("provider PTY session");
    pty_write(session, "e");
    std::thread::sleep(std::time::Duration::from_millis(100));
    pty_write(session, "\x15");
    pty_write(session, &format!("{variable}\r"));
    let session = world.pty_session.take().expect("provider PTY session");
    finish_pty_session(world, session);
}

#[then("provider setup should exit successfully")]
fn provider_setup_exits_successfully(world: &mut WatnWorld) {
    assert_eq!(
        world.exit_status,
        Some(0),
        "provider output: {:?}",
        world.output
    );
}

#[then(regex = r##"^the config file should contain credential reference "([^\"]+)"$"##)]
fn config_contains_credential_reference(world: &mut WatnWorld, reference: String) {
    let path = world
        .temp_dir
        .as_ref()
        .expect("config directory")
        .path()
        .join("watn/config.toml");
    let content = std::fs::read_to_string(path).expect("provider config");
    assert!(content.contains(&format!("api_key = \"{reference}\"")));
}

#[when(regex = r##"^I choose provider "OpenRouter"$"##)]
fn choose_openrouter_provider(world: &mut WatnWorld) {
    let session = world.pty_session.as_mut().expect("setup PTY session");
    pty_write(session, "\r");
    wait_for_page(session, "URL");
}

#[when("accept the default completion endpoint")]
fn accept_default_completion_endpoint(world: &mut WatnWorld) {
    let session = world.pty_session.as_mut().expect("setup PTY session");
    pty_write(session, "\r");
    wait_for_page(session, "API key");
}

#[when("choose to paste an API key")]
fn choose_pasted_api_key(world: &mut WatnWorld) {
    let session = world.pty_session.as_mut().expect("setup PTY session");
    pty_write(session, "p");
    std::thread::sleep(std::time::Duration::from_millis(100));
}

#[when(regex = r##"^enter API key "([^\"]+)"$"##)]
fn enter_e2e_api_key(world: &mut WatnWorld, key: String) {
    let session = world.pty_session.as_mut().expect("setup PTY session");
    pty_write(session, &format!("{key}\r"));
    wait_for_page(session, "Catalog");
}

#[when("accept the derived catalog endpoint")]
fn accept_derived_catalog_endpoint(world: &mut WatnWorld) {
    let session = world.pty_session.as_mut().expect("setup PTY session");
    pty_write(session, "\r");
    wait_for_page(session, "Small Model");
}

#[when(regex = r##"^choose "([^\"]+)" for the (normal|thinking) role$"##)]
fn choose_e2e_model(world: &mut WatnWorld, model: String, role: String) {
    let next = match role.as_str() {
        "normal" => "Normal Reasoning",
        "thinking" => "Thinking Reasoning",
        _ => unreachable!(),
    };
    let session = world.pty_session.as_mut().expect("setup PTY session");
    pty_write(session, &model);
    std::thread::sleep(std::time::Duration::from_millis(350));
    pty_write(session, "\r");
    wait_for_page(session, next);
}

#[when(regex = r##"^I choose "([^\"]+)" for the small role$"##)]
fn choose_e2e_small_model(world: &mut WatnWorld, model: String) {
    let session = world.pty_session.as_mut().expect("models PTY session");
    pty_write(session, &model);
    std::thread::sleep(std::time::Duration::from_millis(350));
    pty_write(session, "\r");
    wait_for_page(session, "Small Reasoning");
}

#[when(regex = r##"^choose reasoning "([^\"]+)" for the (small|normal|thinking) role$"##)]
fn choose_e2e_reasoning(world: &mut WatnWorld, effort: String, role: String) {
    let steps = match effort.as_str() {
        "off" => 0,
        "low" => 1,
        "minimal" => 2,
        "medium" => 3,
        "high" => 4,
        _ => panic!("unsupported E2E reasoning effort {effort}"),
    };
    let next = match role.as_str() {
        "small" => "Normal Model",
        "normal" => "Thinking Model",
        "thinking" => "Shell Completion",
        _ => unreachable!(),
    };
    let session = world.pty_session.as_mut().expect("setup PTY session");
    for _ in 0..steps {
        pty_write(session, "\x1b[B");
    }
    pty_write(session, "\r");
    if next == "Shell Completion" && world.pending_config.contains_key("e2e_models_flow") {
        let session = world.pty_session.take().expect("models PTY session");
        finish_pty_session(world, session);
        return;
    }
    if next == "Shell Completion" {
        std::thread::sleep(std::time::Duration::from_millis(250));
        let output = visible_output(&pty_snapshot(session));
        assert!(
            output.to_ascii_lowercase().contains("shell completion"),
            "shell completion page was not rendered: {output:?}"
        );
    } else {
        wait_for_page(session, next);
    }
}

#[then("the model setup should begin with the small role")]
fn model_setup_begins_with_small_role(world: &mut WatnWorld) {
    let session = world.pty_session.as_ref().expect("models PTY session");
    let output = visible_output(&pty_snapshot(session));
    assert!(
        output.contains("Small Model"),
        "small role was not active: {output:?}"
    );
    world
        .pending_config
        .insert("e2e_models_flow".to_string(), "true".to_string());
}

#[then("models setup should exit successfully")]
fn models_setup_exits_successfully(world: &mut WatnWorld) {
    assert_eq!(
        world.exit_status,
        Some(0),
        "models output: {:?}",
        world.output
    );
}

#[then("the config file should contain the three selected model roles")]
fn config_contains_three_selected_model_roles(world: &mut WatnWorld) {
    let path = world
        .temp_dir
        .as_ref()
        .expect("config directory")
        .path()
        .join("watn/config.toml");
    let content = std::fs::read_to_string(path).expect("models config");
    for model in ["small-model", "normal-model", "thinking-model"] {
        assert!(
            content.contains(model),
            "selected model missing: {content:?}"
        );
    }
}

#[when("choose no shell completion integrations")]
fn choose_no_e2e_completion_integrations(world: &mut WatnWorld) {
    let session = world.pty_session.as_mut().expect("setup PTY session");
    pty_write(session, "\r");
    std::thread::sleep(std::time::Duration::from_millis(250));
    let output = visible_output(&pty_snapshot(session));
    assert!(
        output.to_ascii_lowercase().contains("shortcut"),
        "shell shortcut page was not rendered: {output:?}"
    );
}

#[when("choose no Ctrl-W shortcut integrations")]
fn choose_no_e2e_shortcut_integrations(world: &mut WatnWorld) {
    let session = world.pty_session.as_mut().expect("setup PTY session");
    pty_write(session, "\r");
    wait_for_page(session, "Review");
}

#[when("confirm the setup review")]
fn confirm_e2e_setup_review(world: &mut WatnWorld) {
    let session = world.pty_session.as_mut().expect("setup PTY session");
    pty_write(session, "\r");
    let session = world.pty_session.take().expect("setup PTY session");
    finish_pty_session(world, session);
}

#[then(
    regex = r##"^the config file should contain (small|normal|thinking) model "([^\"]+)" with reasoning "([^\"]+)"$"##
)]
fn e2e_config_contains_role(world: &mut WatnWorld, role: String, model: String, reasoning: String) {
    let path = world
        .temp_dir
        .as_ref()
        .expect("config directory")
        .path()
        .join("watn/config.toml");
    let content = std::fs::read_to_string(path).expect("E2E config");
    let config: watn::config::types::Config = toml::from_str(&content).expect("parse E2E config");
    let saved_model = match role.as_str() {
        "small" => config.tiers.small.as_deref(),
        "normal" => config.tiers.normal.as_deref(),
        "thinking" => config.tiers.thinking.as_deref(),
        _ => unreachable!(),
    };
    let saved_reasoning = match role.as_str() {
        "small" => config.tiers.reasoning.small.as_deref(),
        "normal" => config.tiers.reasoning.normal.as_deref(),
        "thinking" => config.tiers.reasoning.thinking.as_deref(),
        _ => unreachable!(),
    };
    assert_eq!(saved_model, Some(model.as_str()));
    assert_eq!(saved_reasoning, Some(reasoning.as_str()));
}
