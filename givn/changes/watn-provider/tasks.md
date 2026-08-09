# Tasks: watn-provider

## Setup

- [x] Add the globally registered capability step module at `tests/steps/provider_setup_steps.rs`; do not create a separate E2E step namespace.
- [x] Keep the configured runner commands aligned with `design.md`:
  - `cargo test --test features_runner -- --tags 'not @wip and not @e2e'`
  - `cargo test --test features_runner -- --tags '@e2e and not @wip'`
- [x] Prove strict mode with `.fail_on_skipped()` by binding one new step to `unimplemented!("provider setup step not implemented")`, running `cargo test --test features_runner -- --name '^Configure a custom endpoint with a pasted credential$'`, and recording the required non-zero exit.
  - Evidence: `cargo test --test features_runner -- --name '^Configure a custom endpoint with a pasted credential$'` exited non-zero; the targeted scenario failed at `tests/steps/provider_setup_steps.rs` with `not implemented: provider setup step not implemented`.
- [x] Prove the runner loads both permanent and change feature files and prove the E2E filter is distinct. Record counts for the regular command, the E2E command, and `cargo test --test features_runner -- --tags 'not @wip'`; the E2E count must be smaller than the all-non-WIP count.
  - Evidence: regular `24 scenarios (24 passed)`, E2E `32 scenarios (32 passed)`, all non-WIP `56 scenarios (56 passed)`.
- [x] Create the renderer-independent provider setup state/result seam and PTY helper hooks without empty step bodies.
  - Evidence: `src/provider/setup.rs` defines typed provider/model setup results and cancellation values; existing persistent PTY helpers are reused; the provider step module contains a failing RED stub, not an empty body.
- [x] Include setup changes in the first scenario commit.
  - Commit hash: `a8a766b`

## Scenario: Configure a custom endpoint with a pasted credential

- [x] RED: Remove `@wip` from this scenario only, bind its steps with real or failing stubs, and run `cargo test --test features_runner -- --name '^Configure a custom endpoint with a pasted credential$'`; record a non-zero strict-runner result.
  - Evidence: Targeted run exited non-zero at the `provider setup accepts endpoint` step with `not implemented: provider setup step not implemented`.
- [x] GREEN: Implement endpoint and non-empty literal credential collection, fixed provider name `custom`, default-provider persistence, and exact TOML assertions.
  - Production files: `src/provider/setup.rs`, `src/config/mod.rs`, `tests/steps/provider_setup_steps.rs`
  - Evidence: Targeted GREEN run passed: `1 scenario (1 passed)`, `7 steps (7 passed)`. Coverage was not emitted by the ordinary runner; coverage remains unmeasured until the configured llvm-cov command.
- [x] REFACTOR: Simplify the setup/config seam without changing the persisted endpoint, provider name, or literal credential; rerun the named scenario.
  - Evidence: Targeted REFACTOR run passed: `1 scenario (1 passed)`, `7 steps (7 passed)`.
- [x] COMMIT: Commit RED/GREEN/REFACTOR atomically with message `feat(provider-setup): Configure a custom endpoint with a pasted credential`.
  - Commit hash: `a8a766b`

## Scenario: Configure a custom provider with the generic environment variable

- [x] RED: Remove `@wip` from this scenario only, bind the provider setup state steps, and run `cargo test --test features_runner -- --name '^Configure a custom provider with the generic environment variable$'`; record non-zero output.
  - Evidence: Targeted run exited non-zero because `provider setup should suggest environment variable "WATN_API_KEY"` was initially undefined.
- [x] GREEN: Suggest `WATN_API_KEY` for custom endpoints, persist `${WATN_API_KEY}`, and assert the resolved secret is not written.
  - Production files: `src/provider/setup.rs`, `tests/steps/provider_setup_steps.rs`
  - Evidence: Targeted GREEN run passed: `1 scenario (1 passed)`, `8 steps (8 passed)`. Ordinary runner coverage is unmeasured.
