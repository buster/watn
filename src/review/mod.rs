mod buffer;
mod card;
mod flow;
mod panel;
mod response;
mod routing;
pub mod session;

pub use buffer::ReviewBuffer;
pub use card::{color_terminal_supports_card, render_card_lines, terminal_supports_color};
pub use flow::{derive_command_flow, CommandFlow, CommandStage, StageSupport, UnsupportedSpan};
pub use panel::{
    controlling_terminal_is_usable, sanitize_terminal_text, ControllingTerminal, InlineLayout,
    InlineReviewPanel, ModelChooser, PanelInputMode, PanelOutcome, ReviewContext, ReviewOperation,
    ReviewPanelState, TierChoice,
};
pub use response::{
    candidate_from_provider_response, parse_structured_review_response, CandidateIdentity,
    PurposeStatus, ReviewCandidate, ReviewParseResult, ReviewResponse, ReviewResponseError,
    ReviewStage, REVIEW_VERSION,
};
pub use routing::{request_route, resolve_review_enabled, RequestRoute};
