# Tasks: streamlined-setup-flow

## Setup: strict feature runner and step skeleton

- [x] Configure `./run-tests.sh` and `./run-tests.sh --e2e` as the verification commands; keep `Cucumber::fail_on_skipped()` strictness; create/register `tests/steps/streamlined_setup_steps.rs` and `tests/steps/streamlined_setup_e2e_steps.rs`; prove an undefined or `unimplemented!()` step exits non-zero. Evidence: `./run-tests.sh` exited nonzero with `Step doesn't match any function` for the active setup scenario; summary was `104 scenarios (103 passed, 1 failed)` and `595 steps (594 passed, 1 failed)`.
- [x] Confirm the runner executes permanent `givn/specs/**` and active change `specs/**`, and confirm the E2E tag filter is a strict subset. Evidence: the failed run collected the active `Streamlined setup flow` feature before permanent features; `run-tests.sh` uses `not @wip and not @e2e`, while `run-tests.sh --e2e` uses `@e2e and not @wip`.
- [x] Setup production files changed: no production files changed during runner setup; the test skeleton is intentionally isolated and production implementation begins in the first scenario GREEN phase. Evidence: `tests/steps/mod.rs`, `tests/steps/streamlined_setup_steps.rs`, and `tests/steps/streamlined_setup_e2e_steps.rs` compile with the existing runner.
- [x] Setup commit hash: `b6924d7`

## Non-E2E Scenarios

### Coordinated setup displays one separate reasoning question after each model

- [x] RED: remove only this scenario's `@wip`, add strict stubs, and run `./run-tests.sh --name "Coordinated setup displays one separate reasoning question after each model"`. Evidence: non-zero; `Step doesn't match any function` at the active scenario's `advance to the small model question` step.
- [x] GREEN: implement separate model/reasoning question state and assertions. Production files: `src/setup.rs`; test/runner files: `tests/steps/streamlined_setup_steps.rs`, `run-tests.sh`. Evidence: targeted run passed: `1 scenario (1 passed)`, `8 steps (8 passed)`.
- [x] REFACTOR: clean up without behavior change and rerun the same command. Evidence: post-`cargo fmt --all` targeted run passed: `1 scenario (1 passed)`, `8 steps (8 passed)`.
- [x] COMMIT: commit message references `Coordinated setup displays one separate reasoning question after each model`. Hash: `1e165d5`

### Rerunning coordinated setup prefills current values and preserves a masked literal credential

- [x] RED: target `Rerunning coordinated setup prefills current values and preserves a masked literal credential`; remove only its `@wip`; evidence: non-zero; the first unimplemented `provider credential` step reported `Step doesn't match any function`.
- [x] GREEN: implement prefilled draft values and masked credential preservation. Production files: `src/setup.rs`; test files: `tests/steps/streamlined_setup_steps.rs`, `tests/steps/mod.rs`; spec: active scenario. Evidence: targeted run passed: `1 scenario (1 passed)`, `10 steps (10 passed)`.
- [x] REFACTOR: rerun the named scenario after cleanup. Evidence: post-format targeted run passed with `10 steps (10 passed)`; first separate-reasoning scenario also remained green after the provider page addition.
- [x] COMMIT: commit title references `Rerunning coordinated setup prefills current values and preserves a masked literal credential`. Hash: `0bfc8a9`

### Cancelling coordinated setup leaves an existing configuration unchanged

- [x] RED: target `Cancelling coordinated setup leaves an existing configuration unchanged`; evidence: non-zero; the first new `existing config content is recorded` step was undefined.
- [x] GREEN: implement baseline recording and real PTY Escape/discard cancellation assertion. Production files: none beyond existing cancellation path; test/spec files: `tests/steps/streamlined_setup_steps.rs`, active feature. Evidence: targeted run passed: `1 scenario (1 passed)`, `5 steps (5 passed)`.
- [x] REFACTOR: rerun the named scenario after `cargo fmt --all`. Evidence: targeted run passed with `5 steps (5 passed)`.
- [x] COMMIT: commit title references `Cancelling coordinated setup leaves an existing configuration unchanged`. Hash: `c19ba7b`

