use std::io::{self, IsTerminal, Write};

use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use crossterm::terminal;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PromptResult {
    Execute,
    Explain,
    Cancelled,
    Interrupted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Confirmation {
    Execute,
    Explain,
    Cancelled,
    Interrupted,
}

/// Ask for execution confirmation. When `allow_explain` is set the prompt also
/// offers `?`, which returns [`PromptResult::Explain`] without executing.
pub fn prompt_for_execution(command: &str, allow_explain: bool) -> PromptResult {
    let _ = command;
    if allow_explain {
        eprint!("Execute now? [Y/n/?] ");
    } else {
        eprint!("Execute now? [Y/n] ");
    }
    std::io::stderr().flush().ok();

    let confirmation = if io::stdin().is_terminal() {
        read_terminal_confirmation(allow_explain)
    } else {
        read_line_confirmation(allow_explain)
    };

    match confirmation {
        Confirmation::Execute => PromptResult::Execute,
        Confirmation::Explain => PromptResult::Explain,
        Confirmation::Cancelled => PromptResult::Cancelled,
        Confirmation::Interrupted => PromptResult::Interrupted,
    }
}

/// Executes an already-authorized command and exits with its status. Review
/// acceptance is the sole authorization for review-eligible `-x` requests.
pub fn execute(command: &str) -> ! {
    let status = std::process::Command::new("sh")
        .arg("-c")
        .arg(command)
        .status()
        .expect("failed to execute command");
    std::process::exit(status.code().unwrap_or(0));
}

/// Pure confirmation classification: the answer string plus whether the
/// explanation option is offered becomes the prompt outcome.
pub fn classify_confirmation(input: &str, allow_explain: bool) -> PromptResult {
    match confirmation_from_input(input, allow_explain) {
        Confirmation::Execute => PromptResult::Execute,
        Confirmation::Explain => PromptResult::Explain,
        Confirmation::Cancelled => PromptResult::Cancelled,
        Confirmation::Interrupted => PromptResult::Interrupted,
    }
}

fn read_line_confirmation(allow_explain: bool) -> Confirmation {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => confirmation_from_input(&input, allow_explain),
        Err(error) if error.kind() == io::ErrorKind::Interrupted => Confirmation::Interrupted,
        Err(_) => Confirmation::Cancelled,
    }
}

fn read_terminal_confirmation(allow_explain: bool) -> Confirmation {
    if terminal::enable_raw_mode().is_err() {
        return read_line_confirmation(allow_explain);
    }

    let _raw_mode = RawModeGuard;
    let mut input = String::new();

    loop {
        let event = match event::read() {
            Ok(event) => event,
            Err(error) if error.kind() == io::ErrorKind::Interrupted => {
                return Confirmation::Interrupted;
            }
            Err(_) => return Confirmation::Cancelled,
        };

        let Event::Key(key) = event else {
            continue;
        };
        if key.kind != KeyEventKind::Press {
            continue;
        }

        if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
            let _ = writeln!(io::stderr());
            return Confirmation::Interrupted;
        }

        match key.code {
            KeyCode::Esc => {
                let _ = writeln!(io::stderr());
                return Confirmation::Cancelled;
            }
            KeyCode::Enter => {
                let _ = writeln!(io::stderr());
                return confirmation_from_input(&input, allow_explain);
            }
            KeyCode::Backspace => {
                if input.pop().is_some() {
                    eprint!("\u{8} \u{8}");
                    io::stderr().flush().ok();
                }
            }
            KeyCode::Char(c)
                if !key
                    .modifiers
                    .intersects(KeyModifiers::CONTROL | KeyModifiers::ALT) =>
            {
                input.push(c);
                eprint!("{}", c);
                io::stderr().flush().ok();
            }
            _ => {}
        }
    }
}

fn confirmation_from_input(input: &str, allow_explain: bool) -> Confirmation {
    if input.contains('\u{3}') {
        return Confirmation::Interrupted;
    }

    let input = input.trim().to_lowercase();
    if input.is_empty() || input == "y" || input == "yes" {
        Confirmation::Execute
    } else if allow_explain && input == "?" {
        Confirmation::Explain
    } else {
        Confirmation::Cancelled
    }
}

struct RawModeGuard;

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        terminal::disable_raw_mode().ok();
    }
}

#[cfg(test)]
mod tests {
    use super::{confirmation_from_input, Confirmation};

    #[test]
    fn confirmation_accepts_empty_and_yes_answers() {
        assert_eq!(confirmation_from_input("\n", false), Confirmation::Execute);
        assert_eq!(confirmation_from_input("Y\n", false), Confirmation::Execute);
        assert_eq!(
            confirmation_from_input("yes\n", false),
            Confirmation::Execute
        );
    }

    #[test]
    fn confirmation_rejects_non_yes_answers() {
        assert_eq!(
            confirmation_from_input("n\n", false),
            Confirmation::Cancelled
        );
        assert_eq!(
            confirmation_from_input("no\n", false),
            Confirmation::Cancelled
        );
        assert_eq!(
            confirmation_from_input("\u{1b}", false),
            Confirmation::Cancelled
        );
    }

    #[test]
    fn explanation_answer_is_only_available_when_allowed() {
        assert_eq!(confirmation_from_input("?\n", true), Confirmation::Explain);
        assert_eq!(
            confirmation_from_input("?\n", false),
            Confirmation::Cancelled
        );
    }

    #[test]
    fn confirmation_recognizes_ctrl_c() {
        assert_eq!(
            confirmation_from_input("\u{3}", false),
            Confirmation::Interrupted
        );
    }
}
