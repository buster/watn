# Tasks: quicksetup-first-run

Runner: `./run-tests.sh` (verify.command) and `./run-tests.sh --e2e`
(verify.e2e_command, both configured in `givn/commands.yaml`). cucumber-rs with
`.fail_on_skipped()` in `tests/features_runner.rs`. Step file for this
capability: `tests/steps/quicksetup_steps.rs` (one file). Stub for
unimplemented steps: `unimplemented!()`.

## Setup

- [x] **S1 — Step skeleton and isolation guard.** Create
  `tests/steps/quicksetup_steps.rs` with `isolate_quicksetup_env(world)`
  (fresh TempDir; set `HOME`, `XDG_CONFIG_HOME`, replace `PATH` with
  `<tmp>/bin`; panic if the guard fails) and stub step definitions for the
  quicksetup scenarios (`unimplemented!()` bodies, never empty). Register the
  module in `tests/steps/mod.rs`. Non-quicksetup behaviour is untouched.
  Evidence (build output):
  - [x] `cargo build --features test-support --tests` succeeds
    (`Finished dev profile ... 14.09s` after two PathBuf/Pattern fixes)
- [x] **S2 — Proof of strictness.** Run one quicksetup scenario with stub
  steps via the single-scenario command from design.md; confirm non-zero
  exit. Paste command + tail of output:
  - [x] evidence: scenario-1 RED run: `1 scenario (1 failed)` /
    `4 steps (3 passed, 1 failed)` — the guidance assertion panicked against
    the old provider guidance; cargo reported `error: test failed`,
    non-zero run. Strict mode proven (failing step fails the run).
- [ ] **S3 — Runner subset proof.** Run `./run-tests.sh` and
  `./run-tests.sh --e2e`; record both scenario counts from the output
  (e2e count must be strictly smaller). Paste counts:
  - [ ] evidence:

## Non-@e2e scenarios (in .feature order)

### 1. Quick setup without a terminal prints guidance instead of asking

- [x] RED: removed `@wip`; targeted run → `1 scenario (1 failed)`,
  `4 steps (3 passed, 1 failed)`, `Step panicked ... quicksetup guidance
  missing: "No provider is configured. Run `watn setup` ..."` — non-zero.
- [x] GREEN: production files: `src/main.rs` (`Commands::Quicksetup`,
  `run_quicksetup_command` non-TTY guidance, dispatch), `src/quicksetup.rs`
  (new module, full plain-line flow), `src/lib.rs` (`pub mod quicksetup`).
  Targeted run → `1 scenario (1 passed)`, `5 steps (5 passed)`.
- [x] REFACTOR: no behaviour change; re-run → `1 scenario (1 passed)`,
  `5 steps (5 passed)`.
- [x] COMMIT: `feat(quicksetup): Quick setup without a terminal prints guidance instead of asking`
  Hash: 734e954

### 2. A model question without a suggestion requires a non-empty answer

- [x] RED: removed `@wip`; targeted run → `Step match is ambiguous` for
  `I enter endpoint` (collision with streamlined_setup regex) — run failed
  non-zero. Fixed by renaming quicksetup answer steps to unique wording.
- [x] GREEN: production files: none beyond scenario 1 (flow already covers
  re-ask); tests/steps/quicksetup_steps.rs implemented the endpoint,
  credential, small-model answer steps and re-ask assertion. Targeted run →
  `1 scenario (1 passed)`, `7 steps (7 passed)`.
- [x] REFACTOR: no-op; re-run → `1 scenario (1 passed)`, `7 steps (7 passed)`.
- [x] COMMIT: `feat(quicksetup): A model question without a suggestion requires a non-empty answer`
  Hash: 2c38c0b

### 3. Quick setup does not ask reasoning questions and stores no reasoning

- [x] RED: removed `@wip`; targeted run → stub panic in the compound When,
  `1 scenario (1 failed)`, non-zero.
- [x] GREEN: production files: `src/config/types.rs`
  (`skip_serializing_if` for empty `TierReasoning` — reasoning stays
  absent from the saved file), tests/steps/quicksetup_steps.rs (compound
  When with session start, no-reasoning + model-without-reasoning
  assertions). Targeted run → `1 scenario (1 passed)`, `7 steps (7 passed)`.
- [x] REFACTOR: fixed unclosed-impl compile error from the serde edit;
  re-run → `1 scenario (1 passed)`, `7 steps (7 passed)`.
- [x] COMMIT: `feat(quicksetup): Quick setup does not ask reasoning questions and stores no reasoning`
  Hash: a124976

### 4. An OpenAI endpoint suggests the OpenAI credential and no model

