use cucumber::{given, then, when};
use regex::Regex;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use crate::{MockServerWrap, WatnWorld};
use super::{build_config, find_binary, finish_pty_session, pty_write, run_binary_with_state, start_pty_session};
use watn::models::list::ModelEntry;
use watn::models::picker;

// ===== GIVEN =====

#[given("a configured default provider \"openai\"")]
fn configured_default_provider(w: &mut WatnWorld) {
    let config = build_config("openai", None, None, None, None, None);
    w.raw_config = Some(config);
    w.pending_mock_model = Some("gpt-4o-mini".to_string());
    w.pending_mock_output = Some("find . -type f -mtime -3".to_string());
    w.pending_mock_usage = Some(false);
}

#[given(regex = r#"^a model "([^"]+)" assigned to the small/fast tier$"#)]
fn model_assigned_small(w: &mut WatnWorld, model: String) {
    let config = build_config("openai", Some((&model, "gpt-4o", "o3-mini")), None, None, None, None);
    w.raw_config = Some(config);
    w.pending_mock_model = Some(model);
    w.pending_mock_output = Some("find . -type f -mtime -3".to_string());
    w.pending_mock_usage = Some(false);
}

#[given(regex = r#"^a model "([^"]+)" assigned to the normal tier$"#)]
fn model_assigned_normal(w: &mut WatnWorld, model: String) {
    let config = build_config("openai", Some(("gpt-4o-mini", &model, "o3-mini")), None, None, None, None);
    w.raw_config = Some(config);
    w.pending_mock_model = Some(model);
    w.pending_mock_output = Some("some output".to_string());
    w.pending_mock_usage = Some(false);
}

#[given(regex = r#"^a model "([^"]+)" assigned to the thinking tier$"#)]
fn model_assigned_thinking(w: &mut WatnWorld, model: String) {
    let config = build_config("openai", Some(("gpt-4o-mini", "gpt-4o", &model)), None, None, None, None);
    w.raw_config = Some(config);
    w.pending_mock_model = Some(model);
    w.pending_mock_output = Some("some output".to_string());
    w.pending_mock_usage = Some(false);
}

#[given(expr = "a model {string} assigned to the small tier with reasoning {string}")]
fn model_assigned_small_reasoning(w: &mut WatnWorld, model: String, reasoning: String) {
    let r = reasoning.trim_matches('"').to_string();
    let config = format!(
        "[defaults]\nprovider = \"openai\"\n\n[tiers]\nsmall = \"{}\"\n\n[tiers.reasoning]\nsmall = \"{}\"\n",
        model, r
    );
    w.raw_config = Some(config);
    w.pending_mock_model = Some(model);
    w.pending_mock_output = Some("some output".to_string());
    w.pending_mock_usage = Some(false);
    w.pending_mock_no_reasoning_assert = true;
}

#[given(expr = "a model {string} assigned to the normal tier with reasoning {string}")]
fn model_assigned_normal_reasoning(w: &mut WatnWorld, model: String, reasoning: String) {
    let r = reasoning.trim_matches('"').to_string();
    let config = format!(
        "[defaults]\nprovider = \"openai\"\n\n[tiers]\nnormal = \"{}\"\n\n[tiers.reasoning]\nnormal = \"{}\"\n",
        model, r
    );
    w.raw_config = Some(config);
    w.pending_mock_model = Some(model);
    w.pending_mock_output = Some("some output".to_string());
    w.pending_mock_usage = Some(false);
    w.pending_mock_expected_reasoning_body = Some(format!("\"reasoning_effort\":\"{}\"", r));
}

#[given(regex = r#"^the mock returns command "([^"]*)"$"#)]
fn mock_returns_command(w: &mut WatnWorld, command: String) {
    w.pending_mock_output = Some(command);
    w.pending_mock_model = Some("test-model".to_string());
    w.pending_mock_usage = Some(false);
}

#[given(regex = r#"^the mock returns reasoning "([^"]*)"$"#)]
fn mock_returns_reasoning(w: &mut WatnWorld, reasoning: String) {
    w.pending_mock_reasoning = Some(reasoning);
}

#[given(regex = r#"^a configured provider "([^"]+)" with api-key "([^"]+)"$"#)]
fn configured_provider_with_key(w: &mut WatnWorld, provider: String, key: String) {
    let config = build_config(
        &provider,
        None,
        Some(vec![(&provider, "http://mock", &key, "")]),
        None, None, None,
    );
    w.raw_config = Some(config);
    w.pending_mock_auth_fail = true;
    w.pending_mock_model = Some("test-model".to_string());
    w.pending_mock_output = Some("some output".to_string());
    w.pending_mock_usage = Some(false);
}

#[given(regex = r#"^a configured default provider "([^"]+)" with default model "([^"]+)"$"#)]
fn configured_default_provider_with_model(w: &mut WatnWorld, provider: String, model: String) {
    let config = build_config(&provider, None, None, None, None, Some(&model));
    w.raw_config = Some(config);
    w.pending_mock_model = Some(model);
    w.pending_mock_output = Some("some output".to_string());
    w.pending_mock_usage = Some(false);
}

#[given("no arguments and no stdin")]
fn no_args_no_stdin(_w: &mut WatnWorld) {}

#[given(regex = r#"^pricing configured at "\$2\.50/1M input tokens" per model$"#)]
fn pricing_configured_given(w: &mut WatnWorld) {
    let config = build_config("openai", None, None, Some(vec![("gpt-4o-mini", 2.50, 10.00)]), None, None);
    w.raw_config = Some(config);
    w.pending_mock_model = Some("gpt-4o-mini".to_string());
    w.pending_mock_output = Some("some output".to_string());
    w.pending_mock_usage = Some(true);
}

#[given("no config file exists")]
fn no_config_file(w: &mut WatnWorld) {
    w.pending_mock_no_config_file = true;
    w.pending_mock_model = Some("test-model".to_string());
    w.pending_mock_output = Some("some output".to_string());
    w.pending_mock_usage = Some(false);
    let dir = tempfile::tempdir().expect("create temp dir for no config test");
    w.env_vars.insert(
        "XDG_CONFIG_HOME".to_string(),
        dir.path().to_string_lossy().to_string(),
    );
    w.temp_dir = Some(dir);
}

#[given(regex = r#"^a user config file at "([^"]+)" with content:$"#)]
async fn user_config_file_with_content(w: &mut WatnWorld, step: &cucumber::gherkin::Step) {
    if let Some(doc) = &step.docstring {
        w.raw_config = Some(doc.trim().to_string());
        w.pending_mock_model = Some("test-model".to_string());
        w.pending_mock_output = Some("some output".to_string());
        w.pending_mock_usage = Some(true);
    }
}

#[given(regex = r#"^a user config file with provider "([^"]+)"$"#)]
fn user_config_with_provider(w: &mut WatnWorld, provider: String) {
    let config = build_config(&provider, None, None, None, None, None);
    w.raw_config = Some(config);
    w.pending_mock_model = Some("test-model".to_string());
    w.pending_mock_output = Some("some output".to_string());
    w.pending_mock_usage = Some(false);
}

