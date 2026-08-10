use cucumber::{given, then};
use regex::Regex;
use serde_json::json;
use std::fmt;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use crate::WatnWorld;

#[derive(Default)]
pub struct StreamingState {
    server: Option<StreamingServer>,
    requested_model: Option<String>,
    pricing: Option<Pricing>,
}

impl fmt::Debug for StreamingState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("StreamingState")
            .field("server", &self.server.as_ref().map(|server| &server.endpoint))
            .field("requested_model", &self.requested_model)
            .field("pricing", &self.pricing)
            .finish()
    }
}

#[derive(Debug, Clone)]
struct Pricing {
    model: String,
    input: f64,
    output: f64,
}

struct StreamingServer {
    endpoint: String,
    handle: Option<JoinHandle<()>>,
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
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

impl StreamingServer {
    fn start(events: Vec<Vec<u8>>) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind streaming provider twin");
        let address = listener
            .local_addr()
            .expect("read streaming provider address");
        let endpoint = format!("http://{}", address);
        let handle = thread::spawn(move || {
            let Ok((mut stream, _)) = listener.accept() else {
                return;
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

            for (index, event) in events.iter().enumerate() {
                if index > 0 {
                    thread::sleep(Duration::from_millis(30));
                }
                stream
                    .write_all(event)
                    .expect("write streaming provider event");
                stream.flush().expect("flush streaming provider event");
            }
        });

        Self {
            endpoint,
            handle: Some(handle),
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
    world.streaming.server = Some(StreamingServer::start(events));
    update_config(world);
}

#[given(
    regex = r##"^pricing is configured only for "([^"]+)" at ([0-9]+\.[0-9]+) input and ([0-9]+\.[0-9]+) output per million tokens$"##
)]
fn response_model_pricing(
    world: &mut WatnWorld,
    model: String,
    input: f64,
    output: f64,
) {
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
    let stderr = world.stderr_output.as_deref().expect("stderr was not captured");
    assert!(
        stderr.contains(&format!("{model} ·")),
        "expected final metadata for {model:?}, got {stderr:?}"
    );
}

#[then(expr = "stderr should not contain final metadata for {string}")]
fn stderr_not_name_model(world: &mut WatnWorld, model: String) {
    let stderr = world.stderr_output.as_deref().expect("stderr was not captured");
    assert!(
        !stderr.contains(&format!("{model} ·")),
        "did not expect final metadata for {model:?}, got {stderr:?}"
    );
}

#[then(expr = "stderr should contain a non-zero cost for {string}")]
fn stderr_has_nonzero_cost(world: &mut WatnWorld, model: String) {
    let stderr = world.stderr_output.as_deref().expect("stderr was not captured");
    assert!(stderr.contains(&model), "expected model metadata in {stderr:?}");
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
    let stderr = world.stderr_output.as_deref().expect("stderr was not captured");
    let tok_s = Regex::new(r"(\d+(?:\.\d+)?) tok/s")
        .expect("valid throughput regex")
        .captures(stderr)
        .and_then(|captures| captures.get(1))
        .and_then(|value| value.as_str().parse::<f64>().ok())
        .expect("metadata should contain throughput");
    assert!(tok_s > 0.0, "expected positive throughput, got {tok_s}");
}
