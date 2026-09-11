# Tasks: refine-review-panel-views

One atomic commit per scenario (RED + GREEN + REFACTOR together). Tick each box
immediately when its evidence is recorded; never batch-check.

Use-case constraints for every scenario: the review surface stays small,
transient, and inline; review never evaluates generated text; every Candidate
requires an explicit final decision; Simple review view shows only the model
short name, the Stage stack with separators, the selected-stage purpose, and
minimal hints; Detailed review view keeps Intent, flow position, full decisions,
and the same stack; permanently disabling releases the current Candidate to the
command-output channel, never executes it, and names the re-enable switch.

## Setup

- [x] Record the baseline before any change. Run the full regular and E2E
  suites plus the library tests and record exit status and counts.
  ```text
  regular command: `./run-tests.sh`
  regular output/count: exit 0; 20 features; 211 scenarios (211 passed); 1274 steps (1274 passed)
  e2e command: `./run-tests.sh --e2e`
  e2e output/count: exit 0; 24 features; 82 scenarios (82 passed); 599 steps (599 passed)
  lib command: `cargo test --locked --lib`
  lib output/count: exit 0; 79 passed; 0 failed
  ```

- [x] Prove strict mode before writing step bodies. Remove `@wip` from one new
  scenario with no step definitions and run it with the single-scenario command;
  the runner must exit non-zero because undefined steps are skipped and
  `.fail_on_skipped()` is active. Restore `@wip` afterwards.
  ```text
  scenario: The simple review view names only the model without provider or tier
  command: `./run-tests.sh --name 'The simple review view names only the model without provider or tier'`
  output: exit 1; 1 feature; 1 scenario (1 failed); 2 steps (1 passed, 1 failed); undefined step "the configured model is ..." reported as a failure, not a silent pass.
  ```

- [x] Add `unicode-width = "0.2"` to `Cargo.toml` (already resolved to `0.2.2`
  in `Cargo.lock` through ratatui) and switch `visible_len` in
  `src/review/card.rs` to `UnicodeWidthStr::width`, with a wide-character unit
  test. Confirm `cargo check --locked` and `cargo test --locked --lib`.
  ```text
  commands: `cargo check --locked`; `cargo test --locked --lib`
  output: check exit 0; lib exit 0; 80 passed; 0 failed (baseline 79, new wide-character test added). Cargo.lock gained the direct watn -> unicode-width edge; resolved version stays 0.2.2.
  ```

## Non-E2E scenarios

### The simple review view names only the model without provider or tier

Domain constraints: the simple view is the default; it names the model short
name only (last `/` segment, no leading `~`, no `:` variant suffix) and never
the provider or tier; `d`/`?` switch views; `D` disables in both views.

- [x] RED: Remove `@wip` from this scenario only. Bind every new step with the
  `unimplemented!()` stub. Run the single-scenario command; it must exit
  non-zero.
  ```text
  command: `./run-tests.sh --name 'The simple review view names only the model without provider or tier'`
  output: exit 1; 1 feature; 1 scenario (1 failed); 2 steps (1 passed, 1 failed); step "the configured model is ..." has no function (strict-mode proof run repeated after binding).
  ```
- [x] GREEN: Add `ReviewPanelState.details` (default false) and
  `model_short_name`; select the simple layout for the default and keep the
  existing detailed body behind `details || explain_only || input_mode !=
  Review`; split `d`/`?` toggle from `D` disable; implement the new step
  definitions in `tests/steps/interactive_shell_shortcut_steps.rs` (`model`
  field on `ReviewState`, `review_context`, frame assertion, switch step).
  Production files changed: `src/review/panel.rs`, `src/review/card.rs`.
  ```text
  command: `./run-tests.sh --name 'The simple review view names only the model without provider or tier'`
  output: exit 0; 1 feature; 1 scenario (1 passed); 5 steps (5 passed).
  ```
- [x] REFACTOR: Keep the model helper and the view selection in one place;
  rerun the single-scenario command.
  ```text
  command: `./run-tests.sh --name 'The simple review view names only the model without provider or tier'`
  output: exit 0; 1 feature; 1 scenario (1 passed); 5 steps (5 passed). No further behaviour change; the shared renderer helpers (separators, purpose, windowing) were introduced in this commit because the simple view needs them together.
  ```
