use cucumber::{given, then, when};
use regex::Regex;
use serde_json::json;
use std::fmt;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use super::{finish_pty_session, pty_snapshot, start_pty_session};
use crate::WatnWorld;

#[derive(Default)]
pub struct StreamingState {
    server: Option<StreamingServer>,
    requested_model: Option<String>,
    pricing: Option<Pricing>,
    controlled_prefix: Option<String>,
    controlled_visible: Option<String>,
    controlled_error: Option<String>,
    controlled_metadata: bool,
    controlled_execution_prompted: bool,
}

impl fmt::Debug for StreamingState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("StreamingState")
            .field(
                "server",
                &self.server.as_ref().map(|server| &server.endpoint),
            )
            .field("requested_model", &self.requested_model)
            .field("pricing", &self.pricing)
            .field("controlled_visible", &self.controlled_visible)
            .field("controlled_error", &self.controlled_error)
            .field("controlled_metadata", &self.controlled_metadata)
            .field(
                "controlled_execution_prompted",
                &self.controlled_execution_prompted,
            )
            .finish()
    }
}

#[derive(Debug, Clone)]
struct Pricing {
    model: String,
    input: f64,
    output: f64,
}

struct FailingWriter {
    output: Vec<u8>,
    writes: usize,
    fail_after: usize,
}

impl FailingWriter {
    fn fails_on_next_write() -> Self {
        Self {
            output: Vec::new(),
            writes: 0,
            fail_after: 1,
        }
    }
}

impl Write for FailingWriter {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if self.writes >= self.fail_after {
            return Err(io::Error::new(
                io::ErrorKind::BrokenPipe,
                "controlled output failure",
            ));
        }
        self.writes += 1;
        self.output.extend_from_slice(bytes);
        Ok(bytes.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

struct StreamingServer {
    endpoint: String,
    handle: Option<JoinHandle<()>>,
    release: Option<Arc<ReleaseGate>>,
    stop: Arc<AtomicBool>,
}

#[derive(Default)]
struct ReleaseGate {
    released: Mutex<bool>,
    changed: Condvar,
}

impl ReleaseGate {
    fn wait(&self) {
        let mut released = self.released.lock().expect("lock release gate");
        while !*released {
            released = self.changed.wait(released).expect("wait for release gate");
        }
    }

    fn release(&self) {
        let mut released = self.released.lock().expect("lock release gate");
        *released = true;
        self.changed.notify_all();
    }
}

impl fmt::Debug for StreamingServer {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("StreamingServer")
            .field("endpoint", &self.endpoint)
            .field("running", &self.handle.is_some())
            .finish()
    }
}

impl Drop for StreamingServer {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(release) = &self.release {
            release.release();
        }
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

impl StreamingServer {
    fn start(events: Vec<Vec<u8>>, hold_after: Option<usize>) -> Self {
        Self::start_with_options(events, hold_after, false)
    }

    fn start_with_options(
        events: Vec<Vec<u8>>,
        hold_after: Option<usize>,
        bytewise_first: bool,
    ) -> Self {
        Self::start_with_initial_delay(events, hold_after, bytewise_first, Duration::ZERO)
    }

