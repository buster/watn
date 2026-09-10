# Review: recover-review-stage-purposes

## Fabrication audit

| # | Check | Result |
|---:|---|---|
| 0 | `@e2e` tag integrity | PASS — the delta adds only `@givn.added` regular scenarios; no `@e2e` scenario was modified or removed |
| 1 | Empty/no-op step bodies | PASS — 0 empty bodies across the touched step file |
| 2 | Checked tasks have commits touching production | PASS — `f59db1c` (`src/review/response.rs`, `src/main.rs`), `4c25fbe` (`src/review/response.rs` normalization in the strict path), `eb86b7b` exercises the guard from `f59db1c` and records that in tasks.md |
| 3 | design.md components exist | PASS — `normalize_command`, `recovered_purposes`, recovered-purpose path, prompt rules |
| 4 | Strict-mode proof | PASS — tasks.md records the non-zero `unimplemented!()` proof (exit 1) |
| 5 | `@e2e` Then steps assert real interface | N/A — no new or changed `@e2e` scenario; archived interaction scenarios still pass (81/81) |
| 6 | Browser-UI check | N/A — CLI/terminal capability |
| 7 | `verify.e2e_command` binding | PASS — `./run-tests.sh --e2e`; distinct from `verify.command`; no parallel E2E implementation |
| 8 | One `@e2e` per distinct action | PASS — no interaction added; four inventory actions remain covered |
| 9 | Local runnability | PASS — loopback provider twin and PTY seams; no live provider |
| 10 | `verify.command` != `verify.e2e_command` | PASS — 188 regular vs 81 E2E scenarios; strict subset |
| 11 | Implementation vs design.md | PASS — recovery order, normalization, and guard implemented exactly as designed |
| 13 | Interaction coverage cross-reference | PASS — unchanged inventory; archived @e2e rows unchanged |
| 14 | Coverage measurement validity | PASS — both binaries and the Gherkin runner are instrumented; recovery paths report non-zero coverage |

### Interaction coverage matrix (unchanged inventory)

| Inventory entry (`usecase.md`) | `@e2e` scenario | Driving mechanism |
|---|---|---|
| review and accept a generated candidate from Ctrl-W | Developer accepts an explained candidate from Ctrl-W | Real Bash PTY, installed widget |
| cancel a candidate review from Ctrl-W | Developer cancels a review without changing the shell buffer | Real Bash PTY, installed widget |
| review and accept a direct interactive request | Developer accepts a candidate from an interactive terminal request | Real `watn` subprocess in a PTY with captured stdout |
| review and execute an accepted eligible `-x` candidate | Developer accepts an eligible `-x` candidate and it executes once | Real `watn -x` subprocess in a PTY |

The three new delta scenarios are purpose-recovery variants of these actions,
not new interactions.

## Overlap dispositions

| Scenario A | Scenario B | Disposition |
|---|---|---|
| An invalid structured response with a command stays reviewable | Mismatched stage text still shows purpose-unavailable | variant |

## Split-or-keep

No scenario exceeds the deterministic long-scenario threshold; no split-or-keep
decision is required.

## Coverage classification

Measurement: `./measure-coverage.sh` then `./merge-coverages.sh` (both exit 0;
regular 188/188, E2E 81/81). Merged Cobertura line rate: **92.4%**.

| Region | Coverage |
|---|---|
| `src/review/response.rs` | 351/360 (97.5%) |
| `src/review/panel.rs` | 714/737 (96.9%) |
| `src/main.rs` | 418/521 (80.2%) |

Remaining gaps, classified:

| Region | Bucket | Disposition |
|---|---|---|
| `response.rs` explicit guard returns in `locate_json_payload` / `recovered_purposes` (no fence body, whitespace-only purpose, empty payload) | 3 — hard to test | Defensive guards; the observable outcomes are covered by the mismatch, missing-purpose, and unavailable scenarios and unit tests |
| `response.rs` branch-attribution artifacts in the key-presence condition and braces | 3 — hard to test | Statements are exercised by the recovery scenarios; llvm-cov attributes uncovered branch edges to the `||` continuation |
| `panel.rs` and `main.rs` pre-existing I/O-failure and setup branches | 3 — hard to test (pre-existing) | Unchanged by this change; outcome-level behavior is scenario-classified |

No bucket-1 dead code and no bucket-2 missing coverage remain; 64 lib tests
pass.

## Arc42 implementation conformance

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| Recovery order and command normalization | `docs/arc42/06-runtime-view.md` | Affected | Recovery Order; Command Normalization | Scenarios 1-2 | `normalize_command`, `recovered_purposes` | Yes |
| Stage-text agreement and no invented purposes | `docs/arc42/08-crosscutting-concepts.md` | Affected | Recovery Order | Scenarios 1, 3 | Guard requires exact trimmed stage text and non-empty provider purpose | Yes |
| QS-072 unknown-status and multiline cases | `docs/arc42/10-quality-requirements.md` | Affected | Failure Outcomes | Scenarios 1-3 | Scenario assertions | Yes |
| R-072 vocabulary and formatting drift tolerance | `docs/arc42/11-risks-and-technical-debt.md` | Affected | Failure Outcomes | Scenarios 1-2 | Recovery and normalization | Yes |
| Chapters 1-5, 7, 9, 12 | n/a | Unaffected | ADR qualification | n/a | No other change | Yes |

ARC42 CONFORMANCE: CLEAN

## Ubiquitous language conformance

Recorded terms `Candidate`, `Command flow`, `Stage text`, `Stage purpose`,
`purpose-unavailable`, and `Review surface` are reused with their glossary
meanings. No new term or synonym drift exists.

UBIQUITOUS LANGUAGE: CLEAN

## README impact decision

README-IMPACT: none

No command, flag, configuration key, or documented output format changed.

## Verification runs

- `./run-tests.sh` → exit 0; 21 features, 188 scenarios, 1130 steps passed
- `./run-tests.sh --e2e` → exit 0; 24 features, 81 scenarios, 592 steps passed
- `./measure-coverage.sh` → both Cobertura reports generated, exit 0
- `./merge-coverages.sh` → merged report refreshed, exit 0
- `givn lint --change recover-review-stage-purposes` → clean, exit 0
- `cargo check --locked` → clean
- `cargo test --locked --lib --features test-support` → 64 passed

## Sign-off

- Fabrication audit: clean.
- Every checked task has a verified commit touching production code or a
  documented shared-behavior commit.
- Every promised component exists.
- Strict-mode proof present and passing.
- `verify.command` and `verify.e2e_command` both exit 0.
- Coverage measured across all test binaries, including the Gherkin runner.
- Every coverage gap classified and resolved under the three buckets.
- No `@wip` tags remain; the spec contains no implementation-layer detail.
- No interaction was added; the four archived inventory actions keep their
  `@e2e` coverage.
- Local run command starts the full stack including the loopback provider twin.

REVIEW: PASS
