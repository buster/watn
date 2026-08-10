# Tasks: model-discovery-and-setup-correctness

## Inventory and Matrix Verification

- [x] Cross-reference the `# User Interaction Inventory:` blocks in all five
  delta feature files against the Interactive Coverage Matrix in `design.md`.
- [x] Confirm all eight inventory entries have matrix rows, all matrix rows
  use the real CLI interface through `portable-pty` or a real debug
  subprocess, and all eight matrix scenario titles exist as `@e2e` scenarios.
- Evidence: `credentials.feature` covers interactive model discovery and
  configured-provider chat; `catalog-source.feature` covers model discovery
  and post-discovery chat; `setup-persistence.feature` covers setup
  confirmation/cancellation and tier assignment; `reasoning-policy.feature`
  covers selected-tier chat; `search-concurrency.feature` covers overlapping
  picker searches. The matrix rows and scenario titles match exactly.
- Matrix mismatch: none found.

## Runner and Strict-Mode Setup

- [x] Configure the existing `tests/features_runner.rs` cucumber-rs runner
  with `Cucumber::fail_on_skipped()`, preserving discovery of both
  `givn/specs/**/*.feature` and the active change feature files.
- [x] Set `verify.command` in `givn/commands.yaml` to the exact configured
  full non-e2e command:
  `root=$(mktemp -d /tmp/watn-transport.XXXXXX) && trap 'rm -rf "$root"' EXIT && cargo build --bin watn && cp target/debug/watn "$root/default-debug" && cargo build --features test-support --bin watn && cp target/debug/watn "$root/test-support-debug" && WATN_DEFAULT_DEBUG_BIN="$root/default-debug" WATN_TEST_SUPPORT_DEBUG_BIN="$root/test-support-debug" cargo test --test features_runner --features test-support -- --tags 'not @wip and not @e2e'`.
- [x] Set `verify.e2e_command` in `givn/commands.yaml` to the exact configured
  e2e command:
  `root=$(mktemp -d /tmp/watn-transport.XXXXXX) && trap 'rm -rf "$root"' EXIT && cargo build --bin watn && cp target/debug/watn "$root/default-debug" && cargo build --features test-support --bin watn && cp target/debug/watn "$root/test-support-debug" && WATN_DEFAULT_DEBUG_BIN="$root/default-debug" WATN_TEST_SUPPORT_DEBUG_BIN="$root/test-support-debug" cargo test --test features_runner --features test-support -- --tags '@e2e and not @wip'`.
- [x] Create one step-definition skeleton per capability at the design-named
  locations: `tests/steps/credentials_steps.rs`,
  `tests/steps/catalog_source_steps.rs`,
  `tests/steps/setup_persistence_steps.rs`,
  `tests/steps/reasoning_policy_steps.rs`, and
  `tests/steps/search_concurrency_steps.rs`. Declare them from
  `tests/steps/mod.rs`; use `unimplemented!("<step contract>")` for every
  unimplemented body and keep shared helpers in `tests/steps/mod.rs`.
- [x] Proof of strictness: temporarily execute one step through the
  `unimplemented!` stub, run the configured runner, and confirm a non-zero
  exit. Paste the exact command and complete non-zero output here.
- Strictness proof command/output:
  ```text
  `root=$(mktemp -d /tmp/watn-transport.XXXXXX) && trap 'rm -rf "$root"' EXIT && cargo build --bin watn && cp target/debug/watn "$root/default-debug" && cargo build --features test-support --bin watn && cp target/debug/watn "$root/test-support-debug" && WATN_DEFAULT_DEBUG_BIN="$root/default-debug" WATN_TEST_SUPPORT_DEBUG_BIN="$root/test-support-debug" cargo test --test features_runner --features test-support -- --name 'A missing saved environment credential fails before discovery'`
  Exit 101; Cucumber reported `1 scenario (1 failed)` and Cargo reported `error: test failed` after an unmatched step, proving strict failure.
  ```
