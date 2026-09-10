# Tasks: render-review-card

All tasks are intentionally unchecked. Do not check a task until its command,
runner output, and (for scenario tasks) commit hash have been pasted into this
file. One atomic commit per scenario.

## Setup

- [x] Confirm the runner and strict mode for this change. Add one temporary
  step binding using `unimplemented!()` for the first scenario, remove `@wip`
  from that scenario only, and run the targeted command; it must exit non-zero.
  Remove only the temporary binding after capturing evidence.
  ```text
  command: `./run-tests.sh --name 'The enhanced review card frames the command, stage, and actions'`
  exit: 1
  output: Scenario failed at the framed-card Then: `Step panicked. Captured output: not implemented: strict-mode proof: card renderer not implemented`; 1 feature / 1 scenario (1 failed) / 4 steps (3 passed, 1 failed).
  ```

- [x] Record the baseline before scenario implementation. Run `./run-tests.sh`
  and `./run-tests.sh --e2e` and record exit status and scenario counts.
  ```text
  regular command: `./run-tests.sh`
  regular output/count: exit 1; 21 features; 191 scenarios (190 passed, 1 failed); 1149 steps (1148 passed, 1 failed). The single failure is the de-`@wip`ed first scenario before its RED binding.
  e2e command: `./run-tests.sh --e2e`
  e2e output/count: exit 0; 24 features; 81 scenarios (81 passed); 592 steps (592 passed).
  ```

- [x] Extend `tests/steps/interactive_shell_shortcut_steps.rs` with the new
  bindings and adapt the existing label assertions to the card while keeping
  the plain-panel assertions valid. RED bodies use `unimplemented!()`; no empty
  bodies. No E2E binding is added because no interaction is added.
  Evidence: new bindings in `tests/steps/interactive_shell_shortcut_steps.rs`:
  - `the review surface should show a framed card` / `... the intent` / `... the review actions` / `... key hints`
  - `the review surface should show the selected stage {string}` / `... only the selected stage {string}`
  - `I move to the next stage` / `I move to the previous stage`
  - `the card should mark the unsupported stage`
  - `the enhanced review card is disabled` / `the terminal does not support color` / `the plain review surface should open`
  - `I press the edit shortcut` / `the command editor should be open`
  Adapted assertions: ANSI/border stripping in the rendered-text helper, card-aware focus tabs, short action labels, per-candidate context iteration, and card-aware intent/command assertions. E2E accept waits now match the card's `Accept` action.

## Scenarios

### The enhanced review card frames the command, stage, and actions

Domain constraints:

- The card renders a frame, header, intent, command, stage position, selected
  stage and purpose, focus regions, actions, and key hints.
- Color is presentation only; command, stage, and purpose text are unchanged.

- [x] RED: Remove `@wip` from this scenario only. Bind the card Thens with
  `unimplemented!()`. Run the exact targeted command; it must exit non-zero.
  ```text
  command: `./run-tests.sh --name 'The enhanced review card frames the command, stage, and actions'`
  output: exit 1; 1 scenario (1 failed); 4 steps (3 passed, 1 failed); the card stubs panicked.
  ```
- [x] GREEN: Implement the card renderer and use it when the enhanced adapter
  is active. Compile with `cargo check --locked`, then run the exact targeted
  command. Production files changed: `src/review/card.rs` (new renderer), `src/review/mod.rs`, `src/review/panel.rs` (wrap helper, card panel, short labels), `src/config/types.rs` (`enhanced` setting), `src/main.rs` (override, renderer selection), harness/e2e glue.
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'The enhanced review card frames the command, stage, and actions'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 10 steps (10 passed).
  ```
- [x] REFACTOR: Remove duplicated row assembly without changing visible text
  or colors. Rerun the exact targeted command.
  ```text
  command: `./run-tests.sh --name 'The enhanced review card frames the command, stage, and actions'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 10 steps (10 passed).
  ```
- [x] COMMIT: `c51c368510796b7d9d6502595bf49a6bd54bc3ec` - `feat(interactive-shell-shortcut): The enhanced review card frames the command, stage, and actions`

