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

- [ ] RED: target `Cancelling coordinated setup leaves an existing configuration unchanged`; evidence:
- [ ] GREEN: implement baseline snapshot cancellation. Production files: . Evidence:
- [ ] REFACTOR: rerun the named scenario. Evidence:
- [ ] COMMIT: commit title references `Cancelling coordinated setup leaves an existing configuration unchanged`. Hash: 

### Provider setup requires a custom endpoint

- [ ] RED: target `Provider setup requires a custom endpoint`; evidence:
- [ ] GREEN: implement explicit provider list and required Custom endpoint. Production files: . Evidence:
- [ ] REFACTOR: rerun the named scenario. Evidence:
- [ ] COMMIT: commit title references `Provider setup requires a custom endpoint`. Hash: 

### Provider setup refuses an unresolved environment credential

- [ ] RED: target `Provider setup refuses an unresolved environment credential`; evidence:
- [ ] GREEN: validate environment references before leaving/saving credential setup. Production files: . Evidence:
- [ ] REFACTOR: rerun the named scenario. Evidence:
- [ ] COMMIT: commit title references `Provider setup refuses an unresolved environment credential`. Hash: 

### Provider setup preserves unrelated settings

- [ ] RED: target `Provider setup preserves unrelated settings`; evidence:
- [ ] GREEN: persist provider-owned fields while preserving unrelated providers, pricing, and legacy data. Production files: . Evidence:
- [ ] REFACTOR: rerun the named scenario. Evidence:
- [ ] COMMIT: commit title references `Provider setup preserves unrelated settings`. Hash: 

### Provider setup does not probe the catalog

- [ ] RED: target `Provider setup does not probe the catalog`; evidence:
- [ ] GREEN: keep provider command free of catalog requests. Production files: . Evidence:
- [ ] REFACTOR: rerun the named scenario. Evidence:
- [ ] COMMIT: commit title references `Provider setup does not probe the catalog`. Hash: 

### Models setup gives guidance when no provider is configured

- [ ] RED: target `Models setup gives guidance when no provider is configured`; evidence:
- [ ] GREEN: make `watn models` provide focused guidance without opening provider UI. Production files: . Evidence:
- [ ] REFACTOR: rerun the named scenario. Evidence:
- [ ] COMMIT: commit title references `Models setup gives guidance when no provider is configured`. Hash: 

### Available catalog restricts model choices

- [ ] RED: target `Available catalog restricts model choices`; evidence:
- [ ] GREEN: restrict catalog mode to returned identifiers and require replacement of stale saved models. Production files: . Evidence:
- [ ] REFACTOR: rerun the named scenario. Evidence:
- [ ] COMMIT: commit title references `Available catalog restricts model choices`. Hash: 

### Unavailable catalog allows manual model identifiers

- [ ] RED: target `Unavailable catalog allows manual model identifiers`; evidence:
- [ ] GREEN: implement visible catalog failure and manual model mode. Production files: . Evidence:
- [ ] REFACTOR: rerun the named scenario. Evidence:
- [ ] COMMIT: commit title references `Unavailable catalog allows manual model identifiers`. Hash: 

### Catalog metadata selects supported reasoning efforts for the chosen model

- [ ] RED: target `Catalog metadata selects supported reasoning efforts for the chosen model`; evidence:
- [ ] GREEN: implement metadata suggestions and catalog default selection. Production files: . Evidence:
- [ ] REFACTOR: rerun the named scenario. Evidence:
- [ ] COMMIT: commit title references `Catalog metadata selects supported reasoning efforts for the chosen model`. Hash: 

### Missing reasoning metadata provides generic efforts and free-form input

- [ ] RED: target `Missing reasoning metadata provides generic efforts and free-form input`; evidence:
- [ ] GREEN: implement generic efforts, custom entry, and non-empty validation. Production files: . Evidence:
- [ ] REFACTOR: rerun the named scenario. Evidence:
- [ ] COMMIT: commit title references `Missing reasoning metadata provides generic efforts and free-form input`. Hash: 

### Off reasoning omits the reasoning setting from a request

- [ ] RED: target `Off reasoning omits the reasoning setting from a request`; evidence:
- [ ] GREEN: omit `reasoning_effort` for `off`. Production files: . Evidence:
- [ ] REFACTOR: rerun the named scenario. Evidence:
- [ ] COMMIT: commit title references `Off reasoning omits the reasoning setting from a request`. Hash: 

### Shell setup prefills installed integrations and removes only managed blocks when deselected

- [ ] RED: target `Shell setup prefills installed integrations and removes only managed blocks when deselected`; evidence:
- [ ] GREEN: implement filesystem prefill and safe managed-block removal. Production files: . Evidence:
- [ ] REFACTOR: rerun the named scenario. Evidence:
- [ ] COMMIT: commit title references `Shell setup prefills installed integrations and removes only managed blocks when deselected`. Hash: 

### Shell setup refuses malformed managed markers

