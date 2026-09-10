# Tasks: compact-stage-marker

All tasks are intentionally unchecked. One atomic commit for the scenario.
Use-case constraints: the stage text and purpose never change; the marker is
advisory; the card stays small; mono cards stay escape-free.

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

### The card marks a stage it cannot decompose

Domain constraints:

- A stage Watn does not decompose carries one amber `…` in the note's
  position; no worded note remains; the stage and purpose text are unchanged.

- [ ] RED: Remove `@wip` from this scenario only. Bind the new step with
  `unimplemented!()`. Run the exact targeted command; it must exit non-zero.
  ```text
  command: `./run-tests.sh --name 'The card marks a stage it cannot decompose'`
  output: <paste non-zero runner output>
  ```
- [ ] GREEN: Replace both worded marker render paths with the amber ellipsis,
  migrate the live marker step and the replaced card step to raw-sequence
  assertions without the old words, and update the card/panel unit tests,
  including the mono and own-row fallbacks. Compile with
  `cargo check --locked`, then run the exact targeted command.
  Production files changed: `<paste paths>`.
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'The card marks a stage it cannot decompose'`
  output: <paste passing output>
  ```
- [ ] REFACTOR: Keep the marker in one constant. Rerun the exact targeted
  command.
  ```text
  command: `./run-tests.sh --name 'The card marks a stage it cannot decompose'`
  output: <paste passing output>
  ```
- [ ] COMMIT: `<hash>` - feat(interactive-shell-shortcut): The card marks a stage it cannot decompose

## Final verification

- [ ] Run `givn lint --change compact-stage-marker`
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
- [ ] Run `givn status --change compact-stage-marker`
  ```text
  output: <paste>
  ```
