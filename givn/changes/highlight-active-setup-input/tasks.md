# Tasks: Highlight Active Setup Input

## Setup

- [x] Configure and prove the strict Cucumber/PTY test path.
  - Confirm `tests/features_runner.rs` uses `Cucumber::fail_on_skipped()` and
    that the configured `verify.command` and `verify.e2e_command` execute the
    repository and change `.feature` files through `features_runner`.
  - Create `tests/steps/highlight_active_setup_input_steps.rs` as the
    capability-specific step-definition file and register it from
    `tests/steps/mod.rs`.
  - Use `unimplemented!()` for the first new assertion step, remove `@wip`
    from only `The initial URL input has a green border`, and run the exact
    single-scenario E2E command. Record the required non-zero result as the
    proof that undefined/pending behavior cannot pass.
  - Run the configured regular and E2E commands and record their scenario
    counts. The E2E count must be smaller because the full suite includes
    non-E2E scenarios.
  - Start the local test environment through the configured E2E command: the
    PTY starts `watn`, and the scenario harness starts the loopback `httpmock`
    catalog twin. The PTY child removes inherited `NO_COLOR` and sets
    `TERM=xterm-256color`; no live provider or Docker service is allowed.
  - Evidence: `Cucumber::fail_on_skipped()` is present in
    `tests/features_runner.rs`. The corrected targeted bootstrap command
    produced `1 scenario (1 failed)` and `4 steps (3 passed, 1 failed)` with
    the captured `not implemented` panic from the new URL assertion. The
    configured regular command passed `93 scenarios` and `543 steps`. The
    configured E2E command selected `62 scenarios` and `411 steps` (60 passed,
    2 failed: the new stub and the pre-existing release-version mismatch), so
    the E2E filter is a strict subset. The PTY removed inherited `NO_COLOR`,
    set `TERM=xterm-256color`, and the existing `httpmock` twin was the only
    catalog dependency.

## Scenario: The initial URL input has a green border

- [x] RED: Remove `@wip` from this scenario only, implement its new step with
  the not-implemented stub, and run:

  ```text
  root=$(mktemp -d /tmp/watn-transport.XXXXXX) && trap 'rm -rf "$root"' EXIT && cargo build --locked --bin watn && cp target/debug/watn "$root/default-debug" && cargo build --locked --features test-support --bin watn && cp target/debug/watn "$root/test-support-debug" && WATN_DEFAULT_DEBUG_BIN="$root/default-debug" WATN_TEST_SUPPORT_DEBUG_BIN="$root/test-support-debug" cargo test --locked --test features_runner --features test-support -- --name 'The initial URL input has a green border'
  ```

  Expected result: non-zero. Evidence: the full-bootstrap run reached the
  real PTY setup, passed the three setup steps, then failed the new assertion
  with `Step panicked. Captured output: not implemented`; summary was `1
  scenario (1 failed)` and `4 steps (3 passed, 1 failed)`.

- [x] GREEN: Replace the stub with a real PTY assertion that the latest
  `URL (editing)` border line carries the green SGR. Add the minimum production
  rendering change needed to style the active URL block. Compile first, then
  run the single-scenario command and record a zero exit. Production files:
  `src/setup.rs`; test file: `tests/steps/highlight_active_setup_input_steps.rs`.
  Evidence: the full-bootstrap targeted command passed with `1 scenario`, `4
  steps`, and zero failures. The assertion reconstructed the real 120x40 PTY
  screen and verified the URL border's semantic green foreground. Production
  file changed: `src/setup.rs`; test file changed:
  `tests/steps/highlight_active_setup_input_steps.rs`.

- [x] REFACTOR: Keep behavior unchanged, centralize any shared frame-local ANSI
  parsing needed by later scenarios, rerun the single-scenario command, and
  record the zero exit. Evidence: `cargo fmt --all -- --check` passed and the
  same full-bootstrap targeted command passed with `1 scenario`, `4 steps`.

- [x] COMMIT: Create one atomic commit for this scenario with a message that
  references `The initial URL input has a green border` verbatim. Record the
  hash here: `7cc4423`.

## Scenario: The green border follows API key focus