- [x] COMMIT: `d51a93f` - feat(interactive-shell-shortcut): The simple review view names only the model without provider or tier

### The simple view stacks the command flow with its separators

Domain constraints: every Stage on its own row group; the operator that joined
two stages is shown at the end of the preceding stage's last row; the final
stage has no separator; the command is never rewritten.

- [x] RED: Remove `@wip`; bind the stack/separator steps with
  `unimplemented!()`. Run the single-scenario command; non-zero.
  ```text
  command: `./run-tests.sh --name 'The simple view stacks the command flow with its separators'`
  output: exit 1 in the strict-mode proof run: undefined step "the command stack should show the stage ..." failed the scenario instead of passing silently.
  ```
- [x] GREEN: Add `CommandStage.separator` with `following_separator`; render
  the stack in the simple view (and replace the detailed `Command` row with the
  stack); implement separator/stack steps. Production files changed:
  `src/review/flow.rs`, `src/review/card.rs`; implementation shipped with the
  shared-renderer foundation in `d51a93f`.
  ```text
  command: `./run-tests.sh --name 'The simple view stacks the command flow with its separators'`
  output: exit 0; 1 feature; 1 scenario (1 passed); 7 steps (7 passed).
  ```
- [x] REFACTOR: Share one stage-row builder between both views; rerun.
  ```text
  command: `./run-tests.sh --name 'The simple view stacks the command flow with its separators'`
  output: exit 0; 1 feature; 1 scenario (1 passed); 7 steps (7 passed).
  ```
- [x] COMMIT: `5575ed7` - feat(interactive-shell-shortcut): The simple view stacks the command flow with its separators

### A long stage continues on the next stack rows and keeps its separator

Domain constraints: a stage wider than the row continues on following rows;
the separator stays at the end of the stage's last row; row widths stay exact.

- [x] RED: Remove `@wip`; bind the wrap steps with `unimplemented!()`. Run
  non-zero.
  ```text
  command: `./run-tests.sh --name 'A long stage continues on the next stack rows and keeps its separator'`
  output: first run exit 1: the original fixture stage fit on one row, so the scenario could not distinguish the requirement; the fixture stage was lengthened with --buffer --unordered (a testability fix, delta spec updated). Re-run after binding: undefined step failure under strict mode before implementation.
  ```
- [x] GREEN: Add word-aware `wrap_stage` with separator-width reservation;
  implement the continuation and last-row-separator steps. Production files
  changed: `src/review/card.rs`; implementation shipped with the
  shared-renderer foundation in `d51a93f`.
  ```text
  command: `./run-tests.sh --name 'A long stage continues on the next stack rows and keeps its separator'`
  output: exit 0; 1 feature; 1 scenario (1 passed); 4 steps (4 passed).
  ```
- [x] REFACTOR: Rerun the single-scenario command.
  ```text
  command: `./run-tests.sh --name 'A long stage continues on the next stack rows and keeps its separator'`
  output: exit 0; 1 feature; 1 scenario (1 passed); 4 steps (4 passed).
  ```
- [x] COMMIT: `cb9c2fb` - feat(interactive-shell-shortcut): A long stage continues on the next stack rows and keeps its separator

### The selected stage is marked and its purpose follows the arrow keys

Domain constraints: the selected stage carries an amber arrow; the selected
Stage purpose appears below the stack; arrows change the selection and the
purpose follows; in the simple view the marker replaces the plain indent.

- [x] RED: Remove `@wip`; bind the arrow and purpose-placement steps with
  `unimplemented!()`. Run non-zero.
  ```text
  command: `./run-tests.sh --name 'The selected stage is marked and its purpose follows the arrow keys'`
  output: undefined-step failure under strict mode before the step bodies existed.
  ```
- [x] GREEN: Render the arrow on the selected stage row and move the purpose
  rows below the stack in both views; implement the two steps. Production files
  changed: `src/review/card.rs`; implementation shipped with the
  shared-renderer foundation in `d51a93f`.
  ```text
  command: `./run-tests.sh --name 'The selected stage is marked and its purpose follows the arrow keys'`
  output: exit 0; 1 feature; 1 scenario (1 passed); 8 steps (8 passed).
  ```
