# Design Review: model-discovery-and-setup-correctness

## Scope

The proposal scope is covered by the delta features: credential-source
preservation, catalog/chat separation, provider confirmation persistence,
reasoning policy, and overlapping-search lifecycle. No unrelated behavior was
added. The provider-specific then generic fallback scenario was made explicit.

## Technology Choices

No issue found. Existing blocking HTTP, ratatui/PTY setup, pure reasoning
resolution, generation guards, and cucumber-rs seams fit the observable
scenarios without a new dependency or asynchronous application model.

## Missing Scenarios

Resolved. The artifacts cover missing saved environment references as an
authentication error before a request, explicit confirmation before provider
persistence, provider-specific-before-generic fallback, mandatory reasoning
without usable metadata, and catalog failure/cancellation boundaries.

## Testability

Resolved. Then-steps assert exact credential sources, request routing,
Authorization behavior, persisted tiers, reasoning values, final search IDs,
and worker cleanup. E2E tags remain on the interactive and subprocess scenarios.

## Risk

The highest risk is a false-green concurrency test that serializes workers or
uses completion order instead of user-entry generation. The design now requires
coordinated workers, exact final IDs, and cleanup assertions. The catalog/chat
E2E path also performs discovery before asserting chat routing.

## Arc42

All twelve chapter files exist and contain decision-specific content. The
affected chapter set matches the proposal and design; deployment remains
unaffected. The chapters use Mermaid rather than ASCII-art diagrams. Chapter
09 records the independent catalog and provider-confirmation decision, and
chapter 11 records its consequences. Runtime and cross-cutting documentation
now state explicit credential confirmation, closed reasoning strengths including
`minimal`, and user-entry rather than completion-order search generations.

DESIGN-REVIEW: PASS
