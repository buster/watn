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

- [ ] RED: Remove `@wip`; bind the parameter start step with
  `unimplemented!()`. Run the E2E scenario targeted; non-zero.
  ```text
  command: `./run-tests.sh --e2e --name 'Quick setup parameters prefill the dialog with an environment credential'`
  output: <paste>
  ```
- [ ] GREEN: Add `QuickSetupDefaults` and `run_with_defaults` with url/key/model
  suggestions, wire `run_quicksetup_command`, and implement the docstring
  parameter step. Production files changed: `src/quicksetup.rs`,
  `src/main.rs`, `tests/steps/quicksetup_steps.rs`.
  ```text
  command: `./run-tests.sh --e2e --name 'Quick setup parameters prefill the dialog with an environment credential'`
  output: <paste>
  ```
- [ ] REFACTOR: Rerun.
  ```text
  command: `./run-tests.sh --e2e --name 'Quick setup parameters prefill the dialog with an environment credential'`
  output: <paste>
  ```
- [ ] COMMIT: `<hash>` - feat(quicksetup): Quick setup parameters prefill the dialog

### A small-model parameter seeds the remaining tiers

Domain constraints: `--model-small` prefills small and the other tiers fall
back to the accepted small answer.

- [ ] RED: Remove `@wip`; run the E2E scenario targeted; it fails until the
  suggestion resolution lands.
  ```text
  command: `./run-tests.sh --e2e --name 'A small-model parameter seeds the remaining tiers'`
  output: <paste>
  ```
- [ ] GREEN: Route `model_small` into the small suggestion and keep the
  accepted-small fallback for normal and thinking.
  ```text
  command: `./run-tests.sh --e2e --name 'A small-model parameter seeds the remaining tiers'`
  output: <paste>
  ```
- [ ] REFACTOR: Rerun.
  ```text
  command: `./run-tests.sh --e2e --name 'A small-model parameter seeds the remaining tiers'`
  output: <paste>
  ```
- [ ] COMMIT: `<hash>` - feat(quicksetup): A small-model parameter seeds the remaining tiers

### Tier parameters override the shared model prefill

Domain constraints: `--model` seeds all tiers; `--model-small` and
`--model-thinking` win for their tier; normal keeps the shared value.

- [ ] RED: Remove `@wip`; run the E2E scenario targeted; non-zero.
  ```text
  command: `./run-tests.sh --e2e --name 'Tier parameters override the shared model prefill'`
  output: <paste>
  ```
- [ ] GREEN: Apply tier precedence (`model_small`/`model_normal`/
  `model_thinking` before `model`).
  ```text
  command: `./run-tests.sh --e2e --name 'Tier parameters override the shared model prefill'`
  output: <paste>
  ```
- [ ] REFACTOR: Rerun.
  ```text
  command: `./run-tests.sh --e2e --name 'Tier parameters override the shared model prefill'`
  output: <paste>
  ```
- [ ] COMMIT: `<hash>` - feat(quicksetup): Tier parameters override the shared model prefill

## Final verification

- [ ] Run `cargo fmt --all -- --check`
  ```text
  output: <paste>
  ```
- [ ] Run `cargo clippy --locked --all-targets -- -D warnings`
  ```text
  output: <paste>
  ```
- [ ] Run `givn lint --change quicksetup-parameters`
  ```text
  output: <paste>
  ```
- [ ] Run the full regular suite `./run-tests.sh`
  ```text
  output: <paste>
  ```
- [ ] Run the full E2E suite `./run-tests.sh --e2e`
  ```text
  output: <paste>
  ```
- [ ] Run `cargo test --locked --lib`
  ```text
  output: <paste>
  ```
- [ ] Run `givn status --change quicksetup-parameters`
  ```text
  output: <paste>
  ```
