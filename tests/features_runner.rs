use std::collections::HashMap;
use std::fmt;

use cucumber::World;

pub mod steps;

pub struct MockServerWrap(pub Option<httpmock::MockServer>);

impl fmt::Debug for MockServerWrap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MockServerWrap").finish()
    }
}

impl Default for MockServerWrap {
    fn default() -> Self {
        Self(None)
    }
}

#[derive(Debug, Default, World)]
pub struct WatnWorld {
    pub env_vars: HashMap<String, String>,
    pub mock_server: MockServerWrap,
    pub temp_dir: Option<tempfile::TempDir>,
    pub config_content: Option<String>,
    pub pending_config: HashMap<String, String>,
    pub raw_config: Option<String>,
    pub output: Option<String>,
    pub stderr_output: Option<String>,
    pub exit_status: Option<i32>,
    pub executed_command: Option<String>,
    pub pending_mock_model: Option<String>,
    pub pending_mock_output: Option<String>,
    pub pending_mock_usage: Option<bool>,
    pub pending_mock_auth_fail: bool,
    pub pending_mock_returned_models: Vec<String>,
    pub stdin_input: Option<String>,
    pub pending_mock_delay_ms: Option<u64>,
}

#[tokio::main]
async fn main() {
    let spec_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("givn")
        .join("specs");

    WatnWorld::cucumber()
        .fail_on_skipped()
        .max_concurrent_scenarios(1)
        .run_and_exit(spec_dir)
        .await;
}
