# Tasks: Isolate Test Transport

## Setup

- [x] Configure the no-default `test-support` feature and the four isolated
  debug/release binary paths from `design.md`; update `givn/commands.yaml` so
  verify and verify-e2e build those paths before invoking the Cucumber runner.
  Record the exact build and runner commands here:
  ```text
  cargo build --bin watn --target-dir <root>/default-debug
  cargo build --features test-support --bin watn --target-dir <root>/test-support-debug
  cargo build --release --bin watn --target-dir <root>/default-release
  cargo build --release --features test-support --bin watn --target-dir <root>/test-support-release
  WATN_DEFAULT_DEBUG_BIN=<root>/default-debug/debug/watn WATN_TEST_SUPPORT_DEBUG_BIN=<root>/test-support-debug/debug/watn WATN_DEFAULT_RELEASE_BIN=<root>/default-release/release/watn WATN_TEST_SUPPORT_RELEASE_BIN=<root>/test-support-release/release/watn cargo test --test features_runner --features test-support -- --tags 'not @wip and not @e2e'
  WATN_DEFAULT_DEBUG_BIN=<root>/default-debug/debug/watn WATN_TEST_SUPPORT_DEBUG_BIN=<root>/test-support-debug/debug/watn WATN_DEFAULT_RELEASE_BIN=<root>/default-release/release/watn WATN_TEST_SUPPORT_RELEASE_BIN=<root>/test-support-release/release/watn cargo test --test features_runner --features test-support -- --tags '@e2e and not @wip'
  ```
- [x] Register `tests/steps/transport_steps.rs` as the transport capability
  step-definition module. New steps must use `unimplemented!()` until their
  scenario enters GREEN; no empty bodies are allowed.
- [x] Prove strict mode by removing `@wip` from one transport scenario,
  running its single-scenario command against the stub steps, and recording a
  non-zero result from `.fail_on_skipped()`/the panic. Restore `@wip` after the
  proof.
  ```text
  cargo test --test features_runner --features test-support -- --name "Provider readiness ignores the test routing setting"
  Result: non-zero; the first matched stub panicked with `not implemented` and
  the runner reported `1 step failed`.
  ```
- [x] Run `givn lint --change isolate-test-transport` and confirm that only
  expected `@wip` findings remain.
  ```text
  givn lint: 1 file(s) checked, 3 finding(s); all three findings are the
  expected @wip transport E2E scenarios.
  ```

## Scenario: Provider readiness ignores the test routing setting

- [x] RED: Remove `@wip` from this scenario, add the non-E2E step bindings with
  real `unimplemented!()` stubs, and run only:
  ```text
  WATN_DEFAULT_DEBUG_BIN=... WATN_TEST_SUPPORT_DEBUG_BIN=... WATN_DEFAULT_RELEASE_BIN=... WATN_TEST_SUPPORT_RELEASE_BIN=... cargo test --test features_runner --features test-support -- --name "Provider readiness ignores the test routing setting"
  ```
  Expected result: non-zero exit. Evidence: the runner matched the stub and
  exited non-zero with `Step panicked ... not implemented` and `1 step failed`.
- [x] GREEN: Implement the readiness setup and assertions. Confirm readiness
  uses the configured provider record, ignores the competing route, starts no
  HTTP request, and preserves the configured endpoint. Production files:
  `src/provider/transport.rs` only if the transport boundary is needed by the
  assertion; otherwise investigate before claiming GREEN. Test files:
  `tests/steps/transport_steps.rs` and shared world state as required. The
  transport boundary was implemented in `src/provider/transport.rs`; the
  readiness scenario uses the existing provider readiness path. Targeted run:
  1 scenario, 6 steps passed.
- [x] REFACTOR: Remove duplication in the transport fixture/assertion helpers
  without changing behavior. The targeted rerun passed with 1 scenario and 6
  steps.
- [x] COMMIT: Commit RED/GREEN/REFACTOR atomically with a message containing
  `Provider readiness ignores the test routing setting`. Commit hash: `e0dd980`.

