# Tasks: Responsive Setup Model Filtering

## Setup

- [x] Confirm the configured Cucumber runner executes permanent specs and the
  active change spec, retains `.fail_on_skipped()`, and uses the exact commands
  from `givn/commands.yaml`.
  - Create `tests/steps/responsive_setup_model_filtering_steps.rs` and register
    it from `tests/steps/mod.rs`.
  - Use `unimplemented!()` for the first new assertion step and run the exact
    targeted command for `A complete catalog is filtered locally`.
   - Evidence: `tests/features_runner.rs` retains `.fail_on_skipped()`, the
     capability step module is registered, and the targeted bootstrap exited
     non-zero with `1 scenario (1 failed)`, `1 step (1 failed)`, and the
     captured `not implemented` panic.

## Scenario: A complete catalog is filtered locally

- [x] RED: Remove `@wip` from this scenario only, bind its new steps with
  `unimplemented!()`, and run the single-scenario command from `design.md`.
  Expected result: non-zero exit proving the local filtering behavior is not
  already passing.
   - Evidence: the exact targeted command compiled the default and
     `test-support` binaries, then exited non-zero with `1 scenario (1 failed)`
     and `1 step (1 failed)` at the explicit `unimplemented!()` Given step.

- [x] GREEN: Load a catalog identified as complete, keep the typed query
  visible, filter the cached entries locally, and assert that no provider
  search request is sent. Production files created or modified: list them
  here. Test file: `tests/steps/responsive_setup_model_filtering_steps.rs`.
  Run the targeted scenario and record a zero exit.
  - Evidence: `src/setup.rs` now classifies short catalog pages as complete,
    filters cached models locally, and renders `Filter: <query>` in the table
    title. Targeted runner passed `1 scenario` and `7 steps`.

- [x] REFACTOR: Keep local filtering and query rendering behavior unchanged,
  remove duplication in the catalog/request assertions, rerun the targeted
  scenario, and record a zero exit.
  - Evidence: `cargo fmt --all -- --check` passed and the targeted runner
    passed `1 scenario` and `7 steps` after the terminal-screen parser was
    formatted and reused by all assertions. Coverage is deferred to the
    configured full measurement command because `./run-tests.sh` emits no
    coverage report.

- [x] COMMIT: Create one atomic commit referencing `A complete catalog is filtered locally`.
- Commit hash: `6a62cb5`

## Scenario: A catalog requiring more data uses provider-backed filtering

- [x] RED: Remove `@wip` from this scenario only, bind its new provider-search
  assertions with `unimplemented!()`, and run the exact targeted command.
  Expected result: non-zero exit.
  - Evidence: the exact targeted command compiled the test runner and exited
    non-zero with `1 scenario (1 failed)` and `1 step (1 failed)` at the
    explicit `unimplemented!()` catalog Given step.

- [x] GREEN: Treat a full catalog page as incomplete, preserve the visible
  query, debounce the provider-backed search by 200 ms, and apply its matching
  result. Production files created or modified: list them here. Run the
  targeted scenario and record a zero exit.
  - Evidence: the remote incomplete-catalog path in `src/setup.rs` was reused
    from the first scenario; the new provider fixture and request assertion
    passed the targeted runner with `1 scenario` and `6 steps`.

- [x] REFACTOR: Preserve provider request behavior and current selection/status
  rendering, simplify the search-path assertions, rerun the targeted scenario,
  and record a zero exit.
  - Evidence: duplicated search-hit diagnostics were removed from the visible
    suggestion assertion; `cargo fmt --all -- --check` passed and the targeted
    runner passed `1 scenario` and `6 steps`.

- [x] COMMIT: Create one atomic commit referencing `A catalog requiring more data uses provider-backed filtering`.
- Commit hash: `c88d6fe`

## Scenario: A newer model query remains authoritative

- [x] RED: Remove `@wip` from this scenario only, bind the stale-result
  assertions with `unimplemented!()`, and run the exact targeted command.
  Expected result: non-zero exit.
  - Evidence: the exact targeted command compiled the runner and exited
    non-zero with `1 scenario (1 failed)` and `1 step (1 failed)` at the
    explicit `unimplemented!()` provider Given step.

