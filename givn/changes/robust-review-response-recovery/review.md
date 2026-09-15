# Review: robust-review-response-recovery

## Fabrication audit

| # | Check | Result |
|---:|---|---|
| 0 | `@e2e` tag integrity | PASS — neither delta feature ever carried an `@e2e` tag (`git log -p` over both files shows no `@e2e` addition or removal); the change adds no `@e2e` scenario and touches no existing one |
| 1 | Empty/no-op step bodies | PASS — 0 empty bodies across the two touched step files; the single `_ => {}` at `tests/steps/explain_command_steps.rs:199` is a match arm in a text sanitizer, not a step body; no `unimplemented!()`/`todo!()` remains |
| 2 | Checked tasks have commits touching production | PASS with the shared-behavior note below — 5 of 14 scenario commits touch production (`0b24aaf` S1, `9fb145f` S2, `c5bd4ca` S6, `3c28749` S7, `75d1aed` S13); the other 9 add only scenario assertions for behavior that landed in S1/S2/S7/S13 and record "production files: none beyond …" in tasks.md; every named production symbol exists in the committed tree and every scenario passes |
| 3 | design.md components exist | PASS — `repair_json_control_characters`, `recover_command_value`, `looks_like_review_payload`, `IncompleteResponse`, `card_reason()`, `unavailable_reason`, `finish_reason`, `src/review/diagnostics.rs` (`capture_unusable_response`), `config::unusable_response_path()`, `render::print_raw_response`, `Explain::verbose`, and the `explain_command_candidate` return signature all exist as named |
| 4 | Strict-mode proof | PASS — tasks.md records the temporary undefined-step probe exiting non-zero (`Step doesn't match any function`, exit 1) |
| 5 | `@e2e` Then steps assert real interface | N/A — no new or changed `@e2e` scenario; the 89 existing `@e2e` scenarios drive the real binary through a PTY and assert transcripts/stdout |
| 6 | Browser-UI check | N/A — stated the opposite classification first: a rendered page is a Web UI; watn renders no HTML/CSS/JS, its surface is a terminal card, and the real interface is the CLI subprocess (PTY), so the browser-driver rule does not apply |
| 7 | `verify.e2e_command` binding | PASS — `givn/commands.yaml` runs `./run-tests.sh --e2e`; that path invokes `features_e2e` with the tag filter `@e2e and not @wip and not @givn.removed and not @sandbox`; the touched capabilities' e2e steps live in `tests/steps/interactive_shell_shortcut_e2e_steps.rs` and `tests/steps/explain_command_e2e_steps.rs`; no second/parallel e2e implementation exists (working tree has no untracked files) |
| 8 | One `@e2e` per distinct action | PASS — no inventory entry is added or changed; repair, recovery, reason naming, capture, and verbose output are variants of the three existing actions, so they are regular scenarios and no excess `@e2e` scenario exists |
| 9 | Local runnability | PASS — `cargo run --release -- --help` exits 0; tests use the in-process `httpmock` provider twin and a `portable-pty` terminal twin; no live service or external network |
| 10 | `verify.command` != `verify.e2e_command` | PASS — `./run-tests.sh` vs `./run-tests.sh --e2e` are distinct and the e2e scope is tag-filtered; regular 259 scenarios vs e2e 89 scenarios, strictly smaller; this change has no archive receipt yet, so the per-scope counts come from the runs below and the archive receipt will carry the authoritative record |
| 11 | Implementation vs design.md | FIXED — the post-implementation design edit was re-reviewed (R16/R17 in design-review.md): the Purpose-reason table now matches `card_reason()` arm-for-arm, `explain_reason()` keeps the stderr wording, and the command-only paragraph is path-scoped; no other deviation from a design-named command, file layout, or framework was found |
| 13 | Interaction coverage cross-reference | PASS — see the matrix below; the one use-case inventory entry not in the matrix belongs to the untouched `shell-completions` capability and keeps its permanent `@e2e` evidence |
| 14 | Coverage measurement validity | PASS — `measure-coverage.sh` runs the instrumented `watn` binaries and the Gherkin runner with per-process `%p-%m.profraw` output, passes the instrumented binaries to the PTY steps via `WATN_DEFAULT_DEBUG_BIN`/`WATN_TEST_SUPPORT_DEBUG_BIN`, and `merge-coverages.sh` consumes both source reports; the merged report is newer than both sources (13:54 vs 11:11/11:12); `src/review/diagnostics.rs` 38/39 and `src/review/response.rs` 673/890 prove non-zero production coverage |

### Interaction coverage cross-reference