    fn start_with_initial_delay(
        events: Vec<Vec<u8>>,
        hold_after: Option<usize>,
        bytewise_first: bool,
        initial_delay: Duration,
    ) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind streaming provider twin");
        listener
            .set_nonblocking(true)
            .expect("set streaming provider listener mode");
        let address = listener
            .local_addr()
            .expect("read streaming provider address");
        let endpoint = format!("http://{}", address);
        let release = hold_after.map(|_| Arc::new(ReleaseGate::default()));
        let thread_release = release.clone();
        let stop = Arc::new(AtomicBool::new(false));
        let thread_stop = Arc::clone(&stop);
        let handle = thread::spawn(move || {
            let (mut stream, _) = loop {
                match listener.accept() {
                    Ok(connection) => break connection,
                    Err(error) if error.kind() == io::ErrorKind::WouldBlock => {
                        if thread_stop.load(Ordering::Relaxed) {
                            return;
                        }
                        thread::sleep(Duration::from_millis(5));
                    }
                    Err(_) => return,
                }
            };
            read_request_headers(&mut stream);

            let body_len: usize = events.iter().map(Vec::len).sum();
            let headers = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nContent-Length: {body_len}\r\nConnection: close\r\n\r\n"
            );
            stream
                .write_all(headers.as_bytes())
                .expect("write streaming provider headers");
            stream.flush().expect("flush streaming provider headers");
            if !initial_delay.is_zero() {
                thread::sleep(initial_delay);
            }

            for (index, event) in events.iter().enumerate() {
                if index > 0 {
                    thread::sleep(Duration::from_millis(30));
                }
                if bytewise_first && index == 0 {
                    for byte in event {
                        stream
                            .write_all(&[*byte])
                            .expect("write partial streaming provider event");
                        stream
                            .flush()
                            .expect("flush partial streaming provider event");
                        thread::sleep(Duration::from_millis(2));
                    }
                } else {
                    stream
                        .write_all(event)
                        .expect("write streaming provider event");
                    stream.flush().expect("flush streaming provider event");
                }
                if hold_after == Some(index) {
                    thread_release
                        .as_ref()
                        .expect("release gate for held stream")
                        .wait();
                }
            }
        });

        Self {
            endpoint,
            handle: Some(handle),
            release,
            stop,
        }
    }

    fn release(&self) {
        if let Some(release) = &self.release {
            release.release();
        }
    }
}

fn read_request_headers(stream: &mut TcpStream) {
    let mut request = Vec::new();
    let mut chunk = [0_u8; 1024];
    while !request.windows(4).any(|window| window == b"\r\n\r\n") {
        let Ok(bytes_read) = stream.read(&mut chunk) else {
            return;
        };
        if bytes_read == 0 {
            return;
        }
        request.extend_from_slice(&chunk[..bytes_read]);
    }
}

fn content_event(model: &str, content: &str) -> Vec<u8> {
    format!(
        "data: {}\n\n",
        json!({
            "id": "stream-1",
            "model": model,
            "choices": [{"index": 0, "delta": {"content": content}, "finish_reason": null}]
        })
    )
    .into_bytes()
}

pub(crate) fn reasoning_event(model: &str, reasoning: &str) -> Vec<u8> {
    format!(
        "data: {}\n\n",
        json!({
            "id": "stream-1",
            "model": model,
            "choices": [{"index": 0, "delta": {"reasoning": reasoning}, "finish_reason": null}]
        })
    )
    .into_bytes()
}

fn usage_event(model: &str, prompt_tokens: u32, completion_tokens: u32) -> Vec<u8> {
    format!(
        "data: {}\n\n",
        json!({
            "id": "stream-1",
            "model": model,
            "choices": [],
            "usage": {
                "prompt_tokens": prompt_tokens,
                "completion_tokens": completion_tokens,
                "total_tokens": prompt_tokens + completion_tokens
            }
        })
    )
    .into_bytes()
}

fn done_event() -> Vec<u8> {
    b"data: [DONE]\n\n".to_vec()
}

fn update_config(world: &mut WatnWorld) {
    let server = world
        .streaming
        .server
        .as_ref()
        .expect("streaming provider was not started");
    let requested_model = world
        .streaming
        .requested_model
        .as_deref()
        .unwrap_or("requested-model");
    let pricing_block = world
        .streaming
        .pricing
        .as_ref()
        .map(|pricing| {
            format!(
                "\n[pricing.\"{}\"]\ninput = {}\noutput = {}\n",
                pricing.model, pricing.input, pricing.output
            )
        })
        .unwrap_or_default();
    world.raw_config = Some(format!(
        "[defaults]\nprovider = \"streaming\"\nmodel = \"{requested_model}\"\n\n[providers.streaming]\nendpoint = \"{}\"\napi_key = \"test-key\"\n{}",
        server.endpoint, pricing_block
    ));
}

