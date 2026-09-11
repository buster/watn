# Review: refine-review-panel-views

## Fabrication audit

| # | Check | Result |
|---:|---|---|
| 0 | `@e2e` tag integrity | PASS — three delta scenarios carry `@e2e`; git history shows only `@wip` removals, never an `@e2e` removal |
| 1 | Empty/no-op step bodies | PASS — no empty bodies in `tests/steps/interactive_shell_shortcut_steps.rs` or `interactive_shell_shortcut_e2e_steps.rs`; all new steps assert |
| 2 | Checked tasks have commits touching production | PASS — see the commit mapping below; the shared renderer landed in `d51a93f`, the disable/flag CLI in `730210b`/`0781c49`, and the review-phase fixes in `6535884`, `91e729b`, `35ab5b1` |
| 3 | design.md components exist | PASS — `CommandStage.separator`, `following_separator`, `ReviewPanelState.details`, `model_short_name`, `stack_window_rows`, `stage_group_rows`, `wrap_stage`, `Ink::purpose_marker`/`purpose`, `unicode-width` padding, disable release, flag-only branch |
| 4 | Strict-mode proof present | PASS — tasks.md setup records exit 1 with an undefined step under `.fail_on_skipped()` |
| 5 | E2E Then steps assert the real interface | PASS — PTY snapshots (`?`, `D`, `Intent`, hint), stdout file, persisted config, exit status; no repository-only assertions |
| 6 | Browser-UI driver check | N/A — CLI capability |
| 7 | `verify.e2e_command` binding and duplicate implementations | PASS — `./run-tests.sh --e2e` selects `@e2e`; the tree contains one step implementation per capability |
| 8 | E2E action normalization | PASS — three new actions: view toggle, permanent disable, flag-only switch; the modified permanent scenarios reuse the same actions instead of adding variants |
| 9 | `verify.command` vs `verify.e2e_command` | PASS — distinct strings; 223 regular vs 86 e2e scenarios |
| 10 | Implementation vs design.md | PASS — view promotion on `e`/`r`, `D` in both views, `⋮`/`…` marker split, clean-machine persistence, `-x` release without execution, unicode-width; the only test-glue refinements (ANSI-stripped assertions) do not change the design |
| 11 | Interaction coverage cross-reference | PASS — see below |
| 12 | Coverage measurement validity | PASS — see Coverage classification |
| 13 | Local run command and twins | PASS — `cargo run --release -- --help`; the in-process mock provider server is the only external twin and is started per test |
| 14 | README impact | `README-IMPACT: updated - Review surface` |

### Checked-task commit mapping

The renderer is one cohesive unit shared by the first eight scenarios; its
production change is `d51a93f`. Each scenario still has its own commit with
its targeted run evidence and its step/permanent-spec synchronization.

| Scenario group | Production commit | Verification commit |
|---|---|---|
| Simple model name, stage stack, separators, long stage, selected stage, purpose, view toggle, truncation, narrow window | `d51a93f` | `5575ed7`, `cbbbba8`, `e30e505`, `0e0a9de`, `74d89a9`, `e649925`, `22a66be` |
| Rephrasing, higher-tier, decision-key emphasis, exposed shortcuts (view sync) | `d51a93f` | `b52278a`, `3c1e9e4`, `3a88b06`, `47a547f` |
| E2E view toggle | `d51a93f` | `f51f8a9` |
| Permanent disable release | `730210b` | `730210b` |
| Flag-only switch | `0781c49` | `0781c49` |
| Review-phase piped request and clean-machine switch | `91e729b` | `91e729b` |
| Raw-candidate and replacement visibility under colored stack rows | `6535884` | `6535884` |
| Non-selected unsupported flow marker | `35ab5b1` | `35ab5b1` |

### Interaction coverage cross-reference

