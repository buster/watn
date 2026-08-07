# Design: Reasoning Support

## Technology Decisions

- **Language:** Rust (unchanged, edition 2021)
- **CLI framework:** `clap` 4 (unchanged)
- **HTTP client:** `reqwest` 0.12 blocking (unchanged)
- **JSON:** `serde_json` 1 (unchanged)
- **Test framework:** `cucumber` 0.23, `httpmock` 0.7 (unchanged)
- **Streaming model:** The non-streaming `chat_completions_blocking` is not changed. Reasoning support only matters for the streaming path (the one users actually use). Both paths are modified for symmetry, but the non-streaming path is unused in production.
- **Reasoning extraction regex:** None needed — we parse the `delta` object from SSE chunks and extract `delta["reasoning"]` the same way we extract `delta["content"]`.

## Architecture Impact

**Modules affected:**

| Module | Change |
|---|---|
| `src/provider/mod.rs` | Add `reasoning_effort: Option<String>` and `verbose: bool` to `RequestOptions` |
| `src/provider/openai_compat.rs` | Add `reasoning` key to request body when effort is set; read `delta["reasoning"]` from response chunks; accumulate reasoning content separately |
| `src/main.rs` | Add `-v`/`--verbose` CLI arg; pass `verbose` into `RequestOptions`; after streaming completes, print reasoning to stderr if verbose |
| `src/provider/mod.rs` | Add `reasoning_content: Option<String>` to `StreamingResponse` |

**New fields on existing types:**

- `RequestOptions.reasoning_effort: Option<String>` — `None` for tiers 1/2, `Some("high")` for tier 3
- `RequestOptions.verbose: bool` — reflects the `-v`/`--verbose` CLI flag
- `StreamingResponse.reasoning_content: Option<String>` — accumulated reasoning text from `delta["reasoning"]`

**No new modules or files.** This is a surgical change.

**Test infrastructure additions:**

- `WatnWorld.pending_mock_reasoning: Option<String>` — when set, the mock SSE response interleaves `"reasoning"` field in the delta alongside `"content"`
- `WatnWorld.last_request_body: Option<serde_json::Value>` — captures the last received request body from the mock server for request-assertion steps

## Data Model Changes

```rust
// src/provider/mod.rs

#[derive(Debug, Clone)]
pub struct RequestOptions {
    pub model: String,
    pub streaming: bool,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
    pub reasoning_effort: Option<String>,  // NEW: None | Some("high") — sent as top-level "reasoning_effort" in request body
    pub verbose: bool,                     // NEW
}

#[derive(Debug, Clone)]
pub struct StreamingResponse {
    pub chunks: Vec<StreamChunk>,
    pub final_usage: Option<TokenUsage>,
    pub model: String,
    pub full_content: String,
    pub elapsed_secs: f64,
    pub reasoning_content: Option<String>, // NEW
}
```

No change to `CompleteResponse` or `StreamChunk`.

## Request Body Change (openai_compat.rs)

In `chat_completions_streaming`, conditionally add `reasoning_effort` as a top-level string to the JSON body:

```rust
let mut body = serde_json::json!({
    "model": options.model,
    "messages": ...,
    "stream": true,
    "temperature": options.temperature.unwrap_or(0.7),
    "max_tokens": options.max_tokens.unwrap_or(1024),
});

if let Some(effort) = &options.reasoning_effort {
    body["reasoning_effort"] = serde_json::json!(effort);
}
```

Same pattern applied to `chat_completions_blocking` for consistency (though currently unused).

## Response Parsing Change

In the SSE line loop, after extracting `delta["content"]`:

```rust
if let Some(reasoning) = delta["reasoning"].as_str() {
    reasoning_content.push_str(reasoning);
}
```

And store in `StreamingResponse { reasoning_content: Some(reasoning_content), .. }`.

## CLI Flag Change (main.rs)

Add to `Cli` struct:

```rust
#[arg(short = 'v', long = "verbose")]
verbose: bool,
```

## Main Logic Change (main.rs)

At the call site where `RequestOptions` is constructed, compute `reasoning_effort` based on tier and pass `cli.verbose`:

```rust
let reasoning_effort = if cli.tier_thinking {
    Some("high".to_string())
} else {
    None
};

let options = RequestOptions {
    model: model.clone(),
    streaming: true,
    temperature: None,
    max_tokens: None,
    reasoning_effort,
    verbose: cli.verbose,
};
```

After receiving the response, if `verbose` is true and `response.reasoning_content` is `Some`:

```rust
if cli.verbose {
    if let Some(ref reasoning) = response.reasoning_content {
        if !reasoning.trim().is_empty() {
            eprintln!("reasoning: {}", reasoning.trim());
        }
    }
}
```

The reasoning line goes to stderr alongside the existing metadata lines (model, tok/s, cost).

## Step Definition Locations

All new step definitions go in the existing `tests/steps/ask_steps.rs` file under the capability `ask` (since these scenarios live in `specs/reasoning.feature` which imports ask/reasoning steps).

**New step definitions needed:**

1. `the mock returns reasoning "{text}"` — sets `WatnWorld.pending_mock_reasoning` so the mock SSE response includes `"reasoning"` field in delta alongside `"content"`
2. `the API request should include reasoning with effort "{effort}"` — reads `WatnWorld.last_request_body["reasoning_effort"]` to verify the top-level reasoning parameter
3. `stderr should not contain "{text}"` — negated variant of existing `stderr should contain`

**File:** `tests/steps/ask_steps.rs`

**WatnWorld fields added:**

```rust
pub pending_mock_reasoning: Option<String>,
pub last_request_body: Option<serde_json::Value>,
```

## Test Runner Command

