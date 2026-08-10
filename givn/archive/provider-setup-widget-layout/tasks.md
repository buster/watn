# Tasks: provider-setup-widget-layout

## Setup

- [x] Confirm the User Interaction Inventory and Interaction Coverage Matrix
  match: both entries in
  `givn/changes/provider-setup-widget-layout/specs/provider-setup-widget-layout/provider-setup-widget-layout.feature`
  have exactly one `@e2e` scenario and one corresponding CLI/PTy row in
  `design.md`.
- [x] Confirm the existing Cucumber-rs runner is the executable-spec runner,
  scans `givn/specs/**` and active change specs, and uses
  `.fail_on_skipped()`; create capability-specific step modules
  `tests/steps/provider_setup_layout_steps.rs` and
  `tests/steps/model_picker_layout_steps.rs` without duplicating the existing
  provider-start step.
- [x] Prove strict mode before scenario work by running the targeted provider
  scenario with a new `unimplemented!()` step stub. Record the non-zero command
  and output here:
  ```text
  command: `cargo test --test features_runner -- --name 'Provider setup separates choices, details, and guidance'`
  output: non-zero; Cucumber matched the new step, panicked on `unimplemented!()`, and reported `1 scenario (1 failed)` / `1 step failed`.
  ```
- [x] Confirm `givn/commands.yaml` keeps distinct commands:
  `cargo test --test features_runner -- --tags 'not @wip and not @e2e'` for
  regular verification and
  `cargo test --test features_runner -- --tags '@e2e and not @wip'` for E2E.
  Record the baseline scenario counts here:
  ```text
  regular count: 43 scenarios (43 passed)
  e2e count: 35 scenarios (34 passed, provider layout stub failed as intended)
  ```

## E2E Scenario: Provider setup separates choices, details, and guidance

- [x] **RED** Remove `@wip` from this scenario only. Add non-empty step
  skeletons for invalid-endpoint input, masked-credential input, and each new
  provider-layout assertion. Run the single scenario through the E2E command;
  it must fail. Record evidence:
  ```text
  command: `cargo test --test features_runner -- --name 'Provider setup separates choices, details, and guidance'`
  output: non-zero; 2 Given/When steps passed and the first new Then step matched `provider_setup_layout_steps.rs` then failed on `unimplemented!()` (`1 scenario failed`, `1 step failed`).
  ```
- [x] **GREEN** Replace the skeletons with PTY assertions and deterministic
  lifecycle handling. Reuse the existing `I start \`watn provider\` in a
  terminal` step. Refactor `src/provider/setup.rs` to render the fully stacked
  bordered/list/table/paragraph layout while preserving validation and masking.
  Compile first, then run the single scenario through the E2E command; it must
  pass. Production files changed: `src/provider/setup.rs`.
  ```text
  command: `cargo check`; then `cargo test --test features_runner -- --name 'Provider setup separates choices, details, and guidance'`
  output: compile succeeded; 1 scenario passed, 10 steps passed. The live PTY showed the bordered setup, source list, details table, guidance, validation, and masked input.
  ```
- [x] **REFACTOR** Remove duplicate helper logic, keep the visible-label
  assertions and cleanup behavior unchanged, and rerun the single scenario.
  Record evidence:
  ```text
  command: `cargo test --test features_runner -- --name 'Provider setup separates choices, details, and guidance'`
  output: 1 scenario passed, 10 steps passed after removing the unused plain-text PTY wait helper.
  ```
- [x] COMMIT: `a7593b3` — feat(provider-setup-widget-layout): Provider setup separates choices, details, and guidance

## E2E Scenario: Model picker makes tiers and long model lists easy to scan

- [x] **RED** Remove `@wip` from this scenario only. Add non-empty step
  skeletons for starting the model picker, the layout assertions, and the
  Down + Enter navigation assertions. Run the single scenario through the E2E
  command; it must fail. Record evidence:
  ```text
  command: `cargo test --test features_runner -- --name 'Model picker makes tiers and long model lists easy to scan'`
  output: non-zero; the long-list Given passed and the new terminal-start step matched then failed on `unimplemented!()` (`1 scenario failed`, `1 step failed`).
  ```
- [x] **GREEN** Replace the skeletons with real PTY steps and readiness/cleanup.
  Refactor `src/models/dialog.rs` to render stacked tabs, paragraphs, a
  stateful metadata table, and an overflow-only scrollbar. Move search to a
  200 ms worker/debounce flow with generation checks, status handling, and no
  selectable empty placeholder. Preserve existing filter and selected-row
  contracts. Compile first, then run the single scenario through the E2E
  command; it must pass. Production files changed: `src/models/dialog.rs`.
  ```text
  command: `cargo check`; then `cargo test --test features_runner -- --name 'Model picker makes tiers and long model lists easy to scan'`
  output: compile succeeded; 1 scenario passed, 9 steps passed. The live PTY showed the bordered picker, tier tabs, aligned table headings, overflow scrollbar, active normal tier, and selected model row.
  ```
- [x] **REFACTOR** Consolidate row/status/scrollbar helpers and verify existing
  model-picker, autosuggest, empty-state, and stale-result scenarios still pass.
  Rerun the single scenario through the E2E command. Record evidence:
  ```text
  command: `cargo test --test features_runner -- --name 'Model picker makes tiers and long model lists easy to scan'`; also `cargo test --test features_runner -- --tags '@e2e and not @wip'` and `cargo test --test features_runner -- --tags 'not @wip and not @e2e'`.
  output: targeted scenario passed (9 steps); full E2E passed (36 scenarios, 208 steps); regular suite passed (43 scenarios, 234 steps). Existing filter, page navigation, reasoning, back-navigation, empty-state, fallback, and stale-result scenarios remained green.
  ```
- [x] COMMIT: `49c7856` — feat(provider-setup-widget-layout): Model picker makes tiers and long model lists easy to scan

## Final Verification

- [x] Run `givn lint --change provider-setup-widget-layout`; no WIP findings
  remain and both `@e2e` tags are preserved.
- [x] Run the regular verify command and record zero exit plus scenario count:
  ```text
  command: `cargo test --test features_runner -- --tags 'not @wip and not @e2e'`
  output: zero exit; 8 features, 43 scenarios (43 passed), 234 steps (234 passed).
  ```
- [x] Run the E2E verify command and record zero exit plus a count strictly
  smaller than the regular suite:
  ```text
  command: `cargo test --test features_runner -- --tags '@e2e and not @wip'`
  output: zero exit; 9 features, 36 scenarios (36 passed), 208 steps (208 passed). E2E count 36 is strictly below regular-plus-e2e full coverage (the regular command covers 43 non-E2E scenarios).
  ```
