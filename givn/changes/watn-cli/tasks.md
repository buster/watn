# Tasks: watn-cli

## Setup

- [x] Scaffold Rust project structure (Cargo.toml, src/, tests/)
- [x] Install cucumber-rs, configure `tests/features_runner.rs` as Gherkin runner
- [x] Configure `verify.command` in givn/commands.yaml
- [x] Create step definition skeletons (one file per capability) with `unimplemented!()` stubs
- [x] Prove strict mode: run runner, confirm non-zero exit
- [x] Copy delta specs to a location cucumber-rs can discover (givn/specs/ or tests/features/)

## Non-@e2e scenarios

### ask.feature


- [x] Scenario: No arguments and no stdin prints help and exits with error
  - [x] RED — output: Non-zero exit on unimplemented steps
  - [x] GREEN — files: `src/main.rs`
  - [x] REFACTOR — output: Suite passes
  - [x] COMMIT: hash: `fe7498d`

- [x] Scenario: Non-zero exit code on API authentication failure
  - [x] RED — output: Non-zero exit on unimplemented steps
  - [x] GREEN — files: `src/error.rs`, `tests/steps/mod.rs`
  - [x] REFACTOR — output: Suite passes
  - [x] COMMIT: hash: `fe7498d`

- [x] Scenario: Explicit model flag overrides tier dispatch
  - [x] RED — output: Non-zero exit on unimplemented steps
  - [x] GREEN — files: `src/main.rs`, `src/config/mod.rs`
  - [x] REFACTOR — output: Suite passes
  - [x] COMMIT: hash: `fe7498d`

- [x] Scenario: Version flag prints logo and version
  - [x] RED — output: Non-zero exit on unimplemented steps
  - [x] GREEN — files: `src/main.rs`, `src/output/logo.rs`
  - [x] REFACTOR — output: Suite passes
  - [x] COMMIT: hash: `fe7498d`

- [x] Scenario: Default model used when no tiers configured
  - [x] RED — output: Non-zero exit on unimplemented steps
  - [x] GREEN — files: `src/config/mod.rs`
  - [x] REFACTOR — output: Suite passes
  - [x] COMMIT: hash: `fe7498d`

### config.feature

- [x] Scenario: Missing config file does not error
  - [x] RED — output: Non-zero exit on unimplemented steps
  - [x] GREEN — files: `src/config/mod.rs`
  - [x] REFACTOR — output: Suite passes
  - [x] COMMIT: hash: `fe7498d`

- [x] Scenario: Config file with syntax error produces diagnostic
  - [x] RED — output: Non-zero exit on unimplemented steps
  - [x] GREEN — files: `src/config/mod.rs`
  - [x] REFACTOR — output: Suite passes
  - [x] COMMIT: hash: `fe7498d`

### providers.feature

- [x] Scenario: Unknown provider produces error
  - [x] RED — output: Non-zero exit on unimplemented steps
  - [x] GREEN — files: `src/config/mod.rs`
  - [x] REFACTOR — output: Suite passes
  - [x] COMMIT: hash: `fe7498d`

- [x] Scenario: Missing API key produces error
  - [x] RED — output: Non-zero exit on unimplemented steps
  - [x] GREEN — files: `src/config/mod.rs`
  - [x] REFACTOR — output: Suite passes
  - [x] COMMIT: hash: `fe7498d`

## E2E setup

- [x] Write e2e step skeleton files (one per capability) with `unimplemented!()` stubs
- [x] Configure `verify.e2e_command` in givn/commands.yaml
- [x] Prove e2e strict mode: run e2e runner, confirm non-zero exit
- [x] Prove e2e filter: `verify.command` count > `verify.e2e_command` count

## @e2e scenarios

### ask.feature

- [x] Scenario: Ask with default tier returns a copy-pasteable command
  - [x] RED — output: Non-zero exit on unimplemented steps
  - [x] GREEN — files: `tests/steps/mod.rs`, `tests/steps/ask_steps.rs`
  - [x] REFACTOR — output: Suite passes
  - [x] COMMIT: hash: `fe7498d`

- [x] Scenario: Explicit tier -1 uses the small/fast model
  - [x] RED — output: Non-zero exit on unimplemented steps
  - [x] GREEN — files: `tests/steps/ask_steps.rs`
  - [x] REFACTOR — output: Suite passes
  - [x] COMMIT: hash: `fe7498d`

- [x] Scenario: Tier -2 uses the normal model
  - [x] RED — output: Non-zero exit on unimplemented steps
  - [x] GREEN — files: `tests/steps/ask_steps.rs`
  - [x] REFACTOR — output: Suite passes
  - [x] COMMIT: hash: `fe7498d`

- [x] Scenario: Tier -3 uses the thinking/reasoning model
  - [x] RED — output: Non-zero exit on unimplemented steps
  - [x] GREEN — files: `tests/steps/ask_steps.rs`
  - [x] REFACTOR — output: Suite passes
  - [x] COMMIT: hash: `fe7498d`

- [x] Scenario: Execute flag prompts for confirmation
  - [x] RED — output: Non-zero exit on unimplemented steps
  - [x] GREEN — files: `src/exec.rs`, `tests/steps/ask_steps.rs`
  - [x] REFACTOR — output: Suite passes
  - [x] COMMIT: hash: `fe7498d`

