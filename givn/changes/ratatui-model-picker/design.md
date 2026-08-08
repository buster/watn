# Design: ratatui-model-picker

## Technology decisions

| Concern | Choice | Rationale |
|---|---|---|
| Language | Rust (latest stable) | Existing project language |
| TUI framework | `ratatui` (latest stable, resolved via `cargo add ratatui` at setup; vendored backend `crossterm` is its default feature) | Proposal explicitly requires a keyboard-driven dialog "built with the ratatui crate". Ratatui provides the event loop, `List`/`ListState`, `Layout`, and highlight rendering needed for a guided multi-step model picker. |
| Event backend | `crossterm` (via `ratatui` default features) | Ratatui's bundled backend; `crossterm::event` yields `KeyCode::Up/Down/PageUp/PageDown/Enter` used by the dialog. |
| HTTP client | `reqwest::blocking` | Already in use; search requests run on a worker thread. |
| JSON parsing | `serde_json` | Already in use. |
| Debounce / stale-result guard | Generation counter (`Arc<AtomicU64>`) + tick-rate event loop with 200 ms debounce | Existing stale-result guard carries over; ratatui's `event::poll(timeout)` loop yields a natural debounce window (redraw/resolve ~200 ms after the last key). |
| Config persistence | TOML `config/types.rs` | New per-level reasoning strength is persisted in the config so the choice "sticks" between runs. |
| E2E driver | `portable-pty` (existing dev-dep) | Drives the real binary through a PTY; writes raw escape sequences for arrow/page/enter keys. |

## Architecture impact

### New module: `src/models/dialog.rs`

A ratatui-based dialog replaces the console raw-mode `ModelPicker` for the
interactive TTY flow of `watn models`. It walks the three levels (small,
normal, thinking) in a guided sequence, and for each level lets the user pick
a model **and** a reasoning strength (off, low, medium, high). The user can
return to a previous level and change it before confirming.

```
struct SettingsDialog {
    term_events: crossterm events,       // via ratatui event loop
    endpoint: String,
    api_key: Option<String>,
    all_models: Vec<ModelEntry>,         // default (unfiltered) first-page models
    level: usize,                        // 0=small, 1=normal, 2=thinking
    levels: [LevelChoice; 3],            // per-level model + reasoning selection
    query: String,                       // live filter text
    suggestions: Vec<ModelEntry>,        // currently displayed models
    selected_index: usize,               // cursor within suggestions
    generation: Arc<AtomicU64>,          // stale-result guard
    search_hint: Option<String>,         // e.g. "model search is not supported"
}

struct LevelChoice {
    model: Option<ModelEntry>,
    reasoning: ReasoningStrength,        // Off | Low | Medium | High (default Off)
}
```

`SettingsDialog::run(...)` owns the ratatui event loop:

1. For the current level, render a two-pane dialog: a filter/input line
   always showing the current filter text, the matching model list with the
   selected entry highlighted (ratatui `List` + `ListState`), a reasoning
   control (off/low/medium/high selector), and a status line (empty state or
   unsupported-search notice).
2. Read a key via `crossterm::event::read()`:
   - `Up`/`Down`: move `selected_index` within `suggestions`.
   - `PageUp`/`PageDown`: move `selected_index` by a page (window height).
   - `Char(c)`/`Backspace`: update `query`, debounce (200 ms), run search.
   - `Tab`/`Enter` with focus on reasoning: cycle reasoning strength.
   - `Enter` with focus on model list: accept the highlighted model for the
     current level, advance to the next level.
   - `Esc`: leave the current level back to the previous level (keep its
     selection) when not on the first level.
   - Confirming on the third level ends the dialog and returns the three
     `LevelChoice`s.
3. Filter matching is per-word and order-independent against the model id:
   words are split on whitespace and each word must appear (case-insensitive)
   anywhere in the id, in any order. This replaces the previous whole-query
   substring match.
4. Key contract (deterministic, focus-free):
   - Typed characters / Backspace edit the filter (no separate filter widget
     focus — the model list is the single active area, matching the existing
     autosuggest behaviour).
   - Up/Down move the selection through the list.
   - PageUp/PageDown move the selection by `PAGE_SIZE = 10` rows.
   - Tab cycles the reasoning strength for the current level:
     off → low → medium → high → off.
   - Enter confirms the current level (model = highlighted row, reasoning =
     current strength), advances to the next level; on the thinking level it
     finishes the dialog.
   - Escape returns to the previous level (restoring its saved model +
     reasoning, clearing the filter to show the full list again).
   - Ctrl-C exits the process.
