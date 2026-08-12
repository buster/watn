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

- [x] COMMIT: Create one atomic commit referencing `A successful generation keeps the original request visible as a comment`.
- Commit hash: `3ac22d8`

## Scenario: Only the generated command executes when the buffer is committed

- [x] RED: Remove `@wip` from this scenario only, bind new commit/execution steps
  with `unimplemented!()`, and run the exact targeted command. Expected
  non-zero.
  - Evidence: The targeted runner exited non-zero after the Given and initial
    When passed. The registered `I execute the resulting Bash buffer` step
    matched and panicked with `not implemented`.

- [x] GREEN: Add a step that executes the produced Bash buffer with `bash -c`
  and asserts the generated command side effect runs while a comment-embedded
  command does not. Production files: (none; step infrastructure only).
  Run the targeted scenario and record zero exit.
  - Evidence: `cargo check --locked --test features_runner --features
    test-support` passed. The targeted runner passed with `1 scenario (1
    passed)` and `5 steps (5 passed)`; it executed the returned buffer through
    `bash -c`, found the generated-command file, and confirmed the
    comment-embedded command file was absent.

- [x] REFACTOR: Preserve behavior; rerun the targeted scenario and record zero
  exit.
  - Evidence: The unchanged targeted runner passed again with `1 scenario (1
    passed)` and `5 steps (5 passed)`; no behavior-changing refactor was
    necessary.

- [x] COMMIT: Create one atomic commit referencing `Only the generated command executes when the buffer is committed`.
- Commit hash: `c806db3`

## Scenario: Requests with metacharacters and embedded newlines remain one comment line

- [x] RED: Remove `@wip` from this scenario only, bind the flattening assertion
  with `unimplemented!()`, and run the exact targeted command. Expected
  non-zero.
  - Evidence: After adding the escaped-control-character fixture step, the
    targeted runner passed the Given, When, and exact line assertion, then
    matched `the preserved request comment should be a single line` and
    exited non-zero on its `not implemented` body.

- [x] GREEN: Ensure the widget flattens CR/LF/TAB to spaces in the comment.
  Production files: `src/shell_shortcut.rs`. Run the targeted scenario and
  record zero exit.
  - Evidence: `cargo check --locked --test features_runner --features
    test-support` passed. The targeted runner passed with `1 scenario (1
    passed)` and `4 steps (4 passed)`, including an input containing an
    embedded newline and metacharacters; the request became one `#` comment
    line with the control character replaced by a space.

- [x] REFACTOR: Keep behavior unchanged; rerun the targeted scenario and record
  zero exit.
  - Evidence: The unchanged targeted runner passed again with `1 scenario (1
    passed)` and `4 steps (4 passed)`; the shared buffer parser is reused by
    the execution and comment assertions.

- [x] COMMIT: Create one atomic commit referencing `Requests with metacharacters and embedded newlines remain one comment line`.
- Commit hash: `7d37e64`

## Scenario: Failed or empty generation preserves the original buffer

- [x] RED: Remove `@wip` from this scenario only, bind its steps by reusing the
  existing failure/empty fixture and `current line` steps, and run the exact
  targeted command. If all steps are reused, record the zero exit as immediate
  GREEN evidence.
  - Evidence: `@wip` was removed and every step reused an existing strict
    binding; the targeted runner passed with `1 scenario (1 passed)` and `6
    steps (6 passed)`, so no new RED stub was required.

- [x] GREEN: Confirm the failure/empty preservation contract is unchanged by the
  widget updates. Production files: `src/shell_shortcut.rs` (verification only).
  Run the targeted scenario and record zero exit.
  - Evidence: The same targeted run passed with `1 scenario (1 passed)` and `6
    steps (6 passed)`; both non-zero and empty generation left the original
    buffer unchanged. No production change was needed.

- [x] REFACTOR: Preserve behavior; rerun the targeted scenario and record zero
  exit.
  - Evidence: The unchanged targeted runner passed again with `1 scenario (1
    passed)` and `6 steps (6 passed)`; the existing failure and empty-output
    fixture remains sufficient.

- [x] COMMIT: Create one atomic commit referencing `Failed or empty generation preserves the original buffer`.
- Commit hash: `6735dbd`

## Scenario: Zsh and Fish widgets preserve the request as a comment

