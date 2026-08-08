# Design Review: implement-empty-step-assertions

## Grilling

### Scope
The proposal asks for four empty step definitions to be filled in. The base spec
covers three `@e2e` scenarios that exercise three of the four steps. The delta
spec adds a `@givn.added @e2e` scenario for the auth header assertion and a
`@givn.added` scenario for the missing-API-key error path (whose steps are
already implemented).

Finding: the delta spec's "Provider API key from environment variable" scenario
duplicates the identically-named scenario already in the base spec
(`givn/specs/providers/providers.feature:28`). A duplicate scenario name
causes duplicate-registration panics in Cucumber-rs. The delta scenario serves
only to document that the empty step is being addressed — it should be removed
before implementation (the base spec already covers it).

### Tech choices
httpmock 0.7 is already a dev-dependency and used throughout the harness.
`Mock::new(id, server).hits()` is confirmed working (line 494 of ask_steps.rs).
The header-match approach (adding `.header("Authorization", ...)` to `when`)
is idiomatic httpmock and matches existing usage patterns.

### Missing scenarios
None identified. The four empty steps correspond to the four assertion points
in the `@e2e` scenarios.

### Testability
Every scenario can fail in RED: removing the step body causes
`fail_on_skipped()` to panic. Assertions use `.hits() > 0` which is a concrete
observable value.

Risk for the LiteLLM model-discovery scenario: the test infrastructure does not
currently set up a models mock for that scenario. The `Then` step will need a
models mock to assert against. This is addressed by making `setup_models_mock`
return its mock ID and storing it in `WatnWorld`, then asserting in the step.

### Risk
The auth header approach requires adding an `Authorization` matcher to the chat
completion mock. If applied globally (all scenarios), non-auth scenarios would
break. Mitigation: only pass the auth header when `WATN_OPENAI_API_KEY` is
present in `world.env_vars`.

### Arc42 documentation
The arc42 assessment correctly marks all 12 chapters as "No". This is a pure
test-harness change with no production architecture impact. No separate chapter
files exist under `docs/arc42/` — the assessment is self-contained in
`arc42.md`. Satisfactory.

## Hardening

### Delta spec fix
The `@givn.added @e2e` scenario "Provider API key from environment variable"
in the delta spec duplicates the base spec. It will be removed to avoid
duplicate-registration panics in Cucumber-rs. The base spec already exercises
the `request_has_auth_header` step.

### Implementation approach confirmed
1. Add `models_mock_id: Option<usize>` to `WatnWorld`.
2. Make `setup_models_mock` return the mock ID.
3. In `ensure_test_env`, store chat and models mock IDs.
4. Add optional auth header parameter to `setup_chat_completion_mock`.
5. Compute `Bearer` value from `WATN_OPENAI_API_KEY` env var in `ensure_test_env`.
6. Implement the four Then steps using `Mock::new(id, server).hits()`.

DESIGN-REVIEW: PASS