- [x] GREEN: Retain remote search worker handles, reject stale generations
  before request/publish/apply, and join all workers when the wizard exits.
  Preserve current newest-result behavior and selection rules. Production files
  created or modified: list them here. Run the targeted scenario and record a
  zero exit.
  - Evidence: the retained worker and generation checks in `src/setup.rs` were
    exercised by delayed `gpt` and immediate `o3` provider twins. The follow-up
    compatibility fix also updated `src/models/list.rs` and the permanent
    paginated-catalog fixture so explicit `meta.has_more` prevents false local
    completeness. Targeted runner passed `1 scenario` and `4 steps`; the late
    `gpt` result did not replace the visible `o3` result.

- [x] REFACTOR: Keep generation ownership singular, reap finished handles, and
  preserve cancellation and save/discard behavior. Rerun the targeted scenario
  and record a zero exit.
  - Evidence: `cargo fmt --all -- --check` passed after formatting the delayed
    provider fixture and PTY replacement helper; the targeted runner passed
    `1 scenario` and `4 steps`.

- [x] COMMIT: Create one atomic commit referencing `A newer model query remains authoritative`.
- Commit hashes: `144adc5`, follow-up `7d9eb2c`

## E2E Setup

- [x] After all regular scenarios are GREEN, run the configured regular command
  and record its scenario/step count.
- [x] Run the configured E2E command with the `@e2e and not @wip` filter and
  record a strictly smaller scenario count.
- [x] Confirm the PTY starts the instrumented `watn setup` binary and the
  scenario-local `httpmock` model-provider twin starts and stops cleanly.
- [x] Confirm the E2E step file remains the capability-specific
  `tests/steps/responsive_setup_model_filtering_steps.rs` and strict mode still
  rejects undefined/pending steps.
- Evidence: `./run-tests.sh` passed `98 scenarios` and `570 steps`; the
  configured E2E command passed `66 scenarios` and `463 steps`, proving
  `66 < 98`. The final E2E scenario started a real `watn models` PTY child and
  an in-process `httpmock` twin cleanly. `.fail_on_skipped()` remains active;
  the capability step file is registered at the designed path.

## Scenario: The terminal model filter stays responsive during a delayed search

- [x] RED: Remove `@wip` from this scenario only, bind its PTY query/result
  assertions with `unimplemented!()`, and run the E2E command targeted by name.
  Expected result: non-zero exit.
  - Evidence: the targeted E2E bootstrap using `--name` exited non-zero with
    `1 scenario (1 failed)` and `1 step (1 failed)` at the explicit
    `unimplemented!()` Given step. The runner rejects combining `--tags` and
    `--name`, so the name-filtered command is the targeted E2E proof.

- [x] GREEN: Drive the real setup wizard through the PTY, assert the visible
  current query and matching row while the provider response is delayed, then
  assert that a subsequent filter change is accepted. Repository/request-count
  checks may support the terminal assertion but cannot replace it. Production
  files created or modified: list them here. Run the targeted E2E scenario and
  record a zero exit.
  - Evidence: `tests/steps/responsive_setup_model_filtering_steps.rs` drove the
    real `watn models` PTY against delayed `gpt` and immediate `o3` provider
    twins. The terminal retained `Filter: o3`, rendered `o3-pro`, and accepted
    a later `gpt` filter. Targeted E2E runner passed `1 scenario` and `9 steps`.

- [x] REFACTOR: Keep the PTY screen polling stable across incremental redraws,
  remove assertion duplication, rerun the targeted E2E scenario, and record a
  zero exit.
  - Evidence: the terminal-screen reconstruction helper was formatted and the
    unused mutable binding was removed; `cargo fmt --all -- --check` passed and
    the targeted E2E runner passed `1 scenario` and `9 steps`.

- [x] COMMIT: Create one atomic commit referencing `The terminal model filter stays responsive during a delayed search`.
- Commit hash: `2bd4627`

## Full Verification

- [x] Run `givn lint --change responsive-setup-model-filtering` with no findings
  other than none of the completed scenarios retaining `@wip`.
- [x] Run `./run-tests.sh` and `./run-tests.sh --e2e`; both pass with the E2E
  count strictly smaller than the regular count.
- [x] Run `./measure-coverage.sh` and `./merge-coverages.sh`; confirm the
  runner and instrumented PTY child are included and the merged report is
  fresh.
- [x] Run `cargo fmt --all -- --check`, `cargo check --locked`, and
  `git diff --check`.
- Evidence: `givn lint --change responsive-setup-model-filtering` was clean;
  regular verification passed `98 scenarios` and `570 steps`; E2E verification
  passed `66 scenarios` and `463 steps`; coverage measurement and merge passed
  with fresh reports; `cargo fmt --all -- --check`, `cargo check --locked`, and
  `git diff --check` passed.
