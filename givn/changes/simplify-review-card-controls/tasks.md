# Tasks: simplify-review-card-controls

All tasks are intentionally unchecked. One atomic commit per scenario.
Use-case constraints apply to every scenario:

- Every candidate requires explicit final acceptance; cancellation and failure
  release nothing and preserve the original input.
- Rejection releases no candidate, keeps the active Intent, and opens the model
  chooser; the chooser regenerates; a failed regeneration keeps the previous
  candidate.
- Review display and buffer replacement never evaluate a candidate.
- The card stays small and transient; review text never reaches stdout.

## Setup

- [ ] Confirm the runner and strict mode for this change. Bind one temporary
  step for the first scenario with `unimplemented!()`, remove `@wip` from that
  scenario only, and run the targeted command; it must exit non-zero.
  ```text
  command: `./run-tests.sh --name 'The review card opens on the first command-flow stage'`
  exit: <paste non-zero exit status>
  output: <paste the failing stub output>
  ```

- [ ] Record the baseline before scenario implementation. Run `./run-tests.sh`
  and `./run-tests.sh --e2e` and record exit status and scenario counts.
  ```text
  regular command: `./run-tests.sh`
  regular output/count: <paste>
  e2e command: `./run-tests.sh --e2e`
  e2e output/count: <paste>
  ```

