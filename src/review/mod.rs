mod buffer;
mod card;
mod flow;
mod panel;
mod response;
mod routing;
pub mod session;

pub use buffer::ReviewBuffer;
pub use card::{
    color_terminal_supports_card, disable_hint, render_card_lines, terminal_supports_color,
};
pub use flow::{derive_command_flow, CommandFlow, CommandStage, StageSupport, UnsupportedSpan};
pub use panel::{
    controlling_terminal_is_usable, explanation_terminal_is_usable, sanitize_terminal_text,
    ControllingTerminal, InlineLayout, InlineReviewPanel, ModelChooser, PanelInputMode,
    PanelOutcome, ReviewContext, ReviewOperation, ReviewPanelState, TierChoice,
};
pub use response::{
    apply_explanation, apply_explanation_outcome, apply_explanation_outcome_with_finish,
    candidate_from_provider_response, candidate_from_provider_response_with_finish,
    parse_structured_review_response, CandidateIdentity, ExplanationOutcome, PurposeStatus,
    ReviewCandidate, ReviewParseResult, ReviewResponse, ReviewResponseError, ReviewStage,
    REVIEW_VERSION,
};
pub use routing::{request_route, resolve_review_enabled, RequestRoute};
