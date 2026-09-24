//! End-to-end steps for the `visible-request-amount` capability
//! (observe-request-cost).
//!
//! These drive the real `watn` binary on a pseudo-terminal and assert on what
//! the terminal shows; the in-process assertions live in
//! `observe_request_cost_steps.rs`.

use cucumber::{given, then};

use crate::steps::build_config;
use crate::WatnWorld;

/// The model the mock provider answers as, with a recorded per-million price
/// and reported usage, so the review surface can show what the request cost.
#[given(
    expr = "a configured model {string} with a recorded price of {float} input and {float} output per million tokens"
)]
fn e2e_configured_model_with_price(world: &mut WatnWorld, model: String, input: f64, output: f64) {
    world.raw_config = Some(build_config(
        "openai",
        None,
        None,
        Some(vec![(model.as_str(), input, output)]),
        None,
        Some(model.as_str()),
    ));
    world.pending_mock_model = Some(model.clone());
    world.pending_mock_usage = Some(true);
    world.review.model = model;
}

fn assert_terminal_shows_billed_amount(world: &WatnWorld, cents: &str) {
    let session = world.pty_session.as_ref().expect("PTY session");
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
    while !crate::steps::pty_snapshot(session).contains('¢') && std::time::Instant::now() < deadline
    {
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
    let rendered = crate::steps::observe_request_cost_steps::plain_surface_text(
        &crate::steps::pty_snapshot(session),
    );
    crate::steps::observe_request_cost_steps::assert_shows_billed_amount(&rendered, cents, None);
}

#[then(
    expr = "the explanation card should show a billed amount of {string} cents with the model name"
)]
fn e2e_explanation_card_shows_billed_amount(world: &mut WatnWorld, cents: String) {
    assert_terminal_shows_billed_amount(world, &cents);
}

#[then("the Bash command line should be exactly the accepted candidate")]
fn e2e_bash_line_is_exactly_candidate(world: &mut WatnWorld) {
    assert_eq!(
        world.review.bash_command_line.trim_end(),
        super::interactive_shell_shortcut_e2e_steps::expected_e2e_candidate().trim_end(),
        "the shortcut buffer must carry the accepted candidate and nothing else"
    );
}

#[then("the Bash command line should not contain a billed amount")]
fn e2e_bash_line_has_no_amount(world: &mut WatnWorld) {
    assert!(
        !world.review.bash_command_line.contains('¢'),
        "no billed amount may reach the shell buffer, got {:?}",
        world.review.bash_command_line
    );
}
