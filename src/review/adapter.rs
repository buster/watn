/// Terminal-specific renderer of the review surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PresentationAdapter {
    Enhanced,
    Portable,
}

/// Adapter selection result. The portable inline adapter is mandatory; a
/// selected enhanced adapter that fails to open falls back to it without
/// losing the candidate or focus state. A portable failure is `Unavailable`.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct PresentationSelection {
    attempts: Vec<PresentationAdapter>,
    active: Option<PresentationAdapter>,
    unavailable: bool,
}

impl PresentationSelection {
    /// `enhanced_open` is `Some(true)` when the enhanced adapter opened,
    /// `Some(false)` when it was selected but failed, and `None` when no
    /// enhanced adapter was selected.
    pub fn open(enhanced_open: Option<bool>, portable_open: bool) -> Self {
        let mut attempts = Vec::new();
        if enhanced_open == Some(true) {
            attempts.push(PresentationAdapter::Enhanced);
            return Self {
                attempts,
                active: Some(PresentationAdapter::Enhanced),
                unavailable: false,
            };
        }
        if enhanced_open == Some(false) {
            attempts.push(PresentationAdapter::Enhanced);
        }
        attempts.push(PresentationAdapter::Portable);
        if portable_open {
            Self {
                attempts,
                active: Some(PresentationAdapter::Portable),
                unavailable: false,
            }
        } else {
            Self {
                attempts,
                active: None,
                unavailable: true,
            }
        }
    }

    pub fn attempts(&self) -> &[PresentationAdapter] {
        &self.attempts
    }

    pub fn active(&self) -> Option<PresentationAdapter> {
        self.active
    }

    pub fn is_unavailable(&self) -> bool {
        self.unavailable
    }
}

#[cfg(test)]
mod tests {
    use super::{PresentationAdapter, PresentationSelection};

    #[test]
    fn enhanced_failure_falls_back_to_portable_without_losing_the_review() {
        let selection = PresentationSelection::open(Some(false), true);
        assert_eq!(
            selection.attempts(),
            &[PresentationAdapter::Enhanced, PresentationAdapter::Portable]
        );
        assert_eq!(selection.active(), Some(PresentationAdapter::Portable));
        assert!(!selection.is_unavailable());
    }

    #[test]
    fn portable_failure_is_unavailable_and_releases_nothing() {
        let selection = PresentationSelection::open(None, false);
        assert_eq!(selection.attempts(), &[PresentationAdapter::Portable]);
        assert_eq!(selection.active(), None);
        assert!(selection.is_unavailable());
    }

    #[test]
    fn enhanced_success_stays_on_the_enhanced_adapter() {
        let selection = PresentationSelection::open(Some(true), true);
        assert_eq!(selection.attempts(), &[PresentationAdapter::Enhanced]);
        assert_eq!(selection.active(), Some(PresentationAdapter::Enhanced));
    }
}