- [x] RED: Remove `@wip` from this scenario only, add real step bindings with
  `unimplemented!()` for new assertions, and run the E2E command targeted to
  `The green border follows API key focus`. Expected result: non-zero.
  Evidence: the targeted full-bootstrap run reached the API-key page, passed
  four setup steps, and failed the new credential assertion with `not
  implemented`; summary was `1 scenario (1 failed)` and `5 steps (4 passed, 1
  failed)`.

- [x] GREEN: Assert through the live PTY that the storage list is green when it
  owns focus, the API key value block becomes green after `p`, and the inactive
  credential block has no green SGR. Apply the minimum corresponding
  `src/setup.rs` rendering change. Run the targeted E2E scenario and record a
  zero exit. Production files: `src/setup.rs`; test file:
  `tests/steps/highlight_active_setup_input_steps.rs`. Evidence: the targeted
  full-bootstrap run passed `1 scenario` and `9 steps`, including green storage
  and value borders plus both inactive baseline assertions.

- [x] REFACTOR: Remove duplication in the titled-border/style assertion helper
  without changing behavior, rerun the targeted scenario, and record a zero
  exit. Evidence: `cargo fmt --all -- --check` passed and the targeted
  full-bootstrap run passed `1 scenario` and `9 steps`.

- [x] COMMIT: Create one atomic commit for this scenario with a message that
  references `The green border follows API key focus` verbatim. Record the
  hash here: `b35ce07`.

## Scenario: The green border follows model focus

- [x] RED: Remove `@wip` from this scenario only, bind any new toggle/assertion
  steps with `unimplemented!()`, and run the E2E command targeted to
  `The green border follows model focus`. Expected result: non-zero.
  Evidence: the targeted full-bootstrap run passed seven setup/catalog steps
  and failed the model-border assertion with `not implemented`; summary was
  `1 scenario (1 failed)` and `8 steps (7 passed, 1 failed)`.

- [x] GREEN: Assert through the live PTY that the model table border is green
  initially, Ctrl-R moves green to the reasoning block, and the inactive model
  table has no green SGR. Apply the minimum corresponding `src/setup.rs`
  rendering change. Run the targeted E2E scenario and record a zero exit.
  Production files: `src/setup.rs`; test file:
  `tests/steps/highlight_active_setup_input_steps.rs`. Evidence: the targeted
  full-bootstrap run passed `1 scenario` and `12 steps`, including model-table
  and reasoning green borders plus both inactive baseline assertions.

- [x] REFACTOR: Keep the focus-to-border mapping explicit and share only the
  ANSI inspection logic, rerun the targeted scenario, and record a zero exit.
  Evidence: `cargo fmt --all -- --check` passed and the targeted full-bootstrap
  run passed `1 scenario` and `12 steps`.

- [x] COMMIT: Create one atomic commit for this scenario with a message that
  references `The green border follows model focus` verbatim. Record the hash
  here: `bcfcd4d`.

## Scenario: The green border follows optional shortcut focus

- [x] RED: Remove `@wip` from this scenario only, bind its new shortcut
  navigation/assertion steps with `unimplemented!()`, and run the E2E command
  targeted to `The green border follows optional shortcut focus`. Expected
  result: non-zero. Evidence: the targeted full-bootstrap run passed nine
  setup/model steps and failed the shortcut transition assertion with `not
  implemented`; summary was `1 scenario (1 failed)` and `10 steps (9 passed, 1
  failed)`.

- [x] GREEN: Drive the final setup page through the live PTY, assert the
  shortcut question border is green before `y`, assert the shell list border is
  green after `y`, and assert the inactive question border is not green. Apply
  the minimum corresponding `src/setup.rs` rendering change. Run the targeted
  E2E scenario and record a zero exit. Production files: `src/setup.rs`; test
  file: `tests/steps/highlight_active_setup_input_steps.rs`. Evidence: the
  targeted full-bootstrap run passed `1 scenario` and `15 steps`, including
  question/list focus and both inactive baseline assertions.

- [x] REFACTOR: Remove only assertion duplication, preserve the real PTY
  interaction, rerun the targeted scenario, and record a zero exit. Evidence:
  `cargo fmt --all -- --check` passed and the targeted full-bootstrap run
  passed `1 scenario` and `15 steps`.

- [ ] COMMIT: Create one atomic commit for this scenario with a message that
  references `The green border follows optional shortcut focus` verbatim.
  Record the hash here: pending.
