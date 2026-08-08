use console::{Key, Term};
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
}
