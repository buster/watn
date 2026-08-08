# ADR-0009: Server-side filtering for paginated model catalogs

- **Status:** proposed
- **Date:** 2026-08-08
- **Decision-makers:** architect

## Context and Problem Statement

The `watn models` picker fetches the complete model list from `GET /models` and
presents it as a single scrollable `dialoguer::Select` list. Providers with
large catalogs (OpenRouter exposes thousands) make this unusable: the list is
too long to scan, and models beyond the first page of a paginated response are
invisible.

How should the picker help users find a specific model when the catalog spans
multiple pages?

## Decision Drivers

- Must work with providers whose `/models` endpoint is paginated.
- Must not require fetching the entire catalog (could be megabytes).
- Must degrade gracefully for providers that do not support server-side search.
- Interaction must feel responsive — results should appear as the user types.

## Considered Options

- **Client-side filter after full fetch** — fetch all pages, concatenate, filter
  locally. Slow first load; memory-heavy for large catalogs.
- **Static list with no filter** — current behavior. Not viable for large
  catalogs.
- **Server-side search with `?search=` parameter** — send the user's query to
  the provider as `GET /models?search=<query>`. The provider returns only
  matching models. Fast, minimal data transfer, works across pages.

## Decision Outcome

Chosen: **Server-side search with `?search=` parameter**, backed by a local
substring safety filter. The picker sends `GET /models?search=<query>` on each
keystroke (debounced in the worker thread, guarded against stale results via
a generation counter). If the provider does not respond with `meta.search`
echoing the query, the client applies a secondary case-insensitive substring
filter on the returned `id` fields. If the endpoint returns 4xx/5xx on a
non-empty search, the picker displays "Model search is not supported by this
provider" and retains whatever suggestions were previously visible.

The autosuggest picker uses raw terminal I/O (`console` crate, already a
transitive dep via `dialoguer`) instead of `dialoguer::Select` because
`Select` does not support dynamic, server-driven item lists or raw keystroke
reading.

## Consequences

- Good: works with paginated catalogs without fetching every page.
- Good: degrades to local substring filter when the provider does not echo
  the search query, so it still narrows results to some degree.
- Bad: depends on the provider implementing `?search=` (or at least tolerating
  it without error). Providers that reject unknown query params get the
  unsupported-search error message.
- Bad: raw terminal input is sensitive to terminal emulator quirks. PTY-based
  E2E tests mitigate this but add a new test dependency (`portable-pty`).

## Confirmation

E2E scenario: mock paginated catalog, type a query in the PTY-driven picker,
verify the correct suggestions appear and the stale-result guard discards a
slow response.