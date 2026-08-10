use cucumber::{given, then, when};
use regex::Regex;
use std::fmt;
use std::io::Read;
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use crate::WatnWorld;

use super::{finish_pty_session, pty_snapshot, pty_write, start_pty_session};

pub struct LiveInvocation {
    child: Child,
    stdout: Arc<Mutex<Vec<u8>>>,
    stderr: Arc<Mutex<Vec<u8>>>,
    readers: Vec<JoinHandle<()>>,
}

impl fmt::Debug for LiveInvocation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LiveInvocation")
            .field("child_id", &self.child.id())
            .field("readers", &self.readers.len())
            .finish()
    }
}

impl LiveInvocation {
    fn start(world: &mut WatnWorld, args: &[&str]) -> Self {
        let binary = super::find_binary();
        super::ensure_test_env(world);
        let mut command = Command::new(binary);
        command.args(args);
        super::apply_env(world, &mut command);
        command.stdin(Stdio::null());
        command.stdout(Stdio::piped());
        command.stderr(Stdio::piped());

        let mut child = command.spawn().expect("start live watn invocation");
        let stdout = Arc::new(Mutex::new(Vec::new()));
        let stderr = Arc::new(Mutex::new(Vec::new()));
        let stdout_reader = child.stdout.take().expect("live stdout pipe");
        let stderr_reader = child.stderr.take().expect("live stderr pipe");
        let readers = vec![
            spawn_reader(stdout_reader, Arc::clone(&stdout)),
            spawn_reader(stderr_reader, Arc::clone(&stderr)),
        ];

        Self {
            child,
            stdout,
            stderr,
            readers,
        }
    }

    fn stdout_snapshot(&self) -> String {
        String::from_utf8_lossy(&self.stdout.lock().expect("live stdout lock")).to_string()
    }

    fn stderr_snapshot(&self) -> String {
        String::from_utf8_lossy(&self.stderr.lock().expect("live stderr lock")).to_string()
    }

    fn finish(mut self) -> (i32, String, String) {
        let deadline = Instant::now() + Duration::from_secs(10);
        let status = loop {
            if let Some(status) = self.child.try_wait().expect("poll live watn child") {
                break status;
            }
            if Instant::now() >= deadline {
                let _ = self.child.kill();
                break self.child.wait().expect("wait for live watn child");
            }
            thread::sleep(Duration::from_millis(25));
        };
        for reader in self.readers.drain(..) {
            let _ = reader.join();
        }
        let stdout = self.stdout_snapshot();
        let stderr = self.stderr_snapshot();
        (status.code().unwrap_or(-1), stdout, stderr)
    }
}

impl Drop for LiveInvocation {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
        for reader in self.readers.drain(..) {
            let _ = reader.join();
        }
    }
}

fn spawn_reader<R>(mut reader: R, buffer: Arc<Mutex<Vec<u8>>>) -> JoinHandle<()>
where
    R: Read + Send + 'static,
{
    thread::spawn(move || {
        let mut bytes = [0_u8; 1024];
        loop {
            match reader.read(&mut bytes) {
                Ok(0) => break,
                Ok(count) => buffer
                    .lock()
                    .expect("live pipe lock")
                    .extend_from_slice(&bytes[..count]),
                Err(_) => break,
            }
        }
    })
}

#[given(
    regex = r##"^a streaming provider flushes content "([^"]+)" and delays content "([^"]+)" while keeping the connection open$"##
)]
fn delayed_provider(world: &mut WatnWorld, first: String, second: String) {
    super::incremental_sse_rendering_steps::configure_delayed_content(world, first, second);
}

#[when(regex = r##"^I start the delayed streaming command `watn "([^"]*)"` in a terminal$"##)]
fn start_delayed_stream(world: &mut WatnWorld, question: String) {
    let session = start_pty_session(world, &[&question]);
    world.pty_session = Some(session);
}

#[then("the progress indicator is visible before the first streamed content")]
fn progress_before_content(world: &mut WatnWorld) {
    wait_for_terminal_text(world, "Asking");
    let output = pty_snapshot(world.pty_session.as_ref().expect("streaming PTY session"));
    assert!(
        !output.contains("printf first"),
        "content arrived before progress observation"
    );
}

#[then(
    regex = r##"^the first streamed content "([^"]+)" is visible before the provider releases the delayed event$"##
)]
fn first_content_before_release(world: &mut WatnWorld, first: String) {
    wait_for_terminal_text(world, &first);
    let output = pty_snapshot(world.pty_session.as_ref().expect("streaming PTY session"));
    assert!(output.contains(&first));
    assert!(
        !output.contains("printf second"),
        "delayed content arrived before release"
    );
}

#[then("the terminal shows spinner cleanup after the first streamed content")]
fn spinner_cleanup_after_content(world: &mut WatnWorld) {
    let output = pty_snapshot(world.pty_session.as_ref().expect("streaming PTY session"));
    assert!(
        output.contains("\x1b[2K"),
        "expected terminal clear-line evidence, got {output:?}"
    );
}

