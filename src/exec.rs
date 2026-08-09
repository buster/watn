use std::io::{self, IsTerminal, Write};

use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use crossterm::terminal;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PromptResult {
    Cancelled,
    Interrupted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Confirmation {
    Execute,
    Cancelled,
    Interrupted,
}

pub fn prompt_and_execute(command: &str) -> PromptResult {
    eprint!("Execute now? [Y/n] ");
    std::io::stderr().flush().ok();

    let confirmation = if io::stdin().is_terminal() {
        read_terminal_confirmation()
    } else {
        read_line_confirmation()
    };

    match confirmation {
        Confirmation::Execute => {
            let status = std::process::Command::new("sh")
                .arg("-c")
                .arg(command)
                .status()
                .expect("failed to execute command");
            std::process::exit(status.code().unwrap_or(0));
        }
        Confirmation::Cancelled => PromptResult::Cancelled,
        Confirmation::Interrupted => PromptResult::Interrupted,
    }
}

fn read_line_confirmation() -> Confirmation {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => confirmation_from_input(&input),
        Err(error) if error.kind() == io::ErrorKind::Interrupted => Confirmation::Interrupted,
        Err(_) => Confirmation::Cancelled,
    }
}

fn read_terminal_confirmation() -> Confirmation {
    if terminal::enable_raw_mode().is_err() {
        return read_line_confirmation();
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
                return confirmation_from_input(&input);
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

fn confirmation_from_input(input: &str) -> Confirmation {
    if input.contains('\u{3}') {
        return Confirmation::Interrupted;
    }

    let input = input.trim().to_lowercase();
    if input.is_empty() || input == "y" || input == "yes" {
        Confirmation::Execute
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
        assert_eq!(confirmation_from_input("\n"), Confirmation::Execute);
        assert_eq!(confirmation_from_input("Y\n"), Confirmation::Execute);
        assert_eq!(confirmation_from_input("yes\n"), Confirmation::Execute);
    }

    #[test]
    fn confirmation_rejects_non_yes_answers() {
        assert_eq!(confirmation_from_input("n\n"), Confirmation::Cancelled);
        assert_eq!(confirmation_from_input("no\n"), Confirmation::Cancelled);
        assert_eq!(confirmation_from_input("\u{1b}"), Confirmation::Cancelled);
    }

    #[test]
    fn confirmation_recognizes_ctrl_c() {
        assert_eq!(confirmation_from_input("\u{3}"), Confirmation::Interrupted);
    }
}
