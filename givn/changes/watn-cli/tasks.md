# Tasks: watn-cli

## Setup

- [ ] Scaffold Rust project structure (Cargo.toml, src/, tests/)
- [ ] Install cucumber-rs, configure `tests/features_runner.rs` as Gherkin runner
- [ ] Configure `verify.command` in givn/commands.yaml
- [ ] Create step definition skeletons (one file per capability) with `unimplemented!()` stubs
- [ ] Prove strict mode: run runner, confirm non-zero exit
- [ ] Copy delta specs to a location cucumber-rs can discover (givn/specs/ or tests/features/)

## Non-@e2e scenarios

### ask.feature

- [ ] Scenario: Cancelling a streaming response with Ctrl+C
  - [ ] RED — output: ` `
  - [ ] GREEN — files: ` `
  - [ ] REFACTOR — output: ` `
  - [ ] COMMIT — hash: ` `

- [ ] Scenario: No arguments and no stdin prints help and exits with error
  - [ ] RED — output: ` `
  - [ ] GREEN — files: ` `
  - [ ] REFACTOR — output: ` `
  - [ ] COMMIT — hash: ` `

- [ ] Scenario: Non-zero exit code on API authentication failure
  - [ ] RED — output: ` `
  - [ ] GREEN — files: ` `
  - [ ] REFACTOR — output: ` `
  - [ ] COMMIT — hash: ` `

- [ ] Scenario: Explicit model flag overrides tier dispatch
  - [ ] RED — output: ` `
  - [ ] GREEN — files: ` `
  - [ ] REFACTOR — output: ` `
  - [ ] COMMIT — hash: ` `

- [ ] Scenario: Version flag prints logo and version
  - [ ] RED — output: ` `
  - [ ] GREEN — files: ` `
  - [ ] REFACTOR — output: ` `
  - [ ] COMMIT — hash: ` `

- [ ] Scenario: Default model used when no tiers configured
  - [ ] RED — output: ` `
  - [ ] GREEN — files: ` `
  - [ ] REFACTOR — output: ` `
  - [ ] COMMIT — hash: ` `

### config.feature

- [ ] Scenario: Missing config file does not error
  - [ ] RED — output: ` `
  - [ ] GREEN — files: ` `
  - [ ] REFACTOR — output: ` `
  - [ ] COMMIT — hash: ` `

- [ ] Scenario: Config file with syntax error produces diagnostic
  - [ ] RED — output: ` `
  - [ ] GREEN — files: ` `
  - [ ] REFACTOR — output: ` `
  - [ ] COMMIT — hash: ` `

### providers.feature

- [ ] Scenario: Unknown provider produces error
  - [ ] RED — output: ` `
  - [ ] GREEN — files: ` `
  - [ ] REFACTOR — output: ` `
  - [ ] COMMIT — hash: ` `

- [ ] Scenario: Missing API key produces error
  - [ ] RED — output: ` `
  - [ ] GREEN — files: ` `
  - [ ] REFACTOR — output: ` `
  - [ ] COMMIT — hash: ` `

## E2E setup

- [ ] Write e2e step skeleton files (one per capability) with `unimplemented!()` stubs
- [ ] Configure `verify.e2e_command` in givn/commands.yaml
- [ ] Prove e2e strict mode: run e2e runner, confirm non-zero exit
- [ ] Prove e2e filter: `verify.command` count > `verify.e2e_command` count

## @e2e scenarios

### ask.feature

- [ ] Scenario: Ask with default tier returns a copy-pasteable command
  - [ ] RED — output: ` `
  - [ ] GREEN — files: ` `
  - [ ] REFACTOR — output: ` `
  - [ ] COMMIT — hash: ` `

- [ ] Scenario: Explicit tier -1 uses the small/fast model
  - [ ] RED — output: ` `
  - [ ] GREEN — files: ` `
  - [ ] REFACTOR — output: ` `
  - [ ] COMMIT — hash: ` `

- [ ] Scenario: Tier -2 uses the normal model
  - [ ] RED — output: ` `
  - [ ] GREEN — files: ` `
  - [ ] REFACTOR — output: ` `
  - [ ] COMMIT — hash: ` `

