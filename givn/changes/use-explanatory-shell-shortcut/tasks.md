# Tasks: use-explanatory-shell-shortcut

All tasks are intentionally unchecked. Do not check a task until its command,
runner output, production-file list, and (for scenario tasks) commit hash have
been pasted into this file. Implement the non-`@e2e` scenarios first, in the
order below. Implement the `@e2e` scenarios only after every non-`@e2e`
scenario is GREEN and committed. Do not remove an `@e2e` tag.

## Setup

- [x] Confirm the hardened contract before implementation. The permanent owner
  is `use-shell`; the changed capability is `interactive-shell-shortcut`.
  `review-command-candidate` and `route-reviewed-command` remain design
  subfunctions, not new capability roots. The confirmed Persona
  `terminal-developer--interactive` is a review lens, not a Gherkin actor;
  use `Terminal developer` and `Shell line editor` as the interaction actors.
  Evidence:
  - Read: `proposal.md`, `specs/use-shell/usecase.md`,
    `specs/use-shell/interactive-shell-shortcut.feature`, `design.md`,
    `design-review.md` (DESIGN-REVIEW: PASS),
    `givn/ideation/terminal-wow-factor/handoff.md`,
    `givn/personas/terminal-developer--interactive.md`.
  - Permanent owner `use-shell` confirmed in `givn/specs/use-shell/usecase.md`
    (Capabilities: `interactive-shell-shortcut`, `shell-completions`); changed
    capability `interactive-shell-shortcut`; subfunctions preserved per
    `design.md` "Scope And Ownership".
  - Actors `Terminal developer` and `Shell line editor` in
    `specs/use-shell/usecase.md`; Persona
    `terminal-developer--interactive` is a review lens only.
  - Artifacts committed at `5c04333` and `9fc1378`.

- [x] Verify the User Interaction Inventory against the Interactive Coverage
  Matrix before implementing any scenario. The owning change use case declares
  exactly these four normalized interactions, and each must have exactly one
  matching `@e2e` scenario title in
  `givn/changes/use-explanatory-shell-shortcut/specs/use-shell/interactive-shell-shortcut.feature`:
  `review and accept a generated candidate from Ctrl-W` -> `Developer accepts
  an explained candidate from Ctrl-W`; `cancel a candidate review from Ctrl-W`
  -> `Developer cancels a review without changing the shell buffer`; `review
  and accept a direct interactive request` -> `Developer accepts a candidate
  from an interactive terminal request`; `review and execute an accepted
  eligible -x candidate` -> `Developer accepts an eligible -x candidate and it
  executes once`. Confirm every matrix row uses a real CLI boundary: a real
  Bash PTY for the two Ctrl-W interactions, a real terminal `watn` subprocess
  for the direct interactive request, and a real PTY `watn -x` subprocess for
  eligible execution. Confirm shell completions are absent from this change's
  inventory and matrix. Evidence:

  Inventory (`specs/use-shell/usecase.md`):

  | Capability | Consumer action | E2E scenario |
  |---|---|---|
  | interactive-shell-shortcut | review and accept a generated candidate from Ctrl-W | Developer accepts an explained candidate from Ctrl-W |
  | interactive-shell-shortcut | cancel a candidate review from Ctrl-W | Developer cancels a review without changing the shell buffer |
  | interactive-shell-shortcut | review and accept a direct interactive request | Developer accepts a candidate from an interactive terminal request |
  | interactive-shell-shortcut | review and execute an accepted eligible `-x` candidate | Developer accepts an eligible `-x` candidate and it executes once |

  Matrix (`design.md` Interaction Coverage Matrix): four rows with real
  interfaces — Bash PTY `Ctrl-W and Enter`; Bash PTY `Ctrl-W and Escape`; real
  `watn` subprocess with terminal stdin/stdout/stderr; real PTY `watn -x`
  subprocess.

  Exact title comparison (`rg -n "Scenario:"` on the delta feature):

  ```text
  5:  Scenario: Developer accepts an explained candidate from Ctrl-W
  15:  Scenario: Developer cancels a review without changing the shell buffer
  194:  Scenario: Developer accepts a candidate from an interactive terminal request
  202:  Scenario: Developer accepts an eligible -x candidate and it executes once
  ```

  All four are tagged `@e2e` (feature lines 4, 14, 193, 201). Shell
  completions are absent: the inventory above contains only
  `interactive-shell-shortcut` rows, and the design matrix states "Shell
  completions are intentionally absent from this change".

  `givn lint --change use-explanatory-shell-shortcut` (exit 2 at setup time;
  the 29 `[WIP]` findings are the deliberately unimplemented scenarios, and
  scenario `The review surface explains a complex command flow` is already
  de-`@wip` for the first task):

  ```text
  [WIP  ] .../interactive-shell-shortcut.feature  scenario 'Developer accepts an explained candidate from Ctrl-W' is tagged @wip
  [WIP  ] .../interactive-shell-shortcut.feature  scenario 'Developer cancels a review without changing the shell buffer' is tagged @wip
  [WIP  ] .../interactive-shell-shortcut.feature  scenario 'A complete candidate is buffered before review' is tagged @wip
  [WIP  ] .../interactive-shell-shortcut.feature  scenario 'Direct command editing preserves the original intent' is tagged @wip
  [WIP  ] .../interactive-shell-shortcut.feature  scenario 'Escape discards a direct command edit' is tagged @wip
  [WIP  ] .../interactive-shell-shortcut.feature  scenario 'Edited candidate purpose refresh failure remains reviewable' is tagged @wip
  [WIP  ] .../interactive-shell-shortcut.feature  scenario 'The compact review surface cycles three focus regions' is tagged @wip
  [WIP  ] .../interactive-shell-shortcut.feature  scenario 'Review actions use Enter and Escape' is tagged @wip
  [WIP  ] .../interactive-shell-shortcut.feature  scenario 'Stage purposes can load after a structured candidate appears' is tagged @wip
  [WIP  ] .../interactive-shell-shortcut.feature  scenario 'A command-only response shows purpose-unavailable' is tagged @wip
  [WIP  ] .../interactive-shell-shortcut.feature  scenario 'Unsupported command flow remains reviewable' is tagged @wip
  [WIP  ] .../interactive-shell-shortcut.feature  scenario 'Enhanced renderer failure falls back to the inline review surface' is tagged @wip
  [WIP  ] .../interactive-shell-shortcut.feature  scenario 'Portable review-surface failure releases no candidate' is tagged @wip
  [WIP  ] .../interactive-shell-shortcut.feature  scenario 'Provider failure preserves a selected candidate during review' is tagged @wip
  [WIP  ] .../interactive-shell-shortcut.feature  scenario 'Disabled review preserves direct Ctrl-W replacement' is tagged @wip
  [WIP  ] .../interactive-shell-shortcut.feature  scenario 'Disabled review preserves direct positional output' is tagged @wip
  [WIP  ] .../interactive-shell-shortcut.feature  scenario 'Disabled review preserves -x confirmation' is tagged @wip
  [WIP  ] .../interactive-shell-shortcut.feature  scenario 'Non-review -x preserves confirmation' is tagged @wip
  [WIP  ] .../interactive-shell-shortcut.feature  scenario 'Developer accepts a candidate from an interactive terminal request' is tagged @wip
  [WIP  ] .../interactive-shell-shortcut.feature  scenario 'Developer accepts an eligible -x candidate and it executes once' is tagged @wip
  [WIP  ] .../interactive-shell-shortcut.feature  scenario 'Rephrasing starts a new candidate cycle' is tagged @wip
  [WIP  ] .../interactive-shell-shortcut.feature  scenario 'Regeneration replaces the current candidate by default' is tagged @wip
  [WIP  ] .../interactive-shell-shortcut.feature  scenario 'Higher-tier review generates a candidate at the next configured tier' is tagged @wip
  [WIP  ] .../interactive-shell-shortcut.feature  scenario 'Highest-tier review opens explicit provider catalog model selection' is tagged @wip
  [WIP  ] .../interactive-shell-shortcut.feature  scenario 'Rejected candidate returns to the current intent' is tagged @wip
  [WIP  ] .../interactive-shell-shortcut.feature  scenario 'Retained candidates can be compared and one selected' is tagged @wip
  [WIP  ] .../interactive-shell-shortcut.feature  scenario 'Interrupting an in-progress review operation preserves the selected candidate' is tagged @wip
  [WIP  ] .../interactive-shell-shortcut.feature  scenario 'Shell repaint remains owned by the line editor after review' is tagged @wip
  [WIP  ] .../interactive-shell-shortcut.feature  scenario 'A narrow terminal keeps the inline review bounded and readable' is tagged @wip
  givn lint: 1 file(s) checked, 29 finding(s)
  ```

- [x] Verify the executable-spec runner and configure it exactly as designed.
  `tests/features_runner.rs` must collect features under `givn/specs/` and the
  active change `specs/` tree, and `tests/steps/mod.rs` must register the step
  modules. Preserve or configure `.fail_on_skipped()` on the
  `cucumber-rs` builder. `givn/commands.yaml` must contain exactly:
  `verify.command: "./run-tests.sh"` and
  `verify.e2e_command: "./run-tests.sh --e2e"`. The regular runner uses the
  `not @wip and not @e2e` tag filter; the E2E runner uses `@e2e and not @wip`.
  The exact targeted commands are `./run-tests.sh --name '<scenario title>'`
  for non-E2E scenarios and `./run-tests.sh --e2e --name '<scenario title>'`
  for E2E scenarios. Evidence:
  - `tests/features_runner.rs:17` `pub mod steps;`;
    `:158` `feature_files.extend(collect_features(&root.join("specs")));`;
    `:162-169` collects each change `specs/` tree unless `GIVN_ARCHIVE_ONLY`;
    `:183` `.fail_on_skipped()`.
  - `givn/commands.yaml`:

    ```yaml
    verify:
      command: "./run-tests.sh"
      e2e_command: "./run-tests.sh --e2e"
    ```

  - `run-tests.sh:24-27` selects `@e2e and not @wip` for E2E and
    `not @wip and not @e2e` for the regular runner; `:30-37` implements the
    exact targeted commands, including `--e2e --name '<scenario title>'`.
  - Captured output: regular full run baseline (task 6) reports
    `21 features / 156 scenarios`; E2E full run baseline reports
    `24 features / 77 scenarios`. Targeted single-scenario invocation captured
    in task 5 exits non-zero from the stub panic.

