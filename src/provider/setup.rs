use crate::error::Error;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Cell, List, ListItem, ListState, Paragraph, Row, Table, Wrap},
};
use std::time::Duration;

pub const OPENROUTER_ENDPOINT: &str = "https://openrouter.ai/api/v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SetupCancellation {
    Escape,
    CtrlC,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderDraft {
    pub name: String,
    pub endpoint: String,
    pub api_key: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProviderSetupResult {
    Configured(ProviderDraft),
    Cancelled(SetupCancellation),
}

pub fn cancellation_result(cancellation: SetupCancellation) -> ProviderSetupResult {
    ProviderSetupResult::Cancelled(cancellation)
}

pub fn configured_result(draft: ProviderDraft) -> ProviderSetupResult {
    ProviderSetupResult::Configured(draft)
}

#[derive(Debug)]
pub enum ModelSetupResult {
    Saved,
    Cancelled(SetupCancellation),
    Failed(Error),
}

pub fn print_setup_guidance() {
    eprintln!(
        "No provider is configured. Run `watn provider` in a terminal or edit ~/.config/watn/config.toml."
    );
}

pub fn run_interactive() -> Result<ProviderSetupResult, Error> {
    let mut terminal = ratatui::init();
    let result = run_interactive_inner(&mut terminal);
    ratatui::restore();
    result
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SetupStage {
    Endpoint,
    CredentialSource,
    CredentialValue,
    Review,
    Confirmed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CredentialSource {
    Paste,
    Environment,
}

fn run_interactive_inner(terminal: &mut DefaultTerminal) -> Result<ProviderSetupResult, Error> {
    let mut stage = SetupStage::Endpoint;
    let mut source = CredentialSource::Paste;
    let mut endpoint = OPENROUTER_ENDPOINT.to_string();
    let mut value = String::new();
    let mut credential = String::new();
    let mut env_name = suggested_api_key_env(OPENROUTER_ENDPOINT).to_string();
    let mut validation = String::new();

    loop {
        if stage == SetupStage::Confirmed {
            let draft = build_provider_draft(&endpoint, &credential)
                .map_err(|error| Error::ConfigError(error_message(&error)))?;
            return Ok(ProviderSetupResult::Configured(draft));
        }
        terminal.draw(|frame| draw_setup(frame, stage, &endpoint, source, &value, &env_name, &validation))?;
        if !event::poll(Duration::from_millis(100))
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
            return Ok(ProviderSetupResult::Cancelled(SetupCancellation::CtrlC));
        }
        if key.code == KeyCode::Esc {
            return Ok(ProviderSetupResult::Cancelled(SetupCancellation::Escape));
        }

        match stage {
            SetupStage::Endpoint => match key.code {
                KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => endpoint.clear(),
                KeyCode::Char(character) => endpoint.push(character),
                KeyCode::Backspace => {
                    endpoint.pop();
                }
                KeyCode::Enter => match normalize_endpoint(&endpoint) {
                    Ok(normalized) => {
                        endpoint = normalized;
                        env_name = suggested_api_key_env(&endpoint).to_string();
                        validation.clear();
                        stage = SetupStage::CredentialSource;
                    }
                    Err(error) => validation = error_message(&error),
                },
                _ => {}
            },
            SetupStage::CredentialSource => match key.code {
                KeyCode::Char('e') | KeyCode::Char('E') => source = CredentialSource::Environment,
                KeyCode::Char('p') | KeyCode::Char('P') => source = CredentialSource::Paste,
                KeyCode::Enter => {
                    value.clear();
                    validation.clear();
                    stage = SetupStage::CredentialValue;
                }
                _ => {}
            },
            SetupStage::CredentialValue => {
                match key.code {
                    KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => value.clear(),
                    KeyCode::Char(character) => value.push(character),
                    KeyCode::Backspace => {
                        value.pop();
                    }
                    KeyCode::Enter => {
                        credential = match source {
                            CredentialSource::Paste => {
                                if value.trim().is_empty() {
                                    validation = "credential cannot be empty".to_string();
                                    continue;
                                }
                                value.clone()
                            }
                            CredentialSource::Environment => {
                                if !valid_environment_name(&env_name) {
                                    validation = "environment variable name is invalid".to_string();
                                    continue;
                                }
                                format!("${{{env_name}}}")
                            }
                        };
                        validation.clear();
                        stage = SetupStage::Review;
                    }
                    _ => {}
                }
            }
            SetupStage::Review => {
                if key.code == KeyCode::Enter {
                    stage = SetupStage::Confirmed;
                }
            }
            SetupStage::Confirmed => {}
        }
    }
}

fn error_message(error: &Error) -> String {
    match error {
        Error::ConfigError(message) => message.clone(),
        other => other.to_string(),
    }
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

fn draw_setup(
    frame: &mut Frame,
    stage: SetupStage,
    endpoint: &str,
    source: CredentialSource,
    value: &str,
    env_name: &str,
    validation: &str,
) {
    let panel = Block::bordered().title("Provider setup");
    let areas = Layout::vertical([
        Constraint::Length(5),
        Constraint::Min(7),
        Constraint::Length(6),
    ])
    .split(panel.inner(frame.area()));
    frame.render_widget(panel, frame.area());

    let credential = match source {
        CredentialSource::Paste => "Paste credential",
        CredentialSource::Environment => "Environment variable",
    };
    let value_display = match source {
        CredentialSource::Paste => "*".repeat(value.chars().count()),
        CredentialSource::Environment => env_name.to_string(),
    };
    let stage_hint = match stage {
        SetupStage::Endpoint => "Endpoint (Enter to accept, Ctrl-U to clear)",
        SetupStage::CredentialSource => "Credential source: [p] Paste credential [e] Environment variable",
        SetupStage::CredentialValue => "Credential value (Enter to confirm)",
        SetupStage::Review => "Review provider configuration (Enter to save)",
        SetupStage::Confirmed => "Provider configuration confirmed",
    };

    let source_items = vec![
        ListItem::new(Line::from("Paste credential [p]")),
        ListItem::new(Line::from("Environment variable [e]")),
    ];
    let mut source_state = ListState::default();
    source_state.select(Some(match source {
        CredentialSource::Paste => 0,
        CredentialSource::Environment => 1,
    }));
    let source_list = List::new(source_items)
        .block(Block::bordered().title("Credential source"))
        .highlight_style(
            Style::default()
                .bg(Color::Cyan)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");
    frame.render_stateful_widget(source_list, areas[0], &mut source_state);

    let details = Table::new(
        [
            Row::new([Cell::from("Endpoint"), Cell::from(endpoint)]),
            Row::new([Cell::from("Credential"), Cell::from(credential)]),
            Row::new([Cell::from("Value"), Cell::from(value_display)]),
        ],
        [Constraint::Length(16), Constraint::Min(1)],
    )
    .header(Row::new([Cell::from("Field"), Cell::from("Current value")]).style(
        Style::default().add_modifier(Modifier::BOLD),
    ))
    .block(Block::bordered().title("Provider details"));
    frame.render_widget(details, areas[1]);

    let guidance = format!(
        "OpenAI-compatible endpoint\n{stage_hint}\nSelected source: {credential}{validation}",
        validation = if validation.is_empty() {
            String::new()
        } else {
            format!("\nValidation: {validation}")
        },
    );
    let guidance = Paragraph::new(guidance)
        .block(Block::bordered().title("Guidance"))
        .wrap(Wrap { trim: true });
    frame.render_widget(guidance, areas[2]);
}

pub fn normalize_endpoint(endpoint: &str) -> Result<String, Error> {
    let endpoint = endpoint.trim().trim_end_matches('/').to_string();
    if endpoint.is_empty() {
        return Err(Error::ConfigError(
            "endpoint must be an HTTP or HTTPS URL".to_string(),
        ));
    }

    let parsed = reqwest::Url::parse(&endpoint).map_err(|_| {
        Error::ConfigError("endpoint must be an HTTP or HTTPS URL".to_string())
    })?;
    if !matches!(parsed.scheme(), "http" | "https") || parsed.host().is_none() {
        return Err(Error::ConfigError(
            "endpoint must be an HTTP or HTTPS URL".to_string(),
        ));
    }

    Ok(endpoint)
}

pub fn provider_name(endpoint: &str) -> &'static str {
    if endpoint == OPENROUTER_ENDPOINT {
        "openrouter"
    } else {
        "custom"
    }
}

pub fn suggested_api_key_env(endpoint: &str) -> &'static str {
    if endpoint == OPENROUTER_ENDPOINT {
        "OPENROUTER_API_KEY"
    } else {
        "WATN_API_KEY"
    }
}

pub fn build_provider_draft(endpoint: &str, api_key: &str) -> Result<ProviderDraft, Error> {
    let endpoint = normalize_endpoint(endpoint)?;
    if api_key.trim().is_empty() {
        return Err(Error::ConfigError("credential cannot be empty".to_string()));
    }

    Ok(ProviderDraft {
        name: provider_name(&endpoint).to_string(),
        endpoint,
        api_key: api_key.to_string(),
    })
}