5. Each level starts with a default reasoning strength of "off" when no
   config value is loaded for it.
6. The model list renders each `ModelEntry` via the existing
   `format_model_entry` display logic: id, optional name, context length,
   and pricing when present.

The search worker thread reuses the existing generation-counter pattern from
`picker::execute_search`: capture the generation at dispatch, and discard the
result if the generation advanced before it landed (newest-result-wins).
Remote search failure (endpoint returns 4xx/5xx) falls back to local
per-word matching over `all_models` and surfaces the unsupported-search
notice.

### Modified module: `src/models/picker.rs`

`execute_search`, `local_filter`, and the `EMPTY_QUERY_NOTICE`/error handling
are retained. **The per-word, order-independent predicate is a shared
function and is applied to the remote search results too** (see
`list::search_models` below) — the dialog's typed filters go through the
remote `search_models` path, so per-word matching must apply there, not only
in the local fallback. The single shared predicate:

```rust
pub fn word_matches(id: &str, query: &str) -> bool
```

splits `query` on whitespace and requires every word to be contained
(case-insensitive) in `id`. `local_filter` (fallback) uses it; `search_models`
applies it as a secondary filter over returned rows. This is backward
compatible with all single-word autosuggest scenarios.

The raw-mode `ModelPicker` struct/`run()` is superseded for the interactive
TTY flow by `SettingsDialog`; the non-interactive `select_model` /
`select_model_non_interactive` paths remain for non-TTY input.

### Modified module: `src/models/list.rs`

`search_models` now applies the shared `word_matches` predicate as a secondary
client-side filter over the provider-returned rows (in place of the current
whole-query substring filter) so the remote path honours per-word matching.

### Modified module: `src/models/mod.rs`

`run_models` restructure:

1. `--set-*` flags: unchanged.
2. Resolve provider and fetch first page (existing logic).
3. If TTY: build `LevelChoice`s (defaulting any loaded config per level),
   run the ratatui `SettingsDialog` once, and read back the three
   `LevelChoice`s. This replaces the three sequential `ModelPicker::run`
   calls.
4. If not TTY: existing `select_model` path remains.
5. Persist model ids **and** reasoning strengths per level to config; print
   confirmation.

### Modified module: `src/config/types.rs` / `src/config/mod.rs`

New persisted configuration carrying per-level reasoning strength so the
choice "sticks until you change it":

```rust
pub struct ModelTiers {
    pub small: Option<String>,
    pub normal: Option<String>,
    pub thinking: Option<String>,
    #[serde(default)]
    pub reasoning: TierReasoning,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TierReasoning {
    pub small: Option<String>,     // "off" | "low" | "medium" | "high"
    pub normal: Option<String>,
    pub thinking: Option<String>,
}
```

`TierReasoning::effort(tier) -> Option<String>` maps a tier to the request's
`reasoning_effort` value: `"off"` or absent → `None` (no reasoning); otherwise
`Some(strength)`. When a level has no explicit reasoning configured, the
existing fallback behaviour is preserved (thinking → `"high"`, others →
`None`) so previously-archived reasoning scenarios keep passing.

### Modified module: `src/main.rs`

`reasoning_effort` for a request is now resolved from
`config.tiers.reasoning.effort(tier)` instead of the hard-coded
`if tier_thinking { Some("high") } else { None }`. A level marked "no
reasoning" never sends `reasoning_effort`; a level with a strength sends that
strength.

### Step definitions

Step definitions go in `tests/steps/ask_steps.rs` (cucumber-rs 0.23 global
registry — existing project constraint). New steps:

**Given steps:**
- `a configured provider "X" with models endpoint` — existing.
- `a configured provider "X" with a long model list` — mock returns 40
  deterministic models `model-01` .. `model-40` (fixed `PAGE_SIZE`=10 so
  down once + page down lands on index 11 = `model-12`).
- `the endpoint returns models [...]` — existing.
- `a provider with models [...] where "A" has pricing` — mock with rich
  metadata for one model, bare for another.
- `a provider that does not support searching its model catalog with models "X" and "Y"` — mock returns 501 for any `/models?search=...` and loads the
  given local models into the world for the local-fallback path.
- `a model "X" assigned to the normal tier with reasoning "Y"` — writes a
  config that assigns the model and reasoning strength to a tier.
- `a configured provider "X" with models endpoint` — existing.

