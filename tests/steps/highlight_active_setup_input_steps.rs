use std::time::{Duration, Instant};

use cucumber::{then, when};

use super::{pty_snapshot, pty_write};
use crate::WatnWorld;

const SCREEN_WIDTH: usize = 120;
const SCREEN_HEIGHT: usize = 40;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Foreground {
    Default,
    Green,
    Other,
}

#[derive(Clone, Copy)]
struct Cell {
    character: char,
    foreground: Foreground,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            character: ' ',
            foreground: Foreground::Default,
        }
    }
}

struct Screen {
    cells: [Cell; SCREEN_WIDTH * SCREEN_HEIGHT],
    cursor_x: usize,
    cursor_y: usize,
    foreground: Foreground,
    saved_cursor: Option<(usize, usize)>,
}

impl Screen {
    fn new() -> Self {
        Self {
            cells: [Cell::default(); SCREEN_WIDTH * SCREEN_HEIGHT],
            cursor_x: 0,
            cursor_y: 0,
            foreground: Foreground::Default,
            saved_cursor: None,
        }
    }

    fn cell(&self, x: usize, y: usize) -> Cell {
        self.cells[y * SCREEN_WIDTH + x]
    }

    fn put(&mut self, character: char) {
        if self.cursor_x < SCREEN_WIDTH && self.cursor_y < SCREEN_HEIGHT {
            let index = self.cursor_y * SCREEN_WIDTH + self.cursor_x;
            self.cells[index] = Cell {
                character,
                foreground: self.foreground,
            };
        }
        self.cursor_x = (self.cursor_x + 1).min(SCREEN_WIDTH.saturating_sub(1));
    }

    fn clear_all(&mut self) {
        self.cells = [Cell::default(); SCREEN_WIDTH * SCREEN_HEIGHT];
    }

    fn clear_line(&mut self, mode: usize) {
        let (start, end) = match mode {
            1 => (0, self.cursor_x.min(SCREEN_WIDTH.saturating_sub(1))),
            2 => (0, SCREEN_WIDTH),
            _ => (self.cursor_x.min(SCREEN_WIDTH), SCREEN_WIDTH),
        };
        for x in start..end {
            self.cells[self.cursor_y * SCREEN_WIDTH + x] = Cell::default();
        }
    }

    fn clear_screen(&mut self, mode: usize) {
        match mode {
            1 => {
                for y in 0..=self.cursor_y.min(SCREEN_HEIGHT.saturating_sub(1)) {
                    let end = if y == self.cursor_y {
                        self.cursor_x.min(SCREEN_WIDTH.saturating_sub(1)) + 1
                    } else {
                        SCREEN_WIDTH
                    };
                    for x in 0..end {
                        self.cells[y * SCREEN_WIDTH + x] = Cell::default();
                    }
                }
            }
            2 | 3 => self.clear_all(),
            _ => {
                for y in self.cursor_y.min(SCREEN_HEIGHT.saturating_sub(1))..SCREEN_HEIGHT {
                    let start = if y == self.cursor_y {
                        self.cursor_x.min(SCREEN_WIDTH)
                    } else {
                        0
                    };
                    for x in start..SCREEN_WIDTH {
                        self.cells[y * SCREEN_WIDTH + x] = Cell::default();
                    }
                }
            }
        }
    }

    fn apply_sgr(&mut self, params: &str) {
        let values = if params.is_empty() {
            vec![0]
        } else {
            params
                .split(';')
                .map(|value| value.parse::<usize>().unwrap_or(0))
                .collect::<Vec<_>>()
        };
        let mut index = 0;
        while index < values.len() {
            match values[index] {
                0 => self.foreground = Foreground::Default,
                30..=37 => {
                    self.foreground = if values[index] == 32 {
                        Foreground::Green
                    } else {
                        Foreground::Other
                    };
                }
                90..=97 => {
                    self.foreground = if values[index] == 92 {
                        Foreground::Green
                    } else {
                        Foreground::Other
                    };
                }
                38 if values.get(index + 1) == Some(&5) => {
                    self.foreground = if values.get(index + 2) == Some(&2) {
                        Foreground::Green
                    } else {
                        Foreground::Other
                    };
                    index += 2;
                }
                39 => self.foreground = Foreground::Default,
                _ => {}
            }
            index += 1;
        }
    }