### Provider setup requires a custom endpoint

- [x] RED: target `Provider setup requires a custom endpoint`; evidence: non-zero; the first new `choose provider "Custom"` step was undefined.
- [x] GREEN: implement explicit provider choice rendering and empty Custom endpoint validation. Production files: `src/setup.rs`; test files: `tests/steps/streamlined_setup_steps.rs`; spec: active scenario. Evidence: targeted run passed: `1 scenario (1 passed)`, `6 steps (6 passed)`.
- [x] REFACTOR: rerun the named scenario after `cargo fmt --all`. Evidence: targeted run passed with `6 steps (6 passed)`.
- [x] COMMIT: commit title references `Provider setup requires a custom endpoint`. Hash: `a525c6d`

### Provider setup refuses an unresolved environment credential

- [x] RED: target `Provider setup refuses an unresolved environment credential`; evidence: non-zero; the new unresolved-environment assertion step was undefined.
- [x] GREEN: validate the unresolved environment source through the existing provider setup seam and prevent a provider write. Production files: existing validation/persistence path reused; test files: `tests/steps/provider_setup_steps.rs`, `tests/steps/streamlined_setup_steps.rs`; Evidence: targeted run passed: `1 scenario (1 passed)`, `5 steps (5 passed)`.
- [x] REFACTOR: rerun the named scenario after `cargo fmt --all`. Evidence: targeted run passed with `5 steps (5 passed)`.
- [x] COMMIT: commit title references `Provider setup refuses an unresolved environment credential`. Hash: `5e97e24`

### Provider setup preserves unrelated settings

- [x] RED: target `Provider setup preserves unrelated settings`; evidence: non-zero; the new combined provider-save step was undefined.
- [x] GREEN: exercise the existing provider persistence seam with concrete preservation assertions for unrelated provider, pricing, and LiteLLM data. Production files: existing config writer reused; test files: `tests/steps/streamlined_setup_steps.rs`; Evidence: targeted run passed: `1 scenario (1 passed)`, `5 steps (5 passed)`.
- [x] REFACTOR: rerun the named scenario after `cargo fmt --all`. Evidence: targeted run passed with `5 steps (5 passed)`.
- [x] COMMIT: commit title references `Provider setup preserves unrelated settings`. Hash: `22097ad`

### Provider setup does not probe the catalog

- [x] RED: target `Provider setup does not probe the catalog`; evidence: non-zero; the existing no-catalog assertion found no catalog mock because the new save step was not implemented.
- [x] GREEN: add an exact zero-hit catalog twin around the provider save seam; provider save performs no discovery request. Production files: existing provider path reused; test/spec files: `tests/steps/streamlined_setup_steps.rs`. Evidence: targeted run passed: `1 scenario (1 passed)`, `3 steps (3 passed)`.
- [x] REFACTOR: rerun the named scenario after `cargo fmt --all`. Evidence: targeted run passed with `3 steps (3 passed)`.
- [x] COMMIT: commit title references `Provider setup does not probe the catalog`. Hash: `28039cd`

### Models setup gives guidance when no provider is configured

- [x] RED: target `Models setup gives guidance when no provider is configured`; evidence: non-zero; the new non-TTY models invocation step was undefined.
- [x] GREEN: make `watn models` print focused provider guidance without opening UI. Production files: `src/models/mod.rs`; test files: `tests/steps/streamlined_setup_steps.rs`. Evidence: targeted run passed: `1 scenario (1 passed)`, `4 steps (4 passed)`.
- [x] REFACTOR: rerun the named scenario after `cargo fmt --all`. Evidence: targeted run passed with `4 steps (4 passed)`.
- [x] COMMIT: commit title references `Models setup gives guidance when no provider is configured`. Hash: `ec16b0e`

### Available catalog restricts model choices

- [x] RED: target `Available catalog restricts model choices`; evidence: non-zero; the two-model catalog Given step was undefined.
- [x] GREEN: drive the real model picker against an isolated catalog and assert stale saved model absence plus catalog-only choices. Production files: existing picker reused; test files: `tests/steps/streamlined_setup_steps.rs`; Evidence: targeted run passed: `1 scenario (1 passed)`, `5 steps (5 passed)`.
- [x] REFACTOR: rerun the named scenario after `cargo fmt --all`. Evidence: targeted run passed with `5 steps (5 passed)`.
- [x] COMMIT: commit title references `Available catalog restricts model choices`. Hash: `97ecf3c`

