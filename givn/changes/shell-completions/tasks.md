# Tasks: shell-completions

## Setup

- [x] Register `tests/steps/shell_completions_steps.rs` and
  `tests/steps/shell_completions_e2e_steps.rs` in the runner, add the minimal
  state required for isolated config/sentinel snapshots, and retain
  `.fail_on_skipped()`. Use `unimplemented!()` for RED bodies.
  ```text
  verify.command: ./run-tests.sh
  verify.e2e_command: ./run-tests.sh --e2e
  strict proof command: root=$(mktemp -d /tmp/watn-completions.XXXXXX) && trap 'rm -rf "$root"' EXIT && cargo build --bin watn && cp target/debug/watn "$root/default-debug" && cargo build --features test-support --bin watn && cp target/debug/watn "$root/test-support-debug" && WATN_DEFAULT_DEBUG_BIN="$root/default-debug" WATN_TEST_SUPPORT_DEBUG_BIN="$root/test-support-debug" cargo test --test features_runner --features test-support -- --name "Unsupported shell returns actionable guidance"
  Result: non-zero; `tests/steps/shell_completions_steps.rs:8` matched the explicit `unimplemented!()` stub, the runner reported `1 step failed`, and Cargo returned `error: test failed`.
  ```
- [x] Run `givn lint --change shell-completions` and record the expected five
  `@wip` findings only.
  ```text
  Result: exit 2 with 1 file checked and 5 expected `@wip` findings; no structural findings.
  ```

## Non-E2E Scenarios

## Scenario: Each supported shell exposes the authoritative command tree

- [x] RED: Remove only this Scenario Outline's `@wip`, bind explicit stubs, and
  target the outline. Expected non-zero result. Evidence:
  ```text
  Targeted runner matched the Bash, Zsh, and Fish scenarios, each explicit
  `regular_completion` stub panicked with `not implemented`, and Cargo
  returned `error: test failed`.
  [Summary]
  1 feature
  3 scenarios (3 failed)
  3 steps (3 failed)
  ```
- [x] GREEN: Add the locked compatible `clap_complete` dependency, the closed
  `CompletionShell` parser, the `completions` subcommand, and early dispatch.
  Generate from `Cli::command()`. Implement Bash/Zsh/Fish regular subprocess
  assertions for all root options, positional argument, subcommands, selector
  suggestions, stdout purity, stderr emptiness, deterministic repeated output,
  and shell parser/source acceptance. Production files: [list]. Test files:
  [list]. Targeted result:
  ```text
  Production files: `Cargo.toml`, `Cargo.lock`, `src/main.rs`.
  Test files: `tests/steps/mod.rs`, `tests/steps/shell_completions_steps.rs`.
  [Summary]
  1 feature
  3 scenarios (3 passed)
  27 steps (27 passed)
  ```
- [x] REFACTOR: Keep shell mapping and output assertions centralized without
  duplicating the command tree. Targeted rerun:
  ```text
  `cargo fmt --all -- --check` passed; the targeted runner reported 1 feature,
  3 scenarios (3 passed), and 27 steps (27 passed).
  ```
- [x] COMMIT: `341671d` - `feat(shell-completions): Each supported shell exposes the authoritative command tree`

## Scenario: Every native clap_complete shell exposes the authoritative command tree

- [x] RED: Bind explicit stubs for the new Elvish and PowerShell scenarios and
  target only those scenarios. Expected non-zero result. Evidence:
  ```text
  The targeted runner matched the Elvish and PowerShell scenarios; both
  `regular_completion` stubs panicked with `not implemented`. The summary was
  1 feature, 2 scenarios (2 failed), and 2 steps (2 failed); Cargo returned
  `error: test failed`.
  ```
- [x] GREEN: Extend the local selector and renderer mapping to every native
  `clap_complete 4.6.9` shell: Bash, Elvish, Fish, PowerShell, and Zsh. Assert
  the new Elvish and PowerShell scripts, deterministic repeated output, root
  tree, stdout/stderr contracts, and parser acceptance when the executables
  are available. Production files: [list]. Test files: [list]. Targeted result:
  ```text
  Production file: `src/main.rs`. Test files:
  `tests/steps/shell_completions_steps.rs` and
  `givn/changes/shell-completions/specs/completions/completions.feature`.
  The targeted runner reported 1 feature, 5 scenarios (5 passed), and 46 steps
  (46 passed). Elvish, PowerShell, and Zsh parser availability limitations were
  reported explicitly where executables were absent.
  ```
