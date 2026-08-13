# ADR-0022: Verbatim reasoning values

- **Status:** accepted
- **Date:** 2026-08-13
- **Decision-makers:** Watn maintainers

## Context and Problem Statement

Providers may expose reasoning modes that are not in Watn's predefined
suggestions. A closed enum discards provider-specific settings and makes a
saved value change silently when it is reused.

## Decision Drivers

- Preserve provider-specific values across setup and requests.
- Keep `off` as an explicit way to omit the request field.
- Reject only values that cannot represent a meaningful effort.

## Considered Options

- **Closed set** - simple UI and validation, but loses provider-specific modes.
- **Non-empty strings with suggestions** - preserves values while keeping the
  common choices easy to select.

## Decision Outcome

Reasoning accepts any non-empty value. Catalog metadata supplies suggested
values and defaults. Whitespace-only custom input is rejected. Every non-`off`
value is persisted and sent unchanged; `off` omits `reasoning_effort`.

## Consequences

### Good

- Provider-specific modes round-trip without normalization.
- Request construction remains compatible with future provider values.

### Bad

- A provider may reject a syntactically valid but semantically unsupported
  value; the provider response remains authoritative.
- UI validation must distinguish blank values from arbitrary non-empty values.

## Confirmation

Feature scenarios assert custom values through TOML reload and exact outbound
request bodies, plus rejection of whitespace-only input and omission of `off`.
