# Design: improve-model-selection-autosuggest

## Technology decisions

| Concern | Choice | Rationale |
|---|---|---|
| Language | Rust (latest stable) | Existing project language |
| Terminal I/O | `console` (explicit dep; already a transitive dep of `dialoguer` 0.11.0, verified in Cargo.lock) | Raw-mode key reading, cursor control, ANSI rendering — avoids bringing in a full TUI framework for a single picker widget |
| HTTP client | `reqwest::blocking` | Already in use; search requests run on a background thread |
| JSON parsing | `serde_json` | Already in use |
| Debounce / stale-result guard | Generation counter (`Arc<AtomicU64>`) per picker session | Each keystroke increments the generation; a worker thread abandons its result if the generation has advanced since it was dispatched |
| Terminal rendering | `console::Term::clear_last_lines()` + manual `write!` | Repaint the suggestion region on each keystroke or result arrival |

## Architecture impact

### New module: `src/models/picker.rs`

The `ModelPicker` owns the raw-terminal autosuggest loop. It replaces
`dialoguer::Select` for the model tier assignment flow.

```
struct ModelPicker {
    term: console::Term,              // raw-mode terminal handle
    all_models: Vec<ModelEntry>,      // default (unfiltered) first-page models
    suggestions: Vec<ModelEntry>,     // currently displayed models
    selected_index: usize,            // cursor position in suggestions
    query: String,                    // live search text
    search_in_flight: bool,           // spinner shown while a request is pending
    generation: Arc<AtomicU64>,       // stale-result guard
    endpoint: String,
    api_key: Option<String>,
}

impl ModelPicker {
    fn run(&mut self, tier: &str) -> ModelEntry  // returns the selected entry
}
```

`run()` enters raw mode, then loops:

1. Render the current state: tier prompt, search query line, spinner (if loading), suggestion list with cursor highlight, status line.
2. Read one key via `console::Term::read_key()`.
3. On printable character: append to `query`, bump generation, spawn search worker thread.
4. On Backspace: remove last char from `query`, bump generation, spawn search.
5. On Up/Down: move `selected_index` within `suggestions`.
6. On Enter: return the currently highlighted `suggestions[selected_index]`.
7. On Escape: clear `query`, restore initial `suggestions`.
8. On Ctrl-C: exit process (same as current behavior).

The search worker thread:
1. Captures the `generation` value at spawn time.
2. Calls `list::search_models(&endpoint, &query, api_key)` — blocking HTTP GET.
3. Before updating `self.suggestions`, checks that `generation` hasn't advanced. If it has, the result is stale and discarded.
4. If endpoint returns 4xx/5xx on a non-empty query, sets an error flag; the render loop displays "Model search is not supported by this provider" and retains the previous suggestions.
5. If endpoint returns empty `data` array, displays "No models found" and keeps the picker active.

### Modified module: `src/models/list.rs`

Two new functions alongside the existing `fetch_models`:

```rust
pub fn search_models(
    endpoint: &str,
    query: &str,
    api_key: Option<&str>,
) -> Result<Vec<ModelEntry>, Error>
```

Sends `GET {endpoint}/models?search={query}`. Parses the same `{ "data": [...] }`
response shape as `fetch_models`. If the response lacks `meta.search` (echo of
the query), still returns whatever models the provider sent — the client
performs a secondary local case-insensitive substring filter on `id` as a
safety net.

```rust
pub fn fetch_models_page(
    endpoint: &str,
    page: u32,
    limit: u32,
    api_key: Option<&str>,
) -> Result<Vec<ModelEntry>, Error>
```

Sends `GET {endpoint}/models?page={page}&limit={limit}` for the initial default
view. Falls back to `fetch_models` if pagination parameters return an error,
so the picker works with providers that only expose the basic `/models`
endpoint.

On a non-2xx response from a search request, `search_models` returns
`Error::ApiError { status, message }`. The picker traps this and surfaces
the unsupported-search message without exiting the tier assignment flow.

### Modified module: `src/models/mod.rs`

`run_models` is restructured:

1. `--set-*` flags: unchanged (direct config write, no picker).
2. Resolve provider and fetch first page via `fetch_models_page` (or
   `fetch_models` as fallback).
3. If TTY: instantiate `ModelPicker` and call `picker.run("small")`,
   `picker.run("normal")`, `picker.run("thinking")` sequentially. Each
   returns the `ModelEntry` the user selected.
4. If not a TTY: the existing `select_model_non_interactive` path remains
   (print list, prompt for index).
5. Save tier assignments to config and print confirmation.

### Step definitions

All new step definitions go in `tests/steps/ask_steps.rs` alongside the
existing ones (cucumber-rs 0.23, verified in Cargo.lock, registers steps
globally — documented constraint from prior changes).

**Given steps:**
- `a provider with a paginated model catalog` — sets up mock: first page
  returns models A, B; second page returns model C.
- `a provider with models "..."` — sets `pending_mock_returned_models`
  (existing field, reused).
- `a provider returns the results for "X" more slowly than the results for "Y"`
  — configures two mock paths with different delays.
- `a provider that does not support searching its model catalog` — mock
  returns 501 for any `/models?search=...` request.

**When steps:**
- `I type "X" into the active tier picker` — PTY-driven: writes keystrokes to
  the subprocess pty.
