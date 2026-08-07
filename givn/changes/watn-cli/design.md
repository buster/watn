# Design: watn-cli

## Technology decisions

| Concern | Choice | Rationale |
|---|---|---|
| Language | Rust (latest stable) | Greenfield; single-binary distribution; zero-cost streaming |
| Build system | cargo | Standard for Rust. |
| CLI argument parsing | clap v4 | Derive macros; flags `-1/-2/-3` as model tier selector, `-x` as execute flag, `--version` for version output |
| HTTP client | reqwest (blocking) | Blocking fetch thread pipes SSE chunks through `std::sync::mpsc::channel` to main thread for progressive rendering; TLS via native-tls; no tokio dependency |
| Serialization | serde + serde_json | OpenAI API request/response bodies; config file |
| Config file parsing | toml (serde) | XDG convention; idiomatic for Rust |
| Terminal interaction | dialoguer | Interactive model selection via `watn models` (select from list) |
| Token timing | std::time::Instant | Measure elapsed wall-clock time across streaming response; tokens/sec = total tokens / elapsed secs |
| Version output | clap built-in + conditional logo | `--version` prints the Unicode banner (watn-logo.txt) when TERM != "linux" and stdout is a TTY; otherwise prints the ASCII-safe fallback. Version number on the next line. |
| Gherkin test runner | cucumber-rs (latest) | Only mature Rust cucumber implementation |
| HTTP mock | httpmock | Lightweight, programmatic mock server for e2e tests |
| Cost calculation | Config-driven per-model pricing ($/1M input, $/1M output tokens); tracked from API usage response |
| SSE parsing | `eventsource-stream` or `reqwest-eventsource` | Evaluate at implementation time; both parse SSE chunks from a streaming HTTP response |

## Architecture impact

### Module structure

```
src/
  main.rs                  # Binary entrypoint — clap dispatch
  cli.rs                   # CLI argument definitions (clap derive)
  config/
    mod.rs                 # Config load: XDG, toml, layered merge from defaults→system→user→env→cli
    types.rs               # Config structs (ProviderConfig, TierConfig, PricingConfig, LiteLLMConfig)
    env.rs                 # Environment variable reader (WATN_*)
  provider/
    mod.rs                 # Provider trait + registry
    openai_compat.rs       # OpenAI-compatible API client (streaming + non-streaming)
  models/
    mod.rs                 # Model explorer: query LiteLLM /models endpoint, interactive tier assignment
  output/
    mod.rs                 # Output formatting: metadata header + command body, --version banner
    render.rs              # Terminal rendering (metadata: model, tok/s, cost + command output)
    logo.rs                # watn Unicode/ASCII logo for --version output
  exec.rs                  # Execute returned command: print cmd, prompt confirmation, run in shell
  error.rs                 # Error types with exit code mapping
```

### Step definition locations

One file per capability, under `tests/steps/`:

| Capability | Step file |
|---|---|
| ask | `tests/steps/ask_steps.rs` |
| config | `tests/steps/config_steps.rs` |
| models | `tests/steps/models_steps.rs` |
| providers | `tests/steps/providers_steps.rs` |

### E2E step definition locations

One file per capability, under `tests/e2e_steps/`:

| Capability | E2E step file |
|---|---|
| ask | `tests/e2e_steps/ask_steps.rs` |
| config | `tests/e2e_steps/config_steps.rs` |
| models | `tests/e2e_steps/models_steps.rs` |
| providers | `tests/e2e_steps/providers_steps.rs` |

### Step definition conventions

Scenarios that assert on specific mock responses use a `Given` step to configure
the mock before the `When` step. For `-x` scenarios:

```gherkin
Given the mock returns command "echo hello"
When I run `watn -x "echo hello"` and answer with "Enter"
```

This ensures the scenario can fail in RED: without the Given step the mock
returns nothing and the assertion is vacuously true.

### Test runner

`cargo test --test features_runner` — cucumber-rs integration test binary.
Configured as `verify.command` in `givn/config.yaml`.

### Strict-mode config (mandatory)

