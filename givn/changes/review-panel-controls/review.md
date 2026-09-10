# Review: review-panel-controls

## Fabrication audit

| # | Check | Result |
|---:|---|---|
| 0 | `@e2e` tag integrity | PASS — delta adds only `@givn.added` regular scenarios and declares four `@givn.removed`; no `@e2e` scenario touched |
| 1 | Empty/no-op step bodies | PASS — 0 empty bodies across the touched step files; removed-scenario placeholders are filtered by the runner |
| 2 | Checked tasks have commits touching production | PASS — `d28a40e` (single card renderer, controls, persistence, confirmation plumbing), `e583329`, `6cfb110`, `04241b4`, `699b647`, `a1120a1` |
| 3 | design.md components exist | PASS — single card renderer, `persist_review_disabled_at`, `d` outcome, explain-only mode, `classify_confirmation` |
| 4 | Strict-mode proof | PASS — tasks.md records the non-zero proof (exit 1) |
| 5 | `@e2e` Then steps assert real interface | PASS — archived `@e2e` scenarios still drive real PTY/subprocess boundaries (81/81) |
| 6 | Browser-UI check | N/A — CLI/terminal capability |
| 7 | `verify.e2e_command` binding | PASS — `./run-tests.sh --e2e`; distinct from `verify.command` |
| 8 | One `@e2e` per distinct action | PASS — no interaction added; `?` and `d` are review-decision variants, not new inventory actions |
| 9 | Local runnability | PASS — loopback provider twin and PTY seams |
| 10 | `verify.command` != `verify.e2e_command` | PASS — 198 regular vs 81 E2E scenarios; strict subset |
| 11 | Implementation vs design.md | PASS with note — the runner now filters scenarios declared `@givn.removed` by an active delta so the permanent tree does not double-run them before the archive merge; this is test-runner glue demanded by the givn removal contract and is documented here |
| 13 | Interaction coverage cross-reference | PASS — unchanged inventory; the four archived @e2e rows still match |
| 14 | Coverage measurement validity | PASS — both binaries and the runner instrumented; card, panel, and persistence paths report non-zero coverage |

### Interaction coverage matrix (unchanged inventory)

| Inventory entry (`usecase.md`) | `@e2e` scenario | Driving mechanism |
|---|---|---|
| review and accept a generated candidate from Ctrl-W | Developer accepts an explained candidate from Ctrl-W | Real Bash PTY, installed widget |
| cancel a candidate review from Ctrl-W | Developer cancels a review without changing the shell buffer | Real Bash PTY, installed widget |
| review and accept a direct interactive request | Developer accepts a candidate from an interactive terminal request | Real `watn` subprocess in a PTY with captured stdout |
| review and execute an accepted eligible `-x` candidate | Developer accepts an eligible `-x` candidate and it executes once | Real `watn -x` subprocess in a PTY |

## Overlap dispositions

| Scenario A | Scenario B | Disposition |
|---|---|---|
| A provider payload without a usable command releases nothing | A failing review card releases no candidate | variant |
| Disabled review preserves -x confirmation | The -x confirmation offers to explain the command | variant |

Removed+added pairs are supersessions: the enhanced/portable adapter scenarios
are replaced by the single-card failure and monochrome scenarios, and the
enhanced-disable scenario is replaced by the panel-disable scenario.

## Split-or-keep

No scenario exceeds the deterministic long-scenario threshold.

## Coverage classification

Measurement: `./measure-coverage.sh` then `./merge-coverages.sh` (both exit 0).
Merged Cobertura line rate: **92.2%**.

| Region | Coverage |
|---|---|
| `src/review/card.rs` | 436/455 (95.8%) |
| `src/review/panel.rs` | 664/694 (95.7%) |
| `src/config/mod.rs` | 270/297 (90.9%) |
| `src/exec.rs` | 84/115 (73.0%) |
| `src/main.rs` | 424/593 (71.5%) |

Remaining gaps, classified:

| Region | Bucket | Disposition |
|---|---|---|
| `exec.rs` interactive raw-mode confirmation reader and `main.rs` explain loop / `run_explanation_card` | 3 — hard to test | Require a real controlling terminal and event stream; the pure seam `classify_confirmation` and the explain-only card rendering are scenario- and unit-tested, and the `-x` scenario covers the observable outcome |
| `config/mod.rs` `persist_review_disabled` xdg path | 3 — hard to test | Requires the user's real config location; `persist_review_disabled_at` is exercised end-to-end by the permanent-disable scenario against an isolated file |
| `main.rs` setup/error branches and `card.rs` degenerate-width guards | 3 — hard to test (pre-existing) | Unchanged defensive paths; observable behavior is scenario-classified |

No bucket-1 dead code and no bucket-2 missing coverage remain; 71 lib tests
pass.

## Arc42 implementation conformance

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| One review presentation | `docs/arc42/04-solution-strategy.md` | Affected | Single Review Renderer | Scenario 1 | `src/review/card.rs` only renderer | Yes |
| Adapter building block removed | `docs/arc42/05-building-block-view.md` | Affected | Single Review Renderer | Scenario 1 | `adapter.rs` deleted | Yes |
| Confirmation explain and permanent disable | `docs/arc42/06-runtime-view.md` | Affected | Permanent Disable; `?` Explanation | Scenarios 5-6 | `classify_confirmation`, `persist_review_disabled_at`, `d` outcome | Yes |
| Color as a property; disable persistence | `docs/arc42/08-crosscutting-concepts.md` | Affected | Single Review Renderer; Permanent Disable | Scenarios 3, 5 | Mono card; atomic save path | Yes |
| QS-067/QS-071 adjusted | `docs/arc42/10-quality-requirements.md` | Affected | Failure Outcomes | Scenarios 1-3 | Card-only assertions | Yes |
| R-071 reduced; card failure added | `docs/arc42/11-risks-and-technical-debt.md` | Affected | Failure Outcomes | Scenarios 2-3 | Input preserved, nothing released | Yes |
| Chapters 1-3, 7, 9, 12 | n/a | Unaffected | ADR qualification | n/a | No other change | Yes |

ARC42 CONFORMANCE: CLEAN

## Ubiquitous language conformance

Recorded terms keep their meanings; no new term or synonym drift.

UBIQUITOUS LANGUAGE: CLEAN

## README impact decision

README-IMPACT: updated - Usage, Configuration

The README documents the card-only surface, the `d` permanent disable, the
`-x` `?` explanation, and removes the enhanced-review toggles.

## Verification runs

- `./run-tests.sh` → exit 0; 21 features, 198 scenarios, 1195 steps passed
- `./run-tests.sh --e2e` → exit 0; 24 features, 81 scenarios, 592 steps passed
- `./measure-coverage.sh` → both Cobertura reports generated, exit 0
- `./merge-coverages.sh` → merged report refreshed, exit 0
- `givn lint --change review-panel-controls` → clean, exit 0
- `cargo check --locked` → clean
- `cargo test --locked --lib --features test-support` → 71 passed

## Sign-off

- Fabrication audit: clean.
- Every checked task has a verified commit touching production code.
- Every promised component exists.
- Strict-mode proof present and passing.
- `verify.command` and `verify.e2e_command` both exit 0.
- Coverage measured across all test binaries, including the Gherkin runner.
- Every coverage gap classified and resolved under the three buckets.
- No `@wip` tags remain; removed scenarios are declared.
- No interaction was added; archived inventory actions keep their `@e2e`
  coverage.
- Local run command starts the full stack including the loopback provider twin.

REVIEW: PASS