- [x] Create or extend one normal step-definition file for this capability at
  `tests/steps/interactive_shell_shortcut_steps.rs` and one separate E2E
  step-definition file at
  `tests/steps/interactive_shell_shortcut_e2e_steps.rs`. Do not put all new
  capabilities into a shared catch-all file, do not use empty bodies, bare
  `pass`, or bare `return`, and do not place E2E driver steps in the normal
  file. New RED steps in Rust use `unimplemented!()` or `todo!()` until GREEN.
  Reuse an existing step only when its observable contract is identical.
  Evidence: both files already exist and are registered for this capability:
  - `tests/steps/mod.rs:11` `pub mod interactive_shell_shortcut_steps;` — normal
    step definitions for capability `interactive-shell-shortcut` (shortcut
    installation, widget behavior, and the new review-surface steps).
  - `tests/steps/mod.rs:10` `pub mod interactive_shell_shortcut_e2e_steps;` —
    E2E driver steps for the same capability (Bash PTY, real terminal `watn`
    subprocess, PTY `watn -x`).
  - Mapping: every non-`@e2e` review scenario binds in the normal file; the
    four `@e2e` scenarios bind in the E2E file. Each scenario adds its own
    non-empty RED stubs during its RED task; no empty bodies are permitted.

- [x] Prove strict mode before scenario work. Add one temporary step binding
  for a new step in the first non-E2E scenario using `unimplemented!()`, remove
  `@wip` from that scenario only, and run
  `./run-tests.sh --name 'The review surface explains a complex command flow'`.
  The process must exit non-zero and report a failed scenario/step. If it exits
  zero, stop and fix `.fail_on_skipped()` or the step body before continuing.
  Remove only the temporary proof binding after capturing the evidence; keep
  the capability skeleton available for the scenario RED task. Evidence:
  ```text
  command: `./run-tests.sh --name 'The review surface explains a complex command flow'`
  exit: 101
  output:
  Feature: Explanatory interactive shell shortcut
    Scenario: The review surface explains a complex command flow
     ✘  Given an installed Bash shortcut and a provider candidate "git log --format='%H' --since='7 days ago' | xargs -n1 git show --stat --oneline && printf 'done'"
        Step failed:
        Defined: .../interactive-shell-shortcut.feature:26:5
        Matched: tests/steps/interactive_shell_shortcut_steps.rs:1054:1
        Step panicked. Captured output: not implemented: strict-mode proof: review surface not implemented
  [Summary]
  1 feature
  1 scenario (1 failed)
  1 step (1 failed)

  thread 'main' panicked at cucumber-0.23.0/src/cucumber.rs:1243:13:
  1 step failed
  error: test failed, to rerun pass `--test features_runner`
  ```
  The temporary binding was removed after this capture; the scenario RED task
  re-adds real stubs.

- [x] Record the baseline before scenario implementation. Run `./run-tests.sh`
  and `./run-tests.sh --e2e` with the current WIP state, and record both exit
  status and scenario counts. These commands are distinct even if the current
  counts are zero for the new WIP scenarios. Evidence:
  ```text
  regular command: `./run-tests.sh`
  regular output/count: exit 101; 21 features; 156 scenarios (155 passed,
    1 failed); 914 steps (913 passed, 1 failed). The single failure is
    `The review surface explains a complex command flow` with
    `Step doesn't match any function` (no bindings yet for this scenario).
  e2e command: `./run-tests.sh --e2e`
  e2e output/count: exit 0; 24 features; 77 scenarios (77 passed); 568 steps
    (568 passed). The four change `@e2e` scenarios are still `@wip` and are
    intentionally excluded.
  ```

## Non-E2E Scenarios

### The review surface explains a complex command flow

Domain constraints:

- A `Candidate` contains the exact command, current `Intent`, tier and
  provider/model context, locally derived `Command flow`, exact `Stage text`,
  and `Purpose status`.
- A valid structured review response has `review_version: 1`, a complete
  non-empty command, exact locally derived stage text, and model-written `Stage
  purpose` values. Watn must not invent replacement purposes locally.
- The review surface is small, transient, bounded, inline, and rendered on the
  `Controlling-terminal channel`; the `Command-output channel` remains
  reserved for an accepted Candidate.
- The explanation is advisory and never a semantic command-risk verdict.

- [x] RED: Remove `@wip` from this scenario only. Bind every new step with
  non-empty Rust bodies, using `unimplemented!()` for the review surface,
  complex-flow stage, success-branch, and model-written-purpose assertions.
  Run the exact single-scenario command; it must exit non-zero.
  ```text
  command: `./run-tests.sh --name 'The review surface explains a complex command flow'`
  output: exit 101
  Feature: Explanatory interactive shell shortcut
    Scenario: The review surface explains a complex command flow
     ✘  Given an installed Bash shortcut and a provider candidate "git log ..."
        Step failed:
        Defined: .../interactive-shell-shortcut.feature:26:5
        Matched: tests/steps/interactive_shell_shortcut_steps.rs:1054:1
        Step panicked. Captured output: not implemented
  [Summary]
  1 feature
  1 scenario (1 failed)
  1 step (1 failed)
  error: test failed, to rerun pass `--test features_runner`
  ```
- [x] GREEN: Replace the stubs with deterministic terminal-writer assertions
  for the `git log`, `xargs`, `git show`, and `printf` stages and every fixture
  purpose. Validate the structured response against exact stage text and keep
  review bytes off stdout. Implement the minimum production behavior. Compile
  first with `cargo check --locked`, then run the exact targeted command.
  Production files changed: `src/review/mod.rs`, `src/review/flow.rs`,
  `src/review/response.rs`, `src/review/panel.rs` (including making
  `ControllingTerminal::render` public so the deterministic writer seam is
  reachable), `src/lib.rs`.
  Note: the `src/review` domain module was authored during the interrupted
  pre-resume session and is introduced here, in its owning scenario's commit,
  rather than in a separate foundation commit.
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'The review surface explains a complex command flow'`
  output: cargo check finished cleanly; runner exit 0
  Feature: Explanatory interactive shell shortcut
    Scenario: The review surface explains a complex command flow
     ✔  Given an installed Bash shortcut and a provider candidate "git log ..."
     ✔  And the provider returns this structured review response:
     ✔  When I invoke Ctrl-W with current input "inspect recent commits"
     ✔  Then the review surface should show the git log stage
     ✔  And the review surface should show the xargs stage
     ✔  And the review surface should show the git show stage
     ✔  And the review surface should show the success branch
     ✔  And the review surface should show each model-written stage purpose
  [Summary]
  1 feature
  1 scenario (1 passed)
  8 steps (8 passed)
  ```
- [x] REFACTOR: Remove duplication in structured-response and stage assertions
  without changing the visible command flow or purpose contract. Rerun the
  exact targeted command and paste passing output.
  ```text
  command: `./run-tests.sh --name 'The review surface explains a complex command flow'`
  output: exit 0
  [Summary]
  1 feature
  1 scenario (1 passed)
  8 steps (8 passed)
  ```
- [x] COMMIT: Create one atomic commit covering this RED/GREEN/REFACTOR loop,
  with commit message `feat(interactive-shell-shortcut): The review surface explains a complex command flow`.
  Production files in commit: `src/review/mod.rs`, `src/review/flow.rs`,
  `src/review/response.rs`, `src/review/panel.rs`, `src/lib.rs`. Test files:
  `tests/features_runner.rs`, `tests/steps/interactive_shell_shortcut_steps.rs`.
  Commit hash: `b574ac6367b860eebc2219cd3ac36829e16cfa9b`.

### A complete candidate is buffered before review

Domain constraints:

- The existing progress line is the first user-visible feedback.
- Review-eligible Candidate text is buffered until the provider sends the exact
  `[DONE]` marker; no partial Candidate may reach stdout or the shell buffer.
- The review surface opens only after the complete Candidate is assembled and
  validated. Final acceptance is the only release gate.
- Disabled/non-review paths retain the existing incremental stream contract.

- [x] RED: Remove `@wip` from this scenario only. Add non-empty stubs for the
  pre-`[DONE]` surface, progress ordering, no-release, and post-`[DONE]`
  assertions. Run the exact targeted command; it must exit non-zero.
  ```text
  command: `./run-tests.sh --name 'A complete candidate is buffered before review'`
  output: exit 101; first stub panicked:
    ✘ Given an installed Bash shortcut and a provider that streams a candidate in multiple events
      Step panicked. Captured output: not implemented
    [Summary] 1 feature / 1 scenario (1 failed) / 1 step (1 failed)
  ```
- [x] GREEN: Add a review-mode buffered sink at the existing synchronous
  provider boundary, preserve the progress line, gate review opening on
  `[DONE]`, and release only after acceptance. Keep non-review incremental
  output unchanged. Compile with `cargo check --locked`, then run the exact
  targeted command. Production files changed: `src/review/buffer.rs` (new
  `ReviewBuffer` gating completion), `src/review/mod.rs`.
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'A complete candidate is buffered before review'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 8 steps (8 passed)
  ```
