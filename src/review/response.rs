use serde::{Deserialize, Serialize};

use super::flow::{command_stage, derive_command_flow, CommandFlow};

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

/// Locate the structured JSON object inside a provider payload. Providers may
/// wrap the object in a markdown code fence, possibly with surrounding prose.
fn locate_json_payload(raw: &str) -> Option<&str> {
    let trimmed = raw.trim();
    if let Some(fence_start) = trimmed.find("```") {
        let after_fence = &trimmed[fence_start + 3..];
        if let Some(line_end) = after_fence.find('\n') {
            let body = &after_fence[line_end + 1..];
            if let Some(fence_end) = body.find("```") {
                let fenced = body[..fence_end].trim();
                if !fenced.is_empty() {
                    return Some(fenced);
                }
            }
        }
    }
    let start = trimmed.find('{')?;
    let end = trimmed.rfind('}')?;
    (end > start).then(|| trimmed[start..=end].trim())
}

/// Parse the structured response object. Markdown fences and surrounding prose
/// are tolerated; command-only provider output is deliberately not a parseable
/// review response.
pub fn parse_structured_review_response(raw: &str) -> Result<ReviewResponse, ReviewResponseError> {
    let payload = locate_json_payload(raw).unwrap_or_else(|| raw.trim());
    serde_json::from_str(payload)
        .map_err(|error| ReviewResponseError::InvalidJson(error.to_string()))
}

/// Normalize a provider-written command to a single line. Line breaks and tabs
/// become spaces; no other character changes.
fn normalize_command(command: &str) -> String {
    command
        .chars()
        .map(|character| match character {
            '\n' | '\r' | '\t' => ' ',
            other => other,
        })
        .collect::<String>()
        .trim()
        .to_string()
}

/// Recover a provider stage split that provably covers the command. Every
/// stage text must appear verbatim in the command, in order, without overlap;
/// gaps may contain only whitespace or shell separators; the remainder after
/// the last stage must be whitespace. Every stage needs a non-empty purpose.
fn provider_stage_split(
    command: &str,
    value: &serde_json::Value,
) -> Option<(CommandFlow, Vec<ReviewStage>)> {
    let stages = value.get("stages")?.as_array()?;
    if stages.is_empty() {
        return None;
    }
    let mut cursor = 0usize;
    let mut flow_stages = Vec::with_capacity(stages.len());
    let mut review_stages = Vec::with_capacity(stages.len());
    for stage in stages {
        let stage_text = stage.get("stage_text")?.as_str()?.trim();
        if stage_text.is_empty() {
            return None;
        }
        let purpose = stage.get("purpose")?.as_str()?.trim();
        if purpose.is_empty() {
            return None;
        }
        let start = cursor + command[cursor..].find(stage_text)?;
        let gap = &command[cursor..start];
        if !gap
            .chars()
            .all(|character| character.is_whitespace() || matches!(character, '|' | '&' | ';'))
        {
            return None;
        }
        let end = start + stage_text.len();
        flow_stages.push(command_stage(command, start, end));
        review_stages.push(ReviewStage {
            stage_text: stage_text.to_string(),
            purpose: Some(purpose.to_string()),
        });
        cursor = end;
    }
    if !command[cursor..].chars().all(char::is_whitespace) {
        return None;
    }
    Some((
        CommandFlow {
            stages: flow_stages,
        },
        review_stages,
    ))
}

/// Build a reviewable candidate from a provider payload.
///
/// - A valid structured response keeps its command and model-written purposes.
/// - A non-canonical response keeps model-written purposes when their stage
///   text matches the derived stages and every purpose is non-empty.
/// - A JSON-shaped payload that fails strict validation contributes only its
///   provider-written command; purposes stay unavailable.
/// - A non-JSON payload is treated as command text.
/// - `None` means no usable command exists: the review is `Unavailable`.
pub fn candidate_from_provider_response(raw: &str) -> Option<ReviewCandidate> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return None;
    }
    if let Ok(parsed) = parse_structured_review_response(trimmed) {
        let command = normalize_command(&parsed.command);
        let mut candidate = ReviewCandidate::from_command(&command);
        if matches!(
            candidate.apply_response(trimmed),
            ReviewParseResult::Ready | ReviewParseResult::Loading
        ) {
            return Some(candidate);
        }
    }
    if let Some(payload) = locate_json_payload(trimmed) {
        if let Ok(value) = serde_json::from_str::<serde_json::Value>(payload) {
            if value.get("review_version").is_some()
                || value.get("command").is_some()
                || value.get("purpose_status").is_some()
            {
                let command = value
                    .get("command")
                    .and_then(|command| command.as_str())
                    .map(normalize_command)
                    .filter(|command| !command.is_empty())?;
                if let Some((flow, stages)) = provider_stage_split(&command, &value) {
                    let mut candidate = ReviewCandidate::from_command(&command);
                    candidate.flow = flow;
                    candidate.stages = stages;
                    candidate.purpose_status = PurposeStatus::Ready;
                    return Some(candidate);
                }
                return Some(ReviewCandidate::from_command(command));
            }
        }
    }
    Some(ReviewCandidate::from_command(trimmed))
}