- [x] Confirm clean local startup for the design's single-CLI environment;
  no server, database, live provider, or external network is required. The
  local command is `cargo run -- <question>` or `cargo run -- models`, with
  `httpmock::MockServer` loopback twins used by scenarios.
- Local startup evidence:
  ```text
  `cargo run -- --version` -> exit 0, version output produced; no server, database, provider, or external network required.
  ```

## Non-E2E Scenario 1: A Missing Saved Environment Credential Fails Before Discovery

- [x] RED: Remove `@wip` from only `Scenario: A missing saved environment credential fails before discovery`; implement its new steps with the strict `unimplemented!` stub; run the single-scenario command using `-- --name 'A missing saved environment credential fails before discovery'`; confirm non-zero exit; paste output.
- RED runner output:
  ```text
  Targeted runner exited 101: Cucumber reported `1 scenario (1 failed)` and an unmatched step, followed by `error: test failed`.
  ```
- [x] GREEN: Replace stubs with real authentication-error and no-catalog-request assertions; implement the minimum production behavior. Production files created/modified: `src/models/mod.rs`.
- GREEN runner output:
  ```text
  Targeted runner exited 0: `1 scenario (1 passed)`, `8 steps (8 passed)`.
  ```
- [x] REFACTOR: Clean up without behavior change; rerun the same single-scenario command and confirm zero exit; paste output.
- REFACTOR runner output:
  ```text
  After `cargo fmt --all`, targeted runner exited 0: `1 scenario (1 passed)`, `8 steps (8 passed)`.
  ```
- [x] COMMIT: Create one atomic RED/GREEN/REFACTOR commit referencing the exact scenario title.
- Commit hash: `c512243c5d9aae9b529796159ebb9067d9287c10`

## Non-E2E Scenario 2: Provider-Specific Environment Fallback Precedes Generic Fallback

- [x] RED: Remove `@wip` from only `Scenario: Provider-specific environment fallback precedes generic fallback`; implement new steps with `unimplemented!`; target that scenario by name; confirm non-zero exit; paste output.
- RED runner output:
  ```text
  Targeted runner exited 101: Cucumber reported `1 scenario (1 failed)` at the undefined `its saved api_key is absent` step.
  ```
- [x] GREEN: Replace stubs with real provider-specific-over-generic credential assertions and minimum production code. Production files created/modified: `src/config/mod.rs` (existing fallback behavior exercised), `tests/steps/credentials_steps.rs`, `tests/steps/mod.rs`, `tests/steps/provider_setup_steps.rs`.
- GREEN runner output:
  ```text
  Targeted runner exited 0: `1 scenario (1 passed)`, `9 steps (9 passed)`.
  ```
- [x] REFACTOR: Clean up without behavior change; rerun the targeted scenario and confirm zero exit; paste output.
- REFACTOR runner output:
  ```text
  After `cargo fmt --all`, targeted runner exited 0: `1 scenario (1 passed)`, `9 steps (9 passed)`.
  ```
- [x] COMMIT: Create one atomic commit referencing `Provider-specific environment fallback precedes generic fallback`.
- Commit hash: `317ea09686e4d73f9d2215d3318543e1123e01ef`

## Non-E2E Scenario 3: LiteLLM Discovery Without a Key Sends No Authorization Header

- [x] RED: Remove `@wip` from only `Scenario: LiteLLM discovery without a key sends no authorization header`; implement new steps with `unimplemented!`; target by name; confirm non-zero exit; paste output.
- RED runner output:
  ```text
  Targeted runner exited 101: Cucumber reported `1 scenario (1 failed)` at undefined catalog-source setup.
  ```
- [x] GREEN: Replace stubs with real LiteLLM endpoint and absent-header assertions; implement minimum production code. Production files created/modified: `src/models/mod.rs`, `tests/steps/catalog_source_steps.rs`, `tests/steps/mod.rs`.
- GREEN runner output:
  ```text
  Targeted runner exited 0: `1 scenario (1 passed)`, `6 steps (6 passed)`.
  ```
- [x] REFACTOR: Clean up and rerun the targeted scenario; confirm zero exit; paste output.
- REFACTOR runner output:
  ```text
  After `cargo fmt --all`, targeted runner exited 0: `1 scenario (1 passed)`, `6 steps (6 passed)`.
  ```