| Inventory entry (change-side usecase.md) | Design matrix row | @e2e scenario | Driving mechanism in steps |
|---|---|---|---|
| validate generated shell configuration | yes | Generated Bash, Zsh, and Fish configurations pass shell syntax checks | real `bash -n`, `zsh -n`, `fish -n` |
| inspect generated Bash widget | yes | The generated Bash widget keeps the request visible and does not evaluate the command | real Bash subprocess with the generated widget |
| use Fish Ctrl-W shortcut | yes | Fish replaces the buffer with the generated command after Ctrl-W | real Fish subprocess |
| review and accept a generated candidate from Ctrl-W | yes | Developer accepts an explained candidate from Ctrl-W | Bash widget in a PTY, `⏎`, Enter |
| switch to the detailed review view during a Ctrl-W review | yes | Developer switches to the detailed review view during Ctrl-W review | Bash widget in a PTY, `?`, `Intent` wait |
| disable the review surface from a candidate review | yes | The panel can permanently disable the review | watn in a PTY, `?`, `D`, stdout file, PTY stderr, config |
| configure the review surface from the command line without a request | yes | The review surface switches configure without a request | real subprocess with empty stdin, config assertions |
| cancel a candidate review from Ctrl-W | yes | Developer cancels a review without changing the shell buffer | Bash widget in a PTY, Esc |
| review and accept a direct interactive request | yes | Developer accepts a candidate from an interactive terminal request | watn in a PTY, stdout file |
| review and execute an accepted eligible -x candidate | yes | Developer accepts an eligible -x candidate and it executes once | watn `-x` in a PTY |
| reject a candidate and regenerate with another model | yes | Developer rejects a candidate and regenerates with another model | watn in a PTY, `r`, tier key |
| generate Bash completions | yes | Built Bash completion generation emits the current command tree | real subprocess `watn completions bash` |

No inventory entry is unmapped, no matrix row is missing an `@e2e` scenario,
and every driver is the real interface named by the matrix.

## Arc42 implementation conformance

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| Developer keys, flag-only switch, command-output release | `docs/arc42/03-context-and-scope.md` | Affected | Disable And Flag-Only Flow | Scenarios 1, 14, 15 | `panel.rs` key map, `main.rs` release/flag branch | Yes |
| Two views, model short name, stack, disable release strategy | `docs/arc42/04-solution-strategy.md` | Affected | Rendering | Scenario group | `card.rs` simple/detailed layouts | Yes |
| Review surface and renderer responsibilities | `docs/arc42/05-building-block-view.md` | Affected | Rendering | Scenario group | `card.rs`, `panel.rs`, `flow.rs` | Yes |
| Review narrative, lifecycle, routing | `docs/arc42/06-runtime-view.md` | Affected | View State And Input Mapping | Scenarios 1-15 | `handle_review_key`, `main.rs` | Yes |
| ADR-0015 disable-release amendment | `docs/adr/0015-...md`, `docs/arc42/09-architecture-decisions.md` | Affected | ADR Qualification And Routing | Scenario 14 | `main.rs` release branch | Yes |
| QS-067, QS-070, QS-073 | `docs/arc42/10-quality-requirements.md` | Affected | Rendering, Failure Outcomes | Scenarios 1-15 | renders and keys verified by scenarios | Yes |
| R-073 wording and R-078 | `docs/arc42/11-risks-and-technical-debt.md` | Affected | ADR section | Scenario 14 | release never executes | Yes |
| Five new/updated glossary terms | `docs/arc42/12-glossary.md` | Affected | View State And Input Mapping | n/a | exact terms used in specs/design/code | Yes |
| Chapters 01, 02, 07, 08 | n/a | Unaffected | — | — | No goals, constraints, deployment or crosscutting change | Yes |

ARC42 CONFORMANCE: CLEAN

## Ubiquitous language conformance

`Simple review view`, `Detailed review view`, `Review view toggle`, `Stage
stack`, and `Model short name` are recorded in `docs/arc42/12-glossary.md` and
used consistently by proposal, specs, design, README, and code (`details`,
`stack`, `model_short_name`). `Review decision`, `Review outcome`, and
`Command-output channel` were updated to the implemented behavior. No term is
undefined or reused with a second meaning; no Persona was invented or promoted.

UBIQUITOUS LANGUAGE: CLEAN

## Overlap dispositions

| Scenario A | Scenario B | Disposition |
|---|---|---|
| Enabling the review panel from the command line persists it | A review switch keeps a piped request | variant |

The shape match shares only the flag-plus-request setup; the piped scenario
proves a stdin question is not swallowed by the flag-only branch, while the
positional scenario proves the flag persists before generation.

