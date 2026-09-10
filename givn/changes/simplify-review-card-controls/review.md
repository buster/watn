# Review: simplify-review-card-controls

## Fabrication audit

| # | Check | Result |
|---:|---|---|
| 0 | `@e2e` tag integrity | PASS — the only `@e2e` scenario was added with `@e2e` in `7ca2fcf` and never lost the tag; `git log -S "@e2e"` shows exactly the adding commit |
| 1 | Empty/no-op step bodies | PASS — 0 `unimplemented!()`/`todo!()` bodies remain across the two capability step files |
| 2 | Checked tasks have commits touching production | PASS — sixteen scenario commits, listed under Verification runs; the six binding-only scenarios are documented reuse of production paths introduced earlier in the change |
| 3 | design.md components exist | PASS — `src/review/session.rs` (`Generation`, `generate_candidate`, `parse_generated_candidate`, `chooser_tiers`, `fetch_catalog`), `ModelChooser`/`TierChoice`, chooser and error rows in `card.rs`, main reject/catalog/regenerate wiring |
| 4 | Strict-mode proof present | PASS — tasks.md setup records the targeted run at exit 1 with the stub panic |
| 5 | E2E Then steps assert the real interface | PASS — the reject E2E asserts PTY snapshot content (replacement candidate, chosen model) and the command-output channel (`normal command output should contain only "du -sh ."`) |
| 6 | Browser-UI driver check | N/A — CLI capability |
| 7 | `verify.e2e_command` binding and duplicate implementations | PASS — `./run-tests.sh --e2e` selects `@e2e` through the runner tag filter; one E2E implementation in `tests/steps/interactive_shell_shortcut_e2e_steps.rs` |
| 8 | E2E action normalization | PASS — one new inventory action (reject and regenerate) with exactly one `@e2e`; no excess |
| 9 | `verify.command` vs `verify.e2e_command` | PASS — distinct strings; 209 regular scenarios vs 82 e2e scenarios in the instrumented runs |
| 10 | Implementation vs design.md | PASS — session extraction, scoped worker, async catalog worker with request id, `event::poll` loop, chooser rows, caret editing, explain-only gate, and two capability step files match design.md; the E2E waits for the replacement-specific purpose label and asserts `du -sh .` and the chosen model id, as the design required |
| 11 | Interaction coverage cross-reference | PASS — table below |
| 12 | Coverage measurement validity | PASS — see Coverage classification |
| 13 | Local run command and twins | PASS — `cargo run --release -- --help`; the provider twin is the in-process `httpmock` server, no external service |
| 14 | README impact | `README-IMPACT: none` — the README does not document the card's key bindings; CLI flags and configuration are unchanged by this change |

### Interaction coverage cross-reference

| Inventory entry (`usecase.md`) | Design matrix row | `@e2e` scenario | Driving mechanism found |
|---|---|---|---|
| validate generated shell configuration | yes | Generated Bash, Zsh, and Fish configurations pass shell syntax checks | real `bash -n`/`zsh -n`/`fish -n` on generated files |
| inspect generated Bash widget | yes | The generated Bash widget keeps the request visible and does not evaluate the command | real Bash PTY, `READLINE_LINE` round-trip |
| use Fish Ctrl-W shortcut | yes | Fish replaces the buffer with the generated command after Ctrl-W | real Fish PTY widget invocation |
| review and accept a generated candidate from Ctrl-W | yes | Developer accepts an explained candidate from Ctrl-W | real Bash PTY; accept key; buffer/history assertions |
| cancel a candidate review from Ctrl-W | yes | Developer cancels a review without changing the shell buffer | real Bash PTY; Escape; unchanged buffer/history |
| review and accept a direct interactive request | yes | Developer accepts a candidate from an interactive terminal request | real `watn` subprocess in PTY; stdout file |
| review and execute an accepted eligible -x candidate | yes | Developer accepts an eligible -x candidate and it executes once | real `watn -x` subprocess in PTY; execution output |
| reject a candidate and regenerate with another model | yes | Developer rejects a candidate and regenerates with another model | real `watn` subprocess in PTY; `r`, `2`, replacement label, accept, stdout file |
| generate Bash completions | yes | Built Bash completion generation emits the current command tree | real `watn completions bash` subprocess |

