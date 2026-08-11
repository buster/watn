# Tasks: interactive-shell-shortcut

## Setup

- [x] Register `tests/steps/interactive_shell_shortcut_steps.rs` and
  `tests/steps/interactive_shell_shortcut_e2e_steps.rs`, extend only the
  required `WatnWorld` state, retain `.fail_on_skipped()`, and use explicit
  `unimplemented!()` RED bodies.
  ```text
  verify.command: ./run-tests.sh
  verify.e2e_command: ./run-tests.sh --e2e
  strict proof command: root=$(mktemp -d /tmp/watn-shortcut.XXXXXX) && trap 'rm -rf "$root"' EXIT && cargo build --bin watn && cp target/debug/watn "$root/default-debug" && cargo build --features test-support --bin watn && cp target/debug/watn "$root/test-support-debug" && WATN_DEFAULT_DEBUG_BIN="$root/default-debug" WATN_TEST_SUPPORT_DEBUG_BIN="$root/test-support-debug" cargo test --test features_runner --features test-support -- --name "Enter accepts the default decline for shortcut setup"
  RED evidence: the targeted runner compiled the new step modules and matched
  `interactive_shell_shortcut_steps.rs:10`; the explicit `unimplemented!()`
  body panicked with `not implemented`, the summary was `1 feature, 1 scenario
  (1 failed), 1 step (1 failed)`, and Cargo returned `error: test failed`.
  ```
- [x] Run `givn lint --change interactive-shell-shortcut`; expected result is one
  feature file with 19 `@wip` findings and no structural findings.
  ```text
  Result: exit 2 with 1 file checked and 19 expected @wip findings; no structural
  findings.
  ```

## Non-E2E Scenarios

## Scenario: Enter accepts the default decline for shortcut setup

- [x] RED: remove only this scenario's `@wip`, bind its steps to explicit
  `unimplemented!()` stubs, and run the single-scenario command. Expected
  non-zero result. Evidence:
  ```text
  The targeted runner matched the explicit `unimplemented!()` stub at
  `tests/steps/interactive_shell_shortcut_steps.rs:10`, reported `1 feature, 1
  scenario (1 failed), 1 step (1 failed)`, and returned non-zero.
  ```
- [x] GREEN: implement the default-decline setup/file assertions and the
  minimum optional-question state needed to leave all targets byte-for-byte
  unchanged. Production file: `src/shell_shortcut.rs`.
  Test files: `tests/steps/interactive_shell_shortcut_steps.rs`.
  Targeted result:
  ```text
  Targeted runner reported `1 feature, 1 scenario (1 passed), 4 steps (4
  passed)` after compiling the new `shell_shortcut` module and fixture steps.
  ```
- [x] REFACTOR: remove duplication without changing behavior and rerun the
  targeted scenario.
  ```text
  `cargo fmt --all` passed; the targeted runner again reported `1 feature, 1
  scenario (1 passed), 4 steps (4 passed)`.
  ```
- [x] COMMIT: record the scenario commit hash.
  ```text
  commit: 3b17d0b
  ```

## Scenario: Selecting no shells leaves shell configuration unchanged

- [x] RED: remove only this scenario's `@wip`, use explicit RED stubs, and run
  the targeted regular scenario. Evidence:
  ```text
  The targeted runner matched the explicit `unimplemented!()` step at
  `tests/steps/interactive_shell_shortcut_steps.rs:69`, reported `1 feature, 1
  scenario (1 failed), 3 steps (2 passed, 1 failed)`, and returned non-zero.
  ```
- [x] GREEN: implement empty multi-selection handling and byte-for-byte target
  snapshots. Production file reused: `src/shell_shortcut.rs`.
  Test files: `tests/steps/interactive_shell_shortcut_steps.rs`.
  ```text
  Targeted runner reported `1 feature, 1 scenario (1 passed), 5 steps (5
  passed)` with the empty install report and unchanged snapshots.
  ```
- [x] REFACTOR: rerun the targeted scenario after cleanup.
  ```text
  `cargo fmt --all` passed; the targeted runner again reported `1 feature, 1
  scenario (1 passed), 5 steps (5 passed)`.
  ```
- [x] COMMIT: record the scenario commit hash.
  ```text
  commit: e760ae2
  ```

## Scenario: The shell basename alone controls shortcut preselection

- [x] RED: remove only this scenario's `@wip`, bind explicit stubs, and run the
  targeted regular scenario. Evidence:
  ```text
  The targeted runner matched the explicit `unimplemented!()` step at
  `tests/steps/interactive_shell_shortcut_steps.rs:84`, reported `1 feature, 1
  scenario (1 failed), 1 step (1 failed)`, and returned non-zero.
  ```
- [x] GREEN: implement basename-only `SHELL` detection and unconstrained
  multi-selection. Production file reused: `src/shell_shortcut.rs`.
  Test files: `tests/steps/interactive_shell_shortcut_steps.rs`.
  ```text
  Targeted runner reported `1 feature, 1 scenario (1 passed), 7 steps (7
  passed)` with `/usr/local/bin/bash` selecting only Bash and later manual
  selection adding Zsh and Fish.
  ```
- [x] REFACTOR: rerun the targeted scenario.
  ```text
  `cargo fmt --all` passed; the targeted runner again reported `1 feature, 1
  scenario (1 passed), 7 steps (7 passed)`.
  ```
- [x] COMMIT: record the scenario commit hash.
  ```text
  commit: ba5c4ba
  ```

## Scenario: Multiple selected shells are installed independently

- [x] RED: remove only this scenario's `@wip`, bind explicit stubs, and run the
  targeted regular scenario. Evidence:
  ```text
  The targeted runner matched the explicit `unimplemented!()` step at
  `tests/steps/interactive_shell_shortcut_steps.rs:154`, reported `1 feature, 1
  scenario (1 failed), 1 step (1 failed)`, and returned non-zero.
  ```
- [x] GREEN: add Bash, Zsh, and Fish target resolution, native generated blocks,
  and success/reload reports. Production files: `src/shell_shortcut.rs`.
  Test files: `tests/steps/interactive_shell_shortcut_steps.rs`.
  ```text
  Targeted runner reported `1 feature, 1 scenario (1 passed), 7 steps (7
  passed)` and verified the Bash, Zsh, and Fish native block contracts and
  per-shell reports.
  ```
- [x] REFACTOR: rerun the targeted scenario and keep generation centralized.
  ```text
  `cargo fmt --all` passed; the targeted runner again reported `1 feature, 1
  scenario (1 passed), 7 steps (7 passed)`.
  ```
- [x] COMMIT: record the scenario commit hash.
  ```text
  commit: 023ae34
  ```

## Scenario: A partial multi-shell failure reports every result without rollback

- [x] RED: remove only this scenario's `@wip`, bind explicit stubs, and run the
  targeted regular scenario. Evidence:
  ```text
  The targeted runner matched the explicit `unimplemented!()` step at
  `tests/steps/interactive_shell_shortcut_steps.rs:240`, reported `1 feature, 1
  scenario (1 failed), 1 step (1 failed)`, and returned non-zero.
  ```
- [x] GREEN: attempt all selected targets, retain successes, collect failures,
  and return an aggregate error. Production files: `src/shell_shortcut.rs`,
  `src/setup.rs`, `src/main.rs`.
  Test files: `tests/steps/interactive_shell_shortcut_steps.rs`.
  ```text
  Targeted runner reported `1 feature, 1 scenario (1 passed), 10 steps (10
  passed)`; Bash and Fish were retained, the Zsh directory target was unchanged,
  and the aggregate error/report contract passed.
  ```
- [x] REFACTOR: rerun the targeted scenario and preserve report ordering.
  ```text
  `cargo fmt --all` passed; the targeted runner again reported `1 feature, 1
  scenario (1 passed), 10 steps (10 passed)`.
  ```
- [x] COMMIT: record the scenario commit hash.
  ```text
  commit: 74fd6fa
  ```

## Scenario: Missing parent directories are created only for selected shells

- [x] RED: remove only this scenario's `@wip`, bind explicit stubs, and run the
  targeted regular scenario. Evidence:
  ```text
  The targeted runner matched the explicit `unimplemented!()` step at
  `tests/steps/interactive_shell_shortcut_steps.rs:350`, reported `1 feature, 1
  scenario (1 failed), 1 step (1 failed)`, and returned non-zero.
  ```