pub(crate) fn configure_delayed_content(world: &mut WatnWorld, first: String, second: String) {
    let requested_model = world
        .streaming
        .requested_model
        .clone()
        .unwrap_or_else(|| "test-model".to_string());
    let events = vec![
        content_event(&requested_model, &first),
        content_event(&requested_model, &second),
        done_event(),
    ];
    world.streaming.server = Some(StreamingServer::start_with_initial_delay(
        events,
        Some(0),
        false,
        Duration::from_millis(200),
    ));
    update_config(world);
}

pub(crate) fn release_stream(world: &WatnWorld) {
    if let Some(server) = &world.streaming.server {
        server.release();
    }
}

pub(crate) fn configure_verbose_content(
    world: &mut WatnWorld,
    reasoning: String,
    first_content: String,
) {
    let requested_model = world
        .streaming
        .requested_model
        .clone()
        .unwrap_or_else(|| "test-model".to_string());
    let events = vec![
        reasoning_event(&requested_model, &reasoning),
        content_event(&requested_model, &first_content),
        content_event(&requested_model, " f"),
        done_event(),
    ];
    world.streaming.server = Some(StreamingServer::start_with_options(events, Some(1), false));
    update_config(world);
}

pub(crate) fn configure_failure_content(world: &mut WatnWorld, content: String) {
    world.streaming.server = Some(StreamingServer::start(
        vec![content_event("test-model", &content)],
        None,
    ));
    update_config(world);
}

pub(crate) fn configure_command_content(world: &mut WatnWorld, content: String) {
    world.streaming.server = Some(StreamingServer::start(
        vec![content_event("test-model", &content), done_event()],
        None,
    ));
    update_config(world);
}

#[given(regex = r##"^the request asks for model "([^"]+)"$"##)]
fn requested_model(world: &mut WatnWorld, model: String) {
    world.streaming.requested_model = Some(model);
}

#[given(
    regex = r##"^a streaming provider emits content "([^"]+)" and a choices-empty usage event with response model "([^"]+)", (\d+) prompt tokens, and (\d+) completion tokens$"##
)]
fn usage_only_provider(
    world: &mut WatnWorld,
    content: String,
    response_model: String,
    prompt_tokens: u32,
    completion_tokens: u32,
) {
    let requested_model = world
        .streaming
        .requested_model
        .clone()
        .unwrap_or_else(|| "requested-model".to_string());
    let events = vec![
        content_event(&requested_model, &content),
        usage_event(&response_model, prompt_tokens, completion_tokens),
        done_event(),
    ];
    world.streaming.server = Some(StreamingServer::start(events, None));
    update_config(world);
}

#[given(
    regex = r##"^pricing is configured only for "([^"]+)" at ([0-9]+\.[0-9]+) input and ([0-9]+\.[0-9]+) output per million tokens$"##
)]
fn response_model_pricing(world: &mut WatnWorld, model: String, input: f64, output: f64) {
    world.streaming.pricing = Some(Pricing {
        model,
        input,
        output,
    });
    update_config(world);
}

#[then(expr = "stdout should contain {string}")]
fn stdout_contains(world: &mut WatnWorld, text: String) {
    let stdout = world.output.as_deref().expect("stdout was not captured");
    assert!(
        stdout.contains(&text),
        "expected stdout to contain {text:?}, got {stdout:?}"
    );
}

#[then(expr = "the final metadata names exactly {string}")]
fn final_metadata_names(world: &mut WatnWorld, model: String) {
    let stderr = world
        .stderr_output
        .as_deref()
        .expect("stderr was not captured");
    assert!(
        stderr.contains(&format!("{model} ·")),
        "expected final metadata for {model:?}, got {stderr:?}"
    );
}

#[then(expr = "stderr should not contain final metadata for {string}")]
fn stderr_not_name_model(world: &mut WatnWorld, model: String) {
    let stderr = world
        .stderr_output
        .as_deref()
        .expect("stderr was not captured");
    assert!(
        !stderr.contains(&format!("{model} ·")),
        "did not expect final metadata for {model:?}, got {stderr:?}"
    );
}

