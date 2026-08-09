use std::collections::HashMap;

pub fn read_env_overrides() -> HashMap<String, String> {
    let mut overrides = HashMap::new();

    for (key, value) in std::env::vars() {
        if let Some(rest) = key.strip_prefix("WATN_") {
            let lower = rest.to_lowercase();
            match lower.as_str() {
                "provider" => {
                    overrides.insert("provider".to_string(), value);
                }
                "model" => {
                    overrides.insert("model".to_string(), value);
                }
                _ => {
                    if let Some(provider) = lower.strip_suffix("_api_key") {
                        overrides.insert(format!("api_key_{}", provider), value);
                    }
                }
            }
        }
    }

    overrides
}

pub fn provider_api_key(provider_name: &str) -> Option<String> {
    let provider_specific = if provider_name == "openrouter" {
        "OPENROUTER_API_KEY".to_string()
    } else {
        format!("WATN_{}_API_KEY", provider_name.to_uppercase())
    };

    std::env::var(&provider_specific)
        .ok()
        .filter(|value| !value.is_empty())
        .or_else(|| std::env::var("WATN_API_KEY").ok().filter(|value| !value.is_empty()))
}