### Unavailable catalog allows manual model identifiers

- [x] RED: target `Unavailable catalog allows manual model identifiers`; evidence: non-zero; the unreachable-catalog Given step was undefined.
- [x] GREEN: implement catalog failure fallback and visible manual model mode. Production files: `src/setup.rs`; test files: `tests/steps/streamlined_setup_steps.rs`; Evidence: targeted run passed: `1 scenario (1 passed)`, `4 steps (4 passed)`.
- [x] REFACTOR: rerun the named scenario after `cargo fmt --all`. Evidence: targeted run passed with `4 steps (4 passed)`.
- [x] COMMIT: commit title references `Unavailable catalog allows manual model identifiers`. Hash: `17247e4`

### Catalog metadata selects supported reasoning efforts for the chosen model

- [x] RED: target `Catalog metadata selects supported reasoning efforts for the chosen model`; evidence: non-zero; the catalog metadata Given step was undefined.
- [x] GREEN: implement metadata-supported effort filtering and default selection, and drive the provider catalog twin. Production files: `src/setup.rs`; test files: `tests/steps/streamlined_setup_steps.rs`; Evidence: targeted run passed: `1 scenario (1 passed)`, `6 steps (6 passed)`.
- [x] REFACTOR: rerun the named scenario after `cargo fmt --all`. Evidence: targeted run passed with `6 steps (6 passed)`.
- [x] COMMIT: commit title references `Catalog metadata selects supported reasoning efforts for the chosen model`. Hash: `533ac96`

### Missing reasoning metadata provides generic efforts and free-form input

- [x] RED: target `Missing reasoning metadata provides generic efforts and free-form input`; evidence: non-zero; the no-metadata catalog Given step was undefined.
- [x] GREEN: implement generic reasoning choices including `minimal`, metadata warning, custom input, and string-valued level choices. Production files: `src/models/dialog.rs`, `src/setup.rs`; test files: `tests/steps/streamlined_setup_steps.rs`. Evidence: targeted run passed: `1 scenario (1 passed)`, `8 steps (8 passed)`.
- [x] REFACTOR: rerun the named scenario after `cargo fmt --all`. Evidence: targeted run passed with `8 steps (8 passed)`.
- [x] COMMIT: commit title references `Missing reasoning metadata provides generic efforts and free-form input`. Hash: `37d93dc`

### Off reasoning omits the reasoning setting from a request

- [x] RED: target `Off reasoning omits the reasoning setting from a request`; evidence: non-zero; the small-role request step was undefined.
- [x] GREEN: drive a real small-tier request against a blocking reasoning-body twin and assert successful omission. Production files: existing request policy reused; test files: `tests/steps/streamlined_setup_steps.rs`; Evidence: targeted run passed: `1 scenario (1 passed)`, `4 steps (4 passed)`.
- [x] REFACTOR: rerun the named scenario after `cargo fmt --all`. Evidence: targeted run passed with `4 steps (4 passed)`.
- [x] COMMIT: commit title references `Off reasoning omits the reasoning setting from a request`. Hash: `36a6c30`

### Shell setup prefills installed integrations and removes only managed blocks when deselected

- [x] RED: target `Shell setup prefills installed integrations and removes only managed blocks when deselected`; evidence: non-zero; `watn shell` command/step was not implemented.
- [x] GREEN: add `watn shell`, filesystem prefill, provider-independent shell result application, and marker-safe completion removal. Production files: `src/main.rs`, `src/setup.rs`, `src/shell_completion.rs`, `src/shell_shortcut.rs`; test files: `tests/steps/streamlined_setup_steps.rs`. Evidence: targeted PTY run passed: `1 scenario (1 passed)`, `7 steps (7 passed)`.
- [x] REFACTOR: rerun the named scenario after `cargo fmt --all`. Evidence: targeted run passed with `7 steps (7 passed)`.
- [x] COMMIT: commit title references `Shell setup prefills installed integrations and removes only managed blocks when deselected`. Hash: `bd53ed3`

