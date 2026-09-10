# Tasks: fix-review-response-fallback

All tasks are intentionally unchecked. Do not check a task until its command,
runner output, and (for scenario tasks) commit hash have been pasted into this
file. Implement the scenarios in order. One atomic commit per scenario.

## Setup

- [x] Confirm the runner and strict mode for this change. The runner is the
  existing `./run-tests.sh` / `./run-tests.sh --e2e` (`givn/commands.yaml`)
  over `tests/features_runner.rs` with `.fail_on_skipped()` and step modules
  registered in `tests/steps/mod.rs`. Add one temporary step binding using
  `unimplemented!()` in the first scenario, remove `@wip` from that scenario
  only, and run `./run-tests.sh --name '<first scenario title>'`; it must exit
  non-zero. Remove only the temporary binding after capturing evidence.
  Evidence:
  ```text
  command: `./run-tests.sh --name 'A markdown-fenced structured response is still explained'`
  exit: 1
  output:
  Feature: Review response recovery and inline rendering safety
    Scenario: A markdown-fenced structured response is still explained
     ✘  And the provider returns a markdown-fenced structured review response:
        Step failed:
        Matched: tests/steps/interactive_shell_shortcut_steps.rs:2568:1
        Step panicked. Captured output: not implemented: strict-mode proof: recovery step not implemented
  [Summary] 1 feature / 1 scenario (1 failed) / 2 steps (1 passed, 1 failed)
  ```

- [x] Record the baseline before scenario implementation. Run `./run-tests.sh`
  and `./run-tests.sh --e2e` and record exit status and scenario counts.
  Evidence:
  ```text
  regular command: `./run-tests.sh`
  regular output/count: exit 1; 21 features; 182 scenarios (181 passed, 1 failed); 1089 steps (1088 passed, 1 failed). The single failure is the de-`@wip`ed first scenario before its RED binding.
  e2e command: `./run-tests.sh --e2e`
  e2e output/count: exit 0; 24 features; 81 scenarios (81 passed); 592 steps (592 passed).
  ```

- [x] Extend the capability step file `tests/steps/interactive_shell_shortcut_steps.rs`
  with the new bindings for this change's scenarios. Do not use empty bodies,
  bare `pass`, or bare `return`; RED bodies use `unimplemented!()`. No E2E
  step is added because no interaction is added.
  Evidence: new bindings in `tests/steps/interactive_shell_shortcut_steps.rs`:
  - `the provider returns a markdown-fenced structured review response:` -> fenced/prose scenario
  - `the provider returns an invalid structured review response with the command {string}` -> invalid-response scenario
  - `the provider returns a structured review payload without a complete command` -> no-command scenario
  - `the provider returns a markdown-fenced structured review response with line breaks` -> multiline scenario
  - `the review surface should show the command {string}` -> invalid-response scenario
  - `every rendered review value should stay on one inline row` -> multiline scenario
  No E2E binding is added because no interaction is added.

## Scenarios

### A markdown-fenced structured response is still explained

Domain constraints:

- A valid structured review response is recognized when the provider wraps it
  in a markdown code fence, with or without surrounding prose.
- Stage text and model-written purposes come from the provider response; Watn
  never authors substitute text.
- Review bytes stay on the controlling-terminal channel, not stdout.

- [x] RED: Remove `@wip` from this scenario only. Bind the new fenced-response
  Given with `unimplemented!()`. Run the exact targeted command; it must exit
  non-zero.
  ```text
  command: `./run-tests.sh --name 'A markdown-fenced structured response is still explained'`
  output: exit 1; 1 scenario (1 failed); 2 steps (1 passed, 1 failed); the stub panicked.
  ```
- [x] GREEN: Implement fence/prose tolerance in the response locator and use
  the recovery function in the review path. Compile with `cargo check --locked`,
  then run the exact targeted command. Production files changed: `src/review/response.rs` (fence/prose locator, tolerant parse, `candidate_from_provider_response`), `src/main.rs` (review path uses the recovery function; prompt forbids code fences), `src/review/mod.rs` export; harness uses the shared recovery function.
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'A markdown-fenced structured response is still explained'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 8 steps (8 passed).
  ```
- [x] REFACTOR: Remove duplication in the payload-locating helpers without
  changing recognized responses. Rerun the exact targeted command.
  ```text
  command: `./run-tests.sh --name 'A markdown-fenced structured response is still explained'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 8 steps (8 passed).
  ```
- [x] COMMIT: `5861b00be6de93fa7e89f8456a88d9a09483809e` - `feat(interactive-shell-shortcut): A markdown-fenced structured response is still explained`

### An invalid structured response with a command stays reviewable

Domain constraints:

- A JSON-shaped payload that fails strict validation but contains a complete
  non-empty `command` keeps that provider-written command reviewable.
- Purpose status is `purpose-unavailable`; Watn does not invent purpose text.
- Final acceptance remains mandatory.

