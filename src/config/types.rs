use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProviderDefaults {
    pub provider: Option<String>,
    pub model: Option<String>,
}

impl Default for ProviderDefaults {
    fn default() -> Self {
        Self {
            provider: Some("openai".to_string()),
            model: None,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProviderConfig {
    pub endpoint: String,
    pub api_key: Option<String>,
    pub default_model: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ModelTiers {
    pub small: Option<String>,
    pub normal: Option<String>,
    pub thinking: Option<String>,
}

impl Default for ModelTiers {
    fn default() -> Self {
        Self {
            small: None,
            normal: None,
            thinking: None,
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

#[derive(Debug, Clone)]
pub struct ResolvedConfig {
    pub provider: String,
    pub model: String,
    pub endpoint: String,
    pub api_key: Option<String>,
    pub pricing: Option<ModelPricing>,
}
