# Design: cancel-running-completion

## Scope And Decisions

- The provider (`reqwest::blocking`) exposes a single timeout value that
  governs BOTH the connect/header phase (`execute_request`) and the per-read
  deadline (`Response::read`). reqwest 0.12.28 (verified in Cargo.lock) has
  no separate `read_timeout` on the blocking builder. Therefore a per-read
  polling loop cannot be layered on the blocking response body, and the
  interrupt check must live at the parse layer (between SSE lines) plus a
  hard fallback for phases that are not checkable.
- Run the streaming call on a dedicated worker thread. The main thread polls
  for worker completion and for the SIGINT flag. This makes every phase
  cancellable:
  - **Stream flowing**: `parse_sse_stream` checks the flag at each loop
    iteration and returns `Error::Interrupted` on the next SSE line.
  - **Stream stalled / connect pending** (`send()` or an idle `read_line`):
    the worker cannot be reached, so the main thread waits up to a 500 ms
    grace period, then detaches the worker and exits 130.
- A single Ctrl+C suffices in all phases; no double-press escalation is
  required because termination is bounded by the 500 ms grace.
- The `ctrlc` handler remains flag-only (`AtomicBool`), matching the current
  design; the change is that the flag is now acted upon during the run.
- Keep the blocking HTTP stack, the SSE parser, the callback contract, and
  the existing `Error` path. No new runtime dependencies.
- On the **join path** (worker returned normally or with `Error::Interrupted`)
  the interrupted result runs the existing error cleanup (`spinner.finish()`,
  `output.finish_partial()` when content exists), suppresses the error line,
  and exits 130; already-streamed stdout content stays.
- On the **grace/detach path** the worker cannot be joined; the main thread
  exits 130 directly without spinner/output cleanup. This is documented as a
  hard-exit cost and is covered by `R-056`/`R-057`.

## Architecture Impact

### Error surface

Add `Interrupted` variant to `Error` (in `src/error.rs`). It is not a user
reported error: exit code 130 and no message. Display arm returns
"interrupted" (unused in practice because main handles `Interrupted` before
printing). Add `Error::Interrupted => 130` to the exhaustive `exit_code()`
match.

### Provider

`src/provider/openai_compat.rs`:

- Add field `interrupt: Arc<AtomicBool>` to `OpenAICompatibleProvider`.
- Constructor becomes `new(endpoint, api_key, interrupt)`.
- `parse_sse_stream` gains an `interrupt: &AtomicBool` parameter. At the top
  of every loop iteration and when mapping a read error, if the flag is set,
  return `Error::Interrupted`.
- `chat_completions_streaming` passes `&self.interrupt` into the parser.
- The `Provider` trait signature is unchanged.

### CLI

`src/main.rs`:

- Create the `Arc<AtomicBool>` before `build_registry`; pass a clone to
  `build_registry`/`OpenAICompatibleProvider::new` and to the `ctrlc`
  handler.
- Keep `spinner` and `output` owned in `main`; move them INTO the worker
  closure along with `registry`, `messages`, `options`, and an owned copy of
  `provider_name`. The closure signature returns the tuple
  `(Result<StreamingResponse, Error>, Option<Spinner>, StreamRenderer<io::Stdout>)`
  via the `JoinHandle`, so `main` regains them for the existing success/error
  handling (spinner finish, `output.complete()`, `finish_partial()`, metadata,
  `-x` prompt).
- A `wait_for_stream_result(handle, interrupt)` helper returns the joined
  tuple:
  - polls `handle.is_finished()` every ~20 ms; on finish, `join()` and return.
  - if the flag is set and the worker is not finished, keep polling
    `is_finished()` up to a 500 ms grace so a worker that returns
    `Error::Interrupted` inside the window is joined, not detached;
  - on grace expiry, `drop(handle)` (detach, no join) and
    `std::process::exit(130)`.
- The `Err` arm of `match stream_result`:
  - finish the spinner and partial output on the join path as today;
  - if the error is `Error::Interrupted`, `std::process::exit(130)`;
  - otherwise print and exit with the mapped code as today.
- Keep the existing end-of-main interrupt check.

### Grace constant and polling

`GRACE = Duration::from_millis(500)`, poll intervals of ~20 ms (outer) and
~10 ms (grace). `JoinHandle::is_finished()` is stable; dropping a
`JoinHandle` detaches without blocking.

## Threading And Locks

- `spinner` and `output` are moved into the worker closure only for the
  duration of streaming (bounded by a block that drops the emit closure), then
  returned via the join tuple. All stdout writes happen inside the worker.
  Cleanup (`finish`/`finish_partial`/`complete`) runs on `main` after the
  worker has been joined.
- The grace/detach path never joins, so no value is reclaimed: `main` exits
  130 directly, skipping spinner/output cleanup. This is the documented
  hard-exit cost.
- A detached worker may still run briefly while `main` exits; stdout writes
  are internally locked, so this is safe.
