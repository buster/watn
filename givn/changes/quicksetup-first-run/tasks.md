# Tasks: quicksetup-first-run

Runner: `./run-tests.sh` (verify.command) and `./run-tests.sh --e2e`
(verify.e2e_command, both configured in `givn/commands.yaml`). cucumber-rs with
`.fail_on_skipped()` in `tests/features_runner.rs`. Step file for this
capability: `tests/steps/quicksetup_steps.rs` (one file). Stub for
unimplemented steps: `unimplemented!()`.

## Setup

- [ ] **S1 — Step skeleton and isolation guard.** Create
  `tests/steps/quicksetup_steps.rs` with `isolate_quicksetup_env(world)`
  (fresh TempDir; set `HOME`, `XDG_CONFIG_HOME`, replace `PATH` with
  `<tmp>/bin`; panic if the guard fails) and stub step definitions for the
  quicksetup scenarios (`unimplemented!()` bodies, never empty). Register the
  module in `tests/steps/mod.rs`. Non-quicksetup behaviour is untouched.
  Evidence (build output):
  - [ ] `cargo build --features test-support --tests` succeeds
- [ ] **S2 — Proof of strictness.** Run one quicksetup scenario with stub
  steps via the single-scenario command from design.md; confirm non-zero
  exit. Paste command + tail of output:
  - [ ] evidence:
- [ ] **S3 — Runner subset proof.** Run `./run-tests.sh` and
  `./run-tests.sh --e2e`; record both scenario counts from the output
  (e2e count must be strictly smaller). Paste counts:
  - [ ] evidence:

## Non-@e2e scenarios (in .feature order)

### 1. Quick setup without a terminal prints guidance instead of asking

- [ ] RED: remove `@wip` from this scenario only; run single-scenario command
  → non-zero. Evidence:
- [ ] GREEN: production files (list): minimum `Commands::Quicksetup` variant,
  `run_quicksetup_command()` non-TTY guidance branch, dispatch.
  Evidence:
- [ ] REFACTOR: re-run single-scenario → still green. Evidence:
- [ ] COMMIT: `feat(quicksetup): Quick setup without a terminal prints guidance instead of asking`
  Hash:

### 2. A model question without a suggestion requires a non-empty answer

- [ ] RED: evidence:
- [ ] GREEN: production files (list): `src/quicksetup.rs` question flow,
  endpoint validation re-ask, empty-suggestion re-ask; `mod quicksetup` in
  `lib.rs`; TTY dispatch wiring; `config::config_file_exists()`.
  Evidence:
- [ ] REFACTOR: evidence:
- [ ] COMMIT: `feat(quicksetup): A model question without a suggestion requires a non-empty answer`
  Hash:

### 3. Quick setup does not ask reasoning questions and stores no reasoning

- [ ] RED: evidence:
- [ ] GREEN: production files (list): persistence (provider draft via
  `build_provider_draft`/`update_provider_draft`, tier models, `save_config`),
  closing message with config path and `watn setup` hint; `${ENV}` credential
  suggestion when the endpoint's suggested key variable is set.
  Evidence:
- [ ] REFACTOR: evidence:
- [ ] COMMIT: `feat(quicksetup): Quick setup does not ask reasoning questions and stores no reasoning`
  Hash:

### 4. An OpenAI endpoint suggests the OpenAI credential and no model

- [ ] RED: evidence:
- [ ] GREEN: production files (list): endpoint-derived suggestion resolution
  (openai endpoint → `${OPENAI_API_KEY}` suggestion, empty model suggestion).
  Evidence:
- [ ] REFACTOR: evidence:
- [ ] COMMIT: `feat(quicksetup): An OpenAI endpoint suggests the OpenAI credential and no model`
  Hash:

### 5. Shell integrations are pre-selected only for shells available on the path

- [ ] RED: evidence:
- [ ] GREEN: production files (list): PATH-based availability helper in
  `src/shell_shortcut.rs`; shell list rendering with `[ ]`/`[x]`, typed
  toggle answers, empty line confirm.
  Evidence:
- [ ] REFACTOR: evidence:
- [ ] COMMIT: `feat(quicksetup): Shell integrations are pre-selected only for shells available on the path`
  Hash:

