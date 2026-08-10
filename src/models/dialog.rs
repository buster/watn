use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::Arc;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Margin},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, Cell, Paragraph, Row, Scrollbar, ScrollbarOrientation, ScrollbarState, Table,
        TableState, Tabs, Wrap,
    },
    DefaultTerminal,
};

use crate::error::Error;

use super::list::{word_matches, ModelEntry};
use super::picker::execute_search;

/// Reasoning strengths selectable per level, cycled with Tab.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReasoningStrength {
    Off,
    Low,
    Minimal,
    Medium,
    High,
}

impl ReasoningStrength {
    pub const ALL: [ReasoningStrength; 5] = [
        ReasoningStrength::Off,
        ReasoningStrength::Low,
        ReasoningStrength::Minimal,
        ReasoningStrength::Medium,
        ReasoningStrength::High,
    ];

    pub fn as_str(&self) -> &'static str {
        match self {
            ReasoningStrength::Off => "off",
            ReasoningStrength::Low => "low",
            ReasoningStrength::Minimal => "minimal",
            ReasoningStrength::Medium => "medium",
            ReasoningStrength::High => "high",
        }
    }

    pub fn parse(s: &str) -> Option<ReasoningStrength> {
        Self::ALL.iter().copied().find(|r| r.as_str() == s)
    }

    fn next(&self) -> ReasoningStrength {
        match self {
            ReasoningStrength::Off => ReasoningStrength::Low,
            ReasoningStrength::Low => ReasoningStrength::Minimal,
            ReasoningStrength::Minimal => ReasoningStrength::Medium,
            ReasoningStrength::Medium => ReasoningStrength::High,
            ReasoningStrength::High => ReasoningStrength::Off,
        }
    }
}

/// Page size for PageUp/PageDown navigation (fixed for deterministic tests).
pub const PAGE_SIZE: usize = 10;

const EMPTY_QUERY_NOTICE: &str = "(no models found)";

type SearchMessage = (
    usize,
    u64,
    Result<(Vec<ModelEntry>, Option<String>, bool), Error>,
);

/// Per-level result of the dialog: chosen model + reasoning strength.
#[derive(Debug, Clone)]
pub struct LevelChoice {
    pub model: ModelEntry,
    pub reasoning: ReasoningStrength,
}

/// A keyboard-driven dialog reviewing the three tiers (small, normal,
/// thinking) in a guided sequence. For each level the user picks a model from
/// the filterable list and chooses a reasoning strength; Enter advances,
/// Escape returns to the previous level.
pub struct SettingsDialog {
    pub endpoint: String,
    pub api_key: Option<String>,
    pub all_models: Vec<ModelEntry>,
    pub initial_reasoning: [ReasoningStrength; 3],
}

const TIERS: [&str; 3] = ["small", "normal", "thinking"];

impl SettingsDialog {
    pub fn new(
        endpoint: String,
        api_key: Option<String>,
        all_models: Vec<ModelEntry>,
        initial_reasoning: [ReasoningStrength; 3],
    ) -> Self {
        Self {
            endpoint,
            api_key,
            all_models,
            initial_reasoning,
        }
    }

    pub fn run(self) -> Result<[LevelChoice; 3], Error> {
        let terminal = ratatui::init();
        let result = self.run_inner(terminal);
        ratatui::restore();
        result
    }