- [x] REFACTOR: Consolidate completion-boundary and buffered-sink cleanup while
  preserving the first-feedback and no-partial-release behavior. Rerun the
  exact targeted command.
  ```text
  command: `./run-tests.sh --name 'A complete candidate is buffered before review'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 8 steps (8 passed)
  ```
- [x] COMMIT: Create one atomic commit with message
  `feat(interactive-shell-shortcut): A complete candidate is buffered before review`.
  Production files in commit: `src/review/buffer.rs`, `src/review/mod.rs`.
  Test files: `tests/steps/interactive_shell_shortcut_steps.rs`, delta feature.
  Commit hash: `c391af85bf5e5c9b5819c502a9ebb873e4aa1d45`.

### Direct command editing preserves the original intent

Domain constraints:

- Direct editing creates refreshed Candidate state but never changes the
  original `Intent`.
- Enter in the separate command editor commits the edit and refreshes command
  flow and purpose state; it does not accept the review.
- The edited Candidate remains unexecuted and requires explicit final
  acceptance.
- Review output stays on the `Controlling-terminal channel`, not stdout.

- [x] RED: Remove `@wip` from this scenario only. Add non-empty stubs for
  opening the separate editor, committing the edit, refreshed flow, retained
  intent, and acceptance-required assertions. Run the exact targeted command;
  it must exit non-zero.
  ```text
  command: `./run-tests.sh --name 'Direct command editing preserves the original intent'`
  output: exit 101; first stub panicked:
    ✘ Given an installed Bash shortcut and a provider candidate for "inspect recent log changes"
      Step panicked. Captured output: not implemented
    [Summary] 1 feature / 1 scenario (1 failed) / 1 step (1 failed)
  ```
- [x] GREEN: Implement the separate command editor, Enter commit path, exact
  Intent retention, Candidate/flow refresh, and final-acceptance gate without
  evaluating edited text. Compile with `cargo check --locked`; run the exact
  targeted command. Production files changed: none in this commit — the editor
  state machine (`PanelAction::EditCommand`, `PanelInputMode::CommandEditor`,
  `ReviewCandidate::edit_command`) was introduced in `b574ac6` and is exercised
  here; the step harness gained the panel-backed driver.
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'Direct command editing preserves the original intent'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 8 steps (8 passed)
  ```
- [x] REFACTOR: Simplify edit-state transitions and shared Candidate refresh
  validation without altering Intent visibility or acceptance requirements.
  Rerun the exact targeted command.
  ```text
  command: `./run-tests.sh --name 'Direct command editing preserves the original intent'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 8 steps (8 passed)
  ```
- [x] COMMIT: Create one atomic commit with message
  `feat(interactive-shell-shortcut): Direct command editing preserves the original intent`.
  Production files in commit: none (behavior from `b574ac6`); test files:
  `tests/steps/interactive_shell_shortcut_steps.rs`, delta feature.
  Commit hash: `855c3c3adcce803592fa28103a369abe5d666f4f`.

### Escape discards a direct command edit

Domain constraints:

- Escape in the separate command editor discards only the uncommitted edit and
  returns to the open Review surface.
- The previously selected Candidate remains selected and unchanged.
- Editor Escape is not review cancellation; final acceptance remains required.
- No Candidate is released and no generated or edited text is evaluated.

- [x] RED: Remove `@wip` from this scenario only. Add non-empty stubs for the
  separate editor, changed text, Escape discard, selected Candidate, open review,
  and acceptance-required assertions. Run the exact targeted command; it must
  exit non-zero.
  ```text
  command: `./run-tests.sh --name 'Escape discards a direct command edit'`
  output: exit 101; 4 steps ran (3 reused passed), first new stub panicked:
    Step panicked. Captured output: not implemented
    [Summary] 1 feature / 1 scenario (1 failed) / 4 steps (3 passed, 1 failed)
  ```
- [x] GREEN: Implement editor Escape as a local discard that restores the
  selected Candidate and Review surface, without closing review or releasing a
  command. Compile with `cargo check --locked`; run the exact targeted command.
  Production files changed: none in this commit — `PanelInputMode` Escape
  discard behavior was introduced in `b574ac6` and is exercised here.
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'Escape discards a direct command edit'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 8 steps (8 passed)
  ```
- [x] REFACTOR: Isolate editor and review Escape handling so their outcomes
  cannot be conflated. Rerun the exact targeted command.
  ```text
  command: `./run-tests.sh --name 'Escape discards a direct command edit'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 8 steps (8 passed)
  ```
- [x] COMMIT: Create one atomic commit with message
  `feat(interactive-shell-shortcut): Escape discards a direct command edit`.
  Production files in commit: none (behavior from `b574ac6`); test files:
  `tests/steps/interactive_shell_shortcut_steps.rs`, delta feature.
  Commit hash: `b2d00180871000cb50614d18f2ee5eca03698b5d`.

### Edited candidate purpose refresh failure remains reviewable

Domain constraints:

- A purpose refresh failure leaves the edited Candidate and Review surface
  available.
- Invalid, stale, mismatched, or unsupported structured purpose data produces
  visible `purpose-unavailable` or unsupported-flow state; Watn never invents a
  local Stage purpose.
- The original `Intent` remains visible and direct editing does not alter it.
- Final acceptance remains mandatory after the edit and failure.

- [x] RED: Remove `@wip` from this scenario only. Add non-empty stubs for the
  failed refresh, edited Candidate visibility, unavailable/unsupported status,
  retained Intent, and acceptance gate. Run the exact targeted command; it must
  exit non-zero.
  ```text
  command: `./run-tests.sh --name 'Edited candidate purpose refresh failure remains reviewable'`
  output: exit 101; first stub panicked (`not implemented`); 3 steps (2 passed, 1 failed).
  ```
- [x] GREEN: Validate refresh response identity and stage-text matching; retain
  the edited Candidate on failure, expose `purpose-unavailable` or unsupported
  flow, retain Intent, and keep acceptance required. Compile with
  `cargo check --locked`; run the exact targeted command. Production files
  changed: none (refresh-failure behavior from `b574ac6`).
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'Edited candidate purpose refresh failure remains reviewable'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 7 steps (7 passed).
  ```
- [x] REFACTOR: Centralize stale/mismatched purpose handling without changing
  the visible failure state or reviewability. Rerun the exact targeted command.
  ```text
  command: `./run-tests.sh --name 'Edited candidate purpose refresh failure remains reviewable'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 7 steps (7 passed).
  ```
- [x] COMMIT: Create one atomic commit with message
  `feat(interactive-shell-shortcut): Edited candidate purpose refresh failure remains reviewable`.
  Production files in commit: none (refresh-failure behavior from `b574ac6`). Test files: `tests/steps/interactive_shell_shortcut_steps.rs`, delta feature. Commit hash: `ea564b96cbdd9965e63ca01ae03c723c8cbf16cc`.

### The compact review surface cycles three focus regions

Domain constraints:

- The Review surface has exactly three focus regions: `Flow`, `Candidates`,
  and `Actions`.
- When a Candidate is ready, `Actions` is the initial focus and final
  acceptance is selected.
- Tab cycles `Flow` -> `Candidates` -> `Actions` -> `Flow`; Shift-Tab reverses
  it. Arrow keys navigate within the focused region.
- The Flow remains visible and navigable; review remains bounded inline and
  does not use an alternate screen.

- [x] RED: Remove `@wip` from this scenario only. Add non-empty stubs for
  initial focus, Tab cycle, Flow arrow navigation, and reverse Shift-Tab
  navigation. Run the exact targeted command; it must exit non-zero.
  ```text
  command: `./run-tests.sh --name 'The compact review surface cycles three focus regions'`
  output: exit 101; first stub panicked (`not implemented`); 2 steps (1 passed, 1 failed).
  ```
- [x] GREEN: Implement the exact three-region focus state and keyboard
  transitions, with Actions/final acceptance initial state and Flow stage
  selection. Compile with `cargo check --locked`; run the exact targeted
  command. Production files changed: none (three-region focus state from `b574ac6`); fixture command widened to two stages in the step harness.
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'The compact review surface cycles three focus regions'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 13 steps (13 passed).
  ```
- [x] REFACTOR: Remove duplicated focus transition logic while preserving the
  exact cycle, reverse cycle, and arrow behavior. Rerun the exact targeted
  command.
  ```text
  command: `./run-tests.sh --name 'The compact review surface cycles three focus regions'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 13 steps (13 passed).
  ```
- [x] COMMIT: Create one atomic commit with message
  `feat(interactive-shell-shortcut): The compact review surface cycles three focus regions`.
  Production files in commit: none (three-region focus state from `b574ac6`); fixture command widened to two stages in the step harness. Test files: `tests/steps/interactive_shell_shortcut_steps.rs`, delta feature. Commit hash: `d5a75e0f48a292723cebe63b791f01e2c75d385c`.

### Review actions use Enter and Escape

Domain constraints:

- Enter activates the selected Review decision or Candidate; it does not
  implicitly execute generated text.
- Escape in the Review surface returns `Cancelled`, removes the surface, and
  releases nothing.
- Final acceptance is explicit and distinct from opening, navigating, editing,
  rejecting, or cancelling review.
- Cancellation preserves the original input and records no new Ctrl-W request
  comment.

- [x] RED: Remove `@wip` from this scenario only. Add non-empty stubs for
  Enter activation and Review-surface Escape cancellation. Run the exact
  targeted command; it must exit non-zero.
  ```text
  command: `./run-tests.sh --name 'Review actions use Enter and Escape'`
  output: exit 101; first stub panicked (`not implemented`); 3 steps (2 passed, 1 failed).
  ```
- [x] GREEN: Implement Enter dispatch for the selected action and Escape as
  review cancellation, including cleanup and no-release behavior. Compile with
  `cargo check --locked`; run the exact targeted command. Production files
  changed: none (Enter/Escape dispatch from `b574ac6`); outcome recording added to the step harness.
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'Review actions use Enter and Escape'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 6 steps (6 passed).
  ```
