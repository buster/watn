use std::fmt;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::Arc;
use std::thread::JoinHandle;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Tabs, Wrap},
    DefaultTerminal, Frame,
};

use crate::config::env::{discover_credentials, env_present, CredentialCandidate};
use crate::config::types::Config;
use crate::config::{self, PersistedConfig};
use crate::error::Error;
use crate::models::dialog::{LevelChoice, ReasoningStrength};
use crate::models::list::{fetch_models, word_matches, ModelEntry};
use crate::provider::setup::{
    build_provider_draft_for_identity, normalize_endpoint, ProviderDraft, ProviderIdentity,
    SetupCancellation, OPENROUTER_ENDPOINT,
};
use crate::shell_completion;
use crate::shell_shortcut::{self, BlockIntent, BlockState, Shell, ShellEnvironment};

const WIDE_LAYOUT_COLUMNS: u16 = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SetupEntryPoint {
    Setup,
    Provider,
    Models,
}

pub struct SetupWizardResult {
    pub config: Config,
    pub provider: ProviderDraft,
    pub choices: [Option<LevelChoice>; 3],
    pub completion_shells: Vec<Shell>,
    pub shortcut_shells: Vec<Shell>,
    pub completion_remove_shells: Vec<Shell>,
    pub shortcut_remove_shells: Vec<Shell>,
    pub completion_attention_shells: Vec<Shell>,
    pub shortcut_attention_shells: Vec<Shell>,
    pub first_run: bool,
    pub catalog_warning: Option<String>,
}

impl fmt::Debug for SetupWizardResult {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SetupWizardResult")
            .field("provider", &self.provider)
            .field(
                "choices",
                &self
                    .choices
                    .iter()
                    .map(|choice| choice.as_ref().map(|value| value.model.id.as_str()))
                    .collect::<Vec<_>>(),
            )
            .field("completion_shells", &self.completion_shells)
            .field("shortcut_shells", &self.shortcut_shells)
            .field("completion_remove_shells", &self.completion_remove_shells)
            .field("shortcut_remove_shells", &self.shortcut_remove_shells)
            .field(
                "completion_attention_shells",
                &self.completion_attention_shells,
            )
            .field("shortcut_attention_shells", &self.shortcut_attention_shells)
            .field("first_run", &self.first_run)
            .field("catalog_warning", &self.catalog_warning)
            .finish()
    }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SetupPage {
    Provider,
    ModelRoles,
    ShellIntegration,
    Review,
}