### Shell setup refuses malformed managed markers

- [x] RED: target `Shell setup refuses malformed managed markers`; evidence: non-zero; malformed shell setup step was undefined.
- [x] GREEN: drive duplicated markers through `watn shell`, assert the user-visible error, and verify unchanged target bytes. Production files: existing marker validation/rejection reused; test files: `tests/steps/streamlined_setup_steps.rs`. Evidence: targeted run passed: `1 scenario (1 passed)`, `4 steps (4 passed)`.
- [x] REFACTOR: rerun the named scenario after `cargo fmt --all`. Evidence: targeted run passed with `4 steps (4 passed)`.
- [x] COMMIT: commit title references `Shell setup refuses malformed managed markers`. Hash: `00ddeff`

### Shell failure does not discard successful shell changes or configuration

- [x] RED: target `Shell failure does not discard successful shell changes or configuration`; evidence: non-zero; coordinated shell failure fixture steps were undefined.
- [x] GREEN: exercise independent shell application with a writable Bash target and directory Zsh target, asserting retained Bash/config and nonzero aggregate result. Production files: existing independent shell result path reused; test files: `tests/steps/streamlined_setup_steps.rs`. Evidence: targeted run passed: `1 scenario (1 passed)`, `6 steps (6 passed)`.
- [x] REFACTOR: rerun the named scenario after `cargo fmt --all`. Evidence: targeted run passed with `6 steps (6 passed)` and no unused-variable warning.
- [x] COMMIT: commit title references `Shell failure does not discard successful shell changes or configuration`. Hash: `b618dc4`

### Non-interactive incomplete request prints setup guidance without probing

- [x] RED: target `Non-interactive incomplete request prints setup guidance without probing`; evidence: non-zero; the nonzero-status step was undefined.
- [x] GREEN: drive the real non-TTY request, add a zero-hit catalog sentinel, and assert actionable setup guidance with no catalog/chat requests. Production files: existing readiness/guidance path reused; test files: `tests/steps/streamlined_setup_steps.rs`; Evidence: targeted run passed: `1 scenario (1 passed)`, `7 steps (7 passed)`.
- [x] REFACTOR: rerun the named scenario after `cargo fmt --all`. Evidence: targeted run passed with `7 steps (7 passed)`.
- [x] COMMIT: commit title references `Non-interactive incomplete request prints setup guidance without probing`. Hash: `424a1c6`

### Malformed configuration is reported without modification

- [x] RED: target `Malformed configuration is reported without modification`; evidence: non-zero; malformed-config fixture and unique unchanged-file assertion were undefined.
- [x] GREEN: load config before non-TTY setup guidance, report parse error, and assert malformed bytes remain unchanged. Production files: `src/main.rs`; test files: `tests/steps/streamlined_setup_steps.rs`; Evidence: targeted run passed: `1 scenario (1 passed)`, `5 steps (5 passed)`.
- [x] REFACTOR: rerun the named scenario after `cargo fmt --all`. Evidence: targeted run passed with `5 steps (5 passed)`.
- [x] COMMIT: commit title references `Malformed configuration is reported without modification`. Hash: `a179247`

### Cancelling after provider and credential validation does not create a config file

- [x] RED: target `Cancelling after provider and credential validation does not create a config file`; evidence: non-zero; coordinated provider/credential cancellation steps were undefined.
- [x] GREEN: remove implicit template creation, remove the pre-catalog provider write, and assert no file/provider/catalog request after PTY cancellation. Production files: `src/config/mod.rs`, `src/setup.rs`; test files: `tests/steps/streamlined_setup_steps.rs`; Evidence: targeted run passed: `1 scenario (1 passed)`, `7 steps (7 passed)`.
- [x] REFACTOR: rerun the named scenario after `cargo fmt --all`; dead template writer removed. Evidence: targeted run passed with `7 steps (7 passed)` and no warnings.
- [x] COMMIT: commit title references `Cancelling after provider and credential validation does not create a config file`. Hash: `9605824`

