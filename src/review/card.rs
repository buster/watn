use super::panel::{sanitize_terminal_text, InlineLayout, ReviewPanelState};

const MAX_CARD_ROWS: u16 = 18;
const HIDDEN_STAGE_MARKER: &str = "⋮";
const SELECTED_STAGE_MARKER: &str = "▶";

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

    fn key(&self, text: &str) -> String {
        self.paint("1;38;5;81", text)
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

    fn purpose_marker(&self, text: &str) -> String {
        self.paint("38;5;81", text)
    }

    fn purpose(&self, text: &str) -> String {
        self.paint("38;5;252", text)
    }
}

fn visible_len(value: &str) -> usize {
    let mut width = 0;
    let mut chars = value.chars();
    while let Some(character) = chars.next() {
        if character == '\u{1b}' {
            for escaped in chars.by_ref() {
                if escaped.is_ascii_alphabetic() {
                    break;
                }
            }
        } else {
            width += unicode_width::UnicodeWidthChar::width(character).unwrap_or(0);
        }
    }
    width
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

fn char_offset(value: &str, count: usize) -> usize {
    value
        .char_indices()
        .nth(count)
        .map(|(index, _)| index)
        .unwrap_or(value.len())
}

/// Word-aware wrapping for one stage. A single word wider than the row is
/// hard-split at a character boundary so the row width stays exact.
fn wrap_stage(text: &str, width: usize) -> Vec<String> {
    let width = width.max(1);
    let mut rows: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut current_len = 0usize;
    for word in text.split_whitespace() {
        let word_len = word.chars().count();
        let projected = if current.is_empty() {
            word_len
        } else {
            current_len + 1 + word_len
        };
        if projected <= width {
            if !current.is_empty() {
                current.push(' ');
                current_len += 1;
            }
            current.push_str(word);
            current_len += word_len;
            continue;
        }
        if !current.is_empty() {
            rows.push(std::mem::take(&mut current));
        }
        if word_len <= width {
            current.push_str(word);
            current_len = word_len;
        } else {
            let mut rest = word;
            while rest.chars().count() > width {
                let split = char_offset(rest, width);
                rows.push(rest[..split].to_string());
                rest = &rest[split..];
            }
            current.push_str(rest);
            current_len = rest.chars().count();
        }
    }
    if !current.is_empty() || rows.is_empty() {
        rows.push(current);
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

/// The user-facing instruction printed on stderr before a permanently released
/// candidate. The escaped version is only used on a color-capable terminal.
pub fn disable_hint(use_color: bool) -> String {
    let label = "⚠ review panel disabled";
    let command = "watn --review-panel";
    if use_color {
        format!(
            "\u{1b}[1;38;5;214m{label}\u{1b}[0m — re-enable with: \u{1b}[1;97m{command}\u{1b}[0m"
        )
    } else {
        format!("{label} — re-enable with: {command}")
    }
}

/// The model label used by the simple view: the last `/`-separated identifier
/// segment, without a leading routing marker or a `:` variant suffix.
pub fn model_short_name(model: &str) -> String {
    let last = model.rsplit('/').next().unwrap_or(model);
    let last = last.strip_prefix('~').unwrap_or(last);
    let short = last.split(':').next().unwrap_or(last);
    if short.is_empty() {
        model.to_string()
    } else {
        short.to_string()
    }
}

fn header_right(state: &ReviewPanelState, detailed: bool) -> String {
    if detailed {
        format!(
            "◆ tier {} · {}/{}",
            state.context.tier, state.context.provider, state.context.model
        )
    } else {
        format!("◆ {}", model_short_name(&state.context.model))
    }
}

/// One stage as a row group: the selected stage carries an amber arrow, the
/// stage text wraps word-aware, and the operator that joined this stage to the
/// next one is shown at the end of its last row in amber.
fn stage_group_rows(
    state: &ReviewPanelState,
    index: usize,
    text_width: usize,
    ink: &Ink,
) -> Vec<String> {
    let stage = &state.candidate().flow.stages[index];
    let selected = index == state.flow_stage;
    let separator = stage.separator.as_deref();
    let reserve = separator
        .map(|value| value.chars().count() + 1)
        .unwrap_or(0);
    let wrap_width = text_width.saturating_sub(reserve).max(1);
    let wrapped = wrap_stage(&stage.stage_text, wrap_width);
    let last_index = wrapped.len().saturating_sub(1);
    let mut rows = Vec::with_capacity(wrapped.len());
    for (row_index, row) in wrapped.iter().enumerate() {
        let mut line = String::new();
        if row_index == 0 {
            if selected {
                line.push_str("  ");
                line.push_str(&ink.amber(SELECTED_STAGE_MARKER));
                line.push(' ');
            } else {
                line.push_str("    ");
            }
        } else {
            line.push_str("    ");
        }
        line.push_str(&colorize_command(row, ink));
        if row_index == last_index {
            if let Some(separator) = separator {
                line.push(' ');
                line.push_str(&ink.amber(separator));
            }
        }
        rows.push(line);
    }
    rows
}

fn hidden_marker_row(ink: &Ink) -> String {
    format!("    {}", ink.dim(HIDDEN_STAGE_MARKER))
}

/// Window the stage groups so the selected stage always fits the budget.
/// Hidden edges are marked with a dim `⋮`; a selected group that alone exceeds
/// the budget is truncated with an unstyled `…`.
fn stack_window_rows(
    state: &ReviewPanelState,
    text_width: usize,
    budget: usize,
    ink: &Ink,
) -> Vec<String> {
    let stages = &state.candidate().flow.stages;
    if stages.is_empty() {
        return vec![format!("    {}", ink.dim("no flow stages"))];
    }
    let selected = state.flow_stage.min(stages.len() - 1);
    let groups: Vec<Vec<String>> = (0..stages.len())
        .map(|index| stage_group_rows(state, index, text_width, ink))
        .collect();
    let heights: Vec<usize> = groups.iter().map(Vec::len).collect();
    let budget = budget.max(1);
    if heights[selected] > budget {
        let mut rows = groups[selected].clone();
        rows.truncate(budget);
        if let Some(last) = rows.last_mut() {
            last.push('…');
        }
        return rows;
    }
    let mut lo = selected;
    let mut hi = selected;
    let mut used = heights[selected];
    loop {
        let mut expanded = false;
        if lo > 0 {
            let edge = usize::from(lo > 0) + usize::from(hi + 1 < groups.len());
            if used + heights[lo - 1] + edge <= budget {
                lo -= 1;
                used += heights[lo];
                expanded = true;
            }
        }
        if hi + 1 < groups.len() {
            let edge = usize::from(lo > 0) + usize::from(hi + 2 < groups.len());
            if used + heights[hi + 1] + edge <= budget {
                hi += 1;
                used += heights[hi];
                expanded = true;
            }
        }
        if !expanded {
            break;
        }
    }
    let mut rows = Vec::new();
    if lo > 0 {
        rows.push(hidden_marker_row(ink));
    }
    for group in groups.iter().take(hi + 1).skip(lo) {
        rows.extend(group.iter().cloned());
    }
    if hi + 1 < groups.len() {
        rows.push(hidden_marker_row(ink));
    }
    rows
}

fn hints(state: &ReviewPanelState, detailed: bool, ink: &Ink) -> String {
    let keyed = |key: &str, label: &str| format!("{}{}", ink.key(key), ink.dim(label));
    match state.input_mode {
        super::panel::PanelInputMode::CommandEditor => {
            format!("{} · {}", keyed("⏎", " commit"), keyed("esc", " discard"))
        }
        super::panel::PanelInputMode::ModelChooser => format!(
            "{} · {} · {} · {} · {}",
            keyed("1/2/3", " switch tier"),
            ink.dim("type to filter"),
            keyed("↑↓", " pick"),
            keyed("⏎", " use"),
            keyed("esc", " back")
        ),
        super::panel::PanelInputMode::Review if state.explain_only => ink.dim("esc close"),
        super::panel::PanelInputMode::Review if detailed => {
            let parts = [
                format!("{} {}", ink.key("⏎"), ink.dim("accept")),
                format!("{}{}", ink.key("↑↓"), ink.dim(" stage")),
                format!("{}{}", ink.key("e"), ink.dim("dit")),
                format!("{}{}", ink.key("r"), ink.dim("eject")),
                format!("{}{}", ink.key("d/?"), ink.dim(" simple")),
                format!("{}{}", ink.key("D"), ink.dim(" disable review")),
                ink.key("esc"),
            ];
            parts.join(&ink.dim(" · "))
        }
        super::panel::PanelInputMode::Review => {
            let parts = [
                format!("{} {}", ink.key("⏎"), ink.dim("accept")),
                format!("{} {}", ink.key("↑↓"), ink.dim("stage")),
                format!("{} {}", ink.key("d/?"), ink.dim("details")),
                ink.key("esc"),
            ];
            parts.join(&ink.dim(" · "))
        }
    }
}

pub fn render_card_lines(
    state: &ReviewPanelState,
    layout: InlineLayout,
    color: bool,
) -> Vec<String> {
    let ink = Ink::new(color);
    let width = layout.width.max(24) as usize;
    let max_rows = ((layout.height.saturating_sub(1)).clamp(5, MAX_CARD_ROWS)) as usize;
    let inner = width.saturating_sub(4);
    let detailed = state.details
        || state.explain_only
        || state.input_mode != super::panel::PanelInputMode::Review;
    let label_width = 7;
    let available = max_rows.saturating_sub(2).max(3);

    let mut content: Vec<(u8, String)> = Vec::new();

    if detailed {
        content.push((
            50,
            format!(
                "{}   {}",
                ink.label(&pad_to("Intent", label_width)),
                ink.dim(&ellipsize(
                    &sanitize_terminal_text(&state.context.intent),
                    inner.saturating_sub(label_width + 3)
                ))
            ),
        ));
        content.push((10, String::new()));
    } else {
        content.push((10, String::new()));
    }

    let purpose_indent = if detailed {
        " ".repeat(label_width + 3 + 4)
    } else {
        "    ".to_string()
    };
    let purpose_width = inner.saturating_sub(purpose_indent.len() + 2);
    let purpose_rows: Vec<String> = wrap_stage(&purpose_text(state), purpose_width)
        .into_iter()
        .take(2)
        .collect();

    let essential_rows = purpose_rows.len() + 4;
    let stack_budget = available.saturating_sub(essential_rows).max(1);

    let base = if detailed {
        " ".repeat(label_width + 3)
    } else {
        String::new()
    };
    let stack_text_width = inner.saturating_sub(base.len() + 4);
    let stack = stack_window_rows(state, stack_text_width, stack_budget, &ink);
    for (index, row) in stack.iter().enumerate() {
        let line = if detailed && index == 0 {
            format!(
                "{}   {base}{row}",
                ink.label(&pad_to("Command", label_width))
            )
        } else {
            format!("{base}{row}")
        };
        content.push((if index == 0 { 100 } else { 95 }, line));
    }

    if !detailed {
        content.push((10, String::new()));
    }
    for (index, row) in purpose_rows.iter().enumerate() {
        let marker = if index == 0 {
            format!("{} ", ink.purpose_marker("↳"))
        } else {
            "  ".to_string()
        };
        content.push((70, format!("{purpose_indent}{marker}{}", ink.purpose(row))));
    }

    if detailed {
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
                        let marker = if choice.model == state.context.model {
                            format!(" {}", ink.green("●"))
                        } else {
                            String::new()
                        };
                        format!(
                            "{} {} · {}{marker}",
                            ink.key(&(index + 1).to_string()),
                            choice.label,
                            choice.model
                        )
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
                        ink.paint("7", &ink.white(model))
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
                        ink.label(&pad_to("Picks", label_width)),
                        ink.dim("loading suggestions…")
                    ),
                ));
            } else if !suggestions.is_empty() {
                content.push((
                    70,
                    format!(
                        "{}   {}",
                        ink.label(&pad_to("Picks", label_width)),
                        suggestions.join("  ")
                    ),
                ));
            }
            let search = if chooser.query.is_empty() {
                format!("{}▏", ink.dim("type a model name…"))
            } else {
                let cursor = super::panel::char_index_to_byte(&chooser.query, chooser.query_cursor);
                let (before, after) = chooser.query.split_at(cursor);
                ink.white(&format!("{before}▏{after}"))
            };
            content.push((
                85,
                format!("{}   {search}", ink.label(&pad_to("Search", label_width))),
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
                        inner.saturating_sub(label_width + 3)
                    ))
                ),
            ));
        }
    }

    content.push((10, String::new()));
    content.push((30, hints(state, detailed, &ink)));

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
        &header_right(state, detailed),
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
    use super::{model_short_name, render_card_lines, terminal_supports_color};
    use crate::review::{InlineLayout, ReviewCandidate, ReviewContext, ReviewPanelState};
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

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
    fn model_short_name_strips_provider_and_variant() {
        assert_eq!(model_short_name("gpt-4o"), "gpt-4o");
        assert_eq!(
            model_short_name("~anthropic/claude-haiku-latest:nitro"),
            "claude-haiku-latest"
        );
        assert_eq!(
            model_short_name("~deepseek/deepseek-v4-flash-latest"),
            "deepseek-v4-flash-latest"
        );
        assert_eq!(model_short_name(""), "");
    }

    #[test]
    fn simple_card_names_the_model_and_stacks_stages() {
        let lines = render_card_lines(&state(), InlineLayout::for_dimensions(100, 40), true);
        let joined = lines.join("\n");

        assert!(joined.contains("◆ review-model"));
        assert!(!joined.contains("Intent"));
        assert!(!joined.contains("tier"));
        assert!(joined.contains("oneline"), "the first stage is stacked");
        assert!(joined.contains("head"), "the second stage is stacked");
        assert!(joined.contains("-5"));
        assert!(
            joined.contains("\u{1b}[38;5;214m|\u{1b}[0m"),
            "the separator is amber on the stage row"
        );
        assert!(joined.contains('▶'), "the selected stage is marked");
        assert!(joined.contains('↳'), "the purpose is marked");
        assert!(joined.contains("\u{1b}[38;5;252m"), "the purpose is bright");
    }

    #[test]
    fn card_frames_content_and_colors_roles_in_the_detailed_view() {
        let mut state = state();
        state.details = true;
        let lines = render_card_lines(&state, InlineLayout::for_dimensions(100, 40), true);
        let joined = lines.join("\n");

        assert!(joined.contains('┌') && joined.contains('┘'));
        assert!(joined.contains("watn"));
        assert!(joined.contains("Intent"));
        assert!(joined.contains("Command"));
        assert!(joined.contains("ccept"));
        for absent in ["Flow", "Stage", "supported", "unsupported"] {
            assert!(
                !joined.contains(absent),
                "the detailed view must not show {absent:?}, got:\n{joined}"
            );
        }
        assert!(joined.contains("\u{1b}[1;38;5;81mesc\u{1b}[0m"));
        assert!(joined.contains("\u{1b}[1;38;5;81me\u{1b}[0m\u{1b}[2mdit\u{1b}[0m"));
        assert!(joined.contains("\u{1b}[38;5;81m"), "labels are colored");
        assert!(joined.contains("\u{1b}[38;5;221m"), "flags are colored");
        assert!(
            joined.contains("\u{1b}[38;5;240m"),
            "separators are colored"
        );
        assert!(joined.contains("◆ tier 1 · loopback/review-model"));
    }

    #[test]
    fn wrapping_helpers_stay_bounded() {
        assert_eq!(super::ellipsize("abc", 0), "");
        assert_eq!(super::ellipsize("abcdef", 3).chars().count(), 3);
        assert_eq!(
            super::wrap_stage("alpha beta", 20),
            vec!["alpha beta".to_string()]
        );
        assert_eq!(
            super::wrap_stage("alpha beta gamma", 11),
            vec!["alpha beta".to_string(), "gamma".to_string()]
        );
        assert_eq!(
            super::wrap_stage("abcdef", 2),
            vec!["ab".to_string(), "cd".to_string(), "ef".to_string()]
        );
        assert_eq!(super::wrap_stage("", 4), vec![String::new()]);
    }

    #[test]
    fn a_long_stage_wraps_and_keeps_its_separator() {
        let candidate = ReviewCandidate::from_command(
            "git cat-file --batch-check='%(objecttype) %(objectname) %(objectsize) %(rest)' | head -5",
        );
        let state = ReviewPanelState::new(
            ReviewContext {
                intent: "inspect recent commits".to_string(),
                tier: "1".to_string(),
                provider: "loopback".to_string(),
                model: "review-model".to_string(),
            },
            candidate,
        );
        let lines = render_card_lines(&state, InlineLayout::for_dimensions(60, 24), true);
        let joined = lines.join("\n");
        assert!(joined.contains("%(objecttype)"), "the stage text wraps");
        assert!(joined.contains("%(rest)"));
        assert!(joined.contains("\u{1b}[38;5;214m|\u{1b}[0m"));
    }

    #[test]
    fn chooser_rows_render_highlight_and_error() {
        let mut chooser = state();
        chooser.open_model_chooser(
            vec![crate::review::TierChoice {
                tier: "2".to_string(),
                label: "normal".to_string(),
                model: "model-b".to_string(),
            }],
            vec!["model-a".to_string(), "model-b".to_string()],
        );
        chooser.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
        let lines = render_card_lines(&chooser, InlineLayout::for_dimensions(80, 24), true);
        let joined = lines.join("\n");
        assert!(joined.contains("Picks"));
        assert!(joined.contains("Search"));
        assert!(joined.contains("\u{1b}[7m"), "the highlight is reversed");
        assert!(joined.contains("\u{1b}[1;38;5;81m1/2/3\u{1b}[0m\u{1b}[2m switch tier\u{1b}[0m"));

        chooser.apply_regeneration_failure("provider exploded");
        chooser.details = true;
        let lines = render_card_lines(&chooser, InlineLayout::for_dimensions(80, 24), true);
        let joined = lines.join("\n");
        assert!(joined.contains("Error"));
        assert!(joined.contains("provider exploded"));
    }

    #[test]
    fn card_stays_bounded_without_unsupported_markers() {
        let mut unsupported = state();
        unsupported.candidate = ReviewCandidate::from_command("printf one; cat < input");
        unsupported.flow_stage = 1;
        let layout = InlineLayout::for_dimensions(40, 8);
        let lines = render_card_lines(&unsupported, layout, true);

        assert!(lines.len() <= 7, "card rows: {}", lines.len());
        assert!(!lines.iter().any(|line| line.contains("\u{1b}[38;5;214m…")));
        assert!(!lines.iter().any(|line| line.contains('\n')));
    }

    #[test]
    fn a_narrow_stack_windows_around_the_selected_stage() {
        let mut state = state();
        state.candidate = ReviewCandidate::from_command("a | b | c | d | e | f | g | h");
        state.flow_stage = 0;
        let lines = render_card_lines(&state, InlineLayout::for_dimensions(40, 8), true);
        let joined = lines.join("\n");
        assert!(joined.contains('⋮'), "hidden stages are marked: {joined}");
        assert!(joined.contains("  ▶ a") || joined.contains("▶"));
        assert!(lines.len() <= 7);
    }

    #[test]
    fn unsupported_stages_render_without_a_marker() {
        let mut unsupported = state();
        unsupported.candidate =
            ReviewCandidate::from_command("while read commit; do git ls-tree -r $commit; done");
        unsupported.flow_stage = 0;
        let lines = render_card_lines(&unsupported, InlineLayout::for_dimensions(40, 24), true);
        assert!(
            !lines.iter().any(|line| line.contains("\u{1b}[38;5;214m…")),
            "undecomposed stages must not be marked, got:\n{}",
            lines.join("\n")
        );
    }

    #[test]
    fn plain_card_has_no_escape_sequences() {
        let layout = InlineLayout::for_dimensions(100, 40);
        let lines = render_card_lines(&state(), layout, false);
        assert!(lines.iter().all(|line| !line.contains('\u{1b}')));

        let mut unsupported = state();
        unsupported.candidate = ReviewCandidate::from_command("printf one; cat < input");
        unsupported.flow_stage = 1;
        let lines = render_card_lines(&unsupported, layout, false);
        assert!(lines.iter().all(|line| !line.contains('\u{1b}')));
        assert!(!lines.iter().any(|line| line.contains('…')));
    }

    #[test]
    fn visible_width_uses_terminal_columns_for_wide_characters() {
        assert_eq!(super::visible_len("abc"), 3);
        assert_eq!(super::visible_len("漢"), 2);
        assert_eq!(super::visible_len("a漢b"), 4);
        assert_eq!(super::visible_len("\u{1b}[97m漢\u{1b}[0m"), 2);
        assert_eq!(super::pad_to("漢", 4), "漢  ");
    }

    #[test]
    fn disable_hint_names_the_disabled_surface_with_and_without_color() {
        assert_eq!(
            super::disable_hint(false),
            "⚠ review panel disabled — re-enable with: watn --review-panel"
        );
        assert_eq!(
            super::disable_hint(true),
            "\u{1b}[1;38;5;214m⚠ review panel disabled\u{1b}[0m — re-enable with: \u{1b}[1;97mwatn --review-panel\u{1b}[0m"
        );
    }
}