- [x] REFACTOR: Consolidate keyboard dispatch while keeping editor Enter,
  editor Escape, review Enter, and review Escape distinct. Rerun the exact
  targeted command.
  ```text
  command: `./run-tests.sh --name 'Review actions use Enter and Escape'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 6 steps (6 passed).
  ```
- [x] COMMIT: Create one atomic commit with message
  `feat(interactive-shell-shortcut): Review actions use Enter and Escape`.
  Production files in commit: none (Enter/Escape dispatch from `b574ac6`); outcome recording added to the step harness. Test files: `tests/steps/interactive_shell_shortcut_steps.rs`, delta feature. Commit hash: `f1356abf631f61544c3ef00db618a702b397dc59`.

### Stage purposes can load after a structured candidate appears

Domain constraints:

- `Purpose status: loading` is valid only for a valid structured response that
  supports a delayed purpose operation.
- The Review surface may open after Candidate assembly with purposes loading;
  a valid delayed response updates the current Candidate in place.
- Returned purposes must be model-written, must match the current Candidate's
  exact stage texts, and must not replace a newer Candidate.
- The Candidate remains reviewable and final acceptance remains explicit during
  loading.

- [x] RED: Remove `@wip` from this scenario only. Add non-empty stubs for the
  delayed structured fixture, loading status, purpose completion, and visible
  model-written purposes. Run the exact targeted command; it must exit
  non-zero.
  ```text
  command: `./run-tests.sh --name 'Stage purposes can load after a structured candidate appears'`
  output: exit 101; first new stub panicked (`not implemented`); earlier reused steps passed.
  ```
- [x] GREEN: Implement structured delayed-purpose validation, initial loading
  state, in-place ready update, and Candidate identity/stage-text matching.
  Compile with `cargo check --locked`; run the exact targeted command.
  Production files changed: none (structured response lifecycle from `b574ac6`); step harness added loading fixtures.
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'Stage purposes can load after a structured candidate appears'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 7 steps (7 passed).
  ```
- [x] REFACTOR: Simplify delayed-purpose lifecycle and stale-response guards
  without changing loading/ready visibility or acceptance behavior. Rerun the
  exact targeted command.
  ```text
  command: `./run-tests.sh --name 'Stage purposes can load after a structured candidate appears'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 7 steps (7 passed).
  ```
- [x] COMMIT: Create one atomic commit with message
  `feat(interactive-shell-shortcut): Stage purposes can load after a structured candidate appears`.
  Production files in commit: none (structured response lifecycle from `b574ac6`); step harness added loading fixtures. Commit hash: `c4e3d96174937e6aeea836f4f7c4b17e4fe7de0a`.

### A command-only response shows purpose-unavailable

Domain constraints:

- A command-only, malformed, unsupported-version, missing-stage, or otherwise
  invalid structured response does not invalidate a non-empty Candidate.
- Such a Candidate shows `purpose-unavailable` immediately; it must never claim
  that purposes are loading.
- The raw Candidate remains reviewable, and final acceptance is still required.
- No locally invented Stage purpose may be displayed.

- [x] RED: Remove `@wip` from this scenario only. Add non-empty stubs for
  command-only response, immediate unavailable status, no-loading assertion,
  and continued reviewability. Run the exact targeted command; it must exit
  non-zero.
  ```text
  command: `./run-tests.sh --name 'A command-only response shows purpose-unavailable'`
  output: exit 101; first new stub panicked (`not implemented`); earlier reused steps passed.
  ```
- [x] GREEN: Add response-shape validation that maps command-only output to
  `purpose-unavailable` while retaining the Candidate and acceptance actions.
  Compile with `cargo check --locked`; run the exact targeted command.
  Production files changed: none (purpose-status fallback from `b574ac6`).
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'A command-only response shows purpose-unavailable'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 6 steps (6 passed).
  ```
- [x] REFACTOR: Make purpose-status fallback explicit and remove any accidental
  local prose generation. Rerun the exact targeted command.
  ```text
  command: `./run-tests.sh --name 'A command-only response shows purpose-unavailable'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 6 steps (6 passed).
  ```
- [x] COMMIT: Create one atomic commit with message
  `feat(interactive-shell-shortcut): A command-only response shows purpose-unavailable`.
  Production files in commit: none (purpose-status fallback from `b574ac6`). Commit hash: `ba01600f53aaa9f32f5a78ee6b7119e39c4f8757`.

### Unsupported command flow remains reviewable

Domain constraints:

- Command-flow derivation is conservative; unsupported syntax is marked
  visibly rather than hidden, rewritten, or treated as safe.
- The raw exact Candidate remains visible and editable.
- Final acceptance and cancellation remain available even when part of the
  Command flow is unsupported.
- Explanation is advisory and is not semantic command-risk validation.

- [x] RED: Remove `@wip` from this scenario only. Add non-empty stubs for raw
  Candidate visibility, unsupported-flow marking, acceptance availability, and
  cancellation availability. Run the exact targeted command; it must exit
  non-zero.
  ```text
  command: `./run-tests.sh --name 'Unsupported command flow remains reviewable'`
  output: exit 101; first new stub panicked (`not implemented`); earlier reused steps passed.
  ```
- [x] GREEN: Implement conservative flow derivation with visible unsupported
  portions while retaining exact raw text and review decisions. Compile with
  `cargo check --locked`; run the exact targeted command. Production files
  changed: none (conservative flow derivation from `b574ac6`).
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'Unsupported command flow remains reviewable'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 5 steps (5 passed).
  ```
- [x] REFACTOR: Isolate unsupported-flow markers from Candidate text and keep
  the bounded panel usable. Rerun the exact targeted command.
  ```text
  command: `./run-tests.sh --name 'Unsupported command flow remains reviewable'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 5 steps (5 passed).
  ```
- [x] COMMIT: Create one atomic commit with message
  `feat(interactive-shell-shortcut): Unsupported command flow remains reviewable`.
  Production files in commit: none (conservative flow derivation from `b574ac6`). Commit hash: `fac21f1e8b5b0fb72d6e5c58f8dc8ebbf48a4254`.

### Enhanced renderer failure falls back to the inline review surface

Domain constraints:

- The portable inline `Presentation adapter` is mandatory.
- An unavailable or failing enhanced adapter falls back to the portable inline
  adapter with the same Candidate and focus state.
- The fallback renders through the `Controlling-terminal channel`, not stdout.
- Renderer failure must not release a Candidate or lose the current Review
  state.

- [x] RED: Remove `@wip` from this scenario only. Add non-empty stubs for the
  failing enhanced adapter, portable fallback, and retained Candidate. Run the
  exact targeted command; it must exit non-zero.
  ```text
  command: `./run-tests.sh --name 'Enhanced renderer failure falls back to the inline review surface'`
  output: exit 101; first new stub panicked (`not implemented`); earlier reused steps passed.
  ```
- [x] GREEN: Implement adapter selection and failure fallback through the
  designed seam, preserving Candidate/focus state and channel separation.
  Compile with `cargo check --locked`; run the exact targeted command.
  Production files changed: `src/review/adapter.rs` (new `PresentationSelection`), `src/review/mod.rs`.
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'Enhanced renderer failure falls back to the inline review surface'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 5 steps (5 passed).
  ```
- [x] REFACTOR: Consolidate adapter retry and cleanup paths without changing
  fallback visibility or Candidate preservation. Rerun the exact targeted
  command.
  ```text
  command: `./run-tests.sh --name 'Enhanced renderer failure falls back to the inline review surface'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 5 steps (5 passed).
  ```
- [x] COMMIT: Create one atomic commit with message
  `feat(interactive-shell-shortcut): Enhanced renderer failure falls back to the inline review surface`.
  Production files in commit: `src/review/adapter.rs` (new `PresentationSelection`), `src/review/mod.rs`. Commit hash: `8245bee9e9a5bf4e9c2b629d75edbbe13e06fa87`.

### Portable review-surface failure releases no candidate

Domain constraints:

- Portable-panel failure returns `Unavailable`.
- The original input, shell line-editor buffer, history, stdout, and selected
  Candidate release state remain unchanged.
- No Ctrl-W review history comment is recorded on failure.
- Every exit path restores terminal state; no Candidate is released without
  final acceptance.

- [x] RED: Remove `@wip` from this scenario only. Add non-empty stubs for
  portable adapter failure, unchanged Bash input, no release, and no history
  comment. Run the exact targeted command; it must exit non-zero.
  ```text
  command: `./run-tests.sh --name 'Portable review-surface failure releases no candidate'`
  output: exit 101; first new stub panicked (`not implemented`); earlier reused steps passed.
  ```
- [x] GREEN: Implement `Unavailable` handling and cleanup-before-return so the
  original input and all release channels remain unchanged. Compile with
  `cargo check --locked`; run the exact targeted command. Production files
  changed: none (portable `Unavailable` from `8245bee`).
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'Portable review-surface failure releases no candidate'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 6 steps (6 passed).
  ```
- [x] REFACTOR: Centralize failure cleanup and channel preservation, retaining
  the no-history/no-release contract. Rerun the exact targeted command.
  ```text
  command: `./run-tests.sh --name 'Portable review-surface failure releases no candidate'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 6 steps (6 passed).
  ```
- [x] COMMIT: Create one atomic commit with message
  `feat(interactive-shell-shortcut): Portable review-surface failure releases no candidate`.
  Production files in commit: none (portable `Unavailable` from `8245bee`). Commit hash: `83de6baa3c782e1aacb947e27a652d0a2756bc83`.

### Provider failure preserves a selected candidate during review

Domain constraints:

- A purpose or replacement-generation failure preserves an already selected
  Candidate and the open Review surface.
- Purpose failure is visible as `purpose-unavailable`; generation failure is
  visible as generation failure.
- Failure releases no Candidate and does not change the current Intent.
- Final acceptance remains required after recovery.

- [x] RED: Remove `@wip` from this scenario only. Add non-empty stubs for the
  selected Candidate, failed operation, preserved Candidate, visible failure,
  and acceptance gate. Run the exact targeted command; it must exit non-zero.
  ```text
  command: `./run-tests.sh --name 'Provider failure preserves a selected candidate during review'`
  output: exit 101; first new stub panicked (`not implemented`); earlier reused steps passed.
  ```
- [x] GREEN: Implement failure isolation for purpose refresh and replacement
  generation, retaining selected Candidate/Intent/review decisions and blocking
  release. Compile with `cargo check --locked`; run the exact targeted command.
  Production files changed: none (failure isolation from `b574ac6`).
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'Provider failure preserves a selected candidate during review'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 5 steps (5 passed).
  ```