### Cancelling after a successful catalog probe leaves the baseline unchanged

- [x] RED: target `Cancelling after a successful catalog probe leaves the baseline unchanged`; evidence: non-zero; catalog-probe cancellation steps were undefined.
- [x] GREEN: drive provider/credential/catalog setup through PTY and cancel after a successful provider-local probe; assert byte-for-byte config and shell-target preservation. Production files: existing draft/cancel path reused; test files: `tests/steps/streamlined_setup_steps.rs`; Evidence: targeted run passed: `1 scenario (1 passed)`, `6 steps (6 passed)`.
- [x] REFACTOR: rerun the named scenario after `cargo fmt --all`. Evidence: targeted run passed with `6 steps (6 passed)`.
- [x] COMMIT: commit title references `Cancelling after a successful catalog probe leaves the baseline unchanged`. Hash: `dafd953`

### Catalog failure does not persist an unconfirmed provider

- [x] RED: target `Catalog failure does not persist an unconfirmed provider`; evidence: non-zero; catalog-failure and cancellation steps were undefined.
- [x] GREEN: drive a failing provider-local catalog probe from a no-config coordinator and assert no provider/config/catalog state is persisted after cancellation. Production files: existing draft/manual fallback path reused; test files: `tests/steps/streamlined_setup_steps.rs`; Evidence: targeted run passed: `1 scenario (1 passed)`, `7 steps (7 passed)`.
- [x] REFACTOR: rerun the named scenario after `cargo fmt --all`. Evidence: targeted run passed with `7 steps (7 passed)`.
- [x] COMMIT: commit title references `Catalog failure does not persist an unconfirmed provider`. Hash: `450c683`

### A successful edited catalog endpoint is promoted only at final confirmation

- [x] RED: target `A successful edited catalog endpoint is promoted only at final confirmation`; evidence: non-zero; provider-local catalog endpoint fixture was undefined.
- [x] GREEN: add backward-compatible persisted `catalog_endpoint` state and assert edited endpoint remains draft-only until confirmation. Production files: `src/config/types.rs`, `src/config/mod.rs`, `src/setup.rs`; test files: `tests/steps/streamlined_setup_steps.rs`, `tests/steps/transport_steps.rs`; Evidence: targeted run passed: `1 scenario (1 passed)`, `6 steps (6 passed)`.
- [x] REFACTOR: rerun the named scenario after `cargo fmt --all`. Evidence: targeted run passed with `6 steps (6 passed)` and no step warnings.
- [x] COMMIT: commit title references `A successful edited catalog endpoint is promoted only at final confirmation`. Hash: `a8596e9`

### A failed edited catalog endpoint preserves the previous endpoint

- [x] RED: target `A failed edited catalog endpoint preserves the previous endpoint`; evidence: non-zero; reachable/edited catalog fixture steps were undefined.
- [x] GREEN: assert failed edit keeps the persisted provider-local catalog endpoint and exposes manual fallback without a pre-confirmation write. Production files: persisted catalog model reused; test files: `tests/steps/streamlined_setup_steps.rs`. Evidence: targeted run passed: `1 scenario (1 passed)`, `6 steps (6 passed)`.
- [x] REFACTOR: rerun the named scenario after `cargo fmt --all`. Evidence: targeted run passed with `6 steps (6 passed)`.
- [x] COMMIT: commit title references `A failed edited catalog endpoint preserves the previous endpoint`. Hash: `72c6ac9`

### A failed new catalog endpoint remains unset

- [x] RED: target `A failed new catalog endpoint remains unset`; evidence: non-zero; no-saved-catalog fixture steps were undefined.
- [x] GREEN: assert a provider without saved catalog state keeps the field unset after an unreachable derived endpoint and remains eligible for manual entry. Production files: persisted catalog model reused; test files: `tests/steps/streamlined_setup_steps.rs`; Evidence: targeted run passed: `1 scenario (1 passed)`, `6 steps (6 passed)`.
- [x] REFACTOR: rerun the named scenario after `cargo fmt --all`. Evidence: targeted run passed with `6 steps (6 passed)`.
- [x] COMMIT: commit title references `A failed new catalog endpoint remains unset`. Hash: `3a500ca`

