# Review: accept-provider-stage-split

## Fabrication audit

| # | Check | Result |
|---:|---|---|
| 0 | `@e2e` tag integrity | PASS — delta adds only `@givn.added` regular scenarios; no `@e2e` scenario touched |
| 1 | Empty/no-op step bodies | PASS — 0 empty bodies across the touched step file |
| 2 | Checked tasks have commits touching production | PASS — `72a007b` (`src/review/flow.rs`, `src/review/response.rs`), `8266132` exercises the guard from `72a007b`; refactor commit removes superseded code |
| 3 | design.md components exist | PASS — `command_stage`, `provider_stage_split`, recovery wiring |
| 4 | Strict-mode proof | PASS — tasks.md records the non-zero proof (exit 1) |
| 5 | `@e2e` Then steps assert real interface | N/A — no new or changed `@e2e` scenario; archived interactions still pass (81/81) |
| 6 | Browser-UI check | N/A — CLI/terminal capability |
| 7 | `verify.e2e_command` binding | PASS — `./run-tests.sh --e2e`; distinct from `verify.command`; no parallel E2E implementation |
| 8 | One `@e2e` per distinct action | PASS — no interaction added |
| 9 | Local runnability | PASS — loopback provider twin and PTY seams |
| 10 | `verify.command` != `verify.e2e_command` | PASS — 190 regular vs 81 E2E; strict subset |
| 11 | Implementation vs design.md | PASS — split contract implemented exactly as designed |
| 13 | Interaction coverage cross-reference | PASS — unchanged inventory; archived @e2e rows unchanged |
| 14 | Coverage measurement validity | PASS — both binaries and the runner instrumented; split paths non-zero |

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
| Mismatched stage text still shows purpose-unavailable | Provider stages that do not cover the command are not trusted | variant |

## Split-or-keep

No scenario exceeds the deterministic long-scenario threshold.

## Coverage classification

Measurement: `./measure-coverage.sh` then `./merge-coverages.sh` (both exit 0).
Merged Cobertura line rate: **92.5%**.

| Region | Coverage |
|---|---|
| `src/review/response.rs` | 423/430 (98.4%) |
| `src/review/flow.rs` | 321/323 (99.4%) |
| `src/review/panel.rs` | 714/737 (96.9%) |
| `src/main.rs` | 420/521 (80.6%) |

Remaining gaps, classified:

| Region | Bucket | Disposition |
|---|---|---|
| `response.rs` empty-payload guard and branch-attribution braces | 3 — hard to test | The empty-candidate outcome is the design's `Unavailable` case; the guards defend future callers and the observable outcome is covered by the no-command and untrusted-split scenarios |
| `flow.rs` defensive `push_stage` guard | 3 — hard to test | Unreachable through `derive_command_flow`; retained as a guard |
| `panel.rs` and `main.rs` pre-existing I/O-failure and setup branches | 3 — hard to test (pre-existing) | Unchanged; outcome-level behavior is scenario-classified |

Dead code found during review (`recovered_purposes`, superseded by the split
contract) was deleted in the refactor commit. No bucket-2 missing coverage
remains; 67 lib tests pass.

## Arc42 implementation conformance

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| Stage acceptance in the review flow | `docs/arc42/06-runtime-view.md` | Affected | Provider Split Contract | Scenarios 1-2 | `provider_stage_split` | Yes |
| Verbatim stage proof and no-invention | `docs/arc42/08-crosscutting-concepts.md` | Affected | Provider Split Contract | Scenarios 1-2 | Guard tests reject fabricated, gapped, trailing, empty, and blank-purpose payloads | Yes |
| QS-072 compound provider splits | `docs/arc42/10-quality-requirements.md` | Affected | Failure Outcomes | Scenarios 1-2 | Scenario assertions | Yes |
| R-072 split-trust rules | `docs/arc42/11-risks-and-technical-debt.md` | Affected | Provider Split Contract | Scenarios 1-2 | Recovery rules | Yes |
| Chapters 1-5, 7, 9, 12 | n/a | Unaffected | ADR qualification | n/a | No other change | Yes |

ARC42 CONFORMANCE: CLEAN

## Ubiquitous language conformance

`Candidate`, `Command flow`, `Stage text`, `Stage purpose`, `purpose-unavailable`
keep their recorded meanings. No new term or synonym drift.

UBIQUITOUS LANGUAGE: CLEAN

## README impact decision

README-IMPACT: none

No command, flag, configuration key, or documented output format changed.

## Verification runs

- `./run-tests.sh` → exit 0; 21 features, 190 scenarios, 1145 steps passed
- `./run-tests.sh --e2e` → exit 0; 24 features, 81 scenarios, 592 steps passed
- `./measure-coverage.sh` → both Cobertura reports generated, exit 0
- `./merge-coverages.sh` → merged report refreshed, exit 0
- `givn lint --change accept-provider-stage-split` → clean, exit 0
- `cargo check --locked` → clean
- `cargo test --locked --lib --features test-support` → 67 passed

## Sign-off

- Fabrication audit: clean.
- Every checked task has a verified commit touching production code.
- Every promised component exists.
- Strict-mode proof present and passing.
- `verify.command` and `verify.e2e_command` both exit 0.
- Coverage measured across all test binaries, including the Gherkin runner.
- Every coverage gap classified and resolved under the three buckets.
- Dead code deleted.
- No `@wip` tags remain.
- No interaction was added; archived inventory actions keep their `@e2e`
  coverage.
- Local run command starts the full stack including the loopback provider twin.

REVIEW: PASS
