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

- [x] Confirm the runner and strict mode for this change. Bind one temporary
  step for the first scenario with `unimplemented!()`, remove `@wip` from that
  scenario only, and run the targeted command; it must exit non-zero.
  ```text
  command: `./run-tests.sh --name 'The review card opens on the first command-flow stage'`
  exit: 1
  output: 1 feature / 1 scenario (1 failed) / 4 steps (3 passed, 1 failed); the first Then stub panicked with `not implemented`.
  ```

- [x] Record the baseline before scenario implementation. Run `./run-tests.sh`
  and `./run-tests.sh --e2e` and record exit status and scenario counts.
  ```text
  regular command: `./run-tests.sh`
  regular output/count: exit 0; 194 scenarios (194 passed); 1157 steps (1157 passed). Six permanent scenarios are filtered because this delta declares them removed.
  e2e command: `./run-tests.sh --e2e`
  e2e output/count: exit 0; 81 scenarios (81 passed); 592 steps (592 passed).
  ```

- [x] Align the release-truth plain-`r` check with its scenario wording ("plain
  r for reasoning focus"): a line may mention `r` as the reject decision, but
  must not present `r` as the reasoning focus. Run the targeted release-truth
  scenario.
  ```text
  command: `./run-tests.sh --name 'Active documentation describes current command streaming'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 11 steps (11 passed). The check now rejects only lines that present plain `r` as the reasoning focus.
  ```

## Scenarios

### The review card opens on the first command-flow stage

Domain constraints:

- The card opens on the command flow; the first stage and its purpose are the
  visible subject; no focus region exists.

- [x] RED: Remove `@wip` from this scenario only. Bind the new steps with
  `unimplemented!()`. Run the exact targeted command; it must exit non-zero.
  ```text
  command: `./run-tests.sh --name 'The review card opens on the first command-flow stage'`
  output: exit 1; 1 feature / 1 scenario (1 failed) / 4 steps (3 passed, 1 failed); the stub panicked.
  ```
- [x] GREEN: Remove `FocusRegion`, `PanelAction`, and the candidate list from
  the panel state; render one candidate with the first stage active and no
  focus strip; migrate every step binding that references the removed APIs,
  including the retained higher-tier and interrupt scenarios and the removed
  scenarios' bindings. Compile with `cargo check --locked`, then run the exact
  targeted command.
  Production files changed: `src/review/panel.rs` (single-candidate state, focus regions and action cursor removed, arrows move stages, `e`/`d`/Enter decisions), `src/review/card.rs` (header, no focus row or action row, hints), `src/review/mod.rs` (exports), `tests/steps/interactive_shell_shortcut_steps.rs` (bindings and removed-API migration), `tests/steps/release_truth_steps.rs` (plain-`r` check aligned with its scenario wording), `tests/steps/interactive_shell_shortcut_e2e_steps.rs` (card wait labels).
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'The review card opens on the first command-flow stage'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 6 steps (6 passed); full suites also green: regular 195/195, e2e 81/81, lib 71 passed.
  ```
- [x] REFACTOR: Keep the state machine and renderer coherent (no stale focus
  helpers). Rerun the exact targeted command.
  ```text
  command: `./run-tests.sh --name 'The review card opens on the first command-flow stage'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 6 steps (6 passed).
  ```
- [x] COMMIT: `7ca2fcf1e8adab3da5d7148c6dc564e265d16ffd` - feat(interactive-shell-shortcut): The review card opens on the first command-flow stage

### The review card exposes the direct decision shortcuts

Domain constraints:

- Decisions are direct keys; accept is the emphasized default.

- [x] RED: Remove `@wip` from this scenario only. Bind the new steps with
  `unimplemented!()`. Run the exact targeted command; it must exit non-zero.
  ```text
  command: `./run-tests.sh --name 'The review card exposes the direct decision shortcuts'`
  output: exit 1; 1 feature / 1 scenario (1 failed) / 3 steps (2 passed, 1 failed); the first Then stub panicked.
  ```
- [x] GREEN: Render the hint line with `a accept`, `e edit`, `r reject`,
  `c cancel`, `d disable`; paint the accept hint green in color mode; add the
  styled-substring assertion helper. Compile and run the exact targeted
  command.
  Production files changed: `src/review/card.rs` (decision hint row with `⏎/a accept` emphasized green), `tests/steps/interactive_shell_shortcut_steps.rs` (generic show binding and emphasis assertion), `tests/steps/interactive_shell_shortcut_e2e_steps.rs` (card wait label).
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'The review card exposes the direct decision shortcuts'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 7 steps (7 passed); full suites green: regular 196/196, e2e 81/81, lib 71 passed.
  ```
- [x] REFACTOR: Keep the hint assembly in one place. Rerun the exact targeted
  command.
  ```text
  command: `./run-tests.sh --name 'The review card exposes the direct decision shortcuts'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 7 steps (7 passed).
  ```
- [x] COMMIT: `5d90d2ed86736731c3510c8d0841a93cacf38b35` - feat(interactive-shell-shortcut): The review card exposes the direct decision shortcuts

### Enter accepts the current candidate

Domain constraints:

- Enter is final acceptance and releases only the current candidate.

- [x] RED: Remove `@wip` from this scenario only. Bind the new steps with
  `unimplemented!()`. Run the exact targeted command; it must exit non-zero.
  ```text
  command: `./run-tests.sh --name 'Enter accepts the current candidate'`
  output: exit 1; 1 feature / 1 scenario (1 failed) / 3 steps (2 passed, 1 failed); the When stub panicked.
  ```
- [x] GREEN: Route Enter in review mode to acceptance with no regeneration.
  Compile and run the exact targeted command.
  Production files changed: `tests/steps/interactive_shell_shortcut_steps.rs` only (shared `drive_review_key` helper); Enter acceptance is the production path introduced by the first scenario.
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'Enter accepts the current candidate'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 5 steps (5 passed); full regular 197/197, e2e 81/81, lib 71 passed.
  ```
- [x] REFACTOR: Keep one acceptance path for Enter and the shortcut. Rerun the
  exact targeted command.
  ```text
  command: `./run-tests.sh --name 'Enter accepts the current candidate'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 5 steps (5 passed).
  ```
- [x] COMMIT: `8f4a93e61d8d9653e2719a7b45f5f917f66be15b` - feat(interactive-shell-shortcut): Enter accepts the current candidate

### The accept shortcut accepts the current candidate

Domain constraints:

- `a` is equivalent to Enter and releases only the current candidate.

- [x] RED: Remove `@wip` from this scenario only. Bind the new steps with
  `unimplemented!()`. Run the exact targeted command; it must exit non-zero.
  ```text
  command: `./run-tests.sh --name 'The accept shortcut accepts the current candidate'`
  output: exit 1; 1 feature / 1 scenario (1 failed) / 3 steps (2 passed, 1 failed); the shortcut stub panicked.
  ```
- [x] GREEN: Handle `a`/`A` in review mode. Compile and run the exact targeted
  command.
  Production files changed: `src/review/panel.rs` (`a`/`A` routes to acceptance), `tests/steps/interactive_shell_shortcut_steps.rs` (shortcut binding).
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'The accept shortcut accepts the current candidate'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 5 steps (5 passed); full regular 198/198, lib 71 passed.
  ```
- [x] REFACTOR: Share the acceptance path with Enter. Rerun the exact targeted
  command.
  ```text
  command: `./run-tests.sh --name 'The accept shortcut accepts the current candidate'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 5 steps (5 passed).
  ```
- [x] COMMIT: `a75973b3a9f3d17143aa02d06997bc0e85dffa34` - feat(interactive-shell-shortcut): The accept shortcut accepts the current candidate

### The cancel shortcut cancels the review

Domain constraints:

- `c` cancels like Escape and releases nothing.

- [x] RED: Remove `@wip` from this scenario only. Bind the new steps with
  `unimplemented!()`. Run the exact targeted command; it must exit non-zero.
  ```text
  command: `./run-tests.sh --name 'The cancel shortcut cancels the review'`
  output: exit 1; 1 feature / 1 scenario (1 failed) / 3 steps (2 passed, 1 failed); the shortcut stub panicked.
  ```
- [x] GREEN: Handle `c`/`C` in review mode. Compile and run the exact targeted
  command.
  Production files changed: `src/review/panel.rs` (`c`/`C` routes to cancellation), `tests/steps/interactive_shell_shortcut_steps.rs` (shortcut binding).
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'The cancel shortcut cancels the review'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 5 steps (5 passed); full regular 199/199, e2e 81/81.
  ```
- [x] REFACTOR: Keep one cancellation path. Rerun the exact targeted command.
  ```text
  command: `./run-tests.sh --name 'The cancel shortcut cancels the review'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 5 steps (5 passed).
  ```
- [x] COMMIT: `b486614138a0f76f65ad89eec87dcf318840fb73` - feat(interactive-shell-shortcut): The cancel shortcut cancels the review

### Rejecting a candidate opens the model chooser

Domain constraints:

- Rejection releases nothing, keeps the Intent, and opens the chooser with the
  configured tiers and a model field.

- [x] RED: Remove `@wip` from this scenario only. Bind the new steps with
  `unimplemented!()`. Run the exact targeted command; it must exit non-zero.
  ```text
  command: `./run-tests.sh --name 'Rejecting a candidate opens the model chooser'`
  output: exit 1; 1 feature / 1 scenario (1 failed) / 3 steps (2 passed, 1 failed); the reject-shortcut stub panicked.
  ```
- [x] GREEN: Add `RejectRequested`, `ModelChooser`, `TierChoice`, the chooser
  render rows, and the reject key; the step binds the chooser from harness
  tiers. Compile and run the exact targeted command.
  Production files changed: `src/review/panel.rs` (`ModelChooser`, `TierChoice`, `RejectRequested`, `RegenerateWith`, reject and chooser keys), `src/review/card.rs` (tier/catalog/query rows, error row, chooser hints), `src/review/mod.rs` (exports), `tests/steps/interactive_shell_shortcut_steps.rs` (bindings).
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'Rejecting a candidate opens the model chooser'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 8 steps (8 passed); full regular 200/200; lib 71 passed.
  ```
- [x] REFACTOR: Keep tier ordering in `chooser_tiers`. Rerun the exact targeted
  command.
  ```text
  command: `./run-tests.sh --name 'Rejecting a candidate opens the model chooser'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 8 steps (8 passed).
  ```
- [x] COMMIT: `d53e959557a08839734550f3b368362174b248ca` - feat(interactive-shell-shortcut): Rejecting a candidate opens the model chooser

### A configured tier can be chosen with its number

Domain constraints:

- Choosing a tier regenerates a fresh candidate at that tier and still requires
  acceptance.

- [x] RED: Remove `@wip` from this scenario only. Bind the new steps with
  `unimplemented!()`. Run the exact targeted command; it must exit non-zero.
  ```text
  command: `./run-tests.sh --name 'A configured tier can be chosen with its number'`
  output: exit 1; 1 feature / 1 scenario (1 failed) / 3 steps (2 passed, 1 failed); the reject-and-choose stub panicked.
  ```
- [x] GREEN: Add `RegenerateWith`, number-key handling, and the library
  `session::generate_candidate`/`parse_generated_candidate`; the scenario drives
  a real in-process provider against the world twin. Compile and run the exact
  targeted command.
  Production files changed: `src/review/session.rs` (new: `Generation`, `generate_candidate`, `parse_generated_candidate`, `chooser_tiers`, `fetch_catalog`), `src/review/mod.rs` (module export), `src/main.rs` (review generation through the session, reject/catalog worker, regenerate outcome), `tests/steps/interactive_shell_shortcut_steps.rs` (real provider twin binding).
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'A configured tier can be chosen with its number'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 6 steps (6 passed); full regular 201/201, e2e 81/81, lib 71 passed.
  ```
- [x] REFACTOR: Keep one regeneration path for tiers and models. Rerun the
  exact targeted command.
  ```text
  command: `./run-tests.sh --name 'A configured tier can be chosen with its number'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 6 steps (6 passed).
  ```
- [x] COMMIT: `5dafdc59c1d3e5fc991b10459e716bffb767f3a9` - feat(interactive-shell-shortcut): A configured tier can be chosen with its number

### A model name is suggested from the provider catalog while typing

Domain constraints:

- Typing filters catalog suggestions; choosing a suggestion regenerates with
  that model.

- [x] RED: Remove `@wip` from this scenario only. Bind the new steps with
  `unimplemented!()`. Run the exact targeted command; it must exit non-zero.
  ```text
  command: `./run-tests.sh --name 'A model name is suggested from the provider catalog while typing'`
  output: exit 1; 1 feature / 1 scenario (1 failed) / 4 steps (3 passed, 1 failed); the reject-and-type stub panicked.
  ```
- [x] GREEN: Add the chooser query, filtering, highlight, and Enter selection;
  `session::fetch_catalog` maps catalog entries. Compile and run the exact
  targeted command.
  Production files changed: `tests/steps/interactive_shell_shortcut_steps.rs` only (chooser typing, filtering, and Enter selection already live in `src/review/panel.rs` from the chooser scenario).
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'A model name is suggested from the provider catalog while typing'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 8 steps (8 passed); full regular 202/202; lib 71 passed.
  ```
- [x] REFACTOR: Keep filtering and highlighting pure. Rerun the exact targeted
  command.
  ```text
  command: `./run-tests.sh --name 'A model name is suggested from the provider catalog while typing'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 8 steps (8 passed).
  ```
- [x] COMMIT: `c298ebfe99ed013cb84f302b338c30f31d0e033a` - feat(interactive-shell-shortcut): A model name is suggested from the provider catalog while typing

### A typed model name works when the catalog is unavailable

Domain constraints:

- A typed model name is usable as entered even with no suggestions.

- [x] RED: Remove `@wip` from this scenario only. Bind the new steps with
  `unimplemented!()`. Run the exact targeted command; it must exit non-zero.
  ```text
  command: `./run-tests.sh --name 'A typed model name works when the catalog is unavailable'`
  output: exit 1; 1 feature / 1 scenario (1 failed) / 2 steps (1 passed, 1 failed); the catalog Given stub panicked.
  ```
- [x] GREEN: Enter with a non-empty query and no highlight regenerates with the
  typed model; the fetch failure path yields an empty catalog. Compile and run
  the exact targeted command.
  Production files changed: `tests/steps/interactive_shell_shortcut_steps.rs` only (`session::fetch_catalog` drives the unavailable-catalog Given; typed-model Enter selection already lives in `src/review/panel.rs`).
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'A typed model name works when the catalog is unavailable'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 7 steps (7 passed); full regular 203/203.
  ```
- [x] REFACTOR: Keep the choose-or-type branch explicit. Rerun the exact
  targeted command.
  ```text
  command: `./run-tests.sh --name 'A typed model name works when the catalog is unavailable'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 7 steps (7 passed).
  ```
- [x] COMMIT: `11a5c5b93fd998ffd0cece9f9a89b755166d53f3` - feat(interactive-shell-shortcut): A typed model name works when the catalog is unavailable

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