- [x] RED: Remove `@wip` from this scenario only. Bind the invalid-response
  Given with `unimplemented!()`. Run the exact targeted command; it must exit
  non-zero.
  ```text
  command: `./run-tests.sh --name 'An invalid structured response with a command stays reviewable'`
  output: exit 1; 1 scenario (1 failed); 2 steps (1 passed, 1 failed); the stub panicked.
  ```
- [x] GREEN: Implement command recovery for invalid JSON-shaped payloads.
  Compile with `cargo check --locked`, then run the exact targeted command.
  Production files changed: none beyond `candidate_from_provider_response` from `5861b00`; unit tests for command recovery.
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'An invalid structured response with a command stays reviewable'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 6 steps (6 passed).
  ```
- [x] REFACTOR: Share the payload-shape classification between strict parsing
  and recovery. Rerun the exact targeted command.
  ```text
  command: `./run-tests.sh --name 'An invalid structured response with a command stays reviewable'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 8 steps (8 passed).
  ```
- [x] COMMIT: `ec19570e1d338667cd603e65242e2528b1e643e7` - `feat(interactive-shell-shortcut): An invalid structured response with a command stays reviewable`

### A provider payload without a usable command releases nothing

Domain constraints:

- A payload with no complete non-empty command is `Unavailable`.
- The original input and shell line-editor buffer are preserved and nothing is
  released or recorded.

- [x] RED: Remove `@wip` from this scenario only. Bind the no-command Given
  with `unimplemented!()`. Run the exact targeted command; it must exit
  non-zero.
  ```text
  command: `./run-tests.sh --name 'A provider payload without a usable command releases nothing'`
  output: exit 1; 1 scenario (1 failed); 2 steps (1 passed, 1 failed); the stub panicked.
  ```
- [x] GREEN: Implement the `Unavailable` outcome for payloads without a usable
  command. Compile with `cargo check --locked`, then run the exact targeted
  command. Production files changed: `src/review/response.rs` (validated-but-invalid candidates fall through to recovery; empty command is unavailable); unit tests.
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'A provider payload without a usable command releases nothing'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 5 steps (5 passed).
  ```
- [x] REFACTOR: Keep the no-command path explicit and separate from command
  recovery. Rerun the exact targeted command.
  ```text
  command: `./run-tests.sh --name 'A provider payload without a usable command releases nothing'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 6 steps (6 passed).
  ```
- [x] COMMIT: `7a56539ad4de9a80121ec567f6177f0f357a2fdb` - `feat(interactive-shell-shortcut): A provider payload without a usable command releases nothing`

### A multiline provider payload keeps every rendered value on one row

Domain constraints:

- Every rendered candidate, intent, stage, purpose, and context value occupies
  a single inline row.
- The inline panel stays bounded and never repaints over its own layout.

- [x] RED: Remove `@wip` from this scenario only. Bind the multiline Given and
  the one-row assertion with `unimplemented!()`. Run the exact targeted
  command; it must exit non-zero.
  ```text
  command: `./run-tests.sh --name 'A multiline provider payload keeps every rendered value on one row'`
  output: exit 1; 1 scenario (1 failed); 2 steps (1 passed, 1 failed); the stubs panicked.
  ```
- [x] GREEN: Flatten line feed, carriage return, and tab in rendered values;
  keep the panel bounded. Compile with `cargo check --locked`, then run the
  exact targeted command. Production files changed: `src/review/panel.rs` sanitization flattens line feed, carriage return, and tab; unit tests.
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'A multiline provider payload keeps every rendered value on one row'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 5 steps (5 passed).
  ```
- [x] REFACTOR: Consolidate sanitization so every rendered field shares one
  flattening rule. Rerun the exact targeted command.
  ```text
  command: `./run-tests.sh --name 'A multiline provider payload keeps every rendered value on one row'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 6 steps (6 passed).
  ```
- [x] COMMIT: `69fd2e8` - `feat(interactive-shell-shortcut): A multiline provider payload keeps every rendered value on one row`

## Final Verification

- [x] Run `givn lint --change fix-review-response-fallback` and confirm no
  `@wip` finding remains.
  ```text
  command: `givn lint --change fix-review-response-fallback`
  output: exit 0; `givn lint: 1 file(s) checked — clean` (the informational subset note is an overlap disposition, recorded in review.md).
  ```
- [x] Run the full regular suite `./run-tests.sh`; confirm zero exit and paste
  counts.
  ```text
  command: `./run-tests.sh`
  output: exit 0; 21 features; 185 scenarios (185 passed); 1111 steps (1111 passed).
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
- [x] Run `givn status --change fix-review-response-fallback` and confirm the
  next artifact is `review`.
  ```text
  command: `givn status --change fix-review-response-fallback`
  output: all 24 tasks checked; artifacts proposal, specs, design, arc42-docs, design-review, tasks complete; next required artifact is `review`.
  ```