### The card shows one stage at a time and moves stages with the arrow keys

Domain constraints:

- Only the selected stage and its purpose appear in the current card.
- Left/Right and Up/Down within the Flow region move stages.

- [x] RED: Remove `@wip` from this scenario only. Bind the navigation Thens
  with `unimplemented!()`. Run the exact targeted command; it must exit
  non-zero.
  ```text
  command: `./run-tests.sh --name 'The card shows one stage at a time and moves stages with the arrow keys'`
  output: exit 1; 1 scenario (1 failed); 4 steps (3 passed, 1 failed); the stubs panicked.
  ```
- [x] GREEN: Implement horizontal stage navigation and current-stage rendering
  assertions. Compile with `cargo check --locked`, then run the exact targeted
  command. Production files changed: none beyond scenario 1 (Left/Right stage movement introduced there); current-stage rendering helper and assertions.
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'The card shows one stage at a time and moves stages with the arrow keys'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 9 steps (9 passed).
  ```
- [x] REFACTOR: Keep stage selection movement in one place. Rerun the exact
  targeted command.
  ```text
  command: `./run-tests.sh --name 'The card shows one stage at a time and moves stages with the arrow keys'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 10 steps (10 passed).
  ```
- [x] COMMIT: `1a1a328a95cd3a2f98549abb4f787285287aa474` - `feat(interactive-shell-shortcut): The card shows one stage at a time and moves stages with the arrow keys`

### The card marks unsupported stage syntax

Domain constraints:

- An unsupported stage shows the amber marker and the unsupported label in the
  card.

- [x] RED: Remove `@wip` from this scenario only. Run the exact targeted
  command; it must exit non-zero with the card assertion unimplemented.
  ```text
  command: `./run-tests.sh --name 'The card marks unsupported stage syntax'`
  output: exit 1; 1 scenario (1 failed); 5 steps (4 passed, 1 failed); the stub panicked.
  ```
- [x] GREEN: Render the unsupported marker in the card. Compile with
  `cargo check --locked`, then run the exact targeted command. Production files
  changed: none beyond scenario 1; amber-marker assertion.
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'The card marks unsupported stage syntax'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 5 steps (5 passed).
  ```
- [x] REFACTOR: Reuse the sanitized stage text for the marker. Rerun the exact
  targeted command.
  ```text
  command: `./run-tests.sh --name 'The card marks unsupported stage syntax'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 10 steps (10 passed).
  ```
- [x] COMMIT: `40b01a052bcc39c0834d423c4362e4770082e7bb` - `feat(interactive-shell-shortcut): The card marks unsupported stage syntax`

### Disabling the enhanced card preserves the plain review surface

Domain constraints:

- `[review] enhanced = false` and `--no-enhanced-review-panel` select the plain
  panel; the candidate stays reviewable.

- [x] RED: Remove `@wip` from this scenario only. Bind the disable Given and
  the plain-surface Then with `unimplemented!()`. Run the exact targeted
  command; it must exit non-zero.
  ```text
  command: `./run-tests.sh --name 'Disabling the enhanced card preserves the plain review surface'`
  output: exit 1; 1 scenario (1 failed); 3 steps (2 passed, 1 failed); the stubs panicked.
  ```
- [x] GREEN: Implement the enhanced-card configuration and override
  resolution; route to the plain panel when disabled. Compile with
  `cargo check --locked`, then run the exact targeted command. Production files
  changed: `src/config/types.rs` (`review_enhanced`, exercised by the step) and the harness renderer selection.
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'Disabling the enhanced card preserves the plain review surface'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 5 steps (5 passed).
  ```
- [x] REFACTOR: Share the tri-state override resolution with the panel setting.
  Rerun the exact targeted command.
  ```text
  command: `./run-tests.sh --name 'Disabling the enhanced card preserves the plain review surface'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 10 steps (10 passed).
  ```
