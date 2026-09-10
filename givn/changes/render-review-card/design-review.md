# Design Review: render-review-card

## Review Scope

The review covered `proposal.md`, the change feature, `design.md`, the
permanent `use-shell` capability, and the affected Arc42 chapters.

## Findings And Dispositions

### Correctness

**Finding:** The card must not hide the command or the model-written content
behind decoration.

**Disposition:** Resolved. Command text wraps to at most two rows and
ellipsizes; stage text and purpose wrap; colors only restyle the same text.

**Finding:** Color and framing must be optional.

**Disposition:** Resolved. `[review] enhanced` defaults on; per-invocation
overrides exist; a color-incapable terminal, a disabled card, or an enhanced
open failure all fall back to the plain panel.

### Testability

**Finding:** Card visuals need observable assertions, and existing plain-panel
assertions must keep working.

**Disposition:** Resolved. The deterministic writer captures SGR sequences;
assertions strip ANSI and check text, and the plain panel remains available for
fallback scenarios. Unit tests cover coloring, capability detection, and
configuration resolution.

### Keyboard

**Finding:** Stage navigation and the `e` shortcut must not break the reviewed
keyboard contract.

**Disposition:** Resolved. Left/Right and Up/Down navigate within the focused
`Flow` region; Tab/Shift-Tab/Enter/Escape behavior is unchanged; `e` is an
additional review-mode shortcut, specified and tested.

### ADR Qualification

`NOT_QUALIFIED`. The card is a renderer behind the existing presentation-adapter
seam; ADR-0015 still owns the stream completion boundary.

### Arc42 Independent Walk

| # | Independent result | Disposition |
|---:|---|---|
| 5 | Yes | A presentation building block is added. |
| 6 | Yes | The review flow presents through the card with fallback. |
| 8 | Yes | Color capability and fallback rules are cross-cutting. |
| 10 | Yes | QS-067 gains the readable-hierarchy requirement. |
| 11 | Yes | R-068/R-071 mitigations gain card fallback. |
| 1-4, 7, 9, 12 | No | No other chapter changes. |

## Hardening Receipt

- `proposal.md`: states the outcome and the optional/disableable requirement.
- `specs/use-shell/interactive-shell-shortcut.feature`: six `@givn.added`
  scenarios.
- `design.md`: layout, color roles, renderer selection, keyboard additions,
  failure outcomes, ADR and Arc42 routing.
- `arc42.md` and affected durable chapters: updated.
- `tasks.md`: not written or modified.

## Status

DESIGN-REVIEW: PASS
