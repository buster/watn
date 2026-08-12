# Tasks: Preserve Ctrl-W Requests In Shell Config

## Setup

- [x] Confirm the configured Cucumber runner executes the change spec and
  permanent specs with `.fail_on_skipped()`, using the commands in
  `givn/commands.yaml`.
  - Create `tests/steps/preserve_ctrl_w_requests_steps.rs`, registered from
    `tests/steps/mod.rs`, reusing shared widget fixtures and helpers.
  - Use `unimplemented!()` for the first new assertion and run the exact
    targeted command; paste the non-zero strict-mode output here.
   - Evidence: The configured runner was verified with:
     `root=$(mktemp -d /tmp/watn-transport.XXXXXX) && trap 'rm -rf "$root"' EXIT && cargo build --locked --bin watn && cp target/debug/watn "$root/default-debug" && cargo build --locked --features test-support --bin watn && cp target/debug/watn "$root/test-support-debug" && WATN_DEFAULT_DEBUG_BIN="$root/default-debug" WATN_TEST_SUPPORT_DEBUG_BIN="$root/test-support-debug" cargo test --locked --test features_runner --features test-support -- --name 'Only the generated command executes when the buffer is committed'`.
     The scenario ran with two existing steps passing and the new
     `I execute the resulting Bash buffer` step matched and panicked with
     `not implemented`; the process exited non-zero. This proves strict
     handling of the registered stub. The capability module is registered in
     `tests/steps/mod.rs`.

## Scenario: A successful generation keeps the original request visible as a comment

- [x] RED: Remove `@wip` from this scenario only, bind new steps with
  `unimplemented!()`, and run the exact targeted command. Expected non-zero.
   - Evidence: The targeted runner exited non-zero. The Given and When steps
     passed, then `the current command line should be exactly
     "# show status\nprintf 'ready'"` failed because the actual line was
     `printf 'ready'`.

- [x] GREEN: Update the Bash widget in `src/shell_shortcut.rs` to replace the
  buffer with `# <flattened>` + newline + result and cursor at the end.
  Production files: `src/shell_shortcut.rs`. Run the targeted scenario and
  record zero exit.
   - Evidence: `cargo check --locked --features test-support` passed. The
     targeted runner then passed with `1 scenario (1 passed)` and `4 steps
     (4 passed)`. The Bash block now flattens CR/LF/TAB in the request and
     assigns the comment plus generated output without evaluating it.

- [x] REFACTOR: Keep behavior unchanged; rerun the targeted scenario and record
  zero exit.
   - Evidence: The unchanged targeted runner passed with `1 scenario (1
     passed)` and `4 steps (4 passed)` after the implementation was reviewed;
     no additional refactor was needed.

- [ ] COMMIT: Create one atomic commit referencing `A successful generation keeps the original request visible as a comment`.
- Commit hash: pending

## Scenario: Only the generated command executes when the buffer is committed

- [ ] RED: Remove `@wip` from this scenario only, bind new commit/execution steps
  with `unimplemented!()`, and run the exact targeted command. Expected
  non-zero.
  - Evidence:

- [ ] GREEN: Add a step that executes the produced Bash buffer with `bash -c`
  and asserts the generated command side effect runs while a comment-embedded
  command does not. Production files: (none; step infrastructure only).
  Run the targeted scenario and record zero exit.
  - Evidence:

- [ ] REFACTOR: Preserve behavior; rerun the targeted scenario and record zero
  exit.
  - Evidence:

- [ ] COMMIT: Create one atomic commit referencing `Only the generated command executes when the buffer is committed`.
- Commit hash: pending

## Scenario: Requests with metacharacters and embedded newlines remain one comment line

- [ ] RED: Remove `@wip` from this scenario only, bind the flattening assertion
  with `unimplemented!()`, and run the exact targeted command. Expected
  non-zero.
  - Evidence:

- [ ] GREEN: Ensure the widget flattens CR/LF/TAB to spaces in the comment.
  Production files: `src/shell_shortcut.rs`. Run the targeted scenario and
  record zero exit.
  - Evidence:

- [ ] REFACTOR: Keep behavior unchanged; rerun the targeted scenario and record
  zero exit.
  - Evidence:

- [ ] COMMIT: Create one atomic commit referencing `Requests with metacharacters and embedded newlines remain one comment line`.
- Commit hash: pending

## Scenario: Failed or empty generation preserves the original buffer

- [ ] RED: Remove `@wip` from this scenario only, bind its steps by reusing the
  existing failure/empty fixture and `current line` steps, and run the exact
  targeted command. If all steps are reused, record the zero exit as immediate
  GREEN evidence.
  - Evidence:

- [ ] GREEN: Confirm the failure/empty preservation contract is unchanged by the
  widget updates. Production files: `src/shell_shortcut.rs` (verification only).
  Run the targeted scenario and record zero exit.
  - Evidence:

- [ ] REFACTOR: Preserve behavior; rerun the targeted scenario and record zero
  exit.
  - Evidence:

- [ ] COMMIT: Create one atomic commit referencing `Failed or empty generation preserves the original buffer`.
- Commit hash: pending

## Scenario: Zsh and Fish widgets preserve the request as a comment

- [ ] RED: Remove `@wip` from this scenario only, bind new Zsh/Fish content and
  syntax steps with `unimplemented!()`, and run the exact targeted command.
  Expected non-zero.
  - Evidence:

- [ ] GREEN: Update `ZSH_BLOCK` and `FISH_BLOCK` in `src/shell_shortcut.rs` to
  build the comment + result buffer. Production files: `src/shell_shortcut.rs`.
  Run the targeted scenario and record zero exit.
  - Evidence:

- [ ] REFACTOR: Keep behavior unchanged; rerun the targeted scenario and record
  zero exit.
  - Evidence:

- [ ] COMMIT: Create one atomic commit referencing `Zsh and Fish widgets preserve the request as a comment`.
- Commit hash: pending

## Scenario: The generated Bash widget keeps the request visible and does not evaluate the command (@e2e)

- [ ] REMOVE `@wip` from this scenario only, bind the E2E assertions, and run the
  E2E command targeted by name. Expected non-zero.
  - Evidence:

- [ ] GREEN: Use the existing E2E Bash subprocess to assert the buffer contains
  `# find all images` above `printf 'hello world'` and that the replacement
  text is not executed. Production files: `src/shell_shortcut.rs`. Run the
  targeted E2E scenario and record zero exit.
  - Evidence:

- [ ] REFACTOR: Remove assertion duplication; rerun the targeted E2E scenario
  and record zero exit.
  - Evidence:

- [ ] COMMIT: Create one atomic commit referencing `The generated Bash widget keeps the request visible and does not evaluate the command`.
- Commit hash: pending

## Modified permanent scenarios

- [ ] Remove `@wip` from `A successful widget inserts one normalized command and moves the cursor to its end` (modified) and verify it passes with the shared
  steps. Run the exact targeted command and record zero exit.
  - Evidence:

- [ ] Remove `@wip` from `Embedded multiline output remains buffer text without evaluation` (modified) and verify it passes. Run the exact targeted command
  and record zero exit.
  - Evidence:

- [ ] Remove `@wip` from `The generated Bash widget runs through Bash without evaluating its result` (modified e2e) and verify it passes. Run the E2E command
  targeted by name and record zero exit.
  - Evidence:

- [ ] Commit the modified-scenario verifications atomically referencing the
  modified scenario titles.
- Commit hash: pending

## Full Verification

- [ ] `givn lint --change preserve-ctrl-w-requests-in-shell-config` clean with
  no `@wip` remaining.
- [ ] `./run-tests.sh` and `./run-tests.sh --e2e` pass with the E2E count
  strictly smaller.
- [ ] `./measure-coverage.sh` and `./merge-coverages.sh` produce fresh reports
  covering the runner and instrumented PTY/child processes.
- [ ] `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features --
  -D warnings`, and `git diff --check` pass.
- Evidence:
