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
    exercised by delayed `gpt` and immediate `o3` provider twins. Targeted
    runner passed `1 scenario` and `4 steps`; the late `gpt` result did not
    replace the visible `o3` result.

- [x] REFACTOR: Keep generation ownership singular, reap finished handles, and
  preserve cancellation and save/discard behavior. Rerun the targeted scenario
  and record a zero exit.
  - Evidence: `cargo fmt --all -- --check` passed after formatting the delayed
    provider fixture and PTY replacement helper; the targeted runner passed
    `1 scenario` and `4 steps`.

- [x] COMMIT: Create one atomic commit referencing `A newer model query remains authoritative`.
- Commit hash: `144adc5`

## E2E Setup

- [ ] After all regular scenarios are GREEN, run the configured regular command
  and record its scenario/step count.
- [ ] Run the configured E2E command with the `@e2e and not @wip` filter and
  record a strictly smaller scenario count.
- [ ] Confirm the PTY starts the instrumented `watn setup` binary and the
  scenario-local `httpmock` model-provider twin starts and stops cleanly.
- [ ] Confirm the E2E step file remains the capability-specific
  `tests/steps/responsive_setup_model_filtering_steps.rs` and strict mode still
  rejects undefined/pending steps.
- Evidence: paste both runner summaries and the local-infrastructure result
  here.

## Scenario: The terminal model filter stays responsive during a delayed search

- [ ] RED: Remove `@wip` from this scenario only, bind its PTY query/result
  assertions with `unimplemented!()`, and run the E2E command targeted by name.
  Expected result: non-zero exit.
  - Evidence: paste the targeted E2E output here.

- [ ] GREEN: Drive the real setup wizard through the PTY, assert the visible
  current query and matching row while the provider response is delayed, then
  assert that a subsequent filter change is accepted. Repository/request-count
  checks may support the terminal assertion but cannot replace it. Production
  files created or modified: list them here. Run the targeted E2E scenario and
  record a zero exit.
  - Evidence: paste the targeted E2E output here.

- [ ] REFACTOR: Keep the PTY screen polling stable across incremental redraws,
  remove assertion duplication, rerun the targeted E2E scenario, and record a
  zero exit.
  - Evidence: paste the targeted E2E output and formatting result here.

- [ ] COMMIT: Create one atomic commit referencing `The terminal model filter stays responsive during a delayed search`.
- Commit hash: pending

## Full Verification

- [ ] Run `givn lint --change responsive-setup-model-filtering` with no findings
  other than none of the completed scenarios retaining `@wip`.
- [ ] Run `./run-tests.sh` and `./run-tests.sh --e2e`; both pass with the E2E
  count strictly smaller than the regular count.
- [ ] Run `./measure-coverage.sh` and `./merge-coverages.sh`; confirm the
  runner and instrumented PTY child are included and the merged report is
  fresh.
- [ ] Run `cargo fmt --all -- --check`, `cargo check --locked`, and
  `git diff --check`.
- Evidence: paste final command summaries here.