Two advisory subset notices remain and are accepted as boundaries: "Developer
switches to the detailed review view during Ctrl-W review" is a subset shape of
"Developer accepts an explained candidate from Ctrl-W" but adds the `?`
decision and the detailed-view assertions; "Developer rejects a candidate and
regenerates with another model" shares only generic direct-request setup with
"The panel can permanently disable the review", which disables, releases, and
exits.

Removed+added: the removed "The card shows one stage at a time and moves stages
with the arrow keys" is superseded by the added stack scenarios, which own the
new observable contract (all stages visible, selected stage marked, purpose
follows navigation).

## Split-or-keep

No scenario exceeds the deterministic long-scenario threshold; no split is
required.

## Coverage classification

Measurement: `./measure-coverage.sh` then `./merge-coverages.sh`, both exit 0.
The runner, both instrumented binaries, and the unit tests are collected; the
merged report was freshly produced by the merge command. Merged Cobertura line
rate: **92.02%** (223 regular and 86 e2e scenarios, all passed). A known
exercised production path (`main.rs` review routing) is non-zero.

| Region | Coverage |
|---|---|
| `src/review/card.rs` | 739/740 (99.9%) |
| `src/review/flow.rs` | 323/325 (99.4%) |
| `src/review/panel.rs` | 860/894 (96.2%) |
| `src/main.rs` | 564/762 (74.0%) |

Gap classification (only lines touched or newly exercised by this change are
attributed here; pre-existing unrelated error paths remain as classified by
earlier reviews):

| Region | Bucket | Disposition |
|---|---|---|
| `card.rs:911` test-macro continuation | 3 — instrumentation | Assertion body executed; only the macro terminator region is unmapped |
| `flow.rs:110` blank-segment early return | 3 — defensive | The `ls ; ` trailing-segment flow test exercises the behavior; the early return is a guard region |
| `flow.rs:208` escaped-token start bookkeeping | 3 — defensive | The `escaped_start` flow test exercises the path; the guard assignment region is unmapped |
| `panel.rs` chooser/query/terminal guards (34 lines) | 3 — pre-existing defensive | Classified in `simplify-review-card-controls`; untouched by this change |
| `main.rs:195-198` TTY sub-branch of the flag-only path | 3 — environment | The pipe harness cannot provide a TTY; the equivalent empty-pipe branch is covered by the e2e scenario and the TTY branch is one call to the same function |
| `main.rs` persist-failure branches (flag-only and disable) | 3 — fault injection | Requires `WATN_TEST_FAIL_CONFIG_WRITE` on a subprocess write; the atomic failure path is owned by ADR-0024 and its config unit tests, and both branches are straight error propagation |
| Remaining `main.rs` zeros | 3 — pre-existing | Setup wizard, provider error, and terminal failure paths unrelated to this change |

No dead code, no unclassified missing test coverage. The review-phase addition
`main.rs` clean-machine branch is now covered by the clean-machine e2e step
(hits recorded in the merged report).

## Verification runs

- `./run-tests.sh` → exit 0; 21 features, 223 scenarios, 1363 steps passed
- `./run-tests.sh --e2e` → exit 0; 25 features, 86 scenarios, 637 steps passed
- `./measure-coverage.sh` → both source reports generated, exit 0
- `./merge-coverages.sh` → merged report refreshed, exit 0
- `givn lint --change refine-review-panel-views` → exit 0, clean; two advisory subset notices dispositioned above
- `cargo check --locked` → clean
- `cargo test --locked --lib` → 84 passed
- Head commits: `d51a93f`, `730210b`, `0781c49`, `6535884`, `91e729b`, `35ab5b1`

## README impact decision

README-IMPACT: updated - Review surface

The Review surface section now documents the simple default view, the stage
stack, the `d`/`?` toggle, the detailed view, the `D` disable release, and the
flag-only persistence behavior.

## Sign-off

- Fabrication audit: clean.
- Every checked task has a verified commit; the shared renderer mapping is recorded above.
- Every promised component exists.
- Strict-mode proof present and passing.
- Both runners GREEN; e2e scope is a strict subset (86 < 223).
- Coverage measured across the runner, both binaries, and the unit tests; every gap classified and resolved.
- No `@wip` tags remain.
- Interaction coverage verified against the durable inventory; all driving mechanisms are real.
- Durable arc42 chapters and the ADR-0015 amendment match the implementation.
- Local run command starts the CLI with the in-process provider twin.
- No finding was excused with a classification outside the three buckets.

REVIEW: PASS
