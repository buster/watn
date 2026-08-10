pub fn endpoint(configured: &str) -> String {
    #[cfg(all(feature = "test-support", debug_assertions))]
    {
        std::env::var("WATN_TEST_ENDPOINT_OVERRIDE")
            .ok()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| configured.to_string())
    }

    #[cfg(not(all(feature = "test-support", debug_assertions)))]
    {
        configured.to_string()
    }
}

#[cfg(all(test, feature = "test-support", debug_assertions))]
mod tests {
    use super::endpoint;

    #[test]
    fn debug_test_support_uses_non_empty_override() {
        let previous = std::env::var_os("WATN_TEST_ENDPOINT_OVERRIDE");
        std::env::set_var("WATN_TEST_ENDPOINT_OVERRIDE", "http://isolated.test/v1");

        assert_eq!(
            endpoint("http://configured.test/v1"),
            "http://isolated.test/v1"
        );

        restore(previous);
    }

    #[test]
    fn debug_test_support_ignores_whitespace_override() {
        let previous = std::env::var_os("WATN_TEST_ENDPOINT_OVERRIDE");
        std::env::set_var("WATN_TEST_ENDPOINT_OVERRIDE", "   ");

        assert_eq!(
            endpoint("http://configured.test/v1"),
            "http://configured.test/v1"
        );

        restore(previous);
    }

    fn restore(previous: Option<std::ffi::OsString>) {
        if let Some(value) = previous {
            std::env::set_var("WATN_TEST_ENDPOINT_OVERRIDE", value);
        } else {
            std::env::remove_var("WATN_TEST_ENDPOINT_OVERRIDE");
        }
    }
}

#[cfg(all(test, not(feature = "test-support")))]
mod default_build_tests {
    use super::endpoint;

    #[test]
    fn default_build_ignores_override() {
        let previous = std::env::var_os("WATN_TEST_ENDPOINT_OVERRIDE");
        std::env::set_var("WATN_TEST_ENDPOINT_OVERRIDE", "http://isolated.test/v1");

        assert_eq!(
            endpoint("http://configured.test/v1"),
            "http://configured.test/v1"
        );

        if let Some(value) = previous {
            std::env::set_var("WATN_TEST_ENDPOINT_OVERRIDE", value);
        } else {
            std::env::remove_var("WATN_TEST_ENDPOINT_OVERRIDE");
        }
    }
}
