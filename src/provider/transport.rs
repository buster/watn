pub fn endpoint(configured: &str) -> String {
    #[cfg(all(feature = "test-support", debug_assertions))]
    {
        return std::env::var("WATN_TEST_ENDPOINT_OVERRIDE")
            .ok()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| configured.to_string());
    }

    #[cfg(not(all(feature = "test-support", debug_assertions)))]
    {
        configured.to_string()
    }
}