/// The result of applying a provider explanation to a developer-supplied
/// command. `Ready` carries the command with model-written purposes;
/// `Unusable` carries the command with local stages and the reason the
/// provider response could not be trusted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExplanationOutcome {
    Ready(ReviewCandidate),
    Unusable {
        candidate: ReviewCandidate,
        reason: ReviewResponseError,
    },
}

/// Apply a provider explanation to a developer-supplied command and report
/// whether the response was usable. The developer's command is authoritative
/// and is never replaced. Model purposes are adopted only when the response
/// echoes the command with the locally derived stages, or when its stage
/// split provably covers the command.
pub fn apply_explanation_outcome(command: &str, raw: &str) -> ExplanationOutcome {
    let mut candidate = ReviewCandidate::from_command(command);
    let reason = match candidate.apply_response(raw) {
        ReviewParseResult::Ready => return ExplanationOutcome::Ready(candidate),
        ReviewParseResult::PurposeUnavailable(error) => error,
        ReviewParseResult::Loading => ReviewResponseError::InvalidLoadingResponse,
    };
    candidate.mark_unavailable();
    if let Some(payload) = locate_json_payload(raw) {
        if let Ok(value) = serde_json::from_str::<serde_json::Value>(payload) {
            if let Some((flow, stages)) = provider_stage_split(&candidate.command, &value) {
                candidate.flow = flow;
                candidate.stages = stages;
                candidate.purpose_status = PurposeStatus::Ready;
                return ExplanationOutcome::Ready(candidate);
            }
        }
    }
    ExplanationOutcome::Unusable { candidate, reason }
}

