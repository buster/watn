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

#[then(
    expr = "the review surface should show a billed amount of {string} cents with the model name"
)]
fn review_surface_shows_billed_amount(world: &mut WatnWorld, cents: String) {
    let expected: f64 = cents.parse().expect("a numeric expected amount");
    let plain = review_rendered_plain(world);
    let header = plain
        .lines()
        .find(|line| line.contains('◆'))
        .unwrap_or_else(|| panic!("the surface should show a model label, got:\n{plain}"));
    let amount_text = header
        .split('·')
        .map(str::trim)
        .find(|part| part.ends_with('¢'))
        .unwrap_or_else(|| panic!("the model label should carry a billed amount, got {header:?}"));
    let shown_text = amount_text.trim_end_matches('¢').trim();
    let shown: f64 = shown_text
        .parse()
        .unwrap_or_else(|_| panic!("a numeric shown amount, got {shown_text:?}"));
    let decimals = shown_text.split('.').nth(1).map_or(0, str::len);
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
    let model = world
        .review
        .panel
        .as_ref()
        .map(|panel| panel.context.model.clone())
        .unwrap_or_else(|| effective_review_model(&world.review));
    let model = model_short_name(&model);
    assert!(
        header.contains(&model),
        "the billed amount should appear with the model name {model:?}, got {header:?}"
    );
}
