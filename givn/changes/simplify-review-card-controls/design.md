# Design: simplify-review-card-controls

## Use-Case Traceability

| Contract | Design decision | Evidence |
|---|---|---|
| The command flow is the card's subject | The card opens with the first stage selected; Left/Right/Up/Down move stages with no focus region | Opens-on-flow and stage-navigation scenarios |
| Every candidate requires explicit final acceptance | Enter and `a` accept; no other key releases a candidate | Enter-accepts, accept-shortcut, and cancel scenarios |
| Review decisions are explicit, small, and direct | `a`/`e`/`r`/`c` shortcuts plus `d` disable and Escape cancel; no Actions region, no focus strip | Shortcut-hint and reject scenarios |
| Rejection returns to the active intent | `r` opens the model chooser; no candidate is released and the intent is unchanged; leaving the chooser keeps the candidate | Reject and leave-chooser scenarios |
| Refinement continues until acceptance | Choosing a tier or a catalog-suggested model regenerates a fresh candidate for the same intent | Tier-choice, catalog-suggestion, and typed-model scenarios |
| Generation failure preserves review state | A failed regeneration keeps the previous candidate and shows the failure in the card | Failed-regeneration scenario |
| Direct command editing preserves the intent and never evaluates | The editor is cursor-aware; Home/End/Left/Right navigate, Backspace/Delete remove at the insertion point | Editor-navigation and deletion scenarios |
| One candidate per review | Candidate retention and comparison are removed; the card shows one candidate whose tier and model context is visible | Removed retained-candidates scenarios; removed `Candidates` region |
| Explanation-only card changes nothing | Explain-only mode keeps `esc close`; `a`/`e`/`r`/`d` are ignored there | Explanation-ignores-decisions scenario |

Actor stays the terminal developer; the confirmed Persona
`terminal-developer--interactive` is the review lens, not a scenario actor.

## Panel State

`src/review/panel.rs` is the single state machine. The redesign:

- `FocusRegion`, `PanelAction`, `candidates`, `candidate_contexts`,
  `selected_candidate`, `candidate_cursor`, and `action_cursor` are removed.
  `ReviewPanelState` holds one `candidate: ReviewCandidate`.
- `PanelInputMode` gains `ModelChooser` next to `Review` and `CommandEditor`.
- `PanelOutcome` drops `Rejected` and gains:
  - `RejectRequested` — the card asks the application to open the model chooser.
  - `RegenerateWith { tier: String, model: String }` — the chooser produced a
    choice; the application regenerates.
- Review-mode key map:

  | Key | Outcome |
  |---|---|
  | `a`/`A`, Enter | `Accepted(candidate)` |
  | `e`/`E` | open `CommandEditor` |
  | `r`/`R` | `RejectRequested` |
  | `c`/`C`, Escape | `Cancelled` |
  | `d`/`D` | `DisableReviewPermanently` |
  | Left/Right/Up/Down | move `flow_stage` |
  | Tab | ignored (no region to cycle) |

- In explain-only mode the same keys are ignored except Escape and Enter,
  which close the card.
- `open_model_chooser(tiers, catalog)` fills `ModelChooser`:
  `tiers: Vec<TierChoice { tier, label, model }>` from the configuration,
  `catalog: Vec<String>`, `catalog_loading: bool`, a `query` string with its
  own insertion point, a filtered suggestion list, and a highlight.
- Chooser keys: `1`/`2`/`3` choose the tier at that position (unconfigured or
  out-of-range numbers are ignored); Left/Right/Home/End/Backspace/Delete and
  characters edit the query like the command editor; Up/Down move the
  suggestion highlight; Enter chooses the highlighted suggestion, or the typed
  query when it is non-empty, and is ignored otherwise; Escape closes the
  chooser and preserves the candidate.
- `open_model_selection(models)` and `select_model(...)` remain as the
  catalog-only entry points used by the higher-tier escalation flow; they now
  populate the same chooser state.
- `set_catalog(models)` fills the suggestions after an asynchronous fetch;
  results carry a request id so a stale fetch cannot replace newer state.
- Regeneration failure is carried by `regeneration_error` and rendered by the
  card; any successful regeneration clears it.

## Command Editor

`editor_buffer` gains `editor_cursor: usize` (a character index). Shared
text-editing helpers insert, remove, and move by character:

- Left/Right move one character; Home/End jump to the start/end; movement
  clamps at both boundaries.
- Backspace removes the character before the insertion point; Delete removes
  the character at it; both are no-ops at the boundaries.
- Typing inserts at the insertion point; commit and discard keep their current
  semantics (Enter commits, Escape restores the original command).

`src/review/card.rs` renders the caret at the insertion point instead of only
at the end, and renders the chooser rows: configured tiers with their number
keys, the model query with caret, matching suggestions with the highlight, a
loading row while the catalog fetch is in flight, and the regeneration error
when present. In color mode the accept hint is painted green; the accept hint
and the removal of the focus strip are what the emphasis scenario asserts.

## Generation Session

The current generation block in `src/main.rs` moves into the library so unit
scenarios drive production code, not harness emulation:

- New `src/review/session.rs`:
  - `pub struct Generation { pub response: StreamingResponse, pub buffer: ReviewBuffer }`
  - `pub fn generate_candidate(provider: &dyn Provider, messages: Vec<Message>,
    options: RequestOptions, interrupt: Arc<AtomicBool>, spinner_label: &str)
    -> Result<Generation, Error>` — owns the existing spinner, sink, and
    worker-thread logic. The thread is scoped inside the function, so the
    borrowed `&dyn Provider` from `ProviderRegistry::get` crosses no `'static`
    boundary and the registry stays owned by `run()`.
  - `pub fn parse_generated_candidate(generation: &Generation) -> Option<ReviewCandidate>`
    — calls `buffer.complete()` and `candidate_from_provider_response`.
  - `pub fn chooser_tiers(config: &Config) -> Vec<TierChoice>` — one entry per
    configured small/normal/thinking model, in that order.
  - `pub fn fetch_catalog(endpoint: &str, catalog_endpoint: Option<&str>,
    api_key: Option<&str>) -> Vec<String>` — uses the provider-local
    `catalog_endpoint` when set, otherwise the chat endpoint, and returns model
    ids; errors are swallowed to an empty list.
- Unit steps for the reject, tier, typed-model, and failure scenarios build an
  `OpenAICompatibleProvider` against the world's in-process `httpmock` twin and
  call `generate_candidate`/`parse_generated_candidate`, so RED failures come
  from production code.

## Application Loop

`src/main.rs` currently generates once and enters `run_review_path`, which
never returns. It is restructured so the card can regenerate:

- `run()` calls `session::generate_candidate` for the first response instead of
  the inline block; the registry, provider name, endpoint, API key, and
  interrupt stay in `run()`.
- `run_review_path` receives a runtime bundle (`&ProviderRegistry`, provider
  endpoint and catalog endpoint, API key, `&Arc<AtomicBool>`, color,
  verbosity) plus the first `Generation`. Its loop keeps the current response,
  buffer, options, and context as mutable state.
- `RejectRequested`: build the tier choices, call
  `panel.open_model_chooser(tiers, Vec::new())`, and spawn a catalog fetch
  worker with a fresh request id. The card loop switches from blocking
  `event::read()` to `event::poll(50ms)`: key events are handled as they
  arrive, and a completed fetch is applied through the receiver channel when
  its request id is current. The chooser is usable while suggestions load.
- `RegenerateWith { tier, model }`: set `options.model`, derive
  `reasoning_effort` from the chosen tier, generate, parse the candidate,
  update the context's tier/model, replace the candidate, and render. Keys are
  not processed while the blocking generation runs; a second reject cannot
  overlap the first. On error, keep the candidate and set `regeneration_error`
  from the provider error.
- Acceptance prints metadata for the last successful response and keeps the
  existing `-x` execution and stdout contract.

## Interfaces

- `src/review/panel.rs`: state machine, chooser, editor cursor.
- `src/review/session.rs`: generation, parse, tier choices, catalog fetch.
- `src/review/card.rs`: hints, chooser rows, caret, error row.
- `src/review/mod.rs`: re-exports for the new types.
- `src/main.rs`: session calls, chooser outcome handling, catalog worker.
- `src/config/types.rs`: read-only; tiers, reasoning effort, and
  `catalog_endpoint` already exist.

## Test Runner

- Unit/integration: `./run-tests.sh` (Gherkin runner
  `cargo test --locked --test features_runner --features test-support`).