- [ ] Scenario: Tier -3 uses the thinking/reasoning model
  - [ ] RED — output: ` `
  - [ ] GREEN — files: ` `
  - [ ] REFACTOR — output: ` `
  - [ ] COMMIT — hash: ` `

- [ ] Scenario: Execute flag prompts for confirmation
  - [ ] RED — output: ` `
  - [ ] GREEN — files: ` `
  - [ ] REFACTOR — output: ` `
  - [ ] COMMIT — hash: ` `

- [ ] Scenario: Execute flag with explicit "y" confirmation
  - [ ] RED — output: ` `
  - [ ] GREEN — files: ` `
  - [ ] REFACTOR — output: ` `
  - [ ] COMMIT — hash: ` `

- [ ] Scenario: Execute flag with "n" answer skips execution
  - [ ] RED — output: ` `
  - [ ] GREEN — files: ` `
  - [ ] REFACTOR — output: ` `
  - [ ] COMMIT — hash: ` `

- [ ] Scenario: Cost is displayed when pricing is configured
  - [ ] RED — output: ` `
  - [ ] GREEN — files: ` `
  - [ ] REFACTOR — output: ` `
  - [ ] COMMIT — hash: ` `

- [ ] Scenario: Tokens/second is displayed after response completes
  - [ ] RED — output: ` `
  - [ ] GREEN — files: ` `
  - [ ] REFACTOR — output: ` `
  - [ ] COMMIT — hash: ` `

- [ ] Scenario: Ask via stdin pipe
  - [ ] RED — output: ` `
  - [ ] GREEN — files: ` `
  - [ ] REFACTOR — output: ` `
  - [ ] COMMIT — hash: ` `

### config.feature

- [ ] Scenario: Configure model tiers in config file
  - [ ] RED — output: ` `
  - [ ] GREEN — files: ` `
  - [ ] REFACTOR — output: ` `
  - [ ] COMMIT — hash: ` `

- [ ] Scenario: Environment variable overrides config file
  - [ ] RED — output: ` `
  - [ ] GREEN — files: ` `
  - [ ] REFACTOR — output: ` `
  - [ ] COMMIT — hash: ` `

- [ ] Scenario: CLI flag overrides environment variable
  - [ ] RED — output: ` `
  - [ ] GREEN — files: ` `
  - [ ] REFACTOR — output: ` `
  - [ ] COMMIT — hash: ` `

- [ ] Scenario: Model pricing configured for cost display
  - [ ] RED — output: ` `
  - [ ] GREEN — files: ` `
  - [ ] REFACTOR — output: ` `
  - [ ] COMMIT — hash: ` `

### models.feature

- [ ] Scenario: Discover models and select tiers interactively
  - [ ] RED — output: ` `
  - [ ] GREEN — files: ` `
  - [ ] REFACTOR — output: ` `
  - [ ] COMMIT — hash: ` `

- [ ] Scenario: Model explorer without LiteLLM endpoint configured
  - [ ] RED — output: ` `
  - [ ] GREEN — files: ` `
  - [ ] REFACTOR — output: ` `
  - [ ] COMMIT — hash: ` `

### providers.feature

- [ ] Scenario: Custom OpenAI-compatible provider from config
  - [ ] RED — output: ` `
  - [ ] GREEN — files: ` `
  - [ ] REFACTOR — output: ` `
  - [ ] COMMIT — hash: ` `

- [ ] Scenario: LiteLLM endpoint in config for model discovery
  - [ ] RED — output: ` `
  - [ ] GREEN — files: ` `
  - [ ] REFACTOR — output: ` `
  - [ ] COMMIT — hash: ` `

- [ ] Scenario: Provider API key from environment variable
  - [ ] RED — output: ` `
  - [ ] GREEN — files: ` `
  - [ ] REFACTOR — output: ` `
  - [ ] COMMIT — hash: ` `

## Verify

- [ ] Full suite GREEN: `cargo test --test features_runner` exits 0
- [ ] Full e2e suite GREEN: `cargo test --test features_runner -- --tags @e2e` exits 0
- [ ] `givn lint --change watn-cli` exits 0
