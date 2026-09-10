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
    DisableReviewPermanently,
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
    pub candidate: ReviewCandidate,
    pub flow_stage: usize,
    pub input_mode: PanelInputMode,
    pub explain_only: bool,
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
        Self {
            configured_model: context.model.clone(),
            context,
            candidate,
            flow_stage: 0,
            input_mode: PanelInputMode::Review,
            explain_only: false,
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
        &self.candidate
    }

    pub fn intent_history(&self) -> &[String] {
        &self.intent_history
    }

    /// Rephrasing replaces the visible active Intent and starts a new candidate
    /// cycle. The prior Intent remains only in current-review history.
    pub fn rephrase_intent(&mut self, intent: impl Into<String>) {
        self.intent_history.push(self.context.intent.clone());
        self.context.intent = intent.into();
        self.flow_stage = 0;
    }

    /// Regeneration and escalation replace the current candidate by default.
    pub fn replace_current(&mut self, candidate: ReviewCandidate) {
        self.candidate = candidate;
        self.flow_stage = 0;
    }

    /// Higher-tier escalation generates a candidate in the next tier context
    /// while preserving the current intent.
    pub fn escalate(&mut self, context: ReviewContext, candidate: ReviewCandidate) {
        self.context = context;
        self.replace_current(candidate);
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
        let plain = !key
            .modifiers
            .intersects(KeyModifiers::CONTROL | KeyModifiers::ALT);
        match key.code {
            KeyCode::Esc => PanelOutcome::Cancelled,
            KeyCode::Up | KeyCode::Left => {
                self.move_flow_stage(-1);
                PanelOutcome::Continue
            }
            KeyCode::Down | KeyCode::Right => {
                self.move_flow_stage(1);
                PanelOutcome::Continue
            }
            KeyCode::Char('e') | KeyCode::Char('E') if plain => self.begin_editor(),
            KeyCode::Char('d') | KeyCode::Char('D') if plain => {
                PanelOutcome::DisableReviewPermanently
            }
            KeyCode::Enter => PanelOutcome::Accepted(self.candidate.clone()),
            _ => PanelOutcome::Continue,
        }
    }

    fn begin_editor(&mut self) -> PanelOutcome {
        self.editor_original = self.candidate.command.clone();
        self.editor_buffer = self.editor_original.clone();
        self.input_mode = PanelInputMode::CommandEditor;
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
                self.candidate.edit_command(command.clone());
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

    fn move_flow_stage(&mut self, delta: isize) {
        let len = self.candidate.flow.stages.len();
        self.flow_stage = move_cursor(self.flow_stage, delta, len);
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

    pub fn begin(&mut self) -> io::Result<()> {
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

    pub fn render_lines(&mut self, lines: &[String]) -> io::Result<()> {
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
    color: bool,
}

impl<W: Write> InlineReviewPanel<W> {
    pub fn new(terminal: ControllingTerminal<W>, state: ReviewPanelState) -> Self {
        Self {
            terminal,
            state,
            color: true,
        }
    }

    pub fn with_color(
        terminal: ControllingTerminal<W>,
        state: ReviewPanelState,
        color: bool,
    ) -> Self {
        Self {
            terminal,
            state,
            color,
        }
    }

    pub fn render(&mut self) -> io::Result<()> {
        let lines =
            crate::review::render_card_lines(&self.state, self.terminal.layout(), self.color);
        self.terminal.render_lines(&lines)
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

/// Wrap one logical value into rows of at most `width` characters.
pub fn wrap_text(text: &str, width: usize) -> Vec<String> {
    let width = width.max(1);
    let mut wrapped = Vec::new();
    let mut remaining = text;
    if remaining.is_empty() {
        return vec![String::new()];
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
                } else if character == '\n' || character == '\r' || character == '\t' {
                    sanitized.push(' ');
                } else if character.is_control() {
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
        && io::stderr().is_terminal()
        && std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open("/dev/tty")
            .is_ok()
        && std::env::var("TERM").as_deref() != Ok("dumb")
}

#[cfg(test)]
mod tests {
    use super::{
        sanitize_terminal_text, ControllingTerminal, InlineLayout, PanelInputMode, PanelOutcome,
        ReviewContext, ReviewOperation, ReviewPanelState,
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
    fn starts_on_the_first_stage_with_no_focus_region() {
        let mut state = state();
        assert_eq!(state.flow_stage, 0);
        state.handle_key(key(KeyCode::Right));
        assert_eq!(state.flow_stage, 0, "single stage cannot move");
        assert_eq!(state.input_mode, PanelInputMode::Review);
    }

    #[test]
    fn arrows_move_stages_and_enter_accepts() {
        let mut state = ReviewPanelState::new(
            state().context.clone(),
            ReviewCandidate::from_command("printf one; printf two"),
        );
        state.handle_key(key(KeyCode::Right));
        assert_eq!(state.flow_stage, 1);
        state.handle_key(key(KeyCode::Left));
        assert_eq!(state.flow_stage, 0);
        state.handle_key(key(KeyCode::Down));
        assert_eq!(state.flow_stage, 1);
        state.handle_key(key(KeyCode::Up));
        assert_eq!(state.flow_stage, 0);

        assert!(matches!(
            state.handle_key(key(KeyCode::Enter)),
            PanelOutcome::Accepted(_)
        ));
    }

    #[test]
    fn edit_shortcut_opens_the_command_editor_and_escape_discards() {
        let mut state = state();
        assert_eq!(
            state.handle_key(key(KeyCode::Char('e'))),
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
    fn enter_commits_the_editor_and_disable_is_permanent() {
        let mut commit = state();
        commit.handle_key(key(KeyCode::Char('e')));
        commit.handle_key(key(KeyCode::Char('x')));
        assert_eq!(
            commit.handle_key(key(KeyCode::Enter)),
            PanelOutcome::EditCommitted("df -hx".to_string())
        );
        assert_eq!(commit.candidate().command, "df -hx");

        let mut disable = state();
        assert_eq!(
            disable.handle_key(key(KeyCode::Char('d'))),
            PanelOutcome::DisableReviewPermanently
        );
    }

    #[test]
    fn escape_cancels_and_release_events_are_ignored() {
        let mut state = state();
        assert_eq!(state.handle_key(key(KeyCode::Esc)), PanelOutcome::Cancelled);

        let mut release = KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE);
        release.kind = crossterm::event::KeyEventKind::Release;
        assert_eq!(state.handle_key(release), PanelOutcome::Continue);
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
        let lines = crate::review::render_card_lines(&state, layout, true);

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

    #[test]
    fn sanitization_flattens_row_breaking_characters() {
        assert_eq!(sanitize_terminal_text("a\nb\tc\rd"), "a b c d");
        assert_eq!(sanitize_terminal_text("{\n  \"a\": 1\n}"), "{   \"a\": 1 }");
    }

    #[test]
    fn model_selection_replaces_the_candidate_and_restores_the_model() {
        let mut panel = state();
        panel.open_model_selection(vec!["model-a".to_string(), "model-b".to_string()]);
        assert!(panel.model_selection().is_some());
        let lines =
            crate::review::render_card_lines(&panel, InlineLayout::for_dimensions(80, 24), true);
        assert!(lines.iter().any(|line| line.contains("Models")));
        panel.select_model("model-b", ReviewCandidate::from_command("ls -la"));
        assert_eq!(panel.candidate().command, "ls -la");
        assert!(panel.model_selection().is_none());
        panel.complete_model_selection();
        assert_eq!(panel.context.model, panel.configured_model());
    }

    #[test]
    fn rephrase_and_operations_preserve_the_candidate() {
        let mut panel = state();
        panel.rephrase_intent("list files");
        assert_eq!(panel.intent_history(), ["show disk usage".to_string()]);
        assert_eq!(panel.context.intent, "list files");
        assert_eq!(panel.candidate().command, "df -h");

        panel.begin_operation(ReviewOperation::ModelSelection);
        assert_eq!(
            panel.pending_operation(),
            Some(ReviewOperation::ModelSelection)
        );
        assert_eq!(
            panel.interrupt_operation(),
            Some(ReviewOperation::ModelSelection)
        );
        assert_eq!(panel.pending_operation(), None);
        assert_eq!(panel.candidate().command, "df -h");
    }

    #[test]
    fn escalation_replaces_the_candidate() {
        let mut panel = state();
        let context = panel.context.clone();
        panel.escalate(context, ReviewCandidate::from_command("ls -l"));
        assert_eq!(panel.candidate().command, "ls -l");
    }

    #[test]
    fn editor_edges_are_covered() {
        let mut panel = state();
        panel.input_mode = PanelInputMode::CommandEditor;
        panel.editor_buffer = "df -h".to_string();
        panel.handle_key(KeyEvent::new(KeyCode::Char('x'), KeyModifiers::ALT));
        panel.handle_key(key(KeyCode::Backspace));
        assert_eq!(panel.handle_key(key(KeyCode::F(1))), PanelOutcome::Continue);

        assert_eq!(super::move_cursor(0, 1, 0), 0);
        assert_eq!(super::move_cursor(0, -1, 0), 0);
        assert_eq!(super::wrap_text("", 2), vec![String::new()]);
        assert_eq!(
            super::wrap_text("abcdef", 2),
            vec!["ab".to_string(), "cd".to_string(), "ef".to_string()]
        );
    }

    #[test]
    fn inline_panel_renders_and_dispatches_through_its_terminal() {
        let layout = InlineLayout::for_dimensions(80, 24);
        let terminal = ControllingTerminal::new(Vec::new(), layout);
        let mut panel = super::InlineReviewPanel::new(terminal, state());
        assert_eq!(panel.terminal().layout().width, 80);
        panel.terminal_mut().begin().unwrap();
        panel.render().unwrap();
        let outcome = panel
            .handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE))
            .unwrap();
        assert_eq!(outcome, PanelOutcome::Continue);
        let outcome = panel.handle_key(key(KeyCode::Enter)).unwrap();
        assert!(matches!(outcome, PanelOutcome::Accepted(_)));
    }

    #[test]
    fn empty_candidate_flow_renders_the_overview_placeholder() {
        let panel = ReviewPanelState::new(
            ReviewContext {
                intent: "no command".to_string(),
                tier: "1".to_string(),
                provider: "loopback".to_string(),
                model: "model".to_string(),
            },
            ReviewCandidate::from_command(""),
        );
        let lines =
            crate::review::render_card_lines(&panel, InlineLayout::for_dimensions(80, 24), true);
        assert!(lines
            .iter()
            .any(|line| line.contains("no supported flow stages")));
    }

    #[test]
    fn terminal_sanitization_handles_escape_terminator_variants() {
        assert_eq!(sanitize_terminal_text("ok\u{1b}Xdone"), "okdone");
        assert_eq!(sanitize_terminal_text("ok\u{1b}]0;t\u{1b}\\done"), "okdone");
        assert_eq!(
            sanitize_terminal_text("ok\u{1b}]0;t\u{1b}X\u{7}done"),
            "okdone"
        );
    }

    #[test]
    fn control_modified_letters_are_not_review_decisions() {
        let mut panel = state();
        let control_e = KeyEvent::new(KeyCode::Char('e'), KeyModifiers::CONTROL);
        assert_eq!(panel.handle_key(control_e), PanelOutcome::Continue);
        assert_eq!(panel.input_mode, PanelInputMode::Review);
        let alt_d = KeyEvent::new(KeyCode::Char('d'), KeyModifiers::ALT);
        assert_eq!(panel.handle_key(alt_d), PanelOutcome::Continue);
    }
}