| Inventory entry (`usecase.md`) | Design matrix row | `@e2e` scenario | Driving mechanism |
|---|---|---|---|
| interactive-shell-shortcut / validate generated shell configuration | yes | Generated Bash, Zsh, and Fish configurations pass shell syntax checks | Real `watn` binary; generated blocks fed to the target shell parser |
| interactive-shell-shortcut / inspect generated Bash widget | yes | The generated Bash widget keeps the request visible and does not evaluate the command | Real `watn` in a `portable-pty` Bash session |
| interactive-shell-shortcut / use Fish Ctrl-W shortcut | yes | Fish replaces the buffer with the generated command after Ctrl-W | Real `watn` in a `portable-pty` Fish session |
| interactive-shell-shortcut / review and accept a generated candidate from Ctrl-W | yes | Developer accepts an explained candidate from Ctrl-W | Real `watn` in a `portable-pty` Bash session with the installed widget |
| interactive-shell-shortcut / switch to the detailed review view during a Ctrl-W review | yes | Developer switches to the detailed review view during Ctrl-W review | Real `watn` in a `portable-pty` session; view toggle read from the card |
| interactive-shell-shortcut / disable the review surface from a candidate review | yes | The panel can permanently disable the review | Real `watn` in a `portable-pty` session; `D` key |
| interactive-shell-shortcut / configure the review surface from the command line without a request | yes | The review surface switches configure without a request | Real `watn` binary with the panel flags; persisted configuration read |
| interactive-shell-shortcut / cancel a candidate review from Ctrl-W | yes | Developer cancels a review without changing the shell buffer | Real `watn` in a `portable-pty` Bash session; Escape |
| interactive-shell-shortcut / review and accept a direct interactive request | yes | Developer accepts a candidate from an interactive terminal request | Real `watn` subprocess in a `portable-pty` session; redirected stdout |
| interactive-shell-shortcut / review and execute an accepted eligible -x candidate | yes | Developer accepts an eligible -x candidate and it executes once | Real `watn -x` subprocess in a `portable-pty` session |
| interactive-shell-shortcut / reject a candidate and regenerate with another model | yes | Developer rejects a candidate and regenerates with another model | Real `watn` in a `portable-pty` session; `r` and tier selection |
| explain-command / explain an existing command in the review card | yes | Developer explains an existing command in the review card | Real `watn` subprocess in a `portable-pty` session |
| shell-completions / generate Bash completions | not touched by this change | Built Bash completion generation emits the current command tree | Unchanged permanent `@e2e` scenario; ran in the e2e scope |

The 14 delta scenarios are response-handling variants of the three touched
actions (`invoke Ctrl-W`, `submit an eligible interactive request`,
`run watn explain`), so they add regular scenarios and no inventory entry, as
the design's normalization note states.

## Overlap dispositions

`givn lint --change robust-review-response-recovery` exits 0 with advisory
shape/subset findings only. The two shape matches (A is the permanent spec
scenario, B the delta scenario):

| Scenario A | Scenario B | Disposition |
|---|---|---|
| A provider payload without a usable command releases nothing | A review response cut off inside the command releases nothing | variant |
| A failing review card releases no candidate | A review response cut off inside the command releases nothing | boundary |

- `variant`: same invariant (release nothing) with a distinct input shape.
- `boundary`: different failure behaviours that share only harness Given/Then
  text; the gate's shape heuristic crosses a scenario boundary.

No removed+added scenario pair exists: every delta scenario is `@givn.added`.

## Split-or-keep

No scenario exceeds the deterministic long-scenario threshold; `givn lint`
reports no long-scenario finding, so no split-or-keep decision is required.

## Advisory subset findings

The five advisory `[SUBST]` findings compare delta scenarios against permanent
scenarios that share harness text; their observables differ and no
consolidation is warranted inside this change:

| Scenario A (subset) | Scenario B (delta) | Disposition |
|---|---|---|
| A command beginning with a dash passes after the option terminator | An explanation response with literal line breaks in its values is read as a structured response | boundary |
| The review-panel switches are inert for explain | An explanation response with literal line breaks in its values is read as a structured response | boundary |
| A command beginning with a dash passes after the option terminator | An explanation response cut off after a complete command keeps the command reviewable | boundary |
| The review-panel switches are inert for explain | An explanation response cut off after a complete command keeps the command reviewable | boundary |
| Developer rejects a candidate and regenerates with another model | Verbose review prints the raw provider response after the surface closes | variant |

## Coverage classification