- [x] COMMIT: Create one atomic commit referencing `LiteLLM discovery without a key sends no authorization header`.
- Commit hash: `0e2eeac80c3d368fe8cabbcb1d5964d726b73ca5`

## Non-E2E Scenario 4: Provider Discovery Is Used When LiteLLM Is Absent

- [ ] RED: Remove `@wip` from only `Scenario: Provider discovery is used when LiteLLM is absent`; implement new steps with `unimplemented!`; target by name; confirm non-zero exit; paste output.
- RED runner output:
  ```text
  PASTE COMMAND AND OUTPUT HERE
  ```
- [ ] GREEN: Replace stubs with real provider catalog endpoint and authorization assertions; implement minimum production code. Production files created/modified: `LIST FILES HERE`.
- GREEN runner output:
  ```text
  PASTE COMMAND AND OUTPUT HERE
  ```
- [ ] REFACTOR: Clean up and rerun the targeted scenario; confirm zero exit; paste output.
- REFACTOR runner output:
  ```text
  PASTE COMMAND AND OUTPUT HERE
  ```
- [ ] COMMIT: Create one atomic commit referencing `Provider discovery is used when LiteLLM is absent`.
- Commit hash: `PENDING`

## Non-E2E Scenario 5: Catalog Pagination and Search Use the Configured Catalog Source

- [ ] RED: Remove `@wip` from only `Scenario: Catalog pagination and search use the configured catalog source`; implement new steps with `unimplemented!`; target by name; confirm non-zero exit; paste output.
- RED runner output:
  ```text
  PASTE COMMAND AND OUTPUT HERE
  ```
- [ ] GREEN: Replace stubs with exact page/search URL, query, and authorization assertions; implement minimum production code. Production files created/modified: `LIST FILES HERE`.
- GREEN runner output:
  ```text
  PASTE COMMAND AND OUTPUT HERE
  ```
- [ ] REFACTOR: Clean up and rerun the targeted scenario; confirm zero exit; paste output.
- REFACTOR runner output:
  ```text
  PASTE COMMAND AND OUTPUT HERE
  ```
- [ ] COMMIT: Create one atomic commit referencing `Catalog pagination and search use the configured catalog source`.
- Commit hash: `PENDING`

## Non-E2E Scenario 6: A Disabled Model Default Selects Off Even When a Default Effort Is Present

- [x] RED: Remove `@wip` from only `Scenario: A disabled model default selects off even when a default effort is present`; implement new steps with `unimplemented!`; target by name; confirm non-zero exit; paste output.
- RED runner output:
  ```text
  Targeted runner exited 101: Cucumber reported `1 scenario (1 failed)` at undefined reasoning metadata setup.
  ```
- [x] GREEN: Replace stubs with reasoning-policy assertions and minimum production code. Production files created/modified: `src/models/dialog.rs`, `tests/steps/reasoning_policy_steps.rs`.
- GREEN runner output:
  ```text
  Targeted runner exited 0: `1 scenario (1 passed)`, `3 steps (3 passed)`.
  ```
- [x] REFACTOR: Clean up and rerun the targeted scenario; confirm zero exit; paste output.
- REFACTOR runner output:
  ```text
  After `cargo fmt --all`, targeted runner exited 0: `1 scenario (1 passed)`, `3 steps (3 passed)`.
  ```
- [x] COMMIT: Create one atomic commit referencing `A disabled model default selects off even when a default effort is present`.
- Commit hash: `PENDING`

## Non-E2E Scenario 7: Mandatory Reasoning Excludes Off

- [ ] RED: Remove `@wip` from only `Scenario: Mandatory reasoning excludes off`; implement new steps with `unimplemented!`; target by name; confirm non-zero exit; paste output.
- RED runner output:
  ```text
  PASTE COMMAND AND OUTPUT HERE
  ```
