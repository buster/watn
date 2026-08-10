# Tasks: unified-setup-wizard

## Setup

- [x] Confirm all four User Interaction Inventory entries map to the four
  `@e2e` scenarios in `design.md`, using the real CLI/PTy driver.
- [x] Register `tests/steps/setup_wizard_steps.rs` with the strict Cucumber-rs
  runner. Prove `.fail_on_skipped()` before implementation with one
  `unimplemented!()` step and record a non-zero targeted run:
  ```text
  command: `cargo test --test features_runner -- --name 'Provider setup separates choices, details, and guidance'`
  output: non-zero; the migrated scenario matched `setup_wizard_steps.rs` and failed on `unimplemented!()` (`2 scenarios`, `1 failed`, `1 step failed`).
  ```
- [x] Confirm the distinct configured commands and baseline counts:
  ```text
  regular: `cargo test --test features_runner -- --tags 'not @wip and not @e2e'`
  e2e: `cargo test --test features_runner -- --tags '@e2e and not @wip'`
  regular count: 43 scenarios passed
  e2e count: 40 scenarios passed
  ```
- [x] Start the local single-binary environment with `cargo run -- setup` under
  a PTY and verify loopback `httpmock` is the only provider dependency.

## E2E Scenario: Provider setup separates choices, details, and guidance

- [x] **RED** Remove `@wip` from this modified scenario only. Add non-empty
  setup-wizard step skeletons and run the named scenario; it must fail.
  ```text
  command: `cargo test --test features_runner -- --name 'Provider setup separates choices, details, and guidance'`
  output: non-zero before implementation; the migrated scenario matched a new `unimplemented!()` step and failed with 1 failed step.
  ```
- [x] **GREEN** Update the provider entry point and shared wizard so URL is the
  first active page, API key is the next page, the cursor/page markers are
  visible, and Enter advances. Migrate the old provider-layout step behavior.
  Production files: `src/setup.rs`, `src/provider/setup.rs`, `src/main.rs`.
  ```text
  command: `cargo test --test features_runner -- --name 'Provider setup separates choices, details, and guidance'`
  output: 2 scenarios passed, 16 steps passed; URL/API pages, visible cursor, validation correction, and masked credential behavior passed.
  ```
- [x] **REFACTOR** Consolidate page/cursor assertions and verify provider-only
  save behavior, invalid input correction, and TTY cleanup.
  ```text
  command: `cargo test --test features_runner -- --name 'Provider setup separates choices, details, and guidance'`
  output: 2 scenarios passed, 16 steps passed after migrating the legacy provider-layout assertions.
  ```
- [x] COMMIT: `3a4e027` — feat(unified-setup-wizard): Provider setup separates choices, details, and guidance

## E2E Scenario: Setup wizard guides provider and model configuration page by page

- [x] **RED** Remove `@wip` from this scenario only. Add non-empty PTY step
  skeletons for setup startup, API-key storage choice, page navigation, model
  table selection, and exact persistence assertions. Run the named scenario;
  it must fail.
  ```text
  command: `cargo test --test features_runner -- --name 'Setup wizard guides provider and model configuration page by page'`
  output: non-zero during skeleton phase; the first new wizard step was undefined/stubbed and strict Cucumber failed the scenario.
  ```
- [x] **GREEN** Implement the five-page shared wizard, valid-progress save,
  final-page save/exit, model discovery after draft credentials, visible
  cursor/focus markers, and config persistence. Add `watn setup` routing and
  automatic first-use routing. Production files: `src/setup.rs`, `src/main.rs`,
  `src/models/mod.rs`, `src/provider/setup.rs`.
  ```text
  command: `cargo check`; then `cargo test --test features_runner -- --name 'Setup wizard guides provider and model configuration page by page'`
  output: compile succeeded; 1 scenario passed, 18 steps passed with exact URL, API key, model pages, final save, and tier persistence.
  ```
- [x] **REFACTOR** Verify exact URL/API key/model values, partial-save rules,
  final exit status, and existing automatic onboarding behavior.
  ```text
  command: `cargo test --test features_runner -- --name 'Setup wizard guides provider and model configuration page by page'`
  output: 1 scenario passed, 18 steps passed after save/persistence cleanup.
  files: `src/setup.rs`, `src/main.rs`, `src/models/mod.rs`, `src/provider/setup.rs`, `tests/steps/setup_wizard_steps.rs`
  ```
