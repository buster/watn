use super::panel::{sanitize_terminal_text, wrap_text, InlineLayout, ReviewPanelState};

const MAX_CARD_ROWS: u16 = 18;
const NARROW_WIDTH: u16 = 60;

/// Pure color-capability decision so tests can exercise every combination
/// without touching the process environment.
pub fn terminal_supports_color(no_color: bool, term: &str, colorterm: bool) -> bool {
    !no_color && term != "dumb" && (colorterm || term.contains("color"))
}

pub fn color_terminal_supports_card() -> bool {
    let no_color = std::env::var_os("NO_COLOR").is_some();
    let term = std::env::var("TERM").unwrap_or_default();
    let colorterm = std::env::var_os("COLORTERM").is_some();
    terminal_supports_color(no_color, &term, colorterm)
}

struct Ink {
    color: bool,
}

impl Ink {
    fn new(color: bool) -> Self {
        Self { color }
    }

    fn paint(&self, code: &str, text: &str) -> String {
        if self.color {
            format!("\u{1b}[{code}m{text}\u{1b}[0m")
        } else {
            text.to_string()
        }
    }

    fn border(&self, text: &str) -> String {
        self.paint("38;5;240", text)
    }

    fn label(&self, text: &str) -> String {
        self.paint("38;5;81", text)
    }

    fn white(&self, text: &str) -> String {
        self.paint("97", text)
    }

    fn dim(&self, text: &str) -> String {
        self.paint("2", text)
    }

    fn italic_dim(&self, text: &str) -> String {
        self.paint("2;3", text)
    }

    fn yellow(&self, text: &str) -> String {
        self.paint("38;5;221", text)
    }

    fn green(&self, text: &str) -> String {
        self.paint("38;5;114", text)
    }

    fn amber(&self, text: &str) -> String {
        self.paint("38;5;214", text)
    }

    fn reverse(&self, text: &str) -> String {
        self.paint("7", text)
    }
}

fn visible_len(value: &str) -> usize {
    let mut count = 0;
    let mut chars = value.chars();
    while let Some(character) = chars.next() {
        if character == '\u{1b}' {
            for escaped in chars.by_ref() {
                if escaped.is_ascii_alphabetic() {
                    break;
                }
            }
        } else {
            count += 1;
        }
    }
    count
}

fn pad_to(value: &str, width: usize) -> String {
    let mut padded = value.to_string();
    let visible = visible_len(value);
    if visible < width {
        padded.push_str(&" ".repeat(width - visible));
    }
    padded
}

fn ellipsize(value: &str, width: usize) -> String {
    if width == 0 {
        return String::new();
    }
    if value.chars().count() <= width {
        return value.to_string();
    }
    let mut truncated: String = value.chars().take(width.saturating_sub(1)).collect();
    truncated.push('…');
    truncated
}

/// Ellipsize a styled value by visible width, preserving escape sequences.
fn ellipsize_visible(value: &str, width: usize) -> String {
    if visible_len(value) <= width {
        return value.to_string();
    }
    let mut result = String::new();
    let mut visible = 0;
    let mut chars = value.chars();
    while let Some(character) = chars.next() {
        if character == '\u{1b}' {
            result.push(character);
            for escaped in chars.by_ref() {
                result.push(escaped);
                if escaped.is_ascii_alphabetic() {
                    break;
                }
            }
        } else if visible + 1 < width {
            result.push(character);
            visible += 1;
        } else {
            result.push('…');
            break;
        }
    }
    result
}

fn wrap_capped(value: &str, width: usize, max_rows: usize) -> Vec<String> {
    let width = width.max(1);
    let mut rows = wrap_text(value, width);
    if rows.len() > max_rows {
        rows.truncate(max_rows);
        if let Some(last) = rows.last_mut() {
            if last.chars().count() >= width {
                let mut shortened: String = last.chars().take(width.saturating_sub(1)).collect();
                shortened.push('…');
                *last = shortened;
            }
        }
    }
    rows
}