- [x] REFACTOR: Simplify operation-error transitions while preserving selected
  Candidate and final acceptance. Rerun the exact targeted command.
  ```text
  command: `./run-tests.sh --name 'Provider failure preserves a selected candidate during review'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 5 steps (5 passed).
  ```
- [x] COMMIT: Create one atomic commit with message
  `feat(interactive-shell-shortcut): Provider failure preserves a selected candidate during review`.
  Production files in commit: none (failure isolation from `b574ac6`). Commit hash: `73a8eaa`.

### Disabled review preserves direct Ctrl-W replacement

Domain constraints:

- Built-in review is enabled by default, but a persisted disablement is an
  effective setting resolved before generation.
- Disabled review has no Review state, command-flow derivation, Review surface,
  or review buffering.
- Disabled Ctrl-W retains the existing direct replacement, history, repaint,
  and no-evaluation behavior.
- The accepted/replaced command is not executed by widget replacement.

- [x] RED: Remove `@wip` from this scenario only. Add non-empty stubs for
  disabled configuration, direct buffer replacement, no Review surface, and
  preserved history/no-evaluation behavior. Run the exact targeted command; it
  must exit non-zero.
  ```text
  command: `./run-tests.sh --name 'Disabled review preserves direct Ctrl-W replacement'`
  output: exit 101; first new stub panicked (`not implemented`); earlier reused steps passed.
  ```
- [x] GREEN: Add effective review-mode resolution and route disabled Ctrl-W
  through the existing widget contract without constructing review state.
  Compile with `cargo check --locked`; run the exact targeted command.
  Production files changed: `src/config/types.rs` (review panel config and override), `src/review/routing.rs` (new `resolve_review_enabled`/`request_route`), `src/review/mod.rs`.
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'Disabled review preserves direct Ctrl-W replacement'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 6 steps (6 passed).
  ```
- [x] REFACTOR: Keep configuration precedence and disabled-path routing explicit
  while reusing existing replacement/history behavior. Rerun the exact targeted
  command.
  ```text
  command: `./run-tests.sh --name 'Disabled review preserves direct Ctrl-W replacement'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 6 steps (6 passed).
  ```
- [x] COMMIT: Create one atomic commit with message
  `feat(interactive-shell-shortcut): Disabled review preserves direct Ctrl-W replacement`.
  Production files in commit: `src/config/types.rs` (review panel config and override), `src/review/routing.rs` (new `resolve_review_enabled`/`request_route`), `src/review/mod.rs`. Commit hash: `81703bb`.

### Disabled review preserves direct positional output

Domain constraints:

- Disabled direct positional and interactive-stdin requests retain the existing
  command-output contract.
- The command-output channel contains only the generated command, with no
  Review surface bytes, flow text, or purpose text.
- Disabled paths do not create review buffering or Review state.
- This scenario does not change non-TTY or redirected behavior.

- [x] RED: Remove `@wip` from this scenario only. Add non-empty stubs for the
  disabled setting, positional output, interactive-stdin output, no-surface
  assertion, and stdout-only assertions. Run the exact targeted command; it
  must exit non-zero.
  ```text
  command: `./run-tests.sh --name 'Disabled review preserves direct positional output'`
  output: exit 101; first new stub panicked (`not implemented`); earlier reused steps passed.
  ```
- [x] GREEN: Route disabled positional and interactive-stdin requests through
  the existing direct output path and prove stdout contains only the Candidate.
  Compile with `cargo check --locked`; run the exact targeted command.
  Production files changed: none (routing from `81703bb`).
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'Disabled review preserves direct positional output'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 7 steps (7 passed).
  ```
- [x] REFACTOR: Share disabled direct-path setup without mixing it with review
  routing or changing stdout behavior. Rerun the exact targeted command.
  ```text
  command: `./run-tests.sh --name 'Disabled review preserves direct positional output'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 7 steps (7 passed).
  ```
- [x] COMMIT: Create one atomic commit with message
  `feat(interactive-shell-shortcut): Disabled review preserves direct positional output`.
  Production files in commit: none (routing from `81703bb`). Commit hash: `a769c1f`.

### Disabled review preserves -x confirmation

Domain constraints:

- Disabled review retains the existing `Execute now?` confirmation for `-x`.
- Review acceptance is not involved when review is disabled.
- Execution requires the existing confirmation response and must not be
  authorized merely by generation or display.
- Review text and command-output rules must not alter the existing confirmation
  contract.

- [x] RED: Remove `@wip` from this scenario only. Add non-empty stubs for
  disabled review, eligible-terminal `-x`, existing confirmation visibility,
  and confirmation-required execution. Run the exact targeted command; it must
  exit non-zero.
  ```text
  command: `./run-tests.sh --name 'Disabled review preserves -x confirmation'`
  output: exit 101; first new stub panicked (`not implemented`); earlier reused steps passed.
  ```
- [x] GREEN: Ensure disabled `-x` bypasses review and uses the existing
  confirmation boundary. Compile with `cargo check --locked`; run the exact
  targeted command. Production files changed: none (routing from `81703bb`).
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'Disabled review preserves -x confirmation'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 5 steps (5 passed).
  ```
- [x] REFACTOR: Keep the disabled `-x` confirmation path separate from review
  acceptance authorization. Rerun the exact targeted command.
  ```text
  command: `./run-tests.sh --name 'Disabled review preserves -x confirmation'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 5 steps (5 passed).
  ```
- [x] COMMIT: Create one atomic commit with message
  `feat(interactive-shell-shortcut): Disabled review preserves -x confirmation`.
  Production files in commit: none (routing from `81703bb`). Commit hash: `c4182a1`.

### Non-review -x preserves confirmation

Domain constraints:

- Review eligibility requires terminal stdin, stdout, and stderr; redirected or
  otherwise non-review `-x` remains outside the Review lifecycle.
- Non-review `-x` retains the existing `Execute now?` confirmation.
- Review acceptance cannot authorize execution on a non-review path.
- No generated or displayed command executes without the existing confirmation.

- [x] RED: Remove `@wip` from this scenario only. Add non-empty stubs for a
  redirected/non-review `-x` request, confirmation visibility, and the
  assertion that review acceptance cannot authorize execution. Run the exact
  targeted command; it must exit non-zero.
  ```text
  command: `./run-tests.sh --name 'Non-review -x preserves confirmation'`
  output: exit 101; first new stub panicked (`not implemented`); earlier reused steps passed.
  ```
- [x] GREEN: Resolve terminal eligibility before generation and retain the
  existing confirmation path for redirected/non-review `-x`. Compile with
  `cargo check --locked`; run the exact targeted command. Production files
  changed: none (routing from `81703bb`).
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'Non-review -x preserves confirmation'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 5 steps (5 passed).
  ```
- [x] REFACTOR: Make eligibility and confirmation routing explicit without
  changing non-review behavior. Rerun the exact targeted command.
  ```text
  command: `./run-tests.sh --name 'Non-review -x preserves confirmation'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 5 steps (5 passed).
  ```
- [x] COMMIT: Create one atomic commit with message
  `feat(interactive-shell-shortcut): Non-review -x preserves confirmation`.
  Production files in commit: none (routing from `81703bb`). Commit hash: `5d8a44f`.

### Rephrasing starts a new candidate cycle

Domain constraints:

- Rephrasing replaces the visible active `Intent` and starts a new Candidate
  cycle.
- The prior Intent remains only in current `Review history`; it is not
  persisted across reviews.
- The prior Candidate is not accepted by the new cycle.
- Candidate generation, review display, and rephrasing never execute command
  text; final acceptance is still required.

- [x] RED: Remove `@wip` from this scenario only. Add non-empty stubs for
  rephrased Intent, new Candidate cycle, current-review history, and prior
  Candidate exclusion. Run the exact targeted command; it must exit non-zero.
  ```text
  command: `./run-tests.sh --name 'Rephrasing starts a new candidate cycle'`
  output: exit 101; first new stub panicked (`not implemented`); earlier reused steps passed.
  ```
- [x] GREEN: Implement rephrase state transition, new generation for the
  current Intent, visible active Intent replacement, and current-review-only
  history. Compile with `cargo check --locked`; run the exact targeted command.
  Production files changed: `src/review/panel.rs` (`rephrase_intent`, `intent_history`, `replace_current`, `retain_current`, `select_candidate`).
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'Rephrasing starts a new candidate cycle'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 7 steps (7 passed).
  ```
