pub mod ask_steps;
pub mod catalog_source_steps;
pub mod config_steps;
pub mod credentials_steps;
pub mod model_picker_layout_steps;
pub mod models_steps;
pub mod provider_setup_layout_steps;
pub mod provider_setup_steps;
pub mod providers_steps;
pub mod reasoning_policy_steps;
pub mod search_concurrency_steps;
pub mod setup_persistence_steps;
pub mod setup_wizard_steps;
pub mod transport_steps;

pub use transport_steps::TransportState;

use std::path::PathBuf;

use httpmock::{Method, MockServer};
use regex::Regex;
use std::sync::{Arc, Mutex};

use crate::MockServerWrap;
use std::io::{Read, Write};

pub(crate) fn find_binary() -> PathBuf {
    binary_from_env("WATN_TEST_SUPPORT_DEBUG_BIN")
}

pub(crate) fn binary_from_env(name: &str) -> PathBuf {
    let path = std::env::var_os(name)
        .map(PathBuf::from)
        .unwrap_or_else(|| panic!("{name} must point to a prebuilt watn binary"));
    assert!(
        path.is_file(),
        "{name} does not point to a file: {}",
        path.display()
    );
    path
}

pub(crate) fn build_config(
    default_provider: &str,
    tiers: Option<(&str, &str, &str)>,
    custom_providers: Option<Vec<(&str, &str, &str, &str)>>,
    pricing: Option<Vec<(&str, f64, f64)>>,
    litellm: Option<(&str, &str)>,
    default_model: Option<&str>,
) -> String {
    let mut lines = Vec::new();

    lines.push("[defaults]".to_string());
    lines.push(format!("provider = \"{}\"", default_provider));
    if let Some(m) = default_model {
        lines.push(format!("model = \"{}\"", m));
    }

    if let Some((s, n, t)) = tiers {
        lines.push(String::new());
        lines.push("[tiers]".to_string());
        lines.push(format!("small = \"{}\"", s));
        lines.push(format!("normal = \"{}\"", n));
        lines.push(format!("thinking = \"{}\"", t));
    }

    if let Some(providers) = custom_providers {
        for (name, endpoint, api_key, default_model) in providers {
            lines.push(String::new());
            lines.push(format!("[providers.{}]", name));
            lines.push(format!("endpoint = \"{}\"", endpoint));
            lines.push(format!("api_key = \"{}\"", api_key));
            if !default_model.is_empty() {
                lines.push(format!("default_model = \"{}\"", default_model));
            }
        }
    }

    if let Some(p) = pricing {
        lines.push(String::new());
        lines.push("[pricing]".to_string());
        for (model, input, output) in p {
            lines.push(format!(
                "\"{}\" = {{ input = {}, output = {} }}",
                model, input, output
            ));
        }
    }

    if let Some((endpoint, api_key)) = litellm {
        lines.push(String::new());
        lines.push("[litellm]".to_string());
        lines.push(format!("endpoint = \"{}\"", endpoint));
        lines.push(format!("api_key = \"{}\"", api_key));
    }

    lines.join("\n")
}

fn setup_chat_completion_mock(
    server_ref: &httpmock::MockServer,
    output: &str,
    include_usage: bool,
    delay_ms: u64,
    reasoning: &Option<String>,
    auth_header: Option<String>,
    body_requirement: Option<String>,
) -> usize {
    let mc = output.to_string();
    let include_usage_val = include_usage;
    let reasoning_clone = reasoning.clone();
    let auth_clone = auth_header.clone();
    let mock = server_ref.mock(move |when, then| {
        let mut when = when.method(Method::POST);
        if let Some(body_req) = &body_requirement {
            when = when.body_contains(body_req);
        }
        if let Some(ref auth) = auth_clone {
            when = when.header("Authorization", auth);
        }
        let _ = when.path("/chat/completions");
        if delay_ms > 0 {
            std::thread::sleep(std::time::Duration::from_millis(delay_ms));
        }
        let reasoning_delta = reasoning_clone.as_ref().map(|r| format!(",\"reasoning\":\"{}\"", r.replace('"', "\\\""))).unwrap_or_default();
        then.status(200)
            .header("Content-Type", "text/event-stream")
            .body(format!(
                "data: {{\"id\":\"1\",\"choices\":[{{\"index\":0,\"delta\":{{\"content\":\"{}\"{}}},\"finish_reason\":\"stop\"}}]{}}}\ndata: [DONE]\n",
                mc.replace('"', "\\\""),
                reasoning_delta,
                if include_usage_val { ",\"usage\":{\"prompt_tokens\":10,\"completion_tokens\":20,\"total_tokens\":30}" } else { "" }
            ));
    });
    mock.id
}