## Arc42 implementation conformance

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| Developer context and review boundary | `docs/arc42/03-context-and-scope.md` | Affected | Context rows | Scenario bindings | Decision keys and model chooser render | Yes |
| Flow-first strategy and reject/regenerate | `docs/arc42/04-solution-strategy.md` | Affected | Solution strategy | Scenarios 1-12 | Panel key map; session regeneration | Yes |
| Review surface and renderer responsibilities | `docs/arc42/05-building-block-view.md` | Affected | Interfaces | Scenarios 1-14 | `panel.rs`, `card.rs` | Yes |
| Review sequence and candidate lifecycle | `docs/arc42/06-runtime-view.md` | Affected | Application loop | Scenarios 1-15 | `run_review_path` loop and diagrams match | Yes |
| Decision semantics and persistence | `docs/arc42/08-crosscutting-concepts.md` | Affected | Panel state | Scenarios 1-12 | `handle_review_key` and main outcomes | Yes |
| QS-067, QS-070, QS-074 | `docs/arc42/10-quality-requirements.md` | Affected | Quality scenarios | Scenarios 1-15 | Flow-first card, direct keys, regeneration | Yes |
| Regeneration and catalog risk | `docs/arc42/11-risks-and-technical-debt.md` | Affected | Failure Outcomes | Scenario 11 | R-077 mitigation implemented | Yes |
| Domain terms | `docs/arc42/12-glossary.md` | Affected | Glossary Updates | Scenarios 1-12 | `Model chooser` term; corrected `Review history`/`Review outcome` | Yes |
| Chapters 1, 2, 7, 9 | n/a | Unaffected | ADR `NOT_QUALIFIED` | n/a | No ADR, no deployment or constraint change | Yes |

ARC42 CONFORMANCE: CLEAN

The durable chapter edits for 08, 10, 11, and 12 were restored after the review
found them missing from the tree (the earlier batch edit stopped at chapter 06);
they are committed in `docs(arc42): record flow-first decisions, model chooser,
and regeneration risk`.

## Ubiquitous language conformance

Specs, design, and code reuse the glossary terms: Candidate, Command flow,
Review surface, Review decision, Intent, Stage text, Stage purpose, and the new
Model chooser with its anti-terms (not the SetupWizard Model picker, not the
model table or tier tabs). Code symbols `ReviewPanelState`, `ModelChooser`,
`TierChoice`, `ReviewOutcome` use the same vocabulary.

UBIQUITOUS LANGUAGE: CLEAN

## Overlap dispositions

| Scenario A | Scenario B | Disposition |
|---|---|---|
| Enter accepts the current candidate | The accept shortcut accepts the current candidate | variant |
| A model name is suggested from the provider catalog while typing | A typed model name works when the catalog is unavailable | variant |
| Disabled review preserves -x confirmation | The explanation card ignores review decisions | variant |
| The -x confirmation offers to explain the command | The explanation card ignores review decisions | variant |

Enter and `a` are two triggers for one acceptance rule, asserted separately so a
regression in either key fails. The catalog scenarios differ in the state under
test (suggestions available versus unavailable). The explanation-card scenario
reuses the confirmation and explanation shapes but adds the ignored-decision
invariant, so it is a variant of both. The new reject E2E is a distinct action
(opening the chooser and issuing a second provider request) and the removed
"Review actions use Enter and Escape" scenario is superseded by the flow-first
key map; those two earlier dispositions no longer match a current shape.

## Split-or-keep

No scenario exceeds the deterministic long-scenario threshold.

## Coverage classification

Measurement: `./measure-coverage.sh` then `./merge-coverages.sh` (both exit 0).
Merged Cobertura line rate: **92.1%** (instrumented runs: 209 regular and 82
e2e scenarios, all passed).

