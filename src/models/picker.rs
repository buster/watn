use console::{Key, Term};
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
            Ok((filtered, Some("model search is not supported by this provider".to_string()), is_empty))
        }
        Err(e) => Err(e),
    }
}

const EMPTY_QUERY_NOTICE: &str = "(type to search)";

/// Interactive raw-mode autosuggest picker used by `watn models` when stdin
/// is a real terminal (or PTY). Displays the tier prompt, echoes the live
/// search query, and lists suggestions fetched from the provider's search
/// endpoint. Enter selects the top suggestion.
pub struct ModelPicker {
    term: Term,
    endpoint: String,
    api_key: Option<String>,
    all_models: Vec<ModelEntry>,
    suggestions: Vec<ModelEntry>,
    query: String,
    generation: Arc<AtomicU64>,
    rendered_lines: usize,
}

impl ModelPicker {
    pub fn new(endpoint: &str, api_key: Option<String>, all_models: Vec<ModelEntry>) -> Self {
        let mut suggestions = all_models.clone();
        if suggestions.is_empty() {
            suggestions.push(ModelEntry {
                id: EMPTY_QUERY_NOTICE.to_string(),
                name: None,
                context_length: None,
                pricing: None,
                supported_features: vec![],
            });
        }
        Self {
            term: Term::stderr(),
            endpoint: endpoint.trim_end_matches('/').to_string(),
            api_key,
            all_models,
            suggestions,
            query: String::new(),
            generation: Arc::new(AtomicU64::new(0)),
            rendered_lines: 0,
        }
    }

    pub fn run(mut self, tier: &str) -> ModelEntry {
        let _ = self.term.write_line(&format!("Select a model for the {} tier:", tier));
        self.render();
        loop {
            let key = self.term.read_key().unwrap_or(Key::Unknown);
            match key {
                Key::Char(c) => {
                    self.query.push(c);
                    self.search();
                    self.render();
                }
                Key::Backspace => {
                    self.query.pop();
                    self.search();
                    self.render();
                }
                Key::Escape => {
                    self.query.clear();
                    self.suggestions = self.initial_list();
                    self.render();
                }
                Key::Enter => {
                    let _ = self.term.clear_last_lines(self.rendered_lines);
                    return self.current_selection();
                }
                Key::CtrlC => {
                    std::process::exit(130);
                }
                _ => {}
            }
        }
    }

    fn initial_list(&self) -> Vec<ModelEntry> {
        let mut list = self.all_models.clone();
        if list.is_empty() {
            list.push(ModelEntry {
                id: EMPTY_QUERY_NOTICE.to_string(),
                name: None,
                context_length: None,
                pricing: None,
                supported_features: vec![],
            });
        }
        list
    }

    fn search(&mut self) {
        if self.query.is_empty() {
            self.suggestions = self.initial_list();
            return;
        }
        let current_gen = self.generation.fetch_add(1, Ordering::SeqCst) + 1;
        match execute_search(
            &self.endpoint,
            self.api_key.as_deref(),
            &self.query,
            &self.all_models,
            &self.generation,
            current_gen,
        ) {
            Ok((results, _error, _no_results)) => {
                self.suggestions = if results.is_empty() { self.initial_list() } else { results };
            }
            Err(_) => {
                self.suggestions = self.initial_list();
            }
        }
    }

    fn current_selection(&self) -> ModelEntry {
        if !self.suggestions.is_empty() && self.suggestions[0].id != EMPTY_QUERY_NOTICE {
            self.suggestions[0].clone()
        } else if !self.all_models.is_empty() {
            self.all_models[0].clone()
        } else {
            ModelEntry {
                id: String::new(),
                name: None,
                context_length: None,
                pricing: None,
                supported_features: vec![],
            }
        }
    }

    fn render(&mut self) {
        if self.rendered_lines > 0 {
            let _ = self.term.clear_last_lines(self.rendered_lines);
        }
        let mut lines = 0;
        let _ = self.term.write_line(&format!("> {}", self.query));
        lines += 1;
        for entry in &self.suggestions {
            if entry.id == EMPTY_QUERY_NOTICE {
                let _ = self.term.write_line("(no models found)");
            } else {
                let _ = self.term.write_line(&entry.id);
            }
            lines += 1;
        }
        self.rendered_lines = lines;
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

    fn search_mock_endpoint(server: &httpmock::MockServer, query: &str, models: &[&str], status: u16) -> String {
        let q = query.to_string();
        let models: Vec<String> = models.iter().map(|s| s.to_string()).collect();
        server.mock(move |when, then| {
            when.method(httpmock::Method::GET)
                .path("/models")
                .query_param("search", &q);
            let data: Vec<serde_json::Value> = models.iter().map(|id| serde_json::json!({"id": id})).collect();
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
        let all_models = vec![
            ModelEntry { id: "gpt-4o".into(), name: None, context_length: None, pricing: None, supported_features: vec![] },
        ];
        let (results, error, no_results) =
            execute_search(&endpoint, None, "gpt", &all_models, &gen, current).unwrap();
        assert_eq!(error.as_deref(), Some("model search is not supported by this provider"));
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
