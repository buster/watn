# Design Review: recover-review-stage-purposes

## Review Scope

The review covered `proposal.md`, the change feature, `design.md`, the
permanent `use-shell` capability, and the affected Arc42 chapters.

## Findings And Dispositions

### Correctness

**Finding:** Recovery must not let a plausible-looking payload invent purposes.

**Disposition:** Resolved. Purposes are copied only when the provider's
trimmed `stage_text` values equal the locally derived stage text in order and
every purpose is non-empty. Any mismatch falls back to `purpose-unavailable`.

**Finding:** Normalizing a command must not silently change command text.

**Disposition:** Resolved. Only line breaks and tabs in a provider-written JSON
`command` become spaces; no other characters change. Command-only payloads are
not normalized.

### Testability

**Finding:** All three outcomes need observable scenarios.

**Disposition:** Resolved. Scenarios cover unknown status with matching
purposes, a line-broken command with matching purposes, and mismatched stage
text. Unit tests cover the recovery function directly.

### ADR Qualification

`NOT_QUALIFIED`. The decision is local to the reviewed review-surface
capability and fully owned by this design and the Gherkin scenarios. ADR-0015
continues to own the stream completion boundary.

### Arc42 Independent Walk

| # | Independent result | Disposition |
|---:|---|---|
| 6 | Yes | Recovery order and normalization extend the runtime review flow. |
| 8 | Yes | Stage-text agreement and the no-invented-purpose rule extend to non-canonical responses. |
| 10 | Yes | QS-072 gains unknown-status and multiline cases. |
| 11 | Yes | R-072 mitigation gains vocabulary and formatting drift tolerance. |
| 1-5, 7, 9, 12 | No | No other chapter changes. |

## Hardening Receipt

- `proposal.md`: states the outcome without implementation detail.
- `specs/use-shell/interactive-shell-shortcut.feature`: three `@givn.added`
  scenarios with observable assertions.
- `design.md`: recovery order, normalization rule, interfaces, failure table,
  ADR and Arc42 routing.
- `arc42.md` and affected durable chapters: updated.
- `tasks.md`: not written or modified.

## Status

DESIGN-REVIEW: PASS
