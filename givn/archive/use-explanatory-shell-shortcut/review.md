# Review: use-explanatory-shell-shortcut

## Fabrication audit

| # | Check | Result |
|---:|---|---|
| 0 | `@e2e` tag integrity | PASS — all four delta `@e2e` scenarios carry `@givn.added @e2e` in tracked state; no scenario lost `@e2e`; `givn lint` reports clean with the four inventory titles intact |
| 1 | Empty/no-op step bodies | PASS — 0 empty bodies across `tests/steps/interactive_shell_shortcut_steps.rs` and `tests/steps/interactive_shell_shortcut_e2e_steps.rs`; no `unimplemented!`/`todo!`/bare `return` remains |
| 2 | Checked tasks have commits touching production | PASS with documented notes — scenario 3 commit `b574ac6` introduces `src/review`; several scenarios (5, 6, 7, 11, 12, 13, 14, 23, 26, 30) exercise behavior introduced in earlier commits and record that explicitly in tasks.md; every scenario task records its atomic commit hash |
| 3 | design.md components exist | PASS — `src/review/{flow,response,buffer,adapter,routing,panel}.rs`, buffered sink wired in `src/main.rs`, `[review]` configuration and override in `src/config/types.rs`, `exec::execute` in `src/exec.rs`, deterministic terminal writer and PTY seam in `tests/steps/mod.rs` |
| 4 | Strict-mode proof | PASS — tasks.md records the scenario-3 non-zero proof (exit 101) and the E2E non-zero proof (exit 101); both stub captures are pasted |
| 5 | `@e2e` Then steps assert real interface | PASS — Ctrl-W scenarios assert the PTY-visible buffer and Bash history plus marker-file absence; direct and `-x` scenarios assert the captured command-output channel from the real subprocess, not repository state |
| 6 | Browser-UI check | N/A — CLI/terminal capability |
| 7 | `verify.e2e_command` binding | PASS — `givn/commands.yaml` binds `verify.e2e_command: "./run-tests.sh --e2e"` and `verify.command: "./run-tests.sh"`; the E2E steps live in the registered separate file `tests/steps/interactive_shell_shortcut_e2e_steps.rs`; no competing E2E implementation exists in the tree |
| 8 | One `@e2e` per distinct action | PASS — four normalized inventory actions, four `@e2e` scenarios; everything else is a variant/rule scenario |
| 9 | Local runnability | PASS — `run-tests.sh --e2e` builds the locked default and `test-support` binaries, starts the `httpmock` loopback provider, and drives real PTYs; no live provider or external network |
| 10 | `verify.command` != `verify.e2e_command` | PASS — 181 full scenarios vs 81 E2E scenarios; the E2E runner is a strict subset |
| 11 | Implementation vs design.md | PASS after amendment — the only deviation (controlling-terminal eligibility without requiring terminal stdout) is recorded in `design.md` and in the `design-review.md` Post-Review Amendment before this sign-off; the reviewed channel contract is preserved |
| 13 | Interaction coverage cross-reference | PASS — see matrix below |
| 14 | Coverage measurement validity | PASS — `measure-coverage.sh` instruments both `watn` binaries and the Gherkin runner with `cargo llvm-cov`, merges per-process profraws via `merge-coverages.sh`, and exercised `src/review` paths report non-zero line coverage |

### Interaction coverage matrix

| Inventory entry (`usecase.md`) | `@e2e` scenario | Driving mechanism in the E2E step file |
|---|---|---|
| review and accept a generated candidate from Ctrl-W | Developer accepts an explained candidate from Ctrl-W | Real Bash PTY (`start_pty_command`) runs the installed widget with the real `watn` on PATH; `pty_write` sends `Ctrl-W` and `Enter`; buffer and history read back from the PTY markers |
| cancel a candidate review from Ctrl-W | Developer cancels a review without changing the shell buffer | Real Bash PTY runs the installed widget; `pty_write` sends `Escape`; unchanged buffer/history read back from the PTY markers |
| review and accept a direct interactive request | Developer accepts a candidate from an interactive terminal request | Real `watn` subprocess in a PTY with stdout captured to a file; `pty_write` sends `Enter`; command-output channel asserted from the file |
| review and execute an accepted eligible `-x` candidate | Developer accepts an eligible `-x` candidate and it executes once | Real `watn -x` subprocess in a PTY with stdout captured; acceptance executes once and no second confirmation appears in the PTY text |

