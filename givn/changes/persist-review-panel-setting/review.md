# Review: persist-review-panel-setting

## Fabrication audit

| # | Check | Result |
|---:|---|---|
| 0 | `@e2e` tag integrity | PASS — delta adds only `@givn.added` regular scenarios; no `@e2e` scenario touched |
| 1 | Empty/no-op step bodies | PASS — 0 empty bodies across the touched step file |
| 2 | Checked tasks have commits touching production | PASS — `61a5d77` (config persistence generalization, main persistence, harness), `79dbbb3` (disable-direction assertion binding) |
| 3 | design.md components exist | PASS — `persist_review_panel_at`, `persist_review_panel`, main override persistence |
| 4 | Strict-mode proof | PASS — tasks.md records the non-zero proof (exit 1) |
| 5 | `@e2e` Then steps assert real interface | PASS — archived `@e2e` scenarios still drive real PTY/subprocess boundaries (81/81) |
| 6 | Browser-UI check | N/A — CLI/terminal capability |
| 7 | `verify.e2e_command` binding | PASS — `./run-tests.sh --e2e`; distinct from `verify.command` |
| 8 | One `@e2e` per distinct action | PASS — no interaction added; the flags already exist |
| 9 | Local runnability | PASS — loopback provider twin |
| 10 | `verify.command` != `verify.e2e_command` | PASS — 200 regular vs 81 E2E scenarios; strict subset |
| 11 | Implementation vs design.md | PASS — resolution and persistence implemented exactly as designed; a failed write warns and does not change the invocation |
| 13 | Interaction coverage cross-reference | PASS — unchanged inventory |
| 14 | Coverage measurement validity | PASS — both binaries and the runner instrumented; the persistence path reports non-zero coverage |

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
| Disabled review preserves -x confirmation | Disabling the review panel from the command line persists it | variant |
| Disabling the review panel preserves the original command handling | Enabling the review panel from the command line persists it | variant |

## Split-or-keep

No scenario exceeds the deterministic long-scenario threshold.

## Coverage classification

Measurement: `./measure-coverage.sh` then `./merge-coverages.sh` (both exit 0).
Merged Cobertura line rate: **92.2%**.

| Region | Coverage |
|---|---|
| `src/config/mod.rs` | 290/314 (92.4%) |
| `src/main.rs` | 428/600 (71.3%) |

Remaining gaps, classified:

| Region | Bucket | Disposition |
|---|---|---|
| `main.rs` setup/error branches and interactive review/explain loops | 3 — hard to test (pre-existing) | Require a real terminal and event stream; the persistence behavior added here is exercised through the real binary with an isolated config home |
| `config/mod.rs` xdg default path (`persist_review_panel`) | 3 — hard to test | `persist_review_panel_at` is exercised end-to-end; the default path only supplies `xdg_config_path()` |
| `config/mod.rs` atomic-write failure branches | 3 — hard to test | Require filesystem failure injection; the failure policy (warn, keep invocation behavior) is documented and the happy path is covered |

No bucket-1 dead code and no bucket-2 missing coverage remain; the persistence
unit test and two scenarios cover the added behavior.

## Arc42 implementation conformance

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| Last chosen preference survives | `docs/arc42/04-solution-strategy.md` | Affected | Resolution | Scenarios 1-2 | Main persists set overrides | Yes |
| Override persists before generation | `docs/arc42/06-runtime-view.md` | Affected | Resolution | Scenarios 1-2 | `persist_review_panel` before resolution | Yes |
| CLI persistence in precedence | `docs/arc42/08-crosscutting-concepts.md` | Affected | Resolution | Scenarios 1-2 | Atomic load-modify-save | Yes |
| Preference-write failure risk | `docs/arc42/11-risks-and-technical-debt.md` | Affected | Failure Outcomes | N/A (policy) | Warning without behavior change | Yes |
| Chapters 1-3, 5, 7, 9, 10, 12 | n/a | Unaffected | ADR qualification | n/a | No other change | Yes |

ARC42 CONFORMANCE: CLEAN

## Ubiquitous language conformance

Recorded terms keep their meanings; no new term or synonym drift.

UBIQUITOUS LANGUAGE: CLEAN

## README impact decision

README-IMPACT: updated - Configuration

The README documents that `--review-panel` / `--no-review-panel` persist the
chosen setting.

## Verification runs

- `./run-tests.sh` → exit 0; 21 features, 200 scenarios, 1207 steps passed
- `./run-tests.sh --e2e` → exit 0; 24 features, 81 scenarios, 592 steps passed
- `./measure-coverage.sh` → both Cobertura reports generated, exit 0
- `./merge-coverages.sh` → merged report refreshed, exit 0
- `givn lint --change persist-review-panel-setting` → clean, exit 0
- `cargo check --locked` → clean
- `cargo test --locked --lib --features test-support` → 72 passed

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
