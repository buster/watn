# Tasks: release-truth-and-repository-cleanup

## Setup

- [x] Register `tests/steps/release_truth_steps.rs` and
  `tests/steps/release_truth_e2e_steps.rs` as separate capability modules,
  register them in `tests/steps/mod.rs`, and add only required release state to
  `WatnWorld`. Confirm `.fail_on_skipped()` remains the strict runner setting.
  Prove strictness by activating one scenario against an explicit
  `unimplemented!()` stub and record the non-zero result:
  ```text
  verify.command: ./run-tests.sh
  verify.e2e_command: ./run-tests.sh --e2e
  strict proof command: root=$(mktemp -d /tmp/watn-release.XXXXXX) && trap 'rm -rf "$root"' EXIT && cargo build --bin watn && cp target/debug/watn "$root/default-debug" && cargo build --features test-support --bin watn && cp target/debug/watn "$root/test-support-debug" && WATN_DEFAULT_DEBUG_BIN="$root/default-debug" WATN_TEST_SUPPORT_DEBUG_BIN="$root/test-support-debug" cargo test --test features_runner --features test-support -- --name "Version flag reports the package version"
  Result: non-zero; `tests/steps/release_truth_steps.rs` matched the explicit `unimplemented!()` stub, the runner reported `1 step failed`, and Cargo returned `error: test failed`.
  ```
- [x] Run `givn lint --change release-truth-and-repository-cleanup` and record
  only the expected `@wip` findings.
  ```text
  Result: exit 2 with 1 file checked and 4 expected `@wip` findings; no structural findings.
  ```

## Non-E2E Scenarios

## Scenario: Release artifact reports target-dependent runtime libraries

- [x] RED: Remove only this scenario's `@wip`, bind explicit stubs, and run the
  named scenario. Expected non-zero result from a matched stub. Evidence:
  ```text
  Targeted command exited non-zero after matching `release_binary_stub`; the
  runner reported `1 step failed` and Cargo returned `error: test failed`.
  ```
- [x] GREEN: Build the release artifact and inspect it with `file` plus Linux
  `ldd` or macOS `otool -L`. Assert dynamic executable classification, successful
  host library inspection with at least one shared-library entry, and current
  deployment documentation. Production files: `docs/arc42/07-deployment-view.md`
  and related active Arc42/README documentation. Test files:
  `tests/steps/release_truth_steps.rs`. Targeted result:
  ```text
  1 feature, 1 scenario, 5 steps passed; dynamic executable, successful `ldd`
  output, and target-dependent documentation were asserted.
  ```
- [x] REFACTOR: Keep host selection and output parsing deterministic without
  changing the release contract. Targeted rerun:
  ```text
  Applied formatting and reran: 1 feature, 1 scenario, 5 steps passed.
  ```
- [x] COMMIT: `befc0f7` - `feat(release-truth-and-repository-cleanup): Release artifact reports target-dependent runtime libraries`

## Scenario: Active documentation describes current command streaming

- [x] RED: Remove only this scenario's `@wip`, bind explicit stubs, and run the
  scenario. Expected non-zero result. Evidence:
  ```text
  Targeted command exited non-zero after matching `active_docs_stub`; the
  runner reported `1 step failed` and Cargo returned `error: test failed`.
  ```
- [x] GREEN: Update README and all active Arc42 chapters containing stale
  streaming, reasoning, Ctrl-R, XDG, static/deferred-release, output-channel,
  and obsolete helper claims. Add absence assertions for each stale claim.
  Remove only obsolete names confirmed by repository search. Production/docs
  files: active README and Arc42 chapters. Test file:
  `tests/steps/release_truth_steps.rs`. Targeted result:
  ```text
  1 feature, 1 scenario, 11 steps passed; positive and stale-claim absence
  assertions all passed.
  ```
- [x] REFACTOR: Consolidate documentation assertions and preserve archived
  snapshot content. Targeted rerun:
  ```text
  Applied formatting and narrowed absence checks to stale positive claims;
  targeted rerun passed with 1 feature, 1 scenario, 11 steps.
  ```
- [x] COMMIT: `6f85e4f` - `docs(release-truth-and-repository-cleanup): Active documentation describes current command streaming`

## Scenario: Active documentation distinguishes archived historical snapshots

- [x] RED: Remove only this scenario's `@wip`, bind explicit stubs, and run the
  scenario. Expected non-zero result. Evidence:
  ```text
  Targeted command exited non-zero after matching `archive_docs_stub`; the
  runner reported `1 step failed` and Cargo returned `error: test failed`.
  ```
