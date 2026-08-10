# Tasks: incremental-sse-rendering

## Setup

- [x] Configure and prove the existing strict Cucumber runner. Confirm that
  `tests/features_runner.rs` executes both `givn/specs/` and this change's
  `specs/` tree, retains `.fail_on_skipped()`, and uses `unimplemented!()` for
  RED step bodies. Create the capability-specific non-E2E and E2E step files
  named in `design.md`, register them, and add the streaming state to the
  world. Keep all new step bodies non-empty; stubs must panic explicitly.
  Record the exact commands and non-zero proof here:
  ```text
  verify.command:
  root=$(mktemp -d /tmp/watn-transport.XXXXXX) && trap 'rm -rf "$root"' EXIT && cargo build --bin watn && cp target/debug/watn "$root/default-debug" && cargo build --features test-support --bin watn && cp target/debug/watn "$root/test-support-debug" && WATN_DEFAULT_DEBUG_BIN="$root/default-debug" WATN_TEST_SUPPORT_DEBUG_BIN="$root/test-support-debug" cargo test --test features_runner --features test-support -- --tags 'not @wip and not @e2e'

  verify.e2e_command:
  root=$(mktemp -d /tmp/watn-transport.XXXXXX) && trap 'rm -rf "$root"' EXIT && cargo build --bin watn && cp target/debug/watn "$root/default-debug" && cargo build --features test-support --bin watn && cp target/debug/watn "$root/test-support-debug" && WATN_DEFAULT_DEBUG_BIN="$root/default-debug" WATN_TEST_SUPPORT_DEBUG_BIN="$root/test-support-debug" cargo test --test features_runner --features test-support -- --tags '@e2e and not @wip'

  strict proof:
  root=$(mktemp -d /tmp/watn-transport.XXXXXX) && trap 'rm -rf "$root"' EXIT && cargo build --bin watn && cp target/debug/watn "$root/default-debug" && cargo build --features test-support --bin watn && cp target/debug/watn "$root/test-support-debug" && WATN_DEFAULT_DEBUG_BIN="$root/default-debug" WATN_TEST_SUPPORT_DEBUG_BIN="$root/test-support-debug" cargo test --test features_runner --features test-support -- --name "A usage-only final event supplies cost and throughput metadata"
  Result: non-zero. The targeted runner matched `tests/steps/incremental_sse_rendering_steps.rs:6`, panicked with `not implemented`, reported `1 step failed`, and Cargo returned `error: test failed`.
  ```
- [x] Run `givn lint --change incremental-sse-rendering` after setup and record
  the expected `@wip` findings only.
  ```text
  Result: exit 2 with 1 feature checked and 11 expected @wip findings; no structural findings.
  ```

## Non-E2E Scenarios

## Scenario: A usage-only final event supplies cost and throughput metadata

- [x] RED: Remove `@wip` from this scenario, bind every step with explicit
  `unimplemented!()` stubs or reused real steps, and run only this scenario with
  the single-scenario command from `design.md`. Expected result: non-zero exit
  caused by a matched stub. Evidence:
  ```text
  Targeted command exited non-zero. The runner matched
  `tests/steps/incremental_sse_rendering_steps.rs:6`, panicked with
  `not implemented`, reported `1 step failed`, and Cargo returned
  `error: test failed`.
  ```
- [x] GREEN: Implement the usage-only loopback stream, distinct requested and
  response models, response-model-only pricing, and stdout/stderr assertions
  for exact final model, positive tok/s, and non-zero cost. Implement the
  minimum provider/parser and CLI changes needed for the final aggregate.
  Production files: `src/provider/mod.rs`, `src/provider/openai_compat.rs`,
  `src/main.rs`, `src/output/render.rs`. Test files:
  `tests/features_runner.rs`, `tests/steps/mod.rs`,
  `tests/steps/incremental_sse_rendering_steps.rs`, and
  `tests/steps/incremental_sse_rendering_e2e_steps.rs`. Targeted result:
  ```text
  1 feature, 1 scenario, 10 steps passed.
  ```
- [x] REFACTOR: Remove fixture/assertion duplication without changing the
  observable contract. Rerun this scenario and record a passing result:
  ```text
  Removed the obsolete final-response renderer and reran the targeted command:
  1 feature, 1 scenario, 10 steps passed.
  ```
