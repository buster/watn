# Design Review: provider-setup-widget-layout

## Grilling Outcomes

### Scope

The proposal initially used “provider choices,” which could imply a new
provider-selection workflow. The existing state machine only selects a
credential source for the endpoint being configured. The scope is now explicit:
the provider screen is credential-source-only; provider naming and persistence
remain unchanged.

The user chose to include asynchronous model search in this change. This is an
intentional scope expansion from a render-only refactor. The implementation must
use the documented 200 ms debounce, worker request, generation guard, local
fallback, and status behavior.

### Technology Choices

Native Ratatui composition remains the minimal fit. Ratatui and Crossterm are
already dependencies and both flows already own their event loops. The change
adds no runtime dependency, no second persistence path, and no freely
switchable-tab navigation model. The provider list is a `List` of credential
sources, not a new provider registry UI.

### Layout Contract

The user selected a fully stacked layout at the existing 120-column by 40-row
PTY size:

- Provider setup: outer border, credential-source list, details table, guidance
  or validation paragraph.
- Model picker: header, tier tabs, filter paragraph, model table with a
  right-side scrollbar, status/help paragraph.

Long cells may truncate; model IDs remain the first model-table column. The
scrollbar is shown only when content exceeds the table viewport and is hidden
for exact-fit and empty lists.

### Missing Scenarios And Observable Coverage

The two change scenarios remain the exact two E2E inventory interactions. The
provider scenario now also drives invalid endpoint validation and masked pasted
credential display. The model scenario now drives Down + Enter and asserts the
active normal tier plus selected-row visibility. Existing permanent model-picker
and provider specs cover empty results, unsupported search, and newest-result
semantics; the widget implementation must preserve those visible status
contracts.

### Testability

The user selected visible-label assertions rather than coordinate/geometry
assertions. The E2E tests still use a real PTY and real subprocess, but assert
stable titles, list labels, table headings, scrollbar symbol, active-tier text,
validation text, and masking. The test harness will wait for a stable title,
snapshot, send Escape or Ctrl-C, reap the child, and drain the reader. It will
not rely on a live session being dropped implicitly.

### E2E Fidelity And Interaction Matrix

Both inventory entries are CLI interactions driven by `portable-pty`; the
matrix in `design.md` has one non-empty row for each. The existing provider start
step is reused rather than redeclared. The model layout module adds the missing
terminal start step and unique layout/navigation assertions. The final E2E
command must select both scenarios after `@wip` is removed.

### Error, Security, And Boundary Behavior

The widgets must preserve inline endpoint validation, empty-credential
validation, masked literal input, unsupported-search status, and empty-result
status. Empty results have no selectable placeholder row. Table selection is
unset and no scrollbar is rendered when there are no rows. Long values are
truncated by table constraints without replacing the model ID column.

### Primary Risk And Mitigation

The primary implementation risk is a plausible frame that breaks existing PTY
contracts or leaves asynchronous workers racing after the dialog exits. The
mitigation is explicit inner-area/layout rules, stable visible labels,
preservation of the `> model-12` and filter-text contracts, generation checks
before and after debounce, ignored Enter while a search is pending, readiness
polling, and guaranteed child cleanup.

### Arc42 Assessment

Arc42 is enabled. Independent assessment agrees with `arc42.md`: chapters 1, 3,
4, 5, 6, 8, 9, 10, 11, and 12 are affected; chapters 2 and 7 are not. All 12
chapter files exist and contain project-specific content. New ADR-0012 records
widget composition and async search consequences; chapter 11 records narrow
terminal and worker-lifecycle risks. All diagrams remain Mermaid or Markdown
tables; no ASCII-art diagrams were introduced.

## Hardening Applied

- Clarified credential-source-only scope in the proposal, design, and arc42
  context chapters.
- Added the fully stacked 120x40 layout, truncation, wrapping, scrollbar
  visibility, empty-state, active-tier, and selected-row contracts to design.
- Added the worker/debounce/newest-result-wins search design and its quality/risk
  documentation.
- Extended the delta feature with live validation, masking, and navigation
  assertions while keeping exactly one E2E scenario per inventory entry.
- Added ADR-0012 and updated arc42 chapters 1, 3, 4, 5, 6, 8, 9, 10, 11, and 12.
- Ran `givn lint --change provider-setup-widget-layout` during planning; the
  only findings then were the two intentional `@wip` scenarios awaiting
  implementation. Final lint is clean.
- Normalized the Step Definitions artifact to the required markdown-table
  format without changing its reviewed file locations or behavior.

## Status

DESIGN-REVIEW: PASS