- The spinner thread is unchanged (stderr writes, cleared on `finish`).

## Test Runner Configuration

- `verify.command`: `./run-tests.sh`
- `verify.e2e_command`: `./run-tests.sh --e2e`
- Single-scenario E2E commands (both scenarios are `@e2e`; the non-E2E
  command does not apply). The cucumber runner's `--name` filter conflicts
  with `--tags`, and `run-tests.sh` ignores trailing args, so a targeted
  run invokes the harness directly with `--name <regex>` (the `+` in the
  scenario title must be escaped):
  - `One Ctrl\+C cancels a completion waiting for streamed output`
  - `One Ctrl\+C cancels a completion waiting for a connection`
  Both need the two binaries built and env set as in `run-tests.sh`:
  ```sh
  root=$(mktemp -d /tmp/watn-cancel.XXXXXX)
  cargo build --bin watn && cp target/debug/watn "$root/default-debug"
  cargo build --features test-support --bin watn && cp target/debug/watn "$root/test-support-debug"
  WATN_DEFAULT_DEBUG_BIN="$root/default-debug" WATN_TEST_SUPPORT_DEBUG_BIN="$root/test-support-debug" \
    cargo test --test features_runner --features test-support -- --name "One Ctrl\\+C cancels a completion waiting for streamed output"
  ```
- Strict mode: `tests/features_runner.rs` uses `.fail_on_skipped()`; RED
  bodies use explicit `unimplemented!()`.
- The full `verify.e2e_command` gate passes only the e2e tag filter; the
  `--name` targeted runs above are used per-scenario during the TDD loop and
  are a subset of it.

## E2E Infrastructure

- The Cucumber process starts loopback servers; nothing external is required.
- **Capability step files** (one per capability):
  - `tests/steps/cancel_completion_steps.rs` — all steps for this change,
    registered in `tests/features_runner.rs`.
  - Reuses, without modifying: `tests/steps/mod.rs` (PTY session helpers,
    binary bootstrap), the release-gated streaming twin
    `StreamingServer::start_with_initial_delay` with `hold_after` in
    `tests/steps/incremental_sse_rendering_steps.rs`, and its `update_config`
    config wiring (provider name `streaming`).
- **New fixture**: a black-hole listener that accepts one TCP connection,
  reads the request headers, and never answers; released/joined on drop.
  Lives in `tests/steps/cancel_completion_steps.rs`.
- Ctrl+C is delivered as the byte `\x03` through the PTY master; the kernel
  line discipline converts it to SIGINT for the `watn` child (the ask flow
  never enables raw mode, so ISIG stays on). Smoke-test this delivery first;
  if `\x03` does not raise SIGINT, fall back to sending SIGINT with
  `kill(PID, SIGINT)` (nix is already a dev-dependency).
- Assertions after exit target the merged PTY output (`world.output`), which
  `finish_pty_session` populates along with `world.exit_status`; it sets
  `world.stderr_output` empty, so "stderr should not contain …" steps must
  assert against `world.output`.

## Interaction Coverage Matrix

| Inventory entry (from the `.feature`) | Scenario | Driving mechanism |
|---|---|---|
| press Ctrl+C once while a completion is streaming response content | One Ctrl+C cancels a completion waiting for streamed output | Real `watn` subprocess in a PTY; `\x03` delivered through the PTY master; held-open streaming twin |
| press Ctrl+C once while the connection is still being established | One Ctrl+C cancels a completion waiting for a connection | Real `watn` subprocess in a PTY; `\x03` delivered through the PTY master; black-hole listener |

## Scenario Step Binding Plan

Both scenarios are `@e2e`; all steps below are new in
`tests/steps/cancel_completion_steps.rs` unless noted:

1. `a streaming provider flushes content "printf first" and holds the stream open without [DONE]` — start `StreamingServer` with one content event held after index 0, no `[DONE]`; write the `[providers.streaming]` raw config via `update_config`-style wiring.
2. `a provider accepts a connection and never sends a response` — start the black-hole listener; write the same streaming raw config.
3. `I start watn with the invocation `watn "output first"` in a terminal` — delegate to `start_pty_session`.
4. `the first streamed content "printf first" is visible` — poll `pty_snapshot` until the fragment appears.
5. `the progress indicator is visible while the connection is pending` — wait for the `Asking` spinner text.
6. `I press Ctrl+C` — write `\x03` to the session writer, then call `finish_pty_session` (sets `world.exit_status`, `world.output`).
7. `the exit status should be 130` — reuse the existing parameterized step (`world.exit_status == Some(130)`).
8. `the terminal output contains "printf first"` — reuse the existing step in `incremental_sse_rendering_e2e_steps.rs`.
9. `stderr should not contain a reported error` / `stderr should not contain final metadata` — assert absence of error strings and of `tok/s` in `world.output` (merged PTY), matching how existing e2e metadata assertions work.