| Region | Coverage |
|---|---|
| `src/review/session.rs` | 69/79 (87.3%) |
| `src/review/panel.rs` | 842/878 (95.9%) |
| `src/review/card.rs` | 521/522 (99.8%) |
| `src/main.rs` | 532/732 (72.7%) |

Remaining gaps, all classified:

| Region | Bucket | Disposition |
|---|---|---|
| `session.rs` input-loop poll/grace and closure terminator | 3 — hard to test | Cancellation timing requires a stalled provider mid-review; the Ctrl+C contract is e2e-covered by the cancel-running-completion capability against the binary |
| `panel.rs` chooser-null/tier-null guards, boundary no-ops, and chooser branches | 3 — defensive/edge | Guards cover unreachable or forged review state; query boundary behavior is asserted by the chooser unit tests that exercise the same branches |
| `panel.rs` terminal open/small-layout/restore errors | 3 — hard to test | Require `/dev/tty` failure injection; the terminal failure behavior is covered by the existing card-failure scenario at the application layer |
| `card.rs` truncation block terminator | 3 — instrumentation artifact | The truncation body executes in the wrapping unit test; only the block-closing line is not mapped |
| `main.rs` registry-error, parse-failure, and render-failure branches | 3 — hard to test | Defensive branches; the failure policy is covered by "A failed regeneration preserves the previous candidate" at the session/panel layer; driving the binary error arm needs a failing twin mid-PTY |
| `main.rs` non-review streaming metadata, setup subcommands, and wait-helper interrupt arm | 3 — pre-existing | Unchanged paths from earlier capabilities; they require verbose/pricing configuration, a real terminal, or an interrupt race |

No bucket-1 dead code and no unclassified bucket-2 gap remain. The unit tests
added during review (`chooser_query_editing_and_suggestion_selection_are_covered`,
`catalog_results_errors_and_highlights_are_applied_safely`,
`editor_arrow_keys_and_explanation_decisions_are_covered`,
`a_long_stage_wraps_to_a_continuation_row`) raised panel coverage from 85.5% to
95.9% and card coverage to 99.8%.

## README impact decision

README-IMPACT: none

The README does not document the review card's key bindings, and no CLI flag,
configuration key, or output contract changed in this capability.

## Verification runs

- `./run-tests.sh` (instrumented) → exit 0; 21 features, 209 scenarios, 1262 steps passed
- `./run-tests.sh --e2e` (instrumented) → exit 0; 24 features, 82 scenarios, 599 steps passed
- `./measure-coverage.sh` → both source reports generated, exit 0
- `./merge-coverages.sh` → merged report refreshed, exit 0
- `givn lint --change simplify-review-card-controls` → exit 0, clean
- `cargo check --locked` → clean
- `cargo test --locked --lib` → 77 passed
- Scenario commits: `7ca2fcf`, `5d90d2e`, `8f4a93e`, `a75973b`, `b486614`,
  `d53e959`, `5dafdc5`, `c298ebf`, `11a5c5b`, `5a23edc`, `19db0a6`, `c0f4e5d`,
  `69ec422`, `13e1cd9`, `7a8ba1c`, `cbbe6c3`; review tests `4621f36` and the
  follow-up.

## Sign-off

- Fabrication audit: clean.
- Every checked task has a verified commit touching production code; binding-only
  scenarios are recorded as reuse.
- Every promised component exists.
- Strict-mode proof present and passing.
- `verify.command` and `verify.e2e_command` both exit 0; the e2e scope is a
  strict subset (82 < 209).
- Coverage measured across the runner, both binaries, and the unit tests; every
  gap classified and resolved under the three buckets.
- No `@wip` tags remain.
- The new interaction has its `@e2e` scenario; existing inventory actions keep
  their coverage.
- Durable arc42 chapters and the glossary match the implementation.
- Local run command starts the CLI; the provider twin runs in-process.

REVIEW: PASS
