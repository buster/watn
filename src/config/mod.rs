pub mod env;
pub mod types;

use std::path::PathBuf;

use crate::config::types::*;
use crate::error::Error;
use crate::provider::setup::ProviderDraft;

pub fn xdg_config_path() -> PathBuf {
    let base = if let Ok(dir) = std::env::var("XDG_CONFIG_HOME") {
        PathBuf::from(dir)
    } else if let Ok(home) = std::env::var("HOME") {
        PathBuf::from(home).join(".config")
    } else {
        PathBuf::from(".")
    };
    base.join("watn").join("config.toml")
}

pub fn load_config() -> Result<Config, Error> {
    let config_path = xdg_config_path();

    if !config_path.exists() {
        return Ok(Config::default());
    }

    let content = std::fs::read_to_string(&config_path)
        .map_err(|e| Error::ConfigError(format!("cannot read config: {}", e)))?;

    #[cfg(unix)]
    let has_real_content = content.lines().any(|line| {
        let line = line.trim();
        !line.is_empty() && !line.starts_with('#')
    });

    let mut config = if content.trim().is_empty() {
        Config::default()
    } else {
        toml::from_str(&content).map_err(|e| Error::ConfigError(format!("parse error: {}", e)))?
    };

    #[cfg(unix)]
    if has_real_content {
        if let Ok(meta) = std::fs::metadata(&config_path) {
            use std::os::unix::fs::PermissionsExt;
            let mode = meta.permissions().mode();
            if mode & 0o004 != 0 {
                eprintln!(
                    "warning: config file is world-readable ({:o})",
                    mode & 0o777
                );
            }
        }
    }

    let env_overrides = env::read_env_overrides();

    if let Some(provider) = env_overrides.get("provider") {
        config.defaults.provider = Some(provider.clone());
    }
    if let Some(model) = env_overrides.get("model") {
        config.defaults.model = Some(model.clone());
    }

    Ok(config)
}

pub fn resolve_provider(config: &Config, provider_name: &str) -> Result<ProviderConfig, Error> {
    if provider_name == "openai" {
        if let Some(pc) = config.providers.get("openai") {
            let mut pc = pc.clone();
            if pc.api_key.is_none() {
                pc.api_key = std::env::var("WATN_OPENAI_API_KEY").ok();
            }
            return Ok(pc);
        }
        return Ok(ProviderConfig {
            endpoint: "https://api.openai.com/v1".to_string(),
            api_key: std::env::var("WATN_OPENAI_API_KEY").ok(),
            default_model: None,
        });
    }

    if provider_name == "openrouter" {
        if let Some(provider) = config.providers.get("openrouter") {
            return Ok(provider.clone());
        }
        return Ok(ProviderConfig {
            endpoint: "https://openrouter.ai/api/v1".to_string(),
            api_key: None,
            default_model: None,
        });
    }

    config
        .providers
        .get(provider_name)
        .cloned()
        .ok_or_else(|| Error::ProviderNotFound(provider_name.to_string()))
}

pub fn provider_ready(config: &Config, provider_name: &str) -> bool {
    resolve_provider(config, provider_name)
        .and_then(|provider| get_provider_api_key(provider_name, &provider).map(|_| ()))
        .is_ok()
}

pub fn resolve_model(
    config: &Config,
    tier: Option<&str>,
    explicit_model: Option<&str>,
) -> Result<String, Error> {
    if let Some(model) = explicit_model {
        return Ok(model.to_string());
    }

    match tier {
        Some("1") | None => {
            if let Some(model) = &config.tiers.small {
                Ok(model.clone())
            } else if let Some(model) = &config.defaults.model {
                Ok(model.clone())
            } else {
                resolve_default_model(config)
            }
        }
        Some("2") => {
            if let Some(model) = &config.tiers.normal {
                Ok(model.clone())
            } else if let Some(model) = &config.defaults.model {
                Ok(model.clone())
            } else {
                resolve_default_model(config)
            }
        }
        Some("3") => {
            if let Some(model) = &config.tiers.thinking {
                Ok(model.clone())
            } else if let Some(model) = &config.defaults.model {
                Ok(model.clone())
            } else {
                resolve_default_model(config)
            }
        }
        Some(t) => Err(Error::ConfigError(format!("unknown tier: {}", t))),
    }
}

fn resolve_default_model(config: &Config) -> Result<String, Error> {
    let provider = config.defaults.provider.as_deref().unwrap_or("openrouter");
    if provider == "openai" {
        Ok("gpt-4o-mini".to_string())
    } else if provider == "openrouter" {
        Ok("~deepseek/deepseek-v4-flash-latest".to_string())
    } else if let Some(pc) = config.providers.get(provider) {
        pc.default_model
            .clone()
            .ok_or_else(|| Error::ConfigError("no default model configured".to_string()))
    } else {
        Err(Error::ConfigError(
            "no default model configured".to_string(),
        ))
    }
}

pub fn save_config(config: &Config) -> Result<(), Error> {
    let config_path = xdg_config_path();
    let content = toml::to_string_pretty(config)
        .map_err(|e| Error::ConfigError(format!("serialize error: {}", e)))?;
    if let Some(parent) = config_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| Error::ConfigError(format!("cannot create config dir: {}", e)))?;
    }
    std::fs::write(&config_path, content)
        .map_err(|e| Error::ConfigError(format!("cannot write config: {}", e)))?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&config_path, std::fs::Permissions::from_mode(0o600))
            .map_err(|e| Error::ConfigError(format!("cannot set config permissions: {}", e)))?;
    }

    Ok(())
}

pub fn save_provider_draft(config: &mut Config, draft: &ProviderDraft) -> Result<(), Error> {
    config.defaults.provider = Some(draft.name.clone());
    config.providers.insert(
        draft.name.clone(),
        ProviderConfig {
            endpoint: draft.endpoint.clone(),
            api_key: Some(draft.api_key.clone()),
            default_model: None,
        },
    );
    save_config(config)
}

fn environment_reference(value: &str) -> Option<&str> {
    let name = value.strip_prefix("${")?.strip_suffix('}')?;
    if name.is_empty()
        || !name.chars().enumerate().all(|(index, character)| {
            (index == 0 && (character == '_' || character.is_ascii_uppercase()))
                || (index > 0
                    && (character == '_'
                        || character.is_ascii_uppercase()
                        || character.is_ascii_digit()))
        })
    {
        return None;
    }
    Some(name)
}

pub fn expand_api_key(value: &str) -> Result<String, Error> {
    if value.trim().is_empty() {
        return Err(Error::AuthError("api key is empty".to_string()));
    }
    let Some(name) = environment_reference(value) else {
        return Ok(value.to_string());
    };
    match std::env::var(name) {
        Ok(key) if !key.is_empty() => Ok(key),
        _ => Err(Error::AuthError(format!(
            "api key environment variable '{}' is not set",
            name
        ))),
    }
}

pub fn get_provider_api_key(
    provider_name: &str,
    provider_config: &ProviderConfig,
) -> Result<String, Error> {
    if let Some(key) = &provider_config.api_key {
        return expand_api_key(key);
    }

    if let Some(key) = env::provider_api_key(provider_name) {
        return Ok(key);
    }

    Err(Error::AuthError(format!(
        "api key not found for provider '{}'",
        provider_name
    )))
}
