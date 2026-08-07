# Design: model-explorer

## Technology decisions

- Language: Rust (latest stable) — existing project language.
- HTTP client: `reqwest::blocking` — already in use for chat completions.
- Interactive prompt: `dialoguer` 0.11 — already declared in Cargo.toml.
- JSON parsing: `serde_json` — already in use.
- Config serialization: `toml` 0.8 — already in use.

## Architecture impact

### New module: `src/models/list.rs`

A new file `src/models/list.rs` that fetches model lists from an
OpenAI-compatible `/v1/models` (or `/models`) endpoint. Contains:

```rust
pub struct ModelEntry {
    pub id: String,
    pub name: Option<String>,
    pub context_length: Option<u64>,
    pub pricing: Option<ModelPricing>,
    pub supported_features: Vec<String>,
}

pub fn fetch_models(endpoint: &str, api_key: Option<&str>) -> Result<Vec<ModelEntry>, Error>
```

The `fetch_models` function:
1. Determines the model URL: `{endpoint}/models` (trim trailing `/`).
2. Sends a GET request with optional `Authorization: Bearer {api_key}` header.
3. Parses the JSON response. Tolerates three shapes:
   - **OpenRouter-style**: `{ "data": [{ "id": "...", "name": "...", "context_length": ..., "pricing": { "prompt": "...", "completion": "..." }, "supported_features": [...] }] }`
   - **OpenAI-style**: `{ "data": [{ "id": "...", "created": ..., "owned_by": "..." }] }`
   - **LiteLLM/custom**: whatever it returns — the parser reads `data[].id` and any known fields, silently drops the rest.
4. Returns `Vec<ModelEntry>`.
5. On HTTP error (401, 4xx, 5xx, connection failure), returns `Error::ApiError` or `Error::NetworkError`.

### Modified module: `src/models/mod.rs`

The `run_models` function is rewritten to:

1. **If all three `--set-*` flags are given**: same direct-write behavior as
   today (no API call, no interactive prompt).
2. **Otherwise**: resolve the provider via `resolve_provider()` to get
   endpoint and API key. If no provider is configured, print the existing
   "No provider endpoint configured" message and return.
3. Call `fetch_models(endpoint, api_key)`.
4. If the fetch fails, print the error to stderr and exit non-zero.
5. Extract model IDs from the response. Display each model with its
   metadata (name, context_length, pricing if available) above a separator.
6. Use `dialoguer::Select` to present the model list three times — once for
   each tier (small, normal, thinking). The prompt says "Select a model for
   the {tier} tier:".
7. Write the three selected model IDs to config via `save_config()`.
8. Print confirmation with the selected tier assignments.

### Step definitions

New step definitions for the `@models` feature go in `tests/steps/ask_steps.rs`
alongside the others (cucumber v0.23 requires global step registration).

**Given steps:**
- `a configured provider "test" with models endpoint` — starts a mock server
  and wires it as provider "test" in the config. Also sets up a mock for
  `/models` that returns the configured model list.
- `the endpoint returns models [...]` — already implemented: sets
  `pending_mock_returned_models` (but the mock is not yet wired to `/models`).
- `a configured provider "test" with failing models endpoint` — mock returns
  500 for `/models`.
- `models endpoint returning rich metadata` — mock returns models with
  pricing and name fields.
- `models endpoint returning bare model IDs` — mock returns models with only
  `id` field.
- `no provider is configured` — already exists.

**When steps:**
- `I run \`watn models\` and select "..." for small, "..." for normal, and "..." for thinking`
  — this is interactive. The interactive selection via dialoguer reads from
  stdin (fd 0). The test needs to pass stdin input to the child process.
  dialoguer's `Select` waits for an Enter key after the index selection.
  The input format is: for each selection, the user types the index (as
  digits) followed by Enter. The test provides this via piped stdin.
  The `Select` reads from `/dev/tty` or falls back to stdin when not a tty.
  `dialoguer`'s `Select::interact_on` with `StreamStdin` will read from the
  piped stdin when it is not a tty, which is the case in tests.

**Then steps:**
- `the config file should contain the selected tier assignments` — already
  implemented in `ask_steps.rs`.
- `running \`watn "hello"\` should use "..."` — already implemented.
- `the exit status should be 0` / `the exit status should be non-zero` — already implemented.
- `the output should contain instructions for configuring providers manually` — already implemented.
- `the output should contain an error message` — already exists as generic assertion.
- `the output should contain model metadata` — new: checks stdout/stderr for
  model name/context_length/pricing.
- `the output should not contain pricing information` — new: checks no pricing shown.

### Test infrastructure changes: `tests/features_runner.rs` and `tests/steps/mod.rs`

In `WatnWorld`, add fields:
- `pending_mock_returned_models_rich: Vec<RichModelEntry>` for metadata tests.

In `ensure_test_env`:
- When `pending_mock_returned_models` is non-empty, set up a mock at
  `/models` (GET) that returns the models as an OpenAI-compatible JSON array.

The `run_binary_with_state` function needs a variant that passes stdin input
for the interactive dialoguer selections.

## Data model changes

No new types in `config/types.rs`. The `ModelEntry` struct lives in
`src/models/list.rs` and is not serialized.

## Strict mode

Already configured: `.fail_on_skipped()` in `tests/features_runner.rs:131`.

## E2E smoke test infrastructure

- **E2E runner command**: `cargo test --test features_runner -- --tags '@e2e and not @wip'`.
- **E2E step definition location**: `tests/steps/ask_steps.rs`.
- **Test infrastructure**: `httpmock::MockServer` for mocking the `/models`
  endpoint and `/chat/completions`. No browser, no database — pure CLI.
- **E2E framework**: `cucumber` 0.23 (already in use).
- **Real-interface assertion rule**: Every `@e2e` scenario asserts on CLI
  output (stdout/stderr) or the config file written to disk by the binary.
- **Interface type**: CLI. Driving mechanism: `std::process::Command`
  to spawn the `watn` binary with arguments and piped stdin.

### Interaction coverage matrix

| Inventory entry | @e2e scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| `watn models` interactive | Discover models and select tiers interactively | CLI | `watn models` with piped stdin containing dialoguer selections |
| `watn models` with no provider | Model explorer without provider configured | CLI | `watn models` — check output contains instructions |
| `watn models` with openrouter env | Model explorer with openrouter default and env var set | CLI | `watn models` with piped stdin containing dialoguer selections |
| `watn models` API failure | Model explorer api call fails | CLI | `watn models` — check exit status non-zero, error message |
| `watn models` rich metadata | Model picker shows metadata when available | CLI | `watn models` with piped stdin — check output contains pricing/features |
| `watn models` bare IDs | Model picker shows model IDs when no metadata available | CLI | `watn models` with piped stdin — check output does NOT contain pricing |

## Verify command

Unit/integration:
```
cargo test --test features_runner -- --tags 'not @e2e and not @wip'
```

E2E smoke tests:
```
cargo test --test features_runner -- --tags '@e2e and not @wip'
```
