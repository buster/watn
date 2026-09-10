# Tasks: clarify-review-card-guidance

All tasks are intentionally unchecked. One atomic commit per scenario.
Use-case constraints: the card stays small and transient; key behavior,
regeneration, and the flow splitter never change; mono cards stay
escape-free.

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

### The model chooser explains how to choose

Domain constraints:

- The chooser states its keys and input: tier digits, typing, arrows, Enter,
  and Escape; the current model is marked; labels are plain language.

- [ ] RED: Remove `@wip` from this scenario only. Bind the new steps with
  `unimplemented!()`. Run the exact targeted command; it must exit non-zero.
  ```text
  command: `./run-tests.sh --name 'The model chooser explains how to choose'`
  output: <paste non-zero runner output>
  ```
- [ ] GREEN: Render the `Models` tier digits as keys with the active-model
  marker, the `Picks` and `Search` labels with the placeholder, and the new
  hint; migrate the chooser step assertions and the unit tests that pin
  `Matches`/`1-3`. Compile with `cargo check --locked`, then run the exact
  targeted command.
  Production files changed: `<paste paths>`.
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'The model chooser explains how to choose'`
  output: <paste passing output>
  ```
- [ ] REFACTOR: Keep one keyed-segment helper for all hints. Rerun the exact
  targeted command.
  ```text
  command: `./run-tests.sh --name 'The model chooser explains how to choose'`
  output: <paste passing output>
  ```
- [ ] COMMIT: `<hash>` - feat(interactive-shell-shortcut): The model chooser explains how to choose

### The first suggestion is ready to choose

Domain constraints:

- A non-empty catalog highlights its first pick when the query is empty, so
  arrows and Enter act immediately; typed text stays authoritative.

- [ ] RED: Remove `@wip` from this scenario only. Bind the new steps with
  `unimplemented!()`. Run the exact targeted command; it must exit non-zero.
  ```text
  command: `./run-tests.sh --name 'The first suggestion is ready to choose'`
  output: <paste non-zero runner output>
  ```
- [ ] GREEN: Set the default highlight in `open_model_chooser` and
  `set_catalog` (empty query only), add the matching unit assertions, and bind
  arrival, highlight, and Enter selection through the session regeneration
  path. Compile and run the exact targeted command.
  Production files changed: `<paste paths>`.
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'The first suggestion is ready to choose'`
  output: <paste passing output>
  ```
- [ ] REFACTOR: Keep the highlight precedence in one place. Rerun the exact
  targeted command.
  ```text
  command: `./run-tests.sh --name 'The first suggestion is ready to choose'`
  output: <paste passing output>
  ```
- [ ] COMMIT: `<hash>` - feat(interactive-shell-shortcut): The first suggestion is ready to choose

### The card marks nested shell syntax

Domain constraints:

- A stage Watn does not decompose is marked `nested syntax` with the same
  amber styling; the flow splitter and the raw stage text are unchanged.

- [ ] RED: Remove `@wip` from this scenario only. Bind the new step with
  `unimplemented!()`. Run the exact targeted command; it must exit non-zero.
  ```text
  command: `./run-tests.sh --name 'The card marks nested shell syntax'`
  output: <paste non-zero runner output>
  ```
- [ ] GREEN: Rename the marker copy in both render paths; update the live
  "Unsupported command flow remains reviewable" step and the two unit tests to
  assert `nested syntax`; replace the old card-marker step. Compile and run the
  exact targeted command.
  Production files changed: `<paste paths>`.
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'The card marks nested shell syntax'`
  output: <paste passing output>
  ```
- [ ] REFACTOR: Keep the marker copy in one constant. Rerun the exact targeted
  command.
  ```text
  command: `./run-tests.sh --name 'The card marks nested shell syntax'`
  output: <paste passing output>
  ```
- [ ] COMMIT: `<hash>` - feat(interactive-shell-shortcut): The card marks nested shell syntax

## Final verification

- [ ] Run `givn lint --change clarify-review-card-guidance`
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
- [ ] Run `givn status --change clarify-review-card-guidance`
  ```text
  output: <paste>
  ```
