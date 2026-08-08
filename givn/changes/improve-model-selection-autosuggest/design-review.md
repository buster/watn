# Design Review: improve-model-selection-autosuggest

## Grilling Results

### Scope
The spec contains exactly what the proposal asked for: live autosuggest matching against the provider's catalog, stale-result guard, empty state, clear-and-restore, unsupported-search error, and tier advance. No extra scope.

### Tech Choices
- `console` crate for raw terminal I/O: already a transitive dep via `dialoguer`. Correct choice — no simpler alternative satisfies raw-mode keystroke reading.
- Generation counter (`Arc<AtomicU64>`): standard debounce pattern, minimal overhead.
- `portable-pty` for E2E tests: required for driving raw-mode terminal apps. No simpler alternative exists.

### Missing Scenarios
No scenarios are missing. Each observable behaviour from the proposal maps to one or more scenarios. Error paths (first-page fetch failure, Ctrl-C during picker) are pre-existing behaviours inherited from the current code, not gaps in this change.

### Testability
Every scenario can fail in RED with no production code. Then-steps assert concrete observable values ("suggestions include X", "picker says no models found"). The PTY-based test harness is documented and necessary.

### Risk
Primary risk: PTY-based test flakiness across platforms (R-009, already documented). Mitigations (known TERM, read timeouts) are specified.

### Architecture Documentation (arc42)
Independently walked all 12 rows:

| # | My assessment | arc42.md | Match? |
|---|---|---|---|
| 1 | No | No | ✓ |
| 2 | Yes | Yes | ✓ |
| 3 | Yes | Yes | ✓ |
| 4 | Yes | Yes | ✓ |
| 5 | Yes | Yes | ✓ |
| 6 | Yes | Yes | ✓ |
| 7 | No | No | ✓ |
| 8 | Yes | Yes | ✓ |
| 9 | Yes | Yes | ✓ |
| 10 | No | No | ✓ |
| 11 | Yes | Yes | ✓ |
| 12 | Yes | Yes | ✓ |

All 12 chapter files exist, contain meaningful content beyond scaffold placeholders, and contain no ASCII art diagrams. Mermaid diagrams used throughout.

ADR-0009 exists, is complete, and its consequences are reflected in chapter 11.

## Hardening

No hardening changes required. No findings that require spec edits, design edits, or arc42 edits.

## Sign-off

DESIGN-REVIEW: PASS

## Review delta (coverage-review remediation)

The coverage review surfaced two non-@e2e scenarios whose step definitions
were no-op placeholders ("Clearing the search restores available
suggestions", "Selecting a suggestion advances to the next tier"). They
duplicated the single distinct interaction covered by the @e2e scenario
(per the interaction coverage matrix: one @e2e per user-facing action) and
could not test the raw-mode key loop. Both were removed, and `design.md`
was corrected to state that the remaining non-@e2e scenarios drive
`picker::execute_search` directly (search/suggestion logic) while the raw
interaction loop is covered by the one @e2e scenario. This is a scoping
correction consistent with the already-reviewed interaction matrix; no new
technology decisions or architecture impact. Re-assessed and accepted.