#[then(expr = "stderr should contain a non-zero cost for {string}")]
fn stderr_has_nonzero_cost(world: &mut WatnWorld, model: String) {
    let stderr = world
        .stderr_output
        .as_deref()
        .expect("stderr was not captured");
    assert!(
        stderr.contains(&model),
        "expected model metadata in {stderr:?}"
    );
    let cost = Regex::new(r"\$(\d+\.\d{4})")
        .expect("valid cost regex")
        .captures(stderr)
        .and_then(|captures| captures.get(1))
        .and_then(|value| value.as_str().parse::<f64>().ok())
        .expect("metadata should contain a cost");
    assert!(cost > 0.0, "expected non-zero cost, got {cost}");
}

#[then("stderr should contain a positive throughput value")]
fn stderr_has_positive_throughput(world: &mut WatnWorld) {
    let stderr = world
        .stderr_output
        .as_deref()
        .expect("stderr was not captured");
    let tok_s = Regex::new(r"(\d+(?:\.\d+)?) tok/s")
        .expect("valid throughput regex")
        .captures(stderr)
        .and_then(|captures| captures.get(1))
        .and_then(|value| value.as_str().parse::<f64>().ok())
        .expect("metadata should contain throughput");
    assert!(tok_s > 0.0, "expected positive throughput, got {tok_s}");
}

#[given(
    regex = r##"^a streaming provider emits content "([^"]+)", sends `\[DONE\]`, and holds the connection open until released$"##
)]
fn done_provider(world: &mut WatnWorld, content: String) {
    let requested_model = world
        .streaming
        .requested_model
        .clone()
        .unwrap_or_else(|| "test-model".to_string());
    let events = vec![content_event(&requested_model, &content), done_event()];
    world.streaming.server = Some(StreamingServer::start(events, Some(1)));
    update_config(world);
}

#[when(regex = r##"^I start the streaming command `watn "([^"]*)"`$"##)]
fn start_streaming_command(world: &mut WatnWorld, question: String) {
    let session = start_pty_session(world, &[&question]);
    world.pty_session = Some(session);
}

#[then("watn exits successfully before the provider connection is released")]
fn exits_before_release(world: &mut WatnWorld) {
    let session = world.pty_session.as_mut().expect("streaming PTY session");
    let deadline = std::time::Instant::now() + Duration::from_secs(3);
    loop {
        if let Some(status) = session.child.try_wait().expect("poll streaming child") {
            assert_eq!(status.exit_code(), 0);
            return;
        }
        assert!(
            std::time::Instant::now() < deadline,
            "watn did not exit before the provider release"
        );
        thread::sleep(Duration::from_millis(25));
    }
}

#[then(regex = r##"^the generated command line "([^"]+)" appears exactly once$"##)]
fn generated_command_once(world: &mut WatnWorld, command: String) {
    let output = world
        .pty_session
        .as_ref()
        .map(pty_snapshot)
        .or_else(|| world.output.clone())
        .expect("streaming output");
    let occurrences = output.match_indices(&command).count();
    assert_eq!(
        occurrences, 1,
        "expected one generated command, got {output:?}"
    );
}

#[when("I release the provider connection")]
fn release_provider(world: &mut WatnWorld) {
    if let Some(server) = &world.streaming.server {
        server.release();
    }
    if let Some(session) = world.pty_session.take() {
        finish_pty_session(world, session);
    }
}

#[given(
    regex = r##"^a streaming provider sends the first content event one byte at a time with content "([^"]+)" and holds the next event$"##
)]
fn partial_provider(world: &mut WatnWorld, content: String) {
    let events = vec![content_event("test-model", &content), done_event()];
    world.streaming.server = Some(StreamingServer::start_with_options(events, Some(0), true));
    update_config(world);
}

#[then(
    regex = r##"^the streamed fragment "([^"]+)" is visible before the provider releases the next event$"##
)]
fn streamed_fragment(world: &mut WatnWorld, fragment: String) {
    let deadline = std::time::Instant::now() + Duration::from_secs(3);
    loop {
        let output = world
            .pty_session
            .as_ref()
            .map(pty_snapshot)
            .expect("streaming PTY session");
        if output.contains(&fragment) {
            return;
        }
        assert!(
            std::time::Instant::now() < deadline,
            "expected streamed fragment {:?} before release, got {:?}",
            fragment,
            output
        );
        thread::sleep(Duration::from_millis(25));
    }
}