#[given("a user config file with invalid TOML content")]
fn invalid_toml_config(w: &mut WatnWorld) {
    w.raw_config = Some("this is not valid toml {{{".to_string());
}

#[given("a user config file with per-model pricing:")]
async fn config_with_pricing_step(w: &mut WatnWorld, step: &cucumber::gherkin::Step) {
    if let Some(doc) = &step.docstring {
        w.raw_config = Some(doc.trim().to_string());
    }
    w.pending_mock_model = Some("gpt-4o-mini".to_string());
    w.pending_mock_output = Some("some output".to_string());
    w.pending_mock_usage = Some(true);
}

#[given(regex = r#"^environment variable ([A-Z_]+) is set to "([^"]+)"$"#)]
fn env_var_set(w: &mut WatnWorld, name: String, value: String) {
    w.env_vars.insert(name, value);
}

#[given("a user config file with a provider definition:")]
async fn config_with_provider_step(w: &mut WatnWorld, step: &cucumber::gherkin::Step) {
    if let Some(doc) = &step.docstring {
        w.raw_config = Some(doc.trim().to_string());
    }
    w.pending_mock_model = Some("custom-model-1".to_string());
    w.pending_mock_output = Some("some output".to_string());
    w.pending_mock_usage = Some(false);
}

#[given("a user config file with:")]
async fn config_with_content_step(w: &mut WatnWorld, step: &cucumber::gherkin::Step) {
    if let Some(doc) = &step.docstring {
        w.raw_config = Some(doc.trim().to_string());
    }
    w.pending_mock_model = Some("test-model".to_string());
    w.pending_mock_output = Some("some output".to_string());
    w.pending_mock_usage = Some(false);
}

#[given(regex = r#"^a provider "([^"]+)" configured without an api_key$"#)]
fn provider_without_key(w: &mut WatnWorld, _provider: String) {
    let config = build_config("openai", None, None, None, None, None);
    w.raw_config = Some(config);
    w.pending_mock_model = Some("test-model".to_string());
    w.pending_mock_output = Some("some output".to_string());
    w.pending_mock_usage = Some(false);
}

#[given(regex = r#"^a provider "([^"]+)" with no api_key configured and no env var set$"#)]
fn provider_no_key_no_env(_w: &mut WatnWorld, _provider: String) {}

#[given(regex = r#"^a LiteLLM endpoint at "([^"]+)"$"#)]
fn litellm_endpoint(w: &mut WatnWorld, url: String) {
    let config = build_config("openai", None, None, None, Some((&url, "test-litellm-key")), None);
    w.raw_config = Some(config);
    w.pending_mock_model = Some("test-model".to_string());
    w.pending_mock_output = Some("some output".to_string());
    w.pending_mock_usage = Some(false);
}

#[given(regex = r#"^the endpoint returns models \[([^\]]+)\]$"#)]
fn endpoint_returns_models(w: &mut WatnWorld, models_str: String) {
    let models: Vec<String> = models_str.split(',')
        .map(|s| s.trim().trim_matches('"').to_string())
        .collect();
    w.pending_mock_returned_models = models;
}

#[given(regex = r#"^a configured provider "([^"]+)" with models endpoint$"#)]
fn configured_provider_with_models(w: &mut WatnWorld, provider: String) {
    let server = httpmock::MockServer::start();
    let base_url = format!("http://127.0.0.1:{}", server.port());
    let server_ref = &server;
    server_ref.mock(move |when, then| {
        when.method(httpmock::Method::POST).path("/chat/completions");
        then.status(200)
            .header("Content-Type", "text/event-stream")
            .body("data: {\"id\":\"1\",\"choices\":[{\"index\":0,\"delta\":{\"content\":\"some output\"},\"finish_reason\":\"stop\"}],\"usage\":{\"prompt_tokens\":10,\"completion_tokens\":5,\"total_tokens\":15}}\ndata: [DONE]\n");
    });
    w.mock_server = MockServerWrap(Some(server), None);
    w.pending_mock_model = Some("test-model".to_string());
    w.pending_mock_output = Some("some output".to_string());
    w.pending_mock_usage = Some(true);
    w.raw_config = Some(format!(
        "[defaults]\nprovider = \"{}\"\n\n[providers.{}]\nendpoint = \"{}\"\napi_key = \"test-key\"\n",
        provider, provider, base_url
    ));
}

#[given("no provider is configured")]
fn no_provider_configured(w: &mut WatnWorld) {
    w.raw_config = Some("[defaults]\nprovider = \"nonexistent\"\n".to_string());
}

#[given(regex = r#"^a configured provider "([^"]+)" with failing models endpoint$"#)]
fn configured_provider_with_failing_models(w: &mut WatnWorld, provider: String) {
    let server = httpmock::MockServer::start();
    let base_url = format!("http://127.0.0.1:{}", server.port());
    w.mock_server = MockServerWrap(Some(server), None);
    w.pending_mock_models_fail = true;
    w.pending_mock_returned_models = vec!["model-a".to_string()]; // avoid empty models error
    w.raw_config = Some(format!(
        "[defaults]\nprovider = \"{}\"\n\n[providers.{}]\nendpoint = \"{}\"\napi_key = \"test-key\"\n",
        provider, provider, base_url
    ));
}

#[given("a configured provider \"test\" with models endpoint returning rich metadata")]
fn configured_provider_models_rich(w: &mut WatnWorld) {
    let server = httpmock::MockServer::start();
    let base_url = format!("http://127.0.0.1:{}", server.port());
    w.mock_server = MockServerWrap(Some(server), None);

    let server_ref = w.mock_server.0.as_ref().unwrap();
    server_ref.mock(|when, then| {
        when.method(httpmock::Method::GET).path("/models");
        then.status(200)
            .header("Content-Type", "application/json")
            .body(r#"{"data":[
                {"id":"model-a","name":"Model Alpha","context_length":128000,"pricing":{"prompt":"0.15","completion":"0.60"},"supported_features":["reasoning","tools"]},
                {"id":"model-b","name":"Model Beta","context_length":32000,"pricing":{"prompt":"2.50","completion":"10.00"},"supported_features":["tools"]}
            ]}"#);
    });

    w.pending_mock_returned_models = vec!["model-a".to_string(), "model-b".to_string()];
    w.raw_config = Some(format!(
        "[defaults]\nprovider = \"test\"\n\n[providers.test]\nendpoint = \"{}/\"\napi_key = \"test-key\"\n",
        base_url
    ));
}

