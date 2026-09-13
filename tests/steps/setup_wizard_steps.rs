use cucumber::{given, then, when};

use super::{finish_pty_session, pty_snapshot, pty_wait_for_label, pty_write, start_pty_session};
use crate::WatnWorld;

fn strip_ansi(output: &str) -> String {
    regex::Regex::new(r"\x1b\[[0-9;?]*[ -/]*[@-~]")
        .expect("ANSI pattern")
        .replace_all(output, "")
        .to_string()
}

fn marker_shown(output: &str, marker: &str, name: &str) -> bool {
    output.contains(&format!("{marker}{name}")) || output.contains(&format!("{marker} {name}"))
}

fn assert_words(output: &str, text: &str) {
    for word in text.split_whitespace() {
        assert!(
            output.contains(word),
            "missing {word:?} in setup output: {output:?}"
        );
    }
}

fn latest_page(output: &str, page: &str) -> bool {
    output
        .rfind("Page")
        .map(|index| {
            let current = &output[index..];
            page.split_whitespace().all(|word| current.contains(word))
        })
        .unwrap_or(false)
}

fn wait_for_page(session: &super::PtySession, page: &str) -> String {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(3);
    loop {
        let output = pty_snapshot(session);
        if page == "Shell Completion" && output.to_ascii_lowercase().contains("completion") {
            return output;
        }
        if page == "Shell Shortcut" && output.to_ascii_lowercase().contains("shortcut") {
            return output;
        }
        if latest_page(&output, page) {
            return output;
        }
        if std::time::Instant::now() >= deadline {
            panic!("setup page {page:?} was not rendered: {output:?}");
        }
        std::thread::sleep(std::time::Duration::from_millis(25));
    }
}

#[when("I start `watn setup` in a terminal")]
fn start_setup_wizard(world: &mut WatnWorld) {
    let session = start_pty_session(world, &["setup"]);
    world.pty_session = Some(session);
    let session = world.pty_session.as_ref().expect("setup PTY session");
    pty_wait_for_label(session, "Setup");
}

#[given("the terminal color capability is disabled")]
fn terminal_color_capability_disabled(world: &mut WatnWorld) {
    world
        .env_vars
        .insert("NO_COLOR".to_string(), "1".to_string());
}

#[then("the setup frame should show the watn setup label")]
fn setup_frame_label(world: &mut WatnWorld) {
    let session = world.pty_session.as_ref().expect("setup PTY session");
    let output = strip_ansi(&pty_snapshot(session));
    assert!(
        output.contains("watn · setup"),
        "setup frame label missing: {output:?}"
    );
}

#[then(expr = "the active page marker should be {string}")]
fn active_page_marker(world: &mut WatnWorld, marker: String) {
    let session = world.pty_session.as_ref().expect("setup PTY session");
    let output = strip_ansi(&pty_snapshot(session));
    assert!(
        ["Provider", "Small Model", "Shell Completion", "Review"]
            .iter()
            .any(|page| marker_shown(&output, &marker, page)),
        "active page marker missing: {output:?}"
    );
}

#[then("the footer should show bold key hints with dim labels")]
fn footer_key_hints(world: &mut WatnWorld) {
    let session = world.pty_session.as_ref().expect("setup PTY session");
    let raw = pty_snapshot(session);
    let visible = strip_ansi(&raw);
    assert!(raw.contains("\u{1b}[1m"), "bold key hint missing: {raw:?}");
    assert!(raw.contains("38;5;81"), "cyan key hint missing: {raw:?}");
    assert!(raw.contains("\u{1b}[2m"), "dim label missing: {raw:?}");
    assert!(visible.contains('⏎'), "enter hint missing: {visible:?}");
}

#[then("the setup palette should use cyan labels and dim borders")]
fn setup_palette(world: &mut WatnWorld) {
    let session = world.pty_session.as_ref().expect("setup PTY session");
    let raw = pty_snapshot(session);
    assert!(raw.contains("38;5;81"), "cyan label color missing: {raw:?}");
    assert!(raw.contains("\u{1b}[2m"), "dim border missing: {raw:?}");
}

#[then(expr = "the setup warning should be marked with {string} in amber")]
fn setup_warning_amber(world: &mut WatnWorld, marker: String) {
    let session = world.pty_session.as_ref().expect("setup PTY session");
    let raw = pty_snapshot(session);
    let visible = strip_ansi(&raw);
    assert!(
        visible.contains(&marker),
        "warning marker {marker:?} missing: {visible:?}"
    );
    assert!(raw.contains("38;5;214"), "warning is not amber: {raw:?}");
}

