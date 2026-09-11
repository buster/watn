# Tasks: model-pricing-visibility

Use-case: `configure-model`. Actor: `Watn user`. Personas: none.
Verify command: `./run-tests.sh` (regular) and `./run-tests.sh --e2e` (e2e).
Single scenario: `./run-tests.sh --name "<scenario title>"`.

## Setup

- [x] Confirm the runner and strict mode.

  `givn/commands.yaml` already declares `verify.command: "./run-tests.sh"`
  and `verify.e2e_command: "./run-tests.sh --e2e"`. Strict mode is
  `.fail_on_skipped()` in `tests/features_runner.rs:208` plus the non-zero
  exit on failed/skipped at line 225. Step definitions are registered by
  `tests/steps/mod.rs`; the capability files are `tests/steps/ask_steps.rs`,
  `tests/steps/model_picker_layout_steps.rs`,
  `tests/steps/streamlined_setup_steps.rs`,
  `tests/steps/streamlined_setup_e2e_steps.rs`, and
  `tests/steps/provider_setup_steps.rs`.

  Evidence (proof of strictness — undefined step must exit non-zero):
  `./run-tests.sh --name "Non-terminal model assignment records catalog prices"`
  → `1 scenario (1 failed)`, `Step doesn't match any function`, `exit status: 1`.

---

## Scenario: Model picker shows metadata when available

Capability `models`. Use-case guarantees: Main flow 1–2 (discover, assign),
display of the chosen model metadata. Modified scenario in
`specs/models/models.feature`.

- [x] RED — remove `@wip` from this scenario only; run it targeted. Undefined
      steps (the added `stderr should not contain "$-"` assertion) must fail
      non-zero. Evidence: `./run-tests.sh --name "Model picker shows metadata when available"` → `2 scenarios (1 passed, 1 failed)`, exit 1; failure: expected `$0.15/1M in, $0.60/1M out`, got `$0.15/1K in, $0.60/1K out`.
- [x] GREEN — production: `src/models/list.rs` (single parse path, per-token
      to $/1M normalization, both-components rule, six-decimal rounding) and
      `src/models/mod.rs` (per-1M `format_model_entry`). Tests: rich fixture
      per-token values, sentinel and bare rows, new stderr assertion.
      Evidence: `./run-tests.sh --name "Model picker shows metadata when available"` → `2 scenarios (2 passed)`, `8 steps (8 passed)`, exit 0. Files: `src/models/list.rs`, `src/models/mod.rs`, `tests/steps/ask_steps.rs`, `specs/models/models.feature`.
- [x] REFACTOR — dedupe `fetch_models` onto `parse_model_data`; no behavior
      change. Evidence: targeted re-run → `2 scenarios (2 passed)`, exit 0; `cargo test --lib` → `86 passed`.
- [x] COMMIT — `feat(models): Model picker shows metadata when available`.
      Hash: 5366fff

## Scenario: Non-terminal model assignment records catalog prices

Capability `models`. Use-case guarantee: Main flow 4 (persist and apply),
minimal guarantee (no corruption). Added scenario.

- [ ] RED — remove `@wip` from this scenario only; run targeted; new config
      assertion steps and `[pricing]` seeding must fail non-zero.
      Evidence: _pending_
- [ ] GREEN — production: `capture_catalog_price` in `src/setup.rs` and its
      call in `src/models/mod.rs::run_models_result` before `save_config`.
      Tests: new given/then bindings in `tests/steps/ask_steps.rs`.
      Evidence: _pending_
- [ ] REFACTOR — share the capture rule with the other write paths' call
      shape; no behavior change. Evidence: _pending_
- [ ] COMMIT — `feat(models): Non-terminal model assignment records catalog prices`.
      Hash: _pending_

## Scenario: Model entry shows additional metadata when available

Capability `ratatui-model-picker`. Guarantee: catalog price shown in the
persisted unit. Modified scenario.

- [ ] RED — remove `@wip` from this scenario only; run targeted; the new
      per-million display assertion must fail non-zero. Evidence: _pending_
