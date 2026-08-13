use std::collections::HashSet;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::Arc;
use std::thread::JoinHandle;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    layout::{Constraint, Layout, Margin},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, Cell, List, ListItem, ListState, Paragraph, Row, Scrollbar, ScrollbarOrientation,
        ScrollbarState, Table, TableState, Tabs, Wrap,
    },
    DefaultTerminal, Frame,
};

use crate::config::types::Config;
use crate::config::{self, resolve_provider};
use crate::error::Error;
use crate::models::dialog::{LevelChoice, ReasoningStrength};
use crate::models::list::{fetch_models, fetch_models_page_info, word_matches, ModelEntry};
use crate::models::picker::execute_search;
use crate::provider::setup::{
    build_provider_draft, suggested_api_key_env, ProviderDraft, SetupCancellation,
    OPENROUTER_ENDPOINT,
};
use crate::shell_completion;
use crate::shell_shortcut::{self, Shell};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SetupEntryPoint {
    Setup,
    Provider,
    Models,
    Shell,
}

#[derive(Debug)]
pub struct SetupWizardResult {
    pub provider: ProviderDraft,
    pub choices: [Option<LevelChoice>; 3],
    pub completion_shells: Vec<Shell>,
    pub shortcut_shells: Vec<Shell>,
}

#[derive(Debug)]
pub enum SetupWizardOutcome {
    Saved(Box<SetupWizardResult>),
    Cancelled(SetupCancellation),
}

pub fn selected_shells(enabled: bool, selected: [bool; 3]) -> Vec<Shell> {
    if !enabled {
        return Vec::new();
    }
    Shell::ALL
        .into_iter()
        .enumerate()
        .filter_map(|(index, shell)| selected[index].then_some(shell))
        .collect()
}

pub fn selected_shortcut_shells(enabled: bool, selected: [bool; 3]) -> Vec<Shell> {
    selected_shells(enabled, selected)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum SetupPage {
    Provider,
    Url,
    ApiKey,
    SmallModel,
    SmallReasoning,
    NormalModel,
    NormalReasoning,
    ThinkingModel,
    ThinkingReasoning,
    ShellCompletion,
    ShellShortcut,
}

impl SetupPage {
    fn title(self) -> &'static str {
        match self {
            Self::Provider => "Provider",
            Self::Url => "URL",
            Self::ApiKey => "API key",
            Self::SmallModel => "Small Model",
            Self::SmallReasoning => "Small Reasoning",
            Self::NormalModel => "Normal Model",
            Self::NormalReasoning => "Normal Reasoning",
            Self::ThinkingModel => "Thinking Model",
            Self::ThinkingReasoning => "Thinking Reasoning",
            Self::ShellCompletion => "Shell Completion",
            Self::ShellShortcut => "Shell Shortcut",
        }
    }

    fn index(self) -> usize {
        match self {
            Self::Provider => 0,
            Self::Url => 1,
            Self::ApiKey => 2,
            Self::SmallModel => 3,
            Self::SmallReasoning => 4,
            Self::NormalModel => 5,
            Self::NormalReasoning => 6,
            Self::ThinkingModel => 7,
            Self::ThinkingReasoning => 8,
            Self::ShellCompletion => 9,
            Self::ShellShortcut => 10,
        }
    }

    fn model_slot(self) -> Option<usize> {
        match self {
            Self::Provider => None,
            Self::SmallModel => Some(0),
            Self::NormalModel => Some(1),
            Self::ThinkingModel => Some(2),
            _ => None,
        }
    }

    fn reasoning_slot(self) -> Option<usize> {
        match self {
            Self::Provider => None,
            Self::SmallReasoning => Some(0),
            Self::NormalReasoning => Some(1),
            Self::ThinkingReasoning => Some(2),
            _ => None,
        }
    }

    fn next(self) -> Option<Self> {
        match self {
            Self::Provider => Some(Self::Url),
            Self::Url => Some(Self::ApiKey),
            Self::ApiKey => Some(Self::SmallModel),
            Self::SmallModel => Some(Self::SmallReasoning),
            Self::SmallReasoning => Some(Self::NormalModel),
            Self::NormalModel => Some(Self::NormalReasoning),
            Self::NormalReasoning => Some(Self::ThinkingModel),
            Self::ThinkingModel => Some(Self::ThinkingReasoning),
            Self::ThinkingReasoning => Some(Self::ShellCompletion),
            Self::ShellCompletion => Some(Self::ShellShortcut),
            Self::ShellShortcut => None,
        }
    }

    fn previous(self) -> Option<Self> {
        match self {
            Self::Provider => None,
            Self::Url => None,
            Self::ApiKey => Some(Self::Url),
            Self::SmallModel => Some(Self::ApiKey),
            Self::SmallReasoning => Some(Self::SmallModel),
            Self::NormalModel => Some(Self::SmallReasoning),
            Self::NormalReasoning => Some(Self::NormalModel),
            Self::ThinkingModel => Some(Self::NormalReasoning),
            Self::ThinkingReasoning => Some(Self::ThinkingModel),
            Self::ShellCompletion => Some(Self::ThinkingReasoning),
            Self::ShellShortcut => Some(Self::ShellCompletion),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ShellInstallFocus {
    Question,
    Shells,
}

type SearchMessage = (
    usize,
    u64,
    Result<(Vec<ModelEntry>, Option<String>, bool), Error>,
);

const CATALOG_PAGE_LIMIT: u32 = 50;

fn setup_block<'a>(title: impl Into<Line<'a>>, focused: bool) -> Block<'a> {
    let block = Block::bordered().title(title);
    if focused {
        block.border_style(Style::default().fg(Color::Green))
    } else {
        block
    }
}

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

pub fn apply_result(config: &mut Config, result: &SetupWizardResult) -> Result<(), Error> {
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
                updated.tiers.reasoning.small = Some(choice.reasoning.clone());
            }
            1 => {
                updated.tiers.normal = Some(choice.model.id.clone());
                updated.tiers.reasoning.normal = Some(choice.reasoning.clone());
            }
            2 => {
                updated.tiers.thinking = Some(choice.model.id.clone());
                updated.tiers.reasoning.thinking = Some(choice.reasoning.clone());
            }
            _ => unreachable!(),
        }
    }
    if changed_tiers {
        config::save_config(&updated)?;
        *config = updated;
    }

    let environment = shell_shortcut::ShellEnvironment::from_process();
    let mut installation_failures = Vec::new();
    if !result.completion_shells.is_empty() {
        let report =
            shell_completion::install_with_environment(&result.completion_shells, &environment);
        for target in &report.results {
            if target.success {
                eprintln!("{}", target.message);
                if let Some(reload) = &target.reload {
                    eprintln!("{}", reload);
                }
            } else {
                eprintln!("{}", target.message);
            }
        }
        if let Some(error) = report.failure_message() {
            installation_failures.push(error);
        }
    }
    if !result.shortcut_shells.is_empty() {
        let report =
            shell_shortcut::install_with_environment(&result.shortcut_shells, &environment);
        for target in &report.results {
            if target.success {
                eprintln!("{}", target.message);
                if let Some(reload) = &target.reload {
                    eprintln!("{}", reload);
                }
            } else {
                eprintln!("{}", target.message);
            }
        }
        if let Some(error) = report.failure_message() {
            installation_failures.push(error);
        }
    }
    if !installation_failures.is_empty() {
        return Err(Error::ConfigError(installation_failures.join("; ")));
    }
    Ok(())
}