Measurement: `coverage/cobertura-coverage.xml`, freshly produced by
`./merge-coverages.sh` (13:54) from `coverage/non-e2e-cobertura.xml` (11:11)
and `coverage/e2e-cobertura.xml` (11:12), both written by
`./measure-coverage.sh` after the last code commit (`b0a3763`, 11:07). Merged
Cobertura line rate: **85.0%** (20,311/23,884). Changed files:

| File | Covered/valid |
|---|---|
| `src/review/diagnostics.rs` | 38/39 (97.4%) |
| `src/review/response.rs` | 673/890 (75.6%) |
| `src/review/card.rs` | 698/864 (80.8%) |
| `src/main.rs` | 794/1224 (64.9%) |
| `src/config/mod.rs` | 293/384 (76.3%) |
| `src/output/render.rs` | 76/93 (81.7%) |
| `src/provider/openai_compat.rs` | 250/322 (77.6%) |

Gaps in code added or changed by this change, classified under the three
buckets:

| Region | Bucket | Disposition |
|---|---|---|
| `repair_json_control_characters` unescaped `\r` and generic C0 arms (`src/review/response.rs:301,303-304`) | 2 — missing test coverage | Closed: the primitive's unit test now feeds `\n`, `\r`, `\t`, and a generic C0 byte (`\u{1}`) inside a value and asserts the parsed purpose; `cargo test --locked --lib --features test-support review::response` → 24 passed. The proposal promised carriage returns; the scenario-level observable was already covered for line breaks and tabs. No RED was possible because the repair behavior pre-existed from S1 — the added test is the missing regression guard for the remaining arms |
| `repair_json_control_characters` escaped-branch arms (`:281-285`) and rare `unescape_json_string` arms (`:200-216`) | 3 — hard to test | Defensive arms for a raw control character preceded by a backslash and for escape forms (`\\`, `\/`, `\uXXXX`, unknown, trailing) that no provider fixture produces; the covered `\n`/`\t`/`"` arms exercise the same mechanism, and the scanner only decodes escapes the provider itself wrote |
| `recover_command_value` fallback and attribution edges (`:409,430-431,438,513`) | 3 — hard to test | llvm-cov attributes uncovered branch edges to the delimiter filter and the early-return paths; the recovery, refusal, and invalid-JSON outcomes are scenario-covered |
| Explain-path capture-failure warning (`src/main.rs:992-994`) | 3 — hard to test | The best-effort capture contract is scenario-covered on the review path (S8) through the same helper and the same blocked state directory; this branch differs only in the explain caller's warning prefix |
| Direct-review no-command diagnostic branch (`src/main.rs:1086-1094`) | 3 — hard to test | The release-nothing outcome is scenario-covered through the widget path (S4 and the permanent payload-without-command scenario); the branch is the direct path's terminal diagnostic and exit, reachable only by PTY fault injection |
| Regeneration capture (`src/main.rs:1254-1261`) | 3 — hard to test | The capture guard and helper are covered by S7/S9; the regeneration caller reuses them and is exercised by the permanent regeneration scenarios with a usable response; reaching it with an unusable regeneration response requires a reject→regenerate PTY fault fixture |
| `config::xdg_state_dir` HOME and `.` fallbacks (`src/config/mod.rs:13-16`) | 3 — hard to test | Mutating the process-global `HOME` in a parallel test binary would be racy; the `$XDG_STATE_HOME` path and the state-file resolver are covered by unit tests and S7/S14 |
| Pre-existing uncovered branches (setup wizard, terminal open/read, provider error mapping, card render variants) | 3 — hard to test | Unchanged by this change; classified identically in the prior archive receipts |

No bucket-1 dead code was found: every added helper is called from a covered
path. No coverage gap is excused by a fourth category.