- [x] GREEN: delay parent creation until after selection and target validation.
  Production files: `src/shell_shortcut.rs`.
  Test files: `tests/steps/interactive_shell_shortcut_steps.rs`.
  ```text
  Targeted runner reported `1 feature, 1 scenario (1 passed), 4 steps (4
  passed)`; selecting Fish created only the isolated XDG Fish parent and left
  the Bash HOME parent absent.
  ```
- [x] REFACTOR: rerun the targeted scenario.
  ```text
  `cargo fmt --all` passed; the targeted runner again reported `1 feature, 1
  scenario (1 passed), 4 steps (4 passed)`.
  ```
- [x] COMMIT: record the scenario commit hash.
  ```text
  commit: df7fa8d
  ```

## Scenario: Installing again replaces the generated block without disturbing user content

- [x] RED: remove only this scenario's `@wip`, bind explicit stubs, and run the
  targeted regular scenario. Evidence:
  ```text
  The targeted runner matched the explicit `unimplemented!()` step at
  `tests/steps/interactive_shell_shortcut_steps.rs:404`, reported `1 feature, 1
  scenario (1 failed), 1 step (1 failed)`, and returned non-zero.
  ```
- [x] GREEN: implement exact single-block replacement while preserving unrelated
  bytes. Production files: `src/shell_shortcut.rs`.
  Test files: `tests/steps/interactive_shell_shortcut_steps.rs`.
  ```text
  Targeted runner reported `1 feature, 1 scenario (1 passed), 4 steps (4
  passed)` and verified one marker pair plus preserved content before and after
  the generated block.
  ```
- [x] REFACTOR: rerun the targeted scenario.
  ```text
  `cargo fmt --all` passed; the targeted runner again reported `1 feature, 1
  scenario (1 passed), 4 steps (4 passed)`.
  ```
- [x] COMMIT: record the scenario commit hash.
  ```text
  commit: f45b663
  ```

## Scenario: A shell configuration failure reports the exact target and reason

- [x] RED: remove only this scenario's `@wip`, bind explicit stubs, and run the
  targeted regular scenario. Evidence:
  ```text
  The targeted runner matched the explicit `unimplemented!()` step at
  `tests/steps/interactive_shell_shortcut_steps.rs:451`, reported `1 feature, 1
  scenario (1 failed), 1 step (1 failed)`, and returned non-zero.
  ```
- [x] GREEN: map target resolution/read/write failures to actionable errors with
  exact paths and preserve the failed target. Production files:
  `src/shell_shortcut.rs`, `src/error.rs`.
  Test files: `tests/steps/interactive_shell_shortcut_steps.rs`.
  ```text
  Targeted runner reported `1 feature, 1 scenario (1 passed), 6 steps (6
  passed)`; the directory target path and `target is a directory` reason were
  reported and the target remained unchanged.
  ```
- [x] REFACTOR: rerun the targeted scenario.
  ```text
  `cargo fmt --all` passed; the targeted runner again reported `1 feature, 1
  scenario (1 passed), 6 steps (6 passed)`.
  ```
- [x] COMMIT: record the scenario commit hash.
  ```text
  commit: 3c97a5a
  ```

## Scenario: Invalid marker layouts fail before any target write

- [x] RED: remove only this scenario's `@wip`, bind explicit stubs, and run the
  targeted table scenario. Evidence:
  ```text
  The targeted runner matched the explicit `unimplemented!()` step at
  `tests/steps/interactive_shell_shortcut_steps.rs:517`, reported `1 feature, 1
  scenario (1 failed), 1 step (1 failed)`, and returned non-zero.
  ```
- [x] GREEN: count exact markers, reject duplicate/unmatched/reversed layouts
  before filesystem mutation, and preserve snapshots. Production files:
  `src/shell_shortcut.rs`.
  Test files: `tests/steps/interactive_shell_shortcut_steps.rs`.
  ```text
  Targeted runner reported `1 feature, 1 scenario (1 passed), 4 steps (4
  passed)` across all six malformed marker rows; every target matched its
  snapshot and reported malformed markers.
  ```
- [x] REFACTOR: rerun the targeted table scenario.
  ```text
  `cargo fmt --all` passed; the targeted runner again reported `1 feature, 1
  scenario (1 passed), 4 steps (4 passed)`.
  ```