pub fn apply_shell_result(result: &SetupWizardResult) -> Result<(), Error> {
    let environment = shell_shortcut::ShellEnvironment::from_process();
    let selected_completion = &result.completion_shells;
    let selected_shortcut = &result.shortcut_shells;
    let mut failures = Vec::new();

    for shell in Shell::ALL {
        if selected_completion.contains(&shell) {
            if let Some(error) =
                shell_completion::install_with_environment(&[shell], &environment).aggregate_error()
            {
                failures.push(error.to_string());
            }
        } else if let Some(error) =
            shell_completion::remove_with_environment(&[shell], &environment).aggregate_error()
        {
            failures.push(error.to_string());
        }

        if selected_shortcut.contains(&shell) {
            if let Some(error) =
                shell_shortcut::install_with_environment(&[shell], &environment).aggregate_error()
            {
                failures.push(error.to_string());
            }
        } else if let Some(error) =
            shell_shortcut::remove_with_environment(&[shell], &environment).aggregate_error()
        {
            failures.push(error.to_string());
        }
    }

    if failures.is_empty() {
        Ok(())
    } else {
        Err(Error::ConfigError(failures.join("; ")))
    }
}

struct SetupWizard {
    config: Config,
    provider_name: String,
    provider_cursor: usize,
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
    manual_models: [String; 3],
    completed: [Option<LevelChoice>; 3],
    reasoning: [ReasoningStrength; 3],
    custom_reasoning: [Option<String>; 3],
    reasoning_explicit: [bool; 3],
    model_focus: ModelFocus,
    search_status: [Option<String>; 3],
    search_pending: [bool; 3],
    catalog_complete: bool,
    catalog_manual: bool,
    generation: Arc<AtomicU64>,
    search_tx: Sender<SearchMessage>,
    search_rx: Receiver<SearchMessage>,
    search_workers: Vec<JoinHandle<()>>,
    validation: String,
    save_prompt: bool,
    initial_models: [Option<String>; 3],
    completion_focus: ShellInstallFocus,
    completion_enabled: bool,
    completion_cursor: usize,
    completion_selected: [bool; 3],
    shortcut_focus: ShellInstallFocus,
    shortcut_enabled: bool,
    shortcut_cursor: usize,
    shortcut_selected: [bool; 3],
}