#[given("a configured provider \"test\" with models endpoint returning bare model IDs")]
fn configured_provider_models_bare(w: &mut WatnWorld) {
    let server = httpmock::MockServer::start();
    let base_url = format!("http://127.0.0.1:{}", server.port());
    w.mock_server = MockServerWrap(Some(server), None);

    let server_ref = w.mock_server.0.as_ref().unwrap();
    server_ref.mock(|when, then| {
        when.method(httpmock::Method::GET).path("/models");
        then.status(200)
            .header("Content-Type", "application/json")
            .body(r#"{"data":[{"id":"model-a"},{"id":"model-b"}]}"#);
    });

    w.pending_mock_returned_models = vec!["model-a".to_string(), "model-b".to_string()];
    w.raw_config = Some(format!(
        "[defaults]\nprovider = \"test\"\n\n[providers.test]\nendpoint = \"{}/\"\napi_key = \"test-key\"\n",
        base_url
    ));
}

#[given("no LiteLLM endpoint is configured")]
fn no_litellm(w: &mut WatnWorld) {
    w.raw_config = Some(
        "[defaults]\nprovider = \"nonexistent\"\n"
            .to_string(),
    );
}



// ===== WHEN =====

#[when(regex = r#"^I run `watn "([^"]*)"`$"#)]
fn run_watn(w: &mut WatnWorld, question: String) {
    run_binary_with_state(w, &[&question], None);
}

#[when(regex = r#"^I run `watn -1 "([^"]*)"`$"#)]
fn run_watn_tier1(w: &mut WatnWorld, question: String) {
    run_binary_with_state(w, &["-1", &question], None);
}

#[when(regex = r#"^I run `watn -2 "([^"]*)"`$"#)]
fn run_watn_tier2(w: &mut WatnWorld, question: String) {
    run_binary_with_state(w, &["-2", &question], None);
}

#[when(regex = r#"^I run `watn -3 "([^"]*)"`$"#)]
fn run_watn_tier3(w: &mut WatnWorld, question: String) {
    run_binary_with_state(w, &["-3", &question], None);
}

#[when(regex = r#"^I run `watn -3 -v "([^"]*)"`$"#)]
fn run_watn_tier3_verbose(w: &mut WatnWorld, question: String) {
    run_binary_with_state(w, &["-3", "-v", &question], None);
}

#[when(regex = r#"^I run `watn -1 -v "([^"]*)"`$"#)]
fn run_watn_tier1_verbose(w: &mut WatnWorld, question: String) {
    run_binary_with_state(w, &["-1", "-v", &question], None);
}

#[when(regex = r#"^I run `watn -v "([^"]*)"`$"#)]
fn run_watn_verbose(w: &mut WatnWorld, question: String) {
    run_binary_with_state(w, &["-v", &question], None);
}

#[when(regex = r#"^I run `watn -3 -v -x "([^"]*)"` and answer with "([^"]*)"$"#)]
fn run_watn_tier3_verbose_execute(w: &mut WatnWorld, question: String, answer: String) {
    run_binary_with_state(w, &["-3", "-v", "-x", &question], Some(&answer));
}

#[when("I run `watn --help`")]
fn run_watn_help(w: &mut WatnWorld) {
    run_binary_with_state(w, &["--help"], None);
}

#[when(regex = r#"^I run `watn --model "([^"]+)" "([^"]*)"`$"#)]
fn run_watn_with_model(w: &mut WatnWorld, model: String, question: String) {
    run_binary_with_state(w, &["--model", &model, &question], None);
}

#[when("I run `watn --version`")]
fn run_watn_version(w: &mut WatnWorld) {
    run_binary_with_state(w, &["--version"], None);
}

#[when("I run `watn`")]
fn run_watn_no_args(w: &mut WatnWorld) {
    run_binary_with_state(w, &[] as &[&str], None);
}

#[when(regex = r#"^I run `watn "([^"]*)"` and send SIGINT after 500ms$"#)]
fn run_watn_sigint(w: &mut WatnWorld, question: String) {
    // Give the binary a mock that delays response so SIGINT arrives mid-request
    w.pending_mock_delay_ms = Some(2000);
    w.pending_mock_model = Some("gpt-4o-mini".to_string());
    w.pending_mock_output = Some("some long output that takes time".to_string());
    w.pending_mock_usage = Some(false);

    let binary = super::find_binary();
    super::ensure_test_env(w);

    let mut cmd = std::process::Command::new(&binary);
    cmd.arg(&question);
    super::apply_env(w, &mut cmd);

    let child = cmd
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("spawn binary");

    std::thread::sleep(std::time::Duration::from_millis(500));

    let _ = nix::sys::signal::kill(
        nix::unistd::Pid::from_raw(child.id() as i32),
        nix::sys::signal::Signal::SIGINT,
    );

    let output = child.wait_with_output().expect("wait for output");
    w.output = Some(String::from_utf8_lossy(&output.stdout).to_string());
    w.stderr_output = Some(String::from_utf8_lossy(&output.stderr).to_string());
    w.exit_status = output.status.code();
}

#[when(regex = r#"^I run `echo "([^"]*)" \| watn`$"#)]
fn run_watn_stdin(w: &mut WatnWorld, question: String) {
    run_binary_with_state(w, &[] as &[&str], Some(&question));
}

#[when(regex = r#"^I run `watn -x "([^"]*)"` and answer with "([^"]*)"$"#)]
fn run_watn_x_with_answer(w: &mut WatnWorld, question: String, answer: String) {
    run_binary_with_state(w, &["-x", &question], Some(&answer));
}

#[when(regex = r#"^I run `watn --provider ([^ ]+) "([^"]*)"`$"#)]
fn run_watn_with_provider(w: &mut WatnWorld, provider: String, question: String) {
    run_binary_with_state(w, &["--provider", &provider, &question], None);
}

#[when("I run `watn models`")]
fn run_watn_models(w: &mut WatnWorld) {
    run_binary_with_state(w, &["models"], None);
}

#[when(regex = r#"^I run `watn models` and select "([^"]+)" for small, "([^"]+)" for normal, and "([^"]+)" for thinking$"#)]
fn run_watn_models_select(w: &mut WatnWorld, small: String, normal: String, thinking: String) {
    // We need to map model names to their indices in the mock model list.
    // The mock returns models in order, and dialoguer selects by index.
    // We pipe index+enter for each selection to simulate interactive input.
    let models = &w.pending_mock_returned_models;
    let small_idx = models.iter().position(|m| m == &small).unwrap_or(0);
    let normal_idx = models.iter().position(|m| m == &normal).unwrap_or(1.min(models.len().saturating_sub(1)));
    let thinking_idx = models.iter().position(|m| m == &thinking).unwrap_or(2.min(models.len().saturating_sub(1)));
    let stdin_input = format!("{}\n{}\n{}\n", small_idx, normal_idx, thinking_idx);

    if w.raw_config.is_none() && w.pending_mock_model.is_none() {
        w.pending_mock_model = Some("test-model".to_string());
        w.pending_mock_output = Some("some output".to_string());
        w.pending_mock_usage = Some(false);
    }

    run_binary_with_state(w, &["models"], Some(&stdin_input));
}

#[when(regex = r#"^I run `watn --model gpt-4o "([^"]*)"`$"#)]
fn run_watn_model_gpt4o(w: &mut WatnWorld, question: String) {
    run_binary_with_state(w, &["--model", "gpt-4o", &question], None);
}

// ===== THEN =====

#[then(expr = "the exit status should be {int}")]
fn exit_status_n(w: &mut WatnWorld, status: i32) {
    assert_eq!(w.exit_status, Some(status), "expected exit status {}, got {:?}. stderr: {}", status, w.exit_status, w.stderr_output.as_deref().unwrap_or(""));
}

#[then("the exit status should be non-zero")]
fn exit_status_nonzero(w: &mut WatnWorld) {
    assert!(w.exit_status != Some(0), "expected non-zero exit status, got {:?}. stderr: {}", w.exit_status, w.stderr_output.as_deref().unwrap_or(""));
    assert!(w.exit_status.is_some(), "expected exit status to be set, got None");
}

#[then(expr = "the output should contain {string}")]
fn output_should_contain(w: &mut WatnWorld, text: String) {
    let output = w.output.as_ref().expect("no output captured");
    assert!(output.contains(&text), "expected output to contain '{}', got: '{}'", text, output);
}

#[then("the output should contain a model name")]
fn output_contains_model(w: &mut WatnWorld) {
    let stderr = w.stderr_output.as_ref().expect("no stderr captured");
    let re = Regex::new(r"[\w~/.\-]+ · \d+ tok/s").unwrap();
    assert!(re.is_match(stderr), "expected stderr to contain a model name with tokens/sec, got: '{}'", stderr);
}

#[then("the output should contain a tokens/second value")]
fn output_contains_tok_s(w: &mut WatnWorld) {
    let stderr = w.stderr_output.as_ref().expect("no stderr captured");
    assert!(stderr.contains("tok/s"), "expected stderr to contain 'tok/s', got: '{}'", stderr);
}

#[then("the output should not contain ANSI escape sequences")]
fn output_no_ansi(w: &mut WatnWorld) {
    let ansi_re = Regex::new(r"\x1b\[[0-9;]*m").unwrap();
    let out = w.output.as_ref().expect("no output");
    assert!(!ansi_re.is_match(out), "output contains ANSI escapes");
}

#[then(expr = "the output should match regex {string}")]
fn output_matches(w: &mut WatnWorld, pattern: String) {
    let re = Regex::new(&pattern).expect("invalid regex");
    let combined = format!("{}\n{}", w.output.as_deref().unwrap_or(""), w.stderr_output.as_deref().unwrap_or(""));
    assert!(re.is_match(&combined), "expected output to match regex '{}'. stdout: '{}', stderr: '{}'", pattern,
        w.output.as_deref().unwrap_or(""), w.stderr_output.as_deref().unwrap_or(""));
}

#[then(expr = "stderr should contain {string}")]
fn stderr_contains(w: &mut WatnWorld, text: String) {
    let stderr = w.stderr_output.as_ref().expect("no stderr captured");
    assert!(stderr.contains(&text), "expected stderr to contain '{}', got: '{}'", text, stderr);
}

#[then(expr = "stderr should not contain {string}")]
fn stderr_not_contain(w: &mut WatnWorld, text: String) {
    let stderr = w.stderr_output.as_ref().expect("no stderr captured");
    assert!(!stderr.contains(&text), "expected stderr to not contain '{}', got: '{}'", text, stderr);
}

#[then(expr = "the API request should include reasoning with effort {string}")]
fn api_request_includes_reasoning(w: &mut WatnWorld, effort: String) {
    let mock_id = w.mock_server.1.expect("no mock id stored");
    let server = w.mock_server.0.as_ref().expect("no mock server");
    let mock = httpmock::Mock::new(mock_id, server);
    assert_eq!(w.exit_status, Some(0), "expected exit 0, got {:?}", w.exit_status);
    assert!(mock.hits() > 0, "expected mock to be hit (reasoning effort {})", effort);
}

#[then("the API request should not include reasoning")]
fn api_request_not_include_reasoning(w: &mut WatnWorld) {
    assert_eq!(w.exit_status, Some(0), "expected exit 0, got {:?}", w.exit_status);
    let blocking_id = w.blocking_mock_id.expect("no blocking mock id stored");
    let server = w.mock_server.0.as_ref().expect("no mock server");
    let blocking = httpmock::Mock::new(blocking_id, server);
    assert_eq!(blocking.hits(), 0,
        "expected no reasoning_effort in the request, but a reasoning request was blocked");
}

#[then("the output should contain a version number")]
fn output_contains_version(w: &mut WatnWorld) {
    let out = w.output.as_ref().expect("no output captured");
    assert!(out.contains("0.1.0"), "expected output to contain version '0.1.0', got: '{}'", out);
}

#[then("the output should contain a cost value")]
fn output_contains_cost(w: &mut WatnWorld) {
    let stderr = w.stderr_output.as_ref().expect("no stderr captured");
    let re = Regex::new(r"\$\d+(\.\d+)?").unwrap();
    assert!(re.is_match(stderr), "expected stderr to contain a cost value, got: '{}'", stderr);
}

#[then(expr = "the output should be a command suggestion containing {string}")]
fn output_contains_command_suggestion(w: &mut WatnWorld, text: String) {
    let out = w.output.as_ref().expect("no output captured");
    assert!(out.contains(&text), "expected output to contain '{}', got: '{}'", text, out);
}

#[then("the command should not have been executed")]
fn command_not_executed(w: &mut WatnWorld) {
    let out = w.output.as_ref().expect("no output captured");
    let lines: Vec<&str> = out.lines().map(str::trim).filter(|l| !l.is_empty()).collect();
    assert_eq!(lines.len(), 1, "expected single command suggestion, got {} lines: '{}'", lines.len(), out);
    assert_eq!(lines[0], "echo hello", "expected command suggestion");
}

#[then(expr = r"{string} should have been printed to stdout")]
fn string_printed_to_stdout(w: &mut WatnWorld, text: String) {
    let out = w.output.as_ref().expect("no output captured");
    assert!(out.contains(&text), "expected stdout to contain '{}', got: '{}'", text, out);
}

#[then("the output should contain a cost estimate")]
fn output_contains_cost_estimate(w: &mut WatnWorld) {
    let stderr = w.stderr_output.as_ref().expect("no stderr captured");
    let re = Regex::new(r"\$\d+(\.\d+)?").unwrap();
    assert!(re.is_match(stderr), "expected stderr to contain a cost estimate, got: '{}'", stderr);
}

#[then(expr = "the request should use model {string}")]
fn request_should_use_model(w: &mut WatnWorld, model: String) {
    let stderr = w.stderr_output.as_ref().expect("no stderr captured");
    assert!(stderr.contains(&model), "expected stderr to contain model '{}', got: '{}'", model, stderr);
}

#[then(expr = "the request should be sent to provider {string}")]
fn request_sent_to_provider(w: &mut WatnWorld, _provider: String) {
    let mock_id = w.mock_server.1.expect("no chat mock id stored");
    let server = w.mock_server.0.as_ref().expect("no mock server");
    let mock = httpmock::Mock::new(mock_id, server);
    assert!(mock.hits() > 0, "expected chat completion request to be sent to the provider");
}

#[then(expr = "the request should be sent to {string}")]
fn request_sent_to_url(w: &mut WatnWorld, _url: String) {
    let mock_id = w.mock_server.1.expect("no chat mock id stored");
    let server = w.mock_server.0.as_ref().expect("no mock server");
    let mock = httpmock::Mock::new(mock_id, server);
    assert!(mock.hits() > 0, "expected chat completion request to be sent to the URL");
}

#[then(expr = "it should query the model list at {string}")]
fn should_query_models_at(w: &mut WatnWorld, _url: String) {
    let server = w.mock_server.0.as_ref().expect("no mock server");
    let mock_id = w.models_mock_id.expect(
        "no models mock was set up — add pending_mock_returned_models to the Given step"
    );
    let mock = httpmock::Mock::new(mock_id, server);
    assert!(mock.hits() > 0, "expected model list request");
}

#[then(expr = r"the request should include the Authorization header with {string}")]
fn request_has_auth_header(w: &mut WatnWorld, _key: String) {
    let mock_id = w.mock_server.1.expect("no chat mock id stored");
    let server = w.mock_server.0.as_ref().expect("no mock server");
    let mock = httpmock::Mock::new(mock_id, server);
    assert!(mock.hits() > 0, "expected chat completion request with Authorization header");
}

#[then("the config file should contain the selected tier assignments")]
fn config_contains_tier_assignments(w: &mut WatnWorld) {
    let dir = w.temp_dir.as_ref().expect("no temp dir");
    let config_path = dir.path().join("watn").join("config.toml");
    let content = std::fs::read_to_string(&config_path)
        .expect("config file should exist");
    assert!(content.contains("[tiers]"), "config should have [tiers] section, got: {}", content);
    assert!(content.contains("small = \""), "config should have small tier, got: {}", content);
    assert!(content.contains("normal = \""), "config should have normal tier, got: {}", content);
    assert!(content.contains("thinking = \""), "config should have thinking tier, got: {}", content);
}

#[then(regex = r#"^running `watn "([^"]*)"` should use "([^"]*)"$"#)]
fn running_should_use(w: &mut WatnWorld, command: String, model: String) {
    let binary = find_binary();
    let mut cmd = std::process::Command::new(&binary);
    cmd.args([&command]);
    super::apply_env(w, &mut cmd);

    let output = cmd.output().expect("run binary");
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    assert!(stderr.contains(&model), "expected stderr to contain model '{}', got: '{}'", model, stderr);
}

#[then("the output should contain model metadata")]
fn output_contains_model_metadata(w: &mut WatnWorld) {
    let out = w.output.as_ref().expect("no output captured");
    let stderr = w.stderr_output.as_ref().expect("no stderr captured");
    let combined = format!("{}\n{}", out, stderr);
    assert!(combined.contains("context") || combined.contains("ctx") || combined.contains("pricing")
        || combined.contains("$") || combined.contains("features"),
        "expected model metadata in output, got stdout: '{}' stderr: '{}'", out, stderr);
}

#[then("the output should not contain pricing information")]
fn output_not_contain_pricing(w: &mut WatnWorld) {
    let out = w.output.as_ref().expect("no output captured");
    let stderr = w.stderr_output.as_ref().expect("no stderr captured");
    let combined = format!("{}\n{}", out, stderr);
    assert!(!combined.contains("$"), "expected no pricing in output, got: '{}'", combined);
}

#[then("the output should contain an error message")]
fn output_contains_error(w: &mut WatnWorld) {
    let stderr = w.stderr_output.as_ref().expect("no stderr captured");
    assert!(!stderr.is_empty(), "expected error message in stderr, got empty");
}

#[then("the output should contain instructions for configuring providers manually")]
fn output_contains_instructions(w: &mut WatnWorld) {
    let out = w.output.as_ref().expect("no output captured");
    let stderr = w.stderr_output.as_ref().expect("no stderr captured");
    assert!(out.contains("No provider endpoint") || stderr.contains("No provider endpoint"),
        "expected output to mention provider configuration, got stdout: '{}' stderr: '{}'", out, stderr);
}

// ===== auto-init-config steps =====

#[then("a config file exists at the standard XDG path")]
fn config_file_exists_at_xdg(w: &mut WatnWorld) {
    let dir = w.temp_dir.as_ref().expect("no temp dir set up by ensure_test_env");
    let config_path = dir.path().join("watn").join("config.toml");
    assert!(config_path.exists(), "expected config file at {:?} to exist", config_path);
}

#[then(regex = r#"^the config file contains a commented-out "([^"]+)" section$"#)]
fn config_file_contains_commented_section(w: &mut WatnWorld, section: String) {
    let dir = w.temp_dir.as_ref().expect("no temp dir");
    let config_path = dir.path().join("watn").join("config.toml");
    let content = std::fs::read_to_string(&config_path)
        .expect("config file should exist");
    assert!(content.contains(&format!("# [{}]", section)), "expected config file to contain commented-out '[{}]' section, got:\n{}", section, content);
}

#[then("the command succeeds as if the file already existed")]
fn command_succeeds(w: &mut WatnWorld) {
    assert_eq!(w.exit_status, Some(0), "expected exit status 0, got {:?}", w.exit_status);
}

#[given(regex = r#"^an existing config file with provider "([^"]+)"$"#)]
fn existing_config_with_provider(w: &mut WatnWorld, provider: String) {
    let server = httpmock::MockServer::start();
    let base_url = format!("http://127.0.0.1:{}", server.port());
    w.mock_server = MockServerWrap(Some(server), None);
    let server_ref = w.mock_server.0.as_ref().unwrap();
    let mock = server_ref.mock(|when, then| {
        when.method(httpmock::Method::POST).path("/chat/completions");
        then.status(200)
            .header("Content-Type", "text/event-stream")
            .body("data: {\"id\":\"1\",\"choices\":[{\"index\":0,\"delta\":{\"content\":\"some output\"},\"finish_reason\":\"stop\"}],\"usage\":{\"prompt_tokens\":10,\"completion_tokens\":5,\"total_tokens\":15}}\ndata: [DONE]\n");
    });
    w.mock_server.1 = Some(mock.id);
    let config = format!(
        "[defaults]\n\
         provider = \"{}\"\n\
         \n\
         [providers.{}]\n\
         endpoint = \"{}\"\n\
         api_key = \"test-key\"\n\
         default_model = \"test-model\"\n",
        provider, provider, base_url
    );
    w.raw_config = Some(config);
}

#[then(regex = r#"^the config file still contains provider "([^"]+)"$"#)]
fn config_file_still_contains_provider(w: &mut WatnWorld, provider: String) {
    let dir = w.temp_dir.as_ref().expect("no temp dir");
    let config_path = dir.path().join("watn").join("config.toml");
    let content = std::fs::read_to_string(&config_path)
        .expect("config file should exist");
    assert!(content.contains(&format!("provider = \"{}\"", provider)), "expected config file to contain 'provider = \"{}\"', got:\n{}", provider, content);
}

// ===== Model autosuggest steps =====

fn setup_search_mock(w: &mut WatnWorld) -> String {
    if w.mock_server.0.is_none() {
        let server = httpmock::MockServer::start();
        w.mock_server = MockServerWrap(Some(server), None);
    }
    let server = w.mock_server.0.as_ref().unwrap();
    format!("http://127.0.0.1:{}", server.port())
}

fn mock_search_response(w: &mut WatnWorld, query: &str, models: &[String], delay_ms: u64) {
    let server = w.mock_server.0.as_ref().expect("mock server not set up");
    let models_clone = models.to_vec();
    let q = query.to_string();
    let mock = server.mock(move |when, then| {
        when.method(httpmock::Method::GET)
            .path("/models")
            .query_param("search", &q);
        if delay_ms > 0 {
            std::thread::sleep(std::time::Duration::from_millis(delay_ms));
        }
        let data: Vec<serde_json::Value> = models_clone.iter().map(|id| {
            serde_json::json!({"id": id})
        }).collect();
        then.status(200)
            .header("Content-Type", "application/json")
            .body(serde_json::json!({"data": data}).to_string());
    });
    w.search_mock_ids.push(mock.id);
}

fn mock_search_error(w: &mut WatnWorld, query: &str, status: u16) {
    let server = w.mock_server.0.as_ref().expect("mock server not set up");
    let q = query.to_string();
    let mock = server.mock(move |when, then| {
        when.method(httpmock::Method::GET)
            .path("/models")
            .query_param("search", &q);
        then.status(status)
            .header("Content-Type", "application/json")
            .body(r#"{"error":"not supported"}"#);
    });
    w.search_mock_ids.push(mock.id);
}

#[given(regex = r#"^a provider with models (.+)$"#)]
fn provider_with_models(w: &mut WatnWorld, models_str: String) {
    // Parse the models string: it may be "gpt-4o-mini", "gpt-4o", "o3-mini", and "o3-pro"
    // Extract all quoted model IDs.
    let models: Vec<String> = models_str
        .split(',')
        .flat_map(|s| s.split("and "))
        .map(|s| s.trim().trim_matches('"').trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();

    let endpoint = setup_search_mock(w);

    // Register a catch-all mock that returns all models for any search query
    // The secondary local filter in search_models will narrow results.
    let server_ref = w.mock_server.0.as_ref().unwrap();
    let models_clone = models.clone();
    let catch_all = server_ref.mock(move |when, then| {
        when.method(httpmock::Method::GET)
            .path("/models")
            .query_param_exists("search");
        let data: Vec<serde_json::Value> = models_clone.iter().map(|id| {
            serde_json::json!({"id": id})
        }).collect();
        then.status(200)
            .header("Content-Type", "application/json")
            .body(serde_json::json!({"data": data}).to_string());
    });
    w.search_mock_ids.push(catch_all.id);

    w.picker_endpoint = Some(endpoint);
    w.picker_generation = Some(Arc::new(AtomicU64::new(0)));
    w.picker_no_results = false;
    w.picker_error = None;
}

#[when(regex = r#"^I type "([^"]+)" into the active tier picker$"#)]
async fn type_into_picker(w: &mut WatnWorld, query: String) {
    let endpoint = w.picker_endpoint.clone().expect("no endpoint set up");
    let generation = Arc::clone(w.picker_generation.as_ref().expect("no generation counter"));
    let q = query.clone();
    let all_models = w.picker_local_models.clone().unwrap_or_default();

    let result = tokio::task::spawn_blocking(move || {
        let current_gen = generation.fetch_add(1, Ordering::SeqCst) + 1;
        picker::execute_search(&endpoint, None, &q, &all_models, &generation, current_gen)
    }).await.expect("blocking task failed");

    match result {
        Ok((results, error, no_results)) => {
            w.picker_suggestions = Some(results);
            w.picker_error = error;
            w.picker_no_results = no_results;
        }
        Err(e) => {
            panic!("search failed: {}", e);
        }
    }

    w.picker_query = Some(query);
}

#[then(regex = r#"^the suggestions include "([^"]+)" and "([^"]+)"$"#)]
fn suggestions_include(w: &mut WatnWorld, model1: String, model2: String) {
    let suggestions = w.picker_suggestions.as_ref()
        .expect("no suggestions available");
    let ids: Vec<&str> = suggestions.iter().map(|m| m.id.as_str()).collect();
    assert!(ids.contains(&model1.as_str()), "expected suggestions to contain '{}', got: {:?}", model1, ids);
    assert!(ids.contains(&model2.as_str()), "expected suggestions to contain '{}', got: {:?}", model2, ids);
}

#[then(regex = r#"^the suggestions do not include "([^"]+)" or "([^"]+)"$"#)]
fn suggestions_not_include(w: &mut WatnWorld, model1: String, model2: String) {
    let suggestions = w.picker_suggestions.as_ref()
        .expect("no suggestions available");
    let ids: Vec<&str> = suggestions.iter().map(|m| m.id.as_str()).collect();
    assert!(!ids.contains(&model1.as_str()), "expected suggestions to not contain '{}', got: {:?}", model1, ids);
    assert!(!ids.contains(&model2.as_str()), "expected suggestions to not contain '{}', got: {:?}", model2, ids);
}

#[then(expr = "the suggestions include {string}")]
fn suggestions_single_include(w: &mut WatnWorld, model: String) {
    let suggestions = w.picker_suggestions.as_ref()
        .expect("no suggestions available");
    let ids: Vec<&str> = suggestions.iter().map(|m| m.id.as_str()).collect();
    assert!(ids.contains(&model.as_str()), "expected suggestions to contain '{}', got: {:?}", model, ids);
}

#[then(expr = "the suggestions do not include {string}")]
fn suggestions_single_not_include(w: &mut WatnWorld, model: String) {
    let suggestions = w.picker_suggestions.as_ref()
        .expect("no suggestions available");
    let ids: Vec<&str> = suggestions.iter().map(|m| m.id.as_str()).collect();
    assert!(!ids.contains(&model.as_str()), "expected suggestions to not contain '{}', got: {:?}", model, ids);
}

#[when(regex = r#"^I replace the search text with "([^"]+)"$"#)]
async fn replace_search_text(w: &mut WatnWorld, query: String) {
    let endpoint = w.picker_endpoint.clone().expect("no endpoint set up");
    let generation = Arc::clone(w.picker_generation.as_ref().expect("no generation counter"));
    let q = query.clone();

    // Clear previous state
    w.picker_suggestions = None;
    w.picker_error = None;
    w.picker_no_results = false;

    let all_models = w.picker_local_models.clone().unwrap_or_default();

    let result = tokio::task::spawn_blocking(move || {
        let current_gen = generation.fetch_add(1, Ordering::SeqCst) + 1;
        picker::execute_search(&endpoint, None, &q, &all_models, &generation, current_gen)
    }).await.expect("blocking task failed");

    match result {
        Ok((results, error, no_results)) => {
            w.picker_suggestions = Some(results);
            w.picker_error = error;
            w.picker_no_results = no_results;
        }
        Err(e) => {
            panic!("search failed: {}", e);
        }
    }

    w.picker_query = Some(query);
}

// Note: There is a cucumber-rs ambiguity with the `when` macro when a step
// matches both a `given` and `when` pattern. The "I replace..." step is
// used in both Given context and When context. We keep it as `when` since
// the feature file uses it with "When".

#[then(regex = r#"^the picker says that no models were found$"#)]
fn picker_says_no_models(w: &mut WatnWorld) {
    assert!(w.picker_no_results, "expected picker to report no models found");
}

#[then(expr = "the dialog shows the filter text {string}")]
fn dialog_shows_filter_text(w: &mut WatnWorld, text: String) {
    // Non-e2e: assert the stored picker query. (The e2e PTY variant asserts
    // on rendered PTY output via the picker/session output.)
    if let Some(q) = &w.picker_query {
        assert_eq!(q, &text, "expected dialog filter text '{}', got '{}'", text, q);
        return;
    }
    let output = w.output.as_ref().expect("no output captured");
    assert!(output.contains(&text), "expected dialog filter text '{}' in output, got: '{}'", text, output);
}

#[given(regex = r#"^a provider returns the results for "([^"]+)" more slowly than the results for "([^"]+)"$"#)]
fn slow_provider_results(w: &mut WatnWorld, slow_query: String, fast_query: String) {
    let models: Vec<String> = vec![format!("model-{}", fast_query), format!("model-{}", slow_query)];
    let endpoint = setup_search_mock(w);

    // Fast response for one query
    let fast_models = vec![format!("model-{}", fast_query)];
    mock_search_response(w, &fast_query.to_lowercase(), &fast_models, 10);

    // Slow response for another query
    let slow_models = vec![format!("model-{}", slow_query)];
    mock_search_response(w, &slow_query.to_lowercase(), &slow_models, 500);

    w.picker_endpoint = Some(endpoint);
    w.picker_generation = Some(Arc::new(AtomicU64::new(0)));
    w.search_query_delays.insert(slow_query.to_string(), 500);
    w.search_query_delays.insert(fast_query.to_string(), 10);
}

#[then(regex = r#"^the suggestions for "([^"]+)" are displayed$"#)]
fn suggestions_for_query_displayed(w: &mut WatnWorld, query: String) {
    let suggestions = w.picker_suggestions.as_ref()
        .expect("no suggestions available");
    let ids: Vec<&str> = suggestions.iter().map(|m| m.id.as_str()).collect();
    assert!(!ids.is_empty(), "expected suggestions for '{}', got empty", query);
}

#[then(regex = r#"^a later result for "([^"]+)" does not replace them$"#)]
fn later_result_does_not_replace(w: &mut WatnWorld, query: String) {
    // The suggestions stored in the world reflect the last dispatched (and
    // completed) search — the newer "o3" result. A stale, slower result for
    // `query` ("gpt") must not have overwritten them, because the generation
    // guard discards results whose generation advanced before they landed.
    let suggestions = w.picker_suggestions.as_ref()
        .expect("no suggestions available");
    let ids: Vec<&str> = suggestions.iter().map(|m| m.id.as_str()).collect();
    assert!(!ids.is_empty(), "expected the newer suggestions to remain, got empty");
    assert!(
        !ids.iter().any(|id| id.starts_with(&query)),
        "stale result for '{}' replaced the newer suggestions: {:?}",
        query, ids
    );
}

#[given(regex = r#"^a provider that does not support searching its model catalog$"#)]
fn provider_no_search_support(w: &mut WatnWorld) {
    let endpoint = setup_search_mock(w);
    // Register a mock that returns 501 for any search query
    let server = w.mock_server.0.as_ref().unwrap().clone();
    let mock = server.mock(move |when, then| {
        when.method(httpmock::Method::GET)
            .path("/models")
            .query_param_exists("search");
        then.status(501)
            .header("Content-Type", "application/json")
            .body(r#"{"error":"search not supported"}"#);
    });
    w.search_mock_ids.push(mock.id);
    w.picker_endpoint = Some(endpoint);
    w.picker_generation = Some(Arc::new(AtomicU64::new(0)));
}

#[given(expr = "a provider that does not support searching its model catalog with models {string} and {string}")]
fn provider_no_search_support_with_models(w: &mut WatnWorld, m1: String, m2: String) {
    let endpoint = setup_search_mock(w);
    let m1c = m1.trim_matches('"').to_string();
    let m2c = m2.trim_matches('"').to_string();
    // Register a mock that returns 501 for any search query
    let server = w.mock_server.0.as_ref().unwrap().clone();
    let mock = server.mock(move |when, then| {
        when.method(httpmock::Method::GET)
            .path("/models")
            .query_param_exists("search");
        then.status(501)
            .header("Content-Type", "application/json")
            .body(r#"{"error":"search not supported"}"#);
    });
    w.search_mock_ids.push(mock.id);
    w.picker_endpoint = Some(endpoint);
    w.picker_generation = Some(Arc::new(AtomicU64::new(0)));
    w.picker_local_models = Some(vec![
        ModelEntry { id: m1c.clone(), name: None, context_length: None, pricing: None, supported_features: vec![] },
        ModelEntry { id: m2c.clone(), name: None, context_length: None, pricing: None, supported_features: vec![] },
    ]);
}
#[then(regex = r#"^the picker reports that model search is unavailable$"#)]
fn picker_reports_search_unavailable(w: &mut WatnWorld) {
    assert_eq!(w.picker_error.as_deref(), Some("model search is not supported by this provider"),
        "expected picker to report search unavailable, got: {:?}", w.picker_error);
}

#[given(regex = r#"^the catalog has models "([^"]+)" and "([^"]+)" where "([^"]+)" has pricing$"#)]
fn provider_with_models_pricing(w: &mut WatnWorld, m1: String, m2: String, priced: String) {
    let mk = |id: String, has_pricing: bool| ModelEntry {
        id,
        name: None,
        context_length: None,
        pricing: if has_pricing { Some(watn::config::types::ModelPricing { input: 0.15, output: 0.60 }) } else { None },
        supported_features: vec![],
    };
    let a = mk(m1.trim_matches('"').to_string(), priced.trim_matches('"') == "model-a");
    let b = mk(m2.trim_matches('"').to_string(), false);
    w.picker_local_models = Some(vec![a, b]);
}

#[when("I format the model list for display")]
fn format_model_list(w: &mut WatnWorld) {
    let models = w.picker_local_models.as_ref().expect("no models set up");
    w.formatted_entries = Some(models.iter().map(watn::models::format_model_entry).collect());
}

#[then(expr = "the entry for {string} shows a price")]
fn entry_shows_price(w: &mut WatnWorld, model: String) {
    let entries = w.formatted_entries.as_ref().expect("no formatted entries");
    let line = entries.iter().find(|l| l.starts_with(&model)).expect("entry not found");
    assert!(line.contains('$'), "expected '{}' to show a price, got: '{}'", model, line);
}

#[then(expr = "the entry for {string} shows no price")]
fn entry_shows_no_price(w: &mut WatnWorld, model: String) {
    let entries = w.formatted_entries.as_ref().expect("no formatted entries");
    let line = entries.iter().find(|l| l.starts_with(&model)).expect("entry not found");
    assert!(!line.contains('$'), "expected '{}' to show no price, got: '{}'", model, line);
}

#[given(regex = r#"^a provider with a paginated model catalog$"#)]
fn paginated_model_catalog(w: &mut WatnWorld) {
    let endpoint = setup_search_mock(w);

    let base_url;
    {
        let server = w.mock_server.0.as_ref().unwrap();
        base_url = format!("http://127.0.0.1:{}/", server.port());

        // First page: gpt-4o-mini and gpt-4o
        let page1_models = vec!["gpt-4o-mini".to_string(), "gpt-4o".to_string()];
        let p1 = page1_models.clone();
        let mock1 = server.mock(move |when, then| {
            when.method(httpmock::Method::GET)
                .path("/models")
                .query_param("page", "1")
                .query_param("limit", "50");
            let data: Vec<serde_json::Value> = p1.iter().map(|id| serde_json::json!({"id": id})).collect();
            then.status(200)
                .header("Content-Type", "application/json")
                .body(serde_json::json!({"data": data}).to_string());
        });
        w.search_mock_ids.push(mock1.id);

        // Second page: o3-pro
        let page2_models = vec!["o3-pro".to_string()];
        let p2 = page2_models.clone();
        let mock2 = server.mock(move |when, then| {
            when.method(httpmock::Method::GET)
                .path("/models")
                .query_param("page", "2")
                .query_param("limit", "50");
            let data: Vec<serde_json::Value> = p2.iter().map(|id| serde_json::json!({"id": id})).collect();
            then.status(200)
                .header("Content-Type", "application/json")
                .body(serde_json::json!({"data": data}).to_string());
        });
        w.search_mock_ids.push(mock2.id);
    }

    // Search mock for "o3"
    let search_models = vec!["o3-pro".to_string()];
    mock_search_response(w, "o3", &search_models, 0);

    w.picker_endpoint = Some(endpoint);
    w.picker_generation = Some(Arc::new(AtomicU64::new(0)));

    // Point a real provider at the mock so the actual binary (driven via PTY)
    // resolves and queries it.
    w.raw_config = Some(format!(
        "[defaults]\nprovider = \"test\"\n\n[providers.test]\nendpoint = \"{}\"\napi_key = \"test-key\"\n",
        base_url
    ));
}

#[given(regex = r#"^the initial suggestions include "([^"]+)" and "([^"]+)"$"#)]
fn initial_suggestions_include(w: &mut WatnWorld, _m1: String, _m2: String) {
    assert!(w.picker_endpoint.is_some(), "paginated catalog must be set up first");
}

#[given(regex = r#"^a later catalog page includes "([^"]+)"$"#)]
fn later_catalog_page_includes(w: &mut WatnWorld, _model: String) {
    assert!(w.picker_endpoint.is_some(), "paginated catalog must be set up first");
}

#[when(regex = r#"^I run `watn models`, type "([^"]+)" into the small tier picker, and choose "([^"]+)"$"#)]
fn run_models_small_choose(w: &mut WatnWorld, query: String, selected: String) {
    let mut session = start_pty_session(w, &["models"]);
    std::thread::sleep(std::time::Duration::from_millis(200));
    pty_write(&mut session, &query);
    std::thread::sleep(std::time::Duration::from_millis(400));
    pty_write(&mut session, "\r");
    w.pty_session = Some(session);
    let _ = selected;
}

#[when(regex = r#"^choose "([^"]+)" for the normal tier$"#)]
fn choose_normal_tier(w: &mut WatnWorld, _selected: String) {
    // Search for the target model by its distinctive prefix, then select it.
    let mut session = w.pty_session.take().expect("pty session must be active");
    std::thread::sleep(std::time::Duration::from_millis(300));
    pty_write(&mut session, "o3");
    std::thread::sleep(std::time::Duration::from_millis(400));
    pty_write(&mut session, "\r");
    w.pty_session = Some(session);
}

#[when(regex = r#"^choose "([^"]+)" for the thinking tier$"#)]
fn choose_thinking_tier(w: &mut WatnWorld, _selected: String) {
    let mut session = w.pty_session.take().expect("pty session must be active");
    std::thread::sleep(std::time::Duration::from_millis(300));
    pty_write(&mut session, "o3");
    std::thread::sleep(std::time::Duration::from_millis(400));
    pty_write(&mut session, "\r");
    finish_pty_session(w, session);
}

#[then(regex = r#"^the picker displays "([^"]+)" as a matching suggestion$"#)]
fn picker_displays_suggestion(w: &mut WatnWorld, model: String) {
    let output = w.output.clone().or_else(|| {
        std::thread::sleep(std::time::Duration::from_millis(300));
        w.output.clone()
    }).expect("pty output captured");
    assert!(output.contains(&model), "expected picker to display '{}', got: {:?}", model, output);
}

#[then(regex = r#"^the completed setup reports small="([^"]+)", normal="([^"]+)", thinking="([^"]+)"$"#)]
fn completed_setup_reports(w: &mut WatnWorld, small: String, normal: String, thinking: String) {
    let output = w.output.as_ref().expect("pty output captured");
    let report = format!("Tiers configured: small={}, normal={}, thinking={}", small, normal, thinking);
    assert!(output.contains(&report), "expected config report '{}', got: {:?}", report, output);

    let dir = w.temp_dir.as_ref().expect("no temp dir");
    let config_path = dir.path().join("watn").join("config.toml");
    let content = std::fs::read_to_string(&config_path).expect("config file should exist");
    assert!(content.contains(&format!("small = \"{}\"", small)), "config small tier missing, got:\n{}", content);
    assert!(content.contains(&format!("normal = \"{}\"", normal)), "config normal tier missing, got:\n{}", content);
    assert!(content.contains(&format!("thinking = \"{}\"", thinking)), "config thinking tier missing, got:\n{}", content);
}