- [x] COMMIT: `fdd65a2` - `feat(incremental-sse-rendering): A usage-only final event supplies cost and throughput metadata`

## Scenario: A DONE event completes a stream successfully

- [x] RED: Remove `@wip`, bind explicit stubs, and run only this scenario. The
  held-connection assertion must fail before production changes because the
  current client waits for the complete body. Evidence:
  ```text
  Targeted command exited non-zero after matching the explicit `done_provider`
  stub; the runner reported `1 step failed` and Cargo returned `error: test failed`.
  ```
- [x] GREEN: Stop parsing at `[DONE]`, return the final aggregate without
  waiting for the server close, and implement the release-gated stream fixture
  and terminal completion assertions. No additional production file was needed;
  the provider callback/parser implementation from the prior scenario was
  exercised. Test file: `tests/steps/incremental_sse_rendering_steps.rs`.
  Targeted result:
  ```text
  1 feature, 1 scenario, 5 steps passed.
  ```
- [x] REFACTOR: Consolidate completion-marker handling and deterministic server
  release cleanup. Targeted rerun:
  ```text
  Renamed the implemented step bindings and reran the targeted command:
  1 feature, 1 scenario, 5 steps passed.
  ```
- [x] COMMIT: `94645d0` - `feat(incremental-sse-rendering): A DONE event completes a stream successfully`

## Scenario: Partial network reads are reassembled into complete events

- [x] RED: Remove `@wip`, bind explicit stubs, and run only this scenario. The
  pre-release observation must fail against the buffered implementation.
  Evidence:
  ```text
  Targeted command exited non-zero after matching `partial_provider`; the
  runner reported `1 step failed` and Cargo returned `error: test failed`.
  ```
- [x] GREEN: Replace whole-body buffering with buffered-reader SSE framing that
  handles byte-sized network writes while emitting the first complete content
  event before the held next event. No additional production files were needed;
  the parser implementation from the first scenario was exercised. Test files:
  `tests/steps/incremental_sse_rendering_steps.rs`; the buffered-reader
  and callback production implementation was already supplied by the first
  scenario. Targeted result:
  ```text
  1 feature, 1 scenario, 6 steps passed.
  ```
- [x] REFACTOR: Keep line framing and release cleanup minimal, then rerun:
  ```text
  Renamed the fixture bindings and reran the targeted command: 1 feature, 1
  scenario, 6 steps passed.
  ```
- [x] COMMIT: `76050ff` - `feat(incremental-sse-rendering): Partial network reads are reassembled into complete events`

## Scenario: Malformed nonessential events do not discard valid content

- [x] RED: Remove `@wip`, bind explicit stubs, and run only this scenario. The
  pre-`[DONE]` content observation must fail against final-body buffering.
  Evidence:
  ```text
  Targeted command exited non-zero after matching `malformed_provider`; the
  runner reported `1 step failed` and Cargo returned `error: test failed`.
  ```
- [x] GREEN: Ignore malformed nonessential JSON data lines, continue parsing,
  emit valid content before the held `[DONE]`, and assert the final command and
  successful status. No additional production files were needed; the parser
  implementation from the first scenario was exercised. Test file:
  `tests/steps/incremental_sse_rendering_steps.rs`. Targeted result:
  ```text
  1 feature, 1 scenario, 6 steps passed.
  ```
- [x] REFACTOR: Centralize malformed-line tolerance without changing valid
  event handling. Targeted rerun:
  ```text
  Reused the release-gated fragment assertion and reran the targeted command:
  1 feature, 1 scenario, 6 steps passed.
  ```
- [x] COMMIT: `2684870` - `feat(incremental-sse-rendering): Malformed nonessential events do not discard valid content`

## Scenario: EOF without DONE is a truncated stream

- [x] RED: Remove `@wip`, bind explicit stubs, and run only this scenario.
  Expected failure: current EOF behavior reports success instead of network
  status 3. Evidence:
  ```text
  Targeted command exited non-zero after matching `eof_provider`; the runner
  reported `1 step failed` and Cargo returned `error: test failed`.
  ```
