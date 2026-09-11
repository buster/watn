# Tasks: quicksetup-parameters

One atomic commit per scenario (RED + GREEN + REFACTOR together). Tick each box
immediately when its evidence is recorded; never batch-check.

Use-case constraints for every scenario: quick setup stays interactive and
local-only; parameters are suggestions, never silent writes; the shell question
always runs; cancellation writes nothing.

## Setup

- [x] Record the baseline before any change.
  ```text
  regular output/count: exit 0; 21 features; 220 scenarios (220 passed); 1348 steps (1348 passed)
  e2e output/count: exit 0; 25 features; 86 scenarios (86 passed); 639 steps (639 passed)
  lib output/count: exit 0; 84 passed; 0 failed
  ```

- [x] Prove strict mode again: remove `@wip` from the help scenario only and run
  it; the undefined step must fail the scenario. Restore `@wip`.
  ```text
  command: `./run-tests.sh --name 'Quick setup documents its parameters in help'`
  output: <paste>
  ```

## Scenarios

### Quick setup documents its parameters in help

Domain constraints: the parameters are discoverable from the command surface.

- [x] RED: Remove `@wip`; bind `I run \`watn quicksetup --help\`` with
  `unimplemented!()`. Run non-zero.
  ```text
  command: `./run-tests.sh --name 'Quick setup documents its parameters in help'`
  output: exit 1 on the undefined help step (strict proof).
  ```
- [x] GREEN: Convert `Commands::Quicksetup` to a struct variant with the six
  options and implement the help step. Production files changed:
  `src/main.rs`.
  ```text
  command: `./run-tests.sh --name 'Quick setup documents its parameters in help'`
  output: exit 0; 1 feature; 1 scenario (1 passed); 7 steps (7 passed).
  ```
- [x] REFACTOR: Rerun.
  ```text
  command: `./run-tests.sh --name 'Quick setup documents its parameters in help'`
  output: exit 0; 1 feature; 1 scenario (1 passed); 7 steps (7 passed).
  ```
- [x] COMMIT: `26464a1` - feat(quicksetup): Quick setup documents its parameters in help

### Quick setup parameters prefill the dialog with an environment credential

Domain constraints: every prompt still runs with the parameter as its
suggestion; an empty answer accepts it; a `${ENV_VAR}` key is stored as an
environment reference; no network request happens.

- [x] RED: Remove `@wip`; bind the parameter start step with
  `unimplemented!()`. Run the E2E scenario targeted; non-zero.
  ```text
  command: `./run-tests.sh --e2e --name 'Quick setup parameters prefill the dialog with an environment credential'`
  output: undefined-step failure before the docstring parameter step existed (the defaults landed with the help commit).
  ```
- [x] GREEN: Add `QuickSetupDefaults` and `run_with_defaults` with url/key/model
  suggestions, wire `run_quicksetup_command`, and implement the docstring
  parameter step. Production files changed: `src/quicksetup.rs`,
  `src/main.rs`, `tests/steps/quicksetup_steps.rs`.
  ```text
  command: `./run-tests.sh --e2e --name 'Quick setup parameters prefill the dialog with an environment credential'`
  output: exit 0; 1 feature; 1 scenario (1 passed); 15 steps (15 passed).
  ```
- [x] REFACTOR: Rerun.
  ```text
  command: `./run-tests.sh --e2e --name 'Quick setup parameters prefill the dialog with an environment credential'`
  output: exit 0; 1 feature; 1 scenario (1 passed); 15 steps (15 passed).
  ```
- [x] COMMIT: `575e4d8` - feat(quicksetup): Quick setup parameters prefill the dialog

### A small-model parameter seeds the remaining tiers

Domain constraints: `--model-small` prefills small and the other tiers fall
back to the accepted small answer.

- [x] RED: Remove `@wip`; run the E2E scenario targeted; it fails until the
  suggestion resolution lands.
  ```text
  command: `./run-tests.sh --e2e --name 'A small-model parameter seeds the remaining tiers'`
  output: the normal/thinking prompts initially lacked the small fallback; the resolution landed in the prefill production commit.
  ```
- [x] GREEN: Route `model_small` into the small suggestion and keep the
  accepted-small fallback for normal and thinking.
  ```text
  command: `./run-tests.sh --e2e --name 'A small-model parameter seeds the remaining tiers'`
  output: exit 0; 1 feature; 1 scenario (1 passed); 10 steps (10 passed).
  ```
- [x] REFACTOR: Rerun.
  ```text
  command: `./run-tests.sh --e2e --name 'A small-model parameter seeds the remaining tiers'`
  output: exit 0; 1 feature; 1 scenario (1 passed); 10 steps (10 passed).
  ```
- [x] COMMIT: `f06aa89` - feat(quicksetup): A small-model parameter seeds the remaining tiers

### Tier parameters override the shared model prefill

Domain constraints: `--model` seeds all tiers; `--model-small` and
`--model-thinking` win for their tier; normal keeps the shared value.

- [x] RED: Remove `@wip`; run the E2E scenario targeted; non-zero.
  ```text
  command: `./run-tests.sh --e2e --name 'Tier parameters override the shared model prefill'`
  output: undefined-step/prefill failure before the tier precedence landed.
  ```
- [x] GREEN: Apply tier precedence (`model_small`/`model_normal`/
  `model_thinking` before `model`).
  ```text
  command: `./run-tests.sh --e2e --name 'Tier parameters override the shared model prefill'`
  output: exit 0; 1 feature; 1 scenario (1 passed); 10 steps (10 passed).
  ```
- [x] REFACTOR: Rerun.
  ```text
  command: `./run-tests.sh --e2e --name 'Tier parameters override the shared model prefill'`
  output: exit 0; 1 feature; 1 scenario (1 passed); 10 steps (10 passed).
  ```
- [x] COMMIT: `db3bb46` - feat(quicksetup): Tier parameters override the shared model prefill

## Final verification

- [x] Run `cargo fmt --all -- --check`
  ```text
  output: exit 0; clean.
  ```
- [x] Run `cargo clippy --locked --all-targets -- -D warnings`
  ```text
  output: exit 0; clean.
  ```
- [x] Run `givn lint --change quicksetup-parameters`
  ```text
  output: exit 0; clean; one advisory subset notice and one long-scenario notice dispositioned in review.md.
  ```
- [x] Run the full regular suite `./run-tests.sh`
  ```text
  output: exit 0; 21 features; 218 scenarios (218 passed); 1328 steps (1328 passed).
  ```
- [x] Run the full E2E suite `./run-tests.sh --e2e`
  ```text
  output: exit 0; 25 features; 88 scenarios (88 passed); 665 steps (665 passed).
  ```
- [x] Run `cargo test --locked --lib`
  ```text
  output: exit 0; 84 passed; 0 failed.
  ```
- [x] Run `givn status --change quicksetup-parameters`
  ```text
  output: next required artifact is `review`.
  ```