- [ ] Align the release-truth plain-`r` check with its scenario wording ("plain
  r for reasoning focus"): a line may mention `r` as the reject decision, but
  must not present `r` as the reasoning focus. Run the targeted release-truth
  scenario.
  ```text
  command: `./run-tests.sh --name 'Active documentation describes current command streaming'`
  output: <paste>
  ```

## Scenarios

### The review card opens on the first command-flow stage

Domain constraints:

- The card opens on the command flow; the first stage and its purpose are the
  visible subject; no focus region exists.

- [ ] RED: Remove `@wip` from this scenario only. Bind the new steps with
  `unimplemented!()`. Run the exact targeted command; it must exit non-zero.
  ```text
  command: `./run-tests.sh --name 'The review card opens on the first command-flow stage'`
  output: <paste non-zero runner output>
  ```
- [ ] GREEN: Remove `FocusRegion`, `PanelAction`, and the candidate list from
  the panel state; render one candidate with the first stage active and no
  focus strip; migrate every step binding that references the removed APIs,
  including the retained higher-tier and interrupt scenarios and the removed
  scenarios' bindings. Compile with `cargo check --locked`, then run the exact
  targeted command.
  Production files changed: `<paste paths>`.
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'The review card opens on the first command-flow stage'`
  output: <paste passing output>
  ```
- [ ] REFACTOR: Keep the state machine and renderer coherent (no stale focus
  helpers). Rerun the exact targeted command.
  ```text
  command: `./run-tests.sh --name 'The review card opens on the first command-flow stage'`
  output: <paste passing output>
  ```
- [ ] COMMIT: `<hash>` - feat(interactive-shell-shortcut): The review card opens on the first command-flow stage

### The review card exposes the direct decision shortcuts

Domain constraints:

- Decisions are direct keys; accept is the emphasized default.

- [ ] RED: Remove `@wip` from this scenario only. Bind the new steps with
  `unimplemented!()`. Run the exact targeted command; it must exit non-zero.
  ```text
  command: `./run-tests.sh --name 'The review card exposes the direct decision shortcuts'`
  output: <paste non-zero runner output>
  ```
- [ ] GREEN: Render the hint line with `a accept`, `e edit`, `r reject`,
  `c cancel`, `d disable`; paint the accept hint green in color mode; add the
  styled-substring assertion helper. Compile and run the exact targeted
  command.
  Production files changed: `<paste paths>`.
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'The review card exposes the direct decision shortcuts'`
  output: <paste passing output>
  ```
- [ ] REFACTOR: Keep the hint assembly in one place. Rerun the exact targeted
  command.
  ```text
  command: `./run-tests.sh --name 'The review card exposes the direct decision shortcuts'`
  output: <paste passing output>
  ```
- [ ] COMMIT: `<hash>` - feat(interactive-shell-shortcut): The review card exposes the direct decision shortcuts

### Enter accepts the current candidate

Domain constraints:

- Enter is final acceptance and releases only the current candidate.

- [ ] RED: Remove `@wip` from this scenario only. Bind the new steps with
  `unimplemented!()`. Run the exact targeted command; it must exit non-zero.
  ```text
  command: `./run-tests.sh --name 'Enter accepts the current candidate'`
  output: <paste non-zero runner output>
  ```
- [ ] GREEN: Route Enter in review mode to acceptance with no regeneration.
  Compile and run the exact targeted command.
  Production files changed: `<paste paths>`.
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'Enter accepts the current candidate'`
  output: <paste passing output>
  ```
- [ ] REFACTOR: Keep one acceptance path for Enter and the shortcut. Rerun the
  exact targeted command.
  ```text
  command: `./run-tests.sh --name 'Enter accepts the current candidate'`
  output: <paste passing output>
  ```
- [ ] COMMIT: `<hash>` - feat(interactive-shell-shortcut): Enter accepts the current candidate

### The accept shortcut accepts the current candidate

Domain constraints:

- `a` is equivalent to Enter and releases only the current candidate.

- [ ] RED: Remove `@wip` from this scenario only. Bind the new steps with
  `unimplemented!()`. Run the exact targeted command; it must exit non-zero.
  ```text
  command: `./run-tests.sh --name 'The accept shortcut accepts the current candidate'`
  output: <paste non-zero runner output>
  ```
- [ ] GREEN: Handle `a`/`A` in review mode. Compile and run the exact targeted
  command.
  Production files changed: `<paste paths>`.
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'The accept shortcut accepts the current candidate'`
  output: <paste passing output>
  ```
- [ ] REFACTOR: Share the acceptance path with Enter. Rerun the exact targeted
  command.
  ```text
  command: `./run-tests.sh --name 'The accept shortcut accepts the current candidate'`
  output: <paste passing output>
  ```
- [ ] COMMIT: `<hash>` - feat(interactive-shell-shortcut): The accept shortcut accepts the current candidate

### The cancel shortcut cancels the review

Domain constraints:

- `c` cancels like Escape and releases nothing.

- [ ] RED: Remove `@wip` from this scenario only. Bind the new steps with
  `unimplemented!()`. Run the exact targeted command; it must exit non-zero.
  ```text
  command: `./run-tests.sh --name 'The cancel shortcut cancels the review'`
  output: <paste non-zero runner output>
  ```
- [ ] GREEN: Handle `c`/`C` in review mode. Compile and run the exact targeted
  command.
  Production files changed: `<paste paths>`.
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'The cancel shortcut cancels the review'`
  output: <paste passing output>
  ```
- [ ] REFACTOR: Keep one cancellation path. Rerun the exact targeted command.
  ```text
  command: `./run-tests.sh --name 'The cancel shortcut cancels the review'`
  output: <paste passing output>
  ```
- [ ] COMMIT: `<hash>` - feat(interactive-shell-shortcut): The cancel shortcut cancels the review

### Rejecting a candidate opens the model chooser

Domain constraints:

- Rejection releases nothing, keeps the Intent, and opens the chooser with the
  configured tiers and a model field.

- [ ] RED: Remove `@wip` from this scenario only. Bind the new steps with
  `unimplemented!()`. Run the exact targeted command; it must exit non-zero.
  ```text
  command: `./run-tests.sh --name 'Rejecting a candidate opens the model chooser'`
  output: <paste non-zero runner output>
  ```
- [ ] GREEN: Add `RejectRequested`, `ModelChooser`, `TierChoice`, the chooser
  render rows, and the reject key; the step binds the chooser from harness
  tiers. Compile and run the exact targeted command.
  Production files changed: `<paste paths>`.
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'Rejecting a candidate opens the model chooser'`
  output: <paste passing output>
  ```
- [ ] REFACTOR: Keep tier ordering in `chooser_tiers`. Rerun the exact targeted
  command.
  ```text
  command: `./run-tests.sh --name 'Rejecting a candidate opens the model chooser'`
  output: <paste passing output>
  ```
- [ ] COMMIT: `<hash>` - feat(interactive-shell-shortcut): Rejecting a candidate opens the model chooser

### A configured tier can be chosen with its number

Domain constraints:

- Choosing a tier regenerates a fresh candidate at that tier and still requires
  acceptance.

- [ ] RED: Remove `@wip` from this scenario only. Bind the new steps with
  `unimplemented!()`. Run the exact targeted command; it must exit non-zero.
  ```text
  command: `./run-tests.sh --name 'A configured tier can be chosen with its number'`
  output: <paste non-zero runner output>
  ```
- [ ] GREEN: Add `RegenerateWith`, number-key handling, and the library
  `session::generate_candidate`/`parse_generated_candidate`; the scenario drives
  a real in-process provider against the world twin. Compile and run the exact
  targeted command.
  Production files changed: `<paste paths>`.
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'A configured tier can be chosen with its number'`
  output: <paste passing output>
  ```
- [ ] REFACTOR: Keep one regeneration path for tiers and models. Rerun the
  exact targeted command.
  ```text
  command: `./run-tests.sh --name 'A configured tier can be chosen with its number'`
  output: <paste passing output>
  ```
- [ ] COMMIT: `<hash>` - feat(interactive-shell-shortcut): A configured tier can be chosen with its number

### A model name is suggested from the provider catalog while typing

Domain constraints:

- Typing filters catalog suggestions; choosing a suggestion regenerates with
  that model.

- [ ] RED: Remove `@wip` from this scenario only. Bind the new steps with
  `unimplemented!()`. Run the exact targeted command; it must exit non-zero.
  ```text
  command: `./run-tests.sh --name 'A model name is suggested from the provider catalog while typing'`
  output: <paste non-zero runner output>
  ```
- [ ] GREEN: Add the chooser query, filtering, highlight, and Enter selection;
  `session::fetch_catalog` maps catalog entries. Compile and run the exact
  targeted command.
  Production files changed: `<paste paths>`.
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'A model name is suggested from the provider catalog while typing'`
  output: <paste passing output>
  ```
- [ ] REFACTOR: Keep filtering and highlighting pure. Rerun the exact targeted
  command.
  ```text
  command: `./run-tests.sh --name 'A model name is suggested from the provider catalog while typing'`
  output: <paste passing output>
  ```
- [ ] COMMIT: `<hash>` - feat(interactive-shell-shortcut): A model name is suggested from the provider catalog while typing

### A typed model name works when the catalog is unavailable

Domain constraints:

- A typed model name is usable as entered even with no suggestions.

- [ ] RED: Remove `@wip` from this scenario only. Bind the new steps with
  `unimplemented!()`. Run the exact targeted command; it must exit non-zero.
  ```text
  command: `./run-tests.sh --name 'A typed model name works when the catalog is unavailable'`
  output: <paste non-zero runner output>
  ```
- [ ] GREEN: Enter with a non-empty query and no highlight regenerates with the
  typed model; the fetch failure path yields an empty catalog. Compile and run
  the exact targeted command.
  Production files changed: `<paste paths>`.
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'A typed model name works when the catalog is unavailable'`
  output: <paste passing output>
  ```
- [ ] REFACTOR: Keep the choose-or-type branch explicit. Rerun the exact
  targeted command.
  ```text
  command: `./run-tests.sh --name 'A typed model name works when the catalog is unavailable'`
  output: <paste passing output>
  ```
- [ ] COMMIT: `<hash>` - feat(interactive-shell-shortcut): A typed model name works when the catalog is unavailable

### The model chooser ignores incomplete choices

Domain constraints:

- Invalid tier numbers and Enter with no choice leave the chooser and candidate
  unchanged.

- [ ] RED: Remove `@wip` from this scenario only. Bind the new steps with
  `unimplemented!()`. Run the exact targeted command; it must exit non-zero.
  ```text
  command: `./run-tests.sh --name 'The model chooser ignores incomplete choices'`
  output: <paste non-zero runner output>
  ```
- [ ] GREEN: Ignore out-of-range numbers and an empty Enter. Compile and run the
  exact targeted command.
  Production files changed: `<paste paths>`.
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'The model chooser ignores incomplete choices'`
  output: <paste passing output>
  ```
- [ ] REFACTOR: Keep the chooser branch table exhaustive. Rerun the exact
  targeted command.
  ```text
  command: `./run-tests.sh --name 'The model chooser ignores incomplete choices'`
  output: <paste passing output>
  ```
- [ ] COMMIT: `<hash>` - feat(interactive-shell-shortcut): The model chooser ignores incomplete choices

### A failed regeneration preserves the previous candidate

Domain constraints:

- On generation failure the previous candidate and review state stay; the card
  reports the failure; acceptance is still required.

- [ ] RED: Remove `@wip` from this scenario only. Bind the new steps with
  `unimplemented!()`. Run the exact targeted command; it must exit non-zero.
  ```text
  command: `./run-tests.sh --name 'A failed regeneration preserves the previous candidate'`
  output: <paste non-zero runner output>
  ```
- [ ] GREEN: Add `regeneration_error` rendering and the failure branch that
  keeps the candidate. Compile and run the exact targeted command.
  Production files changed: `<paste paths>`.
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'A failed regeneration preserves the previous candidate'`
  output: <paste passing output>
  ```
- [ ] REFACTOR: Keep error rendering bounded like other rows. Rerun the exact
  targeted command.
  ```text
  command: `./run-tests.sh --name 'A failed regeneration preserves the previous candidate'`
  output: <paste passing output>
  ```
- [ ] COMMIT: `<hash>` - feat(interactive-shell-shortcut): A failed regeneration preserves the previous candidate

### Leaving the model chooser preserves the candidate

Domain constraints:

- Escape closes the chooser without changing the candidate or the review.

- [ ] RED: Remove `@wip` from this scenario only. Bind the new steps with
  `unimplemented!()`. Run the exact targeted command; it must exit non-zero.
  ```text
  command: `./run-tests.sh --name 'Leaving the model chooser preserves the candidate'`
  output: <paste non-zero runner output>
  ```
- [ ] GREEN: Escape returns from `ModelChooser` to review unchanged. Compile and
  run the exact targeted command.
  Production files changed: `<paste paths>`.
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'Leaving the model chooser preserves the candidate'`
  output: <paste passing output>
  ```
- [ ] REFACTOR: Keep the chooser close path free of candidate mutation. Rerun
  the exact targeted command.
  ```text
  command: `./run-tests.sh --name 'Leaving the model chooser preserves the candidate'`
  output: <paste passing output>
  ```
- [ ] COMMIT: `<hash>` - feat(interactive-shell-shortcut): Leaving the model chooser preserves the candidate

### The command editor moves the insertion point with arrows and Home and End

Domain constraints:

- The editor keeps the original intent; navigation clamps at the boundaries.

- [ ] RED: Remove `@wip` from this scenario only. Bind the new steps with
  `unimplemented!()`. Run the exact targeted command; it must exit non-zero.
  ```text
  command: `./run-tests.sh --name 'The command editor moves the insertion point with arrows and Home and End'`
  output: <paste non-zero runner output>
  ```
- [ ] GREEN: Add `editor_cursor` and the move/insert helpers; render the caret
  at the insertion point. Compile and run the exact targeted command.
  Production files changed: `<paste paths>`.
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'The command editor moves the insertion point with arrows and Home and End'`
  output: <paste passing output>
  ```
- [ ] REFACTOR: Keep one char-indexed editing helper set. Rerun the exact
  targeted command.
  ```text
  command: `./run-tests.sh --name 'The command editor moves the insertion point with arrows and Home and End'`
  output: <paste passing output>
  ```
- [ ] COMMIT: `<hash>` - feat(interactive-shell-shortcut): The command editor moves the insertion point with arrows and Home and End

### Backspace and Delete remove text at the insertion point

Domain constraints:

- Backspace and Delete act at the insertion point and never evaluate.

- [ ] RED: Remove `@wip` from this scenario only. Bind the new steps with
  `unimplemented!()`. Run the exact targeted command; it must exit non-zero.
  ```text
  command: `./run-tests.sh --name 'Backspace and Delete remove text at the insertion point'`
  output: <paste non-zero runner output>
  ```
- [ ] GREEN: Route Backspace and Delete through the cursor helpers. Compile and
  run the exact targeted command.
  Production files changed: `<paste paths>`.
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'Backspace and Delete remove text at the insertion point'`
  output: <paste passing output>
  ```
- [ ] REFACTOR: Keep boundary behavior uniform. Rerun the exact targeted
  command.
  ```text
  command: `./run-tests.sh --name 'Backspace and Delete remove text at the insertion point'`
  output: <paste passing output>
  ```
- [ ] COMMIT: `<hash>` - feat(interactive-shell-shortcut): Backspace and Delete remove text at the insertion point

### The explanation card ignores review decisions

Domain constraints:

- In the explanation-only card only closing is offered; `a`/`e`/`r`/`d` are
  ignored and nothing executes.

- [ ] RED: Remove `@wip` from this scenario only. Bind the new steps with
  `unimplemented!()`. Run the exact targeted command; it must exit non-zero.
  ```text
  command: `./run-tests.sh --name 'The explanation card ignores review decisions'`
  output: <paste non-zero runner output>
  ```
- [ ] GREEN: Gate `a`/`A`, `e`/`E`, `r`/`R`, `d`/`D` behind the explanation
  mode. Compile and run the exact targeted command.
  Production files changed: `<paste paths>`.
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'The explanation card ignores review decisions'`
  output: <paste passing output>
  ```
- [ ] REFACTOR: Keep the mode gate in one place. Rerun the exact targeted
  command.
  ```text
  command: `./run-tests.sh --name 'The explanation card ignores review decisions'`
  output: <paste passing output>
  ```
- [ ] COMMIT: `<hash>` - feat(interactive-shell-shortcut): The explanation card ignores review decisions

## E2E setup

- [ ] Add `pending_tiers` and `pending_mock_replacements` to `WatnWorld`;
  register replacement mocks before the generic chat mock in every
  `ensure_test_env` registration block; confirm the local stack starts with the
  in-process provider twin. Record the verify command counts: `./run-tests.sh`
  full count and `./run-tests.sh --e2e` count, with the e2e count strictly
  smaller.
  ```text
  regular count: <paste>
  e2e count: <paste>
  ```

## E2E scenarios

### Developer rejects a candidate and regenerates with another model

Domain constraints:

- The reject action opens the chooser, the chosen tier's model regenerates, the
  replacement candidate is accepted, and only the replacement reaches the
  command-output channel.

- [ ] RED: Remove `@wip` from this scenario only. Bind the E2E steps with
  `unimplemented!()` or drive the real PTY until the missing assertion fails.
  Run the exact targeted e2e command; it must exit non-zero.
  ```text
  command: `./run-tests.sh --e2e --name 'Developer rejects a candidate and regenerates with another model'`
  output: <paste non-zero runner output>
  ```
- [ ] GREEN: Configure distinct tier models, register the replacement mock for
  the normal tier, drive `r`, `2`, wait for the replacement candidate, accept,
  and assert the output channel. List files touched. Run the exact targeted
  e2e command; it must exit zero.
  ```text
  command: `./run-tests.sh --e2e --name 'Developer rejects a candidate and regenerates with another model'`
  output: <paste passing output>
  ```
- [ ] REFACTOR: Keep PTY label waits displacement-safe. Rerun the exact
  targeted e2e command.
  ```text
  command: `./run-tests.sh --e2e --name 'Developer rejects a candidate and regenerates with another model'`
  output: <paste passing output>
  ```
- [ ] COMMIT: `<hash>` - test(e2e): Developer rejects a candidate and regenerates with another model

## Final verification

- [ ] Run `givn lint --change simplify-review-card-controls`
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
- [ ] Run `cargo check --locked`
  ```text
  output: <paste>
  ```
- [ ] Run `givn status --change simplify-review-card-controls`
  ```text
  output: <paste>
  ```