impl Drop for SetupWizard {
    fn drop(&mut self) {
        self.generation.fetch_add(1, Ordering::SeqCst);
        for worker in self.search_workers.drain(..) {
            let _ = worker.join();
        }
    }
}

impl SetupWizard {
    fn from_config(config: &Config, entry: SetupEntryPoint) -> Result<Self, Error> {
        let provider_name = config.defaults.provider.as_deref().unwrap_or("openrouter");
        let provider = match resolve_provider(config, provider_name) {
            Ok(provider) => provider,
            Err(_error) if entry != SetupEntryPoint::Models => {
                crate::config::types::ProviderConfig {
                    endpoint: OPENROUTER_ENDPOINT.to_string(),
                    api_key: None,
                    default_model: None,
                    catalog_endpoint: None,
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
            None if entry == SetupEntryPoint::Models
                && provider.endpoint == OPENROUTER_ENDPOINT =>
            {
                (
                    CredentialStorage::Environment,
                    suggested_api_key_env(&provider.endpoint).to_string(),
                )
            }
            None => (CredentialStorage::Configuration, String::new()),
        };
        let first_page = match entry {
            SetupEntryPoint::Models => SetupPage::SmallModel,
            SetupEntryPoint::Shell => SetupPage::ShellCompletion,
            SetupEntryPoint::Setup | SetupEntryPoint::Provider => SetupPage::Provider,
        };
        let last_page = match entry {
            SetupEntryPoint::Provider => SetupPage::ApiKey,
            SetupEntryPoint::Shell => SetupPage::ShellShortcut,
            SetupEntryPoint::Setup => SetupPage::ShellShortcut,
            SetupEntryPoint::Models => SetupPage::ThinkingReasoning,
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
        let shell_environment = shell_shortcut::ShellEnvironment::from_process();
        let detected_shells = shell_environment.detected_shells();
        let completion_selected = if entry == SetupEntryPoint::Shell {
            Shell::ALL.map(|shell| {
                shell_completion::has_managed_block_with_environment(shell, &shell_environment)
            })
        } else {
            Shell::ALL.map(|shell| detected_shells.contains(&shell))
        };
        let shortcut_selected = if entry == SetupEntryPoint::Shell {
            Shell::ALL.map(|shell| {
                shell_shortcut::has_managed_block_with_environment(shell, &shell_environment)
            })
        } else {
            Shell::ALL.map(|shell| detected_shells.contains(&shell))
        };
        let generation = Arc::new(AtomicU64::new(0));
        let (search_tx, search_rx) = mpsc::channel();
        let mut wizard = Self {
            config: config.clone(),
            provider_name: provider_name.to_string(),
            provider_cursor: match provider_name {
                "openrouter" => 0,
                "openai" => 1,
                _ => 2,
            },
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
            manual_models: Default::default(),
            completed: [None, None, None],
            reasoning,
            custom_reasoning: [None, None, None],
            reasoning_explicit,
            model_focus: ModelFocus::Table,
            search_status: [None, None, None],
            search_pending: [false, false, false],
            catalog_complete: false,
            catalog_manual: false,
            generation,
            search_tx,
            search_rx,
            search_workers: Vec::new(),
            validation: String::new(),
            save_prompt: false,
            initial_models,
            completion_focus: ShellInstallFocus::Question,
            completion_enabled: false,
            completion_cursor: 0,
            completion_selected,
            shortcut_focus: ShellInstallFocus::Question,
            shortcut_enabled: false,
            shortcut_cursor: 0,
            shortcut_selected,
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
            let Event::Key(key) =
                event::read().map_err(|error| Error::IoError(std::io::Error::other(error)))?
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
            if self.shell_install_page_active() {
                if let Some(result) = self.handle_shell_install_key(key)? {
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
                Ok(Some(SetupWizardOutcome::Cancelled(
                    SetupCancellation::Escape,
                )))
            }
            KeyCode::Char('y') | KeyCode::Char('Y') | KeyCode::Enter => match self.result() {
                Ok(result) => Ok(Some(SetupWizardOutcome::Saved(Box::new(result)))),
                Err(error) => {
                    self.save_prompt = false;
                    self.validation = error.to_string();
                    Ok(None)
                }
            },
            KeyCode::Esc => {
                self.save_prompt = false;
                Ok(None)
            }
            _ => Ok(None),
        }
    }

    fn shell_install_page_active(&self) -> bool {
        matches!(
            self.page,
            SetupPage::ShellCompletion | SetupPage::ShellShortcut
        )
    }

    fn handle_shell_install_key(
        &mut self,
        key: KeyEvent,
    ) -> Result<Option<SetupWizardOutcome>, Error> {
        if key.code == KeyCode::Tab && key.modifiers.contains(KeyModifiers::SHIFT)
            || key.code == KeyCode::BackTab
        {
            return self.move_previous();
        }
        if key.code == KeyCode::Esc {
            self.save_prompt = true;
            return Ok(None);
        }

        let shortcut = self.page == SetupPage::ShellShortcut;
        let advance = if shortcut {
            Self::handle_shell_install_key_inner(
                key,
                &mut self.shortcut_focus,
                &mut self.shortcut_enabled,
                &mut self.shortcut_cursor,
                &mut self.shortcut_selected,
            )
        } else {
            Self::handle_shell_install_key_inner(
                key,
                &mut self.completion_focus,
                &mut self.completion_enabled,
                &mut self.completion_cursor,
                &mut self.completion_selected,
            )
        };
        if advance {
            self.advance_shell_install_page()
        } else {
            Ok(None)
        }
    }

    fn handle_shell_install_key_inner(
        key: KeyEvent,
        focus: &mut ShellInstallFocus,
        enabled: &mut bool,
        cursor: &mut usize,
        selected: &mut [bool; 3],
    ) -> bool {
        match focus {
            ShellInstallFocus::Question => match key.code {
                KeyCode::Char('y') | KeyCode::Char('Y') => {
                    *enabled = true;
                    *focus = ShellInstallFocus::Shells;
                    *cursor = 0;
                    false
                }
                KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Enter | KeyCode::Tab => {
                    *enabled = false;
                    true
                }
                _ => false,
            },
            ShellInstallFocus::Shells => match key.code {
                KeyCode::Up => {
                    *cursor = cursor.saturating_sub(1);
                    false
                }
                KeyCode::Down => {
                    *cursor = (*cursor + 1).min(Shell::ALL.len() - 1);
                    false
                }
                KeyCode::Char(' ') => {
                    selected[*cursor] = !selected[*cursor];
                    false
                }
                KeyCode::Enter | KeyCode::Tab => true,
                _ => false,
            },
        }
    }

    fn advance_shell_install_page(&mut self) -> Result<Option<SetupWizardOutcome>, Error> {
        match self.page {
            SetupPage::ShellCompletion => {
                self.page = SetupPage::ShellShortcut;
                self.validation.clear();
                Ok(None)
            }
            SetupPage::ShellShortcut => {
                Ok(Some(SetupWizardOutcome::Saved(Box::new(self.result()?))))
            }
            _ => unreachable!("shell installation page handler used on another page"),
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
                } else if let Some(slot) = self.page.reasoning_slot() {
                    if matches!(character, 'c' | 'C') {
                        self.custom_reasoning[slot] = Some(String::new());
                        self.reasoning_explicit[slot] = true;
                    } else {
                        self.edit_input(Some(character));
                    }
                } else {
                    self.edit_input(Some(character));
                }
                Ok(None)
            }
            _ => Ok(None),
        }
    }

    fn move_next(&mut self) -> Result<Option<SetupWizardOutcome>, Error> {
        if self.page == SetupPage::Provider {
            self.provider_name = provider_choices()[self.provider_cursor].to_string();
            if self.provider_name == "custom"
                && self.config.defaults.provider.as_deref() != Some("custom")
                && self.endpoint == OPENROUTER_ENDPOINT
            {
                self.endpoint.clear();
            }
        }
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
        if let Some(slot) = self.page.reasoning_slot() {
            if let Some(custom) = &self.custom_reasoning[slot] {
                if custom.trim().is_empty() {
                    self.validation = "reasoning effort cannot be empty".to_string();
                    return Ok(None);
                }
            }
            self.reasoning_explicit[slot] = true;
            let value = self.reasoning_value(slot);
            if let Some(choice) = self.completed[slot].as_mut() {
                choice.reasoning = value;
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
        let provider = if self.first_page == SetupPage::ShellCompletion {
            ProviderDraft {
                name: "custom".to_string(),
                endpoint: String::new(),
                api_key: String::new(),
            }
        } else {
            self.current_provider()?
        };
        Ok(SetupWizardResult {
            provider,
            choices: self.completed.clone(),
            completion_shells: selected_shells(self.completion_enabled, self.completion_selected),
            shortcut_shells: selected_shells(self.shortcut_enabled, self.shortcut_selected),
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
            SetupPage::SmallModel | SetupPage::NormalModel | SetupPage::ThinkingModel
                if self.model_focus == ModelFocus::Table =>
            {
                let Some(slot) = self.page.model_slot() else {
                    return;
                };
                if self.catalog_manual {
                    if let Some(character) = character {
                        self.manual_models[slot].push(character);
                    } else {
                        self.manual_models[slot].pop();
                    }
                    return;
                }
                if let Some(character) = character {
                    self.queries[slot].push(character);
                } else {
                    self.queries[slot].pop();
                }
                self.search(slot);
            }
            SetupPage::SmallReasoning
            | SetupPage::NormalReasoning
            | SetupPage::ThinkingReasoning => {
                let Some(slot) = self.page.reasoning_slot() else {
                    return;
                };
                if self.custom_reasoning[slot].is_some() {
                    let input = self.custom_reasoning[slot].get_or_insert_default();
                    if let Some(character) = character {
                        input.push(character);
                    } else {
                        input.pop();
                    }
                }
            }
            _ => {}
        }
    }

    fn move_up(&mut self) {
        if self.page == SetupPage::Provider {
            self.provider_cursor = self.provider_cursor.saturating_sub(1);
            return;
        }
        if self.page == SetupPage::ApiKey && self.credential_focus == CredentialFocus::Storage {
            self.storage = CredentialStorage::Configuration;
            return;
        }
        let Some(slot) = self
            .page
            .reasoning_slot()
            .or_else(|| self.page.model_slot())
        else {
            return;
        };
        if self.page.reasoning_slot().is_some() || self.model_focus == ModelFocus::Reasoning {
            self.cycle_reasoning(slot, -1);
        } else {
            self.selection[slot] = self.selection[slot].saturating_sub(1);
            self.sync_reasoning(slot);
        }
    }

    fn move_down(&mut self) {
        if self.page == SetupPage::Provider {
            self.provider_cursor = (self.provider_cursor + 1).min(provider_choices().len() - 1);
            return;
        }
        if self.page == SetupPage::ApiKey && self.credential_focus == CredentialFocus::Storage {
            self.storage = CredentialStorage::Environment;
            if self.credential_input.is_empty() {
                self.credential_input = suggested_api_key_env(&self.endpoint).to_string();
            }
            return;
        }
        let Some(slot) = self
            .page
            .reasoning_slot()
            .or_else(|| self.page.model_slot())
        else {
            return;
        };
        if self.page.reasoning_slot().is_some() || self.model_focus == ModelFocus::Reasoning {
            self.cycle_reasoning(slot, 1);
        } else if !self.suggestions[slot].is_empty() {
            self.selection[slot] = (self.selection[slot] + 1).min(self.suggestions[slot].len() - 1);
            self.sync_reasoning(slot);
        }
    }

    fn move_page(&mut self, direction: i32) {
        let Some(slot) = self.page.model_slot() else {
            return;
        };
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
        if self.catalog_manual {
            let model = self.manual_models[slot].trim();
            if model.is_empty() {
                self.validation = "enter a non-empty manual model identifier".to_string();
                return Err(Error::ConfigError(self.validation.clone()));
            }
            self.completed[slot] = Some(LevelChoice {
                model: ModelEntry {
                    id: model.to_string(),
                    name: None,
                    context_length: None,
                    pricing: None,
                    supported_features: Vec::new(),
                    reasoning: None,
                },
                reasoning: self.reasoning_value(slot),
            });
            return Ok(());
        }
        if self.search_pending[slot] || self.suggestions[slot].is_empty() {
            self.validation = "wait for a model result before continuing".to_string();
            return Err(Error::ConfigError(self.validation.clone()));
        }
        self.completed[slot] = Some(LevelChoice {
            model: self.suggestions[slot][self.selection[slot]].clone(),
            reasoning: self.reasoning_value(slot),
        });
        Ok(())
    }

    fn reasoning_value(&self, slot: usize) -> String {
        self.custom_reasoning[slot]
            .clone()
            .unwrap_or_else(|| self.reasoning[slot].as_str().to_string())
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
        let page = match fetch_models_page_info(&self.endpoint, 1, CATALOG_PAGE_LIMIT, Some(&key)) {
            Ok(page) if !page.models.is_empty() => Ok((page.models, page.complete)),
            Ok(_) => fetch_models(&self.endpoint, Some(&key)).map(|models| (models, true)),
            Err(error) => Err(error),
        };
        let (models, catalog_complete) = match page {
            Ok(value) if !value.0.is_empty() => value,
            Ok(_) => {
                self.catalog_manual = true;
                self.validation =
                    "Catalog discovery unavailable. Enter a manual model identifier.".to_string();
                return Ok(());
            }
            Err(error) => {
                self.catalog_manual = true;
                self.validation = format!(
                    "Catalog discovery unavailable. Enter a manual model identifier: {error}"
                );
                return Ok(());
            }
        };
        let mut identifiers = HashSet::new();
        if models
            .iter()
            .any(|model| model.id.trim().is_empty() || !identifiers.insert(model.id.clone()))
        {
            self.catalog_manual = true;
            self.validation =
                "Catalog discovery unavailable. Model identifiers must be unique and non-empty."
                    .to_string();
            return Ok(());
        }
        self.catalog_complete = catalog_complete;
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
        if self.catalog_complete {
            self.suggestions[slot] = self.models[slot]
                .iter()
                .filter(|model| word_matches(&model.id, &query))
                .cloned()
                .collect();
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
        self.reap_search_workers();
        let worker = std::thread::spawn(move || {
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
        self.search_workers.push(worker);
    }

    fn reap_search_workers(&mut self) {
        let mut active = Vec::new();
        for worker in self.search_workers.drain(..) {
            if worker.is_finished() {
                let _ = worker.join();
            } else {
                active.push(worker);
            }
        }
        self.search_workers = active;
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
                    self.search_status[slot] =
                        error.or_else(|| no_results.then(|| "(no models found)".to_string()));
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
                ReasoningStrength::Minimal,
                ReasoningStrength::Medium,
                ReasoningStrength::High,
            ];
        };
        let mut options = Vec::new();
        for effort in &metadata.supported_efforts {
            if let Some(value) = ReasoningStrength::parse(effort) {
                if !options.contains(&value) {
                    options.push(value);
                }
            }
        }
        if !metadata.mandatory
            && metadata
                .supported_efforts
                .iter()
                .any(|effort| effort == "off")
            && !options.contains(&ReasoningStrength::Off)
        {
            options.insert(0, ReasoningStrength::Off);
        }
        if options.is_empty() {
            options.push(ReasoningStrength::Off);
        }
        options
    }

    fn sync_reasoning(&mut self, slot: usize) {
        let options = self.reasoning_options(slot);
        if !self.reasoning_explicit[slot] || !options.contains(&self.reasoning[slot]) {
            self.reasoning[slot] = self.suggestions[slot]
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
        self.custom_reasoning[slot] = None;
        self.reasoning_explicit[slot] = true;
        let value = self.reasoning_value(slot);
        if let Some(choice) = self.completed[slot].as_mut() {
            choice.reasoning = value;
        }
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

        let mut tab_titles = vec![
            Line::from("Provider"),
            Line::from("URL"),
            Line::from("API key"),
            Line::from("Small Model"),
            Line::from("Small Reasoning"),
            Line::from("Normal Model"),
            Line::from("Normal Reasoning"),
            Line::from("Thinking Model"),
            Line::from("Thinking Reasoning"),
            Line::from("Shell Completion"),
            Line::from("Shell Shortcut"),
        ];
        tab_titles.truncate(self.last_page.index() + 1);
        let tabs = Tabs::new(tab_titles)
            .block(Block::bordered().title("Setup pages"))
            .select(self.page.index())
            .highlight_style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )
            .divider(Span::raw(" | "));
        frame.render_widget(tabs, areas[0]);

        let focus = match self.page {
            SetupPage::Provider => "provider choice",
            SetupPage::ShellCompletion => match self.completion_focus {
                ShellInstallFocus::Question => "completion confirmation",
                ShellInstallFocus::Shells => "completion shells",
            },
            SetupPage::ShellShortcut => match self.shortcut_focus {
                ShellInstallFocus::Question => "shortcut confirmation",
                ShellInstallFocus::Shells => "shortcut shells",
            },
            _ => {
                if self.page.reasoning_slot().is_some() {
                    "reasoning"
                } else {
                    match self.page.model_slot() {
                        Some(_) => match self.model_focus {
                            ModelFocus::Table => "model table",
                            ModelFocus::Reasoning => "reasoning",
                        },
                        None => match self.credential_focus {
                            CredentialFocus::Storage => "storage choice",
                            CredentialFocus::Value => "input",
                        },
                    }
                }
            }
        };
        let header = Paragraph::new(format!(
            "Page {} of {}  |  {}  |  Focus: {}",
            self.page.index() + 1,
            self.last_page.index() + 1,
            self.page.title(),
            focus
        ));
        frame.render_widget(header, areas[1]);

        match self.page {
            SetupPage::Provider => self.draw_provider(frame, areas[2]),
            SetupPage::Url => self.draw_url(frame, areas[2]),
            SetupPage::ApiKey => self.draw_api_key(frame, areas[2]),
            SetupPage::SmallModel | SetupPage::NormalModel | SetupPage::ThinkingModel => {
                self.draw_model(frame, areas[2]);
            }
            SetupPage::SmallReasoning
            | SetupPage::NormalReasoning
            | SetupPage::ThinkingReasoning => self.draw_reasoning(frame, areas[2]),
            SetupPage::ShellCompletion => self.draw_shell_install(frame, areas[2], false),
            SetupPage::ShellShortcut => self.draw_shell_install(frame, areas[2], true),
        }

        let footer = if self.save_prompt {
            "Save current settings? [y] Save [n] Discard  [Esc] Return"
        } else {
            match self.page {
                SetupPage::ShellCompletion => match self.completion_focus {
                    ShellInstallFocus::Question => {
                        "[y] Install completion  [Enter] Skip  [Esc] save/discard"
                    }
                    ShellInstallFocus::Shells => {
                        "Up/Down move  Space toggle  Enter continue  Esc save/discard"
                    }
                },
                SetupPage::ShellShortcut => match self.shortcut_focus {
                    ShellInstallFocus::Question => {
                        "[y] Install shortcut  [Enter] Skip  [Esc] save/discard"
                    }
                    ShellInstallFocus::Shells => {
                        "Up/Down move  Space toggle  Enter finish  Esc save/discard"
                    }
                },
                _ if self.page.model_slot().is_some() => {
                    "Enter/Tab next  Shift-Tab back  Esc save/discard  Ctrl-C quit"
                }
                _ => {
                    "Up/Down choose  Enter/Tab next  Shift-Tab back  Esc save/discard  Ctrl-C quit"
                }
            }
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
            .block(setup_block("URL (editing)", true));
        frame.render_widget(input, chunks[1]);
        self.draw_validation(frame, chunks[2]);
    }

    fn draw_provider(&self, frame: &mut Frame, area: ratatui::layout::Rect) {
        let choices = provider_choices();
        let items = choices.iter().map(|provider| {
            ListItem::new(match *provider {
                "openrouter" => "OpenRouter",
                "openai" => "OpenAI",
                _ => "Custom",
            })
        });
        let mut state = ListState::default();
        state.select(Some(self.provider_cursor));
        let list = List::new(items)
            .block(setup_block("Provider (editing)", true))
            .highlight_style(
                Style::default()
                    .bg(Color::Cyan)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("> ");
        frame.render_stateful_widget(list, area, &mut state);
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
            .block(setup_block(
                "Where should the API key be stored?",
                self.credential_focus == CredentialFocus::Storage,
            ))
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
        let input = Paragraph::new(format!("> {}█", value)).block(setup_block(
            "API key / environment name (editing)",
            self.credential_focus == CredentialFocus::Value,
        ));
        frame.render_widget(input, chunks[1]);
        self.draw_validation(frame, chunks[2]);
    }

    fn draw_model(&self, frame: &mut Frame, area: ratatui::layout::Rect) {
        let Some(slot) = self.page.model_slot() else {
            return;
        };
        let chunks = Layout::vertical([Constraint::Min(7)]).split(area);
        let rows = self.suggestions[slot]
            .iter()
            .enumerate()
            .map(|(index, model)| {
                let label = if index == self.selection[slot] {
                    format!("> {}", model.id)
                } else {
                    model.id.clone()
                };
                Row::new([
                    Cell::from(label),
                    Cell::from(
                        model
                            .context_length
                            .map(|value| format!("{}K", value / 1000))
                            .unwrap_or_else(|| "-".to_string()),
                    ),
                    Cell::from(
                        model
                            .pricing
                            .as_ref()
                            .map(|value| format!("${:.2}/${:.2}", value.input, value.output))
                            .unwrap_or_else(|| "-".to_string()),
                    ),
                    Cell::from(model.supported_features.join(", ")),
                ])
            })
            .chain(self.catalog_manual.then(|| {
                Row::new([
                    Cell::from("Catalog discovery unavailable"),
                    Cell::from(""),
                    Cell::from(""),
                    Cell::from(self.validation.clone()),
                ])
            }))
            .chain(self.catalog_manual.then(|| {
                Row::new([
                    Cell::from("Manual model identifier"),
                    Cell::from(""),
                    Cell::from(""),
                    Cell::from("type a model id"),
                ])
            }));
        let mut table_state = TableState::default();
        if !self.suggestions[slot].is_empty() {
            table_state.select(Some(
                self.selection[slot].min(self.suggestions[slot].len() - 1),
            ));
        }
        let title = if self.catalog_manual {
            format!("{} (manual entry)", self.page.title())
        } else if self.queries[slot].is_empty() {
            format!("{} (editing)", self.page.title())
        } else {
            format!(
                "{} (editing) | Filter: {}",
                self.page.title(),
                self.queries[slot]
            )
        };
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
        .block(setup_block(title, self.model_focus == ModelFocus::Table))
        .row_highlight_style(
            Style::default()
                .bg(Color::Cyan)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("");
        frame.render_stateful_widget(table, chunks[0], &mut table_state);
        let visible_rows = chunks[0].height.saturating_sub(4) as usize;
        if self.suggestions[slot].len() > visible_rows.max(1) {
            let mut scrollbar_state =
                ScrollbarState::new(self.suggestions[slot].len()).position(self.selection[slot]);
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
    }

    fn draw_reasoning(&self, frame: &mut Frame, area: ratatui::layout::Rect) {
        let Some(slot) = self.page.reasoning_slot() else {
            return;
        };
        let model = self
            .completed
            .get(slot)
            .and_then(Option::as_ref)
            .map(|choice| choice.model.id.as_str())
            .or_else(|| {
                self.suggestions[slot]
                    .get(self.selection[slot])
                    .map(|model| model.id.as_str())
            })
            .unwrap_or("(no model selected)");
        let options = self
            .reasoning_options(slot)
            .iter()
            .map(ReasoningStrength::as_str)
            .collect::<Vec<_>>()
            .join(", ");
        let metadata_notice = self.suggestions[slot]
            .get(self.selection[slot])
            .and_then(|model| model.reasoning.as_ref())
            .is_none()
            .then_some("Notice: reasoning metadata unavailable")
            .unwrap_or("");
        let text = format!(
            "Model: {}\n{}\nChoose reasoning effort with Up/Down, then press Enter.\nSelected: {}\nChoices: {}\nCustom: {}",
            model,
            metadata_notice,
            self.reasoning_value(slot),
            options,
            self.custom_reasoning[slot]
                .as_deref()
                .unwrap_or("press c to enter a custom effort")
        );
        frame.render_widget(
            Paragraph::new(text)
                .block(setup_block(self.page.title(), true))
                .wrap(Wrap { trim: true }),
            area,
        );
    }

    fn draw_shell_install(&self, frame: &mut Frame, area: ratatui::layout::Rect, shortcut: bool) {
        let (focus, enabled, cursor, selected) = if shortcut {
            (
                self.shortcut_focus,
                self.shortcut_enabled,
                self.shortcut_cursor,
                &self.shortcut_selected,
            )
        } else {
            (
                self.completion_focus,
                self.completion_enabled,
                self.completion_cursor,
                &self.completion_selected,
            )
        };
        let (title, description, question) = if shortcut {
            (
                "Shell shortcut",
                "Install a Ctrl-W widget in each selected shell startup file. Type a natural-language request, press Ctrl-W, review or edit the generated command, then press Enter. The command is never executed automatically.",
                if enabled {
                    "Select the shells where the Ctrl-W widget should be installed."
                } else {
                    "Install the Ctrl-W shell shortcut for watn?"
                },
            )
        } else {
            (
                "Shell completion",
                "Install watn's generated Tab completion in each selected shell startup file. After reloading the file, type watn and press Tab to complete options and subcommands.",
                if enabled {
                    "Select the shells where watn completion should be installed."
                } else {
                    "Install shell completion for watn?"
                },
            )
        };
        let chunks = Layout::vertical([Constraint::Length(7), Constraint::Min(5)]).split(area);
        let explanation = Paragraph::new(format!("{}\n\n{}", description, question))
            .block(setup_block(title, focus == ShellInstallFocus::Question))
            .wrap(Wrap { trim: true });
        frame.render_widget(explanation, chunks[0]);

        let items = Shell::ALL.iter().enumerate().map(|(index, shell)| {
            let marker = if selected[index] { "[x]" } else { "[ ]" };
            ListItem::new(format!("{} {}", marker, shell.name()))
        });
        let mut state = ListState::default();
        state.select(Some(cursor));
        let list = List::new(items)
            .block(setup_block(
                "Select shells",
                focus == ShellInstallFocus::Shells,
            ))
            .highlight_style(
                Style::default()
                    .bg(Color::Cyan)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("> ");
        frame.render_stateful_widget(list, chunks[1], &mut state);
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

fn provider_choices() -> [&'static str; 3] {
    ["openrouter", "openai", "custom"]
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