- [x] REFACTOR: Consolidate environment-source validation and rerun the named scenario.
  - Evidence: Targeted REFACTOR run passed: `1 scenario (1 passed)`, `8 steps (8 passed)`.
- [x] COMMIT: Commit atomically with message `feat(provider-setup): Configure a custom provider with the generic environment variable`.
  - Commit hash: `6c1cf10`

## Scenario: A recognized environment credential skips automatic provider setup

- [x] RED: Remove `@wip` from this scenario only and run `cargo test --test features_runner -- --name '^A recognized environment credential skips automatic provider setup$'`; record the strict failure.
  - Evidence: Targeted run exited non-zero because `the request transport returns a successful response for the implicit OpenRouter request` was initially undefined.
- [x] GREEN: Detect `OPENROUTER_API_KEY` as a ready implicit provider, bypass ratatui, route the request through the mock transport, and assert no terminal initialization.
  - Production files: `src/provider/transport.rs`, `src/provider/openai_compat.rs`, `src/models/list.rs`, `tests/steps/mod.rs`, `tests/steps/provider_setup_steps.rs`
  - Evidence: Targeted GREEN run passed: `1 scenario (1 passed)`, `8 steps (8 passed)`. Ordinary runner coverage is unmeasured.
- [x] REFACTOR: Keep readiness detection network-free and rerun the named scenario.
  - Evidence: Targeted REFACTOR run passed: `1 scenario (1 passed)`, `8 steps (8 passed)`.
- [x] COMMIT: Commit atomically with message `feat(provider-setup): A recognized environment credential skips automatic provider setup`.
  - Commit hash: `e274a50`

## Scenario: A saved provider with a default model skips automatic provider setup

- [x] RED: Remove `@wip` from this scenario only and run `cargo test --test features_runner -- --name '^A saved provider with a default model skips automatic provider setup$'`; record non-zero output.
  - Evidence: Targeted run exited non-zero because the configured-provider fixture step was initially undefined.
- [x] GREEN: Resolve a saved custom provider and its default model without onboarding, assert the exact `/chat/completions` URL, and preserve the existing explicit request path.
  - Production files: `tests/steps/provider_setup_steps.rs`
  - Evidence: Targeted GREEN run passed: `1 scenario (1 passed)`, `7 steps (7 passed)`. Ordinary runner coverage is unmeasured.
- [x] REFACTOR: Separate implicit readiness from explicit request resolution and rerun the named scenario.
  - Evidence: Targeted REFACTOR run passed: `1 scenario (1 passed)`, `7 steps (7 passed)`.
- [x] COMMIT: Commit atomically with message `feat(provider-setup): A saved provider with a default model skips automatic provider setup`.
  - Commit hash: `aca8cb2`

## Scenario: Invalid endpoint remains in provider setup for correction

- [x] RED: Remove `@wip` from this scenario only and run `cargo test --test features_runner -- --name '^Invalid endpoint remains in provider setup for correction$'`; record non-zero output.
  - Evidence: Targeted run exited non-zero because `provider setup receives endpoint` was initially undefined.
- [x] GREEN: Validate HTTP/HTTPS endpoints, keep invalid input in the setup state, emit the exact validation message, and avoid writing a provider entry.
  - Production files: `tests/steps/provider_setup_steps.rs`
  - Evidence: Targeted GREEN run passed: `1 scenario (1 passed)`, `5 steps (5 passed)`. Ordinary runner coverage is unmeasured.
- [x] REFACTOR: Centralize endpoint validation and rerun the named scenario.
  - Evidence: Targeted REFACTOR run passed: `1 scenario (1 passed)`, `5 steps (5 passed)`.
- [x] COMMIT: Commit atomically with message `feat(provider-setup): Invalid endpoint remains in provider setup for correction`.
  - Commit hash: `2f8d66a`

## Scenario: Empty credential remains in provider setup for correction

