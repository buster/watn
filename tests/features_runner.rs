use std::collections::HashMap;
use std::fmt;
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicU64;
use std::sync::Arc;

use watn::models::list::ModelEntry;

use cucumber::gherkin::{Feature, GherkinEnv};
use cucumber::parser::{self, Parser};
use cucumber::runner;
use cucumber::writer;
use cucumber::{Cucumber, World, WriterExt};
use futures::stream;

pub mod steps;

#[derive(Default)]
pub struct MockServerWrap(pub Option<httpmock::MockServer>, pub Option<usize>);

impl fmt::Debug for MockServerWrap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MockServerWrap").finish()
    }
}

#[derive(Debug, Default, World)]
pub struct WatnWorld {
    pub env_vars: HashMap<String, String>,
    pub mock_server: MockServerWrap,
    pub temp_dir: Option<tempfile::TempDir>,
    pub pending_config: HashMap<String, String>,
    pub raw_config: Option<String>,
    pub output: Option<String>,
    pub(crate) pty_session: Option<crate::steps::PtySession>,
    pub stderr_output: Option<String>,
    pub exit_status: Option<i32>,
    pub pending_mock_model: Option<String>,
    pub pending_mock_output: Option<String>,
    pub pending_mock_usage: Option<bool>,
    pub pending_mock_auth_fail: bool,
    pub pending_mock_returned_models: Vec<String>,
    pub pending_mock_delay_ms: Option<u64>,
    pub pending_mock_reasoning: Option<String>,
    pub pending_mock_no_reasoning_assert: bool,
    pub pending_mock_expected_reasoning_body: Option<String>,
    pub blocking_mock_id: Option<usize>,
    pub pending_mock_no_config_file: bool,
    pub pending_mock_models_fail: bool,
    pub shortcut_shells: Vec<String>,
    pub shortcut_targets: HashMap<String, PathBuf>,
    pub shortcut_snapshots: HashMap<String, Vec<u8>>,
    pub shortcut_output: Option<String>,
    pub shortcut_error: Option<String>,
    pub shortcut_status: Option<i32>,
    pub models_mock_id: Option<usize>,
    pub picker_query: Option<String>,
    pub picker_suggestions: Option<Vec<ModelEntry>>,
    pub picker_local_models: Option<Vec<ModelEntry>>,
    pub formatted_entries: Option<Vec<String>>,
    pub picker_error: Option<String>,
    pub picker_no_results: bool,
    pub picker_endpoint: Option<String>,
    pub picker_generation: Option<Arc<AtomicU64>>,
    pub search_mock_ids: Vec<usize>,
    pub search_query_delays: HashMap<String, u64>,
    pub transport: crate::steps::TransportState,
    pub streaming: crate::steps::incremental_sse_rendering_steps::StreamingState,
    pub live_stream: Option<crate::steps::incremental_sse_rendering_e2e_steps::LiveInvocation>,
    pub release_truth: crate::steps::release_truth_steps::ReleaseTruthState,
}

impl Drop for WatnWorld {
    fn drop(&mut self) {
        if let Some(session) = self.pty_session.take() {
            crate::steps::cleanup_pty_session(session);
        }
        for name in self.env_vars.keys() {
            std::env::remove_var(name);
        }
        for name in [
            "OPENROUTER_API_KEY",
            "WATN_API_KEY",
            "WATN_PROVIDER",
            "WATN_MODEL",
            "WATN_TEST_ENDPOINT_OVERRIDE",
        ] {
            std::env::remove_var(name);
        }
    }
}

fn collect_features(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    if !dir.exists() {
        return files;
    }
    let mut stack = vec![dir.to_path_buf()];
    while let Some(path) = stack.pop() {
        if path.is_dir() {
            if path.file_name().and_then(|n| n.to_str()) == Some("archive") {
                continue;
            }
            if let Ok(entries) = std::fs::read_dir(&path) {
                for entry in entries.flatten() {
                    stack.push(entry.path());
                }
            }
        } else if path.extension().is_some_and(|e| e == "feature") {
            files.push(path);
        }
    }
    files
}

#[derive(Clone, Debug)]
struct VecParser;

#[derive(Clone, Debug, Default, clap::Args)]
#[group(skip)]
struct VecParserCli;

impl Parser<Vec<PathBuf>> for VecParser {
    type Cli = VecParserCli;

    type Output = stream::Iter<std::vec::IntoIter<Result<Feature, parser::Error>>>;

    fn parse(self, mut input: Vec<PathBuf>, _cli: Self::Cli) -> Self::Output {
        input.sort();
        let features: Vec<_> = input
            .into_iter()
            .map(|path| {
                let env = GherkinEnv::default();
                match Feature::parse_path(&path, env) {
                    Ok(feature) => Ok(feature),
                    Err(e) => Err(parser::Error::Parsing(Arc::new(e))),
                }
            })
            .collect();
        stream::iter(features)
    }
}

#[tokio::main]
async fn main() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("givn");
    let mut feature_files: Vec<PathBuf> = Vec::new();

    feature_files.extend(collect_features(&root.join("specs")));

    // Skip change-spec files during givn archive verify to avoid duplicate
    // scenarios when delta specs have been merged into the permanent specs.
    if std::env::var("GIVN_ARCHIVE_ONLY").is_err() {
        let changes_dir = root.join("changes");
        if changes_dir.exists() {
            for entry in std::fs::read_dir(&changes_dir).unwrap() {
                let change_dir = entry.unwrap().path();
                feature_files.extend(collect_features(&change_dir.join("specs")));
            }
        }
    }

    feature_files.sort();

    let cucumber_runner = runner::Basic::<WatnWorld>::default();
    let writer = writer::Basic::stdout().normalized().summarized();

    Cucumber::<WatnWorld, VecParser, Vec<PathBuf>, _, _, cucumber::cli::Empty>::custom(
        VecParser,
        cucumber_runner,
        writer,
    )
    .steps(WatnWorld::collection())
    .fail_on_skipped()
    .max_concurrent_scenarios(1)
    .run_and_exit(feature_files)
    .await;
}