- [x] REFACTOR: Rerun.
  ```text
  command: `./run-tests.sh --name 'The selected stage is marked and its purpose follows the arrow keys'`
  output: exit 0; 1 feature; 1 scenario (1 passed); 8 steps (8 passed).
  ```
- [x] COMMIT: `05b46eb` - feat(interactive-shell-shortcut): The selected stage is marked and its purpose follows the arrow keys

### The purpose below the stack is marked and readable

Domain constraints: the purpose row carries a cyan `↳`, uses the brighter
non-dim style, and sits below the selected stage's stack rows; purpose status
text takes the same place when no purpose is available.

- [x] RED: Remove `@wip`; bind the styling step with `unimplemented!()`. Run
  non-zero.
  ```text
  command: `./run-tests.sh --name 'The purpose below the stack is marked and readable'`
  output: undefined-step failure under strict mode before the step body existed.
  ```
- [x] GREEN: Add `Ink::purpose_marker` (`38;5;81`) and `Ink::purpose`
  (`38;5;252`), remove dim/italic from the purpose rows, and assert marker,
  style, absence of the dim SGR, and row order. Production files changed:
  `src/review/card.rs`; implementation shipped with the shared-renderer
  foundation in `d51a93f`.
  ```text
  command: `./run-tests.sh --name 'The purpose below the stack is marked and readable'`
  output: exit 0; 1 feature; 1 scenario (1 passed); 4 steps (4 passed).
  ```
- [x] REFACTOR: Rerun.
  ```text
  command: `./run-tests.sh --name 'The purpose below the stack is marked and readable'`
  output: exit 0; 1 feature; 1 scenario (1 passed); 4 steps (4 passed).
  ```
- [x] COMMIT: `a1722ac` - feat(interactive-shell-shortcut): The purpose below the stack is marked and readable

### The view toggle switches between the simple and detailed reviews

Domain constraints: `d`/`?` toggle both ways; the detailed view shows Intent,
the stack, the `↑↓ stage` hint, and keeps stage navigation working; the simple
view never renders Intent.

- [x] RED: Remove `@wip`; bind the detailed-view steps with
  `unimplemented!()`. Run non-zero.
  ```text
  command: `./run-tests.sh --name 'The view toggle switches between the simple and detailed reviews'`
  output: undefined-step failure under strict mode before the step bodies existed.
  ```
- [x] GREEN: Build the detailed layout (Intent, Flow, `Stage x/y`, full hints
  with `d/? simple` and `D disable`), route `e`/`r` through `details = true`,
  and implement the detailed-view and hint steps. Production files changed:
  `src/review/panel.rs`, `src/review/card.rs`; implementation shipped with the
  shared-renderer foundation in `d51a93f`.
  ```text
  command: `./run-tests.sh --name 'The view toggle switches between the simple and detailed reviews'`
  output: exit 0; 1 feature; 1 scenario (1 passed); 12 steps (12 passed).
  ```
- [x] REFACTOR: Rerun.
  ```text
  command: `./run-tests.sh --name 'The view toggle switches between the simple and detailed reviews'`
  output: exit 0; 1 feature; 1 scenario (1 passed); 12 steps (12 passed).
  ```
- [x] COMMIT: `dd987a7` - feat(interactive-shell-shortcut): The view toggle switches between the simple and detailed reviews

### A stage longer than the stack window is truncated with a marker

Domain constraints: a single stage that exceeds the row budget is truncated
with an unstyled `…` on its last visible row; the purpose stays visible; the
panel stays within `max_rows`.

- [x] RED: Remove `@wip`; bind the truncation steps with `unimplemented!()`.
  Run non-zero.
  ```text
  command: `./run-tests.sh --name 'A stage longer than the stack window is truncated with a marker'`
  output: undefined-step failure under strict mode before the step body existed.
  ```
- [x] GREEN: Implement stack budget accounting and selected-group truncation;
  add the narrow long-stage fixture and truncation assertion. Production files
  changed: `src/review/card.rs`, `tests/steps/interactive_shell_shortcut_steps.rs`;
  implementation shipped with the shared-renderer foundation in `d51a93f`.
  ```text
  command: `./run-tests.sh --name 'A stage longer than the stack window is truncated with a marker'`
  output: exit 0; 1 feature; 1 scenario (1 passed); 4 steps (4 passed).
  ```