### 6. Explicit provider selection skips the first-run quick setup

- [ ] RED: evidence:
- [ ] GREEN: production files (list): first-run branch in the request path
  inside the existing implicit-selection gate; non-TTY path unchanged.
  Evidence:
- [ ] REFACTOR: evidence:
- [ ] COMMIT: `feat(quicksetup): Explicit provider selection skips the first-run quick setup`
  Hash:

### 7. Aborting explicit quick setup leaves the previous configuration unchanged

- [ ] RED: evidence:
- [ ] GREEN: production files (list): none expected beyond existing write-only-
  at-confirm behaviour; verify no-write-before-confirm invariant. An empty list
  is acceptable only with that justification recorded here.
  Evidence:
- [ ] REFACTOR: evidence:
- [ ] COMMIT: `feat(quicksetup): Aborting explicit quick setup leaves the previous configuration unchanged`
  Hash:

### 8. A failed configuration write installs no shell integration

- [ ] RED: evidence:
- [ ] GREEN: production files (list): save-failure error path (no install,
  nonzero exit) using the `WATN_TEST_FAIL_CONFIG_WRITE` seam.
  Evidence:
- [ ] REFACTOR: evidence:
- [ ] COMMIT: `feat(quicksetup): A failed configuration write installs no shell integration`
  Hash:

### 9. A failed shell installation keeps the saved configuration

- [ ] RED: evidence:
- [ ] GREEN: production files (list): install-after-save with aggregated
  nonzero report; fish target forced unwritable by step fixture.
  Evidence:
- [ ] REFACTOR: evidence:
- [ ] COMMIT: `feat(quicksetup): A failed shell installation keeps the saved configuration`
  Hash:

## @e2e scenarios (verify.e2e_command)

### 10. First run without a configuration starts the quick setup

- [ ] RED: evidence:
- [ ] GREEN: production files (list): announcement output; sentinel step
  (httpmock via `WATN_TEST_ENDPOINT_OVERRIDE`, no `WATN_PROVIDER`).
  Evidence:
- [ ] REFACTOR: evidence:
- [ ] COMMIT: `test(e2e): First run without a configuration starts the quick setup`
  Hash:

### 11. Completing the quick setup stores the answers and installs the chosen integrations

- [ ] RED: evidence:
- [ ] GREEN: production files (list): shell install per selected shell
  (completion + Ctrl-W blocks); full happy path through PTY.
  Evidence:
- [ ] REFACTOR: evidence:
- [ ] COMMIT: `test(e2e): Completing the quick setup stores the answers and installs the chosen integrations`
  Hash:

### 12. Explicit quick setup overwrites an existing configuration

- [ ] RED: evidence:
- [ ] GREEN: production files (list): overwrite persistence (provider
  migration to `custom`, literal credential, tier overwrite); shell
  deselection writes nothing.
  Evidence:
- [ ] REFACTOR: evidence:
- [ ] COMMIT: `test(e2e): Explicit quick setup overwrites an existing configuration`
  Hash:

### 13. Aborting quick setup with Ctrl-C on the first run leaves no configuration

- [ ] RED: evidence:
- [ ] GREEN: production files (list): none expected beyond existing
  write-only-at-confirm; justify if empty.
  Evidence:
- [ ] REFACTOR: evidence:
- [ ] COMMIT: `test(e2e): Aborting quick setup with Ctrl-C on the first run leaves no configuration`
  Hash:

## Final

- [ ] **F1 — Authoritative command tree.** Add `quicksetup` to the built-binary
  e2e step list in `tests/steps/shell_completions_e2e_steps.rs`; the five
  modified shell-completions scenarios go GREEN. Run the full suite.
  Evidence:
- [ ] **F2 — Isolation audit.** Grep `tests/steps/quicksetup_steps.rs`: every
  `start_pty_session` / `run_binary_with_state` call site preceded by
  `isolate_quicksetup_env`. Evidence:
- [ ] **F3 — Full verification.** `./run-tests.sh` exit 0; `./run-tests.sh --e2e`
  exit 0. Evidence:
