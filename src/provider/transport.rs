pub fn endpoint(configured: &str) -> String {
    #[cfg(all(feature = "test-support", debug_assertions))]
    {
        endpoint_with_override(
            configured,
            std::env::var("WATN_TEST_ENDPOINT_OVERRIDE").ok().as_deref(),
        )
    }

    #[cfg(not(all(feature = "test-support", debug_assertions)))]
    {
        configured.to_string()
    }
}

#[cfg(all(feature = "test-support", debug_assertions))]
fn endpoint_with_override(configured: &str, override_value: Option<&str>) -> String {
    override_value
        .filter(|value| !value.trim().is_empty())
        .map(str::to_owned)
        .unwrap_or_else(|| configured.to_string())
}

#[cfg(all(test, feature = "test-support", debug_assertions))]
mod tests {
    use super::endpoint_with_override;

    #[test]
    fn debug_test_support_uses_non_empty_override() {
        assert_eq!(
            endpoint_with_override("http://configured.test/v1", Some("http://isolated.test/v1")),
            "http://isolated.test/v1"
        );
    }

    #[test]
    fn debug_test_support_ignores_whitespace_override() {
        assert_eq!(
            endpoint_with_override("http://configured.test/v1", Some("   ")),
            "http://configured.test/v1"
        );
    }
}

#[cfg(all(test, not(feature = "test-support")))]
mod default_build_tests {
    use super::endpoint;

    #[test]
    fn default_build_uses_configured_endpoint() {
        assert_eq!(
            endpoint("http://configured.test/v1"),
            "http://configured.test/v1"
        );
    }
}