### Invalid catalog data switches to manual model selection

- [x] RED: target `Invalid catalog data switches to manual model selection`; evidence: non-zero; empty-catalog fixture and terminal start step were undefined.
- [x] GREEN: drive an empty provider catalog through the real model PTY and assert unusable discovery, no invented models, and manual entry. Production files: existing manual fallback path reused; test files: `tests/steps/streamlined_setup_steps.rs`; Evidence: targeted run passed: `1 scenario (1 passed)`, `5 steps (5 passed)`.
- [x] REFACTOR: rerun the named scenario after `cargo fmt --all`. Evidence: targeted run passed with `5 steps (5 passed)`.
- [x] COMMIT: commit title references `Invalid catalog data switches to manual model selection`. Hash: `942ac46`

### Catalog entries without unique non-empty identifiers are rejected

- [x] RED: target `Catalog entries without unique non-empty identifiers are rejected`; evidence: non-zero; invalid-identifier fixture and assertions were undefined.
- [x] GREEN: reject empty/duplicate provider model identifiers and expose manual selection. Production files: `src/setup.rs`; test files: `tests/steps/streamlined_setup_steps.rs`; Evidence: targeted run passed: `1 scenario (1 passed)`, `5 steps (5 passed)`.
- [x] REFACTOR: rerun the named scenario after `cargo fmt --all`. Evidence: targeted run passed with `5 steps (5 passed)`.
- [x] COMMIT: commit title references `Catalog entries without unique non-empty identifiers are rejected`. Hash: `9e36b7d`

### Provider catalog takes precedence over a conflicting legacy LiteLLM source

- [ ] RED: target `Provider catalog takes precedence over a conflicting legacy LiteLLM source`; evidence:
- [ ] GREEN: route all discovery through provider-local source and preserve legacy config. Production files: . Evidence:
- [ ] REFACTOR: rerun the named scenario. Evidence:
- [ ] COMMIT: commit title references `Provider catalog takes precedence over a conflicting legacy LiteLLM source`. Hash: 

### Provider catalog pagination and search use the provider source

- [ ] RED: target `Provider catalog pagination and search use the provider source`; evidence:
- [ ] GREEN: route page/search requests and authorization through the provider source. Production files: . Evidence:
- [ ] REFACTOR: rerun the named scenario. Evidence:
- [ ] COMMIT: commit title references `Provider catalog pagination and search use the provider source`. Hash: 

### Manual model identifiers are persisted exactly after catalog failure

- [ ] RED: target `Manual model identifiers are persisted exactly after catalog failure`; evidence:
- [ ] GREEN: persist manual identifiers verbatim after focused model confirmation. Production files: . Evidence:
- [ ] REFACTOR: rerun the named scenario. Evidence:
- [ ] COMMIT: commit title references `Manual model identifiers are persisted exactly after catalog failure`. Hash: 

### Changing provider invalidates catalog-backed model choices

- [ ] RED: target `Changing provider invalidates catalog-backed model choices`; evidence:
- [ ] GREEN: mark catalog/model state stale and revalidate after provider change. Production files: . Evidence:
- [ ] REFACTOR: rerun the named scenario. Evidence:
- [ ] COMMIT: commit title references `Changing provider invalidates catalog-backed model choices`. Hash: 

### The final review shows all draft domains without exposing a secret

- [ ] RED: target `The final review shows all draft domains without exposing a secret`; evidence:
- [ ] GREEN: render compact review with masked credential status and all draft domains. Production files: . Evidence:
- [ ] REFACTOR: rerun the named scenario. Evidence:
- [ ] COMMIT: commit title references `The final review shows all draft domains without exposing a secret`. Hash: 

### Final confirmation is blocked while a required draft value is invalid

- [ ] RED: target `Final confirmation is blocked while a required draft value is invalid`; evidence:
- [ ] GREEN: block review confirmation and identify invalid/missing value. Production files: . Evidence:
- [ ] REFACTOR: rerun the named scenario. Evidence:
- [ ] COMMIT: commit title references `Final confirmation is blocked while a required draft value is invalid`. Hash: 