The design matrix rows match the inventory one-to-one; no interaction is
unmapped or double-covered.

## Use-case and Persona conformance

`usecase.md` actors are `Terminal developer` and `Shell line editor`; the
confirmed Persona `terminal-developer--interactive` remains a review lens and
was not used as a Gherkin actor. The minimal guarantee (no candidate changes or
executes without explicit final acceptance) and the success guarantee (the
developer understands and refines a candidate, then places exactly the accepted
candidate into the shell buffer) are preserved by the implementation and by the
acceptance/cancellation/failure scenarios. Capability ownership stays in
`use-shell` / `interactive-shell-shortcut`; no new capability root was created.

## Arc42 implementation conformance

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| Goals: understandable terminal action | `docs/arc42/01-introduction-and-goals.md` | Affected | Use-case traceability row 10 | Scenarios 3, 10-12 | `src/review/flow.rs`, `src/review/response.rs`, panel rendering | Yes |
| Constraints: inline, no alternate screen, channel separation | `docs/arc42/02-architecture-constraints.md` | Affected | Stream Ordering And Output Channels | Scenario 3, 30; E2E channel assertions | `ControllingTerminal` inline rendering; stdout-only candidate | Yes |
| Runtime: buffering, progress-first, lifecycle, routing | `docs/arc42/06-runtime-view.md` | Affected | Candidate Lifecycle; Consumer Routing | Scenarios 4-9, 22-29; E2E | `ReviewBuffer`, `run_review_path`, `request_route`, `exec::execute` | Yes |
| Cross-cutting: validation, terminal restore, config precedence, keyboard | `docs/arc42/08-crosscutting-concepts.md` | Affected | Presentation And Keyboard Contract; Configuration Seams | Scenarios 8, 9, 16-19, 29; unit tests | `sanitize_terminal_text`, focus state machine, `review_panel_enabled` | Yes |
| ADR-0015 amendment: review-mode buffered sink | `docs/arc42/09-architecture-decisions.md`, `docs/adr/0015-...md` | Affected | ADR Qualification: AMEND_ADR | Scenario 4 | `ReviewBuffer` gates opening on `[DONE]`; non-review stream unchanged | Yes |
| Quality: review usability, purpose correctness, failure recovery | `docs/arc42/10-quality-requirements.md` | Affected | Failure Outcomes table | Scenarios 7, 10-15 | Purpose validation and `purpose-unavailable` fallback | Yes |
| Risks: response drift, buffering latency, repaint, channel contamination | `docs/arc42/11-risks-and-technical-debt.md` | Affected | Risks And Mitigations | Scenarios 4, 12, 29, 30; E2E | Buffered sink, unsupported-flow markers, cleanup assertions | Yes |
| Glossary: Candidate, Command flow, Review surface, Review decision, Presentation adapter, channels | `docs/arc42/12-glossary.md` | Affected | Canonical vocabulary | Scenarios 3-30 | Types and terms reuse the glossary vocabulary | Yes |

Chapter 07 (deployment) is unaffected: no deployment artifact or topology
changed.

ARC42 CONFORMANCE: CLEAN

## Ubiquitous language conformance

Specs, design, code, and the change use the glossary terms consistently:
`Candidate`, `Intent`, `Command flow`, `Stage text`, `Stage purpose`,
`Purpose status`, `Review surface`, `Review decision`, `Presentation adapter`,
`Command-output channel`, `Controlling-terminal channel`, `Shell line-editor
buffer`. No spec or design term is missing from `docs/arc42/12-glossary.md`,
and no inconsistent synonym (for example `result`, `answer`, or ambiguous
`action`) remains in the delta specification.

UBIQUITOUS LANGUAGE: CLEAN

## README impact decision

README-IMPACT: updated - Usage, Configuration

