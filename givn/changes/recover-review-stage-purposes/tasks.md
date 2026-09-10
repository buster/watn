# Tasks: recover-review-stage-purposes

All tasks are intentionally unchecked. Do not check a task until its command,
runner output, and (for scenario tasks) commit hash have been pasted into this
file. One atomic commit per scenario.

## Setup

- [x] Confirm the runner and strict mode for this change. Add one temporary
  step binding using `unimplemented!()` for the first scenario, remove `@wip`
  from that scenario only, and run
  `./run-tests.sh --name 'An unknown purpose status keeps matching model-written purposes'`;
  it must exit non-zero. Remove only the temporary binding after capturing
  evidence.
  ```text
  command: `./run-tests.sh --name 'An unknown purpose status keeps matching model-written purposes'`
  exit: 1
  output: Scenario failed at the first Given: `Step panicked. Captured output: not implemented: strict-mode proof: purpose recovery not implemented`; 1 feature / 1 scenario (1 failed) / 2 steps (1 passed, 1 failed).
  ```

- [x] Record the baseline before scenario implementation. Run `./run-tests.sh`
  and `./run-tests.sh --e2e` and record exit status and scenario counts.
  ```text
  regular command: `./run-tests.sh`
  regular output/count: exit 1; 21 features; 186 scenarios (185 passed, 1 failed); 1113 steps (1112 passed, 1 failed). The single failure is the de-`@wip`ed first scenario before its RED binding.
  e2e command: `./run-tests.sh --e2e`
  e2e output/count: exit 0; 24 features; 81 scenarios (81 passed); 592 steps (592 passed).
  ```

- [x] Extend `tests/steps/interactive_shell_shortcut_steps.rs` with the new
  bindings. RED bodies use `unimplemented!()`; no empty bodies. No E2E binding
  is added because no interaction is added.
  Evidence: new bindings in `tests/steps/interactive_shell_shortcut_steps.rs`:
  - `the provider returns a review response with an unknown purpose status and matching stage purposes` -> unknown-status scenario
  - `the provider returns a review response whose command spans several lines with matching stage purposes` -> multiline scenario
  - `the provider returns a review response with mismatched stage text` -> mismatch guard scenario
  - `the review surface should show the stage purpose {string}` -> unknown-status and multiline scenarios
  - `the reviewed candidate command should be a single line` -> multiline scenario
  No E2E binding is added because no interaction is added.

## Scenarios

### An unknown purpose status keeps matching model-written purposes

Domain constraints:

- A response whose trimmed stage text matches the derived stages and whose
  stages all carry non-empty purposes keeps those provider-written purposes.
- An unrecognized purpose-status word alone does not hide them.
- Watn never authors purpose text.

- [x] RED: Remove `@wip` from this scenario only. Bind the unknown-status Given
  and the stage-purpose Then with `unimplemented!()`. Run the exact targeted
  command; it must exit non-zero.
  ```text
  command: `./run-tests.sh --name 'An unknown purpose status keeps matching model-written purposes'`
  output: exit 1; 1 scenario (1 failed); 2 steps (1 passed, 1 failed); the stubs panicked.
  ```
- [x] GREEN: Implement recovered purposes and use them in the review surface.
  Compile with `cargo check --locked`, then run the exact targeted command.
  Production files changed: `src/review/response.rs` (`normalize_command`, `recovered_purposes`, recovered-purpose path in `candidate_from_provider_response`), `src/main.rs` (prompt lists accepted status values and forbids line breaks), unit tests.
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'An unknown purpose status keeps matching model-written purposes'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 7 steps (7 passed).
  ```
- [x] REFACTOR: Share stage/purpose agreement between strict validation and
  recovery. Rerun the exact targeted command.
  ```text
  command: `./run-tests.sh --name 'An unknown purpose status keeps matching model-written purposes'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 7 steps (7 passed).
  ```
