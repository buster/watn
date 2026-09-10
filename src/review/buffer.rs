/// Review-mode provider sink. Complete candidate text is retained internally
/// until the provider reaches its completion boundary (`[DONE]`); nothing is
/// released to the command-output channel before final acceptance.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ReviewBuffer {
    content: String,
    complete: bool,
}

impl ReviewBuffer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn receive(&mut self, chunk: &str) {
        self.content.push_str(chunk);
    }

    pub fn complete(&mut self) {
        self.complete = true;
    }

    pub fn is_complete(&self) -> bool {
        self.complete
    }

    pub fn is_open(&self) -> bool {
        self.complete && !self.content.trim().is_empty()
    }

    pub fn candidate(&self) -> Option<&str> {
        self.is_open().then_some(self.content.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::ReviewBuffer;

    #[test]
    fn retains_chunks_until_completion_and_requires_a_non_empty_candidate() {
        let mut buffer = ReviewBuffer::new();
        buffer.receive("df ");
        buffer.receive("-h");
        assert!(!buffer.is_complete());
        assert_eq!(buffer.candidate(), None);

        buffer.complete();
        assert!(buffer.is_complete());
        assert_eq!(buffer.candidate(), Some("df -h"));
    }

    #[test]
    fn empty_completion_opens_nothing() {
        let mut buffer = ReviewBuffer::new();
        buffer.receive("   ");
        buffer.complete();
        assert!(!buffer.is_open());
        assert_eq!(buffer.candidate(), None);
    }
}