- [x] COMMIT: record the scenario commit hash.
  ```text
  commit: 6c8c0ee
  ```

## Scenario: Generated shell blocks use the installed watn command and preserve shell syntax

- [x] RED: remove only this scenario's `@wip`, bind explicit stubs, and run the
  targeted regular scenario. Evidence:
  ```text
  The targeted runner matched the explicit `unimplemented!()` step at
  `tests/steps/interactive_shell_shortcut_steps.rs:601`, reported `1 feature, 1
  scenario (1 failed), 1 step (1 failed)`, and returned non-zero.
  ```
- [x] GREEN: complete the native Bash/Zsh/Fish block text, PATH invocation, and
  binding contracts. Production files: `src/shell_shortcut.rs`.
  Test files: `tests/steps/interactive_shell_shortcut_steps.rs`.
  ```text
  Targeted runner reported `1 feature, 1 scenario (1 passed), 8 steps (8
  passed)` and verified the three native line-editor contracts and exact
  `command watn -- "$question"` invocation.
  ```
- [x] REFACTOR: rerun the targeted scenario and any available shell parser
  checks.
  ```text
  `cargo fmt --all` passed; the targeted runner again reported `1 feature, 1
  scenario (1 passed), 8 steps (8 passed)`.
  ```
- [x] COMMIT: record the scenario commit hash.
  ```text
  commit: 3781328
  ```

## Scenario: A successful widget inserts one normalized command and moves the cursor to its end

- [x] RED: remove only this scenario's `@wip`, bind explicit stubs, and run the
  targeted regular scenario. Evidence:
  ```text
  The targeted runner matched the explicit `unimplemented!()` step at
  `tests/steps/interactive_shell_shortcut_steps.rs:601`, reported `1 feature, 1
  scenario (1 failed), 1 step (1 failed)`, and returned non-zero.
  ```
- [x] GREEN: implement Bash capture, trailing CR/LF normalization, status check,
  buffer replacement, and cursor movement. Production files:
  `src/shell_shortcut.rs`.
  Test files: `tests/steps/interactive_shell_shortcut_steps.rs`.
  ```text
  Targeted runner reported `1 feature, 1 scenario (1 passed), 4 steps (4
  passed)`; trailing output newlines were removed and the Bash cursor matched the
  inserted command length.
  ```
- [x] REFACTOR: rerun the targeted scenario.
  ```text
  `cargo fmt --all` passed; the targeted runner again reported `1 feature, 1
  scenario (1 passed), 4 steps (4 passed)`.
  ```
- [x] COMMIT: record the scenario commit hash.
  ```text
  commit: b259c06
  ```

## Scenario: Embedded multiline output remains buffer text without evaluation

- [x] RED: remove only this scenario's `@wip`, bind explicit stubs, and run the
  targeted regular scenario. Evidence:
  ```text
  The targeted runner matched the explicit `unimplemented!()` step at
  `tests/steps/interactive_shell_shortcut_steps.rs:713`, reported `1 feature, 1
  scenario (1 failed), 4 steps (3 passed, 1 failed)`, and returned non-zero.
  ```
- [x] GREEN: preserve embedded line breaks as assigned text and prove the
  no-evaluation boundary. Production files: `src/shell_shortcut.rs`.
  Test files: `tests/steps/interactive_shell_shortcut_steps.rs`.
  ```text
  Targeted runner reported `1 feature, 1 scenario (1 passed), 6 steps (6
  passed)`; the embedded line break remained in the Bash buffer and the
  replacement sentinel was not created.
  ```
- [x] REFACTOR: rerun the targeted scenario.
  ```text
  `cargo fmt --all` passed; the targeted runner again reported `1 feature, 1
  scenario (1 passed), 6 steps (6 passed)`.
  ```
- [x] COMMIT: record the scenario commit hash.
  ```text
  commit: a9d71a0
  ```

## Scenario: Empty input does not invoke watn or change the command line

- [x] RED: remove only this scenario's `@wip`, bind explicit stubs, and run the
  targeted regular scenario. Evidence:
  ```text
  The targeted runner matched the explicit `unimplemented!()` step at
  `tests/steps/interactive_shell_shortcut_steps.rs:722`, reported `1 feature, 1
  scenario (1 failed), 1 step (1 failed)`, and returned non-zero.
  ```