- [x] RED: two genuine failures captured. (a) The scenario never answered
  the credential question, so the small-model wait timed out
  (`PTY did not render label "Small model"`); fixed the flow.
  (b) `And I accept the suggested credential reference` after a `Then` is a
  Then-keyword step and did not match the `#[when]` definition — explicit
  `When` keyword added.
- [x] GREEN: production files: none (suggestion logic already endpoint-derived
  in src/quicksetup.rs); step text fix in the delta spec. Targeted run →
  `1 scenario (1 passed)`, `7 steps (7 passed)`.
- [x] REFACTOR: no-op; passing state unchanged.
- [x] COMMIT: `feat(quicksetup): An OpenAI endpoint suggests the OpenAI credential and no model`
  Hash: 0f2ad86 (an earlier commit attempt 21ed020 on the failing state was
  reset locally before pushing; the scenario commit contains the fixed spec).

### 5. Shell integrations are pre-selected only for shells available on the path

- [x] RED: removed `@wip`; targeted run → stub panic in the accept step,
  non-zero. (Follow-up RED: multi-shell Then text "Bash and Zsh" did not
  match the single-shell regex — spec split into per-shell assertions.)
- [x] GREEN: production files: none beyond scenario 1 (PATH detection already
  in `shells_available_on_path`); tests/steps/quicksetup_steps.rs implemented
  the accept-through-models step and shell-list marking assertion. Targeted
  run → `1 scenario (1 passed)`, `8 steps (8 passed)`. Proves PATH
  replacement isolation: the runner's real `/usr/bin/fish` is not detected.
- [x] REFACTOR: removed duplicate stub step and unused import.
- [x] COMMIT: `feat(quicksetup): Shell integrations are pre-selected only for shells available on the path`
  Hash: 0b71815

### 6. Explicit provider selection skips the first-run quick setup

- [x] RED: not applicable — all steps were pre-implemented/reused
  (`I run a request for...` fixture, reused status/config Thens, and the
  quick-setup-mention assertion); legitimate immediate GREEN per the
  step-reuse rule. The bypass invariant holds without new production code;
  the first-run branch itself is exercised by @e2e task 10.
- [x] GREEN: production files: none. Targeted run → `1 scenario (1 passed)`,
  `6 steps (6 passed)`.
- [x] REFACTOR: no-op.
- [x] COMMIT: `feat(quicksetup): Explicit provider selection skips the first-run quick setup`
  Hash: 4f33af9

### 7. Aborting explicit quick setup leaves the previous configuration unchanged

- [x] RED: removed `@wip`; targeted run → abort step panic, then a real
  finding: `ensure_test_env` skips writing the fixture config when the temp
  dir already exists (quicksetup isolation pre-creates it) —
  `config file not readable at abort` panic. Fixed with
  `ensure_quicksetup_fixture_config` in the step file.
- [x] GREEN: production files: none; tests/steps/quicksetup_steps.rs
  (fixture-config materialization, abort-with-Ctrl-C recording baseline,
  shell-target absence assertion). Targeted run → `1 scenario (1 passed)`,
  `6 steps (6 passed)`.
- [x] REFACTOR: no-op.
- [x] COMMIT: `feat(quicksetup): Aborting explicit quick setup leaves the previous configuration unchanged`
  Hash: 091f27f

### 8. A failed configuration write installs no shell integration

- [x] RED: two findings: (a) step-text collision with streamlined_setup's
  `the final configuration write cannot complete` — renamed to
  `the configuration write is forced to fail` (own world.env_vars-based
  fixture, auto-cleaned by WatnWorld::drop instead of leaking the fail flag
  through parent env); (b) the error assertion found empty stderr — the PTY
  harness merges stderr into the output stream, so the assertion checks the
  merged stream.
- [x] GREEN: production files: none (save-failure-before-install ordering
  already in src/quicksetup.rs); step implementations only. Targeted run →
  `1 scenario (1 passed)`, `7 steps (7 passed)`.
- [x] REFACTOR: no-op.
- [x] COMMIT: `feat(quicksetup): A failed configuration write installs no shell integration`
  Hash: a4d9838

### 9. A failed shell installation keeps the saved configuration

- [x] RED: removed `@wip`; targeted run → stub panics in the assertion steps,
  non-zero.
- [x] GREEN: production files: none (install-after-save with aggregated
  nonzero report already in src/quicksetup.rs); tests/steps/quicksetup_steps.rs
  implemented the nonzero-result, model, and shell-block assertions. Targeted
  run → `1 scenario (1 passed)`, `9 steps (9 passed)`. The fish target is a
  directory, forcing the install failure; bash and zsh installs succeeded and
  the config stayed saved.
- [x] REFACTOR: removed the duplicate stub of the nonzero-result step.
- [x] COMMIT: `feat(quicksetup): A failed shell installation keeps the saved configuration`
  Hash: 683bbbf

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