- [ ] RED: target `Shell setup refuses malformed managed markers`; evidence:
- [ ] GREEN: reject malformed marker layouts before any write. Production files: . Evidence:
- [ ] REFACTOR: rerun the named scenario. Evidence:
- [ ] COMMIT: commit title references `Shell setup refuses malformed managed markers`. Hash: 

### Shell failure does not discard successful shell changes or configuration

- [ ] RED: target `Shell failure does not discard successful shell changes or configuration`; evidence:
- [ ] GREEN: retain successful independent shell results and config after later failure. Production files: . Evidence:
- [ ] REFACTOR: rerun the named scenario. Evidence:
- [ ] COMMIT: commit title references `Shell failure does not discard successful shell changes or configuration`. Hash: 

### Non-interactive incomplete request prints setup guidance without probing

- [ ] RED: target `Non-interactive incomplete request prints setup guidance without probing`; evidence:
- [ ] GREEN: implement non-TTY guidance and zero-network readiness path. Production files: . Evidence:
- [ ] REFACTOR: rerun the named scenario. Evidence:
- [ ] COMMIT: commit title references `Non-interactive incomplete request prints setup guidance without probing`. Hash: 

### Malformed configuration is reported without modification

- [ ] RED: target `Malformed configuration is reported without modification`; evidence:
- [ ] GREEN: distinguish malformed/unreadable config and refuse writes. Production files: . Evidence:
- [ ] REFACTOR: rerun the named scenario. Evidence:
- [ ] COMMIT: commit title references `Malformed configuration is reported without modification`. Hash: 

### Cancelling after provider and credential validation does not create a config file

- [ ] RED: target `Cancelling after provider and credential validation does not create a config file`; evidence:
- [ ] GREEN: preserve absent-file state through post-credential cancellation. Production files: . Evidence:
- [ ] REFACTOR: rerun the named scenario. Evidence:
- [ ] COMMIT: commit title references `Cancelling after provider and credential validation does not create a config file`. Hash: 

### Cancelling after a successful catalog probe leaves the baseline unchanged

- [ ] RED: target `Cancelling after a successful catalog probe leaves the baseline unchanged`; evidence:
- [ ] GREEN: keep successful probe state in draft only until confirmation. Production files: . Evidence:
- [ ] REFACTOR: rerun the named scenario. Evidence:
- [ ] COMMIT: commit title references `Cancelling after a successful catalog probe leaves the baseline unchanged`. Hash: 

### Catalog failure does not persist an unconfirmed provider

- [ ] RED: target `Catalog failure does not persist an unconfirmed provider`; evidence:
- [ ] GREEN: preserve absent baseline after catalog failure and cancellation. Production files: . Evidence:
- [ ] REFACTOR: rerun the named scenario. Evidence:
- [ ] COMMIT: commit title references `Catalog failure does not persist an unconfirmed provider`. Hash: 

### A successful edited catalog endpoint is promoted only at final confirmation

- [ ] RED: target `A successful edited catalog endpoint is promoted only at final confirmation`; evidence:
- [ ] GREEN: promote catalog endpoint only in the confirmed candidate. Production files: . Evidence:
- [ ] REFACTOR: rerun the named scenario. Evidence:
- [ ] COMMIT: commit title references `A successful edited catalog endpoint is promoted only at final confirmation`. Hash: 

### A failed edited catalog endpoint preserves the previous endpoint

- [ ] RED: target `A failed edited catalog endpoint preserves the previous endpoint`; evidence:
- [ ] GREEN: preserve prior reachable catalog state after failed edit. Production files: . Evidence:
- [ ] REFACTOR: rerun the named scenario. Evidence:
- [ ] COMMIT: commit title references `A failed edited catalog endpoint preserves the previous endpoint`. Hash: 

### A failed new catalog endpoint remains unset

- [ ] RED: target `A failed new catalog endpoint remains unset`; evidence:
- [ ] GREEN: leave new failed catalog state unset and enable manual mode. Production files: . Evidence:
- [ ] REFACTOR: rerun the named scenario. Evidence:
- [ ] COMMIT: commit title references `A failed new catalog endpoint remains unset`. Hash: 

### Invalid catalog data switches to manual model selection

- [ ] RED: target `Invalid catalog data switches to manual model selection`; evidence:
- [ ] GREEN: reject empty/malformed catalog data and expose manual entry. Production files: . Evidence:
- [ ] REFACTOR: rerun the named scenario. Evidence:
- [ ] COMMIT: commit title references `Invalid catalog data switches to manual model selection`. Hash: 

### Catalog entries without unique non-empty identifiers are rejected

- [ ] RED: target `Catalog entries without unique non-empty identifiers are rejected`; evidence:
- [ ] GREEN: validate identifier uniqueness without inventing or deduplicating models. Production files: . Evidence:
- [ ] REFACTOR: rerun the named scenario. Evidence:
- [ ] COMMIT: commit title references `Catalog entries without unique non-empty identifiers are rejected`. Hash: 

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