#[when("I release the next event and wait for watn to exit")]
fn release_next_event(world: &mut WatnWorld) {
    if let Some(server) = &world.streaming.server {
        server.release();
    }
    if let Some(session) = world.pty_session.take() {
        finish_pty_session(world, session);
    }
}

#[given(
    regex = r##"^a streaming provider sends a malformed event, flushes valid content "([^"]+)", and holds `\[DONE\]`$"##
)]
fn malformed_provider(world: &mut WatnWorld, content: String) {
    let events = vec![
        b"data: {not-json}\n\n".to_vec(),
        content_event("test-model", &content),
        done_event(),
    ];
    world.streaming.server = Some(StreamingServer::start_with_options(events, Some(1), false));
    update_config(world);
}

#[then(
    regex = r##"^the valid streamed fragment "([^"]+)" is visible before the provider releases `\[DONE\]`$"##
)]
fn valid_fragment(world: &mut WatnWorld, fragment: String) {
    streamed_fragment(world, fragment);
}

#[when("I release `[DONE]` and wait for watn to exit")]
fn release_done(world: &mut WatnWorld) {
    release_next_event(world);
}

#[given(
    regex = r##"^a streaming provider flushes valid content "([^"]+)" and closes cleanly without sending `\[DONE\]`$"##
)]
fn eof_provider(world: &mut WatnWorld, content: String) {
    world.streaming.server = Some(StreamingServer::start(
        vec![content_event("test-model", &content)],
        None,
    ));
    update_config(world);
}

#[then("stderr should not contain successful model metadata")]
fn stderr_no_success_metadata(world: &mut WatnWorld) {
    let stderr = world
        .stderr_output
        .as_deref()
        .expect("stderr was not captured");
    assert!(
        !stderr.contains("tok/s"),
        "unexpected successful metadata in stderr: {stderr:?}"
    );
}

#[given(
    regex = r##"^the streaming output sink flushes prefix "([^"]+)" and fails on the next write$"##
)]
fn controlled_sink(world: &mut WatnWorld, prefix: String) {
    world.streaming.controlled_prefix = Some(prefix);
}

#[when("I render the streaming response through the controlled output sink")]
fn render_controlled_sink(world: &mut WatnWorld) {
    let prefix = world
        .streaming
        .controlled_prefix
        .clone()
        .expect("controlled prefix");
    let mut writer = FailingWriter::fails_on_next_write();
    assert!(
        watn::output::render::write_streamed_content(&mut writer, &prefix).is_ok(),
        "the controlled sink should accept the visible prefix"
    );
    let error = watn::output::render::write_streamed_content(&mut writer, " suffix")
        .expect_err("the controlled sink should fail on the next write");
    world.streaming.controlled_visible = Some(String::from_utf8_lossy(&writer.output).to_string());
    world.streaming.controlled_error = Some(error.to_string());
    world.streaming.controlled_metadata = false;
    world.streaming.controlled_execution_prompted = false;
    world.exit_status = Some(1);
}

#[then(regex = r##"^the visible command prefix is preserved as "([^"]+)"$"##)]
fn visible_prefix(world: &mut WatnWorld, prefix: String) {
    assert_eq!(
        world.streaming.controlled_visible.as_deref(),
        Some(prefix.as_str())
    );
}

#[then("the existing I/O error is reported")]
fn io_error_reported(world: &mut WatnWorld) {
    assert_eq!(
        world.streaming.controlled_error.as_deref(),
        Some("controlled output failure")
    );
}

#[then("final success metadata is omitted")]
fn metadata_omitted(world: &mut WatnWorld) {
    assert!(!world.streaming.controlled_metadata);
}

#[then("execution is not prompted")]
fn execution_not_prompted(world: &mut WatnWorld) {
    assert!(!world.streaming.controlled_execution_prompted);
}
