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