- `I replace the search text with "X"` — clears previous input, types new text.
- `I clear the search text` — sends Escape or backspace sequence.
- `I choose "X"` — sends Enter.
- `I run \`watn models\`, type "X" into the small tier picker, and choose "Y"`
  — composite e2e step: spawn, type, select, wait for next tier.

**Then steps:**
- `the suggestions include "X" and "Y"` — asserts picker output contains
  those model IDs.
- `the suggestions do not include "X"` — asserts absence.
- `the picker says that no models were found` — asserts "no models found" text.
- `the picker reports that model search is unavailable` — asserts error message.
- `the completed setup reports small="X", normal="Y", thinking="Z"` — reads
  config file from the test XDG path.
- `the small tier is assigned to "X"` — reads config.
- `the picker presents the normal tier` — asserts output contains "normal tier"
  prompt text.

### Test infrastructure changes

The existing `run_binary_with_state` pipes stdin into the subprocess. The
autosuggest picker reads from the terminal in raw mode, not from stdin
buffered lines. **PTY-based test harness:**

- Dev-dependency: `portable-pty` (latest stable, checked crates.io — provides
  cross-platform pseudo-terminal creation).
- New helper in `tests/steps/mod.rs`: `run_binary_pty(world, args, keystrokes:
  &[(u64 /* delay ms */, &str /* key sequence */)])` — spawns the binary in a
  PTY, writes timed keystroke sequences, reads PTY output, and populates
  `world.output`.
- The PTY helper is used only for the `@model-autosuggest` feature. Existing
  scenarios continue using the piped-stdin path.

## Data model

No changes to `config/types.rs`. No new persisted structs. `ModelPicker` state
is runtime-only and not serialized.

`WatnWorld` gains test-only fields:
- `pty_output_buffer: Option<String>` — accumulated PTY output.
- `pending_mock_page_models: HashMap<u32, Vec<String>>` — per-page mock model
  data.
- `pending_mock_search_delay: HashMap<String, u64>` — per-query response delay
  for stale-result testing.

## Runner and strict mode

- **verify.command**: `cargo test --test features_runner -- --tags 'not @wip'`
- **verify.e2e_command**: `cargo test --test features_runner -- --tags '@e2e and not @wip'`
- **Single scenario**: `cargo test --test features_runner -- --name '<scenario title>'`
- **Strict mode**: `.fail_on_skipped()` at `tests/features_runner.rs:132`
  (verified existing). Undefined/pending steps hard-fail the runner. The
  not-implemented stub for Rust is `unimplemented!("...")`. No step body may be
  left empty; the review audit catches this mechanically.

## E2E smoke test infrastructure

- **E2E runner command**: `cargo test --test features_runner -- --tags '@e2e and not @wip'`
- **E2E step location**: `tests/steps/ask_steps.rs` (same file as other steps;
  cucumber-rs global registry constraint).
- **Local test infrastructure**: PTY-based subprocess via `portable-pty`
  (dev-dep, latest stable); `httpmock::MockServer` on loopback for the
  provider/model API. No browser, no database — pure CLI.
- **E2E framework**: cucumber-rs (existing, verified in Cargo.lock).
- **Interface type**: CLI. Driving mechanism: PTY subprocess with timed
  keystroke injection — the subprocess sees a real terminal and the picker
  operates in raw mode exactly as a user would experience it.
- **Strict mode for E2E runner**: same `.fail_on_skipped()` — the e2e runner is
  the same binary filtered by tag, not a separate runner.

## Local runnability and digital twins

- **Local run command**: `cargo run` (single binary, no server, no db).
- **Isolated network**: not applicable — the tool is a single CLI binary with
  no server component. E2E tests bind `httpmock` to `127.0.0.1` per scenario.
- **Digital twins**:

| External dependency | Digital twin |
|---|---|
| Provider model API (`GET /models`, `GET /models?search=...`) | `httpmock::MockServer` — returns configurable model lists, per-page data, and error codes |
| Provider chat API (`POST /chat/completions`) | `httpmock::MockServer` — returns SSE stream responses (existing, unchanged) |

- **Anticipated interface obstacles**:
  - **Raw-mode terminal input not readable via piped stdin**: the existing
    `run_binary_with_state` pipes stdin as a byte stream. The autosuggest
    picker reads individual keys in raw mode via `/dev/tty` (or the platform
    equivalent). PTY-based test harness (`portable-pty`) provides a real
    pseudo-terminal to the subprocess; keystrokes are written to the PTY master
    and the subprocess reads from the PTY slave as a real terminal.
  - **Race between keystrokes and mock response delays**: the PTY test helper
    supports timed keystroke sequences (`(delay_ms, key_sequence)`) so the
    stale-result scenario can interleave typing and mock response timing
    deterministically.

## Interaction coverage matrix

| Inventory entry | @e2e scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| Run `watn models`, type a model search into the active tier picker, and choose a suggestion | Find a model outside the initial page while assigning tiers | CLI | PTY subprocess: spawn `watn models`, write "o3" + Enter for small tier, Enter for normal and thinking selections; httpmock serves paginated model data |

The primary E2E assertion is on the PTY output (what the terminal displayed)
and the config file written by the binary. The httpmock request logs are a
secondary consistency check.

## Verify command

Unit/integration (all non-wip, non-e2e):
```
cargo test --test features_runner -- --tags 'not @e2e and not @wip'
```

E2E smoke tests:
```
cargo test --test features_runner -- --tags '@e2e and not @wip'
```
