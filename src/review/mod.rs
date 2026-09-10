mod adapter;
mod buffer;
mod flow;
mod panel;
mod response;
mod routing;

pub use adapter::{PresentationAdapter, PresentationSelection};
pub use buffer::ReviewBuffer;
pub use flow::{derive_command_flow, CommandFlow, CommandStage, StageSupport, UnsupportedSpan};
pub use panel::{
    controlling_terminal_is_usable, render_lines, sanitize_terminal_text, ControllingTerminal,
    FocusRegion, InlineLayout, InlineReviewPanel, PanelAction, PanelInputMode, PanelOutcome,
    ReviewContext, ReviewOperation, ReviewPanelState,
};
pub use response::{
    candidate_from_provider_response, parse_structured_review_response, CandidateIdentity,
    PurposeStatus, ReviewCandidate, ReviewParseResult, ReviewResponse, ReviewResponseError,
    ReviewStage, REVIEW_VERSION,
};
pub use routing::{request_route, resolve_review_enabled, RequestRoute};