#[when("I release the delayed event and wait for watn to exit")]
fn release_delayed_event(world: &mut WatnWorld) {
    super::incremental_sse_rendering_steps::release_stream(world);
    if let Some(session) = world.pty_session.take() {
        finish_pty_session(world, session);
    }
}

#[then(regex = r##"^the terminal generated command line "([^"]+)" appears exactly once$"##)]
fn terminal_generated_command_once(world: &mut WatnWorld, command: String) {
    let output = world
        .output
        .as_deref()
        .expect("terminal output was not captured");
    assert_eq!(
        output.match_indices(&command).count(),
        1,
        "expected one generated command in terminal output: {output:?}"
    );
}

#[given(
    regex = r##"^a streaming provider emits reasoning "([^"]+)" and content "([^"]+)" before holding a later completion event$"##
)]
fn verbose_provider(world: &mut WatnWorld, reasoning: String, content: String) {
    super::incremental_sse_rendering_steps::configure_verbose_content(world, reasoning, content);
}

#[when(
    regex = r##"^I start the verbose streaming command `watn -v "([^"]*)"` with captured stdout and stderr$"##
)]
fn start_verbose_stream(world: &mut WatnWorld, question: String) {
    world.live_stream = Some(LiveInvocation::start(world, &["-v", &question]));
}

#[then(
    regex = r##"^stdout has streamed fragment "([^"]+)" before the provider releases completion$"##
)]
fn live_stdout_fragment(world: &mut WatnWorld, fragment: String) {
    let deadline = Instant::now() + Duration::from_secs(3);
    loop {
        let invocation = world
            .live_stream
            .as_ref()
            .expect("live streaming invocation");
        let output = invocation.stdout_snapshot();
        if output.contains(&fragment) {
            return;
        }
        assert!(
            Instant::now() < deadline,
            "stdout did not contain {fragment:?} before release: {output:?}; stderr: {:?}",
            invocation.stderr_snapshot()
        );
        thread::sleep(Duration::from_millis(25));
    }
}

#[then(regex = r##"^stderr does not yet contain "([^"]+)"$"##)]
fn live_stderr_not_contains(world: &mut WatnWorld, text: String) {
    let stderr = world
        .live_stream
        .as_ref()
        .expect("live streaming invocation")
        .stderr_snapshot();
    assert!(
        !stderr.contains(&text),
        "unexpected early stderr output: {stderr:?}"
    );
}

#[when("I release completion and wait for watn to exit")]
fn release_verbose_completion(world: &mut WatnWorld) {
    super::incremental_sse_rendering_steps::release_stream(world);
    let invocation = world.live_stream.take().expect("live streaming invocation");
    let (status, stdout, stderr) = invocation.finish();
    world.exit_status = Some(status);
    world.output = Some(stdout);
    world.stderr_output = Some(stderr);
}

#[then(regex = r##"^stdout generated command line "([^"]+)" appears exactly once$"##)]
fn live_generated_command_once(world: &mut WatnWorld, command: String) {
    let stdout = world.output.as_deref().expect("stdout was not captured");
    assert_eq!(
        stdout.match_indices(&command).count(),
        1,
        "expected one generated command in stdout: {stdout:?}"
    );
}

#[then(expr = "stdout should not contain {string}")]
fn stdout_not_contains(world: &mut WatnWorld, text: String) {
    let stdout = world.output.as_deref().expect("stdout was not captured");
    assert!(
        !stdout.contains(&text),
        "unexpected stdout content {text:?}: {stdout:?}"
    );
}

#[given(
    regex = r##"^a streaming provider flushes content "([^"]+)" and then resets the connection before `\[DONE\]`$"##
)]
fn failure_provider(world: &mut WatnWorld, content: String) {
    super::incremental_sse_rendering_steps::configure_failure_content(world, content);
}

#[when(regex = r##"^I start the failing streaming command `watn "([^"]*)"` in a terminal$"##)]
fn start_failure_stream(world: &mut WatnWorld, question: String) {
    let session = start_pty_session(world, &[&question]);
    world.pty_session = Some(session);
}

#[then(expr = "the terminal output contains {string}")]
fn terminal_output_contains(world: &mut WatnWorld, text: String) {
    if world.pty_session.is_some() {
        wait_for_terminal_text(world, &text);
    } else {
        let output = world
            .output
            .as_deref()
            .expect("terminal output was not captured");
        assert!(
            output.contains(&text),
            "expected terminal output {text:?}: {output:?}"
        );
    }

    if text == "network error" {
        if let Some(session) = world.pty_session.take() {
            finish_pty_session(world, session);
        }
    }
}

