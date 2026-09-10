# Design Review: compact-stage-marker

## Review Scope

A fresh grilling subagent reviewed the proposal, the removed+added scenario,
the design, the renderer and its two marker paths, the step bindings that
assert the current words, the unit tests, the E2E waits, and the arc42
chapters.

## Findings And Dispositions

### Ambiguity with the truncation ellipsis

**Finding:** the amber `…` marker shares its glyph with the unstyled
truncation ellipsis, and mono mode cannot distinguish them.

**Disposition:** Accepted and documented. The user chose the ellipsis as the
marker; it is advisory, the stage text stays complete, and the failure table
records the mono limitation. Color-capable terminals distinguish the amber
sequence.

### Assertion specificity

**Finding:** a plain `…` needle would pass against a truncation ellipsis
because the render helper strips ANSI; the design did not pin the new step's
assertion.

**Disposition:** Resolved. Both marker steps assert the raw amber sequence
`\u{1b}[38;5;214m…` and the absence of the old words.

### Mono and own-row coverage

**Finding:** the mono path and the own-row fallback were promised but
uncovered.

**Disposition:** Resolved. The design adds a unit case rendering an
unsupported candidate without color (plain `…`, no ESC) and a long-stage
narrow case for the fallback row. If the fallback proves unreachable with a
one-character marker, it is simplified rather than left untested.

### Existing assertions to migrate

**Finding:** the worded marker appears in two step bindings, two unit tests,
the removed scenario, and chapter 5.

**Disposition:** Resolved. The design enumerates each site: the live
`review_unsupported_marked`, the replaced card step, the card and panel unit
tests, the delta pair, and the chapter 5 wording.

### E2E and interaction coverage

**Finding:** none. PTY waits are `⏎`, `Models`, and the replacement label; no
E2E asserts the words; no interaction changes.

**Disposition:** No change.

### ADR, arc42, glossary

Chapter set `{5}` is correct; chapter 5 is the sole mention; the glossary and
chapter 6 use generic wording. `NOT_QUALIFIED` holds: one glyph in the card
renderer.

### Lint

`givn lint --change compact-stage-marker` exits 2 with the expected `@wip`
notice; the WIP tag is removed at RED as usual. Coverage artifacts carry the
old test names until the next measurement, which the verification runs
regenerate.

## Hardening Receipt

- `proposal.md`: scoped to the glyph replacement.
- feature delta: one `@givn.removed` and one `@givn.added` scenario.
- `design.md`: raw-sequence assertions, mono/own-row unit coverage, marker
  constant, failure table.
- `arc42.md`: full 12-row table, `STATUS: DONE`.
- `tasks.md`: not written or modified.

## Status

DESIGN-REVIEW: PASS