- [x] REFACTOR: Keep the five-shell mapping and shell-parser probing centralized
  without duplicating the command tree. Targeted rerun:
  ```text
  `cargo fmt --all -- --check` passed; the targeted runner reported 1 feature,
  5 scenarios (5 passed), and 46 steps (46 passed).
  ```
- [x] COMMIT: `e811e75` - `feat(shell-completions): Every native clap_complete shell exposes the authoritative command tree`

## Scenario: Unsupported shell returns actionable guidance

- [x] RED: Remove only this scenario's `@wip`, bind explicit stubs, and target
  it. Expected non-zero result. Evidence:
  ```text
  The targeted runner matched `unsupported_completion`, which panicked with
  `not implemented`; the summary was 1 feature, 1 scenario (1 failed), and 1
  step (1 failed). Cargo returned `error: test failed`.
  ```
- [x] GREEN: Implement the closed selector error containing the rejected value,
  literal `unsupported shell`, and supported values bash/elvish/fish/powershell/zsh. Assert
  non-zero status and stderr contract. Production files: [list]. Test files:
  [list]. Targeted result:
  ```text
  Production file: `src/main.rs`; test file:
  `tests/steps/shell_completions_steps.rs`.
  The targeted runner reported 1 feature, 1 scenario (1 passed), and 4 steps
  (4 passed).
  ```
- [x] REFACTOR: Preserve the normal Clap argument-error framing while keeping
  the local literal contract. Targeted rerun:
  ```text
  `cargo fmt --all -- --check` passed; the targeted runner reported 1 feature,
  1 scenario (1 passed), and 4 steps (4 passed).
  ```
- [x] COMMIT: `48ab5ec` - `feat(shell-completions): Unsupported shell returns actionable guidance`

## Scenario: Completion generation does not load configuration or contact a provider

- [x] RED: Remove only this scenario's `@wip`, bind explicit stubs, and target
  it. Expected non-zero result. Evidence:
  ```text
  The targeted runner matched `no_provider_config`, which panicked with
  `not implemented`; the summary was 1 feature, 1 scenario (1 failed), and 1
  step (1 failed). Cargo returned `error: test failed`.
  ```
- [x] GREEN: Create isolated XDG state and an observable provider sentinel,
  invoke completion generation, and assert no config file or other file is
  created, no provider request occurs, stderr is empty, and stdout contains only
  the script. Production files: [list]. Test files: [list]. Targeted result:
  ```text
  Production files: none beyond the implementation committed in `341671d`.
  Test file: `tests/steps/shell_completions_steps.rs`.
  The targeted runner reported 1 feature, 1 scenario (1 passed), and 12 steps
  (12 passed).
  ```
- [x] REFACTOR: Keep before/after snapshots deterministic and avoid relying on
  provider configuration as the primary assertion. Targeted rerun:
  ```text
  `cargo fmt --all -- --check` passed; the isolated-directory assertion was
  made explicit and the targeted runner reported 1 feature, 1 scenario (1
  passed), and 12 steps (12 passed).
  ```
- [x] COMMIT: `2c6a483` - `feat(shell-completions): Completion generation does not load configuration or contact a provider`

## Scenario: Completion help documents the supported selector and output contract

- [x] RED: Remove only this scenario's `@wip`, bind explicit stubs, and target
  it. Expected non-zero result. Evidence:
  ```text
  The targeted runner matched `completion_help`, which panicked with
  `not implemented`; the summary was 1 feature, 1 scenario (1 failed), and 1
  step (1 failed). Cargo returned `error: test failed`.
  ```
- [x] GREEN: Add command and shell-argument help text covering exact usage,
  supported shells, and stdout install/source purpose. Assert stdout-only help
  and exit 0. Production files: [list]. Test files: [list]. Targeted result:
  ```text
  Production file: `src/main.rs`; test file:
  `tests/steps/shell_completions_steps.rs`.
  The targeted runner reported 1 feature, 1 scenario (1 passed), and 8 steps
  (8 passed).
  ```
- [x] REFACTOR: Keep help metadata in the authoritative command definition and
  remove duplicated help strings. Targeted rerun:
  ```text
  `cargo fmt --all -- --check` passed; the help wording was clarified in the
  authoritative Clap definition and the targeted runner reported 1 feature, 1
  scenario (1 passed), and 8 steps (8 passed).
  ```
