use cucumber::{given, then, when};
use std::fmt;
use std::io;
use std::net::{Shutdown, TcpListener};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use super::{finish_pty_session, pty_snapshot, pty_write, start_pty_session};
use crate::WatnWorld;

use super::incremental_sse_rendering_steps::read_request_headers;

/// A black-hole provider twin: accepts one TCP connection, reads the request
/// headers, and never writes a response until torn down.
pub struct HangServer {
    endpoint: String,
    stop: Arc<AtomicBool>,
    handle: Option<JoinHandle<()>>,
}

impl fmt::Debug for HangServer {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("HangServer")
            .field("endpoint", &self.endpoint)
            .field("running", &self.handle.is_some())
            .finish()
    }
}

impl Drop for HangServer {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

impl HangServer {
    fn start() -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind black-hole provider twin");
        listener
            .set_nonblocking(true)
            .expect("set black-hole listener mode");
        let address = listener.local_addr().expect("read black-hole address");
        let endpoint = format!("http://{}", address);
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
            while !thread_stop.load(Ordering::Relaxed) {
                thread::sleep(Duration::from_millis(20));
            }
            let _ = stream.shutdown(Shutdown::Both);
        });

        Self {
            endpoint,
            stop,
            handle: Some(handle),
        }
    }
}

fn update_config_for(endpoint: &str, world: &mut WatnWorld) {
    world.raw_config = Some(format!(
        "[defaults]\nprovider = \"streaming\"\nmodel = \"test-model\"\n\n[providers.streaming]\nendpoint = \"{}\"\napi_key = \"test-key\"\n",
        endpoint
    ));
}

// ===== GIVEN =====

#[given(
    regex = r##"^a streaming provider flushes content "([^"]+)" and holds the stream open without `\[DONE\]`$"##
)]
fn held_open_provider(world: &mut WatnWorld, content: String) {
    super::incremental_sse_rendering_steps::configure_held_open_without_done(world, content);
}

#[given("a provider accepts a connection and never sends a response")]
fn hanging_provider(world: &mut WatnWorld) {
    let server = HangServer::start();
    let endpoint = server.endpoint.clone();
    world.hanging_server = Some(server);
    update_config_for(&endpoint, world);
}

// ===== WHEN =====

#[when(regex = r##"^I start watn with the invocation `watn "([^"]*)"` in a terminal$"##)]
fn start_cancel_invocation(world: &mut WatnWorld, question: String) {
    let session = start_pty_session(world, &[&question]);
    world.pty_session = Some(session);
}

#[when("I press Ctrl+C")]
fn press_ctrl_c(world: &mut WatnWorld) {
    let session = world.pty_session.as_mut().expect("cancel PTY session");
    pty_write(session, "\x03");
    if let Some(session) = world.pty_session.take() {
        finish_pty_session(world, session);
    }
}

// ===== THEN =====

#[then(regex = r##"^the first streamed content "([^"]+)" is visible$"##)]
fn first_streamed_content(world: &mut WatnWorld, content: String) {
    wait_for_terminal_text(world, &content);
}

#[then("the progress indicator is visible while the connection is pending")]
fn progress_indicator_pending(world: &mut WatnWorld) {
    wait_for_terminal_text(world, "Asking");
}

#[then("stderr should not contain a reported error")]
fn no_reported_error(world: &mut WatnWorld) {
    let output = world
        .output
        .as_deref()
        .expect("merged PTY output was not captured");
    for fragment in [
        "network error",
        "I/O error",
        "API error",
        "authentication error",
        "config error",
        "unknown provider",
    ] {
        assert!(
            !output.contains(fragment),
            "unexpected error text {fragment:?} in output: {output:?}"
        );
    }
}

#[then("stderr should not contain final metadata")]
fn no_final_metadata(world: &mut WatnWorld) {
    let output = world
        .output
        .as_deref()
        .expect("merged PTY output was not captured");
    assert!(
        !output.contains("tok/s"),
        "unexpected final metadata in output: {output:?}"
    );
}

fn wait_for_terminal_text(world: &WatnWorld, text: &str) {
    let deadline = Instant::now() + Duration::from_secs(3);
    loop {
        let output = pty_snapshot(world.pty_session.as_ref().expect("cancel PTY session"));
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
