# Review: fix-review-response-fallback

## Fabrication audit

| # | Check | Result |
|---:|---|---|
| 0 | `@e2e` tag integrity | PASS — the delta adds only `@givn.added` regular scenarios; no `@e2e` scenario was modified or removed and the four archived interaction scenarios keep their tags |
| 1 | Empty/no-op step bodies | PASS — 0 empty bodies across the touched step file; all new bindings assert or set up state |
| 2 | Checked tasks have commits touching production | PASS — scenario 1 `5861b00` (`src/review/response.rs`, `src/main.rs`, `src/review/mod.rs`), scenario 3 `7a56539` (`src/review/response.rs` validation fallthrough), scenario 4 `69fd2e8` (`src/review/panel.rs` sanitization); scenario 2 `ec19570` exercises the recovery function from `5861b00` and records that explicitly in tasks.md |
| 3 | design.md components exist | PASS — `candidate_from_provider_response`, tolerant `parse_structured_review_response`, fence/prose locator (`src/review/response.rs`); flattening sanitization (`src/review/panel.rs`); review path and no-fence prompt (`src/main.rs`) |
| 4 | Strict-mode proof | PASS — tasks.md records the non-zero `unimplemented!()` proof (exit 1, step panicked) |
| 5 | `@e2e` Then steps assert real interface | N/A — no new or changed `@e2e` scenario; archived interaction scenarios are untouched and still pass (81/81) |
| 6 | Browser-UI check | N/A — CLI/terminal capability |
| 7 | `verify.e2e_command` binding | PASS — `./run-tests.sh --e2e` (`givn/commands.yaml`); distinct from `verify.command`; no parallel E2E implementation exists |
| 8 | One `@e2e` per distinct action | PASS — no interaction is added, so no new E2E scenario is due; four inventory actions remain covered |
| 9 | Local runnability | PASS — loopback provider twin and PTY seams; no live provider or external network |
| 10 | `verify.command` != `verify.e2e_command` | PASS — 185 regular vs 81 E2E scenarios; strict subset |
| 11 | Implementation vs design.md | PASS — implemented exactly as designed: locator → strict validation → provider-command recovery → command-only text → `Unavailable`, and row flattening in sanitization |
| 13 | Interaction coverage cross-reference | PASS — unchanged inventory; the four archived @e2e rows still match (matrix below) |
| 14 | Coverage measurement validity | PASS — `measure-coverage.sh` instruments both `watn` binaries and the Gherkin runner, merges per-process profraws, and the new recovery paths report non-zero coverage |

### Interaction coverage matrix (unchanged inventory)

| Inventory entry (`usecase.md`) | `@e2e` scenario | Driving mechanism |
|---|---|---|
| review and accept a generated candidate from Ctrl-W | Developer accepts an explained candidate from Ctrl-W | Real Bash PTY, installed widget, `pty_write` |
| cancel a candidate review from Ctrl-W | Developer cancels a review without changing the shell buffer | Real Bash PTY, installed widget, `pty_write` |
| review and accept a direct interactive request | Developer accepts a candidate from an interactive terminal request | Real `watn` subprocess in a PTY with captured stdout |
| review and execute an accepted eligible `-x` candidate | Developer accepts an eligible `-x` candidate and it executes once | Real `watn -x` subprocess in a PTY |

The four new delta scenarios are response-handling variants of these actions,
not new interactions, so they are regular scenarios.

## Overlap dispositions

`givn lint` reports two informational shape matches, dispositioned here:
each pair is an input or precondition variant of the same invariant, not a
duplicate.

| Scenario A | Scenario B | Disposition |
|---|---|---|
| The review surface explains a complex command flow | A markdown-fenced structured response is still explained | variant |
| Portable review-surface failure releases no candidate | A provider payload without a usable command releases nothing | variant |

## Split-or-keep

No scenario exceeds the deterministic long-scenario threshold; no split-or-keep
decision is required.

## Coverage classification

Measurement: `./measure-coverage.sh` then `./merge-coverages.sh` (both exit 0;
regular 185/185, E2E 81/81). Merged Cobertura line rate: **92.4%**.

| Region | Coverage |
|---|---|
| `src/review/response.rs` | 270/277 (97.5%) |
| `src/review/panel.rs` | 714/737 (96.9%) |
| `src/main.rs` | 420/521 (80.6%) |

Remaining gaps, classified:

| Region | Bucket | Disposition |
|---|---|---|
| `response.rs` empty-payload guard (`candidate_from_provider_response("")`) | 3 — hard to test | The provider buffer never opens for empty content; the design's Empty complete Candidate case produces `Unavailable` and releases nothing, which the no-command scenario covers at the outcome level |
| `response.rs` branch-attribution artifacts in the key-presence condition and function braces | 3 — hard to test | The statements are exercised by the recovery scenario; llvm-cov attributes uncovered branch edges to the `||` continuation |
| `panel.rs` `ControllingTerminal::open`/`restore` I/O-failure branches | 3 — hard to test | Pre-existing failure paths requiring a failing terminal descriptor or simultaneous multi-write failures; the observable portable-failure outcome is scenario-classified in the archived change |
| `main.rs` setup and interactive review error branches | 3 — hard to test (pre-existing) | Setup-wizard and terminal-open/read failure paths, unchanged by this fix |

No bucket-1 dead code and no bucket-2 missing coverage remain; unit tests cover
fence extraction, prose wrapping, command recovery, brace-containing command
text, unavailable payloads, and row flattening (61 lib tests).

## Arc42 implementation conformance

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| Recovery branch in the review flow | `docs/arc42/06-runtime-view.md` | Affected | Provider Response Recovery | Scenarios 1-3 | `candidate_from_provider_response` | Yes |
| Row flattening in text sanitization | `docs/arc42/08-crosscutting-concepts.md` | Affected | Inline Rendering Safety | Scenario 4 | `sanitize_terminal_text` maps `\n`, `\r`, `\t` to space | Yes |
| QS-072 provider-wrapped cases | `docs/arc42/10-quality-requirements.md` | Affected | Failure Outcomes | Scenarios 1-4 | Scenario assertions | Yes |
| R-072 payload-shape tolerance | `docs/arc42/11-risks-and-technical-debt.md` | Affected | Failure Outcomes | Scenarios 2-3 | Recovery and unavailable paths | Yes |
| Chapters 1-5, 7, 9, 12 | n/a | Unaffected | ADR qualification | n/a | No goal, constraint, context, strategy, building-block, deployment, decision, or glossary change | Yes |

ARC42 CONFORMANCE: CLEAN

## Ubiquitous language conformance

The change reuses the recorded terms `Candidate`, `Intent`, `Command flow`,
`Stage purpose`, `purpose-unavailable`, `Review surface`, and the channel
names. No new term is introduced and no synonym drift exists.

UBIQUITOUS LANGUAGE: CLEAN

## README impact decision

README-IMPACT: none

No command, flag, configuration key, or documented output format changed. The
README already describes the review surface generically.

## Verification runs

- `./run-tests.sh` → exit 0; 21 features, 185 scenarios, 1111 steps passed
- `./run-tests.sh --e2e` → exit 0; 24 features, 81 scenarios, 592 steps passed
- `./measure-coverage.sh` → both Cobertura reports generated, exit 0
- `./merge-coverages.sh` → merged report refreshed, exit 0
- `givn lint --change fix-review-response-fallback` → clean, exit 0
- `cargo check --locked` → clean
- `cargo test --locked --lib --features test-support` → 61 passed

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
