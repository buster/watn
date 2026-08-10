use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use crate::error::Error;

use super::list::{search_models, word_matches, ModelEntry};

fn local_filter(models: &[ModelEntry], query: &str) -> Vec<ModelEntry> {
    models
        .iter()
        .filter(|m| word_matches(&m.id, query))
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
            Ok((
                filtered,
                Some("model search is not supported by this provider".to_string()),
                is_empty,
            ))
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
            ModelEntry {
                id: "gpt-4o-mini".into(),
                name: None,
                context_length: None,
                pricing: None,
                supported_features: vec![],
                reasoning: None,
            },
            ModelEntry {
                id: "gpt-4o".into(),
                name: None,
                context_length: None,
                pricing: None,
                supported_features: vec![],
                reasoning: None,
            },
            ModelEntry {
                id: "o3-mini".into(),
                name: None,
                context_length: None,
                pricing: None,
                supported_features: vec![],
                reasoning: None,
            },
            ModelEntry {
                id: "o3-pro".into(),
                name: None,
                context_length: None,
                pricing: None,
                supported_features: vec![],
                reasoning: None,
            },
        ];

        let filtered = local_filter(&models, "gpt");
        assert_eq!(filtered.len(), 2);
        assert!(filtered.iter().any(|m| m.id == "gpt-4o-mini"));
        assert!(filtered.iter().any(|m| m.id == "gpt-4o"));
    }

    #[test]
    fn test_local_filter_case_insensitive() {
        let models = vec![ModelEntry {
            id: "GPT-4o".into(),
            name: None,
            context_length: None,
            pricing: None,
            supported_features: vec![],
            reasoning: None,
        }];

        let filtered = local_filter(&models, "gpt");
        assert_eq!(filtered.len(), 1);
    }

    #[test]
    fn test_local_filter_no_match() {
        let models = vec![ModelEntry {
            id: "gpt-4o".into(),
            name: None,
            context_length: None,
            pricing: None,
            supported_features: vec![],
            reasoning: None,
        }];

        let filtered = local_filter(&models, "claude");
        assert!(filtered.is_empty());
    }

    #[test]
    fn test_local_filter_empty_query_returns_all() {
        let models = vec![
            ModelEntry {
                id: "gpt-4o".into(),
                name: None,
                context_length: None,
                pricing: None,
                supported_features: vec![],
                reasoning: None,
            },
            ModelEntry {
                id: "o3-mini".into(),
                name: None,
                context_length: None,
                pricing: None,
                supported_features: vec![],
                reasoning: None,
            },
        ];

        let filtered = local_filter(&models, "");
        assert_eq!(filtered.len(), 2);
    }

    fn search_mock_endpoint(
        server: &httpmock::MockServer,
        query: &str,
        models: &[&str],
        status: u16,
    ) -> String {
        let q = query.to_string();
        let models: Vec<String> = models.iter().map(|s| s.to_string()).collect();
        server.mock(move |when, then| {
            when.method(httpmock::Method::GET)
                .path("/models")
                .query_param("search", &q);
            let data: Vec<serde_json::Value> = models
                .iter()
                .map(|id| serde_json::json!({"id": id}))
                .collect();
            then.status(status)
                .header("Content-Type", "application/json")
                .body(serde_json::json!({"data": data}).to_string());
        });
        format!("http://127.0.0.1:{}", server.port())
    }

    #[test]
    fn test_execute_search_returns_results() {
        let server = httpmock::MockServer::start();
        let endpoint = search_mock_endpoint(&server, "o3", &["o3-mini"], 200);
        let gen = Arc::new(AtomicU64::new(0));
        let current = gen.fetch_add(1, Ordering::SeqCst) + 1;
        let (results, error, no_results) =
            execute_search(&endpoint, None, "o3", &[], &gen, current).unwrap();
        assert!(error.is_none());
        assert!(!no_results);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "o3-mini");
    }

    #[test]
    fn test_execute_search_stale_result_is_discarded() {
        let server = httpmock::MockServer::start();
        let endpoint = search_mock_endpoint(&server, "o3", &["o3-mini"], 200);
        let gen = Arc::new(AtomicU64::new(0));
        // The request was dispatched when the generation was 1, but the
        // generation has since advanced to 2 before the result landed.
        gen.fetch_add(2, Ordering::SeqCst);
        let (results, error, no_results) =
            execute_search(&endpoint, None, "o3", &[], &gen, 1).unwrap();
        assert!(results.is_empty(), "stale result must be discarded");
        assert!(error.is_none());
        assert!(!no_results);
    }

    #[test]
    fn test_execute_search_unsupported_search_reports_error_and_filters_locally() {
        let server = httpmock::MockServer::start();
        let endpoint = search_mock_endpoint(&server, "gpt", &[], 501);
        let gen = Arc::new(AtomicU64::new(0));
        let current = gen.fetch_add(1, Ordering::SeqCst) + 1;
        let all_models = vec![ModelEntry {
            id: "gpt-4o".into(),
            name: None,
            context_length: None,
            pricing: None,
            supported_features: vec![],
            reasoning: None,
        }];
        let (results, error, no_results) =
            execute_search(&endpoint, None, "gpt", &all_models, &gen, current).unwrap();
        assert_eq!(
            error.as_deref(),
            Some("model search is not supported by this provider")
        );
        assert!(!no_results);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "gpt-4o");
    }

    #[test]
    fn test_execute_search_unsupported_search_stale_is_discarded() {
        let server = httpmock::MockServer::start();
        let endpoint = search_mock_endpoint(&server, "gpt", &[], 501);
        let gen = Arc::new(AtomicU64::new(0));
        // Dispatched at generation 1, but the generation has since advanced.
        gen.fetch_add(2, Ordering::SeqCst);
        let (results, error, no_results) =
            execute_search(&endpoint, None, "gpt", &[], &gen, 1).unwrap();
        assert!(results.is_empty());
        assert!(error.is_none());
        assert!(!no_results);
    }
}