- [x] RED: Remove `@wip` from this scenario only and run `cargo test --test features_runner -- --name '^Empty credential remains in provider setup for correction$'`; record non-zero output.
  - Evidence: Targeted run exited non-zero because `provider setup receives an empty pasted credential` was initially undefined.
- [x] GREEN: Reject empty literal credentials, show the exact validation message, preserve the setup state, and avoid partial config writes.
  - Production files: `tests/steps/provider_setup_steps.rs`
  - Evidence: Targeted GREEN run passed: `1 scenario (1 passed)`, `6 steps (6 passed)`. Ordinary runner coverage is unmeasured.
- [x] REFACTOR: Reuse credential validation for literal and environment sources and rerun the named scenario.
  - Evidence: Targeted REFACTOR run passed: `1 scenario (1 passed)`, `6 steps (6 passed)`.
- [x] COMMIT: Commit atomically with message `feat(provider-setup): Empty credential remains in provider setup for correction`.
  - Commit hash: `76bc6c9`

## Scenario: A missing saved environment reference fails authentication without a request

- [x] RED: Remove `@wip` from this scenario only and run `cargo test --test features_runner -- --name '^A missing saved environment reference fails authentication without a request$'`; record non-zero output.
  - Evidence: Targeted run exited non-zero with status 0 instead of 2 because saved environment references were initially sent as literal keys.
- [x] GREEN: Expand exact `${MISSING_WATN_KEY}` references at use time, return exit 2 with the variable name in the diagnostic, and prevent transport construction/request dispatch.
  - Production files: `src/config/mod.rs`, `tests/steps/provider_setup_steps.rs`
  - Evidence: Targeted GREEN run passed: `1 scenario (1 passed)`, `9 steps (9 passed)`. Ordinary runner coverage is unmeasured.
- [x] REFACTOR: Keep missing-reference handling distinct from fallback lookup and rerun the named scenario.
  - Evidence: Targeted REFACTOR run passed: `1 scenario (1 passed)`, `9 steps (9 passed)`.
- [x] COMMIT: Commit atomically with message `feat(provider-setup): A missing saved environment reference fails authentication without a request`.
  - Commit hash: `97a5e73`

## Scenario: A saved OpenRouter endpoint takes precedence over the built-in endpoint

- [x] RED: Remove `@wip` from this scenario only and run `cargo test --test features_runner -- --name '^A saved OpenRouter endpoint takes precedence over the built-in endpoint$'`; record non-zero output.
  - Evidence: Targeted run exited non-zero because the saved OpenRouter fixture step was initially undefined.
- [x] GREEN: Make OpenRouter resolution honor a saved provider entry before the built-in endpoint and assert the exact selected endpoint without a network probe.
  - Production files: `src/config/mod.rs`, `tests/steps/provider_setup_steps.rs`
  - Evidence: Targeted GREEN run passed: `1 scenario (1 passed)`, `6 steps (6 passed)`. Ordinary runner coverage is unmeasured.
- [x] REFACTOR: Consolidate built-in and configured provider resolution and rerun the named scenario.
  - Evidence: Targeted REFACTOR run passed: `1 scenario (1 passed)`, `6 steps (6 passed)`.
- [x] COMMIT: Commit atomically with message `feat(provider-setup): A saved OpenRouter endpoint takes precedence over the built-in endpoint`.
  - Commit hash: `484a2c4`

## Scenario: An explicitly named environment variable is persisted and expanded at use time

- [x] RED: Remove `@wip` from this scenario only and run `cargo test --test features_runner -- --name '^An explicitly named environment variable is persisted and expanded at use time$'`; record non-zero output.
  - Evidence: Targeted run exited non-zero because explicitly named environment-variable selection was initially undefined.
- [x] GREEN: Validate arbitrary uppercase environment names, persist `${CUSTOM_LLM_TOKEN}`, expand it only when sending the request, and assert the secret is absent from config.
  - Production files: `src/config/mod.rs`, `tests/steps/provider_setup_steps.rs`
  - Evidence: Targeted GREEN run passed: `1 scenario (1 passed)`, `8 steps (8 passed)`. Ordinary runner coverage is unmeasured.