    fn run_inner(self, mut terminal: DefaultTerminal) -> Result<[LevelChoice; 3], Error> {
        let mut level = 0usize;
        // Per-level: filter text, selection index, reasoning strength,
        // and the confirmed model once Enter has accepted this level.
        let mut query: [String; 3] = Default::default();
        let mut reasoning = self.initial_reasoning;
        let mut confirmed: [Option<ModelEntry>; 3] = [None, None, None];
        let mut suggestions: [Vec<ModelEntry>; 3] = [
            self.all_models.clone(),
            self.all_models.clone(),
            self.all_models.clone(),
        ];
        let mut selection: [usize; 3] = [0, 0, 0];
        let mut table_states = [
            TableState::default(),
            TableState::default(),
            TableState::default(),
        ];
        let mut search_status: [Option<String>; 3] = [None, None, None];
        let mut search_pending = [false; 3];

        let generation = Arc::new(AtomicU64::new(0));
        let (search_tx, search_rx) = mpsc::channel();

        loop {
            self.apply_search_results(
                &generation,
                &mut suggestions,
                &mut selection,
                &mut search_status,
                &mut search_pending,
                &search_rx,
            );
            terminal.draw(|f| {
                self.draw(
                    f,
                    level,
                    &query,
                    &reasoning,
                    &suggestions[level],
                    selection[level],
                    &mut table_states[level],
                    search_status[level].as_deref(),
                    search_pending[level],
                );
            })?;

            if !event::poll(Duration::from_millis(50))
                .map_err(|e| Error::IoError(std::io::Error::other(e)))?
            {
                continue;
            }
            let ev = event::read().map_err(|e| Error::IoError(std::io::Error::other(e)))?;

            if let Event::Key(key) = ev {
                if key.kind != KeyEventKind::Press {
                    continue;
                }
                // Ctrl-C: exit.
                if key.code == KeyCode::Char('c')
                    && key.modifiers.contains(KeyModifiers::CONTROL)
                {
                    generation.fetch_add(1, Ordering::SeqCst);
                    return Err(Error::IoError(std::io::Error::other("interrupted")));
                }
                match key.code {
                    KeyCode::Char(c) => {
                        query[level].push(c);
                        self.search(
                            level,
                            &generation,
                            &mut suggestions,
                            &mut selection,
                            &query,
                            &search_tx,
                            &mut search_status,
                            &mut search_pending,
                        );
                    }
                    KeyCode::Backspace => {
                        query[level].pop();
                        self.search(
                            level,
                            &generation,
                            &mut suggestions,
                            &mut selection,
                            &query,
                            &search_tx,
                            &mut search_status,
                            &mut search_pending,
                        );
                    }
                    KeyCode::Up => {
                        if selection[level] > 0 {
                            selection[level] -= 1;
                        }
                    }
                    KeyCode::Down => {
                        let len = suggestions[level].len();
                        if !suggestions[level].is_empty() && selection[level] + 1 < len {
                            selection[level] += 1;
                        }
                    }
                    KeyCode::PageUp => {
                        selection[level] = selection[level].saturating_sub(PAGE_SIZE);
                    }
                    KeyCode::PageDown => {
                        let len = suggestions[level].len();
                        selection[level] =
                            (selection[level] + PAGE_SIZE).min(len.saturating_sub(1));
                    }
                    KeyCode::Tab => {
                        reasoning[level] = reasoning[level].next();
                    }
                    KeyCode::Enter => {
                        if search_pending[level] || suggestions[level].is_empty() {
                            continue;
                        }
                        if level < 2 {
                            // Confirm this level; keep the filter/selection for
                            // back-navigation, then advance.
                            confirmed[level] = Some(self.current_selection(
                                &suggestions[level],
                                &self.all_models,
                                selection[level],
                            ));
                            level += 1;
                        } else {
                            confirmed[level] = Some(self.current_selection(
                                &suggestions[level],
                                &self.all_models,
                                selection[level],
                            ));
                            break;
                        }
                    }
                    KeyCode::Esc => {
                        if level > 0 {
                            generation.fetch_add(1, Ordering::SeqCst);
                            level -= 1;
                            // Return to the previous level: clear its filter and
                            // restore the full list (saved selection is kept).
                            query[level] = String::new();
                            suggestions[level] = self.all_models.clone();
                            selection[level] = 0;
                            search_status[level] = None;
                            search_pending[level] = false;
                        }
                    }
                    _ => {}
                }
            }
        }

        let empty = || ModelEntry {
            id: String::new(),
            name: None,
        context_length: None,
        pricing: None,
        supported_features: vec![],
        reasoning: None,
        };

        let choices: [LevelChoice; 3] = std::array::from_fn(|i| LevelChoice {
            model: confirmed[i].clone().unwrap_or_else(|| empty()),
            reasoning: reasoning[i],
        });
        Ok(choices)
    }

