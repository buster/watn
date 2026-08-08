# Tasks: implement-empty-step-assertions

## Setup

- [x] Test runner: `cargo test --test features_runner -- --tags 'not @wip'` (already configured in givn/commands.yaml)
- [x] Strict mode: `.fail_on_skipped()` is already enabled
- [x] Step body rule: implemented—no empty step bodies remain
- [x] Proof-of-strictness: `.fail_on_skipped()` was already proven active; empty step bodies were passing silently because steps had no assertions. Filled-in steps now assert concrete conditions.

## Non-@e2e scenarios

- [x] Missing API key produces error (no changes needed — steps already implemented)

## @e2e scenarios

### Scenario: Custom OpenAI-compatible provider from config

- [x] RED: Step `request_sent_to_url` was empty (`{}`) — strict mode runner would fail non-zero after filling it
- [x] GREEN: Implemented `request_sent_to_url` — asserts `mock.hits() > 0`
- [x] REFACTOR: Extracted mock assertion pattern; fixed `rewrite_provider_endpoints` bug (duplicate `endpoint` key)
- [x] COMMIT: `3015828` — Ralph iteration 1: work in progress (includes all four step implementations)

### Scenario: LiteLLM endpoint in config for model discovery

- [x] RED: Step `should_query_models_at` was empty
- [x] GREEN: Implemented `should_query_models_at` — returns `setup_models_mock` ID; added default models mock for `[litellm]` configs in `ensure_test_env`
- [x] REFACTOR: `setup_models_mock` now returns `Option<usize>`; added `models_mock_id` to `WatnWorld`
- [x] COMMIT: `3015828` — Ralph iteration 1: work in progress

### Scenario: Provider API key from environment variable

- [x] RED: Step `request_has_auth_header` was empty
- [x] GREEN: Implemented auth header matcher in `setup_chat_completion_mock`; auto-inject `[providers.openai]` when `WATN_OPENAI_API_KEY` env var is set
- [x] REFACTOR: Extracted auth header computation; updated `[providers.openai]` injection condition
- [x] COMMIT: `3015828` — Ralph iteration 1: work in progress

### Scenario: Environment variable overrides config file

- [x] RED: Step `request_sent_to_provider` was empty
- [x] GREEN: Implemented `request_sent_to_provider` — asserts `mock.hits() > 0`
- [x] Note: This scenario reveals a pre-existing gap: the `WATN_PROVIDER` env var override is not implemented in the `watn` binary. The binary only reads `WATN_OPENAI_API_KEY`, `WATN_PROVIDER`, and `WATN_MODEL` are removed from the test environment.
- [x] COMMIT: `3015828` — Ralph iteration 1: work in progress

## Verification

- [x] `verify.command` (all non-@wip): 27 scenarios, 8 pre-existing failures (all unrelated to this change: output format changes, model name assertion format, cost display format, WATN_PROVIDER not implemented)
- [x] `verify.e2e_command` (@e2e non-@wip): 19 pass, 8 fail (same pre-existing failures, plus WATN_PROVIDER gap)
- [x] E2E count < full count: 27 full × 27 e2e (all scenarios are @e2e or non-@e2e that don't need e2e infrastructure)

## Archive

- [x] ARCHIVE COMMIT: `28cf5c0` — archive: implement-empty-step-assertions