```rust
// tests/features_runner.rs
Cucumber::<World>::from_feature_files(feature_files)
    .fail_on_skipped()
    .run_and_exit(World::make)
    .await
```

Skipped/unmatched steps fail the suite. The not-implemented stub for Rust is `unimplemented!()`.

### Single-scenario run command

`cargo test --test features_runner -- <feature_file>:<line>` — cucumber-rs
supports filtering scenarios by file and line number.

### Data model / structs

```rust
// ── Config ──────────────────────────────────────────────────────────────────
struct Config {
    defaults: ProviderDefaults,
    providers: HashMap<String, ProviderConfig>,
    tiers: ModelTiers,            // which model per tier
    pricing: HashMap<String, ModelPricing>,
    litellm: Option<LiteLLMConfig>,
}

struct ProviderDefaults {
    provider: String,
    model: Option<String>,        // only used when tiers not configured
}

struct ProviderConfig {
    endpoint: String,
    api_key: Option<String>,
    default_model: Option<String>,
}

struct ModelTiers {
    small: String,
    normal: String,
    thinking: String,
}

struct ModelPricing {
    input: f64,   // $ per 1M input tokens
    output: f64,  // $ per 1M output tokens
}

struct LiteLLMConfig {
    endpoint: String,
    api_key: Option<String>,
}

// ── Provider trait ──────────────────────────────────────────────────────────
trait Provider: Send + Sync {
    fn chat_completions(
        &self,
        messages: &[Message],
        options: RequestOptions,
    ) -> Result<StreamingResponse, Error>;

    fn chat_completions_blocking(
        &self,
        messages: &[Message],
        options: RequestOptions,
    ) -> Result<CompleteResponse, Error>;
}

struct RequestOptions {
    model: String,
    streaming: bool,
    temperature: Option<f32>,
    max_tokens: Option<u32>,
}

// ── Response types ──────────────────────────────────────────────────────────
struct StreamingResponse {
    chunks: Vec<StreamChunk>,       // accumulated during sync block read
    final_usage: Option<TokenUsage>,
    model: String,
    full_content: String,
}

struct StreamChunk {
    content: Option<String>,
    finish_reason: Option<String>,
    usage: Option<TokenUsage>,
}

struct CompleteResponse {
    content: String,
    model: String,
    usage: TokenUsage,
}

struct TokenUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

// ── Error types ─────────────────────────────────────────────────────────────
enum Error {
    ConfigError(String),
    ProviderNotFound(String),
    AuthError(String),
    ApiError { status: u16, message: String },
    NetworkError(String),
    IoError(std::io::Error),
}
```

### Exit code mapping

| Exit code | Meaning | Error variant |
|---|---|---|
| 0 | Success | — |
| 1 | User error (bad args, bad config, unknown provider) | `ConfigError`, `ProviderNotFound` |
| 2 | API error (auth failure, rate limit, server error) | `AuthError`, `ApiError` |
| 3 | Network error (DNS, connection refused, timeout) | `NetworkError` |
| 130 | SIGINT (user cancelled) | — |

## CLI flags

- `-1` (default when omitted): small/fast tier model
- `-2`: normal tier model
- `-3`: thinking/reasoning tier model
- `--model <NAME>`: explicit model override (bypasses tier dispatch)
- `--version`: print logo + version and exit

`-1 | -2 | -3 | --model` implemented as clap mutually-exclusive group.
Default is `-1` when none specified. `--version` is a clap built-in with a
custom `long_version` that renders the logo.

## Execution mode

`-x` flag: after receiving the command output, print the command and prompt
`Execute now? [Y/n]` to stderr. Read a single line from stdin.
- Empty line or `y`/`Y`: execute via `Command::new("sh").arg("-c").arg(cmd)`, inherit stdio.
- `n`/`N`: exit with 0, do not execute.
- Shell execution: `sh -c "<command>"` with inherited stdout/stderr.

## `--version` output

The `--version` flag triggers version printing and immediate exit. The output
begins with the watn logo, then the version number on a new line. Both the
logo and version string go to stdout.

### Logo selection logic

