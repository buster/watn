use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use crate::config::types::Config;
use crate::error::Error;
use crate::models::list::fetch_models;
use crate::output::spinner::Spinner;
use crate::provider::{Message, Provider, RequestOptions, StreamEvent, StreamingResponse};

use super::buffer::ReviewBuffer;
use super::panel::TierChoice;
use super::response::{
    apply_explanation_outcome, candidate_from_provider_response, ExplanationOutcome,
    ReviewCandidate,
};

const CANCEL_GRACE: Duration = Duration::from_millis(500);
const CANCEL_POLL: Duration = Duration::from_millis(20);

/// A completed review-mode generation: the provider response and the buffered
/// provider payload that is parsed into a candidate.
pub struct Generation {
    pub response: StreamingResponse,
    pub buffer: ReviewBuffer,
}

/// Stream one review-mode candidate. The provider runs on a scoped worker so a
/// borrowed registry entry stays valid, and the shared interrupt flag keeps the
/// existing Ctrl+C grace contract.
pub fn generate_candidate(
    provider: &dyn Provider,
    messages: &[Message],
    options: &RequestOptions,
    interrupt: &Arc<AtomicBool>,
    spinner: Option<Spinner>,
) -> Result<Generation, Error> {
    let mut spinner = spinner;
    let (result, buffer) = thread::scope(|scope| {
        let handle = scope.spawn(|| {
            let mut buffer = ReviewBuffer::new();
            let result = {
                let mut emit_content = |event: StreamEvent| -> Result<(), Error> {
                    let StreamEvent::Content(content) = event;
                    if !content.is_empty() {
                        if let Some(active) = spinner.take() {
                            active.finish();
                        }
                        buffer.receive(&content);
                    }
                    Ok(())
                };
                provider.chat_completions_streaming(messages, options, &mut emit_content)
            };
            (result, buffer)
        });
        loop {
            if handle.is_finished() {
                break handle.join().expect("stream worker panicked");
            }
            if interrupt.load(Ordering::SeqCst) {
                let deadline = Instant::now() + CANCEL_GRACE;
                while !handle.is_finished() && Instant::now() < deadline {
                    thread::sleep(Duration::from_millis(10));
                }
                if !handle.is_finished() {
                    std::process::exit(130);
                }
                break handle.join().expect("stream worker panicked");
            }
            thread::sleep(CANCEL_POLL);
        }
    });
    if let Some(active) = spinner.take() {
        active.finish();
    }
    result.map(|response| Generation { response, buffer })
}

/// Parse the buffered provider payload into a reviewable candidate.
pub fn parse_generated_candidate(generation: &Generation) -> Option<ReviewCandidate> {
    let mut buffer = generation.buffer.clone();
    buffer.complete();
    candidate_from_provider_response(buffer.candidate()?)
}

/// Fetch an explanation for a developer-supplied command and report the
/// provider outcome. A request failure is returned to the caller instead of
/// being swallowed; the caller keeps the developer's command reviewable.
pub fn explain_command_candidate(
    provider: &dyn Provider,
    command: &str,
    messages: &[Message],
    options: &RequestOptions,
    interrupt: &Arc<AtomicBool>,
    spinner: Option<Spinner>,
) -> Result<ExplanationOutcome, Error> {
    let generation = generate_candidate(provider, messages, options, interrupt, spinner)?;
    let mut buffer = generation.buffer.clone();
    buffer.complete();
    Ok(apply_explanation_outcome(
        command,
        buffer.candidate().unwrap_or_default(),
    ))
}

/// The configured model tiers in small/normal/thinking order.
pub fn chooser_tiers(config: &Config) -> Vec<TierChoice> {
    let candidates = [
        ("1", "small", config.tiers.small.as_ref()),
        ("2", "normal", config.tiers.normal.as_ref()),
        ("3", "thinking", config.tiers.thinking.as_ref()),
    ];
    candidates
        .into_iter()
        .filter_map(|(tier, label, model)| {
            model.map(|model| TierChoice {
                tier: tier.to_string(),
                label: label.to_string(),
                model: model.clone(),
            })
        })
        .collect()
}

/// Provider catalog model ids. Uses the provider-local catalog endpoint when
/// configured; an unavailable catalog degrades to an empty list.
pub fn fetch_catalog(
    endpoint: &str,
    catalog_endpoint: Option<&str>,
    api_key: Option<&str>,
) -> Vec<String> {
    let source = catalog_endpoint
        .filter(|value| !value.is_empty())
        .unwrap_or(endpoint);
    fetch_models(source, api_key)
        .map(|entries| entries.into_iter().map(|entry| entry.id).collect())
        .unwrap_or_default()
}
