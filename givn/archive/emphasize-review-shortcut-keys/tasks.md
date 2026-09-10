# Tasks: emphasize-review-shortcut-keys

All tasks are intentionally unchecked. One atomic commit for the scenario.
Use-case constraints: the card stays small and transient; decision wording and
key behavior never change; the mono card stays escape-free.

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

### The review card emphasizes the decision shortcut keys

Domain constraints:

- Decision keys are bold and colored; labels stay dim; accept keeps its green
  default emphasis; wording and key behavior are unchanged.

- [x] RED: Remove `@wip` from this scenario only. Bind the three emphasis steps
  with raw-SGR assertions that fail against the current dim renderer; use
  `unimplemented!()` for any step that cannot yet assert. Run the exact
  targeted command; it must exit non-zero.
  ```text
  command: `./run-tests.sh --name 'The review card emphasizes the decision shortcut keys'`
  output: exit 1; 1 feature / 1 scenario (1 failed) / 3 steps (2 passed, 1 failed); the first emphasis stub panicked.
  ```
- [x] GREEN: Add `Ink::key`/`Ink::accept_key`, emphasize the key tokens in the
  review, command-editor, and model-chooser hints, update
  `review_accept_is_default` to the bold-green sequence, and move the two card
  unit tests to raw-SGR checks. Compile with `cargo check --locked`, then run
  the exact targeted command.
  Production files changed: `src/review/card.rs` (`Ink::key`/`Ink::accept_key`, emphasized letters inside the review hint words, emphasized key tokens in the editor and chooser hints; two unit-test assertions moved to raw SGR), `tests/steps/interactive_shell_shortcut_steps.rs` (accept-default matcher, emphasis assertions), `tests/steps/interactive_shell_shortcut_e2e_steps.rs` (card wait label), delta feature (removed old-title wording scenario, added reworded one).
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'The review card emphasizes the decision shortcut keys'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 8 steps (8 passed); full regular 210/210, e2e 82/82, lib 77 passed.
  ```
- [x] REFACTOR: Keep one key-styling helper for all hint modes. Rerun the exact
  targeted command.
  ```text
  command: `./run-tests.sh --name 'The review card emphasizes the decision shortcut keys'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 8 steps (8 passed).
  ```
- [x] COMMIT: `b382fe10a35b903718d04c53d7a4858fa08f6fba` (renderer and assertions) + `057c9cc22b3dc60f9d7e914b3fcfa150676f8273` (permanent wording scenario rewrite and wait labels) - feat(interactive-shell-shortcut): The review card emphasizes the decision shortcut keys

## Final verification

- [x] Run `givn lint --change emphasize-review-shortcut-keys`
  ```text
  output: exit 0; `givn lint: 1 file(s) checked — clean`; one advisory subset notice is dispositioned in the coverage review.
  ```
- [x] Run the full regular suite `./run-tests.sh`
  ```text
  output: exit 0; 21 features; 210 scenarios (210 passed); 1527 steps (1527 passed).
  ```
- [x] Run the full E2E suite `./run-tests.sh --e2e`
  ```text
  output: exit 0; 24 features; 82 scenarios (82 passed); 605 steps (605 passed).
  ```
- [x] Run `cargo test --locked --lib`
  ```text
  output: 77 passed; 0 failed.
  ```
- [x] Run `givn status --change emphasize-review-shortcut-keys`
  ```text
  output: all tasks checked; artifacts proposal, specs, design, arc42-docs, design-review, tasks complete; next required artifact is `review`.
  ```
