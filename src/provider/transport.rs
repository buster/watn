pub fn endpoint(configured: &str) -> String {
    std::env::var("WATN_TEST_ENDPOINT_OVERRIDE")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| configured.to_string())
}
