# Tasks: Fix Fish Ctrl-W Completion Insertion

## Setup

- [x] Configure and prove the strict Gherkin runner, register the capability's
  step-definition file, and confirm the main and e2e commands remain distinct.
  Strict-mode proof command:
  `cargo test --locked --test features_runner --features test-support -- --name '^Fish keeps the generated command executable after Ctrl-W$'`
  Proof output: non-zero exit; `Step doesn't match any function`, followed by
  `1 scenario (1 failed)` and `error: test failed`.
  The runner uses `.fail_on_skipped()` in `tests/features_runner.rs`; the
  existing `givn/commands.yaml` entries provide separate non-e2e and `@e2e`
  tag-filtered commands.
  Required files: `tests/steps/fish_ctrl_w_completion_e2e_steps.rs` and
  `tests/steps/mod.rs`.

## Scenario: Fish keeps the generated command executable after Ctrl-W

- [x] RED: Remove `@wip` for this scenario, add non-empty pending step
  definitions, and run the exact single-scenario command from `design.md`.
  Runner command:
  `cargo test --locked --test features_runner --features test-support -- --name '^Fish keeps the generated command executable after Ctrl-W$'`
  Runner output: non-zero exit; the matched Given step panicked with
  `not implemented: Fish Ctrl-W fixture is not implemented yet`, followed by
  `1 scenario (1 failed)` and `error: test failed`.
- [x] GREEN: Drive the real Fish reader through `portable-pty`, assert the
  captured command-line buffer contains an actual newline, and update
  `src/shell_shortcut.rs` to construct that buffer correctly. Production files
  modified: `src/shell_shortcut.rs`.
  Runner command:
  `cargo test --locked --test features_runner --features test-support -- --name '^Fish keeps the generated command executable after Ctrl-W$'`
  Runner output: zero exit; `1 scenario (1 passed)` and `3 steps (3 passed)`.
- [x] REFACTOR: Simplify the Fish e2e fixture and generated buffer assembly
  without changing behaviour; rerun the exact single-scenario command.
  The existing Fish source assertion was updated to match the named buffer
  assembly while retaining the same observable requirement.
  Runner command:
  `cargo test --locked --test features_runner --features test-support -- --name '^Fish keeps the generated command executable after Ctrl-W$'`
  Runner output: zero exit; `1 scenario (1 passed)` and `3 steps (3 passed)`.
- [ ] COMMIT: Create one atomic commit whose message references the scenario
  title verbatim and record the commit hash here: pending.