/// Apply a provider explanation to a developer-supplied command. The
/// developer's command is authoritative and is never replaced. Model purposes
/// are adopted only when the response echoes the command with the locally
/// derived stages, or when its stage split provably covers the command.
pub fn apply_explanation(command: &str, raw: &str) -> ReviewCandidate {
    match apply_explanation_outcome(command, raw) {
        ExplanationOutcome::Ready(candidate) | ExplanationOutcome::Unusable { candidate, .. } => {
            candidate
        }
    }
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
        apply_explanation_outcome, candidate_from_provider_response,
        parse_structured_review_response, ExplanationOutcome, PurposeStatus, ReviewCandidate,
        ReviewParseResult, ReviewResponseError, REVIEW_VERSION,
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
    fn explanation_outcome_discriminates_ready_and_unusable() {
        let command = "git log --oneline | head -5";
        let ready = format!(
            "{{\"review_version\":1,\"command\":{command},\"stages\":[{{\"stage_text\":\"git log --oneline\",\"purpose\":\"List commits.\"}},{{\"stage_text\":\"head -5\",\"purpose\":\"Keep five.\"}}],\"purpose_status\":\"ready\"}}",
            command = serde_json::to_string(command).unwrap()
        );
        assert!(matches!(
            apply_explanation_outcome(command, &ready),
            ExplanationOutcome::Ready(_)
        ));

        let unusable = r#"{"review_version":1,"command":"git log --oneline","stages":[{"stage_text":"unrelated stage","purpose":"x"}],"purpose_status":"ready"}"#;
        match apply_explanation_outcome(command, unusable) {
            ExplanationOutcome::Unusable { candidate, reason } => {
                assert_eq!(candidate.command, command);
                assert_eq!(candidate.purpose_status, PurposeStatus::Unavailable);
                assert_eq!(reason, ReviewResponseError::CommandMismatch);
            }
            ExplanationOutcome::Ready(_) => {
                panic!("a mismatched response must not be trusted")
            }
        }
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

    #[test]
    fn markdown_fenced_and_prose_wrapped_responses_are_recognized() {
        let command = "git log --oneline | head -5";
        let response = format!(
            "Here is the review:\n```json\n{{\"review_version\":1,\"command\":{command},\"stages\":[{{\"stage_text\":\"git log --oneline\",\"purpose\":\"List commits.\"}},{{\"stage_text\":\"head -5\",\"purpose\":\"Keep five.\"}}],\"purpose_status\":\"ready\"}}\n```\n",
            command = serde_json::to_string(command).unwrap()
        );

        let parsed = parse_structured_review_response(&response).unwrap();
        assert_eq!(parsed.command, command);

        let candidate = candidate_from_provider_response(&response).unwrap();
        assert_eq!(candidate.command, command);
        assert_eq!(candidate.purpose_status, PurposeStatus::Ready);
        assert_eq!(candidate.stage_purposes().count(), 2);
    }

    #[test]
    fn invalid_status_with_matching_purposes_keeps_the_provider_purpose() {
        let raw = r#"{"review_version":1,"command":"df -h","stages":[{"stage_text":"df -h","purpose":"Show disks."}],"purpose_status":"incomplete"}"#;
        assert!(parse_structured_review_response(raw).is_err());

        let candidate = candidate_from_provider_response(raw).unwrap();
        assert_eq!(candidate.command, "df -h");
        assert_eq!(candidate.purpose_status, PurposeStatus::Ready);
        assert_eq!(
            candidate.stage_purposes().collect::<Vec<_>>(),
            vec![Some("Show disks.")]
        );
    }

    #[test]
    fn command_text_that_merely_contains_braces_stays_a_command() {
        let raw = "awk '{print $1}' file.txt";
        let candidate = candidate_from_provider_response(raw).unwrap();
        assert_eq!(candidate.command, "awk '{print $1}' file.txt");
    }

    #[test]
    fn structured_payloads_without_a_usable_command_are_unavailable() {
        let raw = r#"{"review_version":1,"stages":[],"purpose_status":"ready"}"#;
        assert!(candidate_from_provider_response(raw).is_none());

        let empty = r#"{"review_version":1,"command":"   ","stages":[],"purpose_status":"ready"}"#;
        assert!(candidate_from_provider_response(empty).is_none());
    }

    #[test]
    fn unknown_status_with_matching_purposes_keeps_provider_purposes() {
        let raw = r#"{"review_version":1,"command":"git rev-list --all | xargs -n1 git ls-tree -r | head -5","stages":[{"stage_text":"git rev-list --all","purpose":"List every commit."},{"stage_text":"xargs -n1","purpose":"Pass every commit to the file listing."},{"stage_text":"git ls-tree -r","purpose":"List the files in each commit."},{"stage_text":"head -5","purpose":"Keep the first five files."}],"purpose_status":"incomplete"}"#;
        let candidate = candidate_from_provider_response(raw).unwrap();
        assert_eq!(candidate.purpose_status, PurposeStatus::Ready);
        assert_eq!(
            candidate.stage_purposes().collect::<Vec<_>>(),
            vec![
                Some("List every commit."),
                Some("Pass every commit to the file listing."),
                Some("List the files in each commit."),
                Some("Keep the first five files."),
            ]
        );
    }

    #[test]
    fn line_broken_commands_are_normalized_before_flow_derivation() {
        let raw = serde_json::json!({
            "review_version": 1,
            "command": "git rev-list --all |\nxargs -n1 git ls-tree -r |\nhead -5",
            "stages": [
                {"stage_text": "git rev-list --all", "purpose": "List every commit."},
                {"stage_text": "xargs -n1", "purpose": "Pass every commit to the file listing."},
                {"stage_text": "git ls-tree -r", "purpose": "List the files in each commit."},
                {"stage_text": "head -5", "purpose": "Keep the first five files."}
            ],
            "purpose_status": "ready"
        })
        .to_string();

        let candidate = candidate_from_provider_response(&raw).unwrap();
        assert_eq!(
            candidate.command,
            "git rev-list --all | xargs -n1 git ls-tree -r | head -5"
        );
        assert!(!candidate.command.contains('\n'));
        assert_eq!(candidate.purpose_status, PurposeStatus::Ready);
        assert_eq!(candidate.stage_purposes().count(), 4);
    }

    #[test]
    fn mismatched_or_missing_stage_purposes_fall_back_to_unavailable() {
        let mismatched = r#"{"review_version":1,"command":"df -h","stages":[{"stage_text":"wrong","purpose":"x"}],"purpose_status":"incomplete"}"#;
        let candidate = candidate_from_provider_response(mismatched).unwrap();
        assert_eq!(candidate.command, "df -h");
        assert_eq!(candidate.purpose_status, PurposeStatus::Unavailable);
        assert!(candidate.stage_purposes().all(|purpose| purpose.is_none()));

        let missing = r#"{"review_version":1,"command":"df -h","stages":[{"stage_text":"df -h"}],"purpose_status":"incomplete"}"#;
        let candidate = candidate_from_provider_response(missing).unwrap();
        assert_eq!(candidate.command, "df -h");
        assert_eq!(candidate.purpose_status, PurposeStatus::Unavailable);
        assert!(candidate.stage_purposes().all(|purpose| purpose.is_none()));
    }

    #[test]
    fn provider_splits_that_cover_the_command_are_accepted() {
        let raw = serde_json::json!({
            "review_version": 1,
            "command": "git rev-list --all | while read commit; do git ls-tree -r $commit | awk '{print $4, $3}'; done | sort | uniq | sort -k2 -rn | head -5",
            "stages": [
                {"stage_text": "git rev-list --all", "purpose": "List all commit hashes in the git repository"},
                {"stage_text": "while read commit; do git ls-tree -r $commit | awk '{print $4, $3}'; done", "purpose": "For each commit, recursively list all files with their object hashes and extract filename and object hash"},
                {"stage_text": "sort | uniq", "purpose": "Sort the file entries and remove duplicates"},
                {"stage_text": "sort -k2 -rn", "purpose": "Sort by file size (second column) in descending numerical order"},
                {"stage_text": "head -5", "purpose": "Display only the top 5 largest files"}
            ],
            "purpose_status": "ready"
        })
        .to_string();

        let candidate = candidate_from_provider_response(&raw).unwrap();
        assert_eq!(candidate.purpose_status, PurposeStatus::Ready);
        assert_eq!(
            candidate.flow.stage_texts().collect::<Vec<_>>(),
            vec![
                "git rev-list --all",
                "while read commit; do git ls-tree -r $commit | awk '{print $4, $3}'; done",
                "sort | uniq",
                "sort -k2 -rn",
                "head -5",
            ]
        );
        assert_eq!(candidate.stage_purposes().count(), 5);
    }

    #[test]
    fn incomplete_provider_splits_are_not_trusted() {
        let empty_stages =
            r#"{"review_version":1,"command":"df -h","stages":[],"purpose_status":"ready"}"#;
        let candidate = candidate_from_provider_response(empty_stages).unwrap();
        assert_eq!(candidate.purpose_status, PurposeStatus::Unavailable);

        let empty_text = r#"{"review_version":1,"command":"df -h","stages":[{"stage_text":"   ","purpose":"x"}],"purpose_status":"ready"}"#;
        let candidate = candidate_from_provider_response(empty_text).unwrap();
        assert_eq!(candidate.purpose_status, PurposeStatus::Unavailable);

        let blank_purpose = r#"{"review_version":1,"command":"df -h","stages":[{"stage_text":"df -h","purpose":"   "}],"purpose_status":"ready"}"#;
        let candidate = candidate_from_provider_response(blank_purpose).unwrap();
        assert_eq!(candidate.purpose_status, PurposeStatus::Unavailable);
    }

    #[test]
    fn untrusted_provider_splits_fall_back_to_unavailable() {
        let fabricated = r#"{"review_version":1,"command":"df -h","stages":[{"stage_text":"not part of the command","purpose":"x"}],"purpose_status":"ready"}"#;
        let candidate = candidate_from_provider_response(fabricated).unwrap();
        assert_eq!(candidate.command, "df -h");
        assert_eq!(candidate.purpose_status, PurposeStatus::Unavailable);

        let gap_with_text = r#"{"review_version":1,"command":"df -h","stages":[{"stage_text":"-h","purpose":"x"}],"purpose_status":"ready"}"#;
        let candidate = candidate_from_provider_response(gap_with_text).unwrap();
        assert_eq!(candidate.purpose_status, PurposeStatus::Unavailable);

        let trailing_text = r#"{"review_version":1,"command":"df -h","stages":[{"stage_text":"df","purpose":"x"}],"purpose_status":"ready"}"#;
        let candidate = candidate_from_provider_response(trailing_text).unwrap();
        assert_eq!(candidate.purpose_status, PurposeStatus::Unavailable);
    }
}
