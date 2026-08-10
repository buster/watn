use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::Arc;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout, Margin},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, Cell, List, ListItem, ListState, Paragraph, Row, Scrollbar,
        ScrollbarOrientation, ScrollbarState, Table, TableState, Tabs, Wrap,
    },
};

use crate::config::{self, resolve_provider};
use crate::config::types::Config;
use crate::error::Error;
use crate::models::dialog::{LevelChoice, ReasoningStrength};
use crate::models::list::{fetch_models, fetch_models_page, word_matches, ModelEntry};
use crate::models::picker::execute_search;
use crate::provider::setup::{
    build_provider_draft, suggested_api_key_env, ProviderDraft, SetupCancellation,
    OPENROUTER_ENDPOINT,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SetupEntryPoint {
    Setup,
    Provider,
    Models,
}

#[derive(Debug)]
pub struct SetupWizardResult {
    pub provider: ProviderDraft,
    pub choices: [Option<LevelChoice>; 3],
}

#[derive(Debug)]
pub enum SetupWizardOutcome {
    Saved(Box<SetupWizardResult>),
    Cancelled(SetupCancellation),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum SetupPage {
    Url,
    ApiKey,
    SmallModel,
    MiddleModel,
    LargeModel,
}

impl SetupPage {
    fn title(self) -> &'static str {
        match self {
            Self::Url => "URL",
            Self::ApiKey => "API key",
            Self::SmallModel => "Small Model",
            Self::MiddleModel => "Middle Model",
            Self::LargeModel => "Large Model",
        }
    }

    fn index(self) -> usize {
        match self {
            Self::Url => 0,
            Self::ApiKey => 1,
            Self::SmallModel => 2,
            Self::MiddleModel => 3,
            Self::LargeModel => 4,
        }
    }

    fn model_slot(self) -> Option<usize> {
        match self {
            Self::SmallModel => Some(0),
            Self::MiddleModel => Some(1),
            Self::LargeModel => Some(2),
            _ => None,
        }
    }

    fn next(self) -> Option<Self> {
        match self {
            Self::Url => Some(Self::ApiKey),
            Self::ApiKey => Some(Self::SmallModel),
            Self::SmallModel => Some(Self::MiddleModel),
            Self::MiddleModel => Some(Self::LargeModel),
            Self::LargeModel => None,
        }
    }

    fn previous(self) -> Option<Self> {
        match self {
            Self::Url => None,
            Self::ApiKey => Some(Self::Url),
            Self::SmallModel => Some(Self::ApiKey),
            Self::MiddleModel => Some(Self::SmallModel),
            Self::LargeModel => Some(Self::MiddleModel),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CredentialStorage {
    Configuration,
    Environment,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CredentialFocus {
    Storage,
    Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ModelFocus {
    Table,
    Reasoning,
}

type SearchMessage = (
    usize,
    u64,
    Result<(Vec<ModelEntry>, Option<String>, bool), Error>,
);

pub fn run_with_config(
    config: &Config,
    entry: SetupEntryPoint,
) -> Result<SetupWizardOutcome, Error> {
    let mut wizard = SetupWizard::from_config(config, entry)?;
    if entry == SetupEntryPoint::Models {
        wizard.load_catalog()?;
    }

    let mut terminal = ratatui::init();
    let result = wizard.run_inner(&mut terminal);
    ratatui::restore();
    result
}

pub fn apply_result(
    config: &mut Config,
    result: &SetupWizardResult,
) -> Result<(), Error> {
    config::save_provider_draft(config, &result.provider)?;

    let mut updated = config.clone();
    let mut changed_tiers = false;
    for (index, choice) in result.choices.iter().enumerate() {
        let Some(choice) = choice else {
            continue;
        };
        changed_tiers = true;
        match index {
            0 => {
                updated.tiers.small = Some(choice.model.id.clone());
                updated.tiers.reasoning.small = Some(choice.reasoning.as_str().to_string());
            }
            1 => {
                updated.tiers.normal = Some(choice.model.id.clone());
                updated.tiers.reasoning.normal = Some(choice.reasoning.as_str().to_string());
            }
            2 => {
                updated.tiers.thinking = Some(choice.model.id.clone());
                updated.tiers.reasoning.thinking = Some(choice.reasoning.as_str().to_string());
            }
            _ => unreachable!(),
        }
    }
    if changed_tiers {
        config::save_config(&updated)?;
        *config = updated;
    }
    Ok(())
}

struct SetupWizard {
    page: SetupPage,
    first_page: SetupPage,
    last_page: SetupPage,
    endpoint: String,
    storage: CredentialStorage,
    credential_input: String,
    credential_focus: CredentialFocus,
    models: [Vec<ModelEntry>; 3],
    queries: [String; 3],
    suggestions: [Vec<ModelEntry>; 3],
    selection: [usize; 3],
    completed: [Option<LevelChoice>; 3],
    reasoning: [ReasoningStrength; 3],
    reasoning_explicit: [bool; 3],
    model_focus: ModelFocus,
    search_status: [Option<String>; 3],
    search_pending: [bool; 3],
    generation: Arc<AtomicU64>,
    search_tx: Sender<SearchMessage>,
    search_rx: Receiver<SearchMessage>,
    validation: String,
    save_prompt: bool,
    initial_models: [Option<String>; 3],
}

impl SetupWizard {
    fn from_config(config: &Config, entry: SetupEntryPoint) -> Result<Self, Error> {
        let provider_name = config
            .defaults
            .provider
            .as_deref()
            .unwrap_or("openrouter");
        let provider = match resolve_provider(config, provider_name) {
            Ok(provider) => provider,
            Err(_error) if entry != SetupEntryPoint::Models => {
                crate::config::types::ProviderConfig {
                    endpoint: OPENROUTER_ENDPOINT.to_string(),
                    api_key: None,
                    default_model: None,
                }
            }
            Err(error) => return Err(error),
        };
        let (storage, credential_input) = match provider.api_key.as_deref() {
            Some(value) if value.starts_with("${") && value.ends_with('}') => (
                CredentialStorage::Environment,
                value[2..value.len() - 1].to_string(),
            ),
            Some(value) => (CredentialStorage::Configuration, value.to_string()),
            None => (
                CredentialStorage::Configuration,
                String::new(),
            ),
        };
        let first_page = match entry {
            SetupEntryPoint::Models => SetupPage::SmallModel,
            SetupEntryPoint::Setup | SetupEntryPoint::Provider => SetupPage::Url,
        };
        let last_page = match entry {
            SetupEntryPoint::Provider => SetupPage::ApiKey,
            SetupEntryPoint::Setup | SetupEntryPoint::Models => SetupPage::LargeModel,
        };
        let initial_models = [
            config.tiers.small.clone(),
            config.tiers.normal.clone(),
            config.tiers.thinking.clone(),
        ];
        let reasoning = [
            parse_reasoning(config.tiers.reasoning.small.as_deref()),
            parse_reasoning(config.tiers.reasoning.normal.as_deref()),
            parse_reasoning(config.tiers.reasoning.thinking.as_deref()),
        ];
        let reasoning_explicit = [
            config.tiers.reasoning.small.is_some(),
            config.tiers.reasoning.normal.is_some(),
            config.tiers.reasoning.thinking.is_some(),
        ];
        let generation = Arc::new(AtomicU64::new(0));
        let (search_tx, search_rx) = mpsc::channel();
        let mut wizard = Self {
            page: first_page,
            first_page,
            last_page,
            endpoint: (if provider.endpoint.is_empty() {
                OPENROUTER_ENDPOINT.to_string()
            } else {
                provider.endpoint
            }),
            storage,
            credential_input,
            credential_focus: if first_page >= SetupPage::SmallModel {
                CredentialFocus::Value
            } else {
                CredentialFocus::Storage
            },
            models: [Vec::new(), Vec::new(), Vec::new()],
            queries: Default::default(),
            suggestions: [Vec::new(), Vec::new(), Vec::new()],
            selection: [0, 0, 0],
            completed: [None, None, None],
            reasoning,
            reasoning_explicit,
            model_focus: ModelFocus::Table,
            search_status: [None, None, None],
            search_pending: [false, false, false],
            generation,
            search_tx,
            search_rx,
            validation: String::new(),
            save_prompt: false,
            initial_models,
        };
        if wizard.storage == CredentialStorage::Environment && wizard.credential_input.is_empty() {
            wizard.credential_input = suggested_api_key_env(&wizard.endpoint).to_string();
        }
        Ok(wizard)
    }

    fn run_inner(&mut self, terminal: &mut DefaultTerminal) -> Result<SetupWizardOutcome, Error> {
        loop {
            self.apply_search_results();
            terminal.draw(|frame| self.draw(frame))?;
            if !event::poll(Duration::from_millis(50))
                .map_err(|error| Error::IoError(std::io::Error::other(error)))?
            {
                continue;
            }
            let Event::Key(key) = event::read()
                .map_err(|error| Error::IoError(std::io::Error::other(error)))?
            else {
                continue;
            };
            if key.kind != KeyEventKind::Press {
                continue;
            }
            if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
                self.generation.fetch_add(1, Ordering::SeqCst);
                return Ok(SetupWizardOutcome::Cancelled(SetupCancellation::CtrlC));
            }
            if key.code == KeyCode::Char('u') && key.modifiers.contains(KeyModifiers::CONTROL) {
                match self.page {
                    SetupPage::Url => self.endpoint.clear(),
                    SetupPage::ApiKey if self.credential_focus == CredentialFocus::Value => {
                        self.credential_input.clear()
                    }
                    _ => {}
                }
                continue;
            }
            if self.save_prompt {
                if let Some(result) = self.handle_save_prompt(key)? {
                    return Ok(result);
                }
                continue;
            }
            if key.code == KeyCode::Esc {
                self.save_prompt = true;
                continue;
            }
            if let Some(result) = self.handle_key(key)? {
                return Ok(result);
            }
        }
    }

    fn handle_save_prompt(&mut self, key: KeyEvent) -> Result<Option<SetupWizardOutcome>, Error> {
        match key.code {
            KeyCode::Char('n') | KeyCode::Char('N') => {
                // Invalidate catalog work before abandoning the draft state.
                self.generation.fetch_add(1, Ordering::SeqCst);
                Ok(Some(SetupWizardOutcome::Cancelled(SetupCancellation::Escape)))
            }
            KeyCode::Char('y') | KeyCode::Char('Y') | KeyCode::Enter => {
                match self.result() {
                    Ok(result) => Ok(Some(SetupWizardOutcome::Saved(Box::new(result)))),
                    Err(error) => {
                        self.save_prompt = false;
                        self.validation = error.to_string();
                        Ok(None)
                    }
                }
            }
            KeyCode::Esc => {
                self.save_prompt = false;
                Ok(None)
            }
            _ => Ok(None),
        }
    }

    fn handle_key(&mut self, key: KeyEvent) -> Result<Option<SetupWizardOutcome>, Error> {
        if key.code == KeyCode::Tab && key.modifiers.contains(KeyModifiers::SHIFT) {
            return self.move_previous();
        }
        if key.code == KeyCode::BackTab {
            return self.move_previous();
        }
        match key.code {
            KeyCode::Tab | KeyCode::Enter => self.move_next(),
            KeyCode::Up => {
                self.move_up();
                Ok(None)
            }
            KeyCode::Down => {
                self.move_down();
                Ok(None)
            }
            KeyCode::PageUp => {
                self.move_page(-1);
                Ok(None)
            }
            KeyCode::PageDown => {
                self.move_page(1);
                Ok(None)
            }
            KeyCode::Backspace => {
                self.edit_input(None);
                Ok(None)
            }
            KeyCode::Char(character) => {
                if character == 'r'
                    && key.modifiers.contains(KeyModifiers::CONTROL)
                    && self.page.model_slot().is_some()
                {
                    self.model_focus = match self.model_focus {
                        ModelFocus::Table => ModelFocus::Reasoning,
                        ModelFocus::Reasoning => ModelFocus::Table,
                    };
                } else if self.page == SetupPage::ApiKey
                    && self.credential_focus == CredentialFocus::Storage
                    && matches!(character, 'p' | 'P' | 'e' | 'E')
                {
                    self.choose_storage(character);
                } else {
                    self.edit_input(Some(character));
                }
                Ok(None)
            }
            _ => Ok(None),
        }
    }

    fn move_next(&mut self) -> Result<Option<SetupWizardOutcome>, Error> {
        if self.page == SetupPage::ApiKey && self.credential_focus == CredentialFocus::Storage {
            self.credential_focus = CredentialFocus::Value;
            self.validation.clear();
            return Ok(None);
        }
        if self.validate_current_page().is_err() {
            return Ok(None);
        }
        if let Some(slot) = self.page.model_slot() {
            if let Err(error) = self.confirm_model(slot) {
                self.validation = error.to_string();
                return Ok(None);
            }
        }
        if self.page == self.last_page {
            return Ok(Some(SetupWizardOutcome::Saved(Box::new(self.result()?))));
        }
        if let Some(next) = self.page.next() {
            if next >= self.first_page && next <= self.last_page {
                self.page = next;
                self.validation.clear();
                self.model_focus = ModelFocus::Table;
                if next.model_slot().is_some() {
                    if let Err(error) = self.ensure_catalog() {
                        self.page = SetupPage::ApiKey;
                        self.credential_focus = CredentialFocus::Value;
                        self.validation = error.to_string();
                        return Ok(None);
                    }
                }
            }
        }
        Ok(None)
    }

    fn move_previous(&mut self) -> Result<Option<SetupWizardOutcome>, Error> {
        if let Some(previous) = self.page.previous() {
            if previous >= self.first_page {
                self.page = previous;
                self.validation.clear();
                if self.page == SetupPage::ApiKey {
                    self.credential_focus = CredentialFocus::Value;
                }
                if let Some(slot) = self.page.model_slot() {
                    self.generation.fetch_add(1, Ordering::SeqCst);
                    self.queries[slot].clear();
                    self.suggestions[slot] = self.models[slot].clone();
                    self.selection[slot] = 0;
                    self.search_pending[slot] = false;
                    self.search_status[slot] = None;
                    self.sync_reasoning(slot);
                }
            }
        }
        Ok(None)
    }

    fn validate_current_page(&mut self) -> Result<(), Error> {
        match self.page {
            SetupPage::Url => {
                self.endpoint = crate::provider::setup::normalize_endpoint(&self.endpoint)
                    .inspect_err(|error| {
                        self.validation = error.to_string();
                    })?;
                if self.storage == CredentialStorage::Environment
                    && self.credential_input.is_empty()
                {
                    self.credential_input = suggested_api_key_env(&self.endpoint).to_string();
                }
            }
            SetupPage::ApiKey => {
                if self.credential_input.trim().is_empty() {
                    self.validation = "credential cannot be empty".to_string();
                    return Err(Error::ConfigError(self.validation.clone()));
                }
                if self.storage == CredentialStorage::Environment
                    && !valid_environment_name(&self.credential_input)
                {
                    self.validation = "environment variable name is invalid".to_string();
                    return Err(Error::ConfigError(self.validation.clone()));
                }
                self.validation.clear();
            }
            _ => {}
        }
        Ok(())
    }

    fn current_provider(&self) -> Result<ProviderDraft, Error> {
        let api_key = match self.storage {
            CredentialStorage::Configuration => self.credential_input.clone(),
            CredentialStorage::Environment => format!("${{{}}}", self.credential_input),
        };
        build_provider_draft(&self.endpoint, &api_key)
    }

    fn request_credential(&self) -> Result<String, Error> {
        let draft = self.current_provider()?;
        config::expand_api_key(&draft.api_key)
    }

    fn result(&self) -> Result<SetupWizardResult, Error> {
        Ok(SetupWizardResult {
            provider: self.current_provider()?,
            choices: self.completed.clone(),
        })
    }

    fn choose_storage(&mut self, character: char) {
        self.storage = if matches!(character, 'e' | 'E') {
            CredentialStorage::Environment
        } else {
            CredentialStorage::Configuration
        };
        if self.storage == CredentialStorage::Environment && self.credential_input.is_empty() {
            self.credential_input = suggested_api_key_env(&self.endpoint).to_string();
        }
        self.credential_focus = CredentialFocus::Value;
    }

    fn edit_input(&mut self, character: Option<char>) {
        match self.page {
            SetupPage::Url => {
                if let Some(character) = character {
                    self.endpoint.push(character);
                } else {
                    self.endpoint.pop();
                }
            }
            SetupPage::ApiKey if self.credential_focus == CredentialFocus::Value => {
                if let Some(character) = character {
                    self.credential_input.push(character);
                } else {
                    self.credential_input.pop();
                }
            }
            SetupPage::SmallModel | SetupPage::MiddleModel | SetupPage::LargeModel
                if self.model_focus == ModelFocus::Table =>
            {
                let Some(slot) = self.page.model_slot() else { return };
                if let Some(character) = character {
                    self.queries[slot].push(character);
                } else {
                    self.queries[slot].pop();
                }
                self.search(slot);
            }
            _ => {}
        }
    }

    fn move_up(&mut self) {
        if self.page == SetupPage::ApiKey && self.credential_focus == CredentialFocus::Storage {
            self.storage = CredentialStorage::Configuration;
            return;
        }
        let Some(slot) = self.page.model_slot() else { return };
        if self.model_focus == ModelFocus::Reasoning {
            self.cycle_reasoning(slot, -1);
        } else {
            self.selection[slot] = self.selection[slot].saturating_sub(1);
            self.sync_reasoning(slot);
        }
    }

    fn move_down(&mut self) {
        if self.page == SetupPage::ApiKey && self.credential_focus == CredentialFocus::Storage {
            self.storage = CredentialStorage::Environment;
            if self.credential_input.is_empty() {
                self.credential_input = suggested_api_key_env(&self.endpoint).to_string();
            }
            return;
        }
        let Some(slot) = self.page.model_slot() else { return };
        if self.model_focus == ModelFocus::Reasoning {
            self.cycle_reasoning(slot, 1);
        } else if !self.suggestions[slot].is_empty() {
            self.selection[slot] = (self.selection[slot] + 1).min(self.suggestions[slot].len() - 1);
            self.sync_reasoning(slot);
        }
    }

    fn move_page(&mut self, direction: i32) {
        let Some(slot) = self.page.model_slot() else { return };
        if self.model_focus == ModelFocus::Reasoning {
            return;
        }
        if self.suggestions[slot].is_empty() {
            return;
        }
        let current = self.selection[slot] as i32;
        let last = self.suggestions[slot].len().saturating_sub(1) as i32;
        self.selection[slot] = (current + direction * 10).clamp(0, last) as usize;
        self.sync_reasoning(slot);
    }

    fn confirm_model(&mut self, slot: usize) -> Result<(), Error> {
        if self.search_pending[slot] || self.suggestions[slot].is_empty() {
            self.validation = "wait for a model result before continuing".to_string();
            return Err(Error::ConfigError(self.validation.clone()));
        }
        self.completed[slot] = Some(LevelChoice {
            model: self.suggestions[slot][self.selection[slot]].clone(),
            reasoning: self.reasoning[slot],
        });
        Ok(())
    }

    fn ensure_catalog(&mut self) -> Result<(), Error> {
        if !self.models[0].is_empty() {
            return Ok(());
        }
        self.load_catalog()
    }

    fn load_catalog(&mut self) -> Result<(), Error> {
        let key = self.request_credential().inspect_err(|error| {
            self.validation = error.to_string();
        })?;
        let models = match fetch_models_page(&self.endpoint, 1, 50, Some(&key)) {
            Ok(models) if !models.is_empty() => models,
            _ => fetch_models(&self.endpoint, Some(&key))?,
        };
        if models.is_empty() {
            self.validation = "no models returned from endpoint".to_string();
            return Err(Error::ConfigError(self.validation.clone()));
        }
        for slot in 0..3 {
            self.models[slot] = models.clone();
            self.suggestions[slot] = models.clone();
            if let Some(initial) = &self.initial_models[slot] {
                if let Some(index) = models.iter().position(|model| &model.id == initial) {
                    self.selection[slot] = index;
                }
            }
            self.sync_reasoning(slot);
        }
        Ok(())
    }

    fn search(&mut self, slot: usize) {
        let query = self.queries[slot].clone();
        let generation = self.generation.fetch_add(1, Ordering::SeqCst) + 1;
        if query.is_empty() {
            self.suggestions[slot] = self.models[slot].clone();
            self.selection[slot] = 0;
            self.search_pending[slot] = false;
            self.search_status[slot] = None;
            self.sync_reasoning(slot);
            return;
        }
        self.suggestions[slot].clear();
        self.selection[slot] = 0;
        self.search_pending[slot] = true;
        self.search_status[slot] = Some("Searching...".to_string());
        let endpoint = self.endpoint.clone();
        let key = self.request_credential().ok();
        let models = self.models[slot].clone();
        let generation_ref = Arc::clone(&self.generation);
        let sender = self.search_tx.clone();
        std::thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(200));
            if generation_ref.load(Ordering::SeqCst) != generation {
                return;
            }
            let result = execute_search(
                &endpoint,
                key.as_deref(),
                &query,
                &models,
                &generation_ref,
                generation,
            );
            if generation_ref.load(Ordering::SeqCst) == generation {
                let _ = sender.send((slot, generation, result));
            }
        });
    }

    fn apply_search_results(&mut self) {
        while let Ok((slot, generation, result)) = self.search_rx.try_recv() {
            if self.generation.load(Ordering::SeqCst) != generation {
                continue;
            }
            self.search_pending[slot] = false;
            match result {
                Ok((models, error, no_results)) => {
                    self.suggestions[slot] = models;
                    self.selection[slot] = 0;
                    self.search_status[slot] = error.or_else(|| {
                        no_results.then(|| "(no models found)".to_string())
                    });
                }
                Err(error) => {
                    let filtered = self.models[slot]
                        .iter()
                        .filter(|model| word_matches(&model.id, &self.queries[slot]))
                        .cloned()
                        .collect::<Vec<_>>();
                    self.suggestions[slot] = if filtered.is_empty() {
                        self.models[slot].clone()
                    } else {
                        filtered
                    };
                    self.selection[slot] = 0;
                    self.search_status[slot] = Some(error.to_string());
                }
            }
            self.sync_reasoning(slot);
        }
    }

    fn reasoning_options(&self, slot: usize) -> Vec<ReasoningStrength> {
        let Some(model) = self.suggestions[slot].get(self.selection[slot]) else {
            return vec![ReasoningStrength::Off];
        };
        let Some(metadata) = &model.reasoning else {
            return vec![
                ReasoningStrength::Off,
                ReasoningStrength::Low,
                ReasoningStrength::Medium,
                ReasoningStrength::High,
            ];
        };
        let mut options = Vec::new();
        if !metadata.mandatory {
            options.push(ReasoningStrength::Off);
        }
        for effort in &metadata.supported_efforts {
            if let Some(value) = ReasoningStrength::parse(effort) {
                if !options.contains(&value) {
                    options.push(value);
                }
            }
        }
        if options.is_empty() {
            options.push(ReasoningStrength::Off);
        }
        options
    }

    fn sync_reasoning(&mut self, slot: usize) {
        let options = self.reasoning_options(slot);
        if !self.reasoning_explicit[slot] || !options.contains(&self.reasoning[slot]) {
            self.reasoning[slot] = self
                .suggestions[slot]
                .get(self.selection[slot])
                .and_then(|model| model.reasoning.as_ref())
                .and_then(|metadata| metadata.default_effort.as_deref())
                .and_then(ReasoningStrength::parse)
                .filter(|value| options.contains(value))
                .unwrap_or(options[0]);
        }
    }

    fn cycle_reasoning(&mut self, slot: usize, direction: i32) {
        let options = self.reasoning_options(slot);
        let current = options
            .iter()
            .position(|value| *value == self.reasoning[slot])
            .unwrap_or(0) as i32;
        let next = (current + direction).rem_euclid(options.len() as i32) as usize;
        self.reasoning[slot] = options[next];
        self.reasoning_explicit[slot] = true;
    }

    fn draw(&self, frame: &mut Frame) {
        let panel = Block::bordered().title("Setup");
        let areas = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(2),
            Constraint::Min(8),
            Constraint::Length(3),
        ])
        .split(panel.inner(frame.area()));
        frame.render_widget(panel, frame.area());

        let tabs = Tabs::new([
            Line::from("URL"),
            Line::from("API key"),
            Line::from("Small Model"),
            Line::from("Middle Model"),
            Line::from("Large Model"),
        ])
        .block(Block::bordered().title("Setup pages"))
        .select(self.page.index())
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .divider(Span::raw(" | "));
        frame.render_widget(tabs, areas[0]);

        let focus = match self.page.model_slot() {
            Some(_) => match self.model_focus {
                ModelFocus::Table => "model table",
                ModelFocus::Reasoning => "reasoning",
            },
            None => match self.credential_focus {
                CredentialFocus::Storage => "storage choice",
                CredentialFocus::Value => "input",
            },
        };
        let header = Paragraph::new(format!(
            "Page {} of 5  |  {}  |  Focus: {}",
            self.page.index() + 1,
            self.page.title(),
            focus
        ));
        frame.render_widget(header, areas[1]);

        match self.page {
            SetupPage::Url => self.draw_url(frame, areas[2]),
            SetupPage::ApiKey => self.draw_api_key(frame, areas[2]),
            SetupPage::SmallModel | SetupPage::MiddleModel | SetupPage::LargeModel => {
                self.draw_model(frame, areas[2])
            }
        }

        let footer = if self.save_prompt {
            "Save current settings? [y] Save [n] Discard  [Esc] Return"
        } else {
            "Enter/Tab next  Shift-Tab back  Ctrl-R reasoning  Esc save/discard  Ctrl-C quit"
        };
        let footer = Paragraph::new(footer)
            .block(Block::bordered().title("Controls"))
            .wrap(Wrap { trim: true });
        frame.render_widget(footer, areas[3]);
    }

    fn draw_url(&self, frame: &mut Frame, area: ratatui::layout::Rect) {
        let chunks = Layout::vertical([
            Constraint::Min(3),
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .split(area);
        let explanation = Paragraph::new(
            "Enter an OpenAI/LiteLLM compatible endpoint. The service must expose the standard chat and model APIs.",
        )
        .block(Block::bordered().title("Endpoint explanation"))
        .wrap(Wrap { trim: true });
        frame.render_widget(explanation, chunks[0]);
        let input = Paragraph::new(format!("> {}█", self.endpoint))
            .block(Block::bordered().title("URL (editing)"));
        frame.render_widget(input, chunks[1]);
        self.draw_validation(frame, chunks[2]);
    }

    fn draw_api_key(&self, frame: &mut Frame, area: ratatui::layout::Rect) {
        let chunks = Layout::vertical([
            Constraint::Length(5),
            Constraint::Length(3),
            Constraint::Min(3),
        ])
        .split(area);
        let items = vec![
            ListItem::new(Line::from("Configuration (Paste credential)")),
            ListItem::new(Line::from("Environment variable (store name)")),
        ];
        let mut state = ListState::default();
        state.select(Some(match self.storage {
            CredentialStorage::Configuration => 0,
            CredentialStorage::Environment => 1,
        }));
        let list = List::new(items)
            .block(Block::bordered().title("Where should the API key be stored?"))
            .highlight_style(
                Style::default()
                    .bg(Color::Cyan)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("> ");
        frame.render_stateful_widget(list, chunks[0], &mut state);
        let value = if self.storage == CredentialStorage::Configuration {
            "*".repeat(self.credential_input.chars().count())
        } else {
            self.credential_input.clone()
        };
        let input = Paragraph::new(format!("> {}█", value))
            .block(Block::bordered().title("API key / environment name (editing)"));
        frame.render_widget(input, chunks[1]);
        self.draw_validation(frame, chunks[2]);
    }

    fn draw_model(&self, frame: &mut Frame, area: ratatui::layout::Rect) {
        let Some(slot) = self.page.model_slot() else { return };
        let chunks = Layout::vertical([Constraint::Min(7), Constraint::Length(3)]).split(area);
        let rows = self.suggestions[slot].iter().enumerate().map(|(index, model)| {
            let label = if index == self.selection[slot] {
                format!("> {}", model.id)
            } else {
                model.id.clone()
            };
            Row::new([
                Cell::from(label),
                Cell::from(model.context_length.map(|value| format!("{}K", value / 1000)).unwrap_or_else(|| "-".to_string())),
                Cell::from(model.pricing.as_ref().map(|value| format!("${:.2}/${:.2}", value.input, value.output)).unwrap_or_else(|| "-".to_string())),
                Cell::from(model.supported_features.join(", ")),
            ])
        });
        let mut table_state = TableState::default();
        if !self.suggestions[slot].is_empty() {
            table_state.select(Some(self.selection[slot].min(self.suggestions[slot].len() - 1)));
        }
        let table = Table::new(
            rows,
            [Constraint::Percentage(52), Constraint::Percentage(15), Constraint::Percentage(18), Constraint::Min(0)],
        )
        .header(Row::new(["Model", "Context", "Pricing", "Features"]).style(Style::default().add_modifier(Modifier::BOLD)))
        .block(Block::bordered().title(format!("{} (editing)", self.page.title())))
        .row_highlight_style(Style::default().bg(Color::Cyan).fg(Color::Black).add_modifier(Modifier::BOLD))
        .highlight_symbol("");
        frame.render_stateful_widget(table, chunks[0], &mut table_state);
        let visible_rows = chunks[0].height.saturating_sub(4) as usize;
        if self.suggestions[slot].len() > visible_rows.max(1) {
            let mut scrollbar_state = ScrollbarState::new(self.suggestions[slot].len())
                .position(self.selection[slot]);
            let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .thumb_symbol("#")
                .track_symbol(Some("."));
            frame.render_stateful_widget(
                scrollbar,
                chunks[0].inner(Margin {
                    horizontal: 1,
                    vertical: 1,
                }),
                &mut scrollbar_state,
            );
        }
        let options = self.reasoning_options(slot);
        let options = options.iter().map(ReasoningStrength::as_str).collect::<Vec<_>>().join(", ");
        let reasoning = Paragraph::new(format!(
            "Reasoning {}: {}  |  Supported: {}{}",
            if self.model_focus == ModelFocus::Reasoning { "(focused)" } else { "" },
            self.reasoning[slot].as_str(),
            options,
            if self.suggestions[slot].get(self.selection[slot]).and_then(|model| model.reasoning.as_ref()).map(|reasoning| reasoning.mandatory).unwrap_or(false) { "  | mandatory" } else { "" }
        ))
        .block(Block::bordered().title("Model reasoning"))
        .wrap(Wrap { trim: true });
        frame.render_widget(reasoning, chunks[1]);
    }

    fn draw_validation(&self, frame: &mut Frame, area: ratatui::layout::Rect) {
        let text = if self.validation.is_empty() {
            "The active line is marked with █. Enter or Tab advances the wizard.".to_string()
        } else {
            format!("Validation: {}", self.validation)
        };
        frame.render_widget(Paragraph::new(text).wrap(Wrap { trim: true }), area);
    }
}

fn parse_reasoning(value: Option<&str>) -> ReasoningStrength {
    value
        .and_then(ReasoningStrength::parse)
        .unwrap_or(ReasoningStrength::Off)
}

fn valid_environment_name(name: &str) -> bool {
    !name.is_empty()
        && name.chars().enumerate().all(|(index, character)| {
            (index == 0 && (character == '_' || character.is_ascii_uppercase()))
                || (index > 0
                    && (character == '_'
                        || character.is_ascii_uppercase()
                        || character.is_ascii_digit()))
        })
}