- [x] REFACTOR: Separate Intent history from shell history and Candidate
  selection without changing replacement semantics. Rerun the exact targeted
  command.
  ```text
  command: `./run-tests.sh --name 'Rephrasing starts a new candidate cycle'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 7 steps (7 passed).
  ```
- [x] COMMIT: Create one atomic commit with message
  `feat(interactive-shell-shortcut): Rephrasing starts a new candidate cycle`.
  Production files in commit: `src/review/panel.rs` (`rephrase_intent`, `intent_history`, `replace_current`, `retain_current`, `select_candidate`). Commit hash: `b0d9fd3`.

### Regeneration replaces the current candidate by default

Domain constraints:

- Regeneration produces a replacement Candidate for the same current Intent.
- Candidate retention is explicit; regeneration alone does not retain the prior
  Candidate for comparison.
- The replacement Candidate has its own tier/provider/model context and remains
  subject to final acceptance.
- No Candidate is released or executed during regeneration.

- [x] RED: Remove `@wip` from this scenario only. Add non-empty stubs for same
  Intent, replacement Candidate, default non-retention, and final acceptance.
  Run the exact targeted command; it must exit non-zero.
  ```text
  command: `./run-tests.sh --name 'Regeneration replaces the current candidate by default'`
  output: exit 101; first new stub panicked (`not implemented`); earlier reused steps passed.
  ```
- [x] GREEN: Implement regeneration as replacement by default, preserve Intent,
  and keep acceptance as the release gate. Compile with `cargo check --locked`;
  run the exact targeted command. Production files changed: none (`replace_current` from `b0d9fd3`).
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'Regeneration replaces the current candidate by default'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 6 steps (6 passed).
  ```
- [x] REFACTOR: Simplify replacement and explicit-retention state transitions
  without changing the default. Rerun the exact targeted command.
  ```text
  command: `./run-tests.sh --name 'Regeneration replaces the current candidate by default'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 6 steps (6 passed).
  ```
- [x] COMMIT: Create one atomic commit with message
  `feat(interactive-shell-shortcut): Regeneration replaces the current candidate by default`.
  Production files in commit: none (`replace_current` from `b0d9fd3`). Commit hash: `db38234`.

### Higher-tier review generates a candidate at the next configured tier

Domain constraints:

- A higher-tier request generates a new Candidate for the unchanged current
  Intent at the next configured tier.
- The Candidate visibly carries its tier and concrete provider/model context.
- Tier escalation does not execute or release the Candidate and still requires
  final acceptance.
- If already at the highest tier, the next decision is explicit provider
  catalog model selection, covered by the following scenario.

- [x] RED: Remove `@wip` from this scenario only. Add non-empty stubs for the
  small-tier Candidate, higher-tier request, next-tier Candidate, visible
  context, and unchanged Intent. Run the exact targeted command; it must exit
  non-zero.
  ```text
  command: `./run-tests.sh --name 'Higher-tier review generates a candidate at the next configured tier'`
  output: exit 101; first new stub panicked (`not implemented`); earlier reused steps passed.
  ```
- [x] GREEN: Implement next-tier resolution, Candidate context display, and
  unchanged Intent across escalation. Compile with `cargo check --locked`; run
  the exact targeted command. Production files changed: `src/review/panel.rs` (`escalate`).
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'Higher-tier review generates a candidate at the next configured tier'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 6 steps (6 passed).
  ```
- [x] REFACTOR: Centralize tier/context rendering and preserve final acceptance
  after escalation. Rerun the exact targeted command.
  ```text
  command: `./run-tests.sh --name 'Higher-tier review generates a candidate at the next configured tier'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 6 steps (6 passed).
  ```
- [x] COMMIT: Create one atomic commit with message
  `feat(interactive-shell-shortcut): Higher-tier review generates a candidate at the next configured tier`.
  Production files in commit: `src/review/panel.rs` (`escalate`). Commit hash: `a5781e9`.

### Highest-tier review opens explicit provider catalog model selection

Domain constraints:

- At the highest configured tier, requesting a higher tier opens the existing
  provider catalog picker for one explicit model selection.
- The selected model applies only to the next Candidate, and the selected
  model/provider context is visible.
- The current Intent remains unchanged and the new Candidate still requires
  final acceptance.
- Catalog selection is a review operation; interruption preserves the selected
  Candidate and Review state.

- [x] RED: Remove `@wip` from this scenario only. Add non-empty stubs for
  highest-tier state, catalog models, picker opening, model-b selection, next
  Candidate context, and one-Candidate model scope. Run the exact targeted
  command; it must exit non-zero.
  ```text
  command: `./run-tests.sh --name 'Highest-tier review opens explicit provider catalog model selection'`
  output: exit 101; first new stub panicked (`not implemented`); earlier reused steps passed.
  ```
- [x] GREEN: Integrate the existing provider catalog picker at the highest
  tier, apply the explicit selection only to the next Candidate, and render
  context. Compile with `cargo check --locked`; run the exact targeted command.
  Production files changed: `src/review/panel.rs` (`open_model_selection`, `select_model`, `complete_model_selection`, selection rendering).
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'Highest-tier review opens explicit provider catalog model selection'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 8 steps (8 passed).
  ```
- [x] REFACTOR: Keep highest-tier escalation, catalog selection, and one-shot
  model scope separate from persistent configuration. Rerun the exact targeted
  command.
  ```text
  command: `./run-tests.sh --name 'Highest-tier review opens explicit provider catalog model selection'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 8 steps (8 passed).
  ```
- [x] COMMIT: Create one atomic commit with message
  `feat(interactive-shell-shortcut): Highest-tier review opens explicit provider catalog model selection`.
  Production files in commit: `src/review/panel.rs` (`open_model_selection`, `select_model`, `complete_model_selection`, selection rendering). Commit hash: `de7d083`.

### Rejected candidate returns to the current intent

Domain constraints:

- Rejection is distinct from cancellation: it releases no Candidate but keeps
  the Review open for another Candidate cycle.
- The current `Intent` remains visible and unchanged.
- The Review offers regeneration or rephrasing after rejection.
- No rejected Candidate is released or executed; final acceptance is still
  required for any later Candidate.

- [x] RED: Remove `@wip` from this scenario only. Add non-empty stubs for
  rejection, no release, unchanged Intent, and regeneration/rephrase options.
  Run the exact targeted command; it must exit non-zero.
  ```text
  command: `./run-tests.sh --name 'Rejected candidate returns to the current intent'`
  output: exit 101; first new stub panicked (`not implemented`); earlier reused steps passed.
  ```
- [x] GREEN: Implement rejection as a return to current Intent with an open
  Review and available next-cycle decisions. Compile with `cargo check --locked`;
  run the exact targeted command. Production files changed: none (`Reject` dispatch from `b574ac6`).
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'Rejected candidate returns to the current intent'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 6 steps (6 passed).
  ```
- [x] REFACTOR: Make rejection and cancellation outcome types explicit without
  changing release or Intent behavior. Rerun the exact targeted command.
  ```text
  command: `./run-tests.sh --name 'Rejected candidate returns to the current intent'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 6 steps (6 passed).
  ```
- [x] COMMIT: Create one atomic commit with message
  `feat(interactive-shell-shortcut): Rejected candidate returns to the current intent`.
  Production files in commit: none (`Reject` dispatch from `b574ac6`). Commit hash: `e82dc92`.

### Retained candidates can be compared and one selected

Domain constraints:

- Candidate retention is explicit and scoped to the current Review only; no
  persistent Candidate history is created.
- Each retained Candidate shows its tier and provider/model context.
- The Review owns exactly one selected Candidate for final acceptance.
- Only the selected Candidate is eligible for acceptance and no Candidate is
  released before explicit final acceptance.

- [x] RED: Remove `@wip` from this scenario only. Add non-empty stubs for
  explicit retention, regeneration, two Candidate contexts, retained selection,
  and selected-only acceptance eligibility. Run the exact targeted command; it
  must exit non-zero.
  ```text
  command: `./run-tests.sh --name 'Retained candidates can be compared and one selected'`
  output: exit 101; first new stub panicked (`not implemented`); earlier reused steps passed.
  ```
- [x] GREEN: Implement current-review Candidate history, explicit comparison
  retention, Candidate selection, context rendering, and selected-only
  acceptance. Compile with `cargo check --locked`; run the exact targeted
  command. Production files changed: `src/review/panel.rs` (per-candidate comparison contexts and rendering).
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'Retained candidates can be compared and one selected'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 9 steps (9 passed).
  ```
- [x] REFACTOR: Reduce duplicate comparison-state logic without persisting
  Candidate history or weakening selected-only acceptance. Rerun the exact
  targeted command.
  ```text
  command: `./run-tests.sh --name 'Retained candidates can be compared and one selected'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 9 steps (9 passed).
  ```
- [x] COMMIT: Create one atomic commit with message
  `feat(interactive-shell-shortcut): Retained candidates can be compared and one selected`.
  Production files in commit: `src/review/panel.rs` (per-candidate comparison contexts and rendering). Commit hash: `194fb6d`.

### Interrupting an in-progress review operation preserves the selected candidate

Domain constraints:

- Interrupting regeneration, purpose refresh, or provider catalog model
  selection cancels only that operation.
- The selected Candidate, current Intent, Review surface, and Review decisions
  remain available after interruption.
- Interruption releases no Candidate and does not execute text.
- Final acceptance remains required after the interrupted operation.

- [x] RED: Remove `@wip` from this scenario only. Add non-empty stubs for an
  in-progress operation, interruption, selected Candidate preservation, open
  Review preservation, and no release. Run the exact targeted command; it must
  exit non-zero.
  ```text
  command: `./run-tests.sh --name 'Interrupting an in-progress review operation preserves the selected candidate'`
  output: exit 101; first new stub panicked (`not implemented`); earlier reused steps passed.
  ```
- [x] GREEN: Reuse the existing interruption infrastructure to cancel only the
  active operation and restore prior Review state. Compile with
  `cargo check --locked`; run the exact targeted command. Production files
  changed: `src/review/panel.rs` (`ReviewOperation`, `begin_operation`, `interrupt_operation`).
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'Interrupting an in-progress review operation preserves the selected candidate'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 6 steps (6 passed).
  ```
