use std::io::{self, IsTerminal, Write};

use crossterm::{
    cursor,
    event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    queue,
    style::Print,
    terminal::{self, Clear, ClearType},
};

use super::response::ReviewCandidate;

const MAX_PANEL_ROWS: u16 = 14;
const MIN_PANEL_WIDTH: u16 = 20;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusRegion {
    Flow,
    Candidates,
    Actions,
}

impl FocusRegion {
    pub const ALL: [Self; 3] = [Self::Flow, Self::Candidates, Self::Actions];

    fn next(self) -> Self {
        match self {
            Self::Flow => Self::Candidates,
            Self::Candidates => Self::Actions,
            Self::Actions => Self::Flow,
        }
    }

    fn previous(self) -> Self {
        match self {
            Self::Flow => Self::Actions,
            Self::Candidates => Self::Flow,
            Self::Actions => Self::Candidates,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Flow => "Flow",
            Self::Candidates => "Candidates",
            Self::Actions => "Actions",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PanelAction {
    Accept,
    EditCommand,
    Reject,
    Cancel,
}

impl PanelAction {
    const ALL: [Self; 4] = [Self::Accept, Self::EditCommand, Self::Reject, Self::Cancel];

    fn label(self) -> &'static str {
        match self {
            Self::Accept => "Accept candidate",
            Self::EditCommand => "Edit command",
            Self::Reject => "Reject candidate",
            Self::Cancel => "Cancel review",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PanelInputMode {
    Review,
    CommandEditor,
}

/// In-progress review operations that can be interrupted independently of the
/// selected candidate and the open review state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReviewOperation {
    Regeneration,
    PurposeRefresh,
    ModelSelection,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PanelOutcome {
    Continue,
    Accepted(ReviewCandidate),
    Cancelled,
    EditCommitted(String),
    EditDiscarded,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewContext {
    pub intent: String,
    pub tier: String,
    pub provider: String,
    pub model: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InlineLayout {
    pub width: u16,
    pub height: u16,
    pub max_rows: u16,
    pub content_width: usize,
}

impl InlineLayout {
    pub fn for_dimensions(width: u16, height: u16) -> Self {
        let bounded_width = width.max(MIN_PANEL_WIDTH);
        Self {
            width,
            height,
            max_rows: height.saturating_sub(1).clamp(4, MAX_PANEL_ROWS),
            content_width: bounded_width.saturating_sub(4) as usize,
        }
    }

    pub fn is_bounded(self) -> bool {
        self.width >= MIN_PANEL_WIDTH && self.max_rows <= MAX_PANEL_ROWS
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewPanelState {
    pub context: ReviewContext,
    pub candidates: Vec<ReviewCandidate>,
    candidate_contexts: Vec<ReviewContext>,
    pub selected_candidate: usize,
    pub focus: FocusRegion,
    pub flow_stage: usize,
    pub candidate_cursor: usize,
    pub action_cursor: usize,
    pub input_mode: PanelInputMode,
    intent_history: Vec<String>,
    configured_model: String,
    model_selection: Option<Vec<String>>,
    model_cursor: usize,
    pending_model: Option<String>,
    pending_operation: Option<ReviewOperation>,
    editor_buffer: String,
    editor_original: String,
}

impl ReviewPanelState {
    pub fn new(context: ReviewContext, candidate: ReviewCandidate) -> Self {
        let candidate_contexts = vec![context.clone()];
        Self {
            configured_model: context.model.clone(),
            context,
            candidates: vec![candidate],
            candidate_contexts,
            selected_candidate: 0,
            // Actions is the release gate and the initial decision point.
            focus: FocusRegion::Actions,
            flow_stage: 0,
            candidate_cursor: 0,
            action_cursor: 0,
            input_mode: PanelInputMode::Review,
            intent_history: Vec::new(),
            model_selection: None,
            model_cursor: 0,
            pending_model: None,
            pending_operation: None,
            editor_buffer: String::new(),
            editor_original: String::new(),
        }
    }

    pub fn configured_model(&self) -> &str {
        &self.configured_model
    }

    pub fn model_selection(&self) -> Option<&[String]> {
        self.model_selection.as_deref()
    }

    /// At the highest configured tier, a higher-tier request opens the explicit
    /// provider catalog selection instead of generating.
    pub fn open_model_selection(&mut self, models: Vec<String>) {
        self.model_cursor = 0;
        self.model_selection = Some(models);
    }

    /// Applies the selected model to the next candidate only. The configured
    /// model is restored once that candidate cycle completes.
    pub fn select_model(&mut self, model: impl Into<String>, candidate: ReviewCandidate) {
        let model = model.into();
        self.pending_model = Some(model.clone());
        self.context.model = model;
        self.model_selection = None;
        self.replace_current(candidate);
    }

    pub fn complete_model_selection(&mut self) {
        if self.pending_model.take().is_some() {
            self.context.model = self.configured_model.clone();
        }
    }

    pub fn begin_operation(&mut self, operation: ReviewOperation) {
        self.pending_operation = Some(operation);
    }

    pub fn pending_operation(&self) -> Option<ReviewOperation> {
        self.pending_operation
    }

    /// Only the in-progress operation is cancelled; candidate and review state
    /// are preserved.
    pub fn interrupt_operation(&mut self) -> Option<ReviewOperation> {
        self.pending_operation.take()
    }

    pub fn candidate(&self) -> &ReviewCandidate {
        &self.candidates[self.selected_candidate]
    }

    pub fn intent_history(&self) -> &[String] {
        &self.intent_history
    }

    /// Rephrasing replaces the visible active Intent and starts a new candidate
    /// cycle. The prior Intent remains only in current-review history.
    pub fn rephrase_intent(&mut self, intent: impl Into<String>) {
        self.intent_history.push(self.context.intent.clone());
        self.context.intent = intent.into();
        self.selected_candidate = 0;
        self.candidate_cursor = 0;
        self.flow_stage = 0;
    }

    /// Regeneration and escalation replace the current candidate by default.
    pub fn replace_current(&mut self, candidate: ReviewCandidate) {
        self.candidates[self.selected_candidate] = candidate;
        self.candidate_contexts[self.selected_candidate] = self.context.clone();
        self.flow_stage = 0;
        self.candidate_cursor = self.selected_candidate;
    }

    /// Higher-tier escalation generates a candidate in the next tier context
    /// while preserving the current intent.
    pub fn escalate(&mut self, context: ReviewContext, candidate: ReviewCandidate) {
        self.context = context;
        self.replace_current(candidate);
    }

    /// Explicit comparison retention keeps the current candidate in history.
    pub fn retain_current(&mut self) {
        let retained = self.candidate().clone();
        let retained_context = self.context.clone();
        self.candidates.push(retained);
        self.candidate_contexts.push(retained_context);
    }

    pub fn select_candidate(&mut self, index: usize) {
        self.selected_candidate = index.min(self.candidates.len().saturating_sub(1));
        self.candidate_cursor = self.selected_candidate;
        self.flow_stage = 0;
    }

    pub fn selected_action(&self) -> PanelAction {
        PanelAction::ALL[self.action_cursor]
    }

    pub fn editor_buffer(&self) -> Option<&str> {
        (self.input_mode == PanelInputMode::CommandEditor).then_some(self.editor_buffer.as_str())
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> PanelOutcome {
        if key.kind != KeyEventKind::Press {
            return PanelOutcome::Continue;
        }
        if self.input_mode == PanelInputMode::CommandEditor {
            return self.handle_editor_key(key);
        }
        self.handle_review_key(key)
    }

    fn handle_review_key(&mut self, key: KeyEvent) -> PanelOutcome {
        if key.code == KeyCode::Tab && key.modifiers.contains(KeyModifiers::SHIFT)
            || key.code == KeyCode::BackTab
        {
            self.focus = self.focus.previous();
            return PanelOutcome::Continue;
        }
        match key.code {
            KeyCode::Tab => {
                self.focus = self.focus.next();
            }
            KeyCode::Esc => return PanelOutcome::Cancelled,
            KeyCode::Up => self.move_selection(-1),
            KeyCode::Down => self.move_selection(1),
            KeyCode::Left | KeyCode::Right => {}
            KeyCode::Enter => return self.activate_selection(),
            _ => {}
        }
        PanelOutcome::Continue
    }

    fn handle_editor_key(&mut self, key: KeyEvent) -> PanelOutcome {
        match key.code {
            KeyCode::Esc => {
                self.editor_buffer.clear();
                self.input_mode = PanelInputMode::Review;
                self.editor_buffer = self.editor_original.clone();
                PanelOutcome::EditDiscarded
            }
            KeyCode::Enter => {
                let command = self.editor_buffer.clone();
                self.candidate_mut().edit_command(command.clone());
                self.input_mode = PanelInputMode::Review;
                self.editor_original.clear();
                PanelOutcome::EditCommitted(command)
            }
            KeyCode::Backspace => {
                self.editor_buffer.pop();
                PanelOutcome::Continue
            }
            KeyCode::Char(character)
                if !key
                    .modifiers
                    .intersects(KeyModifiers::CONTROL | KeyModifiers::ALT) =>
            {
                self.editor_buffer.push(character);
                PanelOutcome::Continue
            }
            _ => PanelOutcome::Continue,
        }
    }

    fn activate_selection(&mut self) -> PanelOutcome {
        match self.focus {
            FocusRegion::Actions => match self.selected_action() {
                PanelAction::Accept => PanelOutcome::Accepted(self.candidate().clone()),
                PanelAction::EditCommand => {
                    self.editor_original = self.candidate().command.clone();
                    self.editor_buffer = self.editor_original.clone();
                    self.input_mode = PanelInputMode::CommandEditor;
                    PanelOutcome::Continue
                }
                PanelAction::Reject => PanelOutcome::Rejected,
                PanelAction::Cancel => PanelOutcome::Cancelled,
            },
            FocusRegion::Candidates => {
                self.selected_candidate = self.candidate_cursor.min(self.candidates.len() - 1);
                self.flow_stage = self
                    .flow_stage
                    .min(self.candidate().flow.stages.len().saturating_sub(1));
                PanelOutcome::Continue
            }
            FocusRegion::Flow => PanelOutcome::Continue,
        }
    }

    fn move_selection(&mut self, delta: isize) {
        match self.focus {
            FocusRegion::Flow => {
                let len = self.candidate().flow.stages.len();
                self.flow_stage = move_cursor(self.flow_stage, delta, len);
            }
            FocusRegion::Candidates => {
                self.candidate_cursor =
                    move_cursor(self.candidate_cursor, delta, self.candidates.len());
            }
            FocusRegion::Actions => {
                self.action_cursor = move_cursor(self.action_cursor, delta, PanelAction::ALL.len());
            }
        }
    }

    fn candidate_mut(&mut self) -> &mut ReviewCandidate {
        &mut self.candidates[self.selected_candidate]
    }
}

fn move_cursor(current: usize, delta: isize, len: usize) -> usize {
    if len == 0 {
        return 0;
    }
    if delta < 0 {
        current.saturating_sub(delta.unsigned_abs()).min(len - 1)
    } else {
        current.saturating_add(delta as usize).min(len - 1)
    }
}

pub struct ControllingTerminal<W: Write> {
    writer: Option<W>,
    layout: InlineLayout,
    rows_rendered: u16,
    owns_raw_mode: bool,
    cursor_hidden: bool,
}

impl<W: Write> ControllingTerminal<W> {
    pub fn new(writer: W, layout: InlineLayout) -> Self {
        Self {
            writer: Some(writer),
            layout,
            rows_rendered: 0,
            owns_raw_mode: false,
            cursor_hidden: false,
        }
    }

    pub fn open(writer: W, layout: InlineLayout) -> io::Result<Self> {
        if !controlling_terminal_is_usable() {
            return Err(io::Error::new(
                io::ErrorKind::NotConnected,
                "review panel requires a controlling terminal",
            ));
        }
        if layout.width < MIN_PANEL_WIDTH || layout.height < 4 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "controlling terminal is too small for the review panel",
            ));
        }
        let raw_mode_before = terminal::is_raw_mode_enabled()?;
        if !raw_mode_before {
            terminal::enable_raw_mode()?;
        }
        let mut terminal = Self {
            writer: Some(writer),
            layout,
            rows_rendered: 0,
            owns_raw_mode: !raw_mode_before,
            cursor_hidden: false,
        };
        if let Err(error) = terminal.begin() {
            let _ = terminal.restore();
            return Err(error);
        }
        Ok(terminal)
    }

    pub fn layout(&self) -> InlineLayout {
        self.layout
    }

    pub fn writer(&self) -> &W {
        self.writer.as_ref().expect("terminal writer is present")
    }

    pub fn writer_mut(&mut self) -> &mut W {
        self.writer.as_mut().expect("terminal writer is present")
    }

    pub fn into_writer(mut self) -> W {
        let _ = self.restore();
        self.writer.take().expect("terminal writer is present")
    }

    fn begin(&mut self) -> io::Result<()> {
        queue!(self.writer_mut(), cursor::Hide)?;
        self.cursor_hidden = true;
        self.writer_mut().flush()
    }

    pub fn restore(&mut self) -> io::Result<()> {
        if self.writer.is_none() {
            return Ok(());
        }
        let mut first_error = None;
        if self.rows_rendered > 0 {
            let rows_rendered = self.rows_rendered;
            if let Err(error) = queue!(
                self.writer_mut(),
                cursor::MoveUp(rows_rendered.saturating_sub(1)),
                cursor::MoveToColumn(0),
                Clear(ClearType::FromCursorDown)
            ) {
                first_error = Some(error);
            }
            self.rows_rendered = 0;
        }
        if self.cursor_hidden {
            if let Err(error) = queue!(self.writer_mut(), cursor::Show) {
                if first_error.is_none() {
                    first_error = Some(error);
                }
            }
            self.cursor_hidden = false;
        }
        if let Err(error) = self.writer_mut().flush() {
            if first_error.is_none() {
                first_error = Some(error);
            }
        }
        if self.owns_raw_mode && terminal::is_raw_mode_enabled().unwrap_or(false) {
            if let Err(error) = terminal::disable_raw_mode() {
                if first_error.is_none() {
                    first_error = Some(error);
                }
            }
        }
        first_error.map_or(Ok(()), Err)
    }

    pub fn render(&mut self, state: &ReviewPanelState) -> io::Result<()> {
        let lines = render_lines(state, self.layout);
        if self.rows_rendered > 0 {
            let rows_rendered = self.rows_rendered;
            queue!(
                self.writer_mut(),
                cursor::MoveUp(rows_rendered.saturating_sub(1)),
                cursor::MoveToColumn(0)
            )?;
        } else {
            queue!(self.writer_mut(), cursor::MoveToColumn(0))?;
        }
        queue!(self.writer_mut(), Clear(ClearType::FromCursorDown))?;
        for (index, line) in lines.iter().enumerate() {
            if index > 0 {
                queue!(self.writer_mut(), Print("\r\n"))?;
            }
            queue!(self.writer_mut(), Print(line))?;
        }
        self.rows_rendered = lines.len() as u16;
        self.writer_mut().flush()
    }
}

impl<W: Write> Drop for ControllingTerminal<W> {
    fn drop(&mut self) {
        let _ = self.restore();
    }
}

pub struct InlineReviewPanel<W: Write> {
    terminal: ControllingTerminal<W>,
    pub state: ReviewPanelState,
}

impl<W: Write> InlineReviewPanel<W> {
    pub fn new(terminal: ControllingTerminal<W>, state: ReviewPanelState) -> Self {
        Self { terminal, state }
    }

    pub fn render(&mut self) -> io::Result<()> {
        self.terminal.render(&self.state)
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> io::Result<PanelOutcome> {
        let outcome = self.state.handle_key(key);
        match outcome {
            PanelOutcome::Accepted(_) | PanelOutcome::Cancelled => self.finish()?,
            _ => self.render()?,
        }
        Ok(outcome)
    }

    pub fn finish(&mut self) -> io::Result<()> {
        self.terminal.restore()
    }

    pub fn terminal(&self) -> &ControllingTerminal<W> {
        &self.terminal
    }

    pub fn terminal_mut(&mut self) -> &mut ControllingTerminal<W> {
        &mut self.terminal
    }
}

pub fn render_lines(state: &ReviewPanelState, layout: InlineLayout) -> Vec<String> {
    let candidate = state.candidate();
    let mut lines = Vec::new();
    lines.push(format!(
        "Review | intent: {}",
        sanitize_terminal_text(&state.context.intent)
    ));
    lines.push(format!(
        "Context | tier {} | {}/{}",
        sanitize_terminal_text(&state.context.tier),
        sanitize_terminal_text(&state.context.provider),
        sanitize_terminal_text(&state.context.model)
    ));
    lines.push(format!(
        "Flow [{}/{}]{}",
        state
            .flow_stage
            .saturating_add(1)
            .min(candidate.flow.stages.len()),
        candidate.flow.stages.len(),
        if candidate.flow.has_unsupported() {
            " | unsupported syntax marked"
        } else {
            ""
        }
    ));

    if let Some(stage) = candidate.flow.stages.get(state.flow_stage) {
        lines.push(format!("> {}", sanitize_terminal_text(&stage.stage_text)));
        if !stage.unsupported_spans.is_empty() {
            lines.push(format!(
                "  unsupported: {}",
                stage
                    .unsupported_spans
                    .iter()
                    .map(|span| sanitize_terminal_text(&span.text))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        let purpose = candidate
            .stages
            .get(state.flow_stage)
            .and_then(|stage| stage.purpose.as_deref())
            .map(sanitize_terminal_text)
            .unwrap_or_else(|| candidate.purpose_status.label().to_string());
        lines.push(format!("  purpose: {purpose}"));
    } else {
        lines.push("> no supported flow stages".to_string());
    }
    lines.push(format!(
        "Candidate: {}",
        sanitize_terminal_text(&candidate.command)
    ));
    lines.push(format!(
        "Candidates [{}/{}]",
        state.selected_candidate + 1,
        state.candidates.len()
    ));
    if state.candidates.len() > 1 {
        for (index, compared) in state.candidates.iter().enumerate() {
            let context = &state.candidate_contexts[index];
            let marker = if index == state.selected_candidate {
                ">"
            } else {
                " "
            };
            lines.push(format!(
                "{marker} {}. {} | tier {} | {}/{}",
                index + 1,
                sanitize_terminal_text(&compared.command),
                sanitize_terminal_text(&context.tier),
                sanitize_terminal_text(&context.provider),
                sanitize_terminal_text(&context.model)
            ));
        }
    }
    if state.input_mode == PanelInputMode::CommandEditor {
        lines.push(format!(
            "Edit command: {}",
            sanitize_terminal_text(&state.editor_buffer)
        ));
    } else {
        lines.push(format!(
            "Focus: {} | Actions: {}",
            state.focus.label(),
            PanelAction::ALL
                .iter()
                .enumerate()
                .map(|(index, action)| {
                    if index == state.action_cursor {
                        format!("[{}]", action.label())
                    } else {
                        action.label().to_string()
                    }
                })
                .collect::<Vec<_>>()
                .join(" | ")
        ));
    }
    if let Some(models) = &state.model_selection {
        lines.push(format!(
            "Models: {}",
            models
                .iter()
                .enumerate()
                .map(|(index, model)| {
                    if index == state.model_cursor {
                        format!("[{model}]")
                    } else {
                        model.clone()
                    }
                })
                .collect::<Vec<_>>()
                .join(" | ")
        ));
    }

    wrap_lines(lines, layout.content_width.max(1), layout.max_rows as usize)
}

fn wrap_lines(lines: Vec<String>, width: usize, max_rows: usize) -> Vec<String> {
    let mut wrapped = Vec::new();
    for line in lines {
        let mut remaining = line.as_str();
        if remaining.is_empty() {
            wrapped.push(String::new());
            continue;
        }
        while remaining.chars().count() > width {
            let split_at = remaining
                .char_indices()
                .nth(width)
                .map(|(index, _)| index)
                .unwrap_or(remaining.len());
            wrapped.push(remaining[..split_at].to_string());
            remaining = &remaining[split_at..];
        }
        wrapped.push(remaining.to_string());
    }
    wrapped.truncate(max_rows.max(1));
    wrapped
}

pub fn sanitize_terminal_text(value: &str) -> String {
    #[derive(Clone, Copy, PartialEq, Eq)]
    enum EscapeState {
        Normal,
        Esc,
        Csi,
        Osc,
        OscEsc,
    }

    let mut sanitized = String::with_capacity(value.len());
    let mut state = EscapeState::Normal;
    for character in value.chars() {
        match state {
            EscapeState::Normal => {
                if character == '\u{1b}' {
                    state = EscapeState::Esc;
                } else if character.is_control()
                    && character != '\n'
                    && character != '\r'
                    && character != '\t'
                {
                    sanitized.push('?');
                } else {
                    sanitized.push(character);
                }
            }
            EscapeState::Esc => {
                state = match character {
                    '[' => EscapeState::Csi,
                    ']' => EscapeState::Osc,
                    _ => EscapeState::Normal,
                };
            }
            EscapeState::Csi => {
                if ('@'..='~').contains(&character) {
                    state = EscapeState::Normal;
                }
            }
            EscapeState::Osc => {
                state = if character == '\u{7}' {
                    EscapeState::Normal
                } else if character == '\u{1b}' {
                    EscapeState::OscEsc
                } else {
                    EscapeState::Osc
                };
            }
            EscapeState::OscEsc => {
                state = if character == '\\' {
                    EscapeState::Normal
                } else {
                    EscapeState::Osc
                };
            }
        }
    }
    sanitized
}

pub fn controlling_terminal_is_usable() -> bool {
    io::stdin().is_terminal()
        && io::stdout().is_terminal()
        && io::stderr().is_terminal()
        && std::env::var("TERM").as_deref() != Ok("dumb")
}

#[cfg(test)]
mod tests {
    use super::{
        render_lines, sanitize_terminal_text, ControllingTerminal, FocusRegion, InlineLayout,
        PanelAction, PanelInputMode, PanelOutcome, ReviewContext, ReviewPanelState,
    };
    use crate::review::ReviewCandidate;
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    fn key(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::NONE)
    }

    fn state() -> ReviewPanelState {
        ReviewPanelState::new(
            ReviewContext {
                intent: "show disk usage".to_string(),
                tier: "1".to_string(),
                provider: "openrouter".to_string(),
                model: "model-a".to_string(),
            },
            ReviewCandidate::from_command("df -h"),
        )
    }

    #[test]
    fn starts_on_accept_and_cycles_exactly_three_focus_regions() {
        let mut state = state();
        assert_eq!(state.focus, FocusRegion::Actions);
        assert_eq!(state.selected_action(), PanelAction::Accept);
        state.handle_key(key(KeyCode::Tab));
        assert_eq!(state.focus, FocusRegion::Flow);
        state.handle_key(key(KeyCode::Tab));
        assert_eq!(state.focus, FocusRegion::Candidates);
        state.handle_key(key(KeyCode::Tab));
        assert_eq!(state.focus, FocusRegion::Actions);
        state.handle_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::SHIFT));
        assert_eq!(state.focus, FocusRegion::Candidates);
    }

    #[test]
    fn arrows_navigate_flow_and_editor_keys_are_local() {
        let mut state = state();
        state.handle_key(key(KeyCode::Tab));
        assert_eq!(state.focus, FocusRegion::Flow);
        state.handle_key(key(KeyCode::Down));
        assert_eq!(state.flow_stage, 0);

        let mut multi_stage = ReviewPanelState::new(
            state.context.clone(),
            ReviewCandidate::from_command("printf one; printf two"),
        );
        multi_stage.handle_key(key(KeyCode::Tab));
        multi_stage.handle_key(key(KeyCode::Down));
        assert_eq!(multi_stage.flow_stage, 1);

        state.focus = FocusRegion::Actions;
        state.action_cursor = 1;
        assert_eq!(
            state.handle_key(key(KeyCode::Enter)),
            PanelOutcome::Continue
        );
        assert_eq!(state.input_mode, PanelInputMode::CommandEditor);
        state.handle_key(key(KeyCode::Char('x')));
        assert!(state.editor_buffer().unwrap().ends_with('x'));
        assert_eq!(
            state.handle_key(key(KeyCode::Esc)),
            PanelOutcome::EditDiscarded
        );
        assert_eq!(state.input_mode, PanelInputMode::Review);
        assert_eq!(state.candidate().command, "df -h");
    }

    #[test]
    fn enter_accepts_and_escape_cancels_without_alternate_screen_commands() {
        let mut accepted_state = state();
        assert!(matches!(
            accepted_state.handle_key(key(KeyCode::Enter)),
            PanelOutcome::Accepted(_)
        ));
        let mut cancelled_state = state();
        assert_eq!(
            cancelled_state.handle_key(key(KeyCode::Esc)),
            PanelOutcome::Cancelled
        );

        let layout = InlineLayout::for_dimensions(80, 24);
        let terminal = ControllingTerminal::new(Vec::new(), layout);
        let mut panel = super::InlineReviewPanel::new(terminal, state());
        panel.render().unwrap();
        let bytes = panel.terminal.into_writer();
        let output = String::from_utf8(bytes).unwrap();
        assert!(!output.contains("1049"));
        assert!(output.contains("Flow"));
    }

    #[test]
    fn narrow_layout_stays_bounded_and_marks_unsupported_stage_text() {
        let candidate = ReviewCandidate::from_command("printf one; cat < input");
        let mut state = ReviewPanelState::new(
            ReviewContext {
                intent: "inspect input".to_string(),
                tier: "1".to_string(),
                provider: "test".to_string(),
                model: "model".to_string(),
            },
            candidate,
        );
        state.flow_stage = 1;
        let layout = InlineLayout::for_dimensions(40, 8);
        let lines = render_lines(&state, layout);

        assert!(layout.is_bounded());
        assert!(lines.len() <= layout.max_rows as usize);
        assert!(lines.iter().any(|line| line.contains("unsupported")));
        assert!(lines.iter().any(|line| line.contains("cat < input")));
    }

    #[test]
    fn terminal_text_sanitization_removes_escape_sequences() {
        let clean = sanitize_terminal_text("ok\u{1b}[31mdanger\u{1b}[0m");
        assert_eq!(clean, "okdanger");
        assert_eq!(
            sanitize_terminal_text("ok\u{1b}]0;danger\u{7}done"),
            "okdone"
        );
        assert!(sanitize_terminal_text("line\u{7}bell").contains('?'));
    }
}
