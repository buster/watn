use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use crate::error::Error;

use super::list::{search_models, ModelEntry};

#[derive(Debug, Clone)]
pub struct PickerState {
    pub suggestions: Vec<ModelEntry>,
    pub query: String,
    pub search_in_flight: bool,
    pub error_message: Option<String>,
    pub no_results: bool,
}

impl PickerState {
    pub fn new(suggestions: Vec<ModelEntry>) -> Self {
        Self {
            suggestions,
            query: String::new(),
            search_in_flight: false,
            error_message: None,
            no_results: false,
        }
    }
}

fn local_filter(models: &[ModelEntry], query: &str) -> Vec<ModelEntry> {
    let query_lower = query.to_lowercase();
    models
        .iter()
        .filter(|m| m.id.to_lowercase().contains(&query_lower))
        .cloned()
        .collect()
}

pub fn execute_search(
    endpoint: &str,
    api_key: Option<&str>,
    query: &str,
    all_models: &[ModelEntry],
    generation: &Arc<AtomicU64>,
    current_gen: u64,
) -> Result<(Vec<ModelEntry>, Option<String>, bool), Error> {
    match search_models(endpoint, query, api_key) {
        Ok(results) => {
            if generation.load(Ordering::SeqCst) != current_gen {
                return Ok((Vec::new(), None, false));
            }
            if results.is_empty() {
                Ok((Vec::new(), None, true))
            } else {
                Ok((results, None, false))
            }
        }
        Err(Error::ApiError { status: 501, .. }) | Err(Error::ApiError { status: 400, .. }) => {
            if generation.load(Ordering::SeqCst) != current_gen {
                return Ok((Vec::new(), None, false));
            }
            let filtered = local_filter(all_models, query);
            let is_empty = filtered.is_empty();
            Ok((filtered, Some("model search is not supported by this provider".to_string()), is_empty))
        }
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_local_filter_matches_substring() {
        let models = vec![
            ModelEntry { id: "gpt-4o-mini".into(), name: None, context_length: None, pricing: None, supported_features: vec![] },
            ModelEntry { id: "gpt-4o".into(), name: None, context_length: None, pricing: None, supported_features: vec![] },
            ModelEntry { id: "o3-mini".into(), name: None, context_length: None, pricing: None, supported_features: vec![] },
            ModelEntry { id: "o3-pro".into(), name: None, context_length: None, pricing: None, supported_features: vec![] },
        ];

        let filtered = local_filter(&models, "gpt");
        assert_eq!(filtered.len(), 2);
        assert!(filtered.iter().any(|m| m.id == "gpt-4o-mini"));
        assert!(filtered.iter().any(|m| m.id == "gpt-4o"));
    }

    #[test]
    fn test_local_filter_case_insensitive() {
        let models = vec![
            ModelEntry { id: "GPT-4o".into(), name: None, context_length: None, pricing: None, supported_features: vec![] },
        ];

        let filtered = local_filter(&models, "gpt");
        assert_eq!(filtered.len(), 1);
    }

    #[test]
    fn test_local_filter_no_match() {
        let models = vec![
            ModelEntry { id: "gpt-4o".into(), name: None, context_length: None, pricing: None, supported_features: vec![] },
        ];

        let filtered = local_filter(&models, "claude");
        assert!(filtered.is_empty());
    }

    #[test]
    fn test_local_filter_empty_query_returns_all() {
        let models = vec![
            ModelEntry { id: "gpt-4o".into(), name: None, context_length: None, pricing: None, supported_features: vec![] },
            ModelEntry { id: "o3-mini".into(), name: None, context_length: None, pricing: None, supported_features: vec![] },
        ];

        let filtered = local_filter(&models, "");
        assert_eq!(filtered.len(), 2);
    }
}
