//! Text measurement, wrapping, and ANSI styling for the review card.

pub(super) struct Ink {
    color: bool,
}

impl Ink {
    pub(super) fn new(color: bool) -> Self {
        Self { color }
    }

    pub(super) fn paint(&self, code: &str, text: &str) -> String {
        if self.color {
            format!("\u{1b}[{code}m{text}\u{1b}[0m")
        } else {
            text.to_string()
        }
    }

    pub(super) fn border(&self, text: &str) -> String {
        self.paint("38;5;240", text)
    }

    pub(super) fn label(&self, text: &str) -> String {
        self.paint("38;5;81", text)
    }

    pub(super) fn white(&self, text: &str) -> String {
        self.paint("97", text)
    }

    pub(super) fn dim(&self, text: &str) -> String {
        self.paint("2", text)
    }

    pub(super) fn key(&self, text: &str) -> String {
        self.paint("1;38;5;81", text)
    }

    pub(super) fn yellow(&self, text: &str) -> String {
        self.paint("38;5;221", text)
    }

    pub(super) fn green(&self, text: &str) -> String {
        self.paint("38;5;114", text)
    }

    pub(super) fn amber(&self, text: &str) -> String {
        self.paint("38;5;214", text)
    }

    pub(super) fn purpose_marker(&self, text: &str) -> String {
        self.paint("38;5;81", text)
    }

    pub(super) fn purpose(&self, text: &str) -> String {
        self.paint("38;5;252", text)
    }
}

pub(super) fn visible_len(value: &str) -> usize {
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

pub(super) fn pad_to(value: &str, width: usize) -> String {
    let mut padded = value.to_string();
    let visible = visible_len(value);
    if visible < width {
        padded.push_str(&" ".repeat(width - visible));
    }
    padded
}

pub(super) fn ellipsize(value: &str, width: usize) -> String {
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
pub(super) fn ellipsize_visible(value: &str, width: usize) -> String {
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

pub(super) fn char_offset(value: &str, count: usize) -> usize {
    value
        .char_indices()
        .nth(count)
        .map(|(index, _)| index)
        .unwrap_or(value.len())
}

/// Word-aware wrapping for one stage. A single word wider than the row is
/// hard-split at a character boundary so the row width stays exact.
pub(super) fn wrap_stage(text: &str, width: usize) -> Vec<String> {
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

pub(super) fn colorize_command(command: &str, ink: &Ink) -> String {
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

pub(super) fn model_short_name(model: &str) -> String {
    let last = model.rsplit('/').next().unwrap_or(model);
    let last = last.strip_prefix('~').unwrap_or(last);
    let short = last.split(':').next().unwrap_or(last);
    if short.is_empty() {
        model.to_string()
    } else {
        short.to_string()
    }
}
