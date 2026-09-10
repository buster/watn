use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn comment_toml(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            if line.is_empty() {
                String::new()
            } else {
                format!("# {}", line)
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Config {
    #[serde(default)]
    pub defaults: ProviderDefaults,
    #[serde(default)]
    pub providers: HashMap<String, ProviderConfig>,
    #[serde(default)]
    pub tiers: ModelTiers,
    #[serde(default)]
    pub pricing: HashMap<String, ModelPricing>,
    #[serde(default)]
    pub litellm: Option<LiteLLMConfig>,
    #[serde(default)]
    pub review: ReviewConfig,
}

impl Config {
    pub fn template_content() -> String {
        let example = Config {
            defaults: ProviderDefaults {
                provider: Some("openrouter".to_string()),
                model: Some("~deepseek/deepseek-v4-flash-latest".to_string()),
            },
            tiers: ModelTiers {
                small: Some("~deepseek/deepseek-v4-flash-latest".to_string()),
                normal: Some("deepseek/deepseek-v4-pro".to_string()),
                thinking: Some("z-ai/glm-5.2".to_string()),
                reasoning: TierReasoning::default(),
            },
            providers: {
                let mut m = HashMap::new();
                m.insert(
                    "custom".to_string(),
                    ProviderConfig {
                        endpoint: "https://api.example.com/v1".to_string(),
                        api_key: Some("sk-...".to_string()),
                        default_model: Some("custom-model".to_string()),
                        catalog_endpoint: None,
                    },
                );
                m
            },
            pricing: {
                let mut m = HashMap::new();
                m.insert(
                    "~deepseek/deepseek-v4-flash-latest".to_string(),
                    ModelPricing {
                        input: 0.15,
                        output: 0.60,
                    },
                );
                m.insert(
                    "deepseek/deepseek-v4-pro".to_string(),
                    ModelPricing {
                        input: 2.50,
                        output: 10.00,
                    },
                );
                m.insert(
                    "z-ai/glm-5.2".to_string(),
                    ModelPricing {
                        input: 1.10,
                        output: 4.40,
                    },
                );
                m
            },
            litellm: None,
            review: ReviewConfig::default(),
        };
        let raw = toml::to_string_pretty(&example).unwrap_or_default();
        format!(
            "# watn configuration file\n\
             # Uncomment and edit settings to override the defaults below.\n\
             \n\
             {}",
            comment_toml(&raw)
        )
    }
}

fn default_review_panel() -> bool {
    true
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub struct ReviewConfig {
    #[serde(default = "default_review_panel")]
    pub panel: bool,
}

impl Default for ReviewConfig {
    fn default() -> Self {
        Self { panel: true }
    }
}

/// Per-invocation review-panel override. `Unset` leaves persisted configuration intact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ReviewPanelOverride {
    #[default]
    Unset,
    Enabled,
    Disabled,
}

impl ReviewPanelOverride {
    pub fn from_flags(review_panel: bool, no_review_panel: bool) -> Result<Self, &'static str> {
        match (review_panel, no_review_panel) {
            (true, true) => Err("--review-panel and --no-review-panel are mutually exclusive"),
            (true, false) => Ok(Self::Enabled),
            (false, true) => Ok(Self::Disabled),
            (false, false) => Ok(Self::Unset),
        }
    }

    pub fn resolve(self, persisted: bool) -> bool {
        match self {
            Self::Unset => persisted,
            Self::Enabled => true,
            Self::Disabled => false,
        }
    }
}

pub fn review_panel_enabled(config: &Config, override_value: ReviewPanelOverride) -> bool {
    override_value.resolve(config.review.panel)
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProviderDefaults {
    pub provider: Option<String>,
    pub model: Option<String>,
}

impl Default for ProviderDefaults {
    fn default() -> Self {
        Self {
            provider: Some("openrouter".to_string()),
            model: None,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProviderConfig {
    pub endpoint: String,
    pub api_key: Option<String>,
    pub default_model: Option<String>,
    #[serde(default)]
    pub catalog_endpoint: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModelTiers {
    pub small: Option<String>,
    pub normal: Option<String>,
    pub thinking: Option<String>,
    #[serde(default, skip_serializing_if = "TierReasoning::is_empty")]
    pub reasoning: TierReasoning,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TierReasoning {
    pub small: Option<String>,
    pub normal: Option<String>,
    pub thinking: Option<String>,
}

impl TierReasoning {
    fn is_empty(&self) -> bool {
        self.small.is_none() && self.normal.is_none() && self.thinking.is_none()
    }

    /// Map a tier ("1"/"2"/"3" or None default "1") to a `reasoning_effort`
    /// value. Returns `None` for "off" or an absent config (no reasoning),
    /// otherwise `Some(strength)`. Backwards compatibility: the thinking tier
    /// with no explicit config defaults to "high", matching prior behaviour.
    pub fn effort(&self, tier: Option<&str>) -> Option<String> {
        let value = match tier {
            Some("2") => self.normal.as_deref(),
            Some("3") => self.thinking.as_deref(),
            _ => self.small.as_deref(),
        };
        match value {
            None => {
                if matches!(tier, Some("3")) {
                    Some("high".to_string())
                } else {
                    None
                }
            }
            Some("off") => None,
            Some(s) if matches!(s, "low" | "minimal" | "medium" | "high") => Some(s.to_string()),
            Some(value) if !value.trim().is_empty() => Some(value.to_string()),
            Some(_) => None,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ModelPricing {
    pub input: f64,
    pub output: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LiteLLMConfig {
    pub endpoint: String,
    pub api_key: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::{review_panel_enabled, Config, ReviewConfig, ReviewPanelOverride};

    #[test]
    fn template_does_not_include_schema_version() {
        assert!(!Config::template_content().contains("schema_version"));
    }

    #[test]
    fn review_panel_defaults_to_enabled_and_is_persisted() {
        let config = Config::default();
        assert!(config.review.panel);

        let parsed: Config = toml::from_str("[review]\npanel = false\n").unwrap();
        assert!(!parsed.review.panel);
        assert!(Config::template_content().contains("[review]"));
        assert!(Config::template_content().contains("panel = true"));
    }

    #[test]
    fn review_panel_override_is_tri_state_and_has_precedence() {
        let config = Config {
            review: ReviewConfig { panel: false },
            ..Config::default()
        };

        assert!(!review_panel_enabled(&config, ReviewPanelOverride::Unset));
        assert!(review_panel_enabled(&config, ReviewPanelOverride::Enabled));
        assert!(!review_panel_enabled(
            &config,
            ReviewPanelOverride::Disabled
        ));
        assert_eq!(
            ReviewPanelOverride::from_flags(true, true),
            Err("--review-panel and --no-review-panel are mutually exclusive")
        );
    }

    #[test]
    fn review_panel_override_flags_are_exhaustive_and_precedential() {
        assert_eq!(
            ReviewPanelOverride::from_flags(false, false),
            Ok(ReviewPanelOverride::Unset)
        );
        assert_eq!(
            ReviewPanelOverride::from_flags(true, false),
            Ok(ReviewPanelOverride::Enabled)
        );
        assert_eq!(
            ReviewPanelOverride::from_flags(false, true),
            Ok(ReviewPanelOverride::Disabled)
        );
        assert!(ReviewPanelOverride::Enabled.resolve(false));
        assert!(!ReviewPanelOverride::Disabled.resolve(true));

        let parsed: Config = toml::from_str("[review]\n").unwrap();
        assert!(
            parsed.review.panel,
            "an empty [review] section keeps the enabled default"
        );
    }
}