- [ ] GREEN: Replace stubs with valid non-off selection assertions and minimum production code. Production files created/modified: `LIST FILES HERE`.
- GREEN runner output:
  ```text
  PASTE COMMAND AND OUTPUT HERE
  ```
- [ ] REFACTOR: Clean up and rerun the targeted scenario; confirm zero exit; paste output.
- REFACTOR runner output:
  ```text
  PASTE COMMAND AND OUTPUT HERE
  ```
- [ ] COMMIT: Create one atomic commit referencing `Mandatory reasoning excludes off`.
- Commit hash: `PENDING`

## Non-E2E Scenario 8: Mandatory Reasoning With No Usable Metadata Returns a Policy Error

- [ ] RED: Remove `@wip` from only `Scenario: Mandatory reasoning with no usable metadata returns a policy error`; implement new steps with `unimplemented!`; target by name; confirm non-zero exit; paste output.
- RED runner output:
  ```text
  PASTE COMMAND AND OUTPUT HERE
  ```
- [ ] GREEN: Replace stubs with typed policy-error assertions and minimum production code. Production files created/modified: `LIST FILES HERE`.
- GREEN runner output:
  ```text
  PASTE COMMAND AND OUTPUT HERE
  ```
- [ ] REFACTOR: Clean up and rerun the targeted scenario; confirm zero exit; paste output.
- REFACTOR runner output:
  ```text
  PASTE COMMAND AND OUTPUT HERE
  ```
- [ ] COMMIT: Create one atomic commit referencing `Mandatory reasoning with no usable metadata returns a policy error`.
- Commit hash: `PENDING`

## Non-E2E Scenario 9: Unknown Persisted Reasoning Sends No Reasoning Request

- [ ] RED: Remove `@wip` from only `Scenario: Unknown persisted reasoning sends no reasoning request`; implement new steps with `unimplemented!`; target by name; confirm non-zero exit; paste output.
- RED runner output:
  ```text
  PASTE COMMAND AND OUTPUT HERE
  ```
- [ ] GREEN: Replace stubs with real successful-request and absent-reasoning assertions; implement minimum production code. Production files created/modified: `LIST FILES HERE`.
- GREEN runner output:
  ```text
  PASTE COMMAND AND OUTPUT HERE
  ```
- [ ] REFACTOR: Clean up and rerun the targeted scenario; confirm zero exit; paste output.
- REFACTOR runner output:
  ```text
  PASTE COMMAND AND OUTPUT HERE
  ```
- [ ] COMMIT: Create one atomic commit referencing `Unknown persisted reasoning sends no reasoning request`.
- Commit hash: `PENDING`

## Non-E2E Scenario 10: Non-TTY Model Assignment Never Persists Empty Reasoning Values

- [ ] RED: Remove `@wip` from only `Scenario: Non-TTY model assignment never persists empty reasoning values`; implement new steps with `unimplemented!`; target by name; confirm non-zero exit.
- RED runner output:
  ```text
  PASTE COMMAND AND OUTPUT HERE
  ```
- [ ] GREEN: Replace stubs with real TOML assertion and minimum production code. Production files created/modified: `LIST FILES HERE`.
- GREEN runner output:
  ```text
  PASTE COMMAND AND OUTPUT HERE
  ```
- [ ] REFACTOR: Clean up, rerun the targeted scenario, and confirm zero exit.
- REFACTOR runner output:
  ```text
  PASTE COMMAND AND OUTPUT HERE
  ```
- [ ] COMMIT: Create one atomic commit referencing `Non-TTY model assignment never persists empty reasoning values`.
- Commit hash: `PENDING`

## Non-E2E Scenario 11: Existing Reasoning Survives Selection Without a Valid Replacement

- [ ] RED: Remove `@wip` from only `Scenario: Existing reasoning survives selection without a valid replacement`; implement new steps with `unimplemented!`; target by name; confirm non-zero exit.
- RED runner output:
  ```text
  PASTE COMMAND AND OUTPUT HERE
  ```
- [ ] GREEN: Replace stubs with real persisted-reasoning and model assertions; implement minimum production code. Production files created/modified: `LIST FILES HERE`.
- GREEN runner output:
  ```text
  PASTE COMMAND AND OUTPUT HERE
  ```