- [x] GREEN: short-circuit empty Bash input before spawning `watn`. Production
  files: `src/shell_shortcut.rs`.
  Test files: `tests/steps/interactive_shell_shortcut_steps.rs`.
  ```text
  Targeted runner reported `1 feature, 1 scenario (1 passed), 4 steps (4
  passed)`; the invocation log remained absent and the empty buffer was
  preserved.
  ```
- [x] REFACTOR: rerun the targeted scenario.
  ```text
  `cargo fmt --all` passed; the targeted runner again reported `1 feature, 1
  scenario (1 passed), 4 steps (4 passed)`.
  ```
- [x] COMMIT: record the scenario commit hash.
  ```text
  commit: e530d75
  ```

## Scenario: Failed or empty output preserves the original command line

- [x] RED: remove only this scenario's `@wip`, bind explicit stubs, and run the
  targeted regular scenario. Evidence:
  ```text
  The targeted runner matched the explicit `unimplemented!()` step at
  `tests/steps/interactive_shell_shortcut_steps.rs:769`, reported `1 feature, 1
  scenario (1 failed), 1 step (1 failed)`, and returned non-zero.
  ```
- [x] GREEN: preserve the original buffer on failed or empty normalized output
  while repainting. Production files: `src/shell_shortcut.rs`.
  Test files: `tests/steps/interactive_shell_shortcut_steps.rs`.
  ```text
  Targeted runner reported `1 feature, 1 scenario (1 passed), 6 steps (6
  passed)` for failed output and subsequent empty output; both original buffers
  were preserved.
  ```
- [x] REFACTOR: rerun the targeted scenario.
  ```text
  `cargo fmt --all` passed; the targeted runner again reported `1 feature, 1
  scenario (1 passed), 6 steps (6 passed)`.
  ```
- [x] COMMIT: record the scenario commit hash.
  ```text
  commit: 32a2198
  ```

## Scenario: Non-zero watn status discards partial stdout

- [x] RED: remove only this scenario's `@wip`, bind explicit stubs, and run the
  targeted regular scenario. Evidence:
  ```text
  The targeted runner matched the explicit `unimplemented!()` step at
  `tests/steps/interactive_shell_shortcut_steps.rs:795`, reported `1 feature, 1
  scenario (1 failed), 1 step (1 failed)`, and returned non-zero.
  ```
- [x] GREEN: check process status before assigning captured stdout. Production
  files: `src/shell_shortcut.rs`.
  Test files: `tests/steps/interactive_shell_shortcut_steps.rs`.
  ```text
  Targeted runner reported `1 feature, 1 scenario (1 passed), 4 steps (4
  passed)`; non-zero status preserved the original line and discarded partial
  stdout.
  ```
- [x] REFACTOR: rerun the targeted scenario.
  ```text
  `cargo fmt --all` passed; the targeted runner again reported `1 feature, 1
  scenario (1 passed), 4 steps (4 passed)`.
  ```
- [x] COMMIT: record the scenario commit hash.
  ```text
  commit: cf1734f
  ```

## Scenario: The complete command line is passed as one quoted question

- [x] RED: remove only this scenario's `@wip`, bind explicit stubs, and run the
  targeted regular scenario. Evidence:
  ```text
  The targeted runner matched the explicit `unimplemented!()` step at
  `tests/steps/interactive_shell_shortcut_steps.rs:811`, reported `1 feature, 1
  scenario (1 failed), 1 step (1 failed)`, and returned non-zero.
  ```
- [x] GREEN: assert shell metacharacters and spaces reach the fake `watn` as one
  argument. Production files: `src/shell_shortcut.rs`.
  Test files: `tests/steps/interactive_shell_shortcut_steps.rs`.
  ```text
  Targeted runner reported `1 feature, 1 scenario (1 passed), 4 steps (4
  passed)` and the log contained exactly one unexpanded question argument.
  ```
- [x] REFACTOR: rerun the targeted scenario.
  ```text
  `cargo fmt --all` passed; the targeted runner again reported `1 feature, 1
  scenario (1 passed), 4 steps (4 passed)`.
  ```
- [x] COMMIT: record the scenario commit hash.
  ```text
  commit: b6dfcaa
  ```