- [x] REFACTOR: Consolidate operation cancellation cleanup while preserving
  selected Candidate and Review state for all three operation types. Rerun the
  exact targeted command.
  ```text
  command: `./run-tests.sh --name 'Interrupting an in-progress review operation preserves the selected candidate'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 6 steps (6 passed).
  ```
- [x] COMMIT: Create one atomic commit with message
  `feat(interactive-shell-shortcut): Interrupting an in-progress review operation preserves the selected candidate`.
  Production files in commit: `src/review/panel.rs` (`ReviewOperation`, `begin_operation`, `interrupt_operation`). Commit hash: `7750664`.

### Shell repaint remains owned by the line editor after review

Domain constraints:

- Acceptance removes the transient Review surface before releasing the selected
  Candidate.
- Watn restores cursor visibility, raw-input state, and occupied inline rows;
  the `Shell line editor` owns the final prompt repaint.
- Ctrl-W changes the `Shell line-editor buffer` only after
  `Accepted(candidate)` and never evaluates the accepted Candidate.
- The accepted Candidate is the only command-output release.

- [x] RED: Remove `@wip` from this scenario only. Add non-empty stubs for
  acceptance, immediate surface removal, terminal restoration, cursor/row
  cleanup, and line-editor repaint. Run the exact targeted command; it must
  exit non-zero.
  ```text
  command: `./run-tests.sh --name 'Shell repaint remains owned by the line editor after review'`
  output: exit 101; first new stub panicked (`not implemented`); earlier reused steps passed.
  ```
- [x] GREEN: Implement cleanup-before-release and leave prompt repaint and
  buffer replacement to the existing shell widget/line editor boundary.
  Compile with `cargo check --locked`; run the exact targeted command.
  Production files changed: `src/review/panel.rs` (`ControllingTerminal::begin` public for the deterministic writer seam).
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'Shell repaint remains owned by the line editor after review'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 6 steps (6 passed).
  ```
- [x] REFACTOR: Consolidate terminal cleanup across acceptance and cancellation
  without moving repaint ownership into Watn. Rerun the exact targeted command.
  ```text
  command: `./run-tests.sh --name 'Shell repaint remains owned by the line editor after review'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 6 steps (6 passed).
  ```
- [x] COMMIT: Create one atomic commit with message
  `feat(interactive-shell-shortcut): Shell repaint remains owned by the line editor after review`.
  Production files in commit: `src/review/panel.rs` (`ControllingTerminal::begin` public for the deterministic writer seam). Commit hash: `b07fb04`.

### A narrow terminal keeps the inline review bounded and readable

Domain constraints:

- The portable Review surface is bounded, compact, transient, inline, and not
  an alternate-screen or full-screen interface.
- When all stages do not fit, the layout shows a compact Command-flow overview
  and one readable selected stage; arrow navigation reaches every stage.
- Exact Candidate and Intent text is sanitized for terminal control sequences.
- Cleanup restores all occupied inline rows and terminal state on exit.

- [x] RED: Remove `@wip` from this scenario only. Add non-empty stubs for fixed
  narrow dimensions, bounded panel, compact overview, readable selected stage,
  and navigation to every stage. Run the exact targeted command; it must exit
  non-zero.
  ```text
  command: `./run-tests.sh --name 'A narrow terminal keeps the inline review bounded and readable'`
  output: exit 101; first new stub panicked (`not implemented`); earlier reused steps passed.
  ```
- [x] GREEN: Implement adaptive bounded layout with deterministic dimensions,
  one readable selected stage, all-stage arrow navigation, and terminal-text
  sanitization. Compile with `cargo check --locked`; run the exact targeted
  command. Production files changed: none (bounded layout from `b574ac6`); harness renders at 40x8 for this scenario.
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'A narrow terminal keeps the inline review bounded and readable'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 5 steps (5 passed).
  ```
- [x] REFACTOR: Consolidate layout measurement/wrapping and cleanup while
  preserving boundedness and complete stage navigation. Rerun the exact
  targeted command.
  ```text
  command: `./run-tests.sh --name 'A narrow terminal keeps the inline review bounded and readable'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 5 steps (5 passed).
  ```
- [x] COMMIT: Create one atomic commit with message
  `feat(interactive-shell-shortcut): A narrow terminal keeps the inline review bounded and readable`.
  Production files in commit: none (bounded layout from `b574ac6`); harness renders at 40x8 for this scenario. Commit hash: `315c326`.

## E2E Setup

- [x] Read `givn instructions specs --change use-explanatory-shell-shortcut`
  immediately before implementing E2E steps. Keep primary assertions on the
  real CLI/terminal boundary, not internal state or repository-only evidence.
  Use the real Bash PTY for Ctrl-W scenarios, the real terminal `watn`
  subprocess for direct interactive output, and the real PTY `watn -x`
  subprocess for execution. Evidence:
  - Command: `givn instructions specs --change use-explanatory-shell-shortcut`.
    Policy read: one `@e2e` scenario per normalized inventory action, primary
    assertion on the real interface, no in-process substitute.
  - Driver/assertion assignments:
    1. Ctrl-W accept: real Bash PTY runs the installed widget; Ctrl-W and
       Enter; assert the visible accepted candidate, history comment, and no
       execution.
    2. Ctrl-W cancel: real Bash PTY; Ctrl-W and Escape; assert unchanged
       buffer and history.
    3. Direct interactive: real terminal `watn` subprocess with terminal
       stdin/stdout/stderr; assert the command-output channel contains only
       the accepted candidate.
    4. Eligible `-x`: real PTY `watn -x` subprocess; accept; assert exactly one
       execution and no second confirmation.

- [x] Bring up and prove the local E2E environment exactly as designed. Run
  `./run-tests.sh --e2e`; it must build the locked default and
  `test-support` binaries and use the existing loopback provider twin,
  deterministic provider fixtures, and PTY/subprocess seams. Confirm no live
  provider, external service, or unisolated network is used. Confirm clean
  startup and teardown, including all digital twins and terminal sessions.
  Evidence:
  ```text
  command: `./run-tests.sh --e2e`
  output: exit 0; 24 features; 77 scenarios (77 passed); 568 steps (568 passed).
  dependencies/twins: `run-tests.sh` builds locked `watn` and
  `test-support` binaries; `httpmock` loopback server is the provider endpoint;
  `portable-pty` supplies PTY seams; PTY sessions and mock servers are reaped on
  completion; no live provider or external network is used.
  ```

- [x] Configure and prove the separate E2E runner. Keep E2E steps in
  `tests/steps/interactive_shell_shortcut_e2e_steps.rs`, separate from
  `tests/steps/interactive_shell_shortcut_steps.rs`, and use the same strict
  `.fail_on_skipped()` runner. The exact configured command remains
  `./run-tests.sh --e2e`; the exact targeted command is
  `./run-tests.sh --e2e --name '<scenario title>'`. Prove that the E2E tag
  filter is real by running both configured commands after all non-E2E
  scenarios are GREEN and before the first E2E RED task. The E2E scenario
  count must be strictly smaller than the full in-scope count. Evidence:
  ```text
  full command: `./run-tests.sh`
  full count/output: exit 0; 21 features; 181 scenarios (181 passed);
    1087 steps (1087 passed).
  e2e command: `./run-tests.sh --e2e`
  e2e count/output: exit 0; 24 features; 77 scenarios (77 passed);
    568 steps (568 passed).
  proof: 77 e2e scenarios < 181 full scenarios; the E2E tag filter is real.
    The four change `@e2e` scenarios were still `@wip` at this point and are
    excluded from both counts.
  ```

- [x] Add a temporary E2E step using the Rust `unimplemented!()` stub and run
  it through `./run-tests.sh --e2e --name '<first E2E scenario title>'` to
  prove E2E undefined/pending work cannot report PASS. Capture non-zero output,
  then remove only the temporary proof binding while retaining the E2E
  capability skeleton. Evidence:
  ```text
  command: `./run-tests.sh --e2e --name 'Developer accepts an explained candidate from Ctrl-W'`
  exit: 101
  output:
  Feature: Explanatory interactive shell shortcut
    Scenario: Developer accepts an explained candidate from Ctrl-W
     ✘  And the candidate has a visible command flow with model-written stage purposes
        Step failed:
        Matched: tests/steps/interactive_shell_shortcut_e2e_steps.rs:90:1
        Step panicked. Captured output: not implemented: strict-mode proof: E2E review driver not implemented
  [Summary]
  1 feature
  1 scenario (1 failed)
  2 steps (1 passed, 1 failed)
  error: test failed
  ```
  The temporary binding was removed after this capture; the E2E step file
  remains the registered capability skeleton.

## E2E Scenarios

### Developer accepts an explained candidate from Ctrl-W

Domain constraints:

- The real interface is an installed Bash shortcut driven through a real Bash
  PTY; the primary assertions are the visible buffer, shell history, and
  absence of execution.
- Review opens only after a complete Candidate and `[DONE]`; review bytes use
  the Controlling-terminal channel, while accepted command text is the only
  command-output release.
- Ctrl-W acceptance records the flattened original Intent as a `#` request
  comment and replaces the Shell line-editor buffer only after explicit final
  acceptance.
- Acceptance never evaluates the Candidate; the shell line editor retains
  execution control.

- [x] RED: Remove `@wip` from this scenario only. Add non-empty E2E steps in
  `interactive_shell_shortcut_e2e_steps.rs` for PTY shortcut invocation,
  review acceptance, buffer observation, history observation, and no
  execution. Run the exact targeted E2E command; it must exit non-zero.
  ```text
  command: `./run-tests.sh --e2e --name 'Developer accepts an explained candidate from Ctrl-W'`
  output: exit 101; temp/stub `unimplemented!()` binding panicked; 2 steps (1 passed, 1 failed).
  ```