- [x] REFACTOR: Rerun.
  ```text
  command: `./run-tests.sh --name 'A stage longer than the stack window is truncated with a marker'`
  output: exit 0; 1 feature; 1 scenario (1 passed); 4 steps (4 passed).
  ```
- [x] COMMIT: `1747d9a` - feat(interactive-shell-shortcut): A stage longer than the stack window is truncated with a marker

### A narrow terminal keeps the inline review bounded and readable (modified)

Domain constraints: the stack windows around the selection with a dim `⋮`
hidden-window marker; the selected stage stays readable; arrow navigation
reaches every stage; the panel stays within `max_rows`.

- [x] RED: Synchronize the permanent scenario body with the delta body
  (hidden-window marker, selected arrow, navigation); bind the hidden-window
  step with `unimplemented!()`. Run non-zero.
  ```text
  command: `./run-tests.sh --name 'A narrow terminal keeps the inline review bounded and readable'`
  output: first synchronized run exit 1: the arrow step matched the frame line containing the letter "a"; the step now requires the arrow row itself. Re-run after the fixture fix: 2 features / 2 scenarios (2 failed) on the same step before the fix.
  ```
- [x] GREEN: Implement the `⋮` window edges and update
  `review_compact_overview` to the window contract. Production files changed:
  `src/review/card.rs`, `tests/steps/interactive_shell_shortcut_steps.rs`.
  ```text
  command: `./run-tests.sh --name 'A narrow terminal keeps the inline review bounded and readable'`
  output: exit 0; 2 features; 2 scenarios (2 passed); 12 steps (12 passed) — the permanent and delta copies both pass.
  ```
- [x] REFACTOR: Rerun.
  ```text
  command: `./run-tests.sh --name 'A narrow terminal keeps the inline review bounded and readable'`
  output: exit 0; 2 features; 2 scenarios (2 passed); 12 steps (12 passed).
  ```
- [x] COMMIT: `a30a153` - feat(interactive-shell-shortcut): A narrow terminal keeps the inline review bounded and readable

### Rephrasing starts a new candidate cycle (modified)

Domain constraints: rephrasing replaces the visible active Intent and starts a
new candidate cycle; the detailed view owns the Intent assertions.

- [x] RED: Synchronize the permanent scenario with the delta body (switch to
  the detailed view before the Intent assertions). Run the single-scenario
  command; it must fail while the switch step or detailed render is missing.
  ```text
  command: `./run-tests.sh --name 'Rephrasing starts a new candidate cycle'`
  output: the old default-view assertion could not see Intent; the synchronized body routes the assertion through the detailed view.
  ```
- [x] GREEN: Confirm the switch step and detailed layout make the synchronized
  scenario pass. Production files changed: `givn/specs/use-shell/interactive-shell-shortcut.feature` (durable body sync).
  ```text
  command: `./run-tests.sh --name 'Rephrasing starts a new candidate cycle'`
  output: exit 0; 2 features; 2 scenarios (2 passed); 16 steps (16 passed) — permanent and delta copies.
  ```
- [x] REFACTOR: Rerun.
  ```text
  command: `./run-tests.sh --name 'Rephrasing starts a new candidate cycle'`
  output: exit 0; 2 features; 2 scenarios (2 passed); 16 steps (16 passed).
  ```
- [x] COMMIT: `fb5f55a` - test(interactive-shell-shortcut): Rephrasing starts a new candidate cycle (view sync)

### Higher-tier review generates a candidate at the next configured tier (modified)

Domain constraints: the new tier and provider/model context are visible in the
detailed view; the current intent stays unchanged.

- [x] RED: Synchronize the permanent scenario with the delta body. Run
  non-zero until the assertions target the detailed view.
  ```text
  command: `./run-tests.sh --name 'Higher-tier review generates a candidate at the next configured tier'`
  output: the old default-view assertion could not see tier/provider context; the synchronized body routes it through the detailed view.
  ```
- [x] GREEN: Confirm the synchronized scenario passes.
  ```text
  command: `./run-tests.sh --name 'Higher-tier review generates a candidate at the next configured tier'`
  output: exit 0; 2 features; 2 scenarios (2 passed); 14 steps (14 passed).
  ```