**When steps (e2e PTY):**
- `I run \`watn models\` and configure "M" with reasoning "R" for small, "M2" with reasoning "R2" for normal, and "M3" with reasoning "R3" for thinking` — spawn in PTY; per level: type the filter (the parameter is a unique prefix of the target model), Enter to confirm the level (accepts the top suggestion; reasoning defaults off), then press Tab `(off→low=1, medium=2, high=3)` times to set the strength for the next level, and Enter to advance. On the thinking level Enter finishes. Session kept until the last level is confirmed.
- `I run \`watn models\` and configure "M" with reasoning "R" for small` — spawn in PTY, type the filter, Enter to confirm the small level (session kept alive at the normal level).
- `use the down arrow to move the selection to the second model` — PTY: write `\x1b[B` once.
- `use the page down key to move the selection by a full page` — PTY: write `\x1b[6~` (page size fixed at 10), then Enter x3 to confirm all three levels (normal + thinking accept the top suggestion `model-01`).
- `advance to the normal tier and back to the small tier` — PTY: Enter (advance from small) already happened in the prior step; write `\x1b` (Escape) to return to the small level.
- `change the small tier model to "M" with reasoning "R"` — PTY: on small, type the unique prefix, Enter to confirm.
- `configure "M" with reasoning "R" for normal and "M2" with reasoning "R2" for thinking` — PTY: complete the remaining two levels (type filter, Enter, set reasoning with Tab, Enter).
- `I run \`watn models\`, type "X" into the small tier picker, and choose "Y"` — existing composite PTY step (kept for the filter scenario).
- `choose "Y" for the normal tier` / `choose "Y" for the thinking tier` — **new parameterized steps** that type the chosen model id (or a unique prefix) and press Enter; they do NOT reuse the legacy hardcoded "o3" steps.

**When steps (non-e2e logic):**
- `I type "X" into the active tier picker` — drive `list::search_models`/`picker::execute_search` against httpmock, passing the world's local models as `all_models`, store suggestions.
- `I format the model list for display` — run the model-list entry format function over the world's models, store the rendered lines.

**Then steps:**
- `the config file should contain the selected tier assignments with their reasoning strengths` — asserts tiers model ids **and** `[tiers.reasoning]` values.
- `the dialog highlights the selected model` — asserts PTY output shows the highlight marker on the row reached by navigation.
- `the completed setup reports small="X"` / `small="X", normal="Y", thinking="Z"` — existing config-assert step.
- `the dialog shows the filter text "X"` — e2e PTY: asserts rendered filter line; non-e2e: asserts the world's stored filter query (`picker_query`).
- `the picker displays "M" as a matching suggestion` — existing.
- `the picker says that no models were found` — existing.
- `the picker reports that model search is unavailable` — existing.
- `the suggestions include "X"` / `the suggestions do not include "X"` — existing.
- `the entry for "A" shows a price` / `the entry for "B" shows no price` — asserts the rendered list lines contain/omit the pricing string.
- `the API request should include reasoning with effort "Y"` — **changed mechanism**: the Given for the scenario registers the chat mock with an httpmock body matcher `when.body_contains("\"reasoning_effort\":\"Y\"")` (JSON is compact, no spaces). A request that does not carry that effort value fails to match the mock, so the binary receives an error → non-zero exit. The Then asserts `exit status 0` as the primary real-interface proof and `mock.hits() > 0` as a consistency check. This replaces the vacuous hit-count-only assertion inherited from the archived reasoning-support change (httpmock 0.7, verified in Cargo.lock, exposes no client-side request-body capture; the body-matcher approach is the supported mechanism).
- `the API request should not include reasoning` — **changed mechanism**: the Given registers (1) a chat mock with `.body_contains("\"reasoning_effort\":\"")` that returns HTTP 400, registered first, and (2) a fallback chat mock matching path only that returns 200 SSE, registered second. httpmock matches the first mock by id. If the binary sends any `reasoning_effort`, mock (1) is hit → exit non-zero; if it sends none, mock (2) serves the request. The Then asserts `exit status 0` and `mock(1).hits() == 0`.
- `stderr should not contain "reasoning:"` — existing.

## Data model

`Config.tiers` gains a nested `reasoning` table (see above). `WatnWorld`
reuses existing test fields and gains one field for the long-list mock
(`pending_mock_long_models: Vec<String>` — the deterministic 40-entry list).
No `last_request_body` field is used: reasoning-effort assertions use
httpmock request body-matchers (see step definitions), which is the only
body-assertion mechanism httpmock 0.7 supports.

## Runner and strict mode

- **verify.command**: `cargo test --test features_runner -- --tags 'not @wip'`
- **verify.e2e_command**: `cargo test --test features_runner -- --tags '@e2e and not @wip'`
- **Single scenario**: `cargo test --test features_runner -- --name '<scenario title>'`
- **Strict mode**: `.fail_on_skipped()` at `tests/features_runner.rs`
  (verified existing). Undefined/pending steps hard-fail. No step body may be
  left empty; the review audit catches no-op bodies mechanically.

