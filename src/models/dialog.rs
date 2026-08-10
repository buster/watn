use super::list::ModelEntry;

/// Reasoning strengths persisted for each model tier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReasoningStrength {
    Off,
    Low,
    Minimal,
    Medium,
    High,
}

impl ReasoningStrength {
    pub const ALL: [ReasoningStrength; 5] = [
        ReasoningStrength::Off,
        ReasoningStrength::Low,
        ReasoningStrength::Minimal,
        ReasoningStrength::Medium,
        ReasoningStrength::High,
    ];

    pub fn as_str(&self) -> &'static str {
        match self {
            ReasoningStrength::Off => "off",
            ReasoningStrength::Low => "low",
            ReasoningStrength::Minimal => "minimal",
            ReasoningStrength::Medium => "medium",
            ReasoningStrength::High => "high",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        Self::ALL.iter().copied().find(|strength| strength.as_str() == value)
    }
}

/// A confirmed model and its selected reasoning strength.
#[derive(Debug, Clone)]
pub struct LevelChoice {
    pub model: ModelEntry,
    pub reasoning: ReasoningStrength,
}
