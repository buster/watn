# Tasks: cancel-running-completion

## Setup

- [ ] Configure and prove the existing strict Cucumber runner. Confirm that
  `tests/features_runner.rs` executes both `givn/specs/` and this change's
  `specs/` tree, retains `.fail_on_skipped()`, and uses `unimplemented!()`
  for RED step bodies. Create the capability step file
  `tests/steps/cancel_completion_steps.rs`, register it with
  `pub mod cancel_completion_steps;` in `tests/steps/mod.rs`, and add the
  black-hole listener fixture. Keep all new step bodies non-empty; stubs
  must panic explicitly. Record the exact commands and non-zero proof here:
  ```text
  verify.command:
  ./run-tests.sh

  verify.e2e_command:
  ./run-tests.sh --e2e

  strict proof:
  root=$(mktemp -d /tmp/watn-cancel.XXXXXX) && trap 'rm -rf "$root"' EXIT && cargo build --bin watn && cp target/debug/watn "$root/default-debug" && cargo build --features test-support --bin watn && cp target/debug/watn "$root/test-support-debug" && WATN_DEFAULT_DEBUG_BIN="$root/default-debug" WATN_TEST_SUPPORT_DEBUG_BIN="$root/test-support-debug" cargo test --test features_runner --features test-support -- --name 'One Ctrl\+C cancels a completion waiting for streamed output'
  Result: non-zero. `1 feature`, `2 steps (1 passed, 1 failed)`, panicked with `not implemented: start watn in a PTY`, and Cargo returned `error: test failed`.
  ```
- [x] Run `givn lint --change cancel-running-completion` after setup and record
  the expected `@wip` findings only.
  ```text
  Result: exit 2 with 1 feature checked and 1 expected @wip finding (the
  connection scenario); no structural findings.
  ```

## E2E Scenarios

Both scenarios are `@e2e`; there are no non-E2E scenarios in this change.
The Cucumber runner itself starts the loopback streaming twin and the
black-hole listener per scenario; no external network or service required.

## Scenario: One Ctrl+C cancels a completion waiting for streamed output

- [x] RED: Remove `@wip` from this scenario, bind every step with explicit
  `unimplemented!()` stubs, and run only this scenario via the
  single-scenario E2E command from `design.md`. Expected result: non-zero
  exit caused by a matched stub. Evidence:
  ```text
  WATN_DEFAULT_DEBUG_BIN=$root/default-debug WATN_TEST_SUPPORT_DEBUG_BIN=$root/test-support-debug \
    cargo test --test features_runner --features test-support -- \
    --name 'One Ctrl\+C cancels a completion waiting for streamed output'
  Result: `1 feature`, `2 steps (1 passed, 1 failed)`, step panicked at
  `tests/steps/cancel_completion_steps.rs` with `not implemented: start watn in a PTY`,
  Cargo returned `error: test failed`.
  ```
- [x] GREEN: Drive the real binary in a PTY against the held-open streaming
  twin, wait for "printf first" to appear, press `\x03` (Ctrl+C), then
  assert exit status 130, that the merged PTY output still contains
  "printf first", and that no error text or final metadata appears.
  Implement the minimum production code: `Error::Interrupted` in
  `src/error.rs` (Display + exit code 130), the `interrupt: Arc<AtomicBool>`
  field and `parse_sse_stream` flag check in `src/provider/openai_compat.rs`,
  and the worker-thread + grace + interrupt wiring in `src/main.rs`.
  Test files: `tests/steps/cancel_completion_steps.rs`,
  `tests/steps/mod.rs` (module registration).
  Result: `1 feature, 1 scenario (1 passed), 8 steps (8 passed)`.
  Notes: the initial attempt returned status 3 (`network error: stream ended
  before [DONE]`) because the held-open fixture declared `Content-Length`, so
  the client hit clean EOF; added a no-`Content-Length` held-open mode
  (`StreamingServer::start_held_open`). Reading a partially-`Interrupted`
  read map raced the flag, so `io::ErrorKind::Interrupted` is mapped to
  `Error::Interrupted` unconditionally.
- [x] REFACTOR: Remove fixture/assertion duplication without changing the
  observable contract. Rerun this scenario and record a passing result:
  ```text
  Factored the worker-outcome tuple into `StreamOutcome` and reran the
  targeted command: `1 scenario (1 passed), 8 steps (8 passed)`.
  ```
- [x] COMMIT: `a55e930` - `feat(cancel-running-completion): One Ctrl+C cancels a completion waiting for streamed output`

## Scenario: One Ctrl+C cancels a completion waiting for a connection

- [x] RED: Remove `@wip` from this scenario, bind explicit `unimplemented!()`
  stubs, and run only this scenario via the single-scenario E2E command.
  Expected result: non-zero exit. Evidence — genuine control run with the
  interrupt-flag setting temporarily disabled in the `ctrlc` handler:
  ```text
  Targeted command reported `1 scenario (1 failed)`, `5 steps (4 passed,
  1 failed)` failed on `expected exit status 130, got Some(1)`: without the
  interrupt handling the connect-phase Ctrl+C is ignored, the 10 s PTY
  reaper kills the child, and Cargo returned `error: test failed`.
  ```
- [x] GREEN: Start the black-hole listener, run the real binary in a PTY,
  wait for the `Asking` spinner, press `\x03`, then assert exit status 130
  and no reported error. The black-hole listener accepts one connection,
  reads the request headers, holds, and is released/joined on drop. Production
  code was supplied by the streamed-output scenario; the grace/detach path in
  `src/main.rs` (`wait_for_stream_result`) is exercised here. Result:
  ```text
  `1 scenario (1 passed), 6 steps (6 passed)`.
  ```
- [x] REFACTOR: Keep the black-hole listener and PTY waits deterministic
  without weakening the no-error assertion. Targeted rerun:
  ```text
  Reused `read_request_headers` from the streaming twin module and reran:
  `1 scenario (1 passed), 6 steps (6 passed)`; clippy clean.
  ```
- [x] COMMIT: `0d1130e` - `feat(cancel-running-completion): One Ctrl+C cancels a completion waiting for a connection`

## Final Change Verification

- [ ] Remove all completed scenario `@wip` tags and run
  `givn lint --change cancel-running-completion`.
  ```text
  Result: TBD (clean; 1 file checked and no findings).
  ```
- [ ] Run `verify.command` and record its full scenario/step count and output.
  ```text
  Result: TBD.
  ```
- [ ] Run `verify.e2e_command` and record its full scenario/step count. Confirm
  it is a strict subset of `verify.command`.
  ```text
  Result: TBD.
  ```
- [ ] Run formatting, compilation, lint, unit, documentation, release-build,
  and diff checks appropriate to the repository, recording all results:
  ```text
  cargo fmt --all -- --check
  cargo check --all-targets
  cargo clippy --all-targets --all-features -- -D warnings
  cargo test --all-targets --features test-support (with explicit
  WATN_DEFAULT_DEBUG_BIN and WATN_TEST_SUPPORT_DEBUG_BIN bootstrap)
  cargo test --doc
  cargo build --release
  git diff --check
  Result: TBD.
  ```