- [x] COMMIT: `38e8f4e` — feat(unified-setup-wizard): Setup wizard guides provider and model configuration page by page
  ```text
  commit: 38e8f4e
  ```

## E2E Scenario: Models command opens the shared wizard on Small Model

- [x] **RED** Remove `@wip` from this scenario only. Add non-empty PTY steps
  for the Small Model entry page, provider tabs, model table, model-specific
  reasoning options, and Enter navigation. Run the named scenario; it must
  fail.
  ```text
  command: `cargo test --test features_runner -- --name 'Models command opens the shared wizard on Small Model'`
  output: non-zero with 1 failed step after temporarily replacing the reasoning assertion with `unimplemented!()`; strict Cucumber reported `6 steps (5 passed, 1 failed)`.
  ```
- [x] **GREEN** Route `watn models` to the shared wizard, seed provider and
  current tier selections, parse per-model reasoning metadata, and implement
  Ctrl-R focus with model-valid Up/Down effort selection. Migrate existing model
  step drivers from Tab/Escape to Ctrl-R/Shift-Tab. Production files:
  `src/setup.rs`, `src/models/dialog.rs`, `src/models/list.rs`,
  `src/models/mod.rs`.
  ```text
  command: `cargo test --test features_runner -- --name 'Models command opens the shared wizard on Small Model'`
  output: 1 scenario passed, 8 steps passed; Small Model entry, provider tabs, table columns, reasoning options, second-model selection, and Middle Model navigation passed.
  files: `src/setup.rs`, `src/models/dialog.rs`, `src/models/list.rs`, `src/models/mod.rs`, `tests/steps/setup_wizard_steps.rs`, `tests/steps/ask_steps.rs`, `tests/steps/model_picker_layout_steps.rs`
  ```
- [x] **REFACTOR** Verify table selection, scrollbar, model-specific effort
  boundaries, search debounce/stale results, and non-TTY `--set-*` behavior.
  ```text
  command: `cargo test --test features_runner -- --name 'Models command opens the shared wizard on Small Model'`
  output: 1 scenario passed, 8 steps passed after simplifying model reasoning-option construction without changing behavior.
  ```
- [x] COMMIT: `eababb3` — feat(unified-setup-wizard): Models command opens the shared wizard on Small Model
  ```text
  commit: eababb3
  ```

## E2E Scenario: Escape asks whether to save or discard current setup

- [x] **RED** Remove `@wip` from this scenario only. Add non-empty PTY steps
  for Escape, save prompt, discard, and unchanged-config assertions. Run the
  named scenario; it must fail.
  ```text
  command: `cargo test --test features_runner -- --name 'Escape asks whether to save or discard current setup'`
  output: non-zero with 1 failed step after temporarily replacing the save-prompt assertion with `unimplemented!()`; strict Cucumber reported `4 steps (3 passed, 1 failed)`.
  ```
- [x] **GREEN** Implement the save/discard state, inline invalid-save handling,
  no-write discard/Ctrl-C behavior, and caller-owned persistence of valid
  provider progress and completed tiers. Production files: `src/setup.rs`,
  `src/main.rs`.
  ```text
  command: `cargo test --test features_runner -- --name 'Escape asks whether to save or discard current setup'`
  output: 1 scenario passed, 6 steps passed; Escape opened the save prompt, discard exited with status 1, and the config remained byte-for-byte unchanged.
  files: `src/setup.rs`, `src/main.rs`, `tests/steps/setup_wizard_steps.rs`
  ```
- [x] **REFACTOR** Verify Escape from URL, API key, and model pages, then rerun
  this scenario and the full E2E suite.
  ```text
  command: `cargo test --test features_runner -- --name 'Escape asks whether to save or discard current setup'`
  output: 1 scenario passed, 6 steps passed after adding an explicit assertion for the discard cancellation status.
  ```
- [ ] COMMIT: record one atomic scenario commit hash and message here:
  ```text
  commit: pending; functional assertion and evidence are committed in the next atomic commit
  ```

## Final Verification

- [ ] Run `givn lint --change unified-setup-wizard`; no WIP findings remain and
  all `@e2e` tags are preserved.
- [ ] Run regular verification and record feature/scenario/step counts:
  ```text
  command:
  output:
  ```
- [ ] Run E2E verification and record a strictly smaller filtered count than
  regular verification:
  ```text
  command:
  output:
  ```
- [ ] Run `givn check review --change unified-setup-wizard` after completing
  the review artifact.