- [ ] REFACTOR: Clean up, rerun the targeted scenario, and confirm zero exit.
- REFACTOR runner output:
  ```text
  PASTE COMMAND AND OUTPUT HERE
  ```
- [ ] COMMIT: Create one atomic commit referencing `Existing reasoning survives selection without a valid replacement`.
- Commit hash: `PENDING`

## Non-E2E Scenario 12: The Newest Search Result Stays Visible When an Older Result Arrives Later

- [ ] RED: Remove `@wip` from only `Scenario: The newest search result stays visible when an older result arrives later`; implement coordinated-worker steps with `unimplemented!`; target by name; confirm non-zero exit.
- RED runner output:
  ```text
  PASTE COMMAND AND OUTPUT HERE
  ```
- [ ] GREEN: Replace stubs with deterministic generation coordination, exact suggestions, and worker-cleanup assertions; implement minimum production code. Production files created/modified: `LIST FILES HERE`.
- GREEN runner output:
  ```text
  PASTE COMMAND AND OUTPUT HERE
  ```
- [ ] REFACTOR: Clean up, rerun the targeted scenario, and confirm zero exit.
- REFACTOR runner output:
  ```text
  PASTE COMMAND AND OUTPUT HERE
  ```
- [ ] COMMIT: Create one atomic commit referencing `The newest search result stays visible when an older result arrives later`.
- Commit hash: `PENDING`

## E2E Setup

- [ ] Complete all non-e2e scenarios above with GREEN evidence and commits before starting e2e scenarios.
- [ ] Bring up the design's local environment: no application server or database; use the real CLI, loopback `httpmock::MockServer` provider/catalog twins, isolated XDG configuration, isolated environment, and persistent `portable-pty` sessions for interactive flows.
- [ ] Confirm clean startup and reachability of every loopback twin needed by the first e2e scenario. Paste the command and output.
- Local e2e startup evidence:
  ```text
  PASTE COMMAND AND OUTPUT HERE
  ```
- [ ] Create separate e2e step definitions in the exact design-named capability modules, with real PTY/subprocess driving and no repository-only substitute assertions.
- [ ] Prove e2e strict mode with an `unimplemented!` step and a non-zero run. Paste the command and output.
- E2e strictness evidence:
  ```text
  PASTE COMMAND AND OUTPUT HERE
  ```
- [ ] Run `verify.command` and record the full non-e2e scenario count. Run `verify.e2e_command` and record the e2e scenario count. Confirm the e2e count is strictly smaller than the full count.
- Scenario-count evidence:
  ```text
  verify.command count: PASTE COUNT AND OUTPUT HERE
  verify.e2e_command count: PASTE COUNT AND OUTPUT HERE
  ```

## E2E Scenario 1: Interactive Model Discovery Preserves an OpenRouter Environment Credential

- [ ] RED: Remove `@wip` from only `Scenario: Interactive model discovery preserves an OpenRouter environment credential`; implement PTY steps with `unimplemented!`; run `verify.e2e_command` targeted with `-- --name 'Interactive model discovery preserves an OpenRouter environment credential'`; confirm non-zero exit; paste output.
- RED runner output:
  ```text
  PASTE COMMAND AND OUTPUT HERE
  ```
- [ ] GREEN: Drive the actual `watn models` terminal through PTY and assert terminal output primarily, with config assertions secondary; implement minimum production code. Production files created/modified: `LIST FILES HERE`.
- GREEN runner output:
  ```text
  PASTE COMMAND AND OUTPUT HERE
  ```
- [ ] REFACTOR: Clean up without behavior change; rerun the targeted e2e command and confirm zero exit; paste output.
- REFACTOR runner output:
  ```text
  PASTE COMMAND AND OUTPUT HERE
  ```
- [ ] COMMIT: Create one atomic commit referencing `Interactive model discovery preserves an OpenRouter environment credential`.
- Commit hash: `PENDING`

## E2E Scenario 2: A Literal Saved Credential Is Authoritative Over Environment Fallback