impl SetupPage {
    fn index(self) -> usize {
        match self {
            Self::Provider => 0,
            Self::ModelRoles => 1,
            Self::ShellIntegration => 2,
            Self::Review => 3,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FieldOrigin {
    Loaded,
    Detected,
    Recommended,
    User,
}

impl FieldOrigin {
    fn label(self) -> &'static str {
        match self {
            Self::Loaded => "Loaded from config",
            Self::Detected => "Detected from environment",
            Self::Recommended => "Recommended default",
            Self::User => "Entered by you",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CredentialStorage {
    Configuration,
    Environment,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ProviderFocus {
    Identity,
    Endpoint,
    CredentialChoice,
    CredentialInput,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ModelFocus {
    Roles,
    Search,
    List,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RoleReview {
    Loaded,
    Suggested,
    Manual,
    NeedsReview,
    Confirmed,
}

#[derive(Debug, Clone)]
struct RoleDraft {
    model: Option<String>,
    origin: FieldOrigin,
    review: RoleReview,
    reasoning: ReasoningStrength,
    metadata: Option<crate::models::list::ModelReasoning>,
    query: String,
    selection: usize,
}

impl Default for RoleDraft {
    fn default() -> Self {
        Self {
            model: None,
            origin: FieldOrigin::Recommended,
            review: RoleReview::Suggested,
            reasoning: ReasoningStrength::Off,
            metadata: None,
            query: String::new(),
            selection: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CatalogStatus {
    NotStarted,
    Loading,
    Available,
    Unavailable,
}

type CatalogMessage = Result<Vec<ModelEntry>, String>;

pub fn run_with_config(
    config: &Config,
    entry: SetupEntryPoint,
) -> Result<SetupWizardOutcome, Error> {
    run_with_persisted_config(
        &PersistedConfig {
            config: config.clone(),
            exists: true,
        },
        entry,
    )
}

pub fn run_with_persisted_config(
    persisted: &PersistedConfig,
    entry: SetupEntryPoint,
) -> Result<SetupWizardOutcome, Error> {
    let mut wizard = SetupWizard::from_persisted(persisted, entry)?;
    let mut terminal = ratatui::init();
    let result = wizard.run_inner(&mut terminal);
    ratatui::restore();
    result
}

pub fn apply_result(config: &mut Config, result: &SetupWizardResult) -> Result<(), Error> {
    config::save_config(&result.config)?;
    *config = result.config.clone();

    let environment = ShellEnvironment::from_process();
    let mut failures = Vec::new();
    let completion_intents = build_intents(
        &result.completion_shells,
        &result.completion_remove_shells,
        &result.completion_attention_shells,
    );
    if !completion_intents.is_empty() {
        let report =
            shell_completion::reconcile_with_environment(&completion_intents, &environment);
        report_messages(&report);
        if let Some(message) = report.failure_message() {
            failures.push(message);
        }
    }

    let shortcut_intents = build_intents(
        &result.shortcut_shells,
        &result.shortcut_remove_shells,
        &result.shortcut_attention_shells,
    );
    if !shortcut_intents.is_empty() {
        let report = shell_shortcut::reconcile_with_environment(&shortcut_intents, &environment);
        report_messages(&report);
        if let Some(message) = report.failure_message() {
            failures.push(message);
        }
    }

    if failures.is_empty() {
        Ok(())
    } else {
        Err(Error::ConfigError(format!(
            "configuration saved, but shell integration failed; retry setup: {}",
            failures.join("; ")
        )))
    }
}

pub fn resolve_catalog_source(
    config: &Config,
    provider_endpoint: &str,
    provider_credential: Option<&str>,
) -> Result<(String, Option<String>), Error> {
    let (endpoint, credential) = if let Some(litellm) = &config.litellm {
        (litellm.endpoint.clone(), litellm.api_key.clone())
    } else {
        (
            provider_endpoint.to_string(),
            provider_credential.map(str::to_string),
        )
    };
    let credential = credential
        .map(|source| config::expand_api_key(&source))
        .transpose()?;
    Ok((endpoint, credential))
}

fn build_intents(
    present: &[Shell],
    absent: &[Shell],
    attention: &[Shell],
) -> Vec<(Shell, BlockIntent)> {
    let mut intents = present
        .iter()
        .copied()
        .map(|shell| (shell, BlockIntent::EnsurePresent))
        .collect::<Vec<_>>();
    intents.extend(
        absent
            .iter()
            .copied()
            .map(|shell| (shell, BlockIntent::EnsureAbsent)),
    );
    intents.extend(
        attention
            .iter()
            .copied()
            .map(|shell| (shell, BlockIntent::NeedsAttention)),
    );
    intents
}

fn report_messages(report: &shell_shortcut::InstallReport) {
    for target in &report.results {
        eprintln!("{}", target.message);
        if target.success {
            if let Some(reload) = &target.reload {
                eprintln!("{}", reload);
            }
        }
    }
}

struct SetupWizard {
    persisted: Config,
    first_run: bool,
    page: SetupPage,
    provider_identity: ProviderIdentity,
    endpoint: String,
    endpoint_origin: FieldOrigin,
    initial_endpoint: String,
    storage: CredentialStorage,
    credential_input: String,
    credential_origin: FieldOrigin,
    credential_candidates: Vec<CredentialCandidate>,
    credential_choice: Option<usize>,
    provider_focus: ProviderFocus,
    model_focus: ModelFocus,
    roles: [RoleDraft; 3],
    catalog: Vec<ModelEntry>,
    catalog_status: CatalogStatus,
    catalog_warning: Option<String>,
    catalog_tx: Sender<CatalogMessage>,
    catalog_rx: Receiver<CatalogMessage>,
    catalog_worker: Option<JoinHandle<()>>,
    catalog_cancel: Arc<AtomicBool>,
    completion_selected: [bool; 3],
    completion_initial: [Option<bool>; 3],
    completion_attention: [bool; 3],
    shortcut_selected: [bool; 3],
    shortcut_initial: [Option<bool>; 3],
    shortcut_attention: [bool; 3],
    shell_cursor: usize,
    active_role: usize,
    validation: String,
    discard_prompt: bool,
}

impl Drop for SetupWizard {
    fn drop(&mut self) {
        self.catalog_cancel.store(true, Ordering::SeqCst);
        if let Some(worker) = self.catalog_worker.take() {
            if worker.is_finished() {
                let _ = worker.join();
            }
        }
    }
}

impl SetupWizard {
    fn from_persisted(persisted: &PersistedConfig, entry: SetupEntryPoint) -> Result<Self, Error> {
        let config = persisted.config.clone();
        let provider_name = config.defaults.provider.as_deref().unwrap_or("openrouter");
        let provider = config::resolve_provider(&config, provider_name).unwrap_or_else(|_| {
            crate::config::types::ProviderConfig {
                endpoint: OPENROUTER_ENDPOINT.to_string(),
                api_key: None,
                default_model: None,
            }
        });
        let endpoint = if provider.endpoint.is_empty() {
            OPENROUTER_ENDPOINT.to_string()
        } else {
            provider.endpoint.clone()
        };
        let identity = ProviderIdentity::from_config(provider_name, &endpoint);
        let first_run = !persisted.exists;
        let (storage, credential_input, credential_origin, credential_choice) =
            initial_credential(&provider.api_key, identity, first_run);
        let candidates = discover_credentials(identity.name());
        let roles = initial_roles(&config, first_run);
        let shell_environment = ShellEnvironment::from_process();
        let (completion_initial, completion_attention) =
            inspect_shell_integration(&shell_environment, false);
        let (shortcut_initial, shortcut_attention) =
            inspect_shell_integration(&shell_environment, true);
        let completion_selected = completion_initial.map(|value| value.unwrap_or(false));
        let shortcut_selected = shortcut_initial.map(|value| value.unwrap_or(false));
        let (catalog_tx, catalog_rx) = mpsc::channel();
        let provider_focus = if first_run {
            ProviderFocus::Identity
        } else if provider.api_key.is_some() {
            ProviderFocus::Endpoint
        } else {
            ProviderFocus::CredentialChoice
        };
        let mut wizard = Self {
            persisted: config,
            first_run,
            page: match entry {
                SetupEntryPoint::Models => SetupPage::ModelRoles,
                SetupEntryPoint::Setup | SetupEntryPoint::Provider => SetupPage::Provider,
            },
            provider_identity: identity,
            endpoint: endpoint.clone(),
            endpoint_origin: if first_run {
                FieldOrigin::Recommended
            } else {
                FieldOrigin::Loaded
            },
            initial_endpoint: endpoint,
            storage,
            credential_input,
            credential_origin,
            credential_candidates: candidates,
            credential_choice,
            provider_focus,
            model_focus: ModelFocus::Roles,
            roles,
            catalog: Vec::new(),
            catalog_status: CatalogStatus::NotStarted,
            catalog_warning: None,
            catalog_tx,
            catalog_rx,
            catalog_worker: None,
            catalog_cancel: Arc::new(AtomicBool::new(false)),
            completion_selected,
            completion_initial,
            completion_attention,
            shortcut_selected,
            shortcut_initial,
            shortcut_attention,
            shell_cursor: 0,
            active_role: 0,
            validation: String::new(),
            discard_prompt: false,
        };
        if wizard.page == SetupPage::ModelRoles {
            wizard.start_catalog();
        }
        Ok(wizard)
    }

    fn run_inner(&mut self, terminal: &mut DefaultTerminal) -> Result<SetupWizardOutcome, Error> {
        loop {
            self.apply_catalog_result();
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
                self.catalog_cancel.store(true, Ordering::SeqCst);
                return Ok(SetupWizardOutcome::Cancelled(SetupCancellation::CtrlC));
            }
            if self.discard_prompt {
                if let Some(outcome) = self.handle_discard_prompt(key)? {
                    return Ok(outcome);
                }
                continue;
            }
            if key.code == KeyCode::Esc {
                self.discard_prompt = true;
                continue;
            }
            if key.code == KeyCode::Char('u') && key.modifiers.contains(KeyModifiers::CONTROL) {
                self.clear_active_input();
                continue;
            }
            if let Some(outcome) = self.handle_key(key)? {
                return Ok(outcome);
            }
        }
    }

    fn handle_discard_prompt(
        &mut self,
        key: KeyEvent,
    ) -> Result<Option<SetupWizardOutcome>, Error> {
        match key.code {
            KeyCode::Char('d') | KeyCode::Char('n') => {
                self.catalog_cancel.store(true, Ordering::SeqCst);
                Ok(Some(SetupWizardOutcome::Cancelled(
                    SetupCancellation::Escape,
                )))
            }
            KeyCode::Char('y') | KeyCode::Enter => {
                self.discard_prompt = false;
                match self.result() {
                    Ok(result) => Ok(Some(SetupWizardOutcome::Saved(Box::new(result)))),
                    Err(error) => {
                        self.validation = error.to_string();
                        self.discard_prompt = false;
                        Ok(None)
                    }
                }
            }
            KeyCode::Esc => {
                self.discard_prompt = false;
                Ok(None)
            }
            _ => Ok(None),
        }
    }

    fn handle_key(&mut self, key: KeyEvent) -> Result<Option<SetupWizardOutcome>, Error> {
        if key.code == KeyCode::BackTab
            || (key.code == KeyCode::Tab && key.modifiers.contains(KeyModifiers::SHIFT))
        {
            return self.previous_focus_or_page();
        }
        match self.page {
            SetupPage::Provider => self.handle_provider_key(key),
            SetupPage::ModelRoles => self.handle_model_key(key),
            SetupPage::ShellIntegration => self.handle_shell_key(key),
            SetupPage::Review => self.handle_review_key(key),
        }
    }

    fn handle_provider_key(&mut self, key: KeyEvent) -> Result<Option<SetupWizardOutcome>, Error> {
        match key.code {
            KeyCode::Up => self.provider_up(),
            KeyCode::Down => self.provider_down(),
            KeyCode::Backspace => self.edit_provider(None),
            KeyCode::Char(character) => {
                if self.provider_focus == ProviderFocus::CredentialChoice
                    && matches!(character, 'p' | 'P')
                {
                    self.storage = CredentialStorage::Configuration;
                    self.credential_input.clear();
                    self.credential_origin = FieldOrigin::User;
                    self.provider_focus = ProviderFocus::CredentialInput;
                } else if self.provider_focus == ProviderFocus::CredentialChoice
                    && matches!(character, 'e' | 'E')
                {
                    self.storage = CredentialStorage::Environment;
                    if self.credential_origin != FieldOrigin::Detected {
                        self.credential_input.clear();
                    }
                    self.credential_origin = FieldOrigin::User;
                    self.provider_focus = ProviderFocus::CredentialInput;
                } else {
                    self.edit_provider(Some(character));
                }
            }
            KeyCode::Tab | KeyCode::Enter if self.advance_provider()? => {
                self.page = SetupPage::ModelRoles;
                self.model_focus = ModelFocus::Roles;
                self.start_catalog();
            }
            KeyCode::Tab | KeyCode::Enter => {}
            _ => {}
        }
        Ok(None)
    }

    fn advance_provider(&mut self) -> Result<bool, Error> {
        match self.provider_focus {
            ProviderFocus::Identity => {
                self.provider_focus = ProviderFocus::Endpoint;
                if let Some(default) = self.provider_identity.endpoint() {
                    self.endpoint = default.to_string();
                    self.endpoint_origin = if self.first_run {
                        FieldOrigin::Recommended
                    } else {
                        FieldOrigin::User
                    };
                }
                Ok(false)
            }
            ProviderFocus::Endpoint => {
                self.normalize_endpoint_in_place()?;
                self.provider_focus = if !self.first_run
                    && self
                        .persisted
                        .providers
                        .get(
                            self.persisted
                                .defaults
                                .provider
                                .as_deref()
                                .unwrap_or("openrouter"),
                        )
                        .and_then(|provider| provider.api_key.as_ref())
                        .is_some()
                {
                    ProviderFocus::CredentialInput
                } else {
                    ProviderFocus::CredentialChoice
                };
                Ok(false)
            }
            ProviderFocus::CredentialChoice => {
                if self.credential_choice.is_none() && self.credential_candidates.len() == 1 {
                    self.credential_choice = Some(0);
                }
                if let Some(choice) = self.credential_choice {
                    if let Some(candidate) = self.credential_candidates.get(choice) {
                        self.storage = CredentialStorage::Environment;
                        self.credential_input = candidate.name.clone();
                        self.credential_origin = if candidate.detected {
                            FieldOrigin::Detected
                        } else {
                            FieldOrigin::Recommended
                        };
                    }
                }
                self.provider_focus = ProviderFocus::CredentialInput;
                Ok(false)
            }
            ProviderFocus::CredentialInput => {
                self.validate_provider()?;
                Ok(true)
            }
        }
    }

    fn provider_up(&mut self) {
        match self.provider_focus {
            ProviderFocus::Identity => {
                self.provider_identity = match self.provider_identity {
                    ProviderIdentity::OpenRouter => ProviderIdentity::Custom,
                    ProviderIdentity::OpenAi => ProviderIdentity::OpenRouter,
                    ProviderIdentity::Custom => ProviderIdentity::OpenAi,
                };
                self.apply_identity_defaults();
            }
            ProviderFocus::CredentialChoice => {
                let count = self.credential_candidates.len();
                if count > 0 {
                    self.credential_choice =
                        Some(self.credential_choice.unwrap_or(0).saturating_sub(1));
                }
            }
            _ => {}
        }
    }

    fn provider_down(&mut self) {
        match self.provider_focus {
            ProviderFocus::Identity => {
                self.provider_identity = match self.provider_identity {
                    ProviderIdentity::OpenRouter => ProviderIdentity::OpenAi,
                    ProviderIdentity::OpenAi => ProviderIdentity::Custom,
                    ProviderIdentity::Custom => ProviderIdentity::OpenRouter,
                };
                self.apply_identity_defaults();
            }
            ProviderFocus::CredentialChoice => {
                let count = self.credential_candidates.len();
                if count > 0 {
                    self.credential_choice = Some(
                        self.credential_choice
                            .unwrap_or(0)
                            .saturating_add(1)
                            .min(count.saturating_sub(1)),
                    );
                }
            }
            _ => {}
        }
    }

    fn apply_identity_defaults(&mut self) {
        if let Some(endpoint) = self.provider_identity.endpoint() {
            self.endpoint = endpoint.to_string();
            self.endpoint_origin = FieldOrigin::Recommended;
        } else if self.endpoint_origin != FieldOrigin::User {
            self.endpoint.clear();
            self.endpoint_origin = FieldOrigin::User;
        }
        self.credential_candidates = discover_credentials(self.provider_identity.name());
        self.credential_choice = None;
        let preserve_loaded = !self.first_run && self.credential_origin == FieldOrigin::Loaded;
        if !preserve_loaded {
            self.credential_input.clear();
            self.credential_origin = FieldOrigin::Recommended;
            self.storage = CredentialStorage::Environment;
            self.select_single_detected_credential();
        }
        self.invalidate_roles();
    }

    fn select_single_detected_credential(&mut self) {
        let detected = self
            .credential_candidates
            .iter()
            .enumerate()
            .filter(|(_, candidate)| candidate.detected)
            .map(|(index, _)| index)
            .collect::<Vec<_>>();
        if detected.len() == 1 {
            let index = detected[0];
            self.credential_choice = Some(index);
            self.credential_input = self.credential_candidates[index].name.clone();
            self.credential_origin = FieldOrigin::Detected;
        }
    }

    fn edit_provider(&mut self, character: Option<char>) {
        match self.provider_focus {
            ProviderFocus::Endpoint => {
                if let Some(character) = character {
                    self.endpoint.push(character);
                } else {
                    self.endpoint.pop();
                }
                self.endpoint_origin = FieldOrigin::User;
                self.provider_identity = ProviderIdentity::Custom;
                self.credential_candidates = discover_credentials("custom");
                self.credential_choice = None;
                if self.credential_origin != FieldOrigin::User {
                    self.credential_input.clear();
                    self.credential_origin = FieldOrigin::Recommended;
                    self.storage = CredentialStorage::Environment;
                    self.select_single_detected_credential();
                }
                self.invalidate_roles();
            }
            ProviderFocus::CredentialInput => {
                if let Some(character) = character {
                    self.credential_input.push(character);
                } else {
                    self.credential_input.pop();
                }
                self.credential_origin = FieldOrigin::User;
            }
            ProviderFocus::Identity | ProviderFocus::CredentialChoice => {}
        }
    }

    fn normalize_endpoint_in_place(&mut self) -> Result<(), Error> {
        self.endpoint = normalize_endpoint(&self.endpoint)?;
        if self.endpoint != self.initial_endpoint {
            self.invalidate_roles();
        }
        self.provider_identity =
            ProviderIdentity::from_config(self.provider_identity.name(), &self.endpoint);
        Ok(())
    }

    fn validate_provider(&mut self) -> Result<(), Error> {
        self.normalize_endpoint_in_place()?;
        if self.credential_input.trim().is_empty() {
            return Err(self.set_validation("credential source is required"));
        }
        if self.storage == CredentialStorage::Environment {
            let detected_count = self
                .credential_candidates
                .iter()
                .filter(|candidate| candidate.detected)
                .count();
            if detected_count > 1
                && self.credential_choice.is_none()
                && self.credential_origin != FieldOrigin::User
            {
                return Err(self.set_validation("select one detected credential source"));
            }
            if !valid_environment_name(&self.credential_input) {
                return Err(self.set_validation("environment variable name is invalid"));
            }
            if !env_present(&self.credential_input) {
                return Err(self.set_validation(format!(
                    "environment variable '{}' is not set",
                    self.credential_input
                )));
            }
        }
        self.validation.clear();
        Ok(())
    }

    fn handle_model_key(&mut self, key: KeyEvent) -> Result<Option<SetupWizardOutcome>, Error> {
        if (key.code == KeyCode::Char('f') && key.modifiers.contains(KeyModifiers::CONTROL))
            || (key.code == KeyCode::Char('/') && self.model_focus == ModelFocus::Roles)
        {
            self.model_focus = ModelFocus::Search;
            return Ok(None);
        }
        if key.code == KeyCode::Char('r') && key.modifiers.contains(KeyModifiers::CONTROL) {
            self.cycle_reasoning(1);
            return Ok(None);
        }
        match key.code {
            KeyCode::Up => match self.model_focus {
                ModelFocus::Roles => {
                    self.active_role = self.active_role.saturating_sub(1);
                    self.sync_role_metadata();
                }
                ModelFocus::List => self.move_model_selection(-1),
                ModelFocus::Search => {}
            },
            KeyCode::Down => match self.model_focus {
                ModelFocus::Roles => {
                    self.active_role = (self.active_role + 1).min(2);
                    self.sync_role_metadata();
                }
                ModelFocus::List => self.move_model_selection(1),
                ModelFocus::Search => {}
            },
            KeyCode::Backspace => {
                if self.model_focus == ModelFocus::Search {
                    self.edit_search(None);
                } else {
                    self.edit_role(None);
                }
            }
            KeyCode::Char(character) => {
                if self.model_focus == ModelFocus::Search {
                    self.edit_search(Some(character));
                } else {
                    self.edit_role(Some(character));
                }
            }
            KeyCode::Tab | KeyCode::Enter => match self.model_focus {
                ModelFocus::Roles if key.code == KeyCode::Tab => {
                    self.model_focus = ModelFocus::Search;
                }
                ModelFocus::Roles => {
                    if self.confirm_role()? {
                        if self.active_role < 2 {
                            self.active_role += 1;
                        } else {
                            self.page = SetupPage::ShellIntegration;
                        }
                    }
                }
                ModelFocus::Search => self.model_focus = ModelFocus::List,
                ModelFocus::List => {
                    self.choose_model_from_list();
                    self.model_focus = ModelFocus::Roles;
                }
            },
            _ => {}
        }
        Ok(None)
    }

    fn edit_role(&mut self, character: Option<char>) {
        let role = &mut self.roles[self.active_role];
        if let Some(character) = character {
            role.query.push(character);
            role.model = Some(role.query.clone());
            role.origin = FieldOrigin::User;
            role.review = RoleReview::Manual;
            role.reasoning = ReasoningStrength::Off;
            role.metadata = None;
        } else {
            role.query.pop();
            role.model = (!role.query.is_empty()).then(|| role.query.clone());
        }
    }

    fn edit_search(&mut self, character: Option<char>) {
        let role = &mut self.roles[self.active_role];
        if let Some(character) = character {
            role.query.push(character);
        } else {
            role.query.pop();
        }
        role.selection = 0;
    }

    fn filtered_models(&self) -> Vec<&ModelEntry> {
        let query = &self.roles[self.active_role].query;
        self.catalog
            .iter()
            .filter(|model| word_matches(&model.id, query))
            .collect()
    }

    fn move_model_selection(&mut self, direction: i32) {
        let count = self.filtered_models().len();
        if count == 0 {
            return;
        }
        let role = &mut self.roles[self.active_role];
        role.selection = (role.selection as i32 + direction).rem_euclid(count as i32) as usize;
    }

    fn choose_model_from_list(&mut self) {
        let matches = self.filtered_models();
        let Some(model) = matches.get(self.roles[self.active_role].selection) else {
            return;
        };
        let model = (*model).clone();
        let reasoning = default_reasoning(&model);
        let role = &mut self.roles[self.active_role];
        role.model = Some(model.id);
        role.metadata = model.reasoning.clone();
        role.origin = FieldOrigin::User;
        role.review = RoleReview::Confirmed;
        role.reasoning = reasoning;
    }

    fn confirm_role(&mut self) -> Result<bool, Error> {
        let role = &mut self.roles[self.active_role];
        let Some(model) = role
            .model
            .as_deref()
            .filter(|model| !model.trim().is_empty())
        else {
            self.validation = format!("{} model needs selection", role_label(self.active_role));
            return Ok(false);
        };
        if let Some(entry) = self.catalog.iter().find(|entry| entry.id == model) {
            role.metadata = entry.reasoning.clone();
            role.review = RoleReview::Confirmed;
            role.reasoning = role
                .reasoning
                .min_supported_by(role.metadata.as_ref())
                .unwrap_or(ReasoningStrength::Off);
        } else {
            role.review = RoleReview::Manual;
            role.metadata = None;
            role.reasoning = ReasoningStrength::Off;
        }
        role.origin = if matches!(role.review, RoleReview::Manual) {
            FieldOrigin::User
        } else {
            role.origin
        };
        self.validation.clear();
        Ok(true)
    }

    fn sync_role_metadata(&mut self) {
        let role = &mut self.roles[self.active_role];
        if let Some(model) = role.model.as_deref() {
            if let Some(entry) = self.catalog.iter().find(|entry| entry.id == model) {
                role.metadata = entry.reasoning.clone();
                if role.review != RoleReview::Manual {
                    role.reasoning = role
                        .reasoning
                        .min_supported_by(role.metadata.as_ref())
                        .unwrap_or(ReasoningStrength::Off);
                }
            }
        }
    }

    fn cycle_reasoning(&mut self, direction: i32) {
        let role = &mut self.roles[self.active_role];
        let Some(metadata) = role.metadata.as_ref() else {
            role.reasoning = ReasoningStrength::Off;
            return;
        };
        let mut options = vec![ReasoningStrength::Off];
        if metadata.mandatory {
            options.clear();
        }
        for effort in &metadata.supported_efforts {
            if let Some(strength) = ReasoningStrength::parse(effort) {
                if !options.contains(&strength) {
                    options.push(strength);
                }
            }
        }
        if options.is_empty() {
            role.reasoning = ReasoningStrength::Off;
            return;
        }
        let index = options
            .iter()
            .position(|value| *value == role.reasoning)
            .unwrap_or(0) as i32;
        role.reasoning = options[(index + direction).rem_euclid(options.len() as i32) as usize];
        role.review = RoleReview::Confirmed;
    }

    fn handle_shell_key(&mut self, key: KeyEvent) -> Result<Option<SetupWizardOutcome>, Error> {
        match key.code {
            KeyCode::Up => self.shell_cursor = self.shell_cursor.saturating_sub(1),
            KeyCode::Down => self.shell_cursor = (self.shell_cursor + 1).min(5),
            KeyCode::Char(' ') => self.toggle_shell(),
            KeyCode::Tab | KeyCode::Enter => {
                if self.shell_cursor < 5 && key.code == KeyCode::Tab {
                    self.shell_cursor += 1;
                } else {
                    self.page = SetupPage::Review;
                }
            }
            _ => {}
        }
        Ok(None)
    }

    fn toggle_shell(&mut self) {
        if self.shell_cursor < 3 {
            self.completion_selected[self.shell_cursor] =
                !self.completion_selected[self.shell_cursor];
        } else {
            let index = self.shell_cursor - 3;
            self.shortcut_selected[index] = !self.shortcut_selected[index];
        }
    }

    fn handle_review_key(&mut self, key: KeyEvent) -> Result<Option<SetupWizardOutcome>, Error> {
        if matches!(
            key.code,
            KeyCode::Enter | KeyCode::Char('f') | KeyCode::Char('F')
        ) {
            match self.result() {
                Ok(result) => return Ok(Some(SetupWizardOutcome::Saved(Box::new(result)))),
                Err(error) => self.validation = error.to_string(),
            }
        }
        Ok(None)
    }

    fn previous_focus_or_page(&mut self) -> Result<Option<SetupWizardOutcome>, Error> {
        match self.page {
            SetupPage::Provider => {
                self.provider_focus = match self.provider_focus {
                    ProviderFocus::Identity => ProviderFocus::Identity,
                    ProviderFocus::Endpoint => ProviderFocus::Identity,
                    ProviderFocus::CredentialChoice => ProviderFocus::Endpoint,
                    ProviderFocus::CredentialInput => ProviderFocus::CredentialChoice,
                };
            }
            SetupPage::ModelRoles => match self.model_focus {
                ModelFocus::List => self.model_focus = ModelFocus::Search,
                ModelFocus::Search => self.model_focus = ModelFocus::Roles,
                ModelFocus::Roles if self.active_role > 0 => self.active_role -= 1,
                ModelFocus::Roles => self.page = SetupPage::Provider,
            },
            SetupPage::ShellIntegration => {
                if self.shell_cursor > 0 {
                    self.shell_cursor -= 1;
                } else {
                    self.page = SetupPage::ModelRoles;
                    self.active_role = 2;
                    self.model_focus = ModelFocus::Roles;
                }
            }
            SetupPage::Review => self.page = SetupPage::ShellIntegration,
        }
        Ok(None)
    }

    fn clear_active_input(&mut self) {
        match self.page {
            SetupPage::Provider => match self.provider_focus {
                ProviderFocus::Endpoint => self.endpoint.clear(),
                ProviderFocus::CredentialInput => self.credential_input.clear(),
                ProviderFocus::Identity | ProviderFocus::CredentialChoice => {}
            },
            SetupPage::ModelRoles => {
                if self.model_focus == ModelFocus::Search {
                    self.roles[self.active_role].query.clear();
                    self.roles[self.active_role].selection = 0;
                } else {
                    self.roles[self.active_role].query.clear();
                    self.roles[self.active_role].model = None;
                    self.roles[self.active_role].review = RoleReview::Manual;
                }
            }
            SetupPage::ShellIntegration | SetupPage::Review => {}
        }
    }

    fn start_catalog(&mut self) {
        if self.catalog_status == CatalogStatus::Loading
            || self.catalog_status == CatalogStatus::Available
        {
            return;
        }
        self.catalog_status = CatalogStatus::Loading;
        self.catalog_warning = None;
        let source = self.catalog_source();
        let tx = self.catalog_tx.clone();
        let cancel = Arc::clone(&self.catalog_cancel);
        self.catalog_worker = Some(std::thread::spawn(move || {
            let result = source.and_then(|(endpoint, key)| {
                if cancel.load(Ordering::SeqCst) {
                    return Err("catalog request cancelled".to_string());
                }
                match fetch_models(&endpoint, key.as_deref()) {
                    Ok(models) if !models.is_empty() => Ok(models),
                    Ok(_) => Err("model catalog returned no usable models".to_string()),
                    Err(_) => Err("model catalog request failed; enter roles manually".to_string()),
                }
            });
            if !cancel.load(Ordering::SeqCst) {
                let _ = tx.send(result);
            }
        }));
    }

    fn catalog_source(&self) -> Result<(String, Option<String>), String> {
        let source = match self.storage {
            CredentialStorage::Configuration => Some(self.credential_input.clone()),
            CredentialStorage::Environment => Some(format!("${{{}}}", self.credential_input)),
        };
        resolve_catalog_source(&self.persisted, &self.endpoint, source.as_deref()).map_err(|_| {
            "credential is unavailable; enter a valid source or manual model roles".to_string()
        })
    }

    fn apply_catalog_result(&mut self) {
        let Ok(result) = self.catalog_rx.try_recv() else {
            return;
        };
        self.catalog_status = match result {
            Ok(models) if !models.is_empty() => {
                self.catalog = models;
                self.seed_role_suggestions();
                CatalogStatus::Available
            }
            Ok(_) => {
                self.catalog_warning = Some(
                    "Catalog is unverified. Enter all model roles manually; reasoning will be off."
                        .to_string(),
                );
                CatalogStatus::Unavailable
            }
            Err(message) => {
                self.catalog_warning = Some(format!(
                    "Unverified catalog: {}. Manual model roles are allowed with reasoning off.",
                    message
                ));
                CatalogStatus::Unavailable
            }
        };
        self.catalog_worker = None;
    }

    fn seed_role_suggestions(&mut self) {
        for index in 0..3 {
            let role = &mut self.roles[index];
            if role.model.is_none() {
                if let Some(entry) = self.catalog.get(index) {
                    role.model = Some(entry.id.clone());
                    role.origin = FieldOrigin::Recommended;
                    role.review = RoleReview::Suggested;
                    role.metadata = entry.reasoning.clone();
                    role.reasoning = default_reasoning(entry);
                }
            } else {
                role.selection = role
                    .model
                    .as_deref()
                    .and_then(|id| self.catalog.iter().position(|entry| entry.id == id))
                    .unwrap_or(0);
                role.metadata = role
                    .model
                    .as_deref()
                    .and_then(|id| self.catalog.iter().find(|entry| entry.id == id))
                    .and_then(|entry| entry.reasoning.clone());
            }
        }
    }

    fn invalidate_roles(&mut self) {
        for role in &mut self.roles {
            if role.model.is_some() {
                role.review = RoleReview::NeedsReview;
            }
        }
        self.catalog.clear();
        self.catalog_status = CatalogStatus::NotStarted;
        self.catalog_warning = None;
        self.catalog_cancel.store(true, Ordering::SeqCst);
        self.catalog_cancel = Arc::new(AtomicBool::new(false));
    }

    fn result(&mut self) -> Result<SetupWizardResult, Error> {
        self.validate_provider()?;
        for index in 0..3 {
            if self.roles[index].origin == FieldOrigin::User {
                self.roles[index].review = RoleReview::Manual;
                self.roles[index].reasoning = ReasoningStrength::Off;
            }
            if self.roles[index].model.is_none()
                || matches!(
                    self.roles[index].review,
                    RoleReview::NeedsReview | RoleReview::Suggested
                )
            {
                return Err(self
                    .set_validation(format!("{} model needs explicit review", role_label(index))));
            }
        }
        let api_key = match self.storage {
            CredentialStorage::Configuration => self.credential_input.clone(),
            CredentialStorage::Environment => format!("${{{}}}", self.credential_input),
        };
        let provider =
            build_provider_draft_for_identity(self.provider_identity, &self.endpoint, &api_key)?;
        let mut updated = self.persisted.clone();
        updated.defaults.provider = Some(provider.name.clone());
        let default_model = updated
            .providers
            .get(&provider.name)
            .and_then(|value| value.default_model.clone());
        updated.providers.insert(
            provider.name.clone(),
            crate::config::types::ProviderConfig {
                endpoint: provider.endpoint.clone(),
                api_key: Some(provider.api_key.clone()),
                default_model,
            },
        );
        updated.tiers.small = self.roles[0].model.clone();
        updated.tiers.normal = self.roles[1].model.clone();
        updated.tiers.thinking = self.roles[2].model.clone();
        updated.tiers.reasoning.small = Some(self.roles[0].reasoning.as_str().to_string());
        updated.tiers.reasoning.normal = Some(self.roles[1].reasoning.as_str().to_string());
        updated.tiers.reasoning.thinking = Some(self.roles[2].reasoning.as_str().to_string());

        Ok(SetupWizardResult {
            config: updated,
            provider,
            choices: std::array::from_fn(|index| Some(self.level_choice(index))),
            completion_shells: changed_shells(
                &self.completion_initial,
                &self.completion_selected,
                &self.completion_attention,
                true,
            ),
            shortcut_shells: changed_shells(
                &self.shortcut_initial,
                &self.shortcut_selected,
                &self.shortcut_attention,
                true,
            ),
            completion_remove_shells: changed_shells(
                &self.completion_initial,
                &self.completion_selected,
                &self.completion_attention,
                false,
            ),
            shortcut_remove_shells: changed_shells(
                &self.shortcut_initial,
                &self.shortcut_selected,
                &self.shortcut_attention,
                false,
            ),
            completion_attention_shells: attention_shells(
                &self.completion_attention,
                &self.completion_selected,
            ),
            shortcut_attention_shells: attention_shells(
                &self.shortcut_attention,
                &self.shortcut_selected,
            ),
            first_run: self.first_run,
            catalog_warning: self.catalog_warning.clone(),
        })
    }

    fn level_choice(&self, index: usize) -> LevelChoice {
        let role = &self.roles[index];
        let id = role.model.clone().unwrap_or_default();
        let metadata = self.catalog.iter().find(|entry| entry.id == id);
        LevelChoice {
            model: metadata.cloned().unwrap_or_else(|| ModelEntry {
                id,
                name: None,
                context_length: None,
                pricing: None,
                supported_features: Vec::new(),
                reasoning: None,
            }),
            reasoning: role.reasoning,
        }
    }

    fn can_finish(&self) -> bool {
        if self.endpoint.trim().is_empty()
            || normalize_endpoint(&self.endpoint).is_err()
            || self.credential_input.trim().is_empty()
        {
            return false;
        }
        if self.storage == CredentialStorage::Environment
            && (!valid_environment_name(&self.credential_input)
                || !env_present(&self.credential_input))
        {
            return false;
        }
        self.roles.iter().all(|role| {
            role.model.is_some()
                && (role.origin == FieldOrigin::User
                    || !matches!(role.review, RoleReview::NeedsReview | RoleReview::Suggested))
        })
    }

    fn set_validation(&mut self, message: impl Into<String>) -> Error {
        let message = message.into();
        self.validation = message.clone();
        Error::ConfigError(message)
    }

    fn draw(&self, frame: &mut Frame) {
        let outer = Block::default()
            .borders(Borders::ALL)
            .title("watn setup")
            .border_style(Style::default().fg(Color::DarkGray));
        let inner = outer.inner(frame.area());
        frame.render_widget(outer, frame.area());
        let areas = Layout::vertical([
            Constraint::Length(3),
            Constraint::Min(8),
            Constraint::Length(3),
        ])
        .split(inner);
        let titles = [
            Line::from("Provider"),
            Line::from("Model roles"),
            Line::from("Shell integration"),
            Line::from("Review"),
        ];
        let tabs = Tabs::new(titles)
            .block(Block::default().borders(Borders::ALL).title("Setup topics"))
            .select(self.page.index())
            .highlight_style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )
            .divider(Span::raw(" | "));
        frame.render_widget(tabs, areas[0]);
        match self.page {
            SetupPage::Provider => self.draw_provider(frame, areas[1]),
            SetupPage::ModelRoles => self.draw_model_roles(frame, areas[1]),
            SetupPage::ShellIntegration => self.draw_shell_integration(frame, areas[1]),
            SetupPage::Review => self.draw_review(frame, areas[1]),
        }
        let footer = if self.discard_prompt {
            "Finish this draft? [Enter/y] Finish  [d/n] Discard  [Esc] Return"
        } else if !self.validation.is_empty() {
            &self.validation
        } else {
            "Tab/Enter next  Shift-Tab back  Ctrl-U clear  Esc discard  Ctrl-C quit"
        };
        frame.render_widget(
            Paragraph::new(footer)
                .block(Block::default().borders(Borders::ALL).title("Controls"))
                .wrap(Wrap { trim: true }),
            areas[2],
        );
    }

    fn draw_provider(&self, frame: &mut Frame, area: Rect) {
        let (settings, help) = split_settings_help(area);
        let endpoint_active = matches!(
            self.provider_focus,
            ProviderFocus::Identity | ProviderFocus::Endpoint
        );
        let credential_active = matches!(
            self.provider_focus,
            ProviderFocus::CredentialChoice | ProviderFocus::CredentialInput
        );
        let endpoint_ready = normalize_endpoint(&self.endpoint).is_ok();
        let credential_ready = !self.credential_input.trim().is_empty()
            && (self.storage == CredentialStorage::Configuration
                || (valid_environment_name(&self.credential_input)
                    && env_present(&self.credential_input)));
        let narrow = settings.height < 25;
        let sections = Layout::vertical([
            Constraint::Length(if narrow { 8 } else { 10 }),
            Constraint::Min(if narrow { 9 } else { 13 }),
            Constraint::Length(1),
        ])
        .split(settings);

        let mut endpoint_lines = Vec::new();
        if self.first_run {
            endpoint_lines.push(Line::from(Span::styled(
                "No config file. Nothing is saved until Finish setup.",
                Style::default().fg(Color::Yellow),
            )));
        }
        endpoint_lines.push(setting_line(
            format!(
                "{} Provider type: {}",
                focus_marker(self.provider_focus == ProviderFocus::Identity),
                self.provider_identity.name()
            ),
            self.provider_focus == ProviderFocus::Identity,
        ));
        endpoint_lines.push(Line::from(
            if self.provider_focus == ProviderFocus::Identity {
                "  Up/Down chooses OpenRouter | OpenAI | Custom"
            } else {
                "  Choose the provider identity before editing endpoint"
            },
        ));
        endpoint_lines.push(setting_line(
            format!(
                "{} Endpoint: {} ({})",
                focus_marker(self.provider_focus == ProviderFocus::Endpoint),
                cursor_value(&self.endpoint),
                origin_tag(self.endpoint_origin)
            ),
            self.provider_focus == ProviderFocus::Endpoint,
        ));
        endpoint_lines.push(Line::from(format!(
            "  Provenance: {}",
            self.endpoint_origin.label()
        )));

        frame.render_widget(
            Paragraph::new(Text::from(endpoint_lines))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(panel_border_style(endpoint_active))
                        .title(format!(
                            "1. Endpoint [{}]{}",
                            if endpoint_ready { "x" } else { " " },
                            if endpoint_active { " | active" } else { "" }
                        )),
                )
                .wrap(Wrap { trim: true }),
            sections[0],
        );

        let mut credential_lines = vec![setting_line(
            format!(
                "{} Credential source: {}",
                focus_marker(self.provider_focus == ProviderFocus::CredentialChoice),
                match self.storage {
                    CredentialStorage::Configuration => "stored value",
                    CredentialStorage::Environment => "environment variable",
                }
            ),
            self.provider_focus == ProviderFocus::CredentialChoice,
        )];
        credential_lines.push(Line::from("  Choose one source with Up/Down:"));
        for (index, candidate) in self.credential_candidates.iter().enumerate() {
            let selected = self.credential_choice == Some(index);
            credential_lines.push(Line::from(format!(
                "    {} {} [{}]",
                if selected { ">" } else { " " },
                candidate.name,
                if candidate.detected {
                    "detected"
                } else {
                    "not found"
                }
            )));
        }
        credential_lines.push(setting_line(
            format!(
                "{} Credential value: {}",
                focus_marker(self.provider_focus == ProviderFocus::CredentialInput),
                credential_value_display(self.storage, &self.credential_input)
            ),
            self.provider_focus == ProviderFocus::CredentialInput,
        ));
        credential_lines.push(Line::from(format!(
            "  Provenance: {}",
            self.credential_origin.label()
        )));
        credential_lines.push(Line::from(
            "  P stores a value | E uses an environment variable",
        ));

        frame.render_widget(
            Paragraph::new(Text::from(credential_lines))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(panel_border_style(credential_active))
                        .title(format!(
                            "2. Credential source [{}]{}",
                            if credential_ready { "x" } else { " " },
                            if credential_active { " | active" } else { "" }
                        )),
                )
                .wrap(Wrap { trim: true }),
            sections[1],
        );

        frame.render_widget(
            Paragraph::new(format!(
                "Finish setup: {}",
                if self.can_finish() {
                    "available"
                } else {
                    "unavailable until both required settings are ready"
                }
            )),
            sections[2],
        );

        self.draw_help(frame, help, "endpoint");
    }

    fn draw_model_roles(&self, frame: &mut Frame, area: Rect) {
        let (settings, help) = split_settings_help(area);
        let role = &self.roles[self.active_role];
        let matches = self.filtered_models();
        let narrow = settings.height < 25;
        let sections = Layout::vertical([
            Constraint::Length(if narrow { 7 } else { 9 }),
            Constraint::Min(if narrow { 8 } else { 10 }),
            Constraint::Length(if narrow { 5 } else { 6 }),
        ])
        .split(settings);

        let mut role_lines = vec![Line::from(format!(
            "Step {}/3 | Choose a model, set reasoning, press Enter",
            self.active_role + 1
        ))];
        for index in 0..3 {
            let role = &self.roles[index];
            let active = self.active_role == index;
            let model = role
                .model
                .as_deref()
                .map(|value| clipped(value, 32))
                .unwrap_or_else(|| "Needs selection".to_string());
            role_lines.push(Line::styled(
                format!(
                    "{} {}. {}: {} [{}] {}",
                    if active { ">>" } else { "  " },
                    index + 1,
                    role_label(index),
                    model,
                    role.reasoning.as_str(),
                    role_review_tag(role.review)
                ),
                focus_style(active && self.model_focus == ModelFocus::Roles),
            ));
        }

        frame.render_widget(
            Paragraph::new(Text::from(role_lines))
                .block(Block::default().borders(Borders::ALL).title(format!(
                    "Roles | 3 required | Finish setup: {}",
                    if self.can_finish() {
                        "available"
                    } else {
                        "unavailable"
                    }
                )))
                .wrap(Wrap { trim: true }),
            sections[0],
        );

        let catalog_status = match self.catalog_status {
            CatalogStatus::NotStarted => "not started",
            CatalogStatus::Loading => "loading...",
            CatalogStatus::Available => "available",
            CatalogStatus::Unavailable => "manual entry enabled",
        };
        let mut catalog_lines = vec![Line::from(format!(
            "{} Search models: {}█ (/ or Ctrl-F)",
            focus_marker(self.model_focus == ModelFocus::Search),
            role.query
        ))];
        let visible_models = matches.len().min(if narrow { 3 } else { 6 });
        catalog_lines.push(Line::from(format!(
            "Model list: {} match{} (showing {}) | Up/Down, Enter choose",
            matches.len(),
            if matches.len() == 1 { "" } else { "es" },
            visible_models
        )));
        for (index, model) in matches.iter().take(visible_models).enumerate() {
            let selected = index == role.selection.min(matches.len().saturating_sub(1));
            let style = if selected && self.model_focus == ModelFocus::List {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else if selected {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            catalog_lines.push(Line::styled(
                format!(
                    "  {} {}",
                    if selected { ">>" } else { "  " },
                    clipped(&model.id, 42)
                ),
                style,
            ));
        }
        if matches.is_empty() {
            catalog_lines.push(Line::from(
                "No matches. Type a model ID manually or clear the search.",
            ));
        }
        frame.render_widget(
            Paragraph::new(Text::from(catalog_lines))
                .block(Block::default().borders(Borders::ALL).title(format!(
                    "Catalog: {} | {} models | model list",
                    catalog_status,
                    self.catalog.len()
                )))
                .wrap(Wrap { trim: true }),
            sections[1],
        );

        let mut reasoning_lines = vec![Line::from(format!(
            "Current: {} | {}",
            role.reasoning.as_str(),
            role.model.as_deref().unwrap_or("no model selected")
        ))];
        reasoning_lines.push(Line::from(format!(
            "Choices: {}",
            reasoning_options_label(role.metadata.as_ref(), role.reasoning)
        )));
        reasoning_lines.push(Line::from(
            "Ctrl-R changes reasoning | Enter confirms model + reasoning",
        ));
        frame.render_widget(
            Paragraph::new(Text::from(reasoning_lines))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(format!("Reasoning | {}", role_label(self.active_role))),
                )
                .wrap(Wrap { trim: true }),
            sections[2],
        );
        self.draw_help(frame, help, "model role");
    }

    fn draw_shell_integration(&self, frame: &mut Frame, area: Rect) {
        let (settings, help) = split_settings_help(area);
        let mut lines = vec![Line::from(
            "Optional. Existing marker blocks determine the initial selections.",
        )];
        for index in 0..3 {
            lines.push(Line::styled(
                format!(
                    "{} Completion in {} [{}]{}",
                    focus_marker(self.shell_cursor == index),
                    Shell::ALL[index].name(),
                    if self.completion_selected[index] {
                        "x"
                    } else {
                        " "
                    },
                    if self.completion_attention[index] {
                        " Needs attention"
                    } else {
                        ""
                    }
                ),
                focus_style(self.shell_cursor == index),
            ));
        }
        for index in 0..3 {
            lines.push(Line::styled(
                format!(
                    "{} Ctrl-W shortcut in {} [{}]{}",
                    focus_marker(self.shell_cursor == index + 3),
                    Shell::ALL[index].name(),
                    if self.shortcut_selected[index] {
                        "x"
                    } else {
                        " "
                    },
                    if self.shortcut_attention[index] {
                        " Needs attention"
                    } else {
                        ""
                    }
                ),
                focus_style(self.shell_cursor == index + 3),
            ));
        }
        lines.push(Line::from("Space toggles the selected integration. It never executes generated commands automatically."));
        frame.render_widget(
            Paragraph::new(Text::from(lines))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Shell integration"),
                )
                .wrap(Wrap { trim: true }),
            settings,
        );
        self.draw_help(frame, help, "shell integration");
    }

    fn draw_review(&self, frame: &mut Frame, area: Rect) {
        let (settings, help) = split_settings_help(area);
        let mut lines = vec![Line::from(Span::styled(
            "Review draft before Finish setup",
            Style::default().add_modifier(Modifier::BOLD),
        ))];
        lines.push(Line::from(format!(
            "Provider: {}  Endpoint: {}",
            self.provider_identity.name(),
            self.endpoint
        )));
        lines.push(Line::from(format!(
            "Credential: {} ({})",
            match self.storage {
                CredentialStorage::Configuration => "configuration value",
                CredentialStorage::Environment => self.credential_input.as_str(),
            },
            self.credential_origin.label()
        )));
        for index in 0..3 {
            lines.push(Line::from(format!(
                "{}: {}  Reasoning: {}",
                role_label(index),
                self.roles[index]
                    .model
                    .as_deref()
                    .unwrap_or("Needs selection"),
                self.roles[index].reasoning.as_str()
            )));
        }
        lines.push(Line::from(format!(
            "Shell changes: completion {}, shortcut {}",
            changed_count(&self.completion_initial, &self.completion_selected),
            changed_count(&self.shortcut_initial, &self.shortcut_selected)
        )));
        lines.push(Line::from(format!(
            "Catalog source: {}",
            self.persisted
                .litellm
                .as_ref()
                .map(|catalog| catalog.endpoint.as_str())
                .unwrap_or("selected provider endpoint")
        )));
        if let Some(warning) = &self.catalog_warning {
            lines.push(Line::from(Span::styled(
                format!("Warning: {}", warning),
                Style::default().fg(Color::Yellow),
            )));
        } else {
            lines.push(Line::from("Warnings: none"));
        }
        lines.push(setting_line(
            format!(
                "{} Finish setup action. Press Enter to finish. Finish is {}.",
                focus_marker(true),
                if self.can_finish() {
                    "available"
                } else {
                    "blocked while required settings need attention"
                }
            ),
            true,
        ));
        frame.render_widget(
            Paragraph::new(Text::from(lines))
                .block(Block::default().borders(Borders::ALL).title("Review"))
                .wrap(Wrap { trim: true }),
            settings,
        );
        self.draw_help(frame, help, "review");
    }

    fn draw_help(&self, frame: &mut Frame, area: Rect, active: &str) {
        let text = if area.height < 12 {
            format!(
                "About this setting: {active}\nWhat it is: the {active} value used by watn.\nWhat it enables: provider requests, model discovery, or shell integration.\nRecommendation: review detected values and prefer environment-backed credentials.\nRequirement / tradeoff: endpoints must be HTTP(S); failures remain visible."
            )
        } else {
            format!(
                "About this setting: {active}\n\nWhat it is\nThe {active} value used by watn.\n\nWhat it enables\nProvider requests, model discovery, or safe shell integration.\n\nRecommendation\nReview detected values and prefer environment-backed credentials.\n\nRequirement / tradeoff\nThe endpoint must be HTTP(S); catalog and shell failures remain visible and may require attention."
            )
        };
        let placement = if area.x > 10 { "beside" } else { "below" };
        frame.render_widget(
            Paragraph::new(text)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(format!("About this setting ({placement})")),
                )
                .wrap(Wrap { trim: true }),
            area,
        );
    }
}

fn initial_credential(
    saved: &Option<String>,
    identity: ProviderIdentity,
    first_run: bool,
) -> (CredentialStorage, String, FieldOrigin, Option<usize>) {
    if let Some(value) = saved {
        if let Some(name) = value
            .strip_prefix("${")
            .and_then(|value| value.strip_suffix('}'))
        {
            return (
                CredentialStorage::Environment,
                name.to_string(),
                FieldOrigin::Loaded,
                None,
            );
        }
        return (
            CredentialStorage::Configuration,
            value.clone(),
            FieldOrigin::Loaded,
            None,
        );
    }
    if first_run {
        let candidates = discover_credentials(identity.name());
        let detected = candidates
            .iter()
            .enumerate()
            .filter(|(_, candidate)| candidate.detected)
            .map(|(index, _)| index)
            .collect::<Vec<_>>();
        if detected.len() == 1 {
            let index = detected[0];
            return (
                CredentialStorage::Environment,
                candidates[index].name.clone(),
                FieldOrigin::Detected,
                Some(index),
            );
        }
        if detected.len() > 1 {
            return (
                CredentialStorage::Environment,
                String::new(),
                FieldOrigin::Recommended,
                None,
            );
        }
        let suggested = match identity {
            ProviderIdentity::OpenAi => "OPENAI_API_KEY",
            ProviderIdentity::OpenRouter => "OPENROUTER_API_KEY",
            ProviderIdentity::Custom => "WATN_API_KEY",
        };
        return (
            CredentialStorage::Environment,
            suggested.to_string(),
            FieldOrigin::Recommended,
            None,
        );
    }
    (
        CredentialStorage::Configuration,
        String::new(),
        FieldOrigin::Loaded,
        None,
    )
}

fn initial_roles(config: &Config, first_run: bool) -> [RoleDraft; 3] {
    let values = [
        config.tiers.small.clone(),
        config.tiers.normal.clone(),
        config.tiers.thinking.clone(),
    ];
    let reasoning = [
        parse_reasoning(config.tiers.reasoning.small.as_deref()),
        parse_reasoning(config.tiers.reasoning.normal.as_deref()),
        parse_reasoning(config.tiers.reasoning.thinking.as_deref()),
    ];
    std::array::from_fn(|index| RoleDraft {
        model: values[index].clone(),
        origin: if first_run {
            FieldOrigin::Recommended
        } else {
            FieldOrigin::Loaded
        },
        review: if first_run {
            RoleReview::Suggested
        } else {
            RoleReview::Loaded
        },
        reasoning: reasoning[index],
        ..RoleDraft::default()
    })
}

fn inspect_shell_integration(
    environment: &ShellEnvironment,
    shortcut: bool,
) -> ([Option<bool>; 3], [bool; 3]) {
    let mut initial = [None; 3];
    let mut attention = [false; 3];
    for index in 0..3 {
        let state = if shortcut {
            shell_shortcut::marker_state(Shell::ALL[index], environment)
        } else {
            shell_completion::marker_state(Shell::ALL[index], environment)
        };
        (initial[index], attention[index]) = match state {
            Ok(BlockState::Present) => (Some(true), false),
            Ok(BlockState::Absent) => (Some(false), false),
            Ok(BlockState::Malformed) | Ok(BlockState::Unreadable) | Err(_) => (Some(false), true),
        };
    }
    (initial, attention)
}

fn changed_shells(
    initial: &[Option<bool>; 3],
    selected: &[bool; 3],
    attention: &[bool; 3],
    present: bool,
) -> Vec<Shell> {
    Shell::ALL
        .into_iter()
        .enumerate()
        .filter_map(|(index, shell)| {
            if attention[index] {
                return None;
            }
            let changed = initial[index].is_some_and(|value| value != selected[index]);
            (changed && selected[index] == present).then_some(shell)
        })
        .collect()
}

fn changed_count(initial: &[Option<bool>; 3], selected: &[bool; 3]) -> usize {
    initial
        .iter()
        .zip(selected)
        .filter(|(initial, selected)| initial.is_some_and(|value| value != **selected))
        .count()
}

fn attention_shells(attention: &[bool; 3], selected: &[bool; 3]) -> Vec<Shell> {
    Shell::ALL
        .into_iter()
        .enumerate()
        .filter_map(|(index, shell)| (attention[index] && selected[index]).then_some(shell))
        .collect()
}

fn split_settings_help(area: Rect) -> (Rect, Rect) {
    if area.width >= WIDE_LAYOUT_COLUMNS {
        let chunks = Layout::horizontal([Constraint::Percentage(66), Constraint::Percentage(34)])
            .split(area);
        (chunks[0], chunks[1])
    } else {
        let chunks =
            Layout::vertical([Constraint::Percentage(68), Constraint::Percentage(32)]).split(area);
        (chunks[0], chunks[1])
    }
}

fn focus_marker(focused: bool) -> &'static str {
    if focused {
        ">> ACTIVE"
    } else {
        "         "
    }
}

fn focus_style(focused: bool) -> Style {
    if focused {
        Style::default()
            .fg(Color::Black)
            .bg(Color::Green)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    }
}

fn panel_border_style(active: bool) -> Style {
    if active {
        Style::default().fg(Color::Green)
    } else {
        Style::default().fg(Color::DarkGray)
    }
}

fn setting_line(text: String, focused: bool) -> Line<'static> {
    Line::styled(text, focus_style(focused))
}

fn cursor_value(value: &str) -> String {
    format!("{}█", value)
}

fn clipped(value: &str, max_chars: usize) -> String {
    let mut chars = value.chars();
    let value = chars.by_ref().take(max_chars).collect::<String>();
    if chars.next().is_some() {
        let mut shortened = value
            .chars()
            .take(max_chars.saturating_sub(3))
            .collect::<String>();
        shortened.push_str("...");
        shortened
    } else {
        value
    }
}

fn origin_tag(origin: FieldOrigin) -> &'static str {
    match origin {
        FieldOrigin::Loaded => "loaded",
        FieldOrigin::Detected => "detected",
        FieldOrigin::Recommended => "recommended",
        FieldOrigin::User => "entered",
    }
}

fn credential_value_display(storage: CredentialStorage, input: &str) -> String {
    match storage {
        CredentialStorage::Configuration => {
            if input.is_empty() {
                "enter a value".to_string()
            } else {
                "****************".to_string()
            }
        }
        CredentialStorage::Environment => {
            if input.is_empty() {
                "choose a variable".to_string()
            } else {
                cursor_value(&clipped(input, 28))
            }
        }
    }
}

fn role_label(index: usize) -> &'static str {
    match index {
        0 => "Small / fast",
        1 => "Balanced / normal",
        _ => "Thinking",
    }
}

fn role_review_tag(review: RoleReview) -> &'static str {
    match review {
        RoleReview::Loaded => "Loaded",
        RoleReview::Suggested => "Suggested",
        RoleReview::Manual => "Manual",
        RoleReview::NeedsReview => "Needs attention",
        RoleReview::Confirmed => "Reviewed",
    }
}

fn reasoning_options_label(
    metadata: Option<&crate::models::list::ModelReasoning>,
    current: ReasoningStrength,
) -> String {
    let Some(metadata) = metadata else {
        return if current == ReasoningStrength::Off {
            "[off] only (manual or no catalog metadata)".to_string()
        } else {
            format!("[{}] (manual or no catalog metadata)", current.as_str())
        };
    };
    let mut options = Vec::new();
    if !metadata.mandatory {
        options.push("off".to_string());
    }
    for effort in &metadata.supported_efforts {
        if ReasoningStrength::parse(effort).is_some()
            && !options.iter().any(|value| value == effort)
        {
            options.push(effort.clone());
        }
    }
    if options.is_empty() {
        "off only".to_string()
    } else {
        options
            .into_iter()
            .map(|option| {
                if option == current.as_str() {
                    format!("[{option}]")
                } else {
                    option
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
}

fn default_reasoning(model: &ModelEntry) -> ReasoningStrength {
    model
        .reasoning
        .as_ref()
        .and_then(|metadata| metadata.default_effort.as_deref())
        .and_then(ReasoningStrength::parse)
        .unwrap_or(ReasoningStrength::Off)
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

trait ReasoningSelection {
    fn min_supported_by(
        self,
        metadata: Option<&crate::models::list::ModelReasoning>,
    ) -> Option<Self>
    where
        Self: Sized;
}

impl ReasoningSelection for ReasoningStrength {
    fn min_supported_by(
        self,
        metadata: Option<&crate::models::list::ModelReasoning>,
    ) -> Option<Self> {
        let Some(metadata) = metadata else {
            return Some(ReasoningStrength::Off);
        };
        if metadata.mandatory && self == ReasoningStrength::Off {
            return metadata
                .supported_efforts
                .iter()
                .find_map(|value| ReasoningStrength::parse(value));
        }
        if self == ReasoningStrength::Off
            || metadata
                .supported_efforts
                .iter()
                .any(|value| ReasoningStrength::parse(value) == Some(self))
        {
            Some(self)
        } else {
            Some(default_reasoning(&ModelEntry {
                id: String::new(),
                name: None,
                context_length: None,
                pricing: None,
                supported_features: Vec::new(),
                reasoning: Some(metadata.clone()),
            }))
        }
    }
}