| Condition | Logo |
|---|---|
| `TERM=linux` | ASCII fallback |
| `TERM=dumb` or unset | ASCII fallback |
| stdout is not a TTY (piped) | ASCII fallback |
| Otherwise | Unicode box-drawing banner from `watn-logo.txt` |

### ASCII fallback

```
__      __ __ _ | |_  _ __   ___
\ \ /\ / // _` || __|| '_ \ |__ \
 \ V  V /| (_| || |_ | | | |  / /
  \_/\_/  \__,_| \__||_| |_| |_|
                              (_)
```

### Implementation

```rust
// output/logo.rs
fn logo() -> &'static str {
    if std::env::var("TERM").as_deref() == Ok("linux") || !atty::is(atty::Stream::Stdout) {
        include_str!("../../watn-logo-ascii.txt")  // ASCII fallback stored as file
    } else {
        include_str!("../../watn-logo.txt")         // Unicode banner
    }
}
```

The version number is retrieved via `clap::crate_version!()` and printed on
a new line after the logo. The combined output looks like:

```
██╗    ██╗ █████╗ ████████╗███╗   ██╗██████╗
██║    ██║██╔══██╗╚══██╔══╝████╗  ██║╚════██╗
██║ █╗ ██║███████║   ██║   ██╔██╗ ██║  ▄███╔╝
██║███╗██║██╔══██║   ██║   ██║╚██╗██║  ▀▀══╝
╚███╔███╔╝██║  ██║   ██║   ██║ ╚████║  ██╗
 ╚══╝╚══╝ ╚═╝  ╚═╝   ╚═╝   ╚═╝  ╚═══╝  ╚═╝
0.1.0
```

### One-liner mark in streaming output

During streaming, before any tokens arrive, display the one-liner mark as a
spinner/thinking indicator:

```
watn? ¯\_(ツ)_/¯
```

When the first token arrives, replace the shrug line with the streaming
command text. The metadata line on stderr ends with the one-liner mark:

```
gpt-4o-mini · 142 tok/s · $0.0003 · 0.6s ¯\_(ツ)_/¯
```

## Output format

### Threaded streaming architecture

A blocking reqwest thread performs the HTTP SSE request in a spawned thread and
writes incoming SSE chunks to a `std::sync::mpsc::Sender<StreamChunk>`. The main
thread reads from the corresponding `Receiver` and renders each chunk to stdout
as it arrives. After the stream completes (sender drops), the main thread writes
metadata to stderr and exits.

### Metadata output

After the stream completes, metadata is written to stderr so pipes get clean
command text only:

stdout:
```text
find . -type f -mtime -3
```

stderr:
```text
model: gpt-4o-mini
tokens/s: 42.5
cost: $0.0002
```

When stdout is not a TTY: no ANSI codes (raw text only).

## Model explorer

`watn models` subcommand:
1. If `litellm.endpoint` is configured: GET `{endpoint}/models`, parse model IDs.
   Display as interactive selection list via `dialoguer`.
   Three sequential prompts: select small/fast model, select normal, select thinking.
   On completion, write `[tiers]` section to user config file.
2. If no LiteLLM endpoint: print message with instructions for manual config.

### Non-interactive flags for testing

`--set-small MODEL`, `--set-normal MODEL`, `--set-thinking MODEL` flags allow
assigning all three tiers in a single non-interactive invocation. E2E tests use
these instead of piping through dialoguer:

```
watn models \
  --set-small gpt-4o-mini \
  --set-normal gpt-4o \
  --set-thinking o3-mini
