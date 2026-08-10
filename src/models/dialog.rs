use super::list::ModelEntry;
use super::list::ModelReasoning;
use crate::error::Error;

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
        Self::ALL
            .iter()
            .copied()
            .find(|strength| strength.as_str() == value)
    }
}

pub fn resolve_reasoning_default(
    metadata: &ModelReasoning,
    existing: Option<ReasoningStrength>,
) -> Result<ReasoningStrength, Error> {
    if !metadata.mandatory && !metadata.default_enabled {
        return Ok(ReasoningStrength::Off);
    }
    let supported = metadata
        .supported_efforts
        .iter()
        .filter_map(|effort| ReasoningStrength::parse(effort))
        .filter(|effort| !metadata.mandatory || *effort != ReasoningStrength::Off)
        .collect::<Vec<_>>();
    if let Some(default) = metadata
        .default_effort
        .as_deref()
        .and_then(ReasoningStrength::parse)
        .filter(|effort| !metadata.mandatory || *effort != ReasoningStrength::Off)
        .filter(|effort| supported.contains(effort))
    {
        return Ok(default);
    }
    if let Some(first) = supported.first() {
        return Ok(*first);
    }
    if metadata.mandatory {
        if let Some(existing) = existing.filter(|effort| *effort != ReasoningStrength::Off) {
            return Ok(existing);
        }
        return Err(Error::ConfigError(
            "reasoning policy has no usable effort".to_string(),
        ));
    }
    Ok(existing.unwrap_or(ReasoningStrength::Off))
}

/// A confirmed model and its selected reasoning strength.
#[derive(Debug, Clone)]
pub struct LevelChoice {
    pub model: ModelEntry,
    pub reasoning: ReasoningStrength,
}
