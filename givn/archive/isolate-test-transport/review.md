# Review: isolate-test-transport

## Fabrication Audit

### Tag Integrity

The delta feature contains exactly three `@e2e` scenarios and no `@wip` tags.
The tag history was checked across the implementation commits:

| Commit | Delta tag state |
|---|---|
| `3009286` | All three scenarios had `@e2e` and `@wip` during RED. |
| `f78ed9e` | The first two scenarios retained `@e2e`; their `@wip` tags were removed. |
| `1bce76b` | The first two retained `@e2e`; the fallback scenario still had `@wip`. |
| `686df8c` | All three retained `@e2e`; no `@wip` tags remained. |

No `@e2e` tag was removed.

### Step-Body Audit

All ten files under `tests/steps/` were scanned for `unimplemented!()`,
`todo!()`, empty bodies, bare no-op returns, and bodies without a real
assertion or domain action. Zero empty or stub step bodies were found. The
runner uses `.fail_on_skipped()` in `tests/features_runner.rs`. The setup task
records a targeted non-zero run against an `unimplemented!()` step.

### Task And Commit Audit

Every checked scenario has a commit hash and implementation traceability:

- `3009286` adds the feature boundary, endpoint resolver, transport state, and
  readiness scenario production path.
- `f78ed9e` completes the normal-debug subprocess fixture and assertions; its
  production boundary is the `src/provider/transport.rs` implementation from
  `3009286`.
- `1bce76b` adds the test-support endpoint branch in production and its
  isolated-routing assertions.
- `686df8c` completes the missing/whitespace fallback fixture and assertions;
  it exercises the production fallback branch from `1bce76b`.
- `092b130` records the final Rust formatting and compile/test verification.
- `6d976e6` records the corrected reproducible coverage hooks and Cobertura
  reports.

No checked task is represented only by a feature file or an empty step stub.

### Promised Components And Design Conformance

Every component named by `design.md` exists and is used:

| Design promise | Result |
|---|---|
| No-default `test-support` Cargo feature | Present in `Cargo.toml`. |
| Compile-time debug-only endpoint override | Implemented in `src/provider/transport.rs` with `cfg(all(feature = "test-support", debug_assertions))`. |
| Pure URL builders after endpoint resolution | Existing model and chat URL builders receive the resolved endpoint; the environment lookup occurs only at the transport boundary. |
| Explicit default/test-support debug binary matrix | Implemented in `givn/commands.yaml` and `tests/steps/mod.rs`. |
| Concrete transport-specific state | `TransportState` is owned by `tests/steps/transport_steps.rs` and initialized in `WatnWorld`. |
| Separate loopback provider twins and exact mocks | Implemented with per-scenario `httpmock::MockServer` instances and method/path/header matchers. |
| CLI subprocess interface | Transport scenarios use `std::process::Command` and explicit binary paths. |

The implementation matches the reviewed Cucumber-rs, blocking HTTP,
httpmock, and CLI subprocess decisions. The regular and E2E verify commands
match the filtered commands in `givn/commands.yaml`.

### Real-Interface E2E Audit

All three delta E2E scenarios drive compiled `watn` subprocesses. Their primary
assertions inspect CLI output and process status; exact loopback request URL,
method, path, Authorization, hit counts, and persisted TOML are additional
transport assertions. No E2E Then step asserts only on repository state. This
is a CLI capability, not a browser capability, so no browser driver or HTTP
shortcut is applicable.

The exact E2E command invokes `tests/features_runner.rs` with the
`@e2e and not @wip` filter. The tree contains no second implementation of the
three transport step bindings; all matching bindings are in
`tests/steps/transport_steps.rs`.

### Command Isolation And Local Runnability

The configured commands are distinct:

- Regular: the shared-cache bootstrap followed by the `not @wip and not @e2e` filter; 9 features, 44 scenarios, and 240 steps passed.
- E2E: the same bootstrap followed by the `@e2e and not @wip` filter; 11 features, 42 scenarios, and 267 steps passed.

The E2E count is strictly smaller than the regular count. The local run uses
the Cucumber runner, real CLI subprocesses, and per-scenario loopback
`httpmock::MockServer` twins. No database, container, live provider, or shared
external service is required. The configured commands start and clean up the
complete local test stack themselves.

### Interaction Coverage Cross-Reference

The feature inventory has three entries. The design Interaction Coverage Matrix
has three rows. Each row maps to exactly one delta `@e2e` scenario and the
promised CLI/httpmock driving mechanism:

| Inventory entry | Matrix scenario | Step implementation | Driver verified |
|---|---|---|---|
| Run a normal debug request with a non-empty test routing setting | Normal debug requests ignore test routing settings | `transport_steps.rs` runs `WATN_DEFAULT_DEBUG_BIN` and asserts configured output, exact route, count, Authorization, and TOML | `std::process::Command` plus separate loopback `httpmock` twins |
| Run a test-support request through an isolated provider twin | Test-support requests use isolated routing without changing saved configuration | `transport_steps.rs` runs `WATN_TEST_SUPPORT_DEBUG_BIN` and asserts isolated output, exact route, count, Authorization, and persisted config | `std::process::Command` plus isolated/configured loopback twins |
| Run test-support requests with missing and whitespace overrides and fall back | Missing or whitespace test overrides fall back to the configured provider | `transport_steps.rs` runs the explicit test-support binary twice and asserts both outputs, aggregate counts, exact route, Authorization, and TOML | Two explicit `std::process::Command` invocations plus loopback twins |

The readiness scenario is intentionally non-E2E because it is a local
configuration predicate and not a user interaction. It asserts zero requests
and configured endpoint preservation through the public readiness API.

## Coverage Measurement

The coverage hooks preserve the working historical `cargo llvm-cov run` and
`cargo llvm-cov test` flow from `f72b193`, while adding the explicit debug copy
paths required by this change. Both child copies are generated by
`cargo llvm-cov run`, so the CLI subprocesses are instrumented. The library
tests run with `--test-threads=1` because the endpoint unit tests temporarily
modify process-global environment state. The Cucumber runner is instrumented
by the final `cargo llvm-cov test` invocation.

Measured Cobertura outputs from the final source and profiles:

| Report | Covered lines | Valid lines | Line rate | Branch counters |
|---|---:|---:|---:|---|
| Non-E2E | 1095 | 2320 | 47.20% | 0/0 |
| E2E | 1804 | 2320 | 77.76% | 0/0 |

The final `cargo llvm-cov report --summary-only` aggregate reported 1847 of
2420 lines, or 76.32%. The transport boundary itself reported 13/14 lines in
the non-E2E report and 27/28 lines in the E2E report. The Cobertura export has
zero valid branch counters; branch coverage is recorded as 0/0 rather than
inferred.

### Coverage Classification

- **Dead code:** None in the changed transport boundary or transport step
  implementation.
- **Missing test coverage:** None for the requested transport behavior. The
  default debug boundary, isolated test-support routing, missing override,
  whitespace override, readiness predicate, exact request contract, and
  persisted configuration are all exercised by the delta scenarios or the
  transport unit tests.
- **Legitimately hard to test:** The release-only compile-time branch is not
  reachable from the debug-instrumented coverage processes because
  `debug_assertions` is false only in the release profile. It is compiled by
  `cargo build --release`; a separate release-profile smoke check is explicitly
  owned by `release-truth-and-repository-cleanup`. The remaining uncovered
  regions are unrelated terminal/error and process-handoff failure paths with
  no stable production fault-injection seam.

## Verification

```text
givn lint --change isolate-test-transport
exit 0; 1 file checked, 0 findings

verify.command
9 features, 44 scenarios, 240 steps passed

verify.e2e_command
11 features, 42 scenarios, 267 steps passed

cargo fmt --all -- --check
passed

cargo check --all-targets
passed

cargo clippy --all-targets --all-features -- -D warnings
passed

bootstrapped cargo test --all-targets --features test-support
15 unit tests, 86 scenarios, 507 steps passed

cargo test --doc
0 doc tests passed

cargo build --release
passed

git diff --check
passed

coverage.non_e2e_command and coverage.e2e_command
Cobertura reports generated successfully with measured line coverage
```

## Sign-off Checklist

| Check | Status |
|---|---|
| Fabrication audit clean | PASS |
| Every checked task has verified production traceability | PASS |
| Promised components exist and conform | PASS |
| Strict-mode proof present and non-zero | PASS |
| `verify.command` exits 0 | PASS |
| `verify.e2e_command` exits 0 | PASS |
| E2E count is isolated and smaller | PASS: 42 E2E vs 44 regular scenarios |
| Coverage includes library tests, runner, and spawned CLI | PASS |
| Coverage classified using the three required buckets | PASS |
| No WIP tags remain | PASS |
| E2E scenarios use real CLI interaction and primary interface assertions | PASS |
| Exactly one E2E scenario per inventory action | PASS |
| Local loopback twin stack starts cleanly | PASS |
| Implementation matches the reviewed design | PASS |
| Inventory, matrix, scenarios, and drivers cross-reference cleanly | PASS |

REVIEW: PASS
