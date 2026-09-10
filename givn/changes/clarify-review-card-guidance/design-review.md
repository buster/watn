# Design Review: clarify-review-card-guidance

## Review Scope

A fresh grilling subagent reviewed the proposal, the delta scenarios, the
design, the chooser and renderer code, the step definitions, the permanent
scenarios those steps back, the E2E waits, the arc42 chapters, and the
interaction inventory.

## Findings And Dispositions

### Step bindings for the added scenarios

**Finding:** the delta named six new steps with no bindings, which would fail
rather than RED under strict mode.

**Disposition:** Resolved. The design now names every binding and its driver:
`begin_catalog_load` + `set_catalog` for arrival, `highlight == Some(0)` plus
the reversed pick for the default selection, session regeneration for the
chosen pick, and rendered-copy assertions for guidance, the active marker, and
the nested-syntax marker.

### Steps and unit tests that the rename breaks

**Finding:** `review_unsupported_marked` asserts rendered `unsupported` and
backs the live "Unsupported command flow remains reviewable" scenario;
`review_chooser_keys_emphasized` pins bold `1-3`; the chooser unit tests pin
`Matches`; the narrow-layout tests pin `unsupported`.

**Disposition:** Resolved. The design enumerates every update: the marker
assertions move to `nested syntax`, the chooser tests move to `Picks`, and the
hint assertion moves to `1/2/3`, `↑↓`, `⏎`, `esc`.

### Loading row label

**Finding:** `Matches` survives in the `catalog_loading` state.

**Disposition:** Resolved. Both the loading and suggestion rows use the `Picks`
label; the design states this explicitly.

### Default highlight precedence

**Finding:** auto-highlighting could override typed text if suggestions arrive
after an edit, or break the typed-model and incomplete-choice scenarios.

**Disposition:** Resolved. `set_catalog` highlights the first pick only when
the query is empty; typing clears the highlight, so typed text stays
authoritative. Empty catalogs leave the highlight unset, preserving
"Enter with no choice is ignored". New unit assertions cover all three cases.

### Arc42 chapter set

**Finding:** chapter 10's target was wrong (the chooser guidance is QS-074, not
QS-067/QS-072), and the glossary change had no target text.

**Disposition:** Resolved. The chapter set is 5 and 10; the glossary claim is
dropped because the internal concept stays `unsupported` and only card copy
changes.

### Active-model marker

**Finding:** the comparison source and the no-marker case were ambiguous.

**Disposition:** Resolved. The marker compares `state.context.model`; a model
outside the tiers gets no marker.

### Narrow layout

**Finding:** the longer hint and the marker can be ellipsized at narrow widths.

**Disposition:** Documented in the failure table; the guidance scenario runs at
the standard 100-column layout, matching the existing narrow-layout scenarios
that assert bounded content rather than full copy.

### E2E fidelity

**Finding:** none; the E2E waits are `⏎`, `Models`, and the replacement label,
none of which the rename touches, and the reject E2E uses the tier key `2`.

**Disposition:** No change.

### Interaction coverage

The inventory and matrix are unchanged: the same nine actions with the same
drivers. The scenario "The first suggestion is ready to choose" is a variant of
the catalog scenarios with the new default-selection invariant; lint subset
notices are dispositioned in the coverage review.

### ADR qualification

`NOT_QUALIFIED` with routing to the change design: local copy, styling, and
default-selection state. No ADR owns card presentation; ADR-0015 owns stream
completion.

### Ubiquitous language

The user-facing marker becomes `nested syntax`; the internal `StageSupport`
concept keeps the name `unsupported`, so the glossary is unchanged.

### Lint

`givn lint --change clarify-review-card-guidance` exits 2 with WIP notices and
one subset notice for the default-selection scenario; the subset is retained as
a variant and dispositioned in review.

## Hardening Receipt

- `proposal.md`: scoped to chooser guidance and the marker rename.
- feature delta: three `@givn.added` scenarios and one `@givn.removed` pair.
- `design.md`: exact copy, active-marker rule, default-highlight precedence,
  step and unit-test migration, chapters 5 and 10.
- `arc42.md`: full 12-row table, `STATUS: DONE`.
- `tasks.md`: not written or modified.

## Status

DESIGN-REVIEW: PASS