## Arc42 implementation conformance

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| Requirement 41 (tolerant reading, named reason, capture, `-v`) | `docs/arc42/01-introduction-and-goals.md:41` | Affected | Overview; Purpose reason wording | S1-S14 | Scenarios and `card_reason()`/capture/verbose code | Yes |
| XDG state directory constraint | `docs/arc42/02-architecture-constraints.md:15` | Affected | State-file decision | S7 | `config::xdg_state_dir`/`unusable_response_path` | Yes |
| Structured-response constraint gains tolerant reading | `docs/arc42/02-architecture-constraints.md:30` | Affected | Repair/recovery/refuse | S1-S4 | `parse_structured_review_response`, `recover_command_value`, `looks_like_review_payload` | Yes |
| State-file interface and explanation entry point | `docs/arc42/03-context-and-scope.md:60,73,82` | Affected | State file; explain return | S7, S13, S14 | `capture_unusable_response`, explain capture/path naming | Yes |
| Solution strategy bullet and review-response row | `docs/arc42/04-solution-strategy.md:30,61` | Affected | Overview; Purpose reason wording | S1-S14 | Same as above | Yes |
| CLI and structured-response rows; new Review diagnostics row | `docs/arc42/05-building-block-view.md:48,58,59` | Affected | Architecture Impact | S6, S7, S13 | `print_raw_response`, `capture_unusable_response`, main.rs wiring | Yes |
| Review and explain runtime sequences | `docs/arc42/06-runtime-view.md:110-151,293-342` | Affected | Architecture Impact | S1-S14 | Scenario executions | Yes |
| Provider response diagnostics section; verbose mode | `docs/arc42/08-crosscutting-concepts.md:156-178,253-256` | Affected | Verbose and capture decisions | S6, S7, S13, S14 | Diagnostics scenarios | Yes |
| QS-072, QS-077, QS-080 | `docs/arc42/10-quality-requirements.md:61,63,147,151,152` | Affected | Purpose reason table | S1-S14 | Scenario assertions | Yes |
| R-072, R-089 to R-091 | `docs/arc42/11-risks-and-technical-debt.md:75,91-93` | Affected | Completion cap; state-file privacy | S7, S14 | Design open questions; code | Yes |
| Glossary terms | `docs/arc42/12-glossary.md:114-121` | Affected | Ubiquitous language | All | Terms reused in specs, design, code | Yes |
| Chapters 07 and 09 | n/a | Unaffected | ADR `NOT_QUALIFIED` | n/a | No deployment fact; no ADR file or register change | Yes |

ARC42 CONFORMANCE: CLEAN

## Ubiquitous language conformance

The change reuses the glossary's exact terms: Provider response, Unusable
provider response, Purpose reason, Unusable-response capture, Response repair,
Response recovery, Structured review response, Purpose status, and XDG. The
spec scenarios, the design, and the code use those terms; no synonym drift and
no anti-term (`candidate` on the explain path, generic "list"/"item") appears in
the changed surfaces.

UBIQUITOUS LANGUAGE: CLEAN

## Use-case and Persona conformance

The owning use case is `use-shell`. The delta adds two Extensions (unusable
explanation response; unusable review response) that the implementation
preserves: the provider-written command stays reviewable, purposes stay
unavailable with a named reason, nothing is released without acceptance, the
explained command is never replaced or executed, and disabled/non-review
behavior is untouched. Actors remain `Shell user`, `Terminal developer`, and
`Shell line editor`; the confirmed Persona `terminal-developer--interactive`
stays a review lens and is not used as a Gherkin actor. No Persona was invented
or promoted.

## Deferrals

None. No mandatory check was deferred, so no operator decision is required.

## README impact decision

README-IMPACT: updated - Usage

The README's review-surface and explain sections now document tolerant reading,
the named reason in the card, the state file with its path, and `-v`.

## Verification runs

- `./run-tests.sh` → exit 0; 24 features, 259 scenarios (259 passed), 1573 steps
- `./run-tests.sh --e2e` → exit 0; 25 features, 89 scenarios (89 passed), 682 steps
- `cargo test --locked --lib --features test-support review::response` → 24 passed
- `cargo run --release -- --help` → exit 0 (declared local run command)
- `givn lint --change robust-review-response-recovery` → exit 0, advisories dispositioned above
- Coverage: merged report fresh (13:54) from both configured measure outputs; non-zero production coverage on every changed file

The archive verification receipt for this change does not exist yet; the
per-scope counts above are the execution evidence available at review time and
the archive will publish the authoritative receipt.

## Sign-off

- Fabrication audit: clean; the one design/implementation drift (reason table)
  was fixed and re-reviewed before sign-off.
- Every checked task has a verified commit, and every scenario-only commit
  documents the shared-behavior commit that carries its production code.
- Every promised component exists.
- Strict-mode proof present and passing.
- `verify.command` and `verify.e2e_command` are distinct and both exit 0; the
  e2e scope is strictly smaller (89 vs 259).
- Coverage classification uses the most recent measured evidence; every gap is
  classified under the three buckets and the bucket-2 gap is closed.
- Dead code: none found. Missing tests: added. Hard-to-test gaps: justified.
- No `@wip` tags remain; the spec contains no implementation-layer detail.
- No interaction was added or changed; every touched inventory action keeps its
  `@e2e` scenario and every delta scenario is a regular variant.
- No finding was excused with a classification outside the three buckets.
- README impact recorded and applied.
- No deferral without an operator decision.

REVIEW: PASS
