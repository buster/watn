use std::ops::Range;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StageSupport {
    Supported,
    Unsupported,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnsupportedSpan {
    pub range: Range<usize>,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandStage {
    pub stage_text: String,
    pub support: StageSupport,
    pub unsupported_spans: Vec<UnsupportedSpan>,
    pub separator: Option<String>,
}

impl CommandStage {
    pub fn is_supported(&self) -> bool {
        self.support == StageSupport::Supported
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CommandFlow {
    pub stages: Vec<CommandStage>,
}

impl CommandFlow {
    pub fn stage_texts(&self) -> impl Iterator<Item = &str> {
        self.stages.iter().map(|stage| stage.stage_text.as_str())
    }

    pub fn has_unsupported(&self) -> bool {
        self.stages
            .iter()
            .any(|stage| stage.support == StageSupport::Unsupported)
    }
}

/// Build a stage from a validated byte range of the command, with local
/// unsupported-syntax marking.
pub fn command_stage(command: &str, start: usize, end: usize) -> CommandStage {
    let unsupported_spans = unsupported_spans(command, start, end);
    CommandStage {
        stage_text: command[start..end].to_string(),
        support: if unsupported_spans.is_empty() {
            StageSupport::Supported
        } else {
            StageSupport::Unsupported
        },
        unsupported_spans,
        separator: following_separator(command, end),
    }
}

/// The operator that immediately follows a stage, if any. Only top-level
/// operators are found because accepted provider gaps contain nothing but
/// whitespace and separators.
fn following_separator(command: &str, end: usize) -> Option<String> {
    let rest = command.get(end..)?.trim_start();
    ["&&", "||", "|", ";"]
        .into_iter()
        .find(|operator| rest.starts_with(operator))
        .map(str::to_string)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Segment {
    start: usize,
    end: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Token {
    start: usize,
    end: usize,
}

/// Derive only the shell constructs whose boundaries can be identified without
/// evaluating shell grammar. The original command is never rewritten.
pub fn derive_command_flow(command: &str) -> CommandFlow {
    let segments = split_segments(command);
    let mut stages = Vec::new();

    for segment in segments {
        let Some((start, end)) = trimmed_range(command, segment.start, segment.end) else {
            continue;
        };

        let xargs_split = xargs_command_split(command, start, end);
        if let Some(split_at) = xargs_split {
            push_stage(&mut stages, command, start, split_at);
            push_stage(&mut stages, command, split_at, end);
        } else {
            push_stage(&mut stages, command, start, end);
        }
    }

    CommandFlow { stages }
}

fn push_stage(stages: &mut Vec<CommandStage>, command: &str, start: usize, end: usize) {
    let Some((start, end)) = trimmed_range(command, start, end) else {
        return;
    };
    stages.push(command_stage(command, start, end));
}

fn split_segments(command: &str) -> Vec<Segment> {
    let mut segments = Vec::new();
    let mut segment_start = 0;
    let mut quote = None;
    let mut escaped = false;
    let bytes = command.as_bytes();
    let mut index = 0;

    while index < bytes.len() {
        let byte = bytes[index];
        if escaped {
            escaped = false;
            index += 1;
            continue;
        }
        if byte == b'\\' && quote != Some(b'\'') {
            escaped = true;
            index += 1;
            continue;
        }
        if byte == b'\'' || byte == b'"' {
            if quote == Some(byte) {
                quote = None;
            } else if quote.is_none() {
                quote = Some(byte);
            }
            index += 1;
            continue;
        }
        if quote.is_some() {
            index += 1;
            continue;
        }

        let separator_length = match byte {
            b'|' if bytes.get(index + 1) == Some(&b'|') => 2,
            b'&' if bytes.get(index + 1) == Some(&b'&') => 2,
            b'|' | b';' | b'\n' => 1,
            _ => 0,
        };
        if separator_length != 0 {
            segments.push(Segment {
                start: segment_start,
                end: index,
            });
            index += separator_length;
            segment_start = index;
        } else {
            index += 1;
        }
    }

    segments.push(Segment {
        start: segment_start,
        end: bytes.len(),
    });
    segments
}

fn xargs_command_split(command: &str, start: usize, end: usize) -> Option<usize> {
    let tokens = shell_tokens(command, start, end);
    if tokens.len() < 3 || &command[tokens[0].start..tokens[0].end] != "xargs" {
        return None;
    }

    let mut option_terminator = false;
    for token in tokens.into_iter().skip(1) {
        let text = &command[token.start..token.end];
        if text == "--" {
            option_terminator = true;
            continue;
        }
        if !option_terminator && text.starts_with('-') {
            continue;
        }
        return Some(token.start);
    }
    None
}

fn shell_tokens(command: &str, start: usize, end: usize) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut token_start = None;
    let mut quote = None;
    let mut escaped = false;
    let bytes = command.as_bytes();
    let mut index = start;

    while index < end {
        let byte = bytes[index];
        if escaped {
            escaped = false;
            if token_start.is_none() {
                token_start = Some(index.saturating_sub(1));
            }
            index += 1;
            continue;
        }
        if byte == b'\\' && quote != Some(b'\'') {
            if token_start.is_none() {
                token_start = Some(index);
            }
            escaped = true;
            index += 1;
            continue;
        }
        if byte == b'\'' || byte == b'"' {
            if token_start.is_none() {
                token_start = Some(index);
            }
            if quote == Some(byte) {
                quote = None;
            } else if quote.is_none() {
                quote = Some(byte);
            }
            index += 1;
            continue;
        }
        if quote.is_none() && byte.is_ascii_whitespace() {
            if let Some(token_start) = token_start.take() {
                tokens.push(Token {
                    start: token_start,
                    end: index,
                });
            }
        } else if token_start.is_none() {
            token_start = Some(index);
        }
        index += 1;
    }
    if let Some(token_start) = token_start {
        tokens.push(Token {
            start: token_start,
            end,
        });
    }
    tokens
}

fn unsupported_spans(command: &str, start: usize, end: usize) -> Vec<UnsupportedSpan> {
    let bytes = command.as_bytes();
    let mut spans = Vec::new();
    let mut quote = None;
    let mut escaped = false;
    let mut index = start;

    while index < end {
        let byte = bytes[index];
        if escaped {
            escaped = false;
            index += 1;
            continue;
        }
        if byte == b'\\' && quote != Some(b'\'') {
            escaped = true;
            index += 1;
            continue;
        }
        if byte == b'\'' || byte == b'"' {
            if quote == Some(byte) {
                quote = None;
            } else if quote.is_none() {
                quote = Some(byte);
            }
            index += 1;
            continue;
        }
        if quote.is_some() {
            index += 1;
            continue;
        }

        let length = match byte {
            b'<' | b'>' | b'`' | b'(' | b')' | b'{' | b'}' => 1,
            b'&' if bytes.get(index + 1) != Some(&b'&') => 1,
            b'$' if bytes.get(index + 1) == Some(&b'(') => 2,
            _ => 0,
        };
        if length != 0 {
            spans.push(UnsupportedSpan {
                range: index..(index + length).min(end),
                text: command[index..(index + length).min(end)].to_string(),
            });
            index += length;
        } else {
            index += 1;
        }
    }
    for token in shell_tokens(command, start, end) {
        let text = &command[token.start..token.end];
        if matches!(
            text,
            "for"
                | "do"
                | "done"
                | "if"
                | "then"
                | "elif"
                | "else"
                | "fi"
                | "case"
                | "esac"
                | "while"
                | "until"
                | "function"
        ) && !spans
            .iter()
            .any(|span| span.range == (token.start..token.end))
        {
            spans.push(UnsupportedSpan {
                range: token.start..token.end,
                text: text.to_string(),
            });
        }
    }
    spans.sort_by_key(|span| span.range.start);
    spans
}

fn trimmed_range(command: &str, start: usize, end: usize) -> Option<(usize, usize)> {
    let value = command.get(start..end)?;
    let left = value.len() - value.trim_start().len();
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return None;
    }
    Some((start + left, start + left + trimmed.len()))
}

#[cfg(test)]
mod tests {
    use super::{derive_command_flow, StageSupport};

    #[test]
    fn preserves_complex_flow_stage_text_and_splits_xargs_command() {
        let command = "git log --format='%H' --since='7 days ago' | xargs -n1 git show --stat --oneline && printf 'done'";
        let flow = derive_command_flow(command);

        assert_eq!(
            flow.stage_texts().collect::<Vec<_>>(),
            vec![
                "git log --format='%H' --since='7 days ago'",
                "xargs -n1",
                "git show --stat --oneline",
                "printf 'done'",
            ]
        );
        assert!(flow.stages.iter().all(|stage| stage.is_supported()));
    }

    #[test]
    fn marks_unsupported_shell_syntax_without_rewriting_the_stage() {
        let command = "for file in *.log; do cat < \"$file\"; done";
        let flow = derive_command_flow(command);

        assert_eq!(flow.stages.len(), 3);
        assert_eq!(flow.stages[0].stage_text, "for file in *.log");
        assert_eq!(flow.stages[1].stage_text, "do cat < \"$file\"");
        assert_eq!(flow.stages[2].stage_text, "done");
        assert_eq!(flow.stages[1].support, StageSupport::Unsupported);
        assert!(flow.stages[1]
            .unsupported_spans
            .iter()
            .any(|span| span.text == "<"));
        assert!(flow.has_unsupported());
    }

    #[test]
    fn quotes_escapes_xargs_options_and_empty_segments_are_handled() {
        let quoted = derive_command_flow("printf 'a|b' && echo 'x&&y'");
        assert_eq!(
            quoted.stage_texts().collect::<Vec<_>>(),
            vec!["printf 'a|b'", "echo 'x&&y'"]
        );

        let escaped = derive_command_flow(r"echo a\ b | cat");
        assert_eq!(
            escaped.stage_texts().collect::<Vec<_>>(),
            vec![r"echo a\ b", "cat"]
        );

        let xargs = derive_command_flow("xargs -0 -- rm -f");
        assert_eq!(
            xargs.stage_texts().collect::<Vec<_>>(),
            vec!["xargs -0 --", "rm -f"]
        );

        let xargs_without_command = derive_command_flow("xargs -0");
        assert_eq!(
            xargs_without_command.stage_texts().collect::<Vec<_>>(),
            vec!["xargs -0"]
        );

        let trailing = derive_command_flow("ls ; ");
        assert_eq!(trailing.stages.len(), 1);
        assert_eq!(trailing.stages[0].stage_text, "ls");

        let single_ampersand = derive_command_flow("sleep 1 & echo done");
        assert!(single_ampersand.has_unsupported());

        let simple = derive_command_flow("true");
        assert!(simple.stages[0].is_supported());
        assert_eq!(simple.stage_texts().collect::<Vec<_>>(), vec!["true"]);
        assert!(!simple.has_unsupported());
    }

    #[test]
    fn xargs_without_command_escaped_tokens_and_subshells_are_covered() {
        let options_only = derive_command_flow("xargs -0 --");
        assert_eq!(
            options_only.stage_texts().collect::<Vec<_>>(),
            vec!["xargs -0 --"]
        );

        let escaped_start = derive_command_flow(r"\ echo | cat");
        assert_eq!(
            escaped_start.stage_texts().collect::<Vec<_>>(),
            vec![r"\ echo", "cat"]
        );

        let subshell = derive_command_flow("echo $(date)");
        assert!(subshell.has_unsupported());
        assert!(subshell.stages[0]
            .unsupported_spans
            .iter()
            .any(|span| span.text == "$("));
    }
}