- [x] GREEN: Add an explicit archive-status section and historical links to the
  active Arc42 index. Assert archived assessments are identified as historical
  and are not presented as current architecture. Production files: none. Test
  file: `tests/steps/release_truth_steps.rs`. Targeted result:
  ```text
  1 feature, 1 scenario, 4 steps passed.
  ```
- [x] REFACTOR: Keep active/archive path assertions exact and idempotent.
  Targeted rerun:
  ```text
  Normalized documentation whitespace for stable assertions and reran: 1
  feature, 1 scenario, 4 steps passed.
  ```
- [x] COMMIT: `fe5cfa8` - `docs(release-truth-and-repository-cleanup): Active documentation distinguishes archived historical snapshots`

## Hygiene Verification

- [x] Confirm repository-wide searches before cleanup: remove `_config` from
  `build_registry`, retain public `ProviderRegistry` and setup result wrappers,
  remove only write-only `WatnWorld` fields, and remove obsolete helper names
  only when no active consumer remains. Compile and run the full feature suite
  after each cleanup group. Record every retained public item and why.
  ```text
  Search evidence: `ProviderRegistry`, `ProviderSetupResult`, and public setup
  wrappers still have current consumers or unknown external-consumer risk and
  were retained. `WatnWorld` fields removed as write-only: `config_content`,
  `executed_command`, `stdin_input`, and `last_request_body`. Obsolete active
  documentation names are absent. `_config` was removed from `build_registry`.
  `cargo check --all-targets`, clippy, and `./run-tests.sh` passed with 15
  features, 65 scenarios, and 364 steps.
  ```
- [x] COMMIT: `c911b7b` - `refactor(release-truth-and-repository-cleanup): remove confirmed dead repository code`

## E2E Setup

- [x] Confirm the local environment requires no external service and the E2E
  runner drives the explicit built release/debug binary through a real
  subprocess. Prove full and E2E scenario counts are distinct:
  ```text
  ./run-tests.sh: 15 features, 65 scenarios, 364 steps passed.
  ./run-tests.sh --e2e: 17 features, 57 scenarios, 385 steps passed.
  Result: E2E is a strict subset: yes (57 < 65). The local environment uses
  only the built CLI and repository-owned test doubles.
  ```

## E2E Scenario

## Scenario: Version flag reports the package version

- [x] RED: Remove only this scenario's `@wip`, bind E2E stubs using the unique
  release-binary wording, and run only it through `./run-tests.sh --e2e`.
  Expected non-zero result. Evidence:
  ```text
  Targeted E2E command exited non-zero after matching
  `run_release_version_stub`; the runner reported `1 step failed` and Cargo
  returned `error: test failed`.
  ```
- [x] GREEN: Replace stubs with a real subprocess invocation of the release
  binary using package version metadata. Assert stdout contains `watn`, the
  exact package version, and exit status 0. Production file: `src/main.rs`.
  Test files: `tests/steps/release_truth_e2e_steps.rs` and
  `tests/steps/release_truth_steps.rs`. Targeted E2E result:
  ```text
  1 feature, 1 scenario, 5 steps passed.
  ```
- [x] REFACTOR: Remove the obsolete hardcoded-version assertion and keep the
  package-version lookup authoritative. Targeted E2E rerun:
  ```text
  Applied rustfmt and reran: 1 feature, 1 scenario, 5 steps passed.
  ```
- [x] COMMIT: `[pending hash]` - `test(e2e): Version flag reports the package version`

## Final Change Verification

- [ ] Remove all completed scenario `@wip` tags and run
  `givn lint --change release-truth-and-repository-cleanup`.
  ```text
  Result: [output]
  ```
- [ ] Run `./run-tests.sh` and record the full scenario/step count.
  ```text
  Result: [output]
  ```
- [ ] Run `./run-tests.sh --e2e` and record the count, proving it is a strict
  subset of the full run.
  ```text
  Result: [output]
  ```
- [ ] Run `cargo fmt --all -- --check`, `cargo check --all-targets`,
  `cargo clippy --all-targets --all-features -- -D warnings`,
  `cargo test --all-targets` with explicit binary bootstrap, `cargo test --doc`,
  `cargo build --release`, `file target/release/watn`, host library inspection,
  and `git diff --check`.
  ```text
  Result: [output]
  ```
