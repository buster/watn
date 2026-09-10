# Tasks: accept-provider-stage-split

All tasks are intentionally unchecked. Do not check a task until its command,
runner output, and (for scenario tasks) commit hash have been pasted into this
file. One atomic commit per scenario.

## Setup

- [x] Confirm the runner and strict mode for this change. Add one temporary
  step binding using `unimplemented!()` for the first scenario, remove `@wip`
  from that scenario only, and run the targeted command; it must exit non-zero.
  Remove only the temporary binding after capturing evidence.
  ```text
  command: `./run-tests.sh --name 'A provider stage split that covers the command is shown with its purposes'`
  exit: 1
  output: Scenario failed at the first Given: `Step panicked. Captured output: not implemented: strict-mode proof: provider split acceptance not implemented`; 1 feature / 1 scenario (1 failed) / 2 steps (1 passed, 1 failed).
  ```

- [x] Record the baseline before scenario implementation. Run `./run-tests.sh`
  and `./run-tests.sh --e2e` and record exit status and scenario counts.
  ```text
  regular command: `./run-tests.sh`
  regular output/count: exit 1; 21 features; 189 scenarios (188 passed, 1 failed); 1132 steps (1131 passed, 1 failed). The single failure is the de-`@wip`ed first scenario before its RED binding.
  e2e command: `./run-tests.sh --e2e`
  e2e output/count: exit 0; 24 features; 81 scenarios (81 passed); 592 steps (592 passed).
  ```

- [x] Extend `tests/steps/interactive_shell_shortcut_steps.rs` with the new
  bindings. RED bodies use `unimplemented!()`; no empty bodies. No E2E binding
  is added because no interaction is added.
  Evidence: new bindings in `tests/steps/interactive_shell_shortcut_steps.rs`:
  - `the provider returns a review response whose stage split covers the command` -> covered-split scenario
  - `the provider returns a review response whose stage text is not part of the command` -> untrusted-split scenario
  - `the review surface should show the stage {string}` -> covered-split and untrusted scenarios
  No E2E binding is added because no interaction is added.

## Scenarios

### A provider stage split that covers the command is shown with its purposes

Domain constraints:

- A provider split is accepted when every stage text appears verbatim in the
  command, in order, without overlap, with only whitespace or shell separators
  between, and every stage has a non-empty purpose.
- The accepted stages become the displayed Command flow with model-written
  purposes. Watn never authors stage or purpose text.

- [x] RED: Remove `@wip` from this scenario only. Bind the covered-split Given
  and the stage Then with `unimplemented!()`. Run the exact targeted command;
  it must exit non-zero.
  ```text
  command: `./run-tests.sh --name 'A provider stage split that covers the command is shown with its purposes'`
  output: exit 1; 1 scenario (1 failed); 2 steps (1 passed, 1 failed); the stubs panicked.
  ```
- [x] GREEN: Implement the provider split contract and use it in the review
  surface. Compile with `cargo check --locked`, then run the exact targeted
  command. Production files changed: `src/review/flow.rs` (`command_stage`), `src/review/response.rs` (`provider_stage_split` wired into `candidate_from_provider_response`), unit tests for covered and untrusted splits.
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'A provider stage split that covers the command is shown with its purposes'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 9 steps (9 passed).
  ```
- [x] REFACTOR: Share range validation between the provider split and the
  derived-stage match. Rerun the exact targeted command.
  ```text
  command: `./run-tests.sh --name 'A provider stage split that covers the command is shown with its purposes'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 9 steps (9 passed).
  ```
- [x] COMMIT: `72a007b60f1960a7c402469e44590153dd81ea97` - `feat(interactive-shell-shortcut): A provider stage split that covers the command is shown with its purposes`

### Provider stages that do not cover the command are not trusted

Domain constraints:

- A stage text that does not appear verbatim in the command is not trusted.
- The candidate keeps its recovered command and shows `purpose-unavailable`.
- Final acceptance remains mandatory.

- [x] RED: Remove `@wip` from this scenario only. Bind the untrusted-split
  Given with `unimplemented!()`. Run the exact targeted command; it must exit
  non-zero.
  ```text
  command: `./run-tests.sh --name 'Provider stages that do not cover the command are not trusted'`
  output: exit 1; 1 scenario (1 failed); 2 steps (1 passed, 1 failed); the stub panicked.
  ```
- [x] GREEN: Ensure the guard rejects non-covering splits. Compile with
  `cargo check --locked`, then run the exact targeted command. Production
  files changed: none beyond the verbatim-coverage guard from `72a007b`; untrusted-split unit tests.
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'Provider stages that do not cover the command are not trusted'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 6 steps (6 passed).
  ```
- [x] REFACTOR: Keep the guard explicit and shared. Rerun the exact targeted
  command.
  ```text
  command: `./run-tests.sh --name 'Provider stages that do not cover the command are not trusted'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 9 steps (9 passed).
  ```
- [x] COMMIT: `8266132ae6a64dfef67c5cde2429ecf20aca7a4b` - `feat(interactive-shell-shortcut): Provider stages that do not cover the command are not trusted`

## Final Verification

- [x] Run `givn lint --change accept-provider-stage-split` and confirm no
  `@wip` finding remains.
  ```text
  command: `givn lint --change accept-provider-stage-split`
  output: exit 0; `givn lint: 1 file(s) checked — clean`.
  ```
- [x] Run the full regular suite `./run-tests.sh`; confirm zero exit and paste
  counts.
  ```text
  command: `./run-tests.sh`
  output: exit 0; 21 features; 190 scenarios (190 passed); 1145 steps (1145 passed).
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
- [x] Run `givn status --change accept-provider-stage-split` and confirm the
  next artifact is `review`.
  ```text
  command: `givn status --change accept-provider-stage-split`
  output: all 21 tasks checked; artifacts proposal, specs, design, arc42-docs, design-review, tasks complete; next required artifact is `review`.
  ```