- [x] RED: Remove `@wip` from this scenario only, bind new Zsh/Fish content and
  syntax steps with `unimplemented!()`, and run the exact targeted command.
  Expected non-zero.
  - Evidence: The targeted runner passed the installed Zsh/Fish fixture, then
    matched the new Zsh content assertion and exited non-zero on its
    `not implemented` body. Syntax steps were bound to the existing strict
    shell-check helpers.

- [x] GREEN: Update `ZSH_BLOCK` and `FISH_BLOCK` in `src/shell_shortcut.rs` to
  build the comment + result buffer. Production files: `src/shell_shortcut.rs`.
  Run the targeted scenario and record zero exit.
  - Evidence: `cargo check --locked --test features_runner --features
    test-support` passed. The targeted runner passed with `1 scenario (1
    passed)` and `5 steps (5 passed)`, asserting comment-plus-command
    construction for both generated configurations and invoking the existing
    Zsh/Fish syntax-check steps.

- [x] REFACTOR: Keep behavior unchanged; rerun the targeted scenario and record
  zero exit.
  - Evidence: The unchanged targeted runner passed again with `1 scenario (1
    passed)` and `5 steps (5 passed)`; no behavior-changing refactor was
    needed.

- [x] COMMIT: Create one atomic commit referencing `Zsh and Fish widgets preserve the request as a comment`.
- Commit hash: `8b7e722`

## Scenario: The generated Bash widget keeps the request visible and does not evaluate the command (@e2e)

- [x] REMOVE `@wip` from this scenario only, bind the E2E assertions, and run the
  E2E command targeted by name. Expected non-zero.
  - Evidence: The targeted E2E runner command used the real Bash subprocess
    path and passed the Given, When, and existing exact-buffer assertion before
    matching the new `the Bash process should preserve the request as a
    comment` step, which exited non-zero on `not implemented`. The configured
    tag-filter command cannot combine `--tags` with `--name` in cucumber-rs, so
    the scenario-name invocation was used for this RED run.

- [x] GREEN: Use the existing E2E Bash subprocess to assert the buffer contains
  `# find all images` above `printf 'hello world'` and that the replacement
  text is not executed. Production files: `src/shell_shortcut.rs`. Run the
  targeted E2E scenario and record zero exit.
  - Evidence: `cargo check --locked --test features_runner --features
    test-support` passed. The scenario-name E2E runner passed with `1 scenario
    (1 passed)` and `5 steps (5 passed)` through the real Bash subprocess; it
    asserted the comment-plus-command buffer and no replacement evaluation.

- [x] REFACTOR: Remove assertion duplication; rerun the targeted E2E scenario
  and record zero exit.
  - Evidence: The shared `captured_bash_line` helper now serves both E2E
    assertions. The unchanged scenario-name E2E runner passed with `1 scenario
    (1 passed)` and `5 steps (5 passed)`.

- [x] COMMIT: Create one atomic commit referencing `The generated Bash widget keeps the request visible and does not evaluate the command`.
- Commit hash: `d7ecebc`

## Modified permanent scenarios

- [x] Remove `@wip` from `A successful widget inserts one normalized command and moves the cursor to its end` (modified) and verify it passes with the shared
  steps. Run the exact targeted command and record zero exit.
  - Evidence: Removed only `@wip` from this `@givn.modified` scenario. The
    targeted runner used `--tags '@givn.modified and not @wip'` because
    cucumber-rs cannot combine `--tags` with `--name` and the permanent spec
    has the same title; it passed with `1 scenario (1 passed)` and `4 steps (4
    passed)` using the shared Bash widget steps.

- [x] Remove `@wip` from `Embedded multiline output remains buffer text without evaluation` (modified) and verify it passes. Run the exact targeted command
  and record zero exit.
  - Evidence: Removed only `@wip` from this `@givn.modified` scenario and
    updated the durable permanent expectation to the same comment-preserving
    buffer. The title-targeted runner passed both the delta and permanent
    copies with `2 scenarios (2 passed)` and `12 steps (12 passed)`.

- [x] Remove `@wip` from `The generated Bash widget runs through Bash without evaluating its result` (modified e2e) and verify it passes. Run the E2E command
  targeted by name and record zero exit.
  - Evidence: Removed only `@wip` from the `@givn.modified @e2e` scenario and
    updated the durable permanent scenario to the comment-preserving buffer.
    The title-targeted E2E runner passed both real Bash subprocess copies with
    `2 scenarios (2 passed)` and `9 steps (9 passed)`.

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
