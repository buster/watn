mod adapter;
mod buffer;
mod flow;
mod panel;
mod response;

pub use adapter::{PresentationAdapter, PresentationSelection};
pub use buffer::ReviewBuffer;
pub use flow::{derive_command_flow, CommandFlow, CommandStage, StageSupport, UnsupportedSpan};
pub use panel::{
    controlling_terminal_is_usable, render_lines, sanitize_terminal_text, ControllingTerminal,
    FocusRegion, InlineLayout, InlineReviewPanel, PanelAction, PanelInputMode, PanelOutcome,
    ReviewContext, ReviewPanelState,
};
pub use response::{
    parse_structured_review_response, CandidateIdentity, PurposeStatus, ReviewCandidate,
    ReviewParseResult, ReviewResponse, ReviewResponseError, ReviewStage, REVIEW_VERSION,
};
