use serde::{Deserialize, Serialize};

use super::flow::{derive_command_flow, CommandFlow};

pub const REVIEW_VERSION: u32 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PurposeStatus {
    Ready,
    Loading,
    #[serde(rename = "purpose-unavailable")]
    Unavailable,
}

impl PurposeStatus {
    pub fn label(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::Loading => "loading",
            Self::Unavailable => "purpose-unavailable",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewStage {
    pub stage_text: String,
    #[serde(default)]
    pub purpose: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewResponse {
    pub review_version: u32,
    pub command: String,
    #[serde(default)]
    pub stages: Vec<ReviewStage>,
    pub purpose_status: PurposeStatus,
    #[serde(default)]
    pub purpose_request: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReviewResponseError {
    InvalidJson(String),
    UnsupportedVersion(u32),
    EmptyCommand,
    CommandMismatch,
    StageMismatch,
    MissingPurpose,
    InvalidLoadingResponse,
}

impl std::fmt::Display for ReviewResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidJson(error) => write!(f, "invalid review JSON: {error}"),
            Self::UnsupportedVersion(version) => {
                write!(f, "unsupported review response version: {version}")
            }
            Self::EmptyCommand => write!(f, "review response command is empty"),
            Self::CommandMismatch => write!(f, "review response command does not match candidate"),
            Self::StageMismatch => {
                write!(f, "review response stage text does not match command flow")
            }
            Self::MissingPurpose => write!(f, "review response is missing a stage purpose"),
            Self::InvalidLoadingResponse => {
                write!(f, "loading review response has no delayed-purpose request")
            }
        }
    }
}

impl std::error::Error for ReviewResponseError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CandidateIdentity {
    pub generation: u64,
    pub review_version: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReviewParseResult {
    Ready,
    Loading,
    PurposeUnavailable(ReviewResponseError),
}

/// Parse only the structured response object. Command-only provider output is
/// deliberately not treated as a parseable review response.
pub fn parse_structured_review_response(raw: &str) -> Result<ReviewResponse, ReviewResponseError> {
    serde_json::from_str(raw.trim())
        .map_err(|error| ReviewResponseError::InvalidJson(error.to_string()))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewCandidate {
    pub command: String,
    pub flow: CommandFlow,
    pub stages: Vec<ReviewStage>,
    pub purpose_status: PurposeStatus,
    pub generation: u64,
    pub review_version: Option<u32>,
}

impl ReviewCandidate {
    pub fn from_command(command: impl Into<String>) -> Self {
        let command = command.into();
        let flow = derive_command_flow(&command);
        let stages = unavailable_stages(&flow);
        Self {
            command,
            flow,
            stages,
            purpose_status: PurposeStatus::Unavailable,
            generation: 0,
            review_version: None,
        }
    }

    pub fn identity(&self) -> CandidateIdentity {
        CandidateIdentity {
            generation: self.generation,
            review_version: self.review_version.unwrap_or(REVIEW_VERSION),
        }
    }

    pub fn stage_purposes(&self) -> impl Iterator<Item = Option<&str>> {
        self.stages.iter().map(|stage| stage.purpose.as_deref())
    }

    pub fn apply_response(&mut self, raw: &str) -> ReviewParseResult {
        self.apply_response_for(self.generation, raw)
            .expect("the current candidate generation always matches")
    }

    /// Apply a response only to the candidate generation that requested it.
    /// A stale response is ignored and cannot replace newer purposes.
    pub fn apply_response_for(&mut self, generation: u64, raw: &str) -> Option<ReviewParseResult> {
        if generation != self.generation {
            return None;
        }

        let response = match parse_structured_review_response(raw) {
            Ok(response) => response,
            Err(error) => {
                self.mark_unavailable();
                return Some(ReviewParseResult::PurposeUnavailable(error));
            }
        };
        if let Err(error) = self.validate(&response) {
            self.mark_unavailable();
            return Some(ReviewParseResult::PurposeUnavailable(error));
        }

        self.review_version = Some(response.review_version);
        self.stages = response.stages;
        self.purpose_status = response.purpose_status;
        Some(match response.purpose_status {
            PurposeStatus::Ready => ReviewParseResult::Ready,
            PurposeStatus::Loading => ReviewParseResult::Loading,
            PurposeStatus::Unavailable => {
                ReviewParseResult::PurposeUnavailable(ReviewResponseError::InvalidLoadingResponse)
            }
        })
    }

    pub fn edit_command(&mut self, command: impl Into<String>) {
        self.command = command.into();
        self.flow = derive_command_flow(&self.command);
        self.stages = unavailable_stages(&self.flow);
        self.purpose_status = PurposeStatus::Unavailable;
        self.review_version = None;
        self.generation = self.generation.saturating_add(1);
    }

    fn validate(&self, response: &ReviewResponse) -> Result<(), ReviewResponseError> {
        if response.review_version != REVIEW_VERSION {
            return Err(ReviewResponseError::UnsupportedVersion(
                response.review_version,
            ));
        }
        if response.command.trim().is_empty() {
            return Err(ReviewResponseError::EmptyCommand);
        }
        if response.command != self.command {
            return Err(ReviewResponseError::CommandMismatch);
        }
        let expected = self.flow.stage_texts().collect::<Vec<_>>();
        let actual = response
            .stages
            .iter()
            .map(|stage| stage.stage_text.as_str())
            .collect::<Vec<_>>();
        if expected != actual {
            return Err(ReviewResponseError::StageMismatch);
        }
        match response.purpose_status {
            PurposeStatus::Ready
                if response.stages.iter().any(|stage| {
                    stage
                        .purpose
                        .as_deref()
                        .is_none_or(|purpose| purpose.trim().is_empty())
                }) =>
            {
                Err(ReviewResponseError::MissingPurpose)
            }
            PurposeStatus::Loading
                if response
                    .purpose_request
                    .as_deref()
                    .is_none_or(|request| request.trim().is_empty()) =>
            {
                Err(ReviewResponseError::InvalidLoadingResponse)
            }
            PurposeStatus::Ready | PurposeStatus::Loading | PurposeStatus::Unavailable => Ok(()),
        }
    }

    fn mark_unavailable(&mut self) {
        self.purpose_status = PurposeStatus::Unavailable;
        self.stages = unavailable_stages(&self.flow);
    }
}

fn unavailable_stages(flow: &CommandFlow) -> Vec<ReviewStage> {
    flow.stage_texts()
        .map(|stage_text| ReviewStage {
            stage_text: stage_text.to_string(),
            purpose: None,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{
        parse_structured_review_response, PurposeStatus, ReviewCandidate, ReviewParseResult,
        ReviewResponseError, REVIEW_VERSION,
    };

    const COMMAND: &str = "git log --since='7 days ago' | xargs -n1 git show --stat";

    #[test]
    fn parses_ready_response_and_requires_exact_locally_derived_stages() {
        let raw = format!(
            "{{\"review_version\":1,\"command\":{command},\"stages\":[{{\"stage_text\":\"git log --since='7 days ago'\",\"purpose\":\"Collect commits.\"}},{{\"stage_text\":\"xargs -n1\",\"purpose\":\"Pass each commit.\"}},{{\"stage_text\":\"git show --stat\",\"purpose\":\"Inspect each commit.\"}}],\"purpose_status\":\"ready\"}}",
            command = serde_json::to_string(COMMAND).unwrap()
        );
        let response = parse_structured_review_response(&raw).unwrap();
        assert_eq!(response.purpose_status, PurposeStatus::Ready);
        assert_eq!(response.stages.len(), 3);

        let mut candidate = ReviewCandidate::from_command(COMMAND);
        assert_eq!(candidate.apply_response(&raw), ReviewParseResult::Ready);
        assert_eq!(candidate.stage_purposes().count(), 3);
    }

    #[test]
    fn command_only_and_malformed_responses_fall_back_without_local_purpose_text() {
        let mut candidate = ReviewCandidate::from_command("printf 'done'");
        let result = candidate.apply_response("printf 'done'");
        assert!(matches!(
            result,
            ReviewParseResult::PurposeUnavailable(ReviewResponseError::InvalidJson(_))
        ));
        assert_eq!(candidate.purpose_status, PurposeStatus::Unavailable);
        assert!(candidate.stage_purposes().all(|purpose| purpose.is_none()));
    }

    #[test]
    fn delayed_purposes_are_loading_only_with_a_request_and_stale_results_are_ignored() {
        let mut candidate = ReviewCandidate::from_command(COMMAND);
        let raw = format!(
            "{{\"review_version\":1,\"command\":{command},\"stages\":[{{\"stage_text\":\"git log --since='7 days ago'\"}},{{\"stage_text\":\"xargs -n1\"}},{{\"stage_text\":\"git show --stat\"}}],\"purpose_status\":\"loading\",\"purpose_request\":\"opaque\"}}",
            command = serde_json::to_string(COMMAND).unwrap()
        );
        assert_eq!(candidate.apply_response(&raw), ReviewParseResult::Loading);
        let generation = candidate.generation;
        candidate.edit_command("printf 'new'");
        assert!(candidate.apply_response_for(generation, &raw).is_none());
        assert_eq!(candidate.command, "printf 'new'");
    }

    #[test]
    fn validation_rejections_map_to_explicit_purpose_unavailable_errors() {
        let mut candidate = ReviewCandidate::from_command("df -h");

        let unsupported = r#"{"review_version":2,"command":"df -h","stages":[],"purpose_status":"purpose-unavailable"}"#;
        assert!(matches!(
            candidate.apply_response(unsupported),
            ReviewParseResult::PurposeUnavailable(ReviewResponseError::UnsupportedVersion(2))
        ));

        let empty = r#"{"review_version":1,"command":"","stages":[],"purpose_status":"purpose-unavailable"}"#;
        assert!(matches!(
            candidate.apply_response(empty),
            ReviewParseResult::PurposeUnavailable(ReviewResponseError::EmptyCommand)
        ));

        let mismatch = r#"{"review_version":1,"command":"ls","stages":[],"purpose_status":"purpose-unavailable"}"#;
        assert!(matches!(
            candidate.apply_response(mismatch),
            ReviewParseResult::PurposeUnavailable(ReviewResponseError::CommandMismatch)
        ));

        let stage_mismatch = r#"{"review_version":1,"command":"df -h","stages":[{"stage_text":"wrong"}],"purpose_status":"ready"}"#;
        assert!(matches!(
            candidate.apply_response(stage_mismatch),
            ReviewParseResult::PurposeUnavailable(ReviewResponseError::StageMismatch)
        ));

        let missing_purpose = r#"{"review_version":1,"command":"df -h","stages":[{"stage_text":"df -h"}],"purpose_status":"ready"}"#;
        assert!(matches!(
            candidate.apply_response(missing_purpose),
            ReviewParseResult::PurposeUnavailable(ReviewResponseError::MissingPurpose)
        ));

        let loading_without_request = r#"{"review_version":1,"command":"df -h","stages":[{"stage_text":"df -h"}],"purpose_status":"loading"}"#;
        assert!(matches!(
            candidate.apply_response(loading_without_request),
            ReviewParseResult::PurposeUnavailable(ReviewResponseError::InvalidLoadingResponse)
        ));
    }

    #[test]
    fn identity_stage_purposes_and_error_display_are_exposed() {
        let candidate = ReviewCandidate::from_command("df -h");
        let identity = candidate.identity();
        assert_eq!(identity.generation, 0);
        assert_eq!(identity.review_version, REVIEW_VERSION);
        assert_eq!(candidate.stage_purposes().count(), 1);

        for error in [
            ReviewResponseError::InvalidJson("broken".to_string()),
            ReviewResponseError::UnsupportedVersion(9),
            ReviewResponseError::EmptyCommand,
            ReviewResponseError::CommandMismatch,
            ReviewResponseError::StageMismatch,
            ReviewResponseError::MissingPurpose,
            ReviewResponseError::InvalidLoadingResponse,
        ] {
            assert!(!error.to_string().is_empty());
        }
    }

    #[test]
    fn purpose_labels_and_valid_unavailable_responses_are_covered() {
        assert_eq!(PurposeStatus::Ready.label(), "ready");
        assert_eq!(PurposeStatus::Loading.label(), "loading");
        assert_eq!(PurposeStatus::Unavailable.label(), "purpose-unavailable");

        let mut candidate = ReviewCandidate::from_command("df -h");
        let raw = r#"{"review_version":1,"command":"df -h","stages":[{"stage_text":"df -h"}],"purpose_status":"purpose-unavailable"}"#;
        assert!(matches!(
            candidate.apply_response(raw),
            ReviewParseResult::PurposeUnavailable(ReviewResponseError::InvalidLoadingResponse)
        ));
        assert_eq!(candidate.purpose_status, PurposeStatus::Unavailable);
    }
}