- [x] COMMIT: `9b787cf` - `docs(shell-completions): Completion help documents the supported selector and output contract`

## Scenario: The reserved completion token can remain question text after `--`

- [x] RED: Bind the reserved-token subprocess step as an explicit stub and
  target only this scenario. Expected non-zero result. Evidence:
  ```text
  The targeted runner matched `reserved_completion_token`; the explicit stub
  panicked with `not implemented`. The summary was 1 feature, 1 scenario (1
  failed), and 2 steps (1 passed, 1 failed); Cargo returned `error: test failed`.
  ```
- [x] GREEN: Assert that `watn -- completions find files` reaches the normal
  question path, returns successfully through the configured test provider,
  emits the generated answer, and does not emit Bash completion syntax.
  Production files: none beyond the early-dispatch implementation. Test files:
  [list]. Targeted result:
  ```text
  Production files: none beyond the early-dispatch implementation. Test files:
  `tests/steps/shell_completions_steps.rs` and
  `givn/changes/shell-completions/specs/completions/completions.feature`.
  The targeted runner reported 1 feature, 1 scenario (1 passed), and 9 steps
  (9 passed), covering both the `--` and quoted escape forms.
  ```
- [x] REFACTOR: Keep the reserved-token invocation isolated from completion
  generation fixtures. Targeted rerun:
  ```text
  `cargo fmt --all -- --check` passed; the targeted runner reported 1 feature,
  1 scenario (1 passed), and 9 steps (9 passed).
  ```
- [x] COMMIT: `95381a9` - `feat(shell-completions): The reserved completion token can remain question text after --`

## E2E Setup

- [x] Confirm the local environment needs no external service, register the
  separate E2E step module, and prove the E2E wrapper is a strict subset:
  ```text
  ./run-tests.sh: 16 features, 71 scenarios, 415 steps
  ./run-tests.sh --e2e: 19 features, 59 scenarios, 399 steps
  Result: E2E count is strictly smaller: yes
  ```

## E2E Scenario

## Scenario: Built Bash completion generation emits the current command tree

- [x] RED: Remove only this scenario's `@wip`, bind the E2E stub, and target it
  through `./run-tests.sh --e2e`/the single-scenario equivalent. Expected
  non-zero result. Evidence:
  ```text
  The single-scenario runner matched `tests/steps/shell_completions_e2e_steps.rs:6`,
  where the explicit stub panicked with `not implemented`. The summary was 1
  feature, 1 scenario (1 failed), and 1 step (1 failed); Cargo returned
  `error: test failed`.
  ```
- [x] GREEN: Invoke the explicit built binary through a real subprocess and
  assert Bash script output, exact root tree, all five selector suggestions, stdout-only
  output, deterministic second generation, Bash parser acceptance, and status
  0. Production files: none beyond the regular implementation. Test files:
  [list]. Targeted result:
  ```text
  Production files: none beyond the regular implementation. Test files:
  `tests/steps/shell_completions_e2e_steps.rs` and
  `tests/steps/shell_completions_steps.rs`.
  The targeted runner reported 1 feature, 1 scenario (1 passed), and 9 steps
  (9 passed).
  ```
- [x] REFACTOR: Keep the E2E output assertions primary and distinct from the
  regular shell variants. Targeted rerun:
  ```text
  `cargo fmt --all -- --check` passed; the generated Bash-function assertion
  remained primary and the targeted runner reported 1 feature, 1 scenario (1
  passed), and 9 steps (9 passed).
  ```
- [ ] COMMIT: `[hash]` - `test(e2e): Built Bash completion generation emits the current command tree`

## Final Change Verification

- [ ] Remove all completed scenario `@wip` tags and run
  `givn lint --change shell-completions`.
  ```text
  Result: [output after scope expansion]
  ```
- [ ] Run `./run-tests.sh` and record the full scenario/step count.
  ```text
  Result: [output after scope expansion]
  ```
- [ ] Run `./run-tests.sh --e2e` and prove the count is strictly smaller.
  ```text
  Result: [output after scope expansion]
  ```
- [ ] Run formatting, compilation, clippy, explicit-binary all-target tests,
  docs, release build, coverage measurement/merge, and `git diff --check`.
  ```text
  Result: [output after scope expansion]
  ```
