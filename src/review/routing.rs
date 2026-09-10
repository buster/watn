use crate::config::types::{review_panel_enabled, Config, ReviewPanelOverride};

/// Routing decision for a command request before generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestRoute {
    /// Review surface is enabled and the request is terminal-eligible.
    ReviewSurface,
    /// Existing command-output channel, unchanged.
    DirectCommandOutput,
    /// Existing `Execute now?` confirmation, unchanged.
    ExecuteWithConfirmation,
}

/// Effective review mode: persisted configuration, per-invocation override,
/// and terminal eligibility captured before generation.
pub fn resolve_review_enabled(
    config: &Config,
    override_value: ReviewPanelOverride,
    eligible: bool,
) -> bool {
    review_panel_enabled(config, override_value) && eligible
}

pub fn request_route(review_enabled: bool, execute: bool) -> RequestRoute {
    match (review_enabled, execute) {
        (true, _) => RequestRoute::ReviewSurface,
        (false, true) => RequestRoute::ExecuteWithConfirmation,
        (false, false) => RequestRoute::DirectCommandOutput,
    }
}

#[cfg(test)]
mod tests {
    use super::{request_route, resolve_review_enabled, RequestRoute};
    use crate::config::types::{Config, ReviewConfig, ReviewPanelOverride};

    #[test]
    fn disabled_review_routes_to_the_existing_paths() {
        let disabled = Config {
            review: ReviewConfig { panel: false },
            ..Config::default()
        };
        assert!(!resolve_review_enabled(
            &disabled,
            ReviewPanelOverride::Unset,
            true
        ));
        assert_eq!(
            request_route(false, false),
            RequestRoute::DirectCommandOutput
        );
        assert_eq!(
            request_route(false, true),
            RequestRoute::ExecuteWithConfirmation
        );
    }

    #[test]
    fn ineligible_requests_stay_out_of_review_even_when_enabled() {
        let enabled = Config::default();
        assert!(!resolve_review_enabled(
            &enabled,
            ReviewPanelOverride::Unset,
            false
        ));
        assert!(resolve_review_enabled(
            &enabled,
            ReviewPanelOverride::Enabled,
            true
        ));
    }

    #[test]
    fn eligible_review_routes_to_the_review_surface() {
        assert_eq!(request_route(true, false), RequestRoute::ReviewSurface);
        assert_eq!(request_route(true, true), RequestRoute::ReviewSurface);
    }
}