### Back navigation preserves draft values across model and reasoning questions

- [ ] RED: target `Back navigation preserves draft values across model and reasoning questions`; evidence:
- [ ] GREEN: preserve draft state across backward/forward navigation. Production files: . Evidence:
- [ ] REFACTOR: rerun the named scenario. Evidence:
- [ ] COMMIT: commit title references `Back navigation preserves draft values across model and reasoning questions`. Hash: 

### Selected provider migration moves an arbitrary provider to custom

- [ ] RED: target `Selected provider migration moves an arbitrary provider to custom`; evidence:
- [ ] GREEN: implement selected arbitrary-key migration and source removal. Production files: . Evidence:
- [ ] REFACTOR: rerun the named scenario. Evidence:
- [ ] COMMIT: commit title references `Selected provider migration moves an arbitrary provider to custom`. Hash: 

### Provider migration preserves the destination default model on collision

- [ ] RED: target `Provider migration preserves the destination default model on collision`; evidence:
- [ ] GREEN: implement deterministic custom collision/default-model precedence. Production files: . Evidence:
- [ ] REFACTOR: rerun the named scenario. Evidence:
- [ ] COMMIT: commit title references `Provider migration preserves the destination default model on collision`. Hash: 

### Provider migration is idempotent after the first conversion

- [ ] RED: target `Provider migration is idempotent after the first conversion`; evidence:
- [ ] GREEN: make canonical reruns stable with one custom entry. Production files: . Evidence:
- [ ] REFACTOR: rerun the named scenario. Evidence:
- [ ] COMMIT: commit title references `Provider migration is idempotent after the first conversion`. Hash: 

### Free-form reasoning survives persistence and request construction

- [ ] RED: target `Free-form reasoning survives persistence and request construction`; evidence:
- [ ] GREEN: persist and send non-empty reasoning strings unchanged. Production files: . Evidence:
- [ ] REFACTOR: rerun the named scenario. Evidence:
- [ ] COMMIT: commit title references `Free-form reasoning survives persistence and request construction`. Hash: 

### Existing unknown reasoning remains active after rerunning setup

- [ ] RED: target `Existing unknown reasoning remains active after rerunning setup`; evidence:
- [ ] GREEN: preserve unknown persisted reasoning values through reload and request construction. Production files: . Evidence:
- [ ] REFACTOR: rerun the named scenario. Evidence:
- [ ] COMMIT: commit title references `Existing unknown reasoning remains active after rerunning setup`. Hash: 

### Whitespace-only custom reasoning is rejected

- [ ] RED: target `Whitespace-only custom reasoning is rejected`; evidence:
- [ ] GREEN: reject blank reasoning and preserve baseline. Production files: . Evidence:
- [ ] REFACTOR: rerun the named scenario. Evidence:
- [ ] COMMIT: commit title references `Whitespace-only custom reasoning is rejected`. Hash: 

### Catalog reasoning choices still permit a custom non-empty value

- [ ] RED: target `Catalog reasoning choices still permit a custom non-empty value`; evidence:
- [ ] GREEN: combine catalog suggestions with free-form reasoning entry. Production files: . Evidence:
- [ ] REFACTOR: rerun the named scenario. Evidence:
- [ ] COMMIT: commit title references `Catalog reasoning choices still permit a custom non-empty value`. Hash: 

### Declining shell setup performs no target inspection or write

- [ ] RED: target `Declining shell setup performs no target inspection or write`; evidence:
- [ ] GREEN: make shell decline side-effect free. Production files: . Evidence:
- [ ] REFACTOR: rerun the named scenario. Evidence:
- [ ] COMMIT: commit title references `Declining shell setup performs no target inspection or write`. Hash: 

### Shell removal preserves bytes outside the managed block

- [ ] RED: target `Shell removal preserves bytes outside the managed block`; evidence:
- [ ] GREEN: remove only the valid managed block and preserve surrounding bytes. Production files: . Evidence:
- [ ] REFACTOR: rerun the named scenario. Evidence:
- [ ] COMMIT: commit title references `Shell removal preserves bytes outside the managed block`. Hash: 