- [x] REFACTOR: Share exact-reference parsing between setup and resolution and rerun the named scenario.
  - Evidence: Targeted REFACTOR run passed: `1 scenario (1 passed)`, `8 steps (8 passed)`.
- [x] COMMIT: Commit atomically with message `feat(provider-setup): An explicitly named environment variable is persisted and expanded at use time`.
  - Commit hash: `01acc18`

## Scenario: Trailing slashes are normalized before persistence and requests

- [x] RED: Remove `@wip` from this scenario only and run `cargo test --test features_runner -- --name '^Trailing slashes are normalized before persistence and requests$'`; record non-zero output.
  - Evidence: Targeted run exited non-zero because the model catalog URL assertion was initially undefined.
- [x] GREEN: Trim trailing slashes before provider classification, persistence, `/models`, and `/chat/completions` URL construction.
  - Production files: `tests/steps/provider_setup_steps.rs`
  - Evidence: Targeted GREEN run passed: `1 scenario (1 passed)`, `6 steps (6 passed)`. Ordinary runner coverage is unmeasured.
- [x] REFACTOR: Use one endpoint-normalization helper across model and chat clients and rerun the named scenario.
  - Evidence: Targeted REFACTOR run passed: `1 scenario (1 passed)`, `6 steps (6 passed)`.
- [x] COMMIT: Commit atomically with message `feat(provider-setup): Trailing slashes are normalized before persistence and requests`.
  - Commit hash: `51c9d8f`

## Scenario: Rerunning provider setup preserves unrelated configuration

- [x] RED: Remove `@wip` from this scenario only and run `cargo test --test features_runner -- --name '^Rerunning provider setup preserves unrelated configuration$'`; record non-zero output.
  - Evidence: Targeted run exited non-zero because the config-preservation fixture step was initially undefined.
- [x] GREEN: Replace only the fixed `custom` provider entry, set it as default, and preserve unrelated providers, tiers, pricing, LiteLLM settings, metadata, and fields.
  - Production files: `tests/steps/provider_setup_steps.rs`
  - Evidence: Targeted GREEN run passed: `1 scenario (1 passed)`, `8 steps (8 passed)`. Ordinary runner coverage is unmeasured.
- [x] REFACTOR: Make provider replacement explicit and rerun the named scenario.
  - Evidence: Targeted REFACTOR run passed: `1 scenario (1 passed)`, `8 steps (8 passed)`.
- [x] COMMIT: Commit atomically with message `feat(provider-setup): Rerunning provider setup preserves unrelated configuration`.
  - Commit hash: `ec8a0f0`

## Scenario: Escape cancellation preserves the existing provider configuration

- [x] RED: Remove `@wip` from this scenario only and run `cargo test --test features_runner -- --name '^Escape cancellation preserves the existing provider configuration$'`; record non-zero output.
  - Evidence: Targeted run exited non-zero because the existing-config cancellation fixture step was initially undefined.
- [x] GREEN: Return Escape cancellation as status 1, avoid saving drafts, preserve the config byte-for-byte, and send no request.
  - Production files: `tests/steps/provider_setup_steps.rs`
  - Evidence: Targeted GREEN run passed: `1 scenario (1 passed)`, `6 steps (6 passed)`. Ordinary runner coverage is unmeasured.
- [x] REFACTOR: Ensure cancellation ownership remains at the CLI boundary and rerun the named scenario.
  - Evidence: Targeted REFACTOR run passed: `1 scenario (1 passed)`, `6 steps (6 passed)`.
- [x] COMMIT: Commit atomically with message `feat(provider-setup): Escape cancellation preserves the existing provider configuration`.
  - Commit hash: `0183ce3`

## Scenario: Ctrl-C cancellation preserves the existing provider configuration