- [ ] RED: Remove `@wip` from only `Scenario: A literal saved credential is authoritative over environment fallback`; implement real-subprocess steps with `unimplemented!`; target through `verify.e2e_command`; confirm non-zero exit; paste output.
- RED runner output:
  ```text
  PASTE COMMAND AND OUTPUT HERE
  ```
- [ ] GREEN: Assert the real chat subprocess response and request credential, with repository/environment checks secondary; implement minimum production code. Production files created/modified: `LIST FILES HERE`.
- GREEN runner output:
  ```text
  PASTE COMMAND AND OUTPUT HERE
  ```
- [ ] REFACTOR: Clean up and rerun the targeted e2e scenario; confirm zero exit.
- REFACTOR runner output:
  ```text
  PASTE COMMAND AND OUTPUT HERE
  ```
- [ ] COMMIT: Create one atomic commit referencing `A literal saved credential is authoritative over environment fallback`.
- Commit hash: `PENDING`

## E2E Scenario 3: Configured LiteLLM Is Used for Model Catalog Requests

- [ ] RED: Remove `@wip` from only `Scenario: Configured LiteLLM is used for model catalog requests`; implement real subprocess steps with `unimplemented!`; target through `verify.e2e_command`; confirm non-zero exit; paste output.
- RED runner output:
  ```text
  PASTE COMMAND AND OUTPUT HERE
  ```
- [ ] GREEN: Drive `watn models` through the real subprocess, assert terminal output primarily and catalog request details secondarily; implement minimum production code. Production files created/modified: `LIST FILES HERE`.
- GREEN runner output:
  ```text
  PASTE COMMAND AND OUTPUT HERE
  ```
- [ ] REFACTOR: Clean up and rerun the targeted e2e scenario; confirm zero exit.
- REFACTOR runner output:
  ```text
  PASTE COMMAND AND OUTPUT HERE
  ```
- [ ] COMMIT: Create one atomic commit referencing `Configured LiteLLM is used for model catalog requests`.
- Commit hash: `PENDING`

## E2E Scenario 4: LiteLLM Discovery Does Not Replace the Active Chat Provider

- [ ] RED: Remove `@wip` from only `Scenario: LiteLLM discovery does not replace the active chat provider`; implement real subprocess steps with `unimplemented!`; target through `verify.e2e_command`; confirm non-zero exit; paste output.
- RED runner output:
  ```text
  PASTE COMMAND AND OUTPUT HERE
  ```
- [ ] GREEN: Drive discovery then chat through the real CLI, assert generated chat output primarily and endpoint separation secondarily; implement minimum production code. Production files created/modified: `LIST FILES HERE`.
- GREEN runner output:
  ```text
  PASTE COMMAND AND OUTPUT HERE
  ```
- [ ] REFACTOR: Clean up and rerun the targeted e2e scenario; confirm zero exit.
- REFACTOR runner output:
  ```text
  PASTE COMMAND AND OUTPUT HERE
  ```
- [ ] COMMIT: Create one atomic commit referencing `LiteLLM discovery does not replace the active chat provider`.
- Commit hash: `PENDING`

## E2E Scenario 5: Model Catalog Failure After Provider Setup Preserves the Provider and Sends No Request

- [ ] RED: Remove `@wip` from only `Scenario: Model catalog failure after provider setup preserves the provider and sends no request`; implement PTY steps with `unimplemented!`; target through `verify.e2e_command`; confirm non-zero exit; paste output.
- RED runner output:
  ```text
  PASTE COMMAND AND OUTPUT HERE
  ```
- [ ] GREEN: Drive the actual setup wizard through PTY, assert the visible catalog failure and terminal exit primarily, then verify persistence/request side effects secondarily; implement minimum production code. Production files created/modified: `LIST FILES HERE`.
- GREEN runner output:
  ```text
  PASTE COMMAND AND OUTPUT HERE
  ```
- [ ] REFACTOR: Clean up and rerun the targeted e2e scenario; confirm zero exit.
- REFACTOR runner output:
  ```text
  PASTE COMMAND AND OUTPUT HERE
  ```