#[then("the setup output should not contain indexed color sequences")]
fn no_indexed_color(world: &mut WatnWorld) {
    let session = world.pty_session.as_ref().expect("setup PTY session");
    let raw = pty_snapshot(session);
    assert!(
        !raw.contains("38;5;") && !raw.contains("48;5;"),
        "indexed color leaked into a plain terminal: {raw:?}"
    );
}

#[when("I start the shared `watn models` wizard in a terminal")]
fn start_shared_models_wizard(world: &mut WatnWorld) {
    let session = start_pty_session(world, &["models"]);
    world.pty_session = Some(session);
    world.pending_config.insert(
        "setup_wizard_models_entrypoint".to_string(),
        "true".to_string(),
    );
    let session = world.pty_session.as_ref().expect("setup PTY session");
    wait_for_page(session, "Small Model");
}

#[then(regex = r#"^the setup wizard should show tabs (.+)$"#)]
fn setup_wizard_tabs(_world: &mut WatnWorld, tab_list: String) {
    let titles = tab_list
        .split(", ")
        .map(|title| title.trim_matches('"'))
        .collect::<Vec<_>>();
    let session = _world.pty_session.as_ref().expect("setup PTY session");
    let output = pty_wait_for_label(session, titles[0]);
    for title in titles {
        assert_words(&output, title);
    }
}

#[then(
    regex = r#"^the setup wizard should show the (URL|API key|Catalog|Small Model|Small Reasoning|Normal Model|Normal Reasoning|Thinking Model|Thinking Reasoning|Shell Completion|Shell Shortcut) page as active$"#
)]
fn setup_wizard_active_page(world: &mut WatnWorld, page: String) {
    let session = world.pty_session.as_ref().expect("setup PTY session");
    let output = wait_for_page(session, &page);
    if page == "Shell Completion" {
        assert!(output.to_ascii_lowercase().contains("completion"));
        return;
    }
    if page == "Shell Shortcut" {
        assert!(output.to_ascii_lowercase().contains("shortcut"));
        return;
    }
    assert_words(&output, &page);
}

#[then("the setup wizard should explain OpenAI and LiteLLM compatibility")]
fn setup_wizard_compatibility_explanation(world: &mut WatnWorld) {
    let session = world.pty_session.as_ref().expect("setup PTY session");
    let output = pty_snapshot(session);
    for word in ["OpenAI", "LiteLLM", "compatible"] {
        assert_words(&output, word);
    }
}

#[then("the setup wizard should show a visible cursor on the active input")]
fn setup_wizard_visible_cursor(world: &mut WatnWorld) {
    let session = world.pty_session.as_ref().expect("setup PTY session");
    assert!(pty_snapshot(session).contains('█'), "cursor marker missing");
}

#[then("the setup wizard should show the provider controls and guidance")]
fn setup_wizard_provider_controls_guidance(world: &mut WatnWorld) {
    setup_wizard_tabs(
        world,
        "\"URL\", \"API key\", \"Catalog\", \"Small Model\", \"Normal Model\"".to_string(),
    );
    setup_wizard_active_page(world, "URL".to_string());
    setup_wizard_compatibility_explanation(world);
    setup_wizard_visible_cursor(world);
}

#[when("I enter the default endpoint and advance to the API key page")]
fn enter_default_endpoint(world: &mut WatnWorld) {
    let session = world.pty_session.as_mut().expect("setup PTY session");
    pty_write(session, "\r");
    std::thread::sleep(std::time::Duration::from_millis(100));
}

#[when("I advance to the API key page in provider setup")]
fn advance_provider_to_api_key(world: &mut WatnWorld) {
    enter_default_endpoint(world);
}

#[when("choose to store the API key in the configuration")]
fn choose_configuration_storage(world: &mut WatnWorld) {
    let session = world.pty_session.as_mut().expect("setup PTY session");
    pty_write(session, "p");
    std::thread::sleep(std::time::Duration::from_millis(100));
}

#[when(regex = r#"^enter API key "([^"]+)" and advance to Small Model$"#)]
fn enter_api_key(world: &mut WatnWorld, key: String) {
    let session = world.pty_session.as_mut().expect("setup PTY session");
    pty_write(session, &format!("{key}\r"));
    wait_for_page(session, "Catalog");
    pty_write(session, "\r");
    wait_for_page(session, "Small Model");
}

#[when("I configure the provider and models through the wizard")]
fn configure_provider_and_models(world: &mut WatnWorld) {
    accept_preselected_provider(world);
    enter_default_endpoint(world);
    choose_configuration_storage(world);
    enter_api_key(world, "sk-wizard-key".to_string());
    choose_two_models(world, "model-small".to_string(), "model-middle".to_string());
    type_large_model(world, "model-large".to_string());
    confirm_large_model(world);
}

