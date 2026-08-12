#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CredentialCandidate {
    pub name: String,
    pub detected: bool,
}

const OPENROUTER_CREDENTIALS: [&str; 2] = ["OPENROUTER_API_KEY", "WATN_API_KEY"];
const OPENAI_CREDENTIALS: [&str; 3] = ["WATN_OPENAI_API_KEY", "OPENAI_API_KEY", "WATN_API_KEY"];
const CUSTOM_CREDENTIALS: [&str; 1] = ["WATN_API_KEY"];

/// Discover only allowlisted variable names and whether they contain a
/// non-empty value. Resolved credential values never leave this function.
pub fn discover_credentials(provider_name: &str) -> Vec<CredentialCandidate> {
    let names: &[&str] = match provider_name {
        "openrouter" => &OPENROUTER_CREDENTIALS,
        "openai" => &OPENAI_CREDENTIALS,
        _ => &CUSTOM_CREDENTIALS,
    };
    names
        .iter()
        .map(|name| CredentialCandidate {
            name: (*name).to_string(),
            detected: env_present(name),
        })
        .collect()
}

pub fn env_present(name: &str) -> bool {
    std::env::var(name)
        .ok()
        .is_some_and(|value| !value.is_empty())
}

pub fn provider_api_key(provider_name: &str) -> Option<String> {
    if !matches!(provider_name, "openrouter" | "openai" | "custom") {
        let provider_specific = format!("WATN_{}_API_KEY", provider_name.to_uppercase());
        if let Some(value) = std::env::var(&provider_specific)
            .ok()
            .filter(|value| !value.is_empty())
        {
            return Some(value);
        }
    }
    let names: &[&str] = match provider_name {
        "openrouter" => &OPENROUTER_CREDENTIALS,
        "openai" => &OPENAI_CREDENTIALS,
        _ => &CUSTOM_CREDENTIALS,
    };
    names
        .iter()
        .find_map(|name| std::env::var(name).ok().filter(|value| !value.is_empty()))
}
