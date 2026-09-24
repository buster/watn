//! Steps for the `visible-request-amount` capability (observe-request-cost).
//!
//! In-process scenarios render the review surface through the shared review
//! harness; the pseudo-terminal drivers live in
//! `observe_request_cost_e2e_steps.rs`.

use cucumber::{given, then};

use crate::steps::interactive_shell_shortcut_steps::{
    effective_review_model, review_rendered_plain, ReviewState,
};
use crate::WatnWorld;

/// The Model short name the surface shows for a model id.
fn model_short_name(model: &str) -> String {
    let last = model.rsplit('/').next().unwrap_or(model);
    let last = last.strip_prefix('~').unwrap_or(last);
    let short = last.split(':').next().unwrap_or(last);
    if short.is_empty() {
        model.to_string()
    } else {
        short.to_string()
    }
}

fn recorded_price(state: &mut ReviewState, model: &str, input: f64, output: f64) {
    state.prices.retain(|(id, _, _)| id != model);
    state.prices.push((model.to_string(), input, output));
}

#[given(
    expr = "a recorded price of {float} input and {float} output per million tokens for the configured model"
)]
fn recorded_price_for_configured_model(world: &mut WatnWorld, input: f64, output: f64) {
    let model = effective_review_model(&world.review);
    recorded_price(&mut world.review, &model, input, output);
}

#[given(
    expr = "a recorded price of {float} input and {float} output per million tokens for the model {string}"
)]
fn recorded_price_for_model(world: &mut WatnWorld, input: f64, output: f64, model: String) {
    recorded_price(&mut world.review, &model, input, output);
}

#[given(expr = "the provider response reports {int} prompt and {int} completion tokens")]
fn provider_response_reports_usage(
    world: &mut WatnWorld,
    prompt_tokens: u32,
    completion_tokens: u32,
) {
    world.review.reported_usage = Some((prompt_tokens, completion_tokens));
}

#[given(
    expr = "a regeneration for the model {string} reports {int} prompt and {int} completion tokens"
)]
fn regeneration_reports_usage(
    world: &mut WatnWorld,
    model: String,
    prompt_tokens: u32,
    completion_tokens: u32,
) {
    world.review.catalog_models.push(model);
    world.review.regeneration_usage = Some((prompt_tokens, completion_tokens));
}

/// The rendered surface as plain text, without ANSI escapes or frame glyphs.
pub(crate) fn plain_surface_text(raw: &str) -> String {
    strip_ansi(raw)
        .replace("\r\n", "\n")
        .replace(['│', '┌', '┐', '└', '┘', '─'], " ")
}

/// The header line of a plain rendered surface.
pub(crate) fn header_line(plain: &str) -> Option<&str> {
    plain.lines().rev().find(|line| line.contains('◆'))
}

/// The billed amount shown at the model label: its value, how many decimals
/// the surface displayed, and the text as shown.
pub(crate) fn shown_amount(header: &str) -> Option<(f64, usize, String)> {
    let amount_text = header
        .split('·')
        .map(str::trim)
        .find(|part| part.ends_with('¢'))?;
    let shown_text = amount_text.trim_end_matches('¢').trim().to_string();
    let shown: f64 = shown_text.parse().ok()?;
    let decimals = shown_text.split('.').nth(1).map_or(0, str::len);
    Some((shown, decimals, shown_text))
}

/// Assert that a rendered surface shows the expected amount at the model label.
pub(crate) fn assert_shows_billed_amount(plain: &str, cents: &str, model: Option<&str>) {
    let expected: f64 = cents.parse().expect("a numeric expected amount");
    let header = header_line(plain)
        .unwrap_or_else(|| panic!("the surface should show a model label, got:\n{plain}"));
    let (shown, decimals, shown_text) = shown_amount(header)
        .unwrap_or_else(|| panic!("the model label should carry a billed amount, got {header:?}"));
    let scale = 10f64.powi(decimals as i32);
    let rounded = (expected * scale).round() / scale;
    assert!(
        (shown - rounded).abs() < f64::EPSILON,
        "expected a billed amount of {expected} cents at {decimals} decimals, got {shown_text:?} in {header:?}"
    );
    if expected != 0.0 {
        assert!(
            shown != 0.0,
            "a request the provider accounted for must not read as zero: {header:?}"
        );
    }
    if let Some(model) = model {
        if !header.contains('…') {
            assert!(
                header.contains(model),
                "the billed amount should appear with the model name {model:?}, got {header:?}"
            );
        }
    }
}

fn surface_text(world: &WatnWorld) -> String {
    if world.review.e2e {
        if let Some(session) = world.pty_session.as_ref() {
            let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
            while !crate::steps::pty_snapshot(session).contains('¢')
                && std::time::Instant::now() < deadline
            {
                std::thread::sleep(std::time::Duration::from_millis(50));
            }
            return plain_surface_text(&crate::steps::pty_snapshot(session));
        }
    }
    review_rendered_plain(world)
}

fn strip_ansi(value: &str) -> String {
    let mut out = String::new();
    let mut chars = value.chars();
    while let Some(character) = chars.next() {
        if character == '\u{1b}' {
            for escaped in chars.by_ref() {
                if escaped.is_ascii_alphabetic() {
                    break;
                }
            }
        } else {
            out.push(character);
        }
    }
    out
}