- Single scenario: `./run-tests.sh --name '<scenario title>'`.
- E2E: `./run-tests.sh --e2e`.
- Strict mode: `.fail_on_skipped()` on the cucumber `Cucumber` builder in
  `tests/features_runner.rs`; not-implemented step stubs use `unimplemented!()`.
- Step definitions: one file per capability —
  `tests/steps/interactive_shell_shortcut_steps.rs` (unit/integration) and
  `tests/steps/interactive_shell_shortcut_e2e_steps.rs` (E2E).
- Shared step migration required by the refactor:
  - Delete or rewrite every step touching `FocusRegion`, `Candidates`,
    `Actions`, `PanelAction`, `Rejected`, `retain_current`, `select_candidate`,
    and `candidates` (the removed-scenario steps and the RETAINED
    "Highest-tier review opens explicit provider catalog model selection" and
    "Interrupting an in-progress review operation preserves the selected
    candidate" steps, which must keep passing).
  - Replace literal `"Accept"` assertions and needles with the new hint label
    (`a accept`), including `review_shows_framed_card`, the only-panel,
    mono-card, and direct-review leak assertions.
  - Update `assert_review_rendered_contains` callers that assert `Focus`.
- No new dependencies; all versions resolve from `Cargo.lock` (cucumber 0.23,
  crossterm 0.29, httpmock 0.8, reqwest 0.13 — repo-pinned).

## E2E Infrastructure

- Interface: CLI in a real subprocess PTY (`portable-pty` harness).
- Driving mechanism: write key bytes to the PTY and wait for card labels; the
  direct paths redirect stdout to a file and the primary assertion reads that
  file (no shell buffer is involved on the direct path).
- Provider twin: in-process `httpmock` server configured through the isolated
  `XDG_CONFIG_HOME`; no external network.
- New harness state:
  - `pending_tiers: Option<(String, String, String)>` consumed by
    `ensure_test_env`/`build_config`, so the reject E2E runs with distinct
    small/normal/thinking models.
  - `pending_mock_replacements: Vec<(String, String)>` (model, SSE body)
    registered before the generic chat mock in every `ensure_test_env`
    registration block, so a request whose body contains the replacement model
    is routed to it. `httpmock` matches first-registered-first, and
    `body_includes` already exists.
- Interface obstacle and fix: the stable wait label changes because the action
  row is removed. `pty_wait_for_label` word-splits, so `"a accept"` is matched
  by the contended substring `"a"`; the E2E steps therefore wait for
  contiguous replacement-specific labels (`du -sh .`, the chosen model id)
  before accepting, and the accept/cancel steps wait for the exact hint
  fragment `"accept ·"`.
- E2E strict proof: undefined steps fail because the runner uses
  `.fail_on_skipped()` and `--e2e` selects the same builder; the setup task
  proves it by running one `@wip` scenario and observing a non-zero exit.

## Interaction Coverage Matrix

| Inventory entry | @e2e scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| validate generated shell configuration | Generated Bash, Zsh, and Fish configurations pass shell syntax checks | CLI (generated shell files) | Real shell `bash -n` / `zsh -n` / `fish -n` syntax checks on generated files |
| inspect generated Bash widget | The generated Bash widget keeps the request visible and does not evaluate the command | CLI in Bash PTY | Real Bash process sources the widget, writes `READLINE_LINE`, reads the resulting buffer |
| use Fish Ctrl-W shortcut | Fish replaces the buffer with the generated command after Ctrl-W | CLI in Fish PTY | Real Fish process invokes the installed widget and prints the buffer |
| review and accept a generated candidate from Ctrl-W | Developer accepts an explained candidate from Ctrl-W | CLI in Bash PTY | PTY writes the accept key; reads the resulting `READLINE_LINE` and history |
| cancel a candidate review from Ctrl-W | Developer cancels a review without changing the shell buffer | CLI in Bash PTY | PTY writes Escape; reads the unchanged `READLINE_LINE` and history |
| review and accept a direct interactive request | Developer accepts a candidate from an interactive terminal request | CLI subprocess in PTY | PTY writes the accept key; reads the subprocess stdout |
| review and execute an accepted eligible -x candidate | Developer accepts an eligible -x candidate and it executes once | CLI subprocess in PTY | PTY writes the accept key; reads the execution output |
| reject a candidate and regenerate with another model | Developer rejects a candidate and regenerates with another model | CLI subprocess in PTY | PTY writes `r`, waits for the chooser, writes `2`, waits for the replacement candidate, accepts, reads stdout |
| generate Bash completions | Built Bash completion generation emits the current command tree | CLI subprocess | Real `watn completions bash` invocation and output assertions |

## Local Runnability And Digital Twins

- Local run command: `cargo run --release -- --help` (declared in
  `givn/commands.yaml` as a CLI product).
- The product talks only to user-configured OpenAI-compatible providers; the
  tests replace them with an in-process `httpmock` twin served on loopback in
  the isolated configuration home. No external service is required, and no
  live third-party endpoint is contacted.

## Failure Outcomes

| Condition | Outcome |
|---|---|
| Catalog fetch fails, is slow, or returns nothing | Chooser opens immediately with tiers and the text field; suggests nothing; typed names still work |
| Chosen regeneration succeeds | New candidate replaces the old one; error cleared; acceptance still required |
| Chosen regeneration fails | Previous candidate stays; the card shows the failure; acceptance still required |
| Chooser left with Escape | Previous candidate and review state unchanged |
| Invalid number key, or Enter with no highlight and empty query | Key ignored; chooser stays open |
| Backspace at start, Delete at end, arrows at a boundary | No-op; buffer unchanged |
| Second reject while a regeneration runs | Impossible: keys are not read during the blocking generation |
| Explain-only card receives `a`/`e`/`r`/`d` | Ignored; only Escape/Enter close the card |

## ADR Qualification And Routing

```json
{
  "qualification": "NOT_QUALIFIED",
  "alternatives": "PASS",
  "architectural_impact": "PASS",
  "durable_consequence": "FAIL",
  "lower_level_artifact": "FAIL",
  "existing_adr_check": "PASS",
  "must_be_shared": "SUPPORTING",
  "routing": "CANONICAL_ARTIFACT",
  "canonical_artifact": "givn/changes/simplify-review-card-controls/design.md",
  "target_adr": null,
  "replacement_adr": null,
  "evidence": {
    "alternatives": [
      "Keep the three focus regions, or drive every decision with direct keys; keep one candidate, or keep comparison history; fetch the catalog synchronously, or let the chooser work while suggestions load."
    ],
    "architectural_impact": [
      "The review state machine, the generation helper, and the application review loop change shape; provider, configuration, and presentation boundaries are reused."
    ],
    "durable_consequence": [],
    "lower_level_artifact": [],
    "existing_adr_check": [
      "ADR-0015 owns stream completion; regeneration reuses the same streaming path and does not change the completion contract."
    ]
  }
}
```

## Glossary Updates

- `Review decision`: unchanged meaning; decisions are now direct keys.
- `Review surface`: drop "and its candidates"; one candidate is reviewed.
- `Review outcome`: the typed result now includes `RejectRequested` and
  `RegenerateWith`; drop references that no longer apply.
- `Review history`: prior-Intent state only; candidate retention is removed.
- New `Model chooser`: the in-card list of configured tiers and
  provider-catalog suggestions used to regenerate a rejected candidate.
  Anti-terms: not the SetupWizard Model picker, not the model table or tier
  tabs.

## Architecture Impact

Affected Arc42 chapters, independently re-derived from the selection table:

- 3 (context and scope): the Developer row gains the in-card model chooser.
- 4 (solution strategy): review decisions and candidate replacement.
- 5 (building block view): review state machine, renderer rows that still say
  focus regions/focus tabs/actions.
- 6 (runtime view): "Review and accept" sequence and the Review Candidate
  lifecycle (compare/retain and rejection wording).
- 8 (crosscutting concepts): "Review decisions are explicit"; tier-based
  regeneration and provider-local catalog lookup.
- 10 (quality requirements): QS-067 and QS-070 mention focus regions, actions,
  and the Flow/Candidates/Actions cycle; update them and the matrix rows.
- 11 (risks): add regeneration-failure and catalog-fetch risks near R-068.
- 12 (glossary): the updates above.
- 1, 2, 7, 9 unchanged; chapter 9 keeps no new ADR.

## Verification Contract

The existing Cucumber runner remains the executable specification. Scenarios
now drive the simplified key map, the reject-to-regenerate flow, and the
cursor-aware editor. Removed scenarios are declared with `@givn.removed`.
`./run-tests.sh` and `./run-tests.sh --e2e` remain the verify commands.
