# Tasks: persist-review-panel-setting

All tasks are intentionally unchecked. One atomic commit per scenario.

## Setup

- [x] Confirm the runner and strict mode for this change. Add one temporary
  step binding using `unimplemented!()` for the first scenario, remove `@wip`
  from that scenario only, and run the targeted command; it must exit non-zero.
  ```text
  command: `./run-tests.sh --name 'Enabling the review panel from the command line persists it'`
  exit: 1
  output: Scenario failed at the first Given: `Step panicked. Captured output: not implemented`; 1 feature / 1 scenario (1 failed) / 1 step (1 failed).
  ```

- [x] Record the baseline before scenario implementation. Run `./run-tests.sh`
  and `./run-tests.sh --e2e` and record exit status and scenario counts.
  ```text
  regular command: `./run-tests.sh`
  regular output/count: exit 1; 21 features; 199 scenarios (198 passed, 1 failed); 1196 steps (1195 passed, 1 failed). The single failure is the de-`@wip`ed first scenario with stub bindings.
  e2e command: `./run-tests.sh --e2e`
  e2e output/count: exit 0; 24 features; 81 scenarios (81 passed); 592 steps (592 passed).
  ```

- [x] Extend `tests/steps/interactive_shell_shortcut_steps.rs` with the new
  bindings. RED bodies use `unimplemented!()`; no empty bodies.
  Evidence: new bindings in `tests/steps/interactive_shell_shortcut_steps.rs`:
  - `a configured provider with candidate {string}`
  - `the persisted review surface is disabled` / `the persisted review surface is enabled`
  - `I run watn with --review-panel for {string}` / `I run watn with --no-review-panel for {string}`
  - `the review surface should be enabled in the configuration` / `the review surface should be disabled in the configuration`
  Helpers: `persisted_config_path`, `seed_persisted_panel`, `assert_persisted_panel` drive the real binary against an isolated configuration home.

## Scenarios

### Enabling the review panel from the command line persists it

Domain constraints:

- `--review-panel` enables the surface and persists `[review] panel = true`
  while preserving other configuration.

- [x] RED: Remove `@wip` from this scenario only. Bind the new Givens and Then
  with `unimplemented!()`. Run the exact targeted command; it must exit
  non-zero.
  ```text
  command: `./run-tests.sh --name 'Enabling the review panel from the command line persists it'`
  output: exit 1; 1 scenario (1 failed); the first Given stub panicked.
  ```
- [x] GREEN: Persist a set override through the generalized configuration seam.
  Compile with `cargo check --locked`, then run the exact targeted command.
  Production files changed: `src/config/mod.rs` (`persist_review_panel_at`, `persist_review_panel`), `src/main.rs` (set override persists), harness helpers and bindings.
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'Enabling the review panel from the command line persists it'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 6 steps (6 passed).
  ```
- [x] REFACTOR: Keep one persistence entry point for both directions. Rerun the
  exact targeted command.
  ```text
  command: `./run-tests.sh --name 'Enabling the review panel from the command line persists it'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 6 steps (6 passed).
  ```
- [x] COMMIT: `61a5d779e628ff94c34c0d18c3f9357e198ca34d` - `feat(interactive-shell-shortcut): Enabling the review panel from the command line persists it`

### Disabling the review panel from the command line persists it

Domain constraints:

- `--no-review-panel` disables the surface and persists `[review] panel = false`
  while preserving other configuration.

- [x] RED: Remove `@wip` from this scenario only. Run the exact targeted
  command; it must exit non-zero until the persistence binding is complete.
  ```text
  command: `./run-tests.sh --name 'Disabling the review panel from the command line persists it'`
  output: exit 1; 1 scenario (1 failed); the first Given stub panicked.
  ```
- [x] GREEN: Persist the disable direction and run the exact targeted command.
  Production files changed: none beyond the enable scenario; persisted-disable assertion binding.
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'Disabling the review panel from the command line persists it'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 6 steps (6 passed).
  ```
- [x] REFACTOR: Share the load-modify-save helper with the panel decision.
  Rerun the exact targeted command.
  ```text
  command: `./run-tests.sh --name 'Disabling the review panel from the command line persists it'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 6 steps (6 passed).
  ```
- [x] COMMIT: `79dbbb3f1f9f5bca7baccaa4e77cfca084293ac3` - `feat(interactive-shell-shortcut): Disabling the review panel from the command line persists it`

## Final Verification

- [x] Run `givn lint --change persist-review-panel-setting`.
  ```text
  command: `givn lint --change persist-review-panel-setting`
  output: exit 0; `givn lint: 1 file(s) checked — clean`.
  ```
- [x] Run the full regular suite `./run-tests.sh`.
  ```text
  command: `./run-tests.sh`
  output: exit 0; 21 features; 200 scenarios (200 passed); 1207 steps (1207 passed).
  ```
- [x] Run the full E2E suite `./run-tests.sh --e2e`.
  ```text
  command: `./run-tests.sh --e2e`
  output: exit 0; 24 features; 81 scenarios (81 passed); 592 steps (592 passed).
  ```
- [x] Run `cargo check --locked`.
  ```text
  command: `cargo check --locked`
  output: Finished `dev` profile [unoptimized + debuginfo] target(s) — no warnings or errors.
  ```
- [x] Run `givn status --change persist-review-panel-setting` and confirm the
  next artifact is `review`.
  ```text
  command: `givn status --change persist-review-panel-setting`
  output: all 16 tasks checked; artifacts proposal, specs, design, arc42-docs, design-review, tasks complete; next required artifact is `review`.
  ```