## E2E smoke test infrastructure

- **E2E runner command**: `cargo test --test features_runner -- --tags '@e2e and not @wip'`
- **E2E step location**: `tests/steps/ask_steps.rs` (same file; cucumber-rs
  global registry constraint).
- **Local test infrastructure**: PTY via `portable-pty` (existing dev-dep);
  `httpmock::MockServer` on loopback for the provider model + chat APIs.
- **E2E framework**: cucumber-rs (existing).
- **Interface type**: CLI. Driving mechanism: PTY subprocess with timed
  keystroke injection (raw escape sequences: `\x1b[B` down, `\x1b[A` up,
  `\x1b[6~` page down, `\x1b[5~` page up, `\r` enter, `\x1b` escape). The
  subprocess sees a real terminal and the ratatui dialog operates exactly as
  a user would experience it.
- **Strict mode for E2E runner**: same `.fail_on_skipped()` — same binary,
  tag-filtered.

## Local runnability and digital twins

- **Local run command**: `cargo run` (single CLI binary, no server/db); TTY
  interactive dialog requires a terminal (`cargo run -- models` under a
  terminal, or a PTY).
- **Isolated network**: not applicable — single CLI binary; httpmock binds
  loopback per scenario.
- **Digital twins**:

| External dependency | Digital twin |
|---|---|
| Provider model API (`GET /models`, `GET /models?search=...`) | `httpmock::MockServer` — configurable lists, long lists, rich metadata, error codes |
| Provider chat API (`POST /chat/completions`) | `httpmock::MockServer` — SSE responses (existing) |

- **Anticipated interface obstacles**:
  - **Raw-mode terminal input not readable via piped stdin**: the ratatui
    dialog reads raw keys through crossterm on a real TTY. The existing
    PTY harness (`start_pty_session` / `pty_write` / `finish_pty_session`)
    gives the subprocess a real pseudo-terminal; keystroke escape sequences
    are written to the PTY master.
  - **Escape-sequence mapping**: arrow and page keys must be written as the
    correct crossterm-recognised escape sequences. The harness documents
    `\x1b[A`/`\x1b[B`/`\x1b[5~`/`\x1b[6~` and tests them against a known
    `TERM=xterm-256color`.
  - **Debounce timing in tests**: PTY writes are followed by an explicit
    sleep (the harness's `(delay_ms, seq)` tuples) so the 200 ms debounce
    window can elapse before the next keystroke/assertion.
  - **Guided-level navigation**: Enter/back via Escape are explicitly
    mapped; the e2e steps order keystrokes to walk small → normal → thinking
    and back, matching the dialog's one-active-level model.

## Interaction coverage matrix

| Inventory entry | @e2e scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| Run `watn models`, configure model + reasoning for each of the three levels | Configure model and reasoning for all three levels in the dialog | CLI | PTY: spawn `watn models`, for each level type the filter (unique prefix), Enter, Tab to reasoning, set strength, Tab back, Enter to advance; httpmock serves the model list |
| Browse the model list with arrow/page keys | Browse the model list with arrow keys and page keys | CLI | PTY: `\x1b[B` (down) then `\x1b[6~` (page down) against 40 pinned models; selection lands on `model-12` (PAGE_SIZE=10); assert highlight + config report |
| Type a filter into the dialog | Type a filter and see the matching suggestions | CLI | PTY: type "dee flash", assert matching suggestion + visible filter text |
| Return to a previous level and change it | Return to a previous level and change its selection before confirming | CLI | PTY: advance via Enter, back via Escape, change model + reasoning, finish; assert config |
| Run `watn` so per-level reasoning takes effect | Configured per-level reasoning takes effect on a request | CLI | PTY/HTTP: run `watn -2` against httpmock chat API, assert captured request body `reasoning_effort == "low"` |

Each e2e's primary assertion is on real-interface output (PTY text, chat
request body); config-file assertions are secondary where present. The
metadata display is part of the configure interaction's rendered dialog (not
a distinct interaction); its non-@e2e scenario drives the display formatter
so the pricing presence/absence logic has real coverage without a second
keystroke-level smoke test.

## Verify command

Unit/integration (all non-wip, non-e2e):
```
cargo test --test features_runner -- --tags 'not @e2e and not @wip'
```

E2E smoke tests:
```
cargo test --test features_runner -- --tags '@e2e and not @wip'
```
