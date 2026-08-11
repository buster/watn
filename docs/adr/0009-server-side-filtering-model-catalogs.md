# ADR-0009: Hybrid filtering for complete and paginated model catalogs

- **Status:** accepted
- **Date:** 2026-08-11
- **Decision-makers:** architect

## Context and Problem Statement

The `watn models` SetupWizard model pages present model catalogs through the
shared model-picker search logic. A complete catalog should be cheap to filter
locally, while providers with large paginated catalogs must remain searchable
without fetching every page. The terminal query must remain visible and
responsive while a provider-backed search is in flight.

How should the picker help users find a specific model when the catalog spans
multiple pages?

## Decision Drivers

- Must avoid provider search requests when the complete catalog is already
  loaded locally.
- Must work with providers whose `/models` endpoint is paginated or incomplete.
- Must not require fetching every page of a large catalog.
- Must degrade gracefully for providers that do not support server-side search.
- Interaction must feel responsive and keep the current query visible.
- Search workers must not outlive the SetupWizard.

## Considered Options

- **Client-side filter after full fetch for every catalog** — simple, but
  slow and memory-heavy for large catalogs.
- **Server-side search for every query** — efficient for large catalogs, but
  adds avoidable requests and latency when the complete catalog is already in
  memory.
- **Hybrid local/server filtering** — filter complete catalogs locally and use
  `GET /models?search=<query>` for incomplete catalogs. This preserves local
  responsiveness without requiring every page of a large catalog.

## Decision Outcome

Chosen: **hybrid local/server filtering**. When the loaded response is complete,
the picker applies the existing per-word filter locally and sends no provider
search request. When the catalog is incomplete, the picker keeps the query
visible, waits 200 ms after the latest keystroke, and sends
`GET /models?search=<query>`. Each remote worker carries a generation; stale
workers are rejected before request, before publish, and before apply, and all
retained workers are joined when setup exits. If the provider does not support
search, the client applies its local safety filter to the loaded entries and
shows the existing unsupported-search status.

The SetupWizard model pages use the shared model-picker search logic because a
framework-managed list must support dynamic, server-driven item lists and raw
keystroke reading without a separate prompt loop.

## Consequences

- Good: complete catalogs filter locally with no avoidable network request.
- Good: incomplete catalogs remain searchable without fetching every page.
- Good: generation checks and worker joining prevent stale results and leaked
  search work.
- Bad: incomplete catalogs still depend on the provider implementing
  `?search=` (or at least tolerating it without error).
- Bad: raw terminal input is sensitive to terminal emulator quirks. PTY-based
  E2E tests mitigate this but add a new test dependency (`portable-pty`).

## Confirmation

E2E scenario: mock a complete catalog and a delayed incomplete-catalog search,
type and replace a query in the PTY-driven picker, verify the current query and
suggestions remain visible, and verify the stale-result guard discards a slow
response. Regular scenarios assert the local no-request path, provider-backed
path, newest-result rule, and worker shutdown.