## Scenario: Normal release requests ignore test routing settings

- [x] RED: Remove `@wip` from this scenario, bind every step with real stubs,
  and run only the scenario through the e2e command. Expected result: non-zero
  exit. Evidence: the runner matched the `run_release_binaries` stub and
  exited non-zero with `Step panicked ... not implemented` and `1 step failed`.
- [x] GREEN: Build and select the explicit default-feature release and
  test-support release binaries. Implement separate configured and competing
  loopback twins. Assert both release binaries use the configured full URL,
  method/path, exact Authorization header, response source, request counts,
  and unchanged persisted endpoint. Production files: `Cargo.toml` and
  `src/provider/transport.rs`. Test files: `givn/commands.yaml`,
  `tests/features_runner.rs`, `tests/steps/transport_steps.rs`, and any
  concrete shared test-state file required by the reviewed design. Run only
  this scenario through verify-e2e and record passing output. Evidence: the
  explicit four-binary build matrix completed; targeted run passed with 1
  scenario and 10 steps.
- [x] REFACTOR: Make binary-path and server cleanup deterministic, remove
  catch-all mock matchers, and rerun this scenario through verify-e2e. The
  formatted targeted rerun passed with 1 scenario and 10 steps.
- [x] COMMIT: Commit RED/GREEN/REFACTOR atomically with a message containing
  `Normal release requests ignore test routing settings`. Commit hash:
  `0554516`.

## Scenario: Test-support requests use isolated routing without changing saved configuration

- [x] RED: Remove `@wip` from this scenario, bind every step with real stubs,
  and run only the scenario through verify-e2e. Expected result: non-zero exit.
  Evidence: the runner matched the `run_isolated_debug_binary` stub and exited
  non-zero with `Step panicked ... not implemented` and `1 step failed`.
- [x] GREEN: Implement the debug test-support invocation against the isolated
  loopback twin. Assert isolated response, exact URL and path, exact
  Authorization header, one isolated hit, zero configured hits, unchanged
  configured endpoint, and absence of the isolated endpoint from TOML. Run
  only this scenario through verify-e2e and record passing output. Evidence:
  explicit four-binary build matrix completed; targeted run passed with 1
  scenario and 11 steps.
- [x] REFACTOR: Consolidate shared transport state and preserve the primary
  CLI-output assertion. The cleanup-enabled targeted rerun passed with 1
  scenario and 11 steps.
- [x] COMMIT: Commit RED/GREEN/REFACTOR atomically with a message containing
  `Test-support requests use isolated routing without changing saved configuration`.
  Commit hash: pending until commit creation.

## Scenario: Missing or whitespace test overrides fall back to the configured provider

- [ ] RED: Remove `@wip` from this scenario outline, bind the outline steps
  with real stubs, and run only the outline through verify-e2e. Expected result:
  non-zero exit. Evidence: pending.
- [ ] GREEN: Implement both `missing` and `whitespace` child environments.
  Assert configured response, exact configured URL and path, exact
  Authorization header, one configured hit, zero competing hits, and unchanged
  TOML for both examples. Run only the outline through verify-e2e and record
  passing output. Evidence: pending.
- [ ] REFACTOR: Remove duplicated fallback setup while keeping both examples
  independently observable. Rerun the outline through verify-e2e. Evidence:
  pending.
- [ ] COMMIT: Commit RED/GREEN/REFACTOR atomically with a message containing
  `Missing or whitespace test overrides fall back to the configured provider`.
  Commit hash: pending.

## Final Change Verification

- [ ] Remove all completed scenario `@wip` tags and run `givn lint --change isolate-test-transport`.
- [ ] Run verify.command and record its scenario count and successful output.
- [ ] Run verify.e2e_command and record its scenario count; prove it is a strict
  subset of verify.command.
- [ ] Run `cargo fmt --all -- --check`, `cargo check --all-targets`,
  `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --all-targets`,
  `cargo test --doc`, `cargo build --release`, and `git diff --check`.
- [ ] Confirm default-feature and test-support release binaries both ignore the
  override, and record the exact commands and results.
