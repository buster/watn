# Tasks: emphasize-review-shortcut-keys

All tasks are intentionally unchecked. One atomic commit for the scenario.
Use-case constraints: the card stays small and transient; decision wording and
key behavior never change; the mono card stays escape-free.

## Setup

- [ ] Record the baseline before scenario implementation. Run `./run-tests.sh`
  and `./run-tests.sh --e2e` and record exit status and scenario counts.
  ```text
  regular command: `./run-tests.sh`
  regular output/count: <paste>
  e2e command: `./run-tests.sh --e2e`
  e2e output/count: <paste>
  ```

## Scenarios

### The review card emphasizes the decision shortcut keys

Domain constraints:

- Decision keys are bold and colored; labels stay dim; accept keeps its green
  default emphasis; wording and key behavior are unchanged.

- [ ] RED: Remove `@wip` from this scenario only. Bind the three emphasis steps
  with raw-SGR assertions that fail against the current dim renderer; use
  `unimplemented!()` for any step that cannot yet assert. Run the exact
  targeted command; it must exit non-zero.
  ```text
  command: `./run-tests.sh --name 'The review card emphasizes the decision shortcut keys'`
  output: <paste non-zero runner output>
  ```
- [ ] GREEN: Add `Ink::key`/`Ink::accept_key`, emphasize the key tokens in the
  review, command-editor, and model-chooser hints, update
  `review_accept_is_default` to the bold-green sequence, and move the two card
  unit tests to raw-SGR checks. Compile with `cargo check --locked`, then run
  the exact targeted command.
  Production files changed: `<paste paths>`.
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'The review card emphasizes the decision shortcut keys'`
  output: <paste passing output>
  ```
- [ ] REFACTOR: Keep one key-styling helper for all hint modes. Rerun the exact
  targeted command.
  ```text
  command: `./run-tests.sh --name 'The review card emphasizes the decision shortcut keys'`
  output: <paste passing output>
  ```
- [ ] COMMIT: `<hash>` - feat(interactive-shell-shortcut): The review card emphasizes the decision shortcut keys

## Final verification

- [ ] Run `givn lint --change emphasize-review-shortcut-keys`
  ```text
  output: <paste>
  ```
- [ ] Run the full regular suite `./run-tests.sh`
  ```text
  output: <paste>
  ```
- [ ] Run the full E2E suite `./run-tests.sh --e2e`
  ```text
  output: <paste>
  ```
- [ ] Run `cargo test --locked --lib`
  ```text
  output: <paste>
  ```
- [ ] Run `givn status --change emphasize-review-shortcut-keys`
  ```text
  output: <paste>
  ```