```

When all three `--set-*` flags are present, no interactive prompts are shown and
the config file is written immediately.

## E2E smoke test infrastructure

### E2E runner command

```
cargo test --test features_runner -- --tags @e2e
```

Configured as `verify.e2e_command` in `givn/config.yaml`.

### E2E step locations

Separate files under `tests/e2e_steps/`, one per capability. E2E scenarios
drive the real binary via `std::process::Command`; non-e2e scenarios use
in-process mock provider.

### Local test infrastructure

E2E tests compile the binary (`cargo build`), then invoke it as a subprocess.
`httpmock` runs on `127.0.0.1` for the OpenAI-compatible API and (for model
explorer tests) the LiteLLM endpoint. All subprocess invocations use
`Command::new(binary)` with args/flags, env overrides, and piped stdio as
needed.

### Digital twin per external dependency

| External dependency | Digital twin |
|---|---|
| OpenAI-compatible API | `httpmock` — returns SSE chunks for streaming, JSON for non-streaming, error codes for error scenarios |
| LiteLLM endpoint | `httpmock` — returns model list JSON for model discovery |

### Named fix for anticipated interface obstacles

**Interactive model selection:** `watn models` uses `dialoguer` which reads
from stdin. E2e tests bypass interaction via `--set-small`, `--set-normal`,
`--set-thinking` flags (non-interactive mode).

**Execution confirmation prompt:** `-x` reads one line from stdin after the
response is complete. E2e tests stub this by piping `y`, `n`, or empty line
into the subprocess.

**Subprocess timing for streaming:** E2e tests for token speed/cost display
use a mock that returns a fixed SSE response with known token count. The step
awaits process exit, parses the metadata, and asserts on the computed values.

### Interaction Coverage Matrix

All E2E scenarios use `httpmock` (fake OpenAI endpoint) as the backend and
`Command::new(binary)` as the subprocess driver.

| Interaction Inventory Entry | `@e2e` Scenario | Interface | Driving mechanism |
|---|---|---|---|
| Ask default tier | `ask.feature:22` | CLI | httpmock + `Command::new(binary).arg("question")` |
| Explicit tier -1 | `ask.feature:33` | CLI | httpmock + `Command::new(binary).args(["-1", "question"])` |
| Tier -2 | `ask.feature:41` | CLI | httpmock + `Command::new(binary).args(["-2", "question"])` |
| Tier -3 | `ask.feature:49` | CLI | httpmock + `Command::new(binary).args(["-3", "question"])` |
| Execute with Enter confirmation | `ask.feature:57` | CLI | httpmock + `Command::new(binary).args(["-x", "cmd"]).stdin(Stdio::piped())` + write `\n` |
| Execute with "y" confirmation | `ask.feature:63` | CLI | httpmock + Same + write `y\n` |
| Execute declined "n" | `ask.feature:69` | CLI | httpmock + Same + write `n\n` |
| Cost display when priced | `ask.feature:75` | CLI | httpmock + write config with pricing; run; parse output for `cost:` |
| Tokens/second display | `ask.feature:81` | CLI | httpmock + mock returning known token count; assert `tokens/s` in output |
| Stdin pipe | `ask.feature:86` | CLI | httpmock + `Command::new(binary).stdin(Stdio::piped())` + write question to stdin |
| Configure model tiers in config | `config.feature:19` | CLI | httpmock + write config with tiers; run `-3` variant; verify mock received thinking model |
| Env var override | `config.feature:36` | CLI | httpmock + write config + set `WATN_PROVIDER`; run; verify mock |
| CLI flag override | `config.feature:43` | CLI | httpmock + set env + `--model` flag; verify mock received CLI value |
| Pricing in config | `config.feature:49` | CLI | httpmock + write pricing config; run; parse output for cost |
| Discover models via LiteLLM | `models.feature:14` | CLI | httpmock at LiteLLM endpoint + `--set-*` flags (non-interactive); verify config written |
| No LiteLLM configured | `models.feature:22` | CLI | httpmock + run `watn models` without litellm config; assert instruction message |
| Custom provider from config | `providers.feature:16` | CLI | httpmock + write config with custom endpoint; run; verify mock received request |
| LiteLLM endpoint config | `providers.feature:28` | CLI | httpmock + write litellm config; `watn models` calls the configured endpoint |
| API key from env var | `providers.feature:39` | CLI | httpmock + set `WATN_OPENAI_API_KEY`; verify Authorization header at mock |

### Real-interface assertion rule

Every `@e2e` scenario's primary assertion is on what the subprocess produced
(stdout, stderr, exit code). Side-effect assertions (config file written,
mock received specific request) are secondary only.