fn setup_models_mock(
    server_ref: &httpmock::MockServer,
    models: &[String],
    fail: bool,
) -> Option<usize> {
    let mock = if fail {
        server_ref.mock(move |when, then| {
            when.method(Method::GET).path("/models");
            then.status(500).body(r#"{"error":"server error"}"#);
        })
    } else {
        let models_clone = models.to_vec();
        server_ref.mock(move |when, then| {
            when.method(Method::GET).path("/models");
            let data: Vec<serde_json::Value> = models_clone
                .iter()
                .map(|id| serde_json::json!({"id": id}))
                .collect();
            then.status(200)
                .header("Content-Type", "application/json")
                .body(serde_json::json!({"data": data}).to_string());
        })
    };
    Some(mock.id)
}

fn setup_auth_fail_mock(server_ref: &httpmock::MockServer) {
    server_ref.mock(move |when, then| {
        when.method(Method::POST).path("/chat/completions");
        then.status(401).body(r#"{"error":"unauthorized"}"#);
    });
}

/// Rewrites `endpoint = "..."` inside `[providers.*]` sections to use the mock base URL.
fn rewrite_provider_endpoints(content: &str, base_url: &str) -> String {
    let provider_header = Regex::new(r"^\[providers\.\w+\]$").unwrap();
    let endpoint_re = Regex::new(r#"^(endpoint\s*=\s*)"(?:[^"]*)"\s*$"#).unwrap();
    let mut in_provider = false;
    let mut result = String::new();
    for line in content.lines() {
        if provider_header.is_match(line.trim()) {
            in_provider = true;
            result.push_str(line);
            result.push('\n');
            continue;
        }
        if in_provider {
            if line.starts_with('[') && !line.trim().starts_with('[') {
                // New section — leave provider block
                in_provider = false;
            } else if endpoint_re.is_match(line) {
                result.push_str(&format!("endpoint = \"{}\"", base_url));
                result.push('\n');
                continue;
            }
        }
        result.push_str(line);
        result.push('\n');
    }
    result
}

pub(crate) fn ensure_test_env(world: &mut crate::WatnWorld) {
    let mut config_content = String::new();
    let mut has_config = false;

    if world.pending_mock_model.is_some() || world.pending_mock_output.is_some() {
        let model = world
            .pending_mock_model
            .clone()
            .unwrap_or_else(|| "test-model".to_string());
        let output = world
            .pending_mock_output
            .clone()
            .unwrap_or_else(|| "output".to_string());
        let include_usage = world.pending_mock_usage.unwrap_or(false);
        let auth_fail = world.pending_mock_auth_fail;
        let no_config = world.pending_mock_no_config_file;

        let reuse_existing_server = world.mock_server.0.is_some();
        let server: &httpmock::MockServer;

        if reuse_existing_server {
            server = world.mock_server.0.as_ref().unwrap();
        } else {
            let new_server = MockServer::start();
            world.mock_server = MockServerWrap(Some(new_server), None);
            server = world.mock_server.0.as_ref().unwrap();
        }

        let base_url = format!("http://127.0.0.1:{}", server.port());

        if auth_fail {
            setup_auth_fail_mock(server);
            config_content = build_config(
                "test",
                None,
                Some(vec![("test", &base_url, "test-key", &model)]),
                None,
                None,
                None,
            );
            has_config = true;
        } else {
            if world.pending_mock_no_reasoning_assert {
                // Register a blocking mock FIRST (lower id) that matches any
                // chat request whose body contains "reasoning_effort" and
                // returns HTTP 400. httpmock matches the first mock by id.
                world.blocking_mock_id = Some(
                    server
                        .mock(move |when, then| {
                            when.method(Method::POST)
                                .path("/chat/completions")
                                .body_contains("\"reasoning_effort\"");
                            then.status(400).body(r#"{"error":"should not reason"}"#);
                        })
                        .id,
                );
            }
            let auth_header = world
                .pending_config
                .get("expect_custom_auth")
                .and_then(|_| world.env_vars.get("WATN_CUSTOM_API_KEY"))
                .or_else(|| world.env_vars.get("WATN_OPENAI_API_KEY"))
                .map(|key| format!("Bearer {}", key));
            if !world.pending_config.contains_key("expect_custom_auth") {
                let mock_id = setup_chat_completion_mock(
                    server,
                    &output,
                    include_usage,
                    world.pending_mock_delay_ms.unwrap_or(0),
                    &world.pending_mock_reasoning,
                    auth_header,
                    world.pending_mock_expected_reasoning_body.clone(),
                );
                world.mock_server.1 = Some(mock_id);
            }

            if !world.pending_mock_returned_models.is_empty() && world.models_mock_id.is_none() {
                world.models_mock_id = setup_models_mock(
                    server,
                    &world.pending_mock_returned_models,
                    world.pending_mock_models_fail,
                );
            }

            if reuse_existing_server {
                let raw = world.raw_config.clone().unwrap_or_default();
                config_content = rewrite_provider_endpoints(&raw, &base_url)
                    .replace("http://localhost:4000", &base_url);
                has_config = !no_config;
            } else {
                let raw = world.raw_config.clone().unwrap_or_default();
                let mut lines: Vec<&str> = raw.lines().collect();
                if let Some(defaults_idx) = lines.iter().position(|l| l.trim() == "[defaults]") {
                    let mut end = defaults_idx + 1;
                    while end < lines.len()
                        && !lines[end].starts_with('[')
                        && !lines[end].trim().is_empty()
                    {
                        end += 1;
                    }
                    lines.drain(defaults_idx..end);
                }
                let non_default = lines.join("\n").trim().to_string();

                let mock_cfg = build_config(
                    "test",
                    None,
                    Some(vec![("test", &base_url, "test-key", &model)]),
                    None,
                    None,
                    None,
                );

                if !non_default.is_empty() {
                    let rewritten = rewrite_provider_endpoints(&non_default, &base_url)
                        .replace("http://localhost:4000", &base_url);
                    config_content = format!("{}\n\n{}", mock_cfg, rewritten);
                } else {
                    config_content = mock_cfg;
                }
                has_config = !no_config;

                if world.pending_mock_returned_models.is_empty() && raw.contains("[litellm]") {
                    let default_models = vec!["test-model".to_string()];
                    world.models_mock_id = setup_models_mock(server, &default_models, false);
                    world.pending_mock_returned_models = default_models;
                }
            }
        }
    } else if let Some(ref raw) = world.raw_config {
        has_config = true;

        if let Some(ref server) = world.mock_server.0 {
            let base_url = format!("http://127.0.0.1:{}", server.port());
            config_content = rewrite_provider_endpoints(raw, &base_url);

            if !world.pending_mock_returned_models.is_empty() && world.models_mock_id.is_none() {
                world.models_mock_id = setup_models_mock(
                    server,
                    &world.pending_mock_returned_models,
                    world.pending_mock_models_fail,
                );
            }
            if world.pending_mock_model.is_some() || world.pending_mock_output.is_some() {
                let _model = world.pending_mock_model.as_deref().unwrap_or("test-model");
                let output = world.pending_mock_output.as_deref().unwrap_or("output");
                let include_usage = world.pending_mock_usage.unwrap_or(false);
                let auth_header = world
                    .pending_config
                    .get("expect_custom_auth")
                    .and_then(|_| world.env_vars.get("WATN_CUSTOM_API_KEY"))
                    .or_else(|| world.env_vars.get("WATN_OPENAI_API_KEY"))
                    .map(|key| format!("Bearer {}", key));

                if world.pending_mock_no_reasoning_assert {
                    // Register a blocking mock FIRST (lower id) that matches any
                    // chat request whose body contains "reasoning_effort" and
                    // returns HTTP 400. httpmock matches the first mock by id.
                    world.blocking_mock_id = Some(
                        server
                            .mock(move |when, then| {
                                when.method(Method::POST)
                                    .path("/chat/completions")
                                    .body_contains("\"reasoning_effort\"");
                                then.status(400).body(r#"{"error":"should not reason"}"#);
                            })
                            .id,
                    );
                }

                let mock_id = setup_chat_completion_mock(
                    server,
                    output,
                    include_usage,
                    world.pending_mock_delay_ms.unwrap_or(0),
                    &world.pending_mock_reasoning,
                    auth_header,
                    world.pending_mock_expected_reasoning_body.clone(),
                );
                world.mock_server.1 = Some(mock_id);
            }
        } else {
            config_content = raw.clone();
        }
    }

    if !has_config && world.mock_server.0.is_none() {
        let server = MockServer::start();
        let base_url = format!("http://127.0.0.1:{}", server.port());
        world.mock_server = MockServerWrap(Some(server), None);
        let server_ref = world.mock_server.0.as_ref().unwrap();
        let mock = server_ref.mock(move |when, then| {
            when.method(Method::POST).path("/chat/completions");
            then.status(200)
                .header("Content-Type", "text/event-stream")
                .body("data: {\"id\":\"1\",\"choices\":[{\"index\":0,\"delta\":{\"content\":\"output\"},\"finish_reason\":\"stop\"}],\"usage\":{\"prompt_tokens\":10,\"completion_tokens\":5,\"total_tokens\":15}}\ndata: [DONE]\n");
        });
        world.mock_server.1 = Some(mock.id);

        if !world.pending_mock_no_config_file {
            if let Some(raw) = world.raw_config.clone() {
                config_content = rewrite_provider_endpoints(&raw, &base_url);
                has_config = true;
                if raw.contains("[litellm]") {
                    let models = vec!["test-model".to_string()];
                    world.models_mock_id = setup_models_mock(server_ref, &models, false);
                }
            } else {
                config_content = build_config(
                    "test",
                    None,
                    Some(vec![("test", &base_url, "test-key", "test-model")]),
                    None,
                    None,
                    None,
                );
                has_config = true;
            }
        }
    }

    // If the default provider is "openai" and there is no explicit
    // [providers.openai] section, inject one so the mock server is used.
    // Also inject when WATN_OPENAI_API_KEY env var is set (binary uses
    // --provider openai and needs the endpoint to point to the mock).
    if let Some(ref server) = world.mock_server.0 {
        let base_url = format!("http://127.0.0.1:{}", server.port());
        let needs_openai = config_content.contains(r#"provider = "openai""#)
            || world.env_vars.contains_key("WATN_OPENAI_API_KEY");
        if has_config && !config_content.contains("[providers.openai]") && needs_openai {
            config_content.push_str(&format!(
                "\n[providers.openai]\nendpoint = \"{}\"\n",
                base_url
            ));
        }
    }

    // When WATN_PROVIDER selects a provider, ensure that provider resolves to
    // the mock server so the env-var override scenario can actually reach it.
    if let Some(provider_name) = world.env_vars.get("WATN_PROVIDER").cloned() {
        if let Some(ref server) = world.mock_server.0 {
            let base_url = format!("http://127.0.0.1:{}", server.port());
            let section = format!("[providers.{}]", provider_name);
            if has_config && !config_content.contains(&section) {
                config_content.push_str(&format!(
                    "\n{}\nendpoint = \"{}\"\napi_key = \"test-key\"\ndefault_model = \"test-model\"\n",
                    section, base_url
                ));
            }
        }
    }

    if has_config {
        let (dir, should_write) = if let Some(ref existing_dir) = world.temp_dir {
            (existing_dir.path().to_path_buf(), false)
        } else {
            let new_dir = tempfile::tempdir().expect("create temp dir");
            let path = new_dir.path().to_path_buf();
            world.temp_dir = Some(new_dir);
            (path, true)
        };
        if should_write {
            let config_dir = dir.join("watn");
            std::fs::create_dir_all(&config_dir).expect("create config subdir");
            let config_path = config_dir.join("config.toml");
            std::fs::write(&config_path, &config_content).expect("write config");
        }
        world
            .env_vars
            .entry("XDG_CONFIG_HOME".to_string())
            .or_insert_with(|| dir.to_string_lossy().to_string());
    }
}

pub(crate) fn apply_env(world: &crate::WatnWorld, cmd: &mut std::process::Command) {
    cmd.env_remove("WATN_OPENAI_API_KEY");
    cmd.env_remove("WATN_PROVIDER");
    cmd.env_remove("WATN_MODEL");
    cmd.env_remove("OPENROUTER_API_KEY");
    cmd.env_remove("WATN_API_KEY");
    cmd.env_remove("WATN_TEST_ENDPOINT_OVERRIDE");
    for (key, value) in &world.env_vars {
        cmd.env(key, value);
    }
}

pub(crate) fn run_binary_with_state(
    world: &mut crate::WatnWorld,
    args: &[&str],
    stdin_text: Option<&str>,
) {
    let binary = find_binary();
    ensure_test_env(world);

    let mut cmd = std::process::Command::new(&binary);
    cmd.args(args);
    apply_env(world, &mut cmd);

    let result = if let Some(input) = stdin_text {
        let mut child = cmd
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .expect("spawn binary");
        if let Some(ref mut stdin) = child.stdin {
            stdin.write_all(input.as_bytes()).expect("write stdin");
        }
        child.wait_with_output().expect("wait for output")
    } else {
        cmd.output().expect("run binary")
    };

    world.output = Some(String::from_utf8_lossy(&result.stdout).to_string());
    world.stderr_output = Some(String::from_utf8_lossy(&result.stderr).to_string());
    world.exit_status = result.status.code();
}

/// A persistent PTY subprocess session, kept alive across multiple Gherkin
/// steps so an interactive multi-tier flow can be driven incrementally.
pub(crate) struct PtySession {
    pub child: Box<dyn portable_pty::Child + Send + Sync>,
    pub writer: Option<Box<dyn Write + Send>>,
    pub output_buffer: Arc<Mutex<Vec<u8>>>,
    pub reader_handle: Option<std::thread::JoinHandle<()>>,
    pub finished: bool,
}

impl std::fmt::Debug for PtySession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PtySession")
            .field("child", &self.child)
            .field("writer", &self.writer.is_some())
            .field(
                "output_buffer",
                &self
                    .output_buffer
                    .lock()
                    .map(|buffer| buffer.len())
                    .unwrap_or(0),
            )
            .field("finished", &self.finished)
            .finish()
    }
}

/// Start `watn <args>` in a PTY and return the live session. Environment and
/// config are prepared from the world first.
pub(crate) fn start_pty_session(world: &mut crate::WatnWorld, args: &[&str]) -> PtySession {
    let binary = find_binary();
    ensure_test_env(world);

    let pty_system = portable_pty::native_pty_system();
    let pair = pty_system
        .openpty(portable_pty::PtySize {
            rows: 40,
            cols: 120,
            pixel_width: 0,
            pixel_height: 0,
        })
        .expect("open pty");

    let mut cmd = portable_pty::CommandBuilder::new(binary);
    for a in args {
        cmd.arg(a);
    }
    cmd.env_remove("WATN_OPENAI_API_KEY");
    cmd.env_remove("WATN_PROVIDER");
    cmd.env_remove("WATN_MODEL");
    cmd.env_remove("OPENROUTER_API_KEY");
    cmd.env_remove("WATN_API_KEY");
    cmd.env_remove("WATN_TEST_ENDPOINT_OVERRIDE");
    for (key, value) in &world.env_vars {
        cmd.env(key.as_str(), value.as_str());
    }
    cmd.env("TERM", "xterm-256color");

    let child = pair.slave.spawn_command(cmd).expect("spawn pty command");
    drop(pair.slave);

    let mut reader = pair.master.try_clone_reader().expect("pty reader");
    let writer = pair.master.take_writer().expect("pty writer");

    let output_buffer = Arc::new(Mutex::new(Vec::new()));
    let reader_buffer = Arc::clone(&output_buffer);
    let reader_handle = std::thread::spawn(move || {
        let mut tmp = [0u8; 1024];
        loop {
            match reader.read(&mut tmp) {
                Ok(0) => break,
                Ok(n) => reader_buffer.lock().unwrap().extend_from_slice(&tmp[..n]),
                Err(ref e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
                Err(_) => break,
            }
        }
    });

    PtySession {
        child,
        writer: Some(writer),
        output_buffer,
        reader_handle: Some(reader_handle),
        finished: false,
    }
}

/// Write a keystroke sequence into a live PTY session.
pub(crate) fn pty_write(session: &mut PtySession, seq: &str) {
    let w = session.writer.as_mut().expect("pty writer still open");
    w.write_all(seq.as_bytes())
        .expect("write keystrokes to pty");
    w.flush().ok();
}

pub(crate) fn pty_snapshot(session: &PtySession) -> String {
    String::from_utf8_lossy(&session.output_buffer.lock().unwrap()).to_string()
}

pub(crate) fn pty_wait_for_label(session: &PtySession, label: &str) -> String {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(3);
    loop {
        let output = pty_snapshot(session);
        if label.split_whitespace().all(|word| output.contains(word)) {
            return output;
        }
        if std::time::Instant::now() >= deadline {
            panic!("PTY did not render label {label:?}; output: {output:?}");
        }
        std::thread::sleep(std::time::Duration::from_millis(25));
    }
}

pub(crate) fn cleanup_pty_session(mut session: PtySession) {
    let _ = session.child.kill();
    let _ = session.child.wait();
    session.writer.take();
    if let Some(reader_handle) = session.reader_handle.take() {
        let _ = reader_handle.join();
    }
}

/// Wait for the PTY child to exit, collect its output into the world, and
/// return the captured output text.
pub(crate) fn finish_pty_session(world: &mut crate::WatnWorld, mut session: PtySession) -> String {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    let status = loop {
        if let Some(status) = session.child.try_wait().expect("poll pty child") {
            break status;
        }
        if std::time::Instant::now() >= deadline {
            let _ = session.child.kill();
            break session.child.wait().expect("wait for killed pty child");
        }
        std::thread::sleep(std::time::Duration::from_millis(25));
    };
    session.writer.take();
    if let Some(reader_handle) = session.reader_handle.take() {
        let _ = reader_handle.join();
    }
    let buf = session.output_buffer.lock().unwrap().clone();
    let text = String::from_utf8_lossy(&buf).to_string();
    world.exit_status = Some(status.exit_code() as i32);
    world.output = Some(text.clone());
    world.stderr_output = Some(String::new());
    text
}