    fn apply_csi(&mut self, params: &str, command: char) {
        let values = params
            .split(';')
            .map(|value| value.trim_start_matches('?').parse::<usize>().unwrap_or(0))
            .collect::<Vec<_>>();
        let first = values
            .first()
            .copied()
            .filter(|value| *value != 0)
            .unwrap_or(1);
        match command {
            'A' => self.cursor_y = self.cursor_y.saturating_sub(first),
            'B' => self.cursor_y = (self.cursor_y + first).min(SCREEN_HEIGHT.saturating_sub(1)),
            'C' => self.cursor_x = (self.cursor_x + first).min(SCREEN_WIDTH.saturating_sub(1)),
            'D' => self.cursor_x = self.cursor_x.saturating_sub(first),
            'G' => self.cursor_x = first.saturating_sub(1).min(SCREEN_WIDTH.saturating_sub(1)),
            'd' => self.cursor_y = first.saturating_sub(1).min(SCREEN_HEIGHT.saturating_sub(1)),
            'H' | 'f' => {
                self.cursor_y = values
                    .first()
                    .copied()
                    .unwrap_or(1)
                    .saturating_sub(1)
                    .min(SCREEN_HEIGHT.saturating_sub(1));
                self.cursor_x = values
                    .get(1)
                    .copied()
                    .unwrap_or(1)
                    .saturating_sub(1)
                    .min(SCREEN_WIDTH.saturating_sub(1));
            }
            'J' => self.clear_screen(values.first().copied().unwrap_or(0)),
            'K' => self.clear_line(values.first().copied().unwrap_or(0)),
            'X' => {
                for offset in 0..first {
                    let x = self.cursor_x + offset;
                    if x < SCREEN_WIDTH {
                        self.cells[self.cursor_y * SCREEN_WIDTH + x] = Cell::default();
                    }
                }
            }
            'm' => self.apply_sgr(params),
            _ => {}
        }
    }

    fn border_signature(&self, title: &str) -> Option<Vec<Foreground>> {
        let title: Vec<char> = title.chars().collect();
        for y in 0..SCREEN_HEIGHT {
            for start in 0..=SCREEN_WIDTH.saturating_sub(title.len()) {
                if title
                    .iter()
                    .enumerate()
                    .all(|(offset, character)| self.cell(start + offset, y).character == *character)
                {
                    let Some(left) = (0..start).rev().find(|x| self.cell(*x, y).character == '┌')
                    else {
                        continue;
                    };
                    let Some(right) = (start + title.len()..SCREEN_WIDTH)
                        .find(|x| self.cell(*x, y).character == '┐')
                    else {
                        continue;
                    };
                    let styles = (left..=right)
                        .filter_map(|x| {
                            let cell = self.cell(x, y);
                            is_border_glyph(cell.character).then_some(cell.foreground)
                        })
                        .collect::<Vec<_>>();
                    if styles.len() >= 3 {
                        return Some(styles);
                    }
                }
            }
        }
        None
    }
}

fn is_border_glyph(character: char) -> bool {
    matches!(
        character,
        '┌' | '─' | '┐' | '│' | '└' | '┘' | '├' | '┤' | '┬' | '┴' | '┼'
    )
}