#[then(regex = r##"^the terminal output shows spinner clear-line evidence after "([^"]+)"$"##)]
fn spinner_failure_evidence(world: &mut WatnWorld, _text: String) {
    let output = world.output.clone().unwrap_or_else(|| {
        pty_snapshot(world.pty_session.as_ref().expect("streaming PTY session"))
    });
    assert!(
        output.contains("\x1b[2K"),
        "expected spinner clear-line evidence, got {output:?}"
    );
}

#[then("the terminal output does not contain successful model metadata")]
fn terminal_no_metadata(world: &mut WatnWorld) {
    let output = world
        .output
        .as_deref()
        .expect("terminal output was not captured");
    assert!(
        !output.contains("tok/s"),
        "unexpected successful metadata: {output:?}"
    );
}

#[then(expr = "the terminal output does not contain {string}")]
fn terminal_output_not_contains(world: &mut WatnWorld, text: String) {
    let output = world
        .output
        .as_deref()
        .expect("terminal output was not captured");
    assert!(
        !output.contains(&text),
        "unexpected terminal output {text:?}: {output:?}"
    );
}

#[given(regex = r##"^a streaming provider emits content "([^"]+)"$"##)]
fn command_provider(world: &mut WatnWorld, content: String) {
    super::incremental_sse_rendering_steps::configure_command_content(world, content);
}

#[when(regex = r##"^I start the executable streaming command `watn -x "([^"]*)"` in a terminal$"##)]
fn start_executable_stream(world: &mut WatnWorld, question: String) {
    let session = start_pty_session(world, &["-x", &question]);
    world.pty_session = Some(session);
}

#[then(regex = r##"^the generated command line "([^"]+)" is visible before confirmation$"##)]
fn command_before_confirmation(world: &mut WatnWorld, command: String) {
    wait_for_terminal_text(world, &command);
    wait_for_terminal_text(world, "Execute now? [Y/n]");
}

#[then(
    regex = r##"^the terminal output does not contain an execution output line "([^"]+)" before confirmation$"##
)]
fn execution_absent_before_confirmation(world: &mut WatnWorld, output: String) {
    let terminal = pty_snapshot(world.pty_session.as_ref().expect("streaming PTY session"));
    let clean = strip_ansi(&terminal).replace('\r', "");
    assert!(
        !clean.lines().any(|line| line.trim() == output),
        "execution output appeared before confirmation: {clean:?}"
    );
}

#[when("I confirm execution with the raw terminal Enter key")]
fn confirm_raw_enter(world: &mut WatnWorld) {
    let session = world.pty_session.as_mut().expect("streaming PTY session");
    pty_write(session, "\r");
    let session = world.pty_session.take().expect("streaming PTY session");
    finish_pty_session(world, session);
}

#[then(regex = r##"^the execution output line "([^"]+)" appears exactly once$"##)]
fn execution_output_once(world: &mut WatnWorld, output: String) {
    let terminal = world
        .output
        .as_deref()
        .expect("terminal output was not captured");
    let clean = strip_ansi(terminal).replace('\r', "");
    let count = clean.lines().filter(|line| line.trim() == output).count();
    assert_eq!(count, 1, "expected one execution output line in {clean:?}");
}

fn strip_ansi(text: &str) -> String {
    Regex::new(r"\x1b\[[0-9;?]*[A-Za-z]")
        .expect("valid terminal escape regex")
        .replace_all(text, "")
        .into_owned()
}

fn wait_for_terminal_text(world: &WatnWorld, text: &str) {
    let deadline = Instant::now() + Duration::from_secs(3);
    loop {
        let output = pty_snapshot(world.pty_session.as_ref().expect("streaming PTY session"));
        if output.contains(text) {
            return;
        }
        assert!(
            Instant::now() < deadline,
            "terminal did not contain {text:?}: {output:?}"
        );
        thread::sleep(Duration::from_millis(25));
    }
}

#[when(regex = r##"^I run `watn -x "([^"]*)"` with piped confirmation "([^"]*)"$"##)]
fn piped_confirmation(world: &mut WatnWorld, question: String, confirmation: String) {
    super::run_binary_with_state(world, &["-x", &question], Some(&confirmation));
}

#[then(regex = r##"^the generated command line "([^"]+)" appears exactly once on stdout$"##)]
fn piped_command_once(world: &mut WatnWorld, command: String) {
    let stdout = world.output.as_deref().expect("stdout was not captured");
    let count = stdout.lines().filter(|line| line.trim() == command).count();
    assert_eq!(
        count, 1,
        "expected one generated command line in {stdout:?}"
    );
}

#[then(regex = r##"^the execution output line "([^"]+)" appears exactly once on stdout$"##)]
fn piped_execution_once(world: &mut WatnWorld, output: String) {
    let stdout = world.output.as_deref().expect("stdout was not captured");
    let count = stdout.lines().filter(|line| line.trim() == output).count();
    assert_eq!(count, 1, "expected one execution output line in {stdout:?}");
}