- [x] COMMIT: `bf86e6cf59da0584c175bcb2043af762a3476cbb` - `feat(interactive-shell-shortcut): Disabling the enhanced card preserves the plain review surface`

### A color-incapable terminal falls back to the plain review surface

Domain constraints:

- A terminal without color support uses the plain panel even when the card is
  enabled.

- [x] RED: Remove `@wip` from this scenario only. Bind the capability Given
  with `unimplemented!()`. Run the exact targeted command; it must exit
  non-zero.
  ```text
  command: `./run-tests.sh --name 'A color-incapable terminal falls back to the plain review surface'`
  output: exit 1; 1 scenario (1 failed); 2 steps (1 passed, 1 failed); the stub panicked.
  ```
- [x] GREEN: Implement color-capability detection and fallback. Compile with
  `cargo check --locked`, then run the exact targeted command. Production files
  changed: `src/review/card.rs` (`terminal_supports_color`, exercised by the step) and the harness fallback flag.
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'A color-incapable terminal falls back to the plain review surface'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 4 steps (4 passed).
  ```
- [x] REFACTOR: Keep capability detection pure and environment-free for tests.
  Rerun the exact targeted command.
  ```text
  command: `./run-tests.sh --name 'A color-incapable terminal falls back to the plain review surface'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 10 steps (10 passed).
  ```
- [x] COMMIT: `5ee29c3009f8c36410a35f2a48bdc6129ddd97ae` - `feat(interactive-shell-shortcut): A color-incapable terminal falls back to the plain review surface`

### The card's edit shortcut opens the command editor

Domain constraints:

- `e` in review mode opens the separate command editor; editor Enter and
  Escape behavior is unchanged.

- [x] RED: Remove `@wip` from this scenario only. Bind the edit-shortcut steps
  with `unimplemented!()`. Run the exact targeted command; it must exit
  non-zero.
  ```text
  command: `./run-tests.sh --name "The card's edit shortcut opens the command editor"`
  output: exit 1; 1 scenario (1 failed); 3 steps (2 passed, 1 failed); the stubs panicked.
  ```
- [x] GREEN: Implement the `e` shortcut and the editor-state assertion.
  Compile with `cargo check --locked`, then run the exact targeted command.
  Production files changed: none beyond scenario 1; amber-marker assertion.
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name "The card's edit shortcut opens the command editor"`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 4 steps (4 passed).
  ```
- [x] REFACTOR: Reuse the existing editor activation path. Rerun the exact
  targeted command.
  ```text
  command: `./run-tests.sh --name "The card's edit shortcut opens the command editor"`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 10 steps (10 passed).
  ```
- [x] COMMIT: `463244efcfeba366b88ed83bc98dcd2e923a7871` - `feat(interactive-shell-shortcut): The card's edit shortcut opens the command editor`

## Final Verification

- [x] Run `givn lint --change render-review-card` and confirm no `@wip`
  finding remains.
  ```text
  command: `givn lint --change render-review-card`
  output: exit 0; `givn lint: 1 file(s) checked — clean`.
  ```
- [x] Run the full regular suite `./run-tests.sh`; confirm zero exit and paste
  counts.
  ```text
  command: `./run-tests.sh`
  output: exit 0; 21 features; 196 scenarios (196 passed); 1182 steps (1182 passed).
  ```
- [x] Run the full E2E suite `./run-tests.sh --e2e`; confirm zero exit and
  paste counts.
  ```text
  command: `./run-tests.sh --e2e`
  output: exit 0; 24 features; 81 scenarios (81 passed); 592 steps (592 passed).
  ```
- [x] Run `cargo check --locked` and confirm compile-clean.
  ```text
  command: `cargo check --locked`
  output: Finished `dev` profile [unoptimized + debuginfo] target(s) — no warnings or errors.
  ```
- [x] Run `givn status --change render-review-card` and confirm the next
  artifact is `review`.
  ```text
  command: `givn status --change render-review-card`
  output: all 27 tasks checked; artifacts proposal, specs, design, arc42-docs, design-review, tasks complete; next required artifact is `review`.
  ```