- [x] GREEN: Track whether `[DONE]` was observed, reject clean EOF without it,
  preserve visible content, suppress metadata and execution, and assert status
  3. No additional production files were needed; the mandatory-completion
  implementation from the first scenario was exercised. Test file:
  `tests/steps/incremental_sse_rendering_steps.rs`. Targeted result:
  ```text
  1 feature, 1 scenario, 7 steps passed; exit status 3 was asserted.
  ```
- [x] REFACTOR: Make the truncation error and cleanup path explicit and reuse
  the stream fixture's close behavior. Targeted rerun:
  ```text
  Reused the provider close fixture and reran the targeted command: 1 feature,
  1 scenario, 7 steps passed.
  ```
- [x] COMMIT: `195c2dc` - `feat(incremental-sse-rendering): EOF without DONE is a truncated stream`

## Scenario: Output failure preserves the visible prefix and skips completion actions

- [x] RED: Remove `@wip`, bind explicit controlled-sink stubs, and run only this
  scenario. Expected failure: the current renderer has no streamed output sink
  error boundary. Evidence:
  ```text
  Targeted command exited non-zero after matching `controlled_sink_stub`; the
  runner reported `1 step failed` and Cargo returned `error: test failed`.
  ```
- [x] GREEN: Add a controlled writer seam or direct renderer test that fails on
  the next write/flush. Propagate the existing I/O error, retain the visible
  prefix, finish the spinner, omit metadata, and skip execution. Production
  files: `src/output/render.rs`, `src/main.rs`. Test file:
  `tests/steps/incremental_sse_rendering_steps.rs`. Targeted result:
  ```text
  1 feature, 1 scenario, 7 steps passed; exit status 1 and the controlled I/O
  error were asserted.
  ```
- [x] REFACTOR: Keep the writer seam test-only or narrowly scoped and ensure no
  duplicate output path remains. Targeted rerun:
  ```text
  Kept `write_streamed_content` narrow and reused it from the CLI callback;
  targeted rerun passed with 1 feature, 1 scenario, 7 steps.
  ```
- [x] COMMIT: `3371a0f` - `feat(incremental-sse-rendering): Output failure preserves the visible prefix and skips completion actions`

## E2E Setup

- [x] Before the first E2E scenario, confirm the local run command requires no
  external service: the loopback streaming twin starts inside the Cucumber
  process and is released/joined per scenario. Register
  `tests/steps/incremental_sse_rendering_e2e_steps.rs` separately from the
  non-E2E step module. Prove the configured E2E command is a strict subset of
  the non-E2E command by running both and recording scenario counts here:
  ```text
  verify.command count: 14 features, 62 scenarios, 344 steps passed.
  verify.e2e_command count: 16 features, 52 scenarios, 346 steps passed.
  Result: E2E count is strictly smaller: yes (52 < 62). The loopback streaming
  twin starts and is cleaned up inside the Cucumber process; no external service
  or network dependency is required.
  ```

## E2E Scenarios

## Scenario: Command text appears before a delayed stream completes

- [ ] RED: Remove only this scenario's `@wip`, bind E2E stubs with explicit
  `unimplemented!()`, and run only it through `verify.e2e_command`. Expected
  non-zero result from the first matched stub. Evidence:
  ```text
  [paste targeted E2E output]
  ```
- [ ] GREEN: Drive the real built binary in a PTY, observe spinner startup and
  flushed first content before releasing the delayed event, observe clear-line
  cleanup, then assert the complete command line exactly once and status 0.
  Production files: [list every file]. Test files: [list every file]. Targeted
  E2E result:
  ```text
  [paste targeted E2E output]
  ```
- [ ] REFACTOR: Make PTY waits and server release cleanup deterministic without
  weakening terminal assertions. Targeted E2E rerun:
  ```text
  [paste targeted E2E output]
  ```
- [ ] COMMIT: `[commit hash]` - `test(e2e): Command text appears before a delayed stream completes`

## Scenario: Verbose streaming keeps reasoning on stderr and command text on stdout

- [ ] RED: Remove only this scenario's `@wip`, bind explicit E2E stubs, and run
  only it through `verify.e2e_command`. Expected non-zero result. Evidence:
  ```text
  [paste targeted E2E output]
  ```