### Missing model roles trigger implicit setup even with a usable provider

- [ ] RED: target `Missing model roles trigger implicit setup even with a usable provider`; evidence:
- [ ] GREEN: include role completeness in implicit setup readiness. Production files: . Evidence:
- [ ] REFACTOR: rerun the named scenario. Evidence:
- [ ] COMMIT: commit title references `Missing model roles trigger implicit setup even with a usable provider`. Hash: 

### Focused model setup preserves provider-owned and unrelated fields

- [ ] RED: target `Focused model setup preserves provider-owned and unrelated fields`; evidence:
- [ ] GREEN: restrict model command persistence to roles/reasoning and successful catalog state. Production files: . Evidence:
- [ ] REFACTOR: rerun the named scenario. Evidence:
- [ ] COMMIT: commit title references `Focused model setup preserves provider-owned and unrelated fields`. Hash: 

### A failed final config write prevents shell operations

- [ ] RED: target `A failed final config write prevents shell operations`; evidence:
- [ ] GREEN: implement atomic config failure boundary before shell application. Production files: . Evidence:
- [ ] REFACTOR: rerun the named scenario. Evidence:
- [ ] COMMIT: commit title references `A failed final config write prevents shell operations`. Hash: 

## E2E Setup

- [ ] Confirm `./run-tests.sh` and `./run-tests.sh --e2e` run with isolated loopback httpmock twins and portable PTYs; record full-suite and E2E scenario counts, with E2E strictly smaller. Evidence:
- [ ] Register and prove `tests/steps/streamlined_setup_e2e_steps.rs` against the real CLI terminal. Evidence:

## E2E Scenarios

### Coordinated setup completes provider models reasoning and shell choices

- [ ] RED: remove only this scenario's `@wip`, add E2E stubs, and run `./run-tests.sh --e2e` targeting `Coordinated setup completes provider models reasoning and shell choices`. Evidence:
- [ ] GREEN: drive the real `watn setup` PTY flow and assert visible questions/review/result, with config/mock checks secondary. Production files: . Evidence:
- [ ] REFACTOR: rerun the E2E scenario. Evidence:
- [ ] COMMIT: commit title references `Coordinated setup completes provider models reasoning and shell choices`. Hash: 

### Provider setup configures an OpenAI provider with an environment credential

- [ ] RED: target `Provider setup configures an OpenAI provider with an environment credential`; evidence:
- [ ] GREEN: drive the real `watn provider` PTY flow and assert terminal success plus persisted reference. Production files: . Evidence:
- [ ] REFACTOR: rerun the E2E scenario. Evidence:
- [ ] COMMIT: commit title references `Provider setup configures an OpenAI provider with an environment credential`. Hash: 

### Models setup configures all three roles from an available catalog

- [ ] RED: target `Models setup configures all three roles from an available catalog`; evidence:
- [ ] GREEN: drive the real `watn models` PTY flow and assert visible role progression/result plus config. Production files: . Evidence:
- [ ] REFACTOR: rerun the E2E scenario. Evidence:
- [ ] COMMIT: commit title references `Models setup configures all three roles from an available catalog`. Hash: 

### Shell setup independently configures completion and Ctrl-W integrations

- [ ] RED: target `Shell setup independently configures completion and Ctrl-W integrations`; evidence:
- [ ] GREEN: drive the real `watn shell` PTY flow and assert visible independent choices/result plus target files. Production files: . Evidence:
- [ ] REFACTOR: rerun the E2E scenario. Evidence:
- [ ] COMMIT: commit title references `Shell setup independently configures completion and Ctrl-W integrations`. Hash: 

### Incomplete interactive request opens setup and does not send the original request

- [ ] RED: target `Incomplete interactive request opens setup and does not send the original request`; evidence:
- [ ] GREEN: drive the real interactive request PTY, assert coordinator output, cancellation, exit, and zero chat requests. Production files: . Evidence:
- [ ] REFACTOR: rerun the E2E scenario. Evidence:
- [ ] COMMIT: commit title references `Incomplete interactive request opens setup and does not send the original request`. Hash: 
