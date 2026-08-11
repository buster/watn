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

## Scenario: Unsupported shell returns actionable guidance

- [ ] RED: Remove only this scenario's `@wip`, bind explicit stubs, and target
  it. Expected non-zero result. Evidence:
  ```text
  [targeted runner output]
  ```
- [ ] GREEN: Implement the closed selector error containing the rejected value,
  literal `unsupported shell`, and supported values bash/zsh/fish. Assert
  non-zero status and stderr contract. Production files: [list]. Test files:
  [list]. Targeted result:
  ```text
  [targeted runner output]
  ```
- [ ] REFACTOR: Preserve the normal Clap argument-error framing while keeping
  the local literal contract. Targeted rerun:
  ```text
  [targeted runner output]
  ```
- [ ] COMMIT: `[hash]` - `feat(shell-completions): Unsupported shell returns actionable guidance`

## Scenario: Completion generation does not load configuration or contact a provider

- [ ] RED: Remove only this scenario's `@wip`, bind explicit stubs, and target
  it. Expected non-zero result. Evidence:
  ```text
  [targeted runner output]
  ```
- [ ] GREEN: Create isolated XDG state and an observable provider sentinel,
  invoke completion generation, and assert no config file or other file is
  created, no provider request occurs, stderr is empty, and stdout contains only
  the script. Production files: [list]. Test files: [list]. Targeted result:
  ```text
  [targeted runner output]
  ```
- [ ] REFACTOR: Keep before/after snapshots deterministic and avoid relying on
  provider configuration as the primary assertion. Targeted rerun:
  ```text
  [targeted runner output]
  ```
- [ ] COMMIT: `[hash]` - `feat(shell-completions): Completion generation does not load configuration or contact a provider`

## Scenario: Completion help documents the supported selector and output contract

- [ ] RED: Remove only this scenario's `@wip`, bind explicit stubs, and target
  it. Expected non-zero result. Evidence:
  ```text
  [targeted runner output]
  ```
- [ ] GREEN: Add command and shell-argument help text covering exact usage,
  supported shells, and stdout install/source purpose. Assert stdout-only help
  and exit 0. Production files: [list]. Test files: [list]. Targeted result:
  ```text
  [targeted runner output]
  ```
- [ ] REFACTOR: Keep help metadata in the authoritative command definition and
  remove duplicated help strings. Targeted rerun:
  ```text
  [targeted runner output]
  ```
- [ ] COMMIT: `[hash]` - `docs(shell-completions): Completion help documents the supported selector and output contract`

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

- [ ] RED: Remove only this scenario's `@wip`, bind the E2E stub, and target it
  through `./run-tests.sh --e2e`/the single-scenario equivalent. Expected
  non-zero result. Evidence:
  ```text
  [targeted E2E output]
  ```
- [ ] GREEN: Invoke the explicit built binary through a real subprocess and
  assert Bash script output, exact root tree, selector suggestions, stdout-only
  output, deterministic second generation, Bash parser acceptance, and status
  0. Production files: none beyond the regular implementation. Test files:
  [list]. Targeted result:
  ```text
  [targeted E2E output]
  ```
- [ ] REFACTOR: Keep the E2E output assertions primary and distinct from the
  regular shell variants. Targeted rerun:
  ```text
  [targeted E2E output]
  ```
- [ ] COMMIT: `[hash]` - `test(e2e): Built Bash completion generation emits the current command tree`

## Final Change Verification

- [ ] Remove all completed scenario `@wip` tags and run
  `givn lint --change shell-completions`.
  ```text
  Result: [output]
  ```
- [ ] Run `./run-tests.sh` and record the full scenario/step count.
  ```text
  Result: [output]
  ```
- [ ] Run `./run-tests.sh --e2e` and prove the count is strictly smaller.
  ```text
  Result: [output]
  ```
- [ ] Run formatting, compilation, clippy, explicit-binary all-target tests,
  docs, release build, coverage measurement/merge, and `git diff --check`.
  ```text
  Result: [output]
  ```
