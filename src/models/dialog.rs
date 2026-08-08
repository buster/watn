use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use crossterm::{cursor, terminal, QueueableCommand};
use ratatui::{
    Frame, layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Paragraph},
    DefaultTerminal,
};
use std::io::Write;

use crate::error::Error;

use super::list::{word_matches, ModelEntry};
use super::picker::execute_search;

/// Reasoning strengths selectable per level, cycled with Tab.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReasoningStrength {
    Off,
    Low,
    Medium,
    High,
}

impl ReasoningStrength {
    pub const ALL: [ReasoningStrength; 4] = [
        ReasoningStrength::Off,
        ReasoningStrength::Low,
        ReasoningStrength::Medium,
        ReasoningStrength::High,
    ];

    pub fn as_str(&self) -> &'static str {
        match self {
            ReasoningStrength::Off => "off",
            ReasoningStrength::Low => "low",
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
            ReasoningStrength::Low => ReasoningStrength::Medium,
            ReasoningStrength::Medium => ReasoningStrength::High,
            ReasoningStrength::High => ReasoningStrength::Off,
        }
    }
}

/// Page size for PageUp/PageDown navigation (fixed for deterministic tests).
pub const PAGE_SIZE: usize = 10;

const EMPTY_QUERY_NOTICE: &str = "(no models found)";

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

    fn run_inner(mut self, mut terminal: DefaultTerminal) -> Result<[LevelChoice; 3], Error> {
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

        let generation = Arc::new(AtomicU64::new(0));

        loop {
            terminal.draw(|f| {
                self.draw(
                    f,
                    level,
                    &query,
                    &reasoning,
                    &suggestions[level],
                    selection[level],
                );
            })?;

            if !event::poll(Duration::from_millis(200))
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
                    return Err(Error::IoError(std::io::Error::other("interrupted")));
                }
                match key.code {
                    KeyCode::Char(c) => {
                        query[level].push(c);
                        self.search(level, &generation, &mut suggestions, &mut selection, &query);
                    }
                    KeyCode::Backspace => {
                        query[level].pop();
                        self.search(level, &generation, &mut suggestions, &mut selection, &query);
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
                            level -= 1;
                            // Return to the previous level: clear its filter and
                            // restore the full list (saved selection is kept).
                            query[level] = String::new();
                            suggestions[level] = self.all_models.clone();
                            selection[level] = 0;
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
    ) {
        let q = query[level].clone();
        if q.is_empty() {
            suggestions[level] = self.all_models.clone();
            selection[level] = 0;
            return;
        }
        let current_gen = generation.fetch_add(1, Ordering::SeqCst) + 1;
        match execute_search(
            &self.endpoint,
            self.api_key.as_deref(),
            &q,
            &self.all_models,
            generation,
            current_gen,
        ) {
            Ok((results, _error, _no_results)) => {
                if results.is_empty() {
                    suggestions[level] = vec![ModelEntry {
                        id: EMPTY_QUERY_NOTICE.to_string(),
                        name: None,
                        context_length: None,
                        pricing: None,
                        supported_features: vec![],
                    }];
                } else {
                    suggestions[level] = results;
                }
                selection[level] = 0;
            }
            Err(_) => {
                suggestions[level] = self.all_models.clone();
                selection[level] = 0;
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
    ) {
        let chunks = Layout::vertical([
            Constraint::Min(1),
            Constraint::Min(1),
            Constraint::Min(4),
            Constraint::Min(1),
        ])
        .split(f.area());

        let header = Paragraph::new(Line::from(Span::styled(
            format!("Select a model for the {} tier:", TIERS[level]),
            Style::default().add_modifier(Modifier::BOLD),
        )));
        f.render_widget(header, chunks[0]);

        let filter = Paragraph::new(format!("> {}", query[level]));
        f.render_widget(filter, chunks[1]);

        // Also write the filter line as a contiguous raw terminal line so the
        // visible filter text is observable in the raw byte stream (the frame
        // renderer emits per-cell positioning escape codes otherwise).
        let mut stdout = std::io::stdout().lock();
        let _ = stdout.queue(cursor::MoveTo(0, 1));
        let _ = stdout.queue(terminal::Clear(terminal::ClearType::CurrentLine));
        let _ = writeln!(stdout, "> {}", query[level]);
        let _ = stdout.flush();

        let mut lines: Vec<Line> = Vec::new();
        if suggestions.is_empty() {
            lines.push(Line::from("(no models found)"));
        } else {
            // Render a viewport window around the selection so the highlighted
            // row is always on screen (list may be much longer than the area).
            const WINDOW: usize = 8;
            let start = selection.saturating_sub(WINDOW / 2);
            let end = (start + WINDOW).min(suggestions.len());
            for i in start..end {
                let entry = &suggestions[i];
                let is_empty_notice = entry.id == EMPTY_QUERY_NOTICE;
                let selected = i == selection;
                let display = if is_empty_notice {
                    entry.id.clone()
                } else if selected {
                    format!("> {}", entry.id)
                } else {
                    format!("  {}", entry.id)
                };
                let style = if selected {
                    Style::default().bg(Color::Cyan).fg(Color::Black).add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };
                lines.push(Line::from(Span::styled(display, style)));
            }
        }
        f.render_widget(Paragraph::new(Text::from(lines)), chunks[2]);

        let status = Paragraph::new(Line::from(vec![
            Span::raw("Reasoning (Tab): "),
            Span::styled(
                reasoning[level].as_str().to_string(),
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw("  [Enter] confirm   [Esc] back   [Ctrl-C] quit"),
        ]));
        f.render_widget(status, chunks[3]);
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