#[then(
    expr = "the review surface should show a billed amount of {string} cents with the model name"
)]
fn review_surface_shows_billed_amount(world: &mut WatnWorld, cents: String) {
    let plain = surface_text(world);
    let model = world
        .review
        .panel
        .as_ref()
        .map(|panel| panel.context.model.clone())
        .unwrap_or_else(|| effective_review_model(&world.review));
    let model = model_short_name(&model);
    assert_shows_billed_amount(&plain, &cents, Some(&model));
}

#[then("the review surface should not open")]
fn review_surface_does_not_open(world: &mut WatnWorld) {
    assert!(
        !world.review.surface_open && world.review.rendered.is_empty(),
        "a failed request opens no surface, got {:?}",
        world.review.rendered
    );
}

/// The header line of the rendered surface, without frame glyphs.
fn rendered_header(world: &WatnWorld) -> String {
    let plain = review_rendered_plain(world);
    plain
        .lines()
        .find(|line| line.contains('◆'))
        .unwrap_or_else(|| panic!("the surface should show a model label, got:\n{plain}"))
        .to_string()
}

#[then("the review surface should show no billed amount")]
fn review_surface_shows_no_billed_amount(world: &mut WatnWorld) {
    let plain = review_rendered_plain(world);
    assert!(
        !plain.contains('¢'),
        "no billed amount should be shown, got:\n{plain}"
    );
}

#[then("no billed amount should be shown")]
fn no_billed_amount_is_shown(world: &mut WatnWorld) {
    let plain = review_rendered_plain(world);
    assert!(
        !plain.contains('¢'),
        "no billed amount should be shown, got:\n{plain}"
    );
}

#[given("the provider response reports no usage")]
fn provider_response_reports_no_usage(world: &mut WatnWorld) {
    world.review.reported_usage = None;
}

#[then("the metadata amount for the request should be zero")]
fn metadata_amount_is_zero(world: &mut WatnWorld) {
    let model = effective_review_model(&world.review);
    let (_, input, output) = world
        .review
        .prices
        .iter()
        .find(|(id, _, _)| *id == model)
        .unwrap_or_else(|| panic!("a recorded price for {model:?}"));
    let pricing = watn::config::types::ModelPricing {
        input: *input,
        output: *output,
    };
    let billed = watn::amount::billed_amount(None, Some(&pricing))
        .expect("a recorded price yields a metadata amount");
    assert!(!billed.usage_reported());
    assert_eq!(
        billed.usd(),
        0.0,
        "an unaccounted request keeps the metadata line's existing zero"
    );
}

#[given(expr = "the configured model is {string} reported by the provider as {string}")]
fn configured_model_reported_as(world: &mut WatnWorld, model: String, reported: String) {
    world.review.model = model;
    world.review.reported_model = Some(reported);
}

#[given("the review terminal is 40 columns wide")]
fn review_terminal_is_narrow(world: &mut WatnWorld) {
    world.review.narrow = true;
}

#[then("the review surface should shorten the model name to fit")]
fn review_surface_shortens_model_name(world: &mut WatnWorld) {
    let header = rendered_header(world);
    let model = model_short_name(&effective_review_model(&world.review));
    assert!(
        header.contains('…'),
        "the model name should be shortened to fit, got {header:?}"
    );
    assert!(
        !header.contains(&model),
        "the full model name {model:?} should not fit at this width, got {header:?}"
    );
}

#[then(expr = "the preserved candidate should still show its billed amount of {string} cents")]
fn preserved_candidate_keeps_amount(world: &mut WatnWorld, cents: String) {
    let expected: f64 = cents.parse().expect("a numeric expected amount");
    let header = rendered_header(world);
    let amount_text = header
        .split('·')
        .map(str::trim)
        .find(|part| part.ends_with('¢'))
        .unwrap_or_else(|| {
            panic!("the preserved candidate should keep its amount, got {header:?}")
        });
    let shown_text = amount_text.trim_end_matches('¢').trim();
    let shown: f64 = shown_text
        .parse()
        .unwrap_or_else(|_| panic!("a numeric shown amount, got {shown_text:?}"));
    assert!(
        (shown - expected).abs() < f64::EPSILON,
        "the preserved candidate should keep {expected} cents, got {shown_text:?}"
    );
    let panel = world.review.panel.as_ref().expect("review panel state");
    assert_eq!(
        panel.candidate().command,
        world.review.candidate_command,
        "the preserved candidate is the one the failed regeneration started from"
    );
}

#[given("the provider request fails before completing")]
fn provider_request_fails(world: &mut WatnWorld) {
    world.review.no_candidate = true;
}

#[then(expr = "the released command should be exactly {string}")]
fn released_command_is_exactly(world: &mut WatnWorld, command: String) {
    let released = world
        .review
        .released
        .clone()
        .expect("an accepted candidate should be released");
    assert_eq!(
        released, command,
        "the released command should carry nothing but the accepted candidate"
    );
}

#[then("no billed amount should be shown in the released command")]
fn released_command_has_no_amount(world: &mut WatnWorld) {
    let released = world.review.released.clone().unwrap_or_default();
    let output = world.review.command_output.clone();
    assert!(
        !released.contains('¢') && !output.contains('¢'),
        "no billed amount may reach the released command: released={released:?} output={output:?}"
    );
}
