use crate::error::Error;

pub const OPENROUTER_ENDPOINT: &str = "https://openrouter.ai/api/v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SetupCancellation {
    Escape,
    CtrlC,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderDraft {
    pub name: String,
    pub endpoint: String,
    pub api_key: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProviderSetupResult {
    Configured(ProviderDraft),
    Cancelled(SetupCancellation),
}

#[derive(Debug)]
pub enum ModelSetupResult {
    Saved,
    Cancelled(SetupCancellation),
    Failed(Error),
}

pub fn normalize_endpoint(endpoint: &str) -> Result<String, Error> {
    let endpoint = endpoint.trim().trim_end_matches('/').to_string();
    if endpoint.is_empty() {
        return Err(Error::ConfigError(
            "endpoint must be an HTTP or HTTPS URL".to_string(),
        ));
    }

    let parsed = reqwest::Url::parse(&endpoint).map_err(|_| {
        Error::ConfigError("endpoint must be an HTTP or HTTPS URL".to_string())
    })?;
    if !matches!(parsed.scheme(), "http" | "https") || parsed.host().is_none() {
        return Err(Error::ConfigError(
            "endpoint must be an HTTP or HTTPS URL".to_string(),
        ));
    }

    Ok(endpoint)
}

pub fn provider_name(endpoint: &str) -> &'static str {
    if endpoint == OPENROUTER_ENDPOINT {
        "openrouter"
    } else {
        "custom"
    }
}

pub fn build_provider_draft(endpoint: &str, api_key: &str) -> Result<ProviderDraft, Error> {
    let endpoint = normalize_endpoint(endpoint)?;
    if api_key.trim().is_empty() {
        return Err(Error::ConfigError("credential cannot be empty".to_string()));
    }

    Ok(ProviderDraft {
        name: provider_name(&endpoint).to_string(),
        endpoint,
        api_key: api_key.to_string(),
    })
}