- [ ] GREEN: Drive the real `watn -v` subprocess, observe command stdout before
  completion, verify reasoning is absent before release, then assert final
  separate stdout/stderr channels, exact-once command, buffered reasoning, and
  status 0. Production files: [list every file]. Test files: [list every file].
  Targeted E2E result:
  ```text
  [paste targeted E2E output]
  ```
- [ ] REFACTOR: Reuse the release-gated subprocess fixture and keep channel
  assertions exact. Targeted E2E rerun:
  ```text
  [paste targeted E2E output]
  ```
- [ ] COMMIT: `[commit hash]` - `test(e2e): Verbose streaming keeps reasoning on stderr and command text on stdout`

## Scenario: A mid-stream failure preserves visible content and exits unsuccessfully

- [ ] RED: Remove only this scenario's `@wip`, bind explicit E2E stubs, and run
  only it through `verify.e2e_command`. Expected non-zero result. Evidence:
  ```text
  [paste targeted E2E output]
  ```
- [ ] GREEN: Drive the real binary in a PTY against a connection-reset twin.
  Assert visible prefix, spinner clear-line evidence, network status 3, no final
  metadata, no confirmation prompt, and non-zero exit. Production files: [list
  every file]. Test files: [list every file]. Targeted E2E result:
  ```text
  [paste targeted E2E output]
  ```
- [ ] REFACTOR: Consolidate failure cleanup and make the PTY terminal evidence
  stable. Targeted E2E rerun:
  ```text
  [paste targeted E2E output]
  ```
- [ ] COMMIT: `[commit hash]` - `test(e2e): A mid-stream failure preserves visible content and exits unsuccessfully`

## Scenario: Raw terminal confirmation happens after the complete command arrives

- [ ] RED: Remove only this scenario's `@wip`, bind explicit E2E stubs, and run
  only it through `verify.e2e_command`. Expected non-zero result. Evidence:
  ```text
  [paste targeted E2E output]
  ```
- [ ] GREEN: Drive `watn -x` in a real PTY, assert the generated command is
  visible and execution output absent before confirmation, send raw Enter, and
  assert generated and execution lines exactly once with status 0. Production
  files: [list every file]. Test files: [list every file]. Targeted result:
  ```text
  [paste targeted E2E output]
  ```
- [ ] REFACTOR: Keep raw-terminal confirmation distinct from piped stdin and
  preserve the complete-output-before-prompt invariant. Targeted rerun:
  ```text
  [paste targeted E2E output]
  ```
- [ ] COMMIT: `[commit hash]` - `test(e2e): Raw terminal confirmation happens after the complete command arrives`

## Scenario: Piped confirmation remains available after streamed output

- [ ] RED: Remove only this scenario's `@wip`, bind explicit E2E stubs, and run
  only it through `verify.e2e_command`. Expected non-zero result. Evidence:
  ```text
  [paste targeted E2E output]
  ```
- [ ] GREEN: Drive the real subprocess with piped `y` confirmation and assert
  the generated command line and execution output line separately, each exactly
  once, with status 0. Production files: [list every file]. Test files: [list
  every file]. Targeted result:
  ```text
  [paste targeted E2E output]
  ```
- [ ] REFACTOR: Remove duplicated launch/fixture code while preserving the
  stdout exact-once assertions. Targeted E2E rerun:
  ```text
  [paste targeted E2E output]
  ```
- [ ] COMMIT: `[commit hash]` - `test(e2e): Piped confirmation remains available after streamed output`

## Final Change Verification

- [ ] Remove all completed scenario `@wip` tags and run
  `givn lint --change incremental-sse-rendering`.
  ```text
  Result: [command output]
  ```
- [ ] Run `verify.command` and record its full scenario/step count and output.
  ```text
  Result: [command output]
  ```
- [ ] Run `verify.e2e_command` and record its full scenario/step count. Confirm
  it is a strict subset of `verify.command`.
  ```text
  Result: [command output and subset comparison]
  ```
- [ ] Run formatting, compilation, lint, unit, documentation, release-build,
  and diff checks appropriate to the repository, recording all results:
  ```text
  cargo fmt --all -- --check
  cargo check --all-targets
  cargo clippy --all-targets --all-features -- -D warnings
  cargo test --all-targets
  cargo test --doc
  cargo build --release
  git diff --check
  Result: [command output]
  ```
