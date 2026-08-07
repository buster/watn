# Tasks: model-explorer

## Setup: strict-mode proof

- [ ] Configure runner (already done: `.fail_on_skipped()` in features_runner.rs)
- [ ] Write a step with `unimplemented!()`, run `cargo test --test features_runner -- --tags '@wip'`, confirm non-zero exit.
- [ ] Remove the test step.

## Scenario list (non-@e2e first)

### 1. @wip: Model explorer without provider configured (non-e2e)

- [ ] RED: Remove @wip from this scenario only. Write step definitions with `unimplemented!()`. Run runner → MUST FAIL.
  Evidence:
- [ ] GREEN: Replace stubs with real assertions. Write production code. Files: `src/models/mod.rs`.
  Run runner → PASSES.
  Evidence:
- [ ] REFACTOR: Clean up. Runner still PASSES.
- [ ] COMMIT. Hash:

### 2. @wip: Model explorer api call fails (non-e2e)

- [ ] RED: Remove @wip. Write step defs with `unimplemented!()`. Run → FAIL.
  Evidence:
- [ ] GREEN: Implement fetch_models, error handling. Files: `src/models/list.rs`, `src/models/mod.rs`.
  Run → PASSES.
  Evidence:
- [ ] REFACTOR.
- [ ] COMMIT. Hash:

### 3. @wip: Model picker shows metadata when available (non-e2e)

- [ ] RED: Remove @wip. Write step defs with `unimplemented!()`. Run → FAIL.
  Evidence:
- [ ] GREEN: Implement metadata display in picker. Files: `src/models/list.rs`, `src/models/mod.rs`.
  Run → PASSES.
  Evidence:
- [ ] REFACTOR.
- [ ] COMMIT. Hash:

### 4. @wip: Model picker shows model IDs when no metadata available (non-e2e)

- [ ] RED: Remove @wip. Write step defs with `unimplemented!()`. Run → FAIL.
  Evidence:
- [ ] GREEN: Handle bare model IDs display. Files: `src/models/mod.rs`.
  Run → PASSES.
  Evidence:
- [ ] REFACTOR.
- [ ] COMMIT. Hash:

### 5. @wip @e2e: Discover models and select tiers interactively

- [ ] RED: Remove @wip. Write e2e step defs (interactive stdin). Run e2e → FAIL.
  Evidence:
- [ ] GREEN: Implement interactive dialoguer selection. Files: `src/models/mod.rs`, `src/models/list.rs`.
  Run e2e → PASSES.
  Evidence:
- [ ] REFACTOR.
- [ ] COMMIT. Hash:

### 6. @wip @e2e: Model explorer with openrouter default and env var set

- [ ] RED: Remove @wip. Write step defs. Run e2e → FAIL.
  Evidence:
- [ ] GREEN: Handle openrouter resolution in models command. Files: `src/models/mod.rs`.
  Run e2e → PASSES.
  Evidence:
- [ ] REFACTOR.
- [ ] COMMIT. Hash:

## Final verification

- [ ] Run `cargo test --test features_runner` — all existing + new scenarios pass
- [ ] Run `cargo test --test features_runner -- --tags '@e2e and not @wip'` — all @e2e scenarios pass; count > 0
- [ ] `cargo build` — 0 warnings
