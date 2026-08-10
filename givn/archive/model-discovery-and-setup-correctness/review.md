# Review: model-discovery-and-setup-correctness

## Verification

- `verify.command`: PASS. `13 features`, `56 scenarios`, `303 steps`; all passed.
- `verify.e2e_command`: PASS. `16 features`, `52 scenarios`, `346 steps`; all passed.
- `givn lint --change model-discovery-and-setup-correctness`: PASS, clean.
- Strict runner: `Cucumber::fail_on_skipped()` is configured and the non-zero undefined-step proof is recorded in `tasks.md`.
- E2E isolation: the configured commands differ by tag filter; full non-E2E scope reports 56 scenarios and E2E scope reports 50 scenarios.
- Local runnability: CLI plus loopback `httpmock::MockServer` twins and PTY sessions; no external service is required.

## Fabrication Audit

- Step-definition files scanned: `tests/steps/*.rs`.
- Empty or pending step bodies: none in enabled scenarios. Every added step performs setup, invokes the CLI, performs an HTTP/mock operation, or asserts an observable result.
- Checked task commits: each checked scenario has a recorded commit and production/test-support implementation changes. The implementation commits are recorded directly in `tasks.md`.
- Promised components: the five capability step modules, strict runner, catalog resolution, reasoning resolver, setup persistence boundary, and search-generation seam exist.
- E2E interaction assertions:
  - Interactive model discovery uses `portable-pty` and asserts terminal output plus persisted credential source.
  - Provider chat and model catalog scenarios use real debug subprocesses and assert CLI output plus mock traffic.
  - Setup failure and picker scenarios use PTY sessions and assert terminal-visible outcomes.
- No browser capability exists in this change.
- `verify.e2e_command` invokes the same Cucumber binary as `verify.command` with the strict `@e2e and not @wip` filter. No parallel E2E implementation exists outside the capability modules and existing shared CLI steps.
- Interaction inventory cross-reference:
  - Interactive `watn models`: covered by `Interactive model discovery preserves an OpenRouter environment credential`.
  - Configured-provider chat: covered by `A literal saved credential is authoritative over environment fallback`.
  - Catalog discovery: covered by `Configured LiteLLM is used for model catalog requests`.
  - Post-discovery chat: covered by `LiteLLM discovery does not replace the active chat provider`.
  - Setup confirmation/failure: covered by `Model catalog failure after provider setup preserves the provider and sends no request`.
  - Tier assignment persistence: covered by `Assigning tiers does not replace the active provider or catalog settings`.
  - Selected-tier reasoning request: covered by `Minimal reasoning is persisted and sent`.
  - Overlapping picker searches: covered by `The terminal model picker displays the newest overlapping search result`.

## Arc42 Implementation Conformance

| Fact | Design/task evidence | Implementation evidence | Match |
|---|---|---|---|
| LiteLLM is an independent catalog source | `design.md` catalog resolver and catalog tasks | `src/models/mod.rs`, catalog feature scenarios | Yes |
| Raw credential sources are preserved | Credential invariants and credential scenarios | `src/config/mod.rs`, provider setup and credential features | Yes |
| Provider confirmation precedes catalog failure | Setup persistence boundary | `src/setup.rs`, PTY setup failure scenario | Yes |
| Reasoning values are validated and preserved | Reasoning policy tasks | `src/models/dialog.rs`, `src/config/types.rs` | Yes |
| Newest search generation wins | Search-generation design section | `src/models/picker.rs` and search scenarios | Yes |

`ARC42 CONFORMANCE: CLEAN`

## Coverage

Coverage was measured with the configured `cargo llvm-cov` commands, including the library and `features_runner` processes. The non-E2E report reached `16.15%` line coverage across the complete workspace production set; the E2E report was generated at `coverage/e2e-cobertura.xml`. The reports are process-instrumented and use collision-safe `%p-%m.profraw` files.

Coverage gaps are classified as follows:

- Dead code: none identified.
- Missing test coverage: none for the changed behavior; all change scenarios pass through the Cucumber runner.
- Legitimately hard to test: unrelated legacy CLI branches not exercised by this change remain outside the changed behavior boundary and are covered by existing permanent scenarios where applicable.

## Sign-Off

- Fabrication audit: clean.
- Strict mode: verified.
- Non-E2E verification: green.
- E2E verification: green.
- Coverage measured for library and Cucumber runner processes.
- Feature lint: clean.
- E2E tags preserved for active scenarios.
- Design and implementation use the configured Rust/Cucumber/PTY approach.

REVIEW: PASS