    fn search(
        &self,
        level: usize,
        generation: &Arc<AtomicU64>,
        suggestions: &mut [Vec<ModelEntry>; 3],
        selection: &mut [usize; 3],
        query: &[String; 3],
        search_tx: &Sender<SearchMessage>,
        search_status: &mut [Option<String>; 3],
        search_pending: &mut [bool; 3],
    ) {
        let q = query[level].clone();
        let current_gen = generation.fetch_add(1, Ordering::SeqCst) + 1;
        if q.is_empty() {
            suggestions[level] = self.all_models.clone();
            selection[level] = 0;
            search_status[level] = None;
            search_pending[level] = false;
            return;
        }
        suggestions[level].clear();
        selection[level] = 0;
        search_status[level] = Some("Searching...".to_string());
        search_pending[level] = true;

        let endpoint = self.endpoint.clone();
        let api_key = self.api_key.clone();
        let all_models = self.all_models.clone();
        let generation = Arc::clone(generation);
        let search_tx = search_tx.clone();
        std::thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(200));
            if generation.load(Ordering::SeqCst) != current_gen {
                return;
            }
            let result = execute_search(
                &endpoint,
                api_key.as_deref(),
                &q,
                &all_models,
                &generation,
                current_gen,
            );
            if generation.load(Ordering::SeqCst) == current_gen {
                let _ = search_tx.send((level, current_gen, result));
            }
        });
    }

    fn apply_search_results(
        &self,
        generation: &Arc<AtomicU64>,
        suggestions: &mut [Vec<ModelEntry>; 3],
        selection: &mut [usize; 3],
        search_status: &mut [Option<String>; 3],
        search_pending: &mut [bool; 3],
        search_rx: &Receiver<SearchMessage>,
    ) {
        while let Ok((level, result_gen, result)) = search_rx.try_recv() {
            if generation.load(Ordering::SeqCst) != result_gen {
                continue;
            }
            search_pending[level] = false;
            match result {
                Ok((results, error, no_results)) => {
                    suggestions[level] = results;
                    selection[level] = 0;
                    search_status[level] = error.or_else(|| {
                        no_results.then(|| EMPTY_QUERY_NOTICE.to_string())
                    });
                }
                Err(error) => {
                    suggestions[level] = self.all_models.clone();
                    selection[level] = 0;
                    search_status[level] = Some(error.to_string());
                }
            }
        }
    }

    fn current_selection(
        &self,
        suggestions: &[ModelEntry],
        all_models: &[ModelEntry],
        index: usize,
    ) -> ModelEntry {
        if !suggestions.is_empty() && suggestions[0].id != EMPTY_QUERY_NOTICE {
            suggestions
                .get(index)
                .cloned()
                .or_else(|| all_models.first().cloned())
                .unwrap_or(ModelEntry {
                    id: String::new(),
                    name: None,
                    context_length: None,
                    pricing: None,
                    supported_features: vec![],
                    reasoning: None,
                })
        } else if !all_models.is_empty() {
            all_models[0].clone()
        } else {
            ModelEntry {
                id: String::new(),
                name: None,
                context_length: None,
                pricing: None,
                supported_features: vec![],
                reasoning: None,
            }
        }
    }

    fn draw(
        &self,
        f: &mut Frame,
        level: usize,
        query: &[String; 3],
        reasoning: &[ReasoningStrength; 3],
        suggestions: &[ModelEntry],
        selection: usize,
        table_state: &mut TableState,
        search_status: Option<&str>,
        search_pending: bool,
    ) {
        let panel = Block::bordered().title("Model picker");
        let chunks = Layout::vertical([
            Constraint::Length(2),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(6),
            Constraint::Length(3),
        ])
        .split(panel.inner(f.area()));
        f.render_widget(panel, f.area());

        let header = Paragraph::new(Line::from(vec![
            Span::styled(
                format!("Select a model for the {} tier:", TIERS[level]),
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw(format!("  Active tier: {}", TIERS[level])),
        ]));
        f.render_widget(header, chunks[0]);

        let tabs = Tabs::new(
            TIERS
                .iter()
                .map(|tier| Line::from(*tier))
                .collect::<Vec<_>>(),
        )
        .block(Block::bordered().title("Tiers"))
        .select(level)
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .divider(Span::raw(" | "));
        f.render_widget(tabs, chunks[1]);

        let filter = Paragraph::new(format!("> {}", query[level]))
            .block(Block::bordered().title("Filter"))
            .wrap(Wrap { trim: true });
        f.render_widget(filter, chunks[2]);

        let rows = suggestions.iter().enumerate().map(|(index, entry)| {
            let label = if index == selection {
                format!("> {}", model_label(entry))
            } else {
                model_label(entry)
            };
            Row::new([
                Cell::from(label),
                Cell::from(model_context(entry)),
                Cell::from(model_pricing(entry)),
                Cell::from(model_features(entry)),
            ])
        });
        if !suggestions.is_empty() {
            table_state.select(Some(selection.min(suggestions.len() - 1)));
        } else {
            table_state.select(None);
        }
        let table = Table::new(
            rows,
            [
                Constraint::Percentage(52),
                Constraint::Percentage(15),
                Constraint::Percentage(18),
                Constraint::Min(0),
            ],
        )
        .header(
            Row::new(["Model", "Context", "Pricing", "Features"])
                .style(Style::default().add_modifier(Modifier::BOLD)),
        )
        .block(Block::bordered().title("Models"))
        .row_highlight_style(
            Style::default()
                .bg(Color::Cyan)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("");
        f.render_stateful_widget(table, chunks[3], table_state);

        let visible_rows = chunks[3].height.saturating_sub(4) as usize;
        if !suggestions.is_empty() && suggestions.len() > visible_rows.max(1) {
            let mut scrollbar_state = ScrollbarState::new(suggestions.len())
                .position(selection.min(suggestions.len() - 1));
            let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .thumb_symbol("#")
                .track_symbol(Some("."));
            f.render_stateful_widget(
                scrollbar,
                chunks[3].inner(Margin {
                    horizontal: 1,
                    vertical: 1,
                }),
                &mut scrollbar_state,
            );
        }

        let mut status_text = if search_pending {
            "Searching...".to_string()
        } else {
            search_status.unwrap_or_default().to_string()
        };
        if !status_text.is_empty() {
            status_text.push_str("  |  ");
        }
        status_text.push_str(&format!(
            "Reasoning (Tab): {}  [Enter] confirm  [Esc] back  [Ctrl-C] quit",
            reasoning[level].as_str()
        ));
        let status = Paragraph::new(status_text)
            .block(Block::bordered().title("Status"))
            .wrap(Wrap { trim: true });
        f.render_widget(status, chunks[4]);
    }
}

fn model_label(entry: &ModelEntry) -> String {
    match &entry.name {
        Some(name) => format!("{} ({name})", entry.id),
        None => entry.id.clone(),
    }
}

fn model_context(entry: &ModelEntry) -> String {
    entry
        .context_length
        .map(|length| format!("{}K", length / 1_000))
        .unwrap_or_else(|| "-".to_string())
}

fn model_pricing(entry: &ModelEntry) -> String {
    entry
        .pricing
        .as_ref()
        .map(|pricing| format!("${:.2} / ${:.2}", pricing.input, pricing.output))
        .unwrap_or_else(|| "-".to_string())
}

fn model_features(entry: &ModelEntry) -> String {
    if entry.supported_features.is_empty() {
        "-".to_string()
    } else {
        entry.supported_features.join(", ")
    }
}

/// Local fallback filter used by the dialog when remote search is unavailable.
/// Exposed for tests.
pub fn dialog_local_filter(models: &[ModelEntry], query: &str) -> Vec<ModelEntry> {
    models
        .iter()
        .filter(|m| word_matches(&m.id, query))
        .cloned()
        .collect()
}
