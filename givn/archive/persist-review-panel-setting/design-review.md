# Design Review: persist-review-panel-setting

## Review Scope

The review covered `proposal.md`, the change feature, `design.md`, the
permanent `use-shell` capability, and the affected Arc42 chapters.

## Findings And Dispositions

### Correctness

**Finding:** Persisting an override must not destroy other settings.

**Disposition:** Resolved. The write loads the current configuration, changes
only `review.panel`, and saves atomically.

**Finding:** A persistence failure must not block the requested behavior.

**Disposition:** Resolved. The current invocation follows the override; the
write failure is reported as a warning.

### Testability

**Finding:** The behavior must be observable through the real binary.

**Disposition:** Resolved. Scenarios drive the real binary with an isolated
configuration home and assert the persisted value.

### ADR Qualification

`NOT_QUALIFIED`. Local to the review-preference configuration.

### Arc42 Independent Walk

| # | Independent result | Disposition |
|---:|---|---|
| 4 | Yes | Last chosen setting survives. |
| 6 | Yes | Override persists in the request path. |
| 8 | Yes | Configuration precedence gains CLI persistence. |
| 11 | Yes | Preference-write failure risk added. |
| 1-3, 5, 7, 9, 10, 12 | No | No other chapter changes. |

## Hardening Receipt

- `proposal.md`: states the outcome without implementation detail.
- feature delta: two `@givn.added` scenarios.
- `design.md`: resolution order, interfaces, failure outcomes, routing.
- `arc42.md` and affected durable chapters: updated.
- `tasks.md`: not written or modified.

## Status

DESIGN-REVIEW: PASS