/// Wrap an already-styled value by visible width, preserving escape sequences.
fn wrap_visible(value: &str, width: usize, max_rows: usize) -> Vec<String> {
    let width = width.max(1);
    let mut rows: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut visible = 0;
    let mut chars = value.chars();
    while let Some(character) = chars.next() {
        if character == '\u{1b}' {
            current.push(character);
            for escaped in chars.by_ref() {
                current.push(escaped);
                if escaped.is_ascii_alphabetic() {
                    break;
                }
            }
            continue;
        }
        if visible >= width {
            rows.push(std::mem::take(&mut current));
            visible = 0;
        }
        current.push(character);
        visible += 1;
    }
    if !current.is_empty() || rows.is_empty() {
        rows.push(current);
    }
    if rows.len() > max_rows {
        rows.truncate(max_rows);
        if let Some(last) = rows.last_mut() {
            *last = ellipsize_visible(last, width);
        }
    }
    rows
}

fn colorize_command(command: &str, ink: &Ink) -> String {
    command
        .split_whitespace()
        .map(|token| {
            if matches!(token, "|" | "||" | "&&" | ";" | "&") {
                ink.border(token)
            } else if token.starts_with('-') {
                ink.yellow(token)
            } else if token.starts_with('\'') || token.starts_with('"') {
                ink.green(token)
            } else {
                ink.white(token)
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn purpose_text(state: &ReviewPanelState) -> String {
    let candidate = state.candidate();
    candidate
        .stages
        .get(state.flow_stage)
        .and_then(|stage| stage.purpose.as_deref())
        .map(sanitize_terminal_text)
        .unwrap_or_else(|| candidate.purpose_status.label().to_string())
}

fn flow_strip(state: &ReviewPanelState, ink: &Ink) -> String {
    let stages = &state.candidate().flow.stages;
    stages
        .iter()
        .enumerate()
        .map(|(index, stage)| {
            let number = (index + 1).to_string();
            if index == state.flow_stage {
                ink.reverse(&ink.white(&number))
            } else if stage.support == super::flow::StageSupport::Unsupported {
                ink.amber(&number)
            } else {
                ink.dim(&number)
            }
        })
        .collect::<Vec<_>>()
        .join(" · ")
}

fn header_right(state: &ReviewPanelState) -> String {
    format!(
        "◆ tier {} · {}/{}",
        state.context.tier, state.context.provider, state.context.model
    )
}

pub fn render_card_lines(
    state: &ReviewPanelState,
    layout: InlineLayout,
    color: bool,
) -> Vec<String> {
    let ink = Ink::new(color);
    let width = layout.width.max(24) as usize;
    let narrow = layout.width < NARROW_WIDTH;
    let max_rows = ((layout.height.saturating_sub(1)).clamp(5, MAX_CARD_ROWS)) as usize;
    let inner = width.saturating_sub(4);

    let mut content: Vec<(u8, String)> = Vec::new();
    let label_width = 7;

    if !narrow {
        content.push((
            50,
            format!(
                "{}   {}",
                ink.label(&pad_to("Intent", label_width)),
                ink.dim(&ellipsize(
                    &sanitize_terminal_text(&state.context.intent),
                    inner - label_width - 3
                ))
            ),
        ));
        content.push((10, String::new()));

        let command = colorize_command(&state.candidate().command, &ink);
        for (index, row) in wrap_visible(&command, inner - label_width - 3, 2)
            .iter()
            .enumerate()
        {
            let label = if index == 0 {
                ink.label(&pad_to("Command", label_width))
            } else {
                " ".repeat(label_width)
            };
            content.push((55, format!("{label}   {row}")));
        }
        content.push((10, String::new()));
    }

    content.push((
        100,
        format!(
            "{}   {}",
            ink.label(&pad_to("Flow", label_width)),
            flow_strip(state, &ink)
        ),
    ));

    let stage_count = state.candidate().flow.stages.len();
    let stage_text = state
        .candidate()
        .flow
        .stages
        .get(state.flow_stage)
        .map(|stage| stage.stage_text.clone())
        .unwrap_or_else(|| "no supported flow stages".to_string());
    let stage_rows = wrap_capped(
        &sanitize_terminal_text(&stage_text),
        inner - label_width - 8,
        2,
    );
    let stage_number = format!(
        "{}/{}",
        (state.flow_stage + 1).min(stage_count.max(1)),
        stage_count
    );
    let mut marker_on_own_row = false;
    let unsupported_stage = state
        .candidate()
        .flow
        .stages
        .get(state.flow_stage)
        .is_some_and(|stage| stage.support == super::flow::StageSupport::Unsupported);
    for (index, row) in stage_rows.iter().enumerate() {
        if index == 0 {
            let mut line = format!(
                "{}   {} {}",
                ink.label(&pad_to("Stage", label_width)),
                ink.dim(&stage_number),
                ink.white(row)
            );
            if unsupported_stage {
                let marker = format!("  {}", ink.amber("⚠ unsupported"));
                if visible_len(&line) + visible_len(&marker) <= inner.saturating_sub(2) {
                    line.push_str(&marker);
                } else {
                    marker_on_own_row = true;
                }
            }
            content.push((90, line));
        } else {
            content.push((
                85,
                format!("{}       {}", " ".repeat(label_width), ink.white(row)),
            ));
        }
    }
    if marker_on_own_row {
        content.push((
            65,
            format!(
                "{}   {}",
                " ".repeat(label_width),
                ink.amber("⚠ unsupported")
            ),
        ));
    }
    for row in wrap_capped(&purpose_text(state), inner - label_width - 3, 2) {
        content.push((
            70,
            format!("{}   {}", " ".repeat(label_width), ink.italic_dim(&row)),
        ));
    }

    if state.input_mode == super::panel::PanelInputMode::CommandEditor {
        let buffer = sanitize_terminal_text(state.editor_buffer().unwrap_or(""));
        let cursor = super::panel::char_index_to_byte(&buffer, state.editor_cursor());
        let (before, after) = buffer.split_at(cursor);
        content.push((
            95,
            format!(
                "{}   {}",
                ink.label(&pad_to("Edit", label_width)),
                ink.white(&format!("{before}▏{after}"))
            ),
        ));
    }

    if let Some(chooser) = state.chooser() {
        if !chooser.tiers.is_empty() {
            let tiers = chooser
                .tiers
                .iter()
                .enumerate()
                .map(|(index, choice)| {
                    let label = format!("[{} {} · {}]", index + 1, choice.label, choice.model);
                    if index == 0 {
                        ink.reverse(&ink.white(&label))
                    } else {
                        ink.white(&label)
                    }
                })
                .collect::<Vec<_>>()
                .join("  ");
            content.push((
                75,
                format!("{}   {tiers}", ink.label(&pad_to("Models", label_width))),
            ));
        }
        let suggestions: Vec<String> = chooser
            .filtered()
            .into_iter()
            .take(4)
            .enumerate()
            .map(|(index, model)| {
                if Some(index) == chooser.highlight {
                    ink.reverse(&ink.white(model))
                } else {
                    ink.white(model)
                }
            })
            .collect();
        if chooser.catalog_loading {
            content.push((
                70,
                format!(
                    "{}   {}",
                    ink.label(&pad_to("Matches", label_width)),
                    ink.dim("loading suggestions…")
                ),
            ));
        } else if !suggestions.is_empty() {
            content.push((
                70,
                format!(
                    "{}   {}",
                    ink.label(&pad_to("Matches", label_width)),
                    suggestions.join("  ")
                ),
            ));
        }
        let cursor = super::panel::char_index_to_byte(&chooser.query, chooser.query_cursor);
        let (before, after) = chooser.query.split_at(cursor);
        content.push((
            85,
            format!(
                "{}   {}",
                ink.label(&pad_to("Model", label_width)),
                ink.white(&format!("{before}▏{after}"))
            ),
        ));
    }

    if let Some(error) = state.regeneration_error() {
        content.push((
            95,
            format!(
                "{}   {}",
                ink.label(&pad_to("Error", label_width)),
                ink.amber(&ellipsize(
                    &sanitize_terminal_text(error),
                    inner - label_width - 3
                ))
            ),
        ));
    }

    content.push((10, String::new()));
    let hints = if state.input_mode == super::panel::PanelInputMode::CommandEditor {
        ink.dim("⏎ commit · esc discard")
    } else if state.input_mode == super::panel::PanelInputMode::ModelChooser {
        ink.dim("1-3 tier · ⏎ choose · esc close")
    } else if state.explain_only {
        ink.dim("esc close")
    } else {
        format!(
            "{} · {}",
            ink.green("⏎/a accept"),
            ink.dim("e edit · r reject · c cancel · d disable · esc cancel")
        )
    };
    content.push((30, hints));

    let available = max_rows.saturating_sub(2).max(3);
    while content.len() > available {
        let lowest = content
            .iter()
            .enumerate()
            .min_by_key(|(index, (priority, _))| (*priority, *index))
            .map(|(index, _)| index)
            .unwrap_or(0);
        content.remove(lowest);
    }

    let top_left = format!(" {} · {} ", ink.white("watn"), ink.dim("review"));
    let right = ellipsize(
        &header_right(state),
        inner
            .saturating_sub(visible_len(&top_left))
            .saturating_sub(2),
    );
    let right = ink.border(&right);
    let fill = inner.saturating_sub(visible_len(&top_left) + visible_len(&right));
    let mut lines = vec![format!(
        "{}{}{}{}{}",
        ink.border("┌"),
        top_left,
        ink.border(&"─".repeat(fill)),
        right,
        ink.border("┐")
    )];
    for (_, row) in content {
        lines.push(format!(
            "{} {} {}",
            ink.border("│"),
            pad_to(&ellipsize_visible(&row, inner), inner),
            ink.border("│")
        ));
    }
    lines.push(format!(
        "{}{}{}",
        ink.border("└"),
        ink.border(&"─".repeat(inner + 2)),
        ink.border("┘")
    ));
    lines
}

#[cfg(test)]
mod tests {
    use super::{render_card_lines, terminal_supports_color};
    use crate::review::{InlineLayout, ReviewCandidate, ReviewContext, ReviewPanelState};

    fn state() -> ReviewPanelState {
        ReviewPanelState::new(
            ReviewContext {
                intent: "inspect recent commits".to_string(),
                tier: "1".to_string(),
                provider: "loopback".to_string(),
                model: "review-model".to_string(),
            },
            ReviewCandidate::from_command("git log --oneline | head -5"),
        )
    }

    #[test]
    fn color_capability_is_decided_without_environment_side_effects() {
        assert!(terminal_supports_color(false, "xterm-256color", false));
        assert!(terminal_supports_color(false, "xterm", true));
        assert!(!terminal_supports_color(true, "xterm-256color", true));
        assert!(!terminal_supports_color(false, "dumb", true));
        assert!(!terminal_supports_color(false, "vt100", false));
    }

    #[test]
    fn card_frames_content_and_colors_roles() {
        let layout = InlineLayout::for_dimensions(100, 40);
        let lines = render_card_lines(&state(), layout, true);
        let joined = lines.join("\n");

        assert!(joined.contains('┌') && joined.contains('┘'));
        assert!(joined.contains("watn"));
        assert!(joined.contains("Intent"));
        assert!(joined.contains("Command"));
        assert!(joined.contains("Flow"));
        assert!(joined.contains("Stage"));
        assert!(joined.contains("accept"));
        assert!(joined.contains("esc cancel"));
        assert!(joined.contains("\u{1b}[38;5;81m"), "labels are colored");
        assert!(joined.contains("\u{1b}[38;5;221m"), "flags are colored");
        assert!(
            joined.contains("\u{1b}[38;5;240m"),
            "separators are colored"
        );
        assert!(joined.contains("◆ tier 1 · loopback/review-model"));
    }

    #[test]
    fn card_stays_bounded_and_marks_unsupported_stages() {
        let mut unsupported = state();
        unsupported.candidate = ReviewCandidate::from_command("printf one; cat < input");
        unsupported.flow_stage = 1;
        let layout = InlineLayout::for_dimensions(40, 8);
        let lines = render_card_lines(&unsupported, layout, true);

        assert!(lines.len() <= 7, "card rows: {}", lines.len());
        assert!(lines.iter().any(|line| line.contains("unsupported")));
        assert!(!lines.iter().any(|line| line.contains('\n')));
    }

    #[test]
    fn plain_card_has_no_escape_sequences() {
        let layout = InlineLayout::for_dimensions(100, 40);
        let lines = render_card_lines(&state(), layout, false);
        assert!(lines.iter().all(|line| !line.contains('\u{1b}')));
    }
}
