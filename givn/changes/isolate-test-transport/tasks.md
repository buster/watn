# Tasks: Isolate Test Transport

## Setup

- [x] Configure the no-default `test-support` feature and the two explicit
  debug binary copies from `design.md`; update `givn/commands.yaml` so verify
  and verify-e2e reuse Cargo's shared target cache and copy both debug binaries
  before invoking the Cucumber runner.
  Record the exact build and runner commands here:
  ```text
  root=$(mktemp -d /tmp/watn-transport.XXXXXX) && trap 'rm -rf "$root"' EXIT && cargo build --bin watn && cp target/debug/watn "$root/default-debug" && cargo build --features test-support --bin watn && cp target/debug/watn "$root/test-support-debug" && WATN_DEFAULT_DEBUG_BIN="$root/default-debug" WATN_TEST_SUPPORT_DEBUG_BIN="$root/test-support-debug" cargo test --test features_runner --features test-support -- --tags 'not @wip and not @e2e'
  root=$(mktemp -d /tmp/watn-transport.XXXXXX) && trap 'rm -rf "$root"' EXIT && cargo build --bin watn && cp target/debug/watn "$root/default-debug" && cargo build --features test-support --bin watn && cp target/debug/watn "$root/test-support-debug" && WATN_DEFAULT_DEBUG_BIN="$root/default-debug" WATN_TEST_SUPPORT_DEBUG_BIN="$root/test-support-debug" cargo test --test features_runner --features test-support -- --tags '@e2e and not @wip'
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
- [x] Run `givn lint --change isolate-test-transport` and confirm the corrected
  feature file is clean.
  ```text
  givn lint --change isolate-test-transport: exit 0; 1 file checked, 0 findings.
  ```

## Scenario: Provider readiness ignores the test routing setting

- [x] RED: Remove `@wip` from this scenario, add the non-E2E step bindings with
  real `unimplemented!()` stubs, and run only:
  ```text
  WATN_DEFAULT_DEBUG_BIN=... WATN_TEST_SUPPORT_DEBUG_BIN=... cargo test --test features_runner --features test-support -- --name "Provider readiness ignores the test routing setting"
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
  `Provider readiness ignores the test routing setting`. Commit hash: `3009286`.

## Scenario: Normal debug requests ignore test routing settings

- [x] RED: Remove `@wip` from this scenario, bind every step with real stubs,
  and run only the scenario through the e2e command. Expected result: non-zero
  exit. Evidence: the runner matched the `run_default_debug_binary` stub and
  exited non-zero with `Step panicked ... not implemented` and `1 step failed`.
- [x] GREEN: Build and select the explicit default-feature debug copy using the
  shared Cargo target cache. Implement separate configured and competing
  loopback twins. Assert the debug binary uses the configured full URL,
  method/path, exact Authorization header, response source, request counts,
  and unchanged persisted endpoint. Production files: `Cargo.toml` and
  `src/provider/transport.rs`. Test files: `givn/commands.yaml`,
  `tests/features_runner.rs`, `tests/steps/transport_steps.rs`, and any
  concrete shared test-state file required by the reviewed design. Run only
  this scenario through verify-e2e and record passing output. Evidence: the
  shared-cache two-build bootstrap completed; targeted run passed with 1
  scenario and 10 steps.
- [x] REFACTOR: Make binary-copy and server cleanup deterministic, remove
  catch-all mock matchers, and rerun this scenario through verify-e2e. The
  formatted shared-cache rerun passed with 1 scenario and 10 steps.
- [x] COMMIT: Commit RED/GREEN/REFACTOR atomically with a message containing
  `Normal debug requests ignore test routing settings`. Commit hash: `f78ed9e`.
  The earlier release-scoped implementation commit `0554516` is superseded by
  this debug-scope correction.

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
  Commit hash: `1bce76b`.

## Scenario: Missing or whitespace test overrides fall back to the configured provider

- [x] RED: Remove `@wip` from this scenario, bind the two-invocation steps with
  real stubs, and run only the scenario through verify-e2e. Expected result:
  non-zero exit. Evidence: the runner matched the fallback action stub and
  exited non-zero with `Step panicked ... not implemented` and `1 step failed`.
- [x] GREEN: Implement both `missing` and `whitespace` child environments.
  Assert both configured responses, exact configured URL and path, exact
  Authorization header, two configured hits, zero competing hits, and unchanged
  TOML. Run only the scenario through verify-e2e and record passing output.
  Efficient two-build bootstrap passed with 1 scenario and 10 steps.
- [x] REFACTOR: Remove duplicated fallback setup while keeping both child
  invocations independently observable. The formatted efficient-bootstrap
  rerun passed with 1 scenario and 10 steps.
- [x] COMMIT: Commit RED/GREEN/REFACTOR atomically with a message containing
  `Missing or whitespace test overrides fall back to the configured provider`.
  Commit hash: `686df8c`.

## Final Change Verification

- [x] Remove all completed scenario `@wip` tags and run `givn lint --change isolate-test-transport`.
  Result: exit 0; 1 file checked, 0 findings.
- [x] Run verify.command and record its scenario count and successful output.
  Result: shared-cache two-build bootstrap; 9 features, 44 scenarios, 240
  steps passed.
- [x] Run verify.e2e_command and record its scenario count; prove it is a strict
  subset of verify.command. Result: shared-cache two-build bootstrap; 11
  features, 42 scenarios, 267 steps passed. The E2E count is strictly below
  the 44-scenario verify count.
- [x] Run `cargo fmt --all -- --check`, `cargo check --all-targets`,
  `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --all-targets`,
  `cargo test --doc`, `cargo build --release`, and `git diff --check`.
  Results: applied repository-wide rustfmt; `cargo fmt --all -- --check`,
  `cargo check --all-targets`, and `cargo clippy --all-targets --all-features
  -- -D warnings` pass. The shared-cache bootstrap with explicit default and
  test-support debug binaries and `cargo test --all-targets --features
  test-support` passed 15 unit tests, 86 scenarios, and 507 steps. `cargo test
  --doc` (0 doc tests), `cargo build --release`, and `git diff --check` pass.
  Coverage hooks use the historical `cargo llvm-cov run`/`test` flow with
  explicit debug copies and serialized library tests; non-E2E coverage is
  47.20% line coverage and E2E coverage is 77.76% line coverage. Both reports
  include 0/0 branch counters.
  Verification evidence commit: `092b130`; coverage-command correction is
  recorded in the review follow-up commit.
- [x] Record that release-profile override verification is deferred to
  `release-truth-and-repository-cleanup`; the source guard remains compiled and
  the later change owns the release smoke test.
