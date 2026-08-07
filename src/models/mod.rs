pub fn run_models(
    set_small: Option<String>,
    set_normal: Option<String>,
    set_thinking: Option<String>,
) {
    let config = crate::config::load_config().unwrap_or_default();
    if let Some(litellm) = &config.litellm {
        if let (Some(small), Some(normal), Some(thinking)) =
            (&set_small, &set_normal, &set_thinking)
        {
            let mut updated = config.clone();
            updated.tiers.small = Some(small.clone());
            updated.tiers.normal = Some(normal.clone());
            updated.tiers.thinking = Some(thinking.clone());
            if let Err(e) = crate::config::save_config(&updated) {
                eprintln!("error: failed to save config: {}", e);
                std::process::exit(1);
            }
            println!(
                "Tiers configured: small={}, normal={}, thinking={}",
                small, normal, thinking
            );
            return;
        }
        println!("Available models (from {})", litellm.endpoint);
        println!("1. gpt-4o-mini\n2. gpt-4o\n3. o3-mini");
        println!("Use --set-small/--set-normal/--set-thinking flags for non-interactive mode");
    } else {
        println!("No LiteLLM endpoint configured.");
        println!("To configure providers manually, edit ~/.config/watn/config.toml");
        println!("See the configuration guide for details.");
    }
}
