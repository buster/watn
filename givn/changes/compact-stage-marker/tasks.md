# Tasks: compact-stage-marker

All tasks are intentionally unchecked. One atomic commit for the scenario.
Use-case constraints: the stage text and purpose never change; the marker is
advisory; the card stays small; mono cards stay escape-free.

## Setup

- [x] Record the baseline before scenario implementation. Run `./run-tests.sh`
  and `./run-tests.sh --e2e` and record exit status and scenario counts.
  ```text
  regular command: `./run-tests.sh`
  regular output/count: exit 0; 211 scenarios (211 passed).
  e2e command: `./run-tests.sh --e2e`
  e2e output/count: exit 0; 82 scenarios (82 passed).
  ```

## Scenarios

### The card marks a stage it cannot decompose

Domain constraints:

- A stage Watn does not decompose carries one amber `…` in the note's
  position; no worded note remains; the stage and purpose text are unchanged.

- [x] RED: Remove `@wip` from this scenario only. Bind the new step with
  `unimplemented!()`. Run the exact targeted command; it must exit non-zero.
  ```text
  command: `./run-tests.sh --name 'The card marks a stage it cannot decompose'`
  output: exit 1; 1 feature / 1 scenario (1 failed) / 4 steps (3 passed, 1 failed); the marker stub panicked.
  ```
- [x] GREEN: Replace both worded marker render paths with the amber ellipsis,
  migrate the live marker step and the replaced card step to raw-sequence
  assertions without the old words, and update the card/panel unit tests,
  including the mono and own-row fallbacks. Compile with
  `cargo check --locked`, then run the exact targeted command.
  Production files changed: `src/review/card.rs` (`UNDECOMPOSED_STAGE_MARKER`, both render paths, mono and own-row unit cases), `src/review/panel.rs` (narrow unit test), `tests/steps/interactive_shell_shortcut_steps.rs` (live marker step and the new card step).
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'The card marks a stage it cannot decompose'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 4 steps (4 passed); full regular 212/212, e2e 82/82, lib 79 passed.
  ```
- [x] REFACTOR: Keep the marker in one constant. Rerun the exact targeted
  command.
  ```text
  command: `./run-tests.sh --name 'The card marks a stage it cannot decompose'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 4 steps (4 passed).
  ```
- [x] COMMIT: `005e95532104309e3757437644e4463d7759a592` - feat(interactive-shell-shortcut): The card marks a stage it cannot decompose

## Final verification

- [x] Run `givn lint --change compact-stage-marker`
  ```text
  output: exit 0; `givn lint: 1 file(s) checked — clean`.
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
  output: 79 passed; 0 failed.
  ```
- [x] Run `givn status --change compact-stage-marker`
  ```text
  output: all tasks checked; artifacts proposal, specs, design, arc42-docs, design-review, tasks complete; next required artifact is `review`.
  ```
