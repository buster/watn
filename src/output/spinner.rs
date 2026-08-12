use std::io::{self, IsTerminal, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use crossterm::{
    cursor, execute,
    style::{self, Color},
    terminal::{self, ClearType},
};

const FRAME_INTERVAL: Duration = Duration::from_millis(100);
const THINKING_FRAMES: [&str; 6] = [
    "(._.)",
    "(._. )",
    "( ._.)",
    "( ._. )",
    "( ._.)",
    "(._. )",
];
const PULSE_COLORS: [Color; 13] = [
    Color::Rgb {
        r: 24,
        g: 92,
        b: 60,
    },
    Color::Rgb {
        r: 44,
        g: 116,
        b: 74,
    },
    Color::Rgb {
        r: 64,
        g: 141,
        b: 88,
    },
    Color::Rgb {
        r: 84,
        g: 165,
        b: 102,
    },
    Color::Rgb {
        r: 104,
        g: 190,
        b: 116,
    },
    Color::Rgb {
        r: 124,
        g: 214,
        b: 130,
    },
    Color::Rgb {
        r: 144,
        g: 238,
        b: 144,
    },
    Color::Rgb {
        r: 124,
        g: 214,
        b: 130,
    },
    Color::Rgb {
        r: 104,
        g: 190,
        b: 116,
    },
    Color::Rgb {
        r: 84,
        g: 165,
        b: 102,
    },
    Color::Rgb {
        r: 64,
        g: 141,
        b: 88,
    },
    Color::Rgb {
        r: 44,
        g: 116,
        b: 74,
    },
    Color::Rgb {
        r: 24,
        g: 92,
        b: 60,
    },
];

pub struct Spinner {
    stop: Arc<AtomicBool>,
    worker: Option<JoinHandle<()>>,
}

impl Spinner {
    pub fn start(model: &str) -> Self {
        let stop = Arc::new(AtomicBool::new(false));
        if !terminal_is_usable() {
            return Self { stop, worker: None };
        }

        let worker_stop = Arc::clone(&stop);
        let model = model.to_string();
        let color_enabled = std::env::var_os("NO_COLOR").is_none();
        let worker = thread::spawn(move || {
            let started = Instant::now();
            let mut frame = 0;

            while !worker_stop.load(Ordering::Relaxed) {
                draw_frame(&model, started, frame, color_enabled);
                frame = (frame + 1) % PULSE_COLORS.len();
                thread::sleep(FRAME_INTERVAL);
            }

            clear_line();
        });

        Self {
            stop,
            worker: Some(worker),
        }
    }

    pub fn finish(mut self) {
        self.stop_and_join();
    }

    fn stop_and_join(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
    }
}

impl Drop for Spinner {
    fn drop(&mut self) {
        self.stop_and_join();
    }
}

fn terminal_is_usable() -> bool {
    io::stderr().is_terminal() && std::env::var("TERM").as_deref() != Ok("dumb")
}

fn draw_frame(model: &str, started: Instant, frame: usize, color_enabled: bool) {
    let mut stderr = io::stderr();
    let elapsed = started.elapsed().as_secs_f64();
    let status = format!(
        "  Asking {} · {:.1}s  {}",
        model,
        elapsed,
        THINKING_FRAMES[frame % THINKING_FRAMES.len()]
    );

    if color_enabled {
        let _ = execute!(
            stderr,
            cursor::MoveToColumn(0),
            terminal::Clear(ClearType::CurrentLine),
            style::SetForegroundColor(PULSE_COLORS[frame]),
            style::Print("◈"),
            style::ResetColor,
            style::Print(status),
        );
    } else {
        let _ = execute!(
            stderr,
            cursor::MoveToColumn(0),
            terminal::Clear(ClearType::CurrentLine),
            style::Print("◈"),
            style::Print(status),
        );
    }
    let _ = stderr.flush();
}

fn clear_line() {
    let mut stderr = io::stderr();
    let _ = execute!(
        stderr,
        cursor::MoveToColumn(0),
        terminal::Clear(ClearType::CurrentLine),
    );
    let _ = stderr.flush();
}

#[cfg(test)]
mod tests {
    use super::PULSE_COLORS;

    #[test]
    fn pulse_colors_fade_up_and_back_down() {
        let midpoint = PULSE_COLORS.len() / 2;
        for index in 0..midpoint {
            assert_eq!(
                PULSE_COLORS[index],
                PULSE_COLORS[PULSE_COLORS.len() - index - 1]
            );
        }
        assert_ne!(PULSE_COLORS[0], PULSE_COLORS[midpoint]);
    }
}
