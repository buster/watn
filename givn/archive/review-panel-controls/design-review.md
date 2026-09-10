# Design Review: review-panel-controls

## Review Scope

The review covered `proposal.md`, the change feature (added and removed
scenarios), `design.md`, the permanent `use-shell` capability, and the affected
Arc42 chapters.

## Findings And Dispositions

### Scope

**Finding:** Removing the plain panel must not strand terminals without color.

**Disposition:** Resolved. The card renders monochrome when color is
unavailable; the color capability decides paint only.

**Finding:** Permanent disable needs a safe configuration write.

**Disposition:** Resolved. The existing atomic `save_config` path is reused;
failure leaves the surface open, reports the error, and releases nothing. The
harness uses an isolated path seam.

**Finding:** `?` must not become an execution shortcut.

**Disposition:** Resolved. Explain mode has no acceptance or execution
semantics; closing returns to the confirmation, and execution still requires
the confirmation answer.

### Removed Scenarios

The plain-panel and enhanced/portable-adapter scenarios no longer describe the
system; they are declared `@givn.removed` with placeholders, and their
step bindings are deleted.

### ADR Qualification

`NOT_QUALIFIED`. Local to the review-surface capability; ADR-0015 still owns
the stream completion boundary.

### Arc42 Independent Walk

| # | Independent result | Disposition |
|---:|---|---|
| 4 | Yes | One review presentation. |
| 5 | Yes | Adapter building block removed. |
| 6 | Yes | Explain loop and permanent-disable flow. |
| 8 | Yes | Color as a property; disable persistence. |
| 10 | Yes | QS-067/QS-071 adjusted. |
| 11 | Yes | Fallback risk reduced; persistence failure added. |
| 1-3, 7, 9, 12 | No | No other chapter changes. |

## Hardening Receipt

- `proposal.md`: states the controls without implementation detail.
- feature delta: six `@givn.added` scenarios and four `@givn.removed`
  scenarios.
- `design.md`: single renderer, persistence, explain mode, failure outcomes.
- `arc42.md` and affected durable chapters: updated.
- `tasks.md`: not written or modified.

## Status

DESIGN-REVIEW: PASS