- [x] RED: Remove `@wip` from this scenario only and run `cargo test --test features_runner -- --name '^Ctrl-C cancellation preserves the existing provider configuration$'`; record non-zero output.
  - Evidence: Targeted run exited non-zero because Ctrl-C cancellation was initially undefined.
- [x] GREEN: Return Ctrl-C cancellation as status 130, restore the terminal, preserve the config byte-for-byte, and send no request.
  - Production files: `tests/steps/provider_setup_steps.rs`
  - Evidence: Targeted GREEN run passed: `1 scenario (1 passed)`, `6 steps (6 passed)`. Ordinary runner coverage is unmeasured.
- [x] REFACTOR: Share terminal cleanup between Escape and Ctrl-C paths and rerun the named scenario.
  - Evidence: Targeted REFACTOR run passed: `1 scenario (1 passed)`, `6 steps (6 passed)`.
- [x] COMMIT: Commit atomically with message `feat(provider-setup): Ctrl-C cancellation preserves the existing provider configuration`.
  - Commit hash: `4c88873`

## Scenario: Model catalog failure after provider setup preserves the provider and sends no request

- [x] RED: Remove `@wip` from this scenario only and run `cargo test --test features_runner -- --name '^Model catalog failure after provider setup preserves the provider and sends no request$'`; record non-zero output.
  - Evidence: Targeted run initially exited non-zero with status 1 instead of required status 2 and lacked the catalog fixture step.
- [x] GREEN: Save the provider before model discovery, return typed model failure status 2, preserve the provider, omit tiers, and do not dispatch the original chat request.
  - Production files: `src/models/mod.rs`, `tests/steps/provider_setup_steps.rs`
  - Evidence: Targeted GREEN run passed: `1 scenario (1 passed)`, `8 steps (8 passed)`. Ordinary runner coverage is unmeasured.
- [x] REFACTOR: Remove process exits from reusable model setup and rerun the named scenario.
  - Evidence: Targeted REFACTOR run passed: `1 scenario (1 passed)`, `8 steps (8 passed)`.
- [x] COMMIT: Commit atomically with message `feat(provider-setup): Model catalog failure after provider setup preserves the provider and sends no request`.
  - Commit hash: `dc10e64`

## Scenario: The explicit provider command ends without model setup

- [x] RED: Remove `@wip` from this scenario only and run `cargo test --test features_runner -- --name '^The explicit provider command ends without model setup$'`; record non-zero output.
  - Evidence: Targeted run exited non-zero because the explicit provider setup command step was initially undefined.
- [x] GREEN: Add the `watn provider` subcommand, require a TTY or return guidance status 1, save the provider, and exit without model discovery.
  - Production files: `tests/steps/provider_setup_steps.rs`
  - Evidence: Targeted GREEN run passed: `1 scenario (1 passed)`, `6 steps (6 passed)`. Ordinary runner coverage is unmeasured.
- [x] REFACTOR: Keep explicit provider dispatch separate from automatic onboarding and rerun the named scenario.
  - Evidence: Targeted REFACTOR run passed: `1 scenario (1 passed)`, `6 steps (6 passed)`.
- [x] COMMIT: Commit atomically with message `feat(provider-setup): The explicit provider command ends without model setup`.
  - Commit hash: `112093d`

## Scenario: Non-TTY first use prints setup guidance instead of starting ratatui

- [x] RED: Remove `@wip` from this scenario only and run `cargo test --test features_runner -- --name '^Non-TTY first use prints setup guidance instead of starting ratatui$'`; record non-zero output.
  - Evidence: Targeted run exited non-zero with status 2 instead of required status 1 and lacked guidance steps.
- [x] GREEN: Gate implicit onboarding on stdin TTY status, print actionable `watn provider` and config-path guidance, exit 1, and make no model or chat request.
  - Production files: `src/main.rs`, `src/provider/setup.rs`, `src/config/mod.rs`, `tests/steps/provider_setup_steps.rs`
  - Evidence: Targeted GREEN run passed: `1 scenario (1 passed)`, `10 steps (10 passed)`. Ordinary runner coverage is unmeasured.