fn parse_screen(output: &str) -> Screen {
    let mut screen = Screen::new();
    let mut characters = output.chars().peekable();
    while let Some(character) = characters.next() {
        if character == '\x1b' {
            match characters.next() {
                Some('[') => {
                    let mut sequence = String::new();
                    while let Some(next) = characters.next() {
                        sequence.push(next);
                        if ('@'..='~').contains(&next) {
                            let command = sequence.pop().expect("CSI command");
                            screen.apply_csi(&sequence, command);
                            break;
                        }
                    }
                }
                Some(']') => {
                    while let Some(next) = characters.next() {
                        if next == '\x07' {
                            break;
                        }
                        if next == '\x1b' && characters.peek() == Some(&'\\') {
                            characters.next();
                            break;
                        }
                    }
                }
                Some('7') => screen.saved_cursor = Some((screen.cursor_x, screen.cursor_y)),
                Some('8') => {
                    if let Some((x, y)) = screen.saved_cursor {
                        screen.cursor_x = x;
                        screen.cursor_y = y;
                    }
                }
                _ => {}
            }
        } else {
            match character {
                '\r' => screen.cursor_x = 0,
                '\n' => {
                    screen.cursor_y = (screen.cursor_y + 1).min(SCREEN_HEIGHT.saturating_sub(1))
                }
                '\x08' => screen.cursor_x = screen.cursor_x.saturating_sub(1),
                '\t' => screen.cursor_x = ((screen.cursor_x / 8) + 1) * 8,
                character if !character.is_control() => screen.put(character),
                _ => {}
            }
        }
    }
    screen
}

fn wait_for_border(world: &mut WatnWorld, title: &str) -> Vec<Foreground> {
    let session = world.pty_session.as_ref().expect("setup PTY session");
    let deadline = Instant::now() + Duration::from_secs(3);
    loop {
        let output = pty_snapshot(session);
        let screen = parse_screen(&output);
        if let Some(signature) = screen.border_signature(title) {
            return signature;
        }
        if Instant::now() >= deadline {
            panic!("widget {title:?} was not reconstructed from PTY output: {output:?}");
        }
        std::thread::sleep(Duration::from_millis(25));
    }
}

fn assert_green(signature: &[Foreground], title: &str) {
    assert!(
        !signature.is_empty(),
        "empty border signature for {title:?}"
    );
    assert!(
        signature
            .iter()
            .all(|foreground| *foreground == Foreground::Green),
        "border for {title:?} was not green: {signature:?}"
    );
}

fn assert_default(signature: &[Foreground], title: &str) {
    assert!(
        !signature.is_empty(),
        "empty border signature for {title:?}"
    );
    assert!(
        signature
            .iter()
            .all(|foreground| *foreground == Foreground::Default),
        "border for {title:?} changed from its default styling: {signature:?}"
    );
}

fn signature_key(key: &str) -> String {
    format!("highlight-active-setup-input:{key}")
}

fn remember_signature(world: &mut WatnWorld, key: &str, signature: &[Foreground]) {
    let encoded = signature
        .iter()
        .map(|foreground| match foreground {
            Foreground::Default => 'd',
            Foreground::Green => 'g',
            Foreground::Other => 'o',
        })
        .collect::<String>();
    let encoded = if encoded
        .chars()
        .all(|character| character == encoded.chars().next().unwrap())
    {
        encoded.chars().next().unwrap().to_string()
    } else {
        encoded
    };
    world.pending_config.insert(signature_key(key), encoded);
}

fn assert_matches_signature(world: &WatnWorld, key: &str, signature: &[Foreground], title: &str) {
    let expected = world
        .pending_config
        .get(&signature_key(key))
        .unwrap_or_else(|| panic!("missing border baseline for {key:?}"));
    let actual = signature
        .iter()
        .map(|foreground| match foreground {
            Foreground::Default => 'd',
            Foreground::Green => 'g',
            Foreground::Other => 'o',
        })
        .collect::<String>();
    if expected.len() == 1 {
        assert!(
            actual
                .chars()
                .all(|character| Some(character) == expected.chars().next()),
            "border for {title:?} did not retain its inactive baseline: {actual:?} vs {expected:?}"
        );
    } else {
        assert_eq!(
            actual, *expected,
            "border for {title:?} did not retain its inactive baseline"
        );
    }
}

#[then("the setup wizard should show the active URL input with a green border")]
fn active_url_border(world: &mut WatnWorld) {
    let signature = wait_for_border(world, "URL (editing)");
    assert_green(&signature, "URL (editing)");
}

#[then("the setup wizard should show the active credential location with a green border")]
fn active_credential_border(world: &mut WatnWorld) {
    let storage = wait_for_border(world, "Where should the API key be stored?");
    assert_green(&storage, "Where should the API key be stored?");
    let value = wait_for_border(world, "API key / environment name (editing)");
    remember_signature(world, "credential-value", &value);
}