- [ ] GREEN — production: normalized value flows through
      `format_model_entry`; tests: `provider_with_models_pricing` and the
      displayed-price assertion in `tests/steps/ask_steps.rs`.
      Evidence: _pending_
- [ ] REFACTOR — remove stale per-token fixture assumptions; no behavior
      change. Evidence: _pending_
- [ ] COMMIT — `feat(ratatui-model-picker): Model entry shows additional metadata when available`.
      Hash: _pending_

## Scenario: Interactive model table shows published prices per million tokens

Capability `ratatui-model-picker`. Guarantee: the terminal table shows the
same normalized price as the list. Added scenario.

- [ ] RED — remove `@wip` from this scenario only; run targeted; the new
      table/header assertions and priced-catalog table given must fail
      non-zero. Evidence: _pending_
- [ ] GREEN — production: `src/setup.rs::draw_model` cell and header;
      tests: priced-catalog table given and table assertion steps.
      Evidence: _pending_
- [ ] REFACTOR — reuse the priced-catalog given between this scenario and
      the e2e; no behavior change. Evidence: _pending_
- [ ] COMMIT — `feat(ratatui-model-picker): Interactive model table shows published prices per million tokens`.
      Hash: _pending_

---

## E2E setup

- [ ] Confirm the e2e environment and filter.

  No external service or container is required: every provider is an
  in-process `httpmock` twin on `127.0.0.1`, and PTY flows use
  `portable_pty`. `./run-tests.sh` builds the default and `test-support`
  binaries before running. E2E step bindings live in
  `tests/steps/streamlined_setup_e2e_steps.rs` and
  `tests/steps/provider_setup_steps.rs` (separate from regular bindings, per
  `design.md`). `verify.e2e_command` (`./run-tests.sh --e2e`) is a strict
  subset of `verify.command` via its `@e2e and not @wip` tag filter.

  Evidence (full-run vs e2e-run scenario counts, e2e strictly less):
  _pending_

## Scenario: Models setup configures all three roles from an available catalog

Capability `streamlined-setup`. Use-case guarantee: Main flow 4 across the
focused model write path. Modified `@e2e` scenario.

- [ ] RED — remove `@wip` from this scenario only; run e2e targeted; the
      priced given and pricing assertions must fail non-zero.
      Evidence: _pending_
- [ ] GREEN — production: `apply_models_result` calls
      `capture_catalog_price`; tests: priced-catalog given and config
      assertion steps drive the real PTY flow. Evidence: _pending_
- [ ] REFACTOR — no behavior change. Evidence: _pending_
- [ ] COMMIT — `test(e2e): Models setup configures all three roles from an available catalog`.
      Hash: _pending_

## Scenario: Coordinated setup completes provider models reasoning and shell choices

Capability `streamlined-setup`. Use-case guarantee: Main flow 4 across the
coordinated final-confirmation write path. Modified `@e2e` scenario.

- [ ] RED — remove `@wip` from this scenario only; run e2e targeted; the
      priced transport table and pricing assertions must fail non-zero.
      Evidence: _pending_
- [ ] GREEN — production: `apply_result` calls `capture_catalog_price`;
      tests: priced ephemeral-transport given in
      `tests/steps/provider_setup_steps.rs`. Evidence: _pending_
- [ ] REFACTOR — no behavior change. Evidence: _pending_
- [ ] COMMIT — `test(e2e): Coordinated setup completes provider models reasoning and shell choices`.
      Hash: _pending_

---

## Final checks

- [ ] Full regular suite green: `./run-tests.sh` output pasted, zero exit.
      Evidence: _pending_
- [ ] Full e2e suite green: `./run-tests.sh --e2e` output pasted, zero exit.
      Evidence: _pending_
- [ ] No empty/no-op step bodies introduced (`unimplemented!`/`todo!`
      grep returns none in touched step files). Evidence: _pending_
- [ ] Commit count matches scenario count (six feature commits plus spec
      commits). Evidence: _pending_