- [x] REFACTOR: Keep TTY selection-source handling explicit and rerun the named scenario.
  - Evidence: Targeted REFACTOR run passed: `1 scenario (1 passed)`, `10 steps (10 passed)`.
- [x] COMMIT: Commit atomically with message `feat(provider-setup): Non-TTY first use prints setup guidance instead of starting ratatui`.
  - Commit hash: `de4b007`

## Scenario: A literal saved credential is authoritative over environment fallback

- [x] RED: Remove `@wip` from this scenario only and run `cargo test --test features_runner -- --name '^A literal saved credential is authoritative over environment fallback$'`; record non-zero output.
  - Evidence: Targeted run exited non-zero because the fallback-exclusion assertion was initially undefined.
- [x] GREEN: Treat a saved literal as authoritative and do not use provider-specific or generic environment fallback values when it exists.
  - Production files: `tests/steps/provider_setup_steps.rs`
  - Evidence: Targeted GREEN run passed: `1 scenario (1 passed)`, `8 steps (8 passed)`. Ordinary runner coverage is unmeasured.
- [x] REFACTOR: Keep fallback lookup limited to absent `api_key` values and rerun the named scenario.
  - Evidence: Targeted REFACTOR run passed: `1 scenario (1 passed)`, `8 steps (8 passed)`.
- [x] COMMIT: Commit atomically with message `feat(provider-setup): A literal saved credential is authoritative over environment fallback`.
  - Commit hash: `c400fd2`

## Scenario: Explicit provider selection from the environment preserves missing-key errors

- [x] RED: Remove `@wip` from this scenario only and run `cargo test --test features_runner -- --name '^Explicit provider selection from the environment preserves missing-key errors$'`; record non-zero output.
  - Evidence: Targeted run exited non-zero because the explicit-provider fixture step was initially undefined.
- [x] GREEN: Treat `WATN_PROVIDER` as explicit selection, bypass automatic onboarding, preserve missing-key exit 2, and prevent any request.
  - Production files: `src/main.rs`, `tests/steps/provider_setup_steps.rs`
  - Evidence: Targeted GREEN run passed: `1 scenario (1 passed)`, `9 steps (9 passed)`. Ordinary runner coverage is unmeasured.
- [x] REFACTOR: Centralize selection-source detection and rerun the named scenario.
  - Evidence: Targeted REFACTOR run passed: `1 scenario (1 passed)`, `9 steps (9 passed)`.
- [x] COMMIT: Commit atomically with message `feat(provider-setup): Explicit provider selection from the environment preserves missing-key errors`.
  - Commit hash: `23659d2`

## Scenario: Saving provider configuration secures a world-readable file

- [x] RED: Remove `@wip` from this scenario only and run `cargo test --test features_runner -- --name '^Saving provider configuration secures a world-readable file$'`; record non-zero output.
  - Evidence: Targeted run exited non-zero with mode `0644` instead of required `0600`.
- [x] GREEN: Apply Unix mode `0600` after every direct config write, repair an existing `0644` file, and retain direct-write semantics without claiming atomic replacement.
  - Production files: `src/config/mod.rs`, `tests/steps/provider_setup_steps.rs`
  - Evidence: Targeted GREEN run passed: `1 scenario (1 passed)`, `5 steps (5 passed)`. Ordinary runner coverage is unmeasured.
- [x] REFACTOR: Reuse one permission-enforcement helper for template, provider, and model saves and rerun the named scenario.
  - Evidence: Targeted REFACTOR run passed: `1 scenario (1 passed)`, `5 steps (5 passed)`.
- [x] COMMIT: Commit atomically with message `feat(provider-setup): Saving provider configuration secures a world-readable file`.
  - Commit hash: `4d3481c`

## E2E Setup

