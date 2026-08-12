use crate::error::Error;
use std::fmt;

pub const OPENROUTER_ENDPOINT: &str = "https://openrouter.ai/api/v1";
pub const OPENAI_ENDPOINT: &str = "https://api.openai.com/v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderIdentity {
    OpenRouter,
    OpenAi,
    Custom,
}

impl ProviderIdentity {
    pub fn name(self) -> &'static str {
        match self {
            Self::OpenRouter => "openrouter",
            Self::OpenAi => "openai",
            Self::Custom => "custom",
        }
    }

    pub fn endpoint(self) -> Option<&'static str> {
        match self {
            Self::OpenRouter => Some(OPENROUTER_ENDPOINT),
            Self::OpenAi => Some(OPENAI_ENDPOINT),
            Self::Custom => None,
        }
    }

    pub fn from_config(name: &str, endpoint: &str) -> Self {
        match name {
            "openrouter" if endpoint == OPENROUTER_ENDPOINT => Self::OpenRouter,
            "openai" if endpoint == OPENAI_ENDPOINT => Self::OpenAi,
            _ => Self::Custom,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SetupCancellation {
    Escape,
    CtrlC,
}

#[derive(Clone, PartialEq, Eq)]
pub struct ProviderDraft {
    pub name: String,
    pub endpoint: String,
    pub api_key: String,
    pub identity: ProviderIdentity,
}

impl fmt::Debug for ProviderDraft {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ProviderDraft")
            .field("name", &self.name)
            .field("endpoint", &self.endpoint)
            .field("api_key", &"[REDACTED]")
            .field("identity", &self.identity)
            .finish()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProviderSetupResult {
    Configured(ProviderDraft),
    Cancelled(SetupCancellation),
}

pub fn cancellation_result(cancellation: SetupCancellation) -> ProviderSetupResult {
    ProviderSetupResult::Cancelled(cancellation)
}

pub fn configured_result(draft: ProviderDraft) -> ProviderSetupResult {
    ProviderSetupResult::Configured(draft)
}

#[derive(Debug)]
pub enum ModelSetupResult {
    Saved,
    Cancelled(SetupCancellation),
    Failed(Error),
}

pub fn print_setup_guidance() {
    eprintln!(
        "No watn configuration found. Run `watn setup` in a terminal or edit ~/.config/watn/config.toml."
    );
}

pub fn normalize_endpoint(endpoint: &str) -> Result<String, Error> {
    let endpoint = endpoint.trim().trim_end_matches('/').to_string();
    if endpoint.is_empty() {
        return Err(Error::ConfigError(
            "endpoint must be an HTTP or HTTPS URL".to_string(),
        ));
    }

    let parsed = reqwest::Url::parse(&endpoint)
        .map_err(|_| Error::ConfigError("endpoint must be an HTTP or HTTPS URL".to_string()))?;
    if !matches!(parsed.scheme(), "http" | "https") || parsed.host().is_none() {
        return Err(Error::ConfigError(
            "endpoint must be an HTTP or HTTPS URL".to_string(),
        ));
    }

    Ok(endpoint)
}

pub fn provider_name(endpoint: &str) -> &'static str {
    match endpoint {
        OPENROUTER_ENDPOINT => "openrouter",
        OPENAI_ENDPOINT => "openai",
        _ => "custom",
    }
}

pub fn suggested_api_key_env(endpoint: &str) -> &'static str {
    match endpoint {
        OPENROUTER_ENDPOINT => "OPENROUTER_API_KEY",
        OPENAI_ENDPOINT => "OPENAI_API_KEY",
        _ => "WATN_API_KEY",
    }
}

pub fn build_provider_draft(endpoint: &str, api_key: &str) -> Result<ProviderDraft, Error> {
    let endpoint = normalize_endpoint(endpoint)?;
    let identity = ProviderIdentity::from_config(provider_name(&endpoint), &endpoint);
    build_provider_draft_for_identity(identity, &endpoint, api_key)
}

pub fn build_provider_draft_for_identity(
    identity: ProviderIdentity,
    endpoint: &str,
    api_key: &str,
) -> Result<ProviderDraft, Error> {
    let endpoint = normalize_endpoint(endpoint)?;
    if api_key.trim().is_empty() {
        return Err(Error::ConfigError("credential cannot be empty".to_string()));
    }

    let identity =
        if identity != ProviderIdentity::Custom && identity.endpoint() != Some(endpoint.as_str()) {
            ProviderIdentity::Custom
        } else {
            identity
        };

    Ok(ProviderDraft {
        name: identity.name().to_string(),
        endpoint,
        api_key: api_key.to_string(),
        identity,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provider_debug_redacts_literal_credentials() {
        let draft = build_provider_draft("https://example.test/v1", "sk-secret").unwrap();
        let debug = format!("{draft:?}");
        assert!(!debug.contains("sk-secret"));
        assert!(debug.contains("REDACTED"));
    }
}
