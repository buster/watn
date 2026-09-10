# Review: render-review-card

## Fabrication audit

| # | Check | Result |
|---:|---|---|
| 0 | `@e2e` tag integrity | PASS — delta adds only `@givn.added` regular scenarios; no `@e2e` scenario touched |
| 1 | Empty/no-op step bodies | PASS — 0 empty bodies across the touched step files |
| 2 | Checked tasks have commits touching production | PASS — `c51c368` (card renderer, config, main, panel), `1a1a328` (navigation steps), `40b01a0` (marker), `bf86e6c` (config fallback), `5ee29c3` (capability), `463244e` (`e` shortcut) |
| 3 | design.md components exist | PASS — `src/review/card.rs`, `review_enhanced`, CLI overrides, adapter selection, harness renderer selection |
| 4 | Strict-mode proof | PASS — tasks.md records the non-zero proof (exit 1) |
| 5 | `@e2e` Then steps assert real interface | PASS — unchanged `@e2e` scenarios still drive real PTY/subprocess boundaries (81/81); E2E accept waits now match the card action label |
| 6 | Browser-UI check | N/A — CLI/terminal capability |
| 7 | `verify.e2e_command` binding | PASS — `./run-tests.sh --e2e`; distinct from `verify.command`; no parallel E2E implementation |
| 8 | One `@e2e` per distinct action | PASS — no interaction added |
| 9 | Local runnability | PASS — loopback provider twin and PTY seams |
| 10 | `verify.command` != `verify.e2e_command` | PASS — 196 regular vs 81 E2E scenarios; strict subset |
| 11 | Implementation vs design.md | PASS — card layout, color roles, fallback rules, and keyboard additions implemented as designed |
| 13 | Interaction coverage cross-reference | PASS — unchanged inventory; archived @e2e rows unchanged |
| 14 | Coverage measurement validity | PASS — both binaries and the runner instrumented; card paths report non-zero coverage |

### Interaction coverage matrix (unchanged inventory)

| Inventory entry (`usecase.md`) | `@e2e` scenario | Driving mechanism |
|---|---|---|
| review and accept a generated candidate from Ctrl-W | Developer accepts an explained candidate from Ctrl-W | Real Bash PTY, installed widget |
| cancel a candidate review from Ctrl-W | Developer cancels a review without changing the shell buffer | Real Bash PTY, installed widget |
| review and accept a direct interactive request | Developer accepts a candidate from an interactive terminal request | Real `watn` subprocess in a PTY with captured stdout |
| review and execute an accepted eligible `-x` candidate | Developer accepts an eligible `-x` candidate and it executes once | Real `watn -x` subprocess in a PTY |

The six new delta scenarios are presentation variants of these actions.

## Overlap dispositions

| Scenario A | Scenario B | Disposition |
|---|---|---|
| Disabling the enhanced card preserves the plain review surface | A color-incapable terminal falls back to the plain review surface | variant |

## Split-or-keep

No scenario exceeds the deterministic long-scenario threshold.

## Coverage classification

Measurement: `./measure-coverage.sh` then `./merge-coverages.sh` (both exit 0).
Merged Cobertura line rate: **92.6%**.

| Region | Coverage |
|---|---|
| `src/review/card.rs` | 409/428 (95.6%) |
| `src/review/panel.rs` | 756/790 (95.7%) |
| `src/review/response.rs` | 423/430 (98.4%) |
| `src/config/types.rs` | 188/189 (99.5%) |
| `src/main.rs` | 428/534 (80.1%) |

Remaining gaps, classified:

| Region | Bucket | Disposition |
|---|---|---|
| `card.rs` `ellipsize` zero-width guard, and `wrap_capped` / `wrap_visible` truncate branches | 3 — hard to test | Defensive guards for degenerate widths and values longer than two rows; the observable bounded-card behavior is covered by the narrow-terminal unit test and the archived bounded-layout scenario |
| `card.rs` branch-attribution artifacts (frame assembly `else` edges) | 3 — hard to test | Statements are exercised by every card unit test; llvm-cov attributes uncovered branch edges to inline formatting |
| `panel.rs`, `response.rs`, `main.rs` pre-existing I/O-failure and setup branches | 3 — hard to test (pre-existing) | Unchanged; outcome-level behavior is scenario-classified |

No bucket-1 dead code and no bucket-2 missing coverage remain; 73 lib tests
pass.

## Arc42 implementation conformance

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| Review card renderer building block | `docs/arc42/05-building-block-view.md` | Affected | Card Layout | Scenario 1 | `src/review/card.rs` | Yes |
| Card presentation with plain fallback | `docs/arc42/06-runtime-view.md` | Affected | Renderer Selection | Scenarios 1, 4, 5 | `use_card` selection in `main.rs` and the harness | Yes |
| Color capability and fallback | `docs/arc42/08-crosscutting-concepts.md` | Affected | Renderer Selection | Scenarios 4, 5 | `terminal_supports_color`, fallback tests | Yes |
| QS-067 readable hierarchy | `docs/arc42/10-quality-requirements.md` | Affected | Card Layout | Scenarios 1-3 | Card assertions | Yes |
| R-068/R-071 card fallback | `docs/arc42/11-risks-and-technical-debt.md` | Affected | Failure Outcomes | Scenarios 4, 5 | Disable/capability fallback | Yes |
| Chapters 1-4, 7, 9, 12 | n/a | Unaffected | ADR qualification | n/a | No other change | Yes |

ARC42 CONFORMANCE: CLEAN

## Ubiquitous language conformance

`Candidate`, `Command flow`, `Stage text`, `Stage purpose`,
`purpose-unavailable`, `Review surface`, and `Presentation adapter` keep their
recorded meanings. No new term or synonym drift.

UBIQUITOUS LANGUAGE: CLEAN

## README impact decision

README-IMPACT: updated - Usage, Configuration

The README documents the review card, its key hints, the
`--enhanced-review-panel` / `--no-enhanced-review-panel` flags, the
`[review] enhanced` setting, and the automatic color fallback.

## Verification runs

- `./run-tests.sh` → exit 0; 21 features, 196 scenarios, 1182 steps passed
- `./run-tests.sh --e2e` → exit 0; 24 features, 81 scenarios, 592 steps passed
- `./measure-coverage.sh` → both Cobertura reports generated, exit 0
- `./merge-coverages.sh` → merged report refreshed, exit 0
- `givn lint --change render-review-card` → clean, exit 0
- `cargo check --locked` → clean
- `cargo test --locked --lib --features test-support` → 73 passed

## Sign-off

- Fabrication audit: clean.
- Every checked task has a verified commit touching production code.
- Every promised component exists.
- Strict-mode proof present and passing.
- `verify.command` and `verify.e2e_command` both exit 0.
- Coverage measured across all test binaries, including the Gherkin runner.
- Every coverage gap classified and resolved under the three buckets.
- No `@wip` tags remain.
- No interaction was added; archived inventory actions keep their `@e2e`
  coverage.
- Local run command starts the full stack including the loopback provider twin.

REVIEW: PASS