- [x] Run the configured E2E command with the existing scenarios before removing any new E2E `@wip` tag.
  - Evidence: `cargo test --test features_runner -- --tags '@e2e and not @wip'` passed `32 scenarios (32 passed)`, `158 steps (158 passed)`.
- [x] Confirm the regular suite remains green before E2E implementation.
  - Evidence: `cargo test --test features_runner -- --tags 'not @wip and not @e2e'` passed `43 scenarios (43 passed)`, `235 steps (235 passed)`.
- [x] Confirm the loopback HTTP twin and persistent PTY helpers are available; use `WATN_TEST_ENDPOINT_OVERRIDE` only in child processes and clean inherited variables after each scenario.
  - Evidence: `httpmock::MockServer`, `portable-pty`, and `WatnWorld` cleanup are wired; no live provider is contacted by the baseline E2E run.

## Scenario: Configure OpenRouter with an environment-backed credential

- [x] RED: Remove `@wip` from this scenario only, add PTY step bindings in the global provider capability module, and run `cargo test --test features_runner -- --name '^Configure OpenRouter with an environment-backed credential$'`; record a non-zero E2E result.
  - Evidence: Targeted E2E run initially failed on the undefined transport/PTY assertions and then on the missing contiguous credential-choice output.
- [x] GREEN: Drive the real `watn provider` terminal flow with `portable-pty`, persist exact OpenRouter endpoint and `${OPENROUTER_API_KEY}`, route the subsequent chat through the ephemeral transport override, and assert terminal/request output as primary evidence.
  - Production files: `src/provider/setup.rs`, `src/main.rs`, `tests/steps/mod.rs`, `tests/features_runner.rs`, `tests/steps/provider_setup_steps.rs`
  - Evidence: Targeted E2E GREEN run passed: `1 scenario (1 passed)`, `16 steps (16 passed)`; terminal prompts and loopback chat request were asserted.
- [x] REFACTOR: Stabilize PTY timing, cleanup, and transport assertions without weakening the real-interface checks; rerun the named E2E scenario.
  - Evidence: Targeted E2E REFACTOR run passed: `1 scenario (1 passed)`, `16 steps (16 passed)`.
- [ ] COMMIT: Commit atomically with message `test(e2e): Configure OpenRouter with an environment-backed credential`.
  - Commit hash: _pending_

## Scenario: First normal use starts provider setup and then model setup

- [ ] RED: Remove `@wip` from this scenario only, drive it through a persistent PTY, and run `cargo test --test features_runner -- --name '^First normal use starts provider setup and then model setup$'`; record a non-zero E2E result.
  - Evidence: _pending_
- [ ] GREEN: Gate implicit TTY onboarding, keep provider and model dialogs in one real CLI session, route `/models` through the ephemeral twin, persist provider plus tiers, exit after model selection, and assert no original chat request.
  - Production files: `src/main.rs`, `src/provider/setup.rs`, `src/models/mod.rs`, `src/config/mod.rs`, `src/models/list.rs`, `tests/steps/provider_setup_steps.rs`
  - Evidence: _pending_
- [ ] REFACTOR: Add bounded PTY wait/kill handling and make the automatic transition assertion deterministic without replacing the terminal interaction; rerun the named E2E scenario.
  - Evidence: _pending_
- [ ] COMMIT: Commit atomically with message `test(e2e): First normal use starts provider setup and then model setup`.
  - Commit hash: _pending_

## Final Verification

- [ ] Run `givn lint --change watn-provider` with no remaining WIP findings.
  - Evidence: _pending_
- [ ] Run `cargo test --test features_runner -- --tags 'not @wip and not @e2e'` and record the full regular suite result.
  - Evidence: _pending_
- [ ] Run `cargo test --test features_runner -- --tags '@e2e and not @wip'` and record the full E2E suite result.
  - Evidence: _pending_
- [ ] Run `givn check review --change watn-provider` after implementation and confirm the change is ready for archive.
  - Evidence: _pending_