/// The title of the last editor box rendered by the wizard. Page changes
/// re-render the active editor box, so the last `(editing)` marker names the
/// page that currently owns the input.
pub(crate) fn latest_editor_title(output: &str) -> Option<String> {
    let index = output.rfind("(editing)")?;
    let before = &output[..index];
    let start = before
        .rfind('┌')
        .map(|position| position + '┌'.len_utf8())
        .unwrap_or(0);
    Some(before[start..].trim().to_string())
}

pub(crate) fn wait_for_editor(session: &super::PtySession, title: &str) {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
    loop {
        let output = strip_ansi(&pty_snapshot(session));
        if latest_editor_title(&output).as_deref() == Some(title) {
            return;
        }
        if std::time::Instant::now() >= deadline {
            panic!("setup editor {title:?} was not rendered: {output:?}");
        }
        std::thread::sleep(std::time::Duration::from_millis(25));
    }
}

/// The wizard opens on the provider choice page when an existing custom
/// provider must be confirmed; accept the preselected provider before the URL
/// page. Scenarios that already chose a provider land on the URL page and skip
/// this.
fn accept_preselected_provider(world: &mut WatnWorld) {
    let on_provider_choice = {
        let session = world.pty_session.as_ref().expect("setup PTY session");
        let output = strip_ansi(&pty_snapshot(session));
        latest_editor_title(&output).as_deref() == Some("Provider")
    };
    if on_provider_choice {
        let session = world.pty_session.as_mut().expect("setup PTY session");
        pty_write(session, "\r");
        wait_for_editor(session, "URL");
    }
}

#[when(regex = r#"^choose "([^"]+)" and "([^"]+)" with Enter$"#)]
fn choose_two_models(world: &mut WatnWorld, small: String, middle: String) {
    let session = world.pty_session.as_mut().expect("setup PTY session");
    for (model, reasoning_page, next_page) in [
        (small, "Small Reasoning", "Normal Model"),
        (middle, "Normal Reasoning", "Thinking Model"),
    ] {
        pty_write(session, &model);
        std::thread::sleep(std::time::Duration::from_millis(400));
        pty_write(session, "\r");
        wait_for_page(session, reasoning_page);
        pty_write(session, "\r");
        wait_for_page(session, next_page);
    }
}

#[when(regex = r#"^I confirm the (Small|Normal) Reasoning selection with Enter$"#)]
fn confirm_reasoning_selection(world: &mut WatnWorld, role: String) {
    let next_page = match role.as_str() {
        "Small" => "Normal Model",
        "Normal" => "Thinking Model",
        _ => unreachable!(),
    };
    let session = world.pty_session.as_mut().expect("setup PTY session");
    pty_write(session, "\r");
    wait_for_page(session, next_page);
}

#[when(regex = r#"^I type "([^"]+)" on the Thinking Model page$"#)]
fn type_large_model(world: &mut WatnWorld, model: String) {
    let session = world.pty_session.as_mut().expect("setup PTY session");
    pty_write(session, &model);
    std::thread::sleep(std::time::Duration::from_millis(500));
}

#[when("I confirm the Thinking Model selection with Enter")]
fn confirm_large_model(world: &mut WatnWorld) {
    let models_entrypoint = world
        .pending_config
        .contains_key("setup_wizard_models_entrypoint");
    {
        let session = world.pty_session.as_mut().expect("setup PTY session");
        pty_write(session, "\r");
        wait_for_page(session, "Thinking Reasoning");
        pty_write(session, "\r");
    }
    if models_entrypoint {
        let session = world.pty_session.take().expect("setup PTY session");
        finish_pty_session(world, session);
    } else {
        std::thread::sleep(std::time::Duration::from_millis(300));
    }
}

#[then("the setup wizard should explain shell completion installation")]
fn setup_wizard_shell_completion_explanation(world: &mut WatnWorld) {
    let session = world.pty_session.as_ref().expect("setup PTY session");
    let output = pty_snapshot(session);
    for word in ["Install", "completion", "Tab", "reload"] {
        assert_words(&output, word);
    }
}

#[when("I skip shell completion setup")]
fn skip_shell_completion_setup(world: &mut WatnWorld) {
    let session = world.pty_session.as_mut().expect("setup PTY session");
    pty_write(session, "\r");
    std::thread::sleep(std::time::Duration::from_millis(200));
}

#[then("the setup wizard should explain shell shortcut installation")]
fn setup_wizard_shell_shortcut_explanation(world: &mut WatnWorld) {
    let session = world.pty_session.as_ref().expect("setup PTY session");
    let output = pty_snapshot(session);
    for word in ["Ctrl-W", "review", "Enter", "automatically"] {
        assert_words(&output, word);
    }
}

