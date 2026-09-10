# Tasks: clarify-review-card-guidance

All tasks are intentionally unchecked. One atomic commit per scenario.
Use-case constraints: the card stays small and transient; key behavior,
regeneration, and the flow splitter never change; mono cards stay
escape-free.

## Setup

- [x] Record the baseline before scenario implementation. Run `./run-tests.sh`
  and `./run-tests.sh --e2e` and record exit status and scenario counts.
  ```text
  regular command: `./run-tests.sh`
  regular output/count: exit 0; 209 scenarios (209 passed).
  e2e command: `./run-tests.sh --e2e`
  e2e output/count: exit 0; 82 scenarios (82 passed).
  ```

## Scenarios

### The model chooser explains how to choose

Domain constraints:

- The chooser states its keys and input: tier digits, typing, arrows, Enter,
  and Escape; the current model is marked; labels are plain language.

- [x] RED: Remove `@wip` from this scenario only. Bind the new steps with
  `unimplemented!()`. Run the exact targeted command; it must exit non-zero.
  ```text
  command: `./run-tests.sh --name 'The model chooser explains how to choose'`
  output: exit 1; 1 feature / 1 scenario (1 failed) / 4 steps (3 passed, 1 failed); the guidance stub panicked.
  ```
- [x] GREEN: Render the `Models` tier digits as keys with the active-model
  marker, the `Picks` and `Search` labels with the placeholder, and the new
  hint; migrate the chooser step assertions and the unit tests that pin
  `Matches`/`1-3`. Compile with `cargo check --locked`, then run the exact
  targeted command.
  Production files changed: `src/review/card.rs` (keyed tier digits with active-model marker, `Picks`/`Search` labels, placeholder, new hint), `src/review/panel.rs` (unit label), `tests/steps/interactive_shell_shortcut_steps.rs` (field/hint assertions and new bindings).
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'The model chooser explains how to choose'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 5 steps (5 passed).
  ```
- [x] REFACTOR: Keep one keyed-segment helper for all hints. Rerun the exact
  targeted command.
  ```text
  command: `./run-tests.sh --name 'The model chooser explains how to choose'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 5 steps (5 passed).
  ```
- [x] COMMIT: `f96b31aee5a84c40be44f6f10b0a86f1b44ddbe5` - feat(interactive-shell-shortcut): The model chooser explains how to choose

### The first suggestion is ready to choose

Domain constraints:

- A non-empty catalog highlights its first pick when the query is empty, so
  arrows and Enter act immediately; typed text stays authoritative.

- [x] RED: Remove `@wip` from this scenario only. Bind the new steps with
  `unimplemented!()`. Run the exact targeted command; it must exit non-zero.
  ```text
  command: `./run-tests.sh --name 'The first suggestion is ready to choose'`
  output: exit 1; 1 feature / 1 scenario (1 failed) / 5 steps (4 passed, 1 failed); the arrival stub panicked.
  ```
- [x] GREEN: Set the default highlight in `open_model_chooser` and
  `set_catalog` (empty query only), add the matching unit assertions, and bind
  arrival, highlight, and Enter selection through the session regeneration
  path. Compile and run the exact targeted command.
  Production files changed: `src/review/panel.rs` (default highlight in `open_model_chooser`/`set_catalog` plus unit assertions), `tests/steps/interactive_shell_shortcut_steps.rs` (arrival, highlight, and choice bindings).
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'The first suggestion is ready to choose'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 8 steps (8 passed).
  ```
- [x] REFACTOR: Keep the highlight precedence in one place. Rerun the exact
  targeted command.
  ```text
  command: `./run-tests.sh --name 'The first suggestion is ready to choose'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 8 steps (8 passed).
  ```
- [x] COMMIT: `832f659c8ded8e43db591cc48c3b480cc36488a0` - feat(interactive-shell-shortcut): The first suggestion is ready to choose

### The card marks nested shell syntax

Domain constraints:

- A stage Watn does not decompose is marked `nested syntax` with the same
  amber styling; the flow splitter and the raw stage text are unchanged.

- [x] RED: Remove `@wip` from this scenario only. Bind the new step with
  `unimplemented!()`. Run the exact targeted command; it must exit non-zero.
  ```text
  command: `./run-tests.sh --name 'The card marks nested shell syntax'`
  output: exit 1; 1 feature / 1 scenario (1 failed) / 4 steps (3 passed, 1 failed); the marker stub panicked.
  ```
- [x] GREEN: Rename the marker copy in both render paths; update the live
  "Unsupported command flow remains reviewable" step and the two unit tests to
  assert `nested syntax`; replace the old card-marker step. Compile and run the
  exact targeted command.
  Production files changed: `src/review/card.rs` (`NESTED_SYNTAX_MARKER`, both render paths; unit test), `src/review/panel.rs` (unit test), `tests/steps/interactive_shell_shortcut_steps.rs` (live marker assertion and the new card step), delta feature.
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'The card marks nested shell syntax'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 4 steps (4 passed); full regular 212/212, e2e 82/82, lib 78 passed.
  ```
- [x] REFACTOR: Keep the marker copy in one constant. Rerun the exact targeted
  command.
  ```text
  command: `./run-tests.sh --name 'The card marks nested shell syntax'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 4 steps (4 passed).
  ```
- [x] COMMIT: `61b00bb3cf3eb3e6ae49d612e5560eaee1bb572e` - feat(interactive-shell-shortcut): The card marks nested shell syntax

## Final verification

- [x] Run `givn lint --change clarify-review-card-guidance`
  ```text
  output: exit 0; `givn lint: 1 file(s) checked — clean`; two advisory subset notices for the default-selection scenario are dispositioned in the coverage review.
  ```
- [x] Run the full regular suite `./run-tests.sh`
  ```text
  output: exit 0; 21 features; 212 scenarios (212 passed); 1541 steps (1541 passed).
  ```
- [x] Run the full E2E suite `./run-tests.sh --e2e`
  ```text
  output: exit 0; 24 features; 82 scenarios (82 passed); 605 steps (605 passed).
  ```
- [x] Run `cargo test --locked --lib`
  ```text
  output: 78 passed; 0 failed.
  ```
- [x] Run `givn status --change clarify-review-card-guidance`
  ```text
  output: all tasks checked; artifacts proposal, specs, design, arc42-docs, design-review, tasks complete; next required artifact is `review`.
  ```