## Scenario: Leading-option and reserved-token questions remain one argument

- [x] RED: remove only this scenario's `@wip`, bind explicit stubs, and run the
  targeted regular scenario. Evidence:
  ```text
  The targeted runner matched the explicit `unimplemented!()` step at
  `tests/steps/interactive_shell_shortcut_steps.rs:837`, reported `1 feature, 1
  scenario (1 failed), 1 step (1 failed)`, and returned non-zero.
  ```
- [x] GREEN: use `--` in every generated invocation and assert `--help` and
  `completions find files` remain questions. Production files:
  `src/shell_shortcut.rs`.
  Test files: `tests/steps/interactive_shell_shortcut_steps.rs`.
  ```text
  Targeted runner reported `1 feature, 1 scenario (1 passed), 5 steps (5
  passed)` and recorded both leading-option and reserved-token questions as
  separate arguments.
  ```
- [x] REFACTOR: rerun the targeted scenario.
  ```text
  `cargo fmt --all` passed; the targeted runner again reported `1 feature, 1
  scenario (1 passed), 5 steps (5 passed)`.
  ```
- [ ] COMMIT: record the scenario commit hash.
  ```text
  commit: pending
  ```

## Scenario: Setup reports the exact reload instruction for every modified shell

- [ ] RED: remove only this scenario's `@wip`, bind explicit stubs, and run the
  targeted regular scenario. Evidence:
  ```text
  pending
  ```
- [ ] GREEN: return stable shell-specific reload instructions for Bash, Zsh, and
  Fish. Production files: `src/shell_shortcut.rs`, `src/main.rs`.
  Test files: `tests/steps/interactive_shell_shortcut_steps.rs`.
  ```text
  pending
  ```
- [ ] REFACTOR: rerun the targeted scenario.
  ```text
  pending
  ```
- [ ] COMMIT: record the scenario commit hash.
  ```text
  commit: pending
  ```

## E2E Scenarios

## Scenario: Generated Bash and Fish configurations pass shell syntax checks

- [ ] RED: remove only this scenario's `@wip`, bind explicit E2E stubs in
  `tests/steps/interactive_shell_shortcut_e2e_steps.rs`, and run the targeted
  `./run-tests.sh --e2e` scenario. Evidence:
  ```text
  pending
  ```
- [ ] GREEN: install isolated Bash, Zsh, and Fish targets, run `bash -n` and
  `fish -n` against the generated configurations, and assert both parser
  processes exit successfully. Production file reused: `src/shell_shortcut.rs`.
  Test files: `tests/steps/interactive_shell_shortcut_e2e_steps.rs`.
  ```text
  pending
  ```
- [ ] REFACTOR: rerun the targeted E2E scenario and keep parser exit status as
  the primary assertion.
  ```text
  pending
  ```
- [ ] COMMIT: record the E2E scenario commit hash.
  ```text
  commit: pending
  ```

## Scenario: The generated Bash widget runs through Bash without evaluating its result

- [ ] RED: remove only this scenario's `@wip`, bind explicit E2E stubs, and run
  the targeted `./run-tests.sh --e2e` scenario. Evidence:
  ```text
  pending
  ```
- [ ] GREEN: source the generated block through
  `bash --noprofile --norc -c`, supply a fake `watn` on `PATH`, and assert the
  replacement buffer plus no-evaluation sentinel. Production file reused:
  `src/shell_shortcut.rs`.
  Test files: `tests/steps/interactive_shell_shortcut_e2e_steps.rs`.
  ```text
  pending
  ```
- [ ] REFACTOR: rerun the targeted E2E scenario and retain the real Bash
  process assertions.
  ```text
  pending
  ```
- [ ] COMMIT: record the E2E scenario commit hash.
  ```text
  commit: pending
  ```

## Final Verification

- [ ] Remove all completed scenario `@wip` tags and run `givn lint --change
  interactive-shell-shortcut` clean.
- [ ] Run `./run-tests.sh` and record the complete scenario/step count.
- [ ] Run `./run-tests.sh --e2e` and record a strictly smaller scenario count.
- [ ] Run formatting, compilation, clippy, all-target tests, docs, release
  build, coverage measurement/merge, and `git diff --check`.
- [ ] Complete review and archive; record the archive commit hash.