```
cargo test --test features_runner
```

## Strict-Mode Config

The runner (`tests/features_runner.rs:50`) already calls `.fail_on_skipped()` on the Cucumber builder. New step definitions that are not yet implemented must use `unimplemented!()` as their body. This causes the test to panic rather than skip, which is caught by `.fail_on_skipped()`.

However, since `unimplemented!()` panics, scenarios tagged `@wip` should not be run. The CI pipeline must exclude `@wip` scenarios, or the developer runs them individually after implementing steps.

Actual practice in this project: steps with `unimplemented!()` will fail when run. The pattern is to tag new scenarios `@wip` and remove `@wip` once step definitions are implemented. The CI (`cargo test --test features_runner`) will fail on skipped scenarios via `.fail_on_skipped()`, but `@wip` scenarios are run and will panic on `unimplemented!()`. The intended workflow: implement steps, then remove `@wip`.

## Single-Scenario Run Command

To run a single scenario by name:

```
cargo test --test features_runner -- --name "Thinking tier sends reasoning without printing it"
```

## E2E Smoke Test Infrastructure

E2E scenarios use `httpmock` (already in dev-dependencies). The mock server captures the incoming request body so we can assert on it. Key infrastructure:

- `ensure_test_env()` in `tests/steps/mod.rs` sets up the mock server and config
- `run_binary_with_state()` in `tests/steps/mod.rs` invokes the compiled binary

For reasoning-specific E2E tests, the mock SSE response must include `"reasoning"` in the delta. Example body:

```
data: {"id":"1","choices":[{"index":0,"delta":{"content":"find ","reasoning":"We need to use find"}}]}
data: {"id":"1","choices":[{"index":0,"delta":{"content":".","reasoning":""}}]}
data: [DONE]
```

The mock must also expose the captured request for assertion. A new `pending_mock_captured_request` field (or reusing the httpmock `MockServer`'s hits) allows the "the API request should include reasoning" step definition to verify the request body.

Implementation detail: httpmock's `Mock::expect` with a custom `when` matcher is not great for inspecting the body post-facto. Better approach: store the last received request in `WatnWorld` by creating a dedicated handler. However, for simplicity we can use `httpmock`'s `server.hits()` or a manual capture approach. A cleaner design: add `pending_mock_assert_fn: Option<Box<dyn FnOnce(&serde_json::Value)>>` to `WatnWorld`, or simply test this at the integration layer by verifying the binary's behavior (model selection, reasoning output) rather than HTTP-level assertion. 

**Decision:** The E2E scenarios tagged `@e2e` that assert on the reasoning parameter being sent will use a two-layer approach: (1) verify the mock server captures the body via a dedicated step that checks the last request, OR (2) skip the HTTP-level assertion and instead verify observable behavior (reasoning printed to stderr, correct model selected). Option 2 is simpler and more faithful to Gherkin's black-box principle. The "API request should include reasoning" step can be implemented by storing the last request body from the httpmock server in `WatnWorld`.

**Revised:** Add `last_request_body: Option<serde_json::Value>` to `WatnWorld`. In `ensure_test_env`, configure the mock to record the request body. The step "the API request should include reasoning with effort {string}" reads `last_request_body["reasoning_effort"]` as a top-level string.

## Local Runnability & Digital Twins

All tests run locally. No external API calls are made. The mock server simulates the OpenRouter API behavior.

The `-v` flag is safe to use in production — if the model does not return reasoning tokens, nothing is printed. If the model does return reasoning tokens but verbose is not set, they are accumulated into `StreamingResponse.reasoning_content` but not printed.

## Interaction Coverage Matrix

| CLI invocation | Reasoning param sent? | Reasoning printed? | Notes |
|---|---|---|---|
| `watn "q"` | No | No | Default small tier |
| `watn -1 "q"` | No | No | Explicit small tier |
| `watn -2 "q"` | No | No | Normal tier |
| `watn -3 "q"` | Yes (high) | No | Thinking tier |
| `watn -v "q"` | No | Only if response has it | Default tier + verbose |
| `watn -1 -v "q"` | No | Only if response has it | Small tier + verbose |
| `watn -2 -v "q"` | No | Only if response has it | Normal tier + verbose |
| `watn -3 -v "q"` | Yes (high) | Yes | Thinking + verbose |
| `watn --model M "q"` | No | No | Explicit model override |
| `watn --model M -v "q"` | No | Only if response has it | Explicit model + verbose |
| `watn -3 -v -x "q"` | Yes (high) | Yes | All flags combined |
| `watn models` | No | No | Subcommand, unchanged |
| `watn --help` | No | No | Shows new flag |

## Justification of Technical Choices

1. **Why `reasoning_effort: Option<String>` instead of a boolean?** OpenRouter's API supports different effort levels (`high`, `medium`, `low`). Using `Option<String>` with `Some("high")` makes it straightforward to support other levels in the future without a breaking change.

2. **Why not add a config option for reasoning effort?** Out of scope. The effort level is always "high" when the thinking tier is active. Users who want no reasoning can use tiers 1/2.

3. **Why print to stderr instead of stdout?** The command suggestion on stdout is consumed by piping (`watn "..." | xargs`). Reasoning is diagnostic metadata, not the command itself. Existing metadata (model, tok/s) already goes to stderr.

4. **Why not extract the "reasoning" field from the non-streaming response too?** The non-streaming path (`chat_completions_blocking`) is currently unused. It gets the same reasoning-parameter addition for consistency, but we don't extract reasoning content from it since it's dead code.

5. **Why accumulate reasoning content separately from content?** Reasoning tokens and content tokens are interleaved in the SSE stream. They must be accumulated separately so the command output (all content tokens) stays clean on stdout.