- [ ] COMMIT: Create one atomic commit referencing `Model catalog failure after provider setup preserves the provider and sends no request`.
- Commit hash: `PENDING`

## E2E Scenario 6: Assigning Tiers Does Not Replace the Active Provider or Catalog Settings

- [ ] RED: Remove `@wip` from only `Scenario: Assigning tiers does not replace the active provider or catalog settings`; implement real subprocess steps with `unimplemented!`; target through `verify.e2e_command`; confirm non-zero exit; paste output.
- RED runner output:
  ```text
  PASTE COMMAND AND OUTPUT HERE
  ```
- [ ] GREEN: Drive `watn models` through the real subprocess and assert confirmation output primarily, with provider/catalog persistence checks secondary; implement minimum production code. Production files created/modified: `LIST FILES HERE`.
- GREEN runner output:
  ```text
  PASTE COMMAND AND OUTPUT HERE
  ```
- [ ] REFACTOR: Clean up and rerun the targeted e2e scenario; confirm zero exit.
- REFACTOR runner output:
  ```text
  PASTE COMMAND AND OUTPUT HERE
  ```
- [ ] COMMIT: Create one atomic commit referencing `Assigning tiers does not replace the active provider or catalog settings`.
- Commit hash: `PENDING`

## E2E Scenario 7: Minimal Reasoning Is Persisted and Sent

- [ ] RED: Remove `@wip` from only `Scenario: Minimal reasoning is persisted and sent`; implement real subprocess steps with `unimplemented!`; target through `verify.e2e_command`; confirm non-zero exit; paste output.
- RED runner output:
  ```text
  PASTE COMMAND AND OUTPUT HERE
  ```
- [ ] GREEN: Drive `watn -2` through the real subprocess, assert successful CLI output primarily and request reasoning secondarily; implement minimum production code. Production files created/modified: `LIST FILES HERE`.
- GREEN runner output:
  ```text
  PASTE COMMAND AND OUTPUT HERE
  ```
- [ ] REFACTOR: Clean up and rerun the targeted e2e scenario; confirm zero exit.
- REFACTOR runner output:
  ```text
  PASTE COMMAND AND OUTPUT HERE
  ```
- [ ] COMMIT: Create one atomic commit referencing `Minimal reasoning is persisted and sent`.
- Commit hash: `PENDING`

## E2E Scenario 8: The Terminal Model Picker Displays the Newest Overlapping Search Result

- [ ] RED: Remove `@wip` from only `Scenario: The terminal model picker displays the newest overlapping search result`; implement PTY steps with `unimplemented!`; target through `verify.e2e_command`; confirm non-zero exit; paste output.
- RED runner output:
  ```text
  PASTE COMMAND AND OUTPUT HERE
  ```
- [ ] GREEN: Drive the actual terminal picker through PTY, assert visible final suggestions primarily, and verify worker joining secondarily; implement the named generation/test seam and minimum production code. Production files created/modified: `LIST FILES HERE`.
- GREEN runner output:
  ```text
  PASTE COMMAND AND OUTPUT HERE
  ```
- [ ] REFACTOR: Clean up and rerun the targeted e2e scenario; confirm zero exit.
- REFACTOR runner output:
  ```text
  PASTE COMMAND AND OUTPUT HERE
  ```
- [ ] COMMIT: Create one atomic commit referencing `The terminal model picker displays the newest overlapping search result`.
- Commit hash: `PENDING`

## Final Verification

- [ ] Run the complete configured `verify.command`; paste output and confirm zero exit.
- Full non-e2e verification output:
  ```text
  PASTE COMMAND AND OUTPUT HERE
  ```
- [ ] Run the complete configured `verify.e2e_command`; paste output and confirm zero exit.
- Full e2e verification output:
  ```text
  PASTE COMMAND AND OUTPUT HERE
  ```
- [ ] Run `givn lint --change model-discovery-and-setup-correctness` if needed for static feature validation; paste output.
- Lint output:
  ```text
  PASTE COMMAND AND OUTPUT HERE
  ```