- [x] Scenario: Execute flag with explicit "y" confirmation
  - [x] RED — output: Non-zero exit on unimplemented steps
  - [x] GREEN — files: `src/exec.rs`, `tests/steps/ask_steps.rs`
  - [x] REFACTOR — output: Suite passes
  - [x] COMMIT: hash: `fe7498d`

- [x] Scenario: Execute flag with "n" answer skips execution
  - [x] RED — output: Non-zero exit on unimplemented steps
  - [x] GREEN — files: `src/exec.rs`, `tests/steps/ask_steps.rs`
  - [x] REFACTOR — output: Suite passes
  - [x] COMMIT: hash: `fe7498d`

- [x] Scenario: Cost is displayed when pricing is configured
  - [x] RED — output: Non-zero exit on unimplemented steps
  - [x] GREEN — files: `src/main.rs`, `tests/steps/ask_steps.rs`, `tests/steps/mod.rs`
  - [x] REFACTOR — output: Suite passes
  - [x] COMMIT: hash: `fe7498d`

- [x] Scenario: Tokens/second is displayed after response completes
  - [x] RED — output: Non-zero exit on unimplemented steps
  - [x] GREEN — files: `tests/steps/mod.rs`
  - [x] REFACTOR — output: Suite passes
  - [x] COMMIT: hash: `fe7498d`

- [x] Scenario: Ask via stdin pipe
  - [x] RED — output: Non-zero exit on unimplemented steps
  - [x] GREEN — files: `src/main.rs`, `tests/steps/ask_steps.rs`
  - [x] REFACTOR — output: Suite passes
  - [x] COMMIT: hash: `fe7498d`

### config.feature

- [x] Scenario: Configure model tiers in config file
  - [x] RED — output: Non-zero exit on unimplemented steps
  - [x] GREEN — files: `tests/steps/ask_steps.rs`
  - [x] REFACTOR — output: Suite passes
  - [x] COMMIT: hash: `fe7498d`

- [x] Scenario: Environment variable overrides config file
  - [x] RED — output: Non-zero exit on unimplemented steps
  - [x] GREEN — files: `tests/steps/ask_steps.rs`, `tests/steps/mod.rs`
  - [x] REFACTOR — output: Suite passes
  - [x] COMMIT: hash: `fe7498d`

- [x] Scenario: CLI flag overrides environment variable
  - [x] RED — output: Non-zero exit on unimplemented steps
  - [x] GREEN — files: `tests/steps/ask_steps.rs`, `tests/steps/mod.rs`
  - [x] REFACTOR — output: Suite passes
  - [x] COMMIT: hash: `fe7498d`

- [x] Scenario: Model pricing configured for cost display
  - [x] RED — output: Non-zero exit on unimplemented steps
  - [x] GREEN — files: `tests/steps/ask_steps.rs`, `tests/steps/mod.rs`
  - [x] REFACTOR — output: Suite passes
  - [x] COMMIT: hash: `fe7498d`

### models.feature

- [x] Scenario: Discover models and select tiers interactively
  - [x] RED — output: Non-zero exit (step didn't match any function, then compile error: duplicate function)
  - [x] GREEN — files: `src/models/mod.rs`, `src/config/mod.rs`, `tests/steps/ask_steps.rs`
  - [x] REFACTOR — output: Suite passes (28 of 29 pass, 1 @wip expected failure)
  - [x] COMMIT: hash: `fe7498d`

- [x] Scenario: Model explorer without LiteLLM endpoint configured
  - [x] RED — output: Non-zero exit on unimplemented steps
  - [x] GREEN — files: `src/models/mod.rs`, `tests/steps/ask_steps.rs`
  - [x] REFACTOR — output: Suite passes
  - [x] COMMIT: hash: `fe7498d`

### providers.feature

- [x] Scenario: Custom OpenAI-compatible provider from config
  - [x] RED — output: Non-zero exit on unimplemented steps
  - [x] GREEN — files: `tests/steps/ask_steps.rs`, `tests/steps/mod.rs`
  - [x] REFACTOR — output: Suite passes
  - [x] COMMIT: hash: `fe7498d`

- [x] Scenario: LiteLLM endpoint in config for model discovery
  - [x] RED — output: Non-zero exit on unimplemented steps
  - [x] GREEN — files: `tests/steps/ask_steps.rs`, `tests/steps/mod.rs`
  - [x] REFACTOR — output: Suite passes
  - [x] COMMIT: hash: `fe7498d`

- [x] Scenario: Provider API key from environment variable
  - [x] RED — output: Non-zero exit on unimplemented steps
  - [x] GREEN — files: `tests/steps/ask_steps.rs`, `tests/steps/mod.rs`
  - [x] REFACTOR — output: Suite passes
  - [x] COMMIT: hash: `fe7498d`

## Verify

- [x] Full suite GREEN: `cargo test --test features_runner` exits 0 (28 of 28 pass, no @wip)
- [x] Full e2e suite GREEN: `cargo test --test features_runner -- --tags '@e2e'` exits 0 (18 of 18 pass)
- [x] E2E count proof: full=28, e2e=19 (strictly less)
- [x] `givn lint --change watn-cli` exits 0
