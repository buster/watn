# Design: clarify-review-card-guidance

## Use-Case Traceability

| Contract | Design decision | Evidence |
|---|---|---|
| Review decisions are explicit and direct | The chooser hint line names the tier keys, typing, arrows, Enter, and Escape | Guidance scenario |
| Refinement continues until acceptance | The default highlight makes Enter usable as soon as suggestions exist; typed text still wins after an edit | First-suggestion scenario; existing typed-model scenario |
| Review never invents content | The stage marker is renamed, not removed; the flow splitter is unchanged | Nested-syntax scenario |
| The card stays small and transient | No new rows: only labels, markers, and hint text change | Existing bounds tests |

## Chooser Rendering

`src/review/card.rs` chooser rows become:

| Row | Rendering |
|---|---|
| `Models` | `1 small · <model>` per configured tier, joined by two spaces; the tier digit is `Ink::key` (bold label color); the tier whose model equals the panel context model gets a trailing green `●` |
| `Picks` | Filtered catalog suggestions, first four, highlighted entry reversed; the loading state (`catalog_loading`) also uses the `Picks` label (was labelled `Matches`) |
| `Search` | With an empty query: dim `type a model name…` followed by the caret; otherwise the query with the caret at the cursor (was labelled `Model`) |
| Hint | `1/2/3 switch tier · type to filter · ↑↓ pick · ⏎ use · esc back`, with `1/2/3`, `↑↓`, `⏎`, and `esc` as `Ink::key` tokens and the instruction words dim |

The brackets and the misleading reverse-video on the first tier are removed.
The active marker compares against `state.context.model`; a model outside the
tiers gets no marker. At narrow widths the `Models` row is ellipsized like any
other row, so the marker or the last tiers can be cut; the guidance scenario
runs at the standard 100-column layout.

## Default Highlight

`ReviewPanelState::open_model_chooser` and `set_catalog` set
`highlight = Some(0)` when the catalog is non-empty and the query is empty, so
`↑↓` and Enter act immediately. `set_catalog` sets `highlight = None` when the
developer has already typed a query, preserving the rule that typed text is
used as entered. Typing clears the highlight (existing behavior).

## Stage Marker Rename

Both marker renders in `card.rs` change `⚠ unsupported` to `⚠ nested syntax`
with the same amber styling. The `StageSupport` enum, the flow splitter, and
the purpose-unavailable wording are unchanged.

## Interfaces And Step Migration

- `src/review/card.rs`: chooser rows, hint segments, marker text. Unit tests to
  update: `chooser_rows_render_highlight_and_error` asserts `Picks` and the
  `1/2/3` bold sequence instead of `Matches`; `card_stays_bounded_and_marks_unsupported_stages`
  asserts `nested syntax`.
- `src/review/panel.rs`: default highlight in `open_model_chooser` and
  `set_catalog`; new unit assertions (open with a populated catalog →
  `Some(0)`, `set_catalog` with an empty query → `Some(0)`, `set_catalog`
  after typing → `None`); `narrow_layout_stays_bounded_and_marks_unsupported_stage_text`
  asserts `nested syntax`; `catalog_results_errors_and_highlights_are_applied_safely`
  updates its post-`set_catalog` highlight expectation.
- `tests/steps/interactive_shell_shortcut_steps.rs`:
  - `review_unsupported_marked` asserts the rendered `nested syntax` marker
    (runtime permanent scenario "Unsupported command flow remains reviewable").
  - `review_card_marks_unsupported` is replaced by
    `the card should mark the nested syntax`.
  - `review_model_chooser_field` moves to `Search` and the
    `type a model name…` placeholder.
  - `review_chooser_keys_emphasized` asserts bold `1/2/3`, `↑↓`, `⏎`, `esc`.
  - New bindings: `the model chooser should explain its keys and input`,
    `the current model should be marked`, `the catalog suggestions arrive`
    (`begin_catalog_load` + `set_catalog`), `the first suggestion should be
    highlighted` (`highlight == Some(0)` plus the reversed pick rendered),
    `I choose the highlighted suggestion` (Enter → `RegenerateWith` → session
    regeneration), and `the card should mark the nested syntax`.
- Delta feature: the old marker scenario is removed and re-added under the
  nested-syntax title; the removed title matches the permanent title exactly.

## Test Runner

- Unit/integration: `./run-tests.sh`
- Single scenario: `./run-tests.sh --name '<title>'`
- E2E: `./run-tests.sh --e2e` (unchanged; no new E2E)
- Strict mode: `.fail_on_skipped()`; stubs use `unimplemented!()`
- Step definitions: `tests/steps/interactive_shell_shortcut_steps.rs` and
  `tests/steps/interactive_shell_shortcut_e2e_steps.rs` (one file per
  capability). No new dependencies.

## E2E Infrastructure

No interaction changes: the tier keys, typing, arrows, Enter, and Escape keep
their behavior. The Interaction Coverage Matrix from
`simplify-review-card-controls` remains current, with the same nine actions and
drivers. The E2E reject scenario uses the `2` tier key and does not depend on
the default highlight.

## Failure Outcomes

| Condition | Outcome |
|---|---|
| Catalog empty or unavailable | No highlight; Search field and tiers remain usable |
| Catalog arrives after typing | Query stays authoritative; no highlight is applied |
| Current model not in the catalog or tiers | No `●` marker; nothing else changes |
| Mono terminal | Same words and marker text without escape sequences |

## ADR Qualification And Routing

```json
{
  "qualification": "NOT_QUALIFIED",
  "alternatives": "FAIL",
  "architectural_impact": "FAIL",
  "durable_consequence": "FAIL",
  "lower_level_artifact": "PASS",
  "existing_adr_check": "PASS",
  "must_be_shared": "NO",
  "routing": "CANONICAL_ARTIFACT",
  "canonical_artifact": "givn/changes/clarify-review-card-guidance/design.md",
  "target_adr": null,
  "replacement_adr": null,
  "evidence": {
    "alternatives": [],
    "architectural_impact": [],
    "durable_consequence": [],
    "lower_level_artifact": [
      "The complete decision is local card copy, styling, and default-selection state."
    ],
    "existing_adr_check": [
      "No ADR owns review-card presentation; ADR-0015 owns stream completion."
    ]
  }
}
```

## Architecture Impact

Affected Arc42 chapters: 5 (review card renderer: chooser labels, active-model
marker, marker rename) and 10 (QS-074 gains the chooser guidance and default
selection). Chapter 12 is not affected: the internal `StageSupport` concept
stays `unsupported`; only the card copy says `nested syntax`, so the glossary
keeps its current `Command flow` wording. Chapters 1-4 and 6-9, 11 are
unchanged: the chooser behavior and the flow splitter are untouched.

## Verification Contract

The existing Cucumber runner remains the executable specification. Two added
scenarios cover chooser guidance and default selection; one removed+added pair
covers the marker rename. `./run-tests.sh` and `./run-tests.sh --e2e` remain
the verify commands.
