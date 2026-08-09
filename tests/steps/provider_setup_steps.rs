use cucumber::{then, when};
use std::path::PathBuf;

use crate::WatnWorld;
use watn::config::{self, save_provider_draft};
use watn::config::types::Config;
use watn::provider::setup::build_provider_draft;

#[when(regex = r#"^provider setup accepts endpoint \"([^\"]+)\"$"#)]
fn provider_setup_accepts_endpoint(world: &mut WatnWorld, endpoint: String) {
    world
        .pending_config
        .insert("provider_endpoint".to_string(), endpoint);
}

fn config_path(world: &mut WatnWorld) -> PathBuf {
    let dir = if let Some(dir) = &world.temp_dir {
        dir.path().to_path_buf()
    } else {
        let dir = tempfile::tempdir().expect("create provider setup temp dir");
        let path = dir.path().to_path_buf();
        world.temp_dir = Some(dir);
        path
    };
    let config_dir = dir.join("watn");
    std::fs::create_dir_all(&config_dir).expect("create config directory");
    world.env_vars.insert(
        "XDG_CONFIG_HOME".to_string(),
        dir.to_string_lossy().to_string(),
    );
    std::env::set_var("XDG_CONFIG_HOME", &dir);
    config_dir.join("config.toml")
}

fn load_world_config(world: &mut WatnWorld) -> Config {
    let path = config_path(world);
    if !path.exists() {
        let raw = world
            .raw_config
            .clone()
            .unwrap_or_else(|| "[defaults]\nprovider = \"nonexistent\"\n".to_string());
        std::fs::write(&path, raw).expect("write test config");
    }
    config::load_config().expect("load test config")
}

#[when(regex = r#"^provider setup accepts pasted credential \"([^\"]+)\"$"#)]
fn provider_setup_accepts_pasted_credential(world: &mut WatnWorld, credential: String) {
    let endpoint = world
        .pending_config
        .get("provider_endpoint")
        .cloned()
        .expect("provider endpoint must be accepted first");
    let draft = build_provider_draft(&endpoint, &credential).expect("build provider draft");
    let mut config = load_world_config(world);
    save_provider_draft(&mut config, &draft).expect("save provider draft");
    world
        .pending_config
        .insert("provider_name".to_string(), draft.name.clone());
    world.config_content = Some(toml::to_string_pretty(&config).expect("serialize config"));
}

#[then(regex = r#"^provider setup should return configured provider \"([^\"]+)\"$"#)]
fn provider_setup_returns_provider(world: &mut WatnWorld, provider: String) {
    assert_eq!(
        world.pending_config.get("provider_name"),
        Some(&provider),
        "provider setup did not return the expected provider"
    );
}

#[then(regex = r#"^the config file should contain default provider \"([^\"]+)\"$"#)]
fn config_contains_default_provider(world: &mut WatnWorld, provider: String) {
    let config = load_world_config(world);
    assert_eq!(config.defaults.provider.as_deref(), Some(provider.as_str()));
}

#[then(regex = r#"^the config file should contain endpoint exactly \"([^\"]+)\"$"#)]
fn config_contains_endpoint(world: &mut WatnWorld, endpoint: String) {
    let config = load_world_config(world);
    let provider = config.defaults.provider.as_deref().expect("default provider");
    assert_eq!(config.providers[provider].endpoint, endpoint);
}

#[then(regex = r#"^the config file should contain api_key exactly \"([^\"]+)\"$"#)]
fn config_contains_api_key(world: &mut WatnWorld, api_key: String) {
    let config = load_world_config(world);
    let provider = config.defaults.provider.as_deref().expect("default provider");
    assert_eq!(config.providers[provider].api_key.as_deref(), Some(api_key.as_str()));
}