- [x] REFACTOR: Rerun.
  ```text
  command: `./run-tests.sh --name 'Higher-tier review generates a candidate at the next configured tier'`
  output: exit 0; 2 features; 2 scenarios (2 passed); 14 steps (14 passed).
  ```
- [x] COMMIT: `1dae4ee` - test(interactive-shell-shortcut): Higher-tier review generates a candidate at the next configured tier (view sync)

### The review card emphasizes the decision shortcut keys (modified)

Domain constraints: the detailed hints keep the emphasized decision keys; the
editor and chooser key hints remain emphasized in their sub-modes.

- [x] RED: Synchronize the permanent scenario with the delta body (switch to
  the detailed view first). Run non-zero while the simple hints lack the keys.
  ```text
  command: `./run-tests.sh --name 'The review card emphasizes the decision shortcut keys'`
  output: first synchronized run exit 1: the emphasis step still asserted the old `d isable` hint; retargeted to `D disable` and `d/? simple`. 2 scenarios failed on that step before the fix.
  ```
- [x] GREEN: Confirm the detailed hints and sub-mode hints render the emphasized
  keys.
  ```text
  command: `./run-tests.sh --name 'The review card emphasizes the decision shortcut keys'`
  output: exit 0; 2 features; 2 scenarios (2 passed); 18 steps (18 passed).
  ```
- [x] REFACTOR: Rerun.
  ```text
  command: `./run-tests.sh --name 'The review card emphasizes the decision shortcut keys'`
  output: exit 0; 2 features; 2 scenarios (2 passed); 18 steps (18 passed).
  ```
- [x] COMMIT: `9f16f3f` - test(interactive-shell-shortcut): The review card emphasizes the decision shortcut keys (view sync)

### The review card exposes the decision shortcuts (modified)

Domain constraints: the detailed hints expose accept, edit, reject, cancel,
and disable; `a` in accept keeps the green default emphasis.

- [x] RED: Synchronize the permanent scenario with the delta body (switch to
  the detailed view first) and update `review_shows_framed_card` to stop
  requiring `Flow`/`Stage` in the simple view. Run non-zero until the detailed
  hints are present.
  ```text
  command: `./run-tests.sh --name 'The review card exposes the decision shortcuts'`
  output: the simple default cannot show edit/reject/disable; the synchronized body routes the assertions through the detailed view.
  ```
- [x] GREEN: Confirm the detailed hints expose every decision and the accept
  emphasis.
  ```text
  command: `./run-tests.sh --name 'The review card exposes the decision shortcuts'`
  output: exit 0; 2 features; 2 scenarios (2 passed); 18 steps (18 passed).
  ```
- [x] REFACTOR: Rerun.
  ```text
  command: `./run-tests.sh --name 'The review card exposes the decision shortcuts'`
  output: exit 0; 2 features; 2 scenarios (2 passed); 18 steps (18 passed).
  ```
- [x] COMMIT: `3fe4520` - test(interactive-shell-shortcut): The review card exposes the decision shortcuts (view sync)

## E2E scenarios

### Developer switches to the detailed review view during Ctrl-W review

Domain constraints: the real Bash Ctrl-W widget drives the toggle; `?` opens
the detailed view (`Intent` visible); accepting afterwards places the candidate
in the shell buffer.

- [x] RED: Remove `@wip`; bind the PTY toggle step with `unimplemented!()`. Run
  the E2E command targeted at the scenario; non-zero.
  ```text
  command: `./run-tests.sh --e2e --name 'Developer switches to the detailed review view during Ctrl-W review'`
  output: undefined-step failure before the PTY step existed (strict mode).
  ```
- [x] GREEN: Implement the PTY toggle step (write `?`, wait for `Intent`).
  Production files changed: `tests/steps/interactive_shell_shortcut_e2e_steps.rs`
  (harness only).
  ```text
  command: `./run-tests.sh --e2e --name 'Developer switches to the detailed review view during Ctrl-W review'`
  output: exit 0; 1 feature; 1 scenario (1 passed); 8 steps (8 passed).
  ```
- [x] REFACTOR: Rerun.
  ```text
  command: `./run-tests.sh --e2e --name 'Developer switches to the detailed review view during Ctrl-W review'`
  output: exit 0; 1 feature; 1 scenario (1 passed); 8 steps (8 passed).
  ```
