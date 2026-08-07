pub mod ask_steps;
pub mod config_steps;
pub mod models_steps;
pub mod providers_steps;

use std::path::PathBuf;

use httpmock::{Method, MockServer};

use crate::MockServerWrap;
use std::io::Write;

fn find_binary() -> PathBuf {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest_dir.join("target").join("debug").join("watn")
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
            lines.push(format!("\"{}\" = {{ input = {}, output = {} }}", model, input, output));
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

pub(crate) fn run_binary_with_state(
    world: &mut crate::WatnWorld,
    args: &[&str],
    stdin_text: Option<&str>,
) {
    let binary = find_binary();

    let mut config_content = String::new();
    let mut has_config = false;

    if world.pending_mock_model.is_some() || world.pending_mock_output.is_some() {
        let model = world.pending_mock_model.clone().unwrap_or_else(|| "test-model".to_string());
        let output = world.pending_mock_output.clone().unwrap_or_else(|| "output".to_string());
        let include_usage = world.pending_mock_usage.unwrap_or(false);
        let auth_fail = world.pending_mock_auth_fail;

        let server = MockServer::start();
        let base_url = format!("http://127.0.0.1:{}", server.port());
        world.mock_server = MockServerWrap(Some(server));

        let server_ref = world.mock_server.0.as_ref().unwrap();

        if auth_fail {
            server_ref.mock(move |when, then| {
                when.method(Method::POST).path("/chat/completions");
                then.status(401).body(r#"{"error":"unauthorized"}"#);
            });
            config_content = build_config(
                "test",
                None,
                Some(vec![("test", &base_url, "test-key", &model)]),
                None, None, None,
            );
            has_config = true;
        } else {
            let mc = output.clone();
            let include_usage_val = include_usage;
            server_ref.mock(move |when, then| {
                when.method(Method::POST).path("/chat/completions");
                then.status(200)
                    .header("Content-Type", "text/event-stream")
                    .body(format!(
                        "data: {{\"id\":\"1\",\"choices\":[{{\"index\":0,\"delta\":{{\"content\":\"{}\"}},\"finish_reason\":\"stop\"}}]{}}}\ndata: [DONE]\n",
                        mc.replace('"', "\\\""),
                        if include_usage_val { ",\"usage\":{\"prompt_tokens\":10,\"completion_tokens\":20,\"total_tokens\":30}" } else { "" }
                    ));
            });

            // Merge raw_config with mock provider config
            let raw = world.raw_config.clone().unwrap_or_default();
            // Keep everything from raw except its [defaults] section
            let mut lines: Vec<&str> = raw.lines().collect();
            // Remove [defaults] section from raw (mock provides its own)
            if let Some(defaults_idx) = lines.iter().position(|l| l.trim() == "[defaults]") {
                let mut end = defaults_idx + 1;
                while end < lines.len() && !lines[end].starts_with('[') && !lines[end].trim().is_empty() {
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
                config_content = format!("{}\n\n{}", mock_cfg, non_default);
            } else {
                config_content = mock_cfg;
            }
            has_config = true;
        }
    } else if let Some(ref raw) = world.raw_config {
        config_content = raw.clone();
        has_config = true;
    }

    // If no config at all, create a default mock so the binary has something to talk to
    if !has_config && world.mock_server.0.is_none() {
        let server = MockServer::start();
        let base_url = format!("http://127.0.0.1:{}", server.port());
        world.mock_server = MockServerWrap(Some(server));
        let server_ref = world.mock_server.0.as_ref().unwrap();
        server_ref.mock(move |when, then| {
            when.method(Method::POST).path("/chat/completions");
            then.status(200)
                .header("Content-Type", "text/event-stream")
                .body("data: {\"id\":\"1\",\"choices\":[{\"index\":0,\"delta\":{\"content\":\"output\"},\"finish_reason\":\"stop\"}],\"usage\":{\"prompt_tokens\":10,\"completion_tokens\":5,\"total_tokens\":15}}\ndata: [DONE]\n");
        });

        config_content = build_config(
            "test",
            None,
            Some(vec![("test", &base_url, "test-key", "test-model")]),
            None, None, None,
        );
        has_config = true;
    }

    if has_config {
        let dir = tempfile::tempdir().expect("create temp dir");
        let config_dir = dir.path().join("watn");
        std::fs::create_dir_all(&config_dir).expect("create config subdir");
        let config_path = config_dir.join("config.toml");
        std::fs::write(&config_path, &config_content).expect("write config");
        world.env_vars.insert(
            "XDG_CONFIG_HOME".to_string(),
            dir.path().to_string_lossy().to_string(),
        );
        world.temp_dir = Some(dir);
    }

    // Run binary
    let mut cmd = std::process::Command::new(&binary);
    cmd.args(args);

    // Clear environment variables that could interfere with tests
    cmd.env_remove("WATN_OPENAI_API_KEY");
    cmd.env_remove("WATN_PROVIDER");
    cmd.env_remove("WATN_MODEL");

    for (key, value) in &world.env_vars {
        cmd.env(key, value);
    }

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