- [x] COMMIT: `f59db1c7f5c93c55c267369c439ad1e56a9d0273` - `feat(interactive-shell-shortcut): An unknown purpose status keeps matching model-written purposes`

### A command broken across lines is explained on one row per stage

Domain constraints:

- Line breaks and tabs in a provider-written JSON command become spaces before
  the Command flow is derived.
- Matching stage purposes stay visible and every rendered value stays on one
  inline row.

- [x] RED: Remove `@wip` from this scenario only. Bind the multiline Given and
  the single-line Then with `unimplemented!()`. Run the exact targeted command;
  it must exit non-zero.
  ```text
  command: `./run-tests.sh --name 'A command broken across lines is explained on one row per stage'`
  output: exit 1; 1 scenario (1 failed); 2 steps (1 passed, 1 failed); the stubs panicked.
  ```
- [x] GREEN: Implement command normalization before flow derivation. Compile
  with `cargo check --locked`, then run the exact targeted command.
  Production files changed: `src/review/response.rs` (strict path normalizes the provider command before validation), unit test.
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'A command broken across lines is explained on one row per stage'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 6 steps (6 passed).
  ```
- [x] REFACTOR: Keep normalization in one place for strict and recovered
  responses. Rerun the exact targeted command.
  ```text
  command: `./run-tests.sh --name 'A command broken across lines is explained on one row per stage'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 7 steps (7 passed).
  ```
- [x] COMMIT: `4c25fbef5b25c155fba83a784b8d27d594022c5c` - `feat(interactive-shell-shortcut): A command broken across lines is explained on one row per stage`

### Mismatched stage text still shows purpose-unavailable

Domain constraints:

- When stage text does not match the derived stages, the candidate keeps its
  command and shows `purpose-unavailable`.
- Final acceptance remains mandatory; no purpose text is invented.

- [x] RED: Remove `@wip` from this scenario only. Bind the mismatched Given
  with `unimplemented!()`. Run the exact targeted command; it must exit
  non-zero.
  ```text
  command: `./run-tests.sh --name 'Mismatched stage text still shows purpose-unavailable'`
  output: exit 1; 1 scenario (1 failed); 2 steps (1 passed, 1 failed); the stub panicked.
  ```
- [x] GREEN: Ensure the mismatch guard falls back to purpose-unavailable.
  Compile with `cargo check --locked`, then run the exact targeted command.
  Production files changed: none beyond the agreement guard from `f59db1c`; mismatch/missing-purpose unit test.
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'Mismatched stage text still shows purpose-unavailable'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 6 steps (6 passed).
  ```
- [x] REFACTOR: Make the agreement guard explicit and shared. Rerun the exact
  targeted command.
  ```text
  command: `./run-tests.sh --name 'Mismatched stage text still shows purpose-unavailable'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 7 steps (7 passed).
  ```
- [x] COMMIT: `eb86b7bc47677ed15be09aaf70cb2a0e3ef2fc2b` - `feat(interactive-shell-shortcut): Mismatched stage text still shows purpose-unavailable`

## Final Verification

- [x] Run `givn lint --change recover-review-stage-purposes` and confirm no
  `@wip` finding remains.
  ```text
  command: `givn lint --change recover-review-stage-purposes`
  output: exit 0; `givn lint: 1 file(s) checked — clean`.
  ```
- [x] Run the full regular suite `./run-tests.sh`; confirm zero exit and paste
  counts.
  ```text
  command: `./run-tests.sh`
  output: exit 0; 21 features; 188 scenarios (188 passed); 1130 steps (1130 passed).
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
- [x] Run `givn status --change recover-review-stage-purposes` and confirm the
  next artifact is `review`.
  ```text
  command: `givn status --change recover-review-stage-purposes`
  output: all 24 tasks checked; artifacts proposal, specs, design, arc42-docs, design-review, tasks complete; next required artifact is `review`.
  ```