#[when("I skip shell integration setup")]
fn skip_shell_integration_setup(world: &mut WatnWorld) {
    let session = world.pty_session.as_mut().expect("setup PTY session");
    pty_write(session, "\r");
    wait_for_page(session, "Review");
    pty_write(session, "\r");
    std::thread::sleep(std::time::Duration::from_millis(200));
    let session = world.pty_session.take().expect("setup PTY session");
    finish_pty_session(world, session);
}

#[when("I complete the optional shell pages without integrations")]
fn complete_optional_shell_pages(world: &mut WatnWorld) {
    setup_wizard_active_page(world, "Shell Completion".to_string());
    setup_wizard_shell_completion_explanation(world);
    skip_shell_completion_setup(world);
    setup_wizard_active_page(world, "Shell Shortcut".to_string());
    setup_wizard_shell_shortcut_explanation(world);
    skip_shell_integration_setup(world);
}

#[then("setup should exit successfully")]
fn setup_exits_successfully(world: &mut WatnWorld) {
    assert_eq!(
        world.exit_status,
        Some(0),
        "setup output: {:?}",
        world.output
    );
}

#[then(regex = r#"^the setup wizard should show the URL and API key tabs$"#)]
fn setup_wizard_provider_tabs(world: &mut WatnWorld) {
    let session = world.pty_session.as_ref().expect("setup PTY session");
    let output = pty_snapshot(session);
    assert_words(&output, "URL");
    assert_words(&output, "API key");
}

#[then("the setup wizard should show model choices in a table")]
fn setup_wizard_model_table(world: &mut WatnWorld) {
    let session = world.pty_session.as_ref().expect("setup PTY session");
    let output = pty_snapshot(session);
    for column in ["Model", "Context", "Pricing", "Features"] {
        assert_words(&output, column);
    }
}

#[then("the setup wizard should show model-specific reasoning options")]
fn setup_wizard_reasoning_options(world: &mut WatnWorld) {
    let session = world.pty_session.as_ref().expect("setup PTY session");
    let output = pty_snapshot(session);
    assert_words(&output, "Choices:");
    assert_words(&output, "low");
}

#[when("I choose the second model and advance with Enter")]
fn choose_second_model(world: &mut WatnWorld) {
    let session = world.pty_session.as_mut().expect("setup PTY session");
    pty_write(session, "\x1b[B\r");
    std::thread::sleep(std::time::Duration::from_millis(200));
}

#[when("press Escape in the setup wizard")]
fn press_escape_in_setup_wizard(world: &mut WatnWorld) {
    let dir = world.temp_dir.as_ref().expect("config temp dir");
    let path = dir.path().join("watn").join("config.toml");
    let content = std::fs::read_to_string(&path).expect("config file");
    world
        .pending_config
        .insert("config_before".to_string(), content);
    let session = world.pty_session.as_mut().expect("setup PTY session");
    pty_write(session, "\x1b");
    std::thread::sleep(std::time::Duration::from_millis(150));
}

#[then("the setup wizard should ask whether to save current settings")]
fn setup_wizard_save_prompt(world: &mut WatnWorld) {
    let session = world.pty_session.as_ref().expect("setup PTY session");
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(3);
    loop {
        let output = strip_ansi(&pty_snapshot(session));
        if output.contains("save") && output.contains("discard") {
            return;
        }
        if std::time::Instant::now() >= deadline {
            panic!("save prompt was not rendered: {output:?}");
        }
        std::thread::sleep(std::time::Duration::from_millis(25));
    }
}

#[when("I choose to discard current setup")]
fn discard_current_setup(world: &mut WatnWorld) {
    let session = world.pty_session.as_mut().expect("setup PTY session");
    pty_write(session, "n");
    std::thread::sleep(std::time::Duration::from_millis(100));
    let session = world.pty_session.take().expect("setup PTY session");
    finish_pty_session(world, session);
    assert_eq!(world.exit_status, Some(1), "discard should cancel setup");
}

#[then(
    regex = r#"^the config file should contain small tier "([^"]+)", normal tier "([^"]+)", and thinking tier "([^"]+)"$"#
)]
fn config_contains_wizard_tiers(
    world: &mut WatnWorld,
    small: String,
    middle: String,
    large: String,
) {
    let dir = world.temp_dir.as_ref().expect("config temp dir");
    let path = dir.path().join("watn").join("config.toml");
    let raw = std::fs::read_to_string(&path).expect("read wizard config");
    let config: watn::config::types::Config = toml::from_str(&raw).expect("parse wizard config");
    assert_eq!(config.tiers.small.as_deref(), Some(small.as_str()));
    assert_eq!(config.tiers.normal.as_deref(), Some(middle.as_str()));
    assert_eq!(config.tiers.thinking.as_deref(), Some(large.as_str()));
}