#[then("the setup wizard should show the API key input with a green border")]
fn active_api_key_border(world: &mut WatnWorld) {
    let signature = wait_for_border(world, "API key / environment name (editing)");
    assert_green(&signature, "API key / environment name (editing)");
}

#[then("the inactive API key input should retain its default border styling")]
fn inactive_api_key_border(world: &mut WatnWorld) {
    let signature = wait_for_border(world, "API key / environment name (editing)");
    assert_default(&signature, "API key / environment name (editing)");
    assert_matches_signature(
        world,
        "credential-value",
        &signature,
        "API key / environment name (editing)",
    );
}

#[then("the inactive credential location should retain its default border styling")]
fn inactive_credential_border(world: &mut WatnWorld) {
    let signature = wait_for_border(world, "Where should the API key be stored?");
    assert_matches_signature(
        world,
        "credential-value",
        &signature,
        "Where should the API key be stored?",
    );
}

#[then("the setup wizard should show the model input with a green border")]
fn active_model_border(world: &mut WatnWorld) {
    let model = wait_for_border(world, "Small Model (editing)");
    assert_green(&model, "Small Model (editing)");
    let reasoning = wait_for_border(world, "Model reasoning");
    remember_signature(world, "model-reasoning", &reasoning);
}

#[then("the inactive reasoning input should retain its default border styling")]
fn inactive_reasoning_border(world: &mut WatnWorld) {
    let signature = wait_for_border(world, "Model reasoning");
    assert_default(&signature, "Model reasoning");
    assert_matches_signature(world, "model-reasoning", &signature, "Model reasoning");
}

#[when("I toggle reasoning focus in the setup wizard")]
fn toggle_reasoning_focus(world: &mut WatnWorld) {
    let session = world.pty_session.as_mut().expect("setup PTY session");
    pty_write(session, "\x12");
    std::thread::sleep(Duration::from_millis(100));
}

#[then("the setup wizard should show the reasoning input with a green border")]
fn active_reasoning_border(world: &mut WatnWorld) {
    let signature = wait_for_border(world, "Model reasoning");
    assert_green(&signature, "Model reasoning");
}

#[then("the inactive model input should retain its default border styling")]
fn inactive_model_border(world: &mut WatnWorld) {
    let signature = wait_for_border(world, "Small Model (editing)");
    assert_matches_signature(
        world,
        "model-reasoning",
        &signature,
        "Small Model (editing)",
    );
}

#[when("I confirm the Large Model selection and configure the shortcut")]
fn confirm_large_model_and_configure_shortcut(world: &mut WatnWorld) {
    let session = world.pty_session.as_mut().expect("setup PTY session");
    pty_write(session, "\r");
    let _ = wait_for_border(world, "Shell shortcut");
}

#[then("the setup wizard should show the shortcut question with a green border")]
fn active_shortcut_question_border(world: &mut WatnWorld) {
    let question = wait_for_border(world, "Shell shortcut");
    assert_green(&question, "Shell shortcut");
    let shells = wait_for_border(world, "Select shells");
    remember_signature(world, "shortcut-shells", &shells);
}

#[when("I enable shortcut configuration")]
fn enable_shortcut_configuration(world: &mut WatnWorld) {
    let session = world.pty_session.as_mut().expect("setup PTY session");
    pty_write(session, "y");
    std::thread::sleep(Duration::from_millis(100));
}

#[then("the setup wizard should show shell selection with a green border")]
fn active_shell_selection_border(world: &mut WatnWorld) {
    let shells = wait_for_border(world, "Select shells");
    assert_green(&shells, "Select shells");
}

#[then("the inactive shell selection should retain its default border styling")]
fn inactive_shell_selection_border(world: &mut WatnWorld) {
    let shells = wait_for_border(world, "Select shells");
    assert_default(&shells, "Select shells");
    assert_matches_signature(world, "shortcut-shells", &shells, "Select shells");
}

#[then("the inactive shortcut question should retain its default border styling")]
fn inactive_shortcut_question_border(world: &mut WatnWorld) {
    let question = wait_for_border(world, "Shell shortcut");
    assert_matches_signature(world, "shortcut-shells", &question, "Shell shortcut");
}