The review surface, its keyboard contract, the `--review-panel` /
`--no-review-panel` flags, the `[review] panel` setting, and the eligible `-x`
execution rule were added to the README `Usage` and `Configuration` sections.

## Overlap dispositions

No deterministic shape matches involve this change (`givn lint` reports clean);
the table is empty.

| Finding pair | Decision |
|---|---|
| (none) | — |

## Split-or-keep

No scenario exceeds the deterministic long-scenario threshold; no split-or-keep
decision is required for this change.

## Coverage classification

Measurement: `./measure-coverage.sh` followed by `./merge-coverages.sh`
(both exit 0). Merged Cobertura line rate: **92.4%** overall. The Gherkin
runner and both spawned `watn` binaries are instrumented; per-process profraws
are merged before export.

New/modified production coverage:

| Region | Coverage |
|---|---|
| `src/review/response.rs` | 200/200 (100%) |
| `src/review/adapter.rs` | 53/53 (100%) |
| `src/review/buffer.rs` | 35/35 (100%) |
| `src/review/routing.rs` | 43/43 (100%) |
| `src/review/flow.rs` | 313/315 (99.4%) |
| `src/review/panel.rs` | 712/735 (96.9%) |
| `src/config/types.rs` | 150/151 (99.3%) |

Remaining gaps, classified:

| Region | Bucket | Disposition |
|---|---|---|
| `panel.rs` `ControllingTerminal::open` error branches (unusable terminal, too-small layout) and `restore` error-aggregation branches | 3 — hard to test | These execute only on terminal-open failure or simultaneous multi-write failures. The observable portable-failure outcome (`Unavailable`, preserve input, release nothing) is scenario 14 and the renderer failure seam in the design. |
| `panel.rs` `restore` rollback after `begin()` failure | 3 — hard to test | Requires a failing cursor write on a real descriptor; the deterministic writer cannot fail without a custom writer injected through the production seam. |
| `panel.rs` render line-attribution artifacts (queue continuation) | 3 — hard to test | Statement is exercised by every render assertion; llvm-cov attributes the line to an uncovered closure branch. |
| `flow.rs` defensive `push_stage` empty-range return and escaped-token branch | 3 — hard to test | Unreachable through `derive_command_flow` for any non-empty command; retained as defensive guards for future grammar growth. |
| `src/main.rs` setup/error branches and interactive review I/O failure branches | 3 — hard to test | Pre-existing setup wizard branches plus terminal-open/read failure paths. The review outcomes they guard are classified in the design failure table and exercised at the outcome level. |
| `src/exec.rs` interactive raw-mode confirmation key handling | 3 — hard to test | Pre-existing interactive confirmation path; eligible `-x` in review mode uses `exec::execute` and is covered by scenario 21 (no second confirmation). |
| `config/types.rs` line 213 empty reasoning-effort fallback | 3 — hard to test (pre-existing) | Unchanged code from before this change; requires an empty `[tiers.reasoning]` string, which the setup wizard never writes. |

No bucket-1 dead code remains: the unreachable `apply_response` fallback in
`response.rs` was deleted during review. No bucket-2 missing test coverage
remains: editor rendering, reverse focus cycling, purpose labels, validation
rejections, xargs/escape/subshell flow edges, and review config overrides were
added as unit tests and committed in `edea13e` and the follow-up test commit `8b52369`.

## Verification runs

- `./run-tests.sh` → exit 0; 21 features, 181 scenarios, 1087 steps passed
- `./run-tests.sh --e2e` → exit 0; 25 features, 81 scenarios, 592 steps passed
- `./measure-coverage.sh` → both Cobertura reports generated, exit 0
- `./merge-coverages.sh` → merged report refreshed, exit 0
- `givn lint --change use-explanatory-shell-shortcut` → clean, exit 0
- `cargo check --locked` → clean
- `cargo test --locked --lib --features test-support` → 56 passed

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
- Canonical E2E policy read; every normalized inventory action has exactly one
  `@e2e` scenario.
- Local run command starts the full stack including the loopback provider twin.

REVIEW: PASS