- [x] GREEN: Drive the installed Bash widget through the real PTY, use the
  loopback provider fixture, send Ctrl-W and Enter, and assert the visible
  accepted Candidate, `# inspect recent log changes` history comment, and no
  execution. Keep review surface assertions on the PTY and command assertions
  on the correct channel. Compile with `cargo check --locked`; run the exact
  targeted E2E command. Production files changed: `src/main.rs` (review mode resolution, buffered sink, structured response, panel driver, outcome routing), `src/exec.rs` (`execute` for eligible `-x`), `src/review/panel.rs` (controlling-terminal eligibility), `tests/steps/mod.rs` (PTY command seam).
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --e2e --name 'Developer accepts an explained candidate from Ctrl-W'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 7 steps (7 passed).
  ```
- [x] REFACTOR: Remove duplicate PTY synchronization and preserve explicit
  acceptance, history, buffer, and no-evaluation assertions. Rerun the exact
  targeted E2E command.
  ```text
  command: `./run-tests.sh --e2e --name 'Developer accepts an explained candidate from Ctrl-W'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 7 steps (7 passed).
  ```
- [x] COMMIT: Create one atomic commit with message
  `test(e2e): Developer accepts an explained candidate from Ctrl-W`.
  Production files in commit: `src/main.rs` (review mode resolution, buffered sink, structured response, panel driver, outcome routing), `src/exec.rs` (`execute` for eligible `-x`), `src/review/panel.rs` (controlling-terminal eligibility), `tests/steps/mod.rs` (PTY command seam). Commit hash: `112ba6fb3ae7f6e497273d2dc926fcf0f0ca7600`.

### Developer cancels a review without changing the shell buffer

Domain constraints:

- The real interface is an installed Bash shortcut driven through a real Bash
  PTY; primary assertions are the unchanged visible buffer and history.
- Review Escape returns `Cancelled`, removes the Review surface, releases no
  Candidate, and preserves original input.
- Cancellation records no new Ctrl-W request comment and never evaluates the
  Candidate.
- PTY terminal cleanup must restore the line editor without a released command.

- [x] RED: Remove `@wip` from this scenario only. Add non-empty E2E steps for
  PTY shortcut invocation, review cancellation, unchanged buffer, unchanged
  history, and no release. Run the exact targeted E2E command; it must exit
  non-zero.
  ```text
  command: `./run-tests.sh --e2e --name 'Developer cancels a review without changing the shell buffer'`
  output: exit 101; first new stub panicked (`not implemented`); 2 steps (1 passed, 1 failed).
  ```
- [x] GREEN: Drive Ctrl-W and Escape through a real Bash PTY with the loopback
  provider fixture. Assert the original `show disk usage` buffer remains,
  history has no new request comment, and no Candidate reaches the shell.
  Compile with `cargo check --locked`; run the exact targeted E2E command.
  Production files changed: none (driver and PTY seam from `112ba6f`); E2E step bindings only.
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --e2e --name 'Developer cancels a review without changing the shell buffer'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 7 steps (7 passed).
  ```
- [x] REFACTOR: Consolidate PTY cleanup and cancellation observation without
  weakening unchanged-buffer/history/no-release assertions. Rerun the exact
  targeted E2E command.
  ```text
  command: `./run-tests.sh --e2e --name 'Developer cancels a review without changing the shell buffer'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 7 steps (7 passed).
  ```
- [x] COMMIT: Create one atomic commit with message
  `test(e2e): Developer cancels a review without changing the shell buffer`.
  Production files in commit: none (driver and PTY seam from `112ba6f`); E2E step bindings only. Commit hash: `d13f069`.

### Developer accepts a candidate from an interactive terminal request

Domain constraints:

- The real interface is a real `watn` subprocess with terminal stdin/stdout/
  stderr; primary assertion is command-output channel content.
- Review-eligible direct acceptance writes only the selected Candidate to the
  command-output channel. Review surface, Command flow, and Stage purpose text
  remain on the Controlling-terminal channel and must not contaminate stdout.
- Final acceptance is mandatory; the accepted Candidate is not evaluated on the
  direct interactive path.
- Non-TTY/redirected behavior remains outside this scenario.

- [x] RED: Remove `@wip` from this scenario only. Add non-empty E2E steps for
  configured provider fixture, real interactive request, review acceptance,
  stdout-only Candidate assertion, and absence of review bytes in normal
  command output. Run the exact targeted E2E command; it must exit non-zero.
  ```text
  command: `./run-tests.sh --e2e --name 'Developer accepts a candidate from an interactive terminal request'`
  output: exit 101; first new stub panicked (`not implemented`); 2 steps (1 passed, 1 failed).
  ```
- [x] GREEN: Drive the real terminal subprocess with the loopback provider
  fixture, accept the Candidate in the Review surface, and assert normal
  command output contains only `find . -type f`. Assert review bytes remain on
  the terminal-visible channel. Compile with `cargo check --locked`; run the
  exact targeted E2E command. Production files changed: none (CLI review path from `112ba6f`); E2E step bindings for the redirected-stdout PTY seam.
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --e2e --name 'Developer accepts a candidate from an interactive terminal request'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 5 steps (5 passed).
  ```
- [x] REFACTOR: Remove duplicate subprocess/channel capture logic while
  preserving the real-interface stdout assertion and no-review-contamination
  proof. Rerun the exact targeted E2E command.
  ```text
  command: `./run-tests.sh --e2e --name 'Developer accepts a candidate from an interactive terminal request'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 5 steps (5 passed).
  ```
- [x] COMMIT: Create one atomic commit with message
  `test(e2e): Developer accepts a candidate from an interactive terminal request`.
  Production files in commit: none (CLI review path from `112ba6f`); E2E step bindings for the redirected-stdout PTY seam. Commit hash: `35e06bb`.

### Developer accepts an eligible -x candidate and it executes once

Domain constraints:

- The real interface is `watn -x` in a real terminal PTY with the loopback
  provider fixture; the primary assertion is terminal-visible one-time
  execution.
- Eligible `-x` requires both explicit `-x` opt-in and final Review acceptance;
  acceptance is the sole execution authorization.
- Eligible `-x` must not show a second `Execute now?` confirmation and must
  execute exactly once.
- Review surface text is not execution output, and no command executes during
  generation or display.

- [x] RED: Remove `@wip` from this scenario only. Add non-empty E2E steps for
  the eligible `-x` PTY, provider fixture, Review acceptance, one execution,
  and no second confirmation. Run the exact targeted E2E command; it must exit
  non-zero.
  ```text
  command: `./run-tests.sh --e2e --name 'Developer accepts an eligible -x candidate and it executes once'`
  output: exit 101; scenario-specific Then stub panicked (`not implemented`); 4 steps (3 passed, 1 failed).
  ```
- [x] GREEN: Drive a real `watn -x "print reviewed"` PTY, accept the selected
  Candidate, and assert `reviewed` is printed exactly once with no second
  confirmation. Use the existing execution boundary and preserve channel
  separation. Compile with `cargo check --locked`; run the exact targeted E2E
  command. Production files changed: `src/main.rs` (review-eligible `-x` executes without printing the candidate to the command-output channel).
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --e2e --name 'Developer accepts an eligible -x candidate and it executes once'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 5 steps (5 passed).
  ```
- [x] REFACTOR: Consolidate PTY execution counting and confirmation assertions
  without changing the sole-authorization or exactly-once contract. Rerun the
  exact targeted E2E command.
  ```text
  command: `./run-tests.sh --e2e --name 'Developer accepts an eligible -x candidate and it executes once'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 5 steps (5 passed).
  ```
- [x] COMMIT: Create one atomic commit with message
  `test(e2e): Developer accepts an eligible -x candidate and it executes once`.
  Production files in commit: `src/main.rs` (review-eligible `-x` executes without printing the candidate to the command-output channel). Commit hash: `6d60718`.

## Final Verification

- [x] Run `givn lint --change use-explanatory-shell-shortcut`. Confirm all 30
  scenarios are implemented, no `@wip` finding remains, all four immutable
  `@e2e` tags remain present, and inventory/matrix mappings still match.
  Evidence:
  ```text
  command: `givn lint --change use-explanatory-shell-shortcut`
  output: givn lint: 1 file(s) checked — clean (exit 0; 30 scenarios; no @wip;
    the four inventory @e2e titles unchanged).
  ```

- [x] Run the exact regular verify command on the full suite:
  `./run-tests.sh`. Confirm zero exit and paste the scenario/step count. This
  is the GREEN gate for non-E2E behavior; do not substitute `givn lint` or an
  E2E-only run.
  ```text
  command: `./run-tests.sh`
  output: exit 0
  [Summary]
  21 features
  181 scenarios (181 passed)
  1087 steps (1087 passed)
  ```

- [x] Run the exact E2E verify command on the full E2E suite:
  `./run-tests.sh --e2e`. Confirm zero exit and paste the scenario/step count.
  Confirm the count remains strictly smaller than the full in-scope count and
  that the four change interactions are included.
  ```text
  command: `./run-tests.sh --e2e`
  output: exit 0
  [Summary]
  25 features
  81 scenarios (81 passed)
  592 steps (592 passed)
  count proof: full regular = 181 scenarios; E2E = 81 scenarios; 81 < 181, so
    the E2E runner remains a strict subset. The four change interactions are
    included: `Developer accepts an explained candidate from Ctrl-W`,
    `Developer cancels a review without changing the shell buffer`,
    `Developer accepts a candidate from an interactive terminal request`, and
    `Developer accepts an eligible -x candidate and it executes once`.
  ```

- [x] Run `cargo check --locked` and confirm the final implementation is
  compile-clean. Evidence:
  ```text
  command: `cargo check --locked`
  output: Finished `dev` profile [unoptimized + debuginfo] target(s) — no
    warnings or errors.
  ```

- [x] Run `givn status --change use-explanatory-shell-shortcut` and confirm
  every scenario task has evidence and an atomic commit hash, `tasks` is
  complete, and the next artifact is `review`. Evidence:
  ```text
  command: `givn status --change use-explanatory-shell-shortcut`
  output: all 135 tasks checked; every scenario task records its runner output
    and atomic commit hash; artifacts proposal, specs, design, arc42-docs,
    design-review, tasks are complete; next required artifact is `review`.
  ```