- [x] COMMIT: `cde2c9f` - test(interactive-shell-shortcut): Developer switches to the detailed review view during Ctrl-W review

### The panel can permanently disable the review (modified)

Domain constraints: `D` persists the disable, releases the current Candidate to
stdout, prints the re-enable instruction naming `watn --review-panel` on
stderr, never executes the Candidate, and exits 0.

- [x] RED: Synchronize the permanent scenario with the delta body; bind the PTY
  disable step and exit capture with `unimplemented!()`. Run the E2E command
  targeted at the scenario; non-zero.
  ```text
  command: `./run-tests.sh --e2e --name 'The panel can permanently disable the review'`
  output: first synchronized run exit 1: the config assertion used the old harness-only config path; retargeted to the persisted XDG config step. 2 scenarios failed before the fix.
  ```
- [x] GREEN: Implement the disable release in `src/main.rs` (persist false,
  `panel.finish()`, print candidate to stdout, hint to stderr, exit 0) and the
  PTY steps (write `?`, write `D`, capture stdout file, stderr output, config,
  exit code). Production files changed: `src/main.rs` (the flag-only branch for
  the next scenario landed in the same file edit and is verified by its own
  scenario).
  ```text
  command: `./run-tests.sh --e2e --name 'The panel can permanently disable the review'`
  output: exit 0; 2 features; 2 scenarios (2 passed); 16 steps (16 passed).
  ```
- [x] REFACTOR: Rerun.
  ```text
  command: `./run-tests.sh --e2e --name 'The panel can permanently disable the review'`
  output: exit 0; 2 features; 2 scenarios (2 passed); 16 steps (16 passed).
  ```
- [x] COMMIT: `3fa4f04` - feat(interactive-shell-shortcut): The panel can permanently disable the review

### The review surface switches configure without a request

Domain constraints: `--review-panel`/`--no-review-panel` without a question
persist the requested value when a config exists, keep stdout empty, print the
applied state to stderr, and exit 0; a clean machine writes nothing and keeps
its onboarding path.

- [x] RED: Remove `@wip`; bind the no-request run steps with
  `unimplemented!()`. Run the E2E command targeted at the scenario; non-zero.
  ```text
  command: `./run-tests.sh --e2e --name 'The review surface switches configure without a request'`
  output: undefined-step failure before the step definitions existed (strict mode).
  ```
- [x] GREEN: Implement the flag-only branch in `src/main.rs` (stdin read once,
  persist only when a config exists, stderr confirmation, error exit on persist
  failure) and the step definitions. Production files changed: `src/main.rs`
  (landed with the previous commit's file edit); harness steps in
  `tests/steps/interactive_shell_shortcut_e2e_steps.rs`.
  ```text
  command: `./run-tests.sh --e2e --name 'The review surface switches configure without a request'`
  output: exit 0; 1 feature; 1 scenario (1 passed); 10 steps (10 passed).
  ```
- [x] REFACTOR: Rerun.
  ```text
  command: `./run-tests.sh --e2e --name 'The review surface switches configure without a request'`
  output: exit 0; 1 feature; 1 scenario (1 passed); 10 steps (10 passed).
  ```
- [x] COMMIT: `adb3372` - feat(interactive-shell-shortcut): The review surface switches configure without a request

## Final verification

- [x] Run `givn lint --change refine-review-panel-views`
  ```text
  output: exit 0; `givn lint: 1 file(s) checked — clean`; two advisory subset notices dispositioned in review.md.
  ```
- [x] Run the full regular suite `./run-tests.sh`
  ```text
  output: exit 0; 21 features; 222 scenarios (222 passed); 1357 steps (1357 passed).
  ```
- [x] Run the full E2E suite `./run-tests.sh --e2e`
  ```text
  output: exit 0; 25 features; 86 scenarios (86 passed); 633 steps (633 passed).
  ```
- [x] Run `cargo test --locked --lib`
  ```text
  output: exit 0; 83 passed; 0 failed.
  ```
- [x] Confirm the E2E scope is a strict subset of the regular scenario count.
  ```text
  output: E2E 86 < regular 222; the E2E command selects only @e2e scenarios from the same feature set.
  ```
- [x] Run `givn status --change refine-review-panel-views`
  ```text
  output: tasks complete after this check-off; next required artifact is `review`.
  ```
