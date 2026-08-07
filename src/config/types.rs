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
            },
            providers: {
                let mut m = HashMap::new();
                m.insert("custom".to_string(), ProviderConfig {
                    endpoint: "https://api.example.com/v1".to_string(),
                    api_key: Some("sk-...".to_string()),
                    default_model: Some("custom-model".to_string()),
                });
                m
            },
            pricing: {
                let mut m = HashMap::new();
                m.insert("~deepseek/deepseek-v4-flash-latest".to_string(), ModelPricing { input: 0.15, output: 0.60 });
                m.insert("deepseek/deepseek-v4-pro".to_string(), ModelPricing { input: 2.50, output: 10.00 });
                m.insert("z-ai/glm-5.2".to_string(), ModelPricing { input: 1.10, output: 4.40 });
                m
            },
            litellm: None,
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
#[allow(dead_code)]
pub struct ResolvedConfig {
    pub provider: String,
    pub model: String,
    pub endpoint: String,
    pub api_key: Option<String>,
    pub pricing: Option<ModelPricing>,
}
