# Design: compact-stage-marker

## Use-Case Traceability

| Contract | Design decision | Evidence |
|---|---|---|
| Review never invents content | The stage text and purpose are untouched; only the marker glyph changes | Marker scenario; existing flow scenarios |
| Review decisions are explicit | The marker is not a decision and stays advisory | Existing card scenarios |
| The card stays small and transient | The marker is one character in the note's position; no legend or extra row | Existing bounds tests |

## Marker Rendering

`src/review/card.rs` replaces the worded note in both render paths with one
amber ellipsis:

- `const UNDECOMPOSED_STAGE_MARKER: &str = "…";` rendered through
  `ink.amber(...)`, appended after the stage text exactly where
  `⚠ nested syntax` sat, and on its own row when the first row has no space.
- The stage and purpose text, the amber color, and the mono fallback are
  unchanged; in mono mode the marker is a plain `…`.

The ellipsis means "the stage is shown as one unit because it contains nested
shell constructs". It is not a warning and not a safety verdict.

## Interfaces And Step Migration

- `src/review/card.rs`: marker constant and both render sites; unit test
  `card_stays_bounded_and_marks_unsupported_stages` asserts the amber sequence
  `\u{1b}[38;5;214m…` instead of the words.
- `src/review/panel.rs`: `narrow_layout_stays_bounded_and_marks_unsupported_stage_text`
  asserts the same amber sequence.
- `tests/steps/interactive_shell_shortcut_steps.rs`:
  - `review_unsupported_marked` (live "Unsupported command flow remains
    reviewable") asserts `flow.has_unsupported()`, the raw amber sequence
    `\u{1b}[38;5;214m…`, and that the card shows no `unsupported`/`nested
    syntax` words.
  - `review_card_marks_nested_syntax` is replaced by
    `the card should mark the stage as not decomposed`, asserting the same raw
    amber sequence and the absence of the old words. The raw sequence is
    required because `assert_review_rendered_contains` strips ANSI and would
    match the unstyled truncation ellipsis.
- Unit tests cover the fallbacks: `plain_card_has_no_escape_sequences` gains an
  unsupported candidate case (plain `…`, no ESC), and a long-stage narrow case
  asserts the amber marker when it cannot share the first row.
- Delta feature: the worded-marker scenario is removed and re-added under the
  new title.

## Test Runner

- Unit/integration: `./run-tests.sh`
- Single scenario: `./run-tests.sh --name 'The card marks a stage it cannot decompose'`
- E2E: `./run-tests.sh --e2e` (unchanged; no new E2E)
- Strict mode: `.fail_on_skipped()`; stubs use `unimplemented!()`
- Step definitions: `tests/steps/interactive_shell_shortcut_steps.rs`. No new
  dependencies.

## E2E Infrastructure

No interaction changes and no card wait labels change: the E2E waits are `⏎`,
`Models`, and the replacement label, none of which the marker touches. The
Interaction Coverage Matrix is unchanged.

## Failure Outcomes

| Condition | Outcome |
|---|---|
| Stage not decomposed | Amber `…` after the stage text (or on its own row when space runs out) |
| Mono terminal | Same `…` without escape sequences; it is indistinguishable from a truncation ellipsis, which is accepted because the marker is advisory and the stage text stays complete |
| Text also truncated | The truncation ellipsis is unstyled, the marker is amber; both can appear |

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
  "canonical_artifact": "givn/changes/compact-stage-marker/design.md",
  "target_adr": null,
  "replacement_adr": null,
  "evidence": {
    "alternatives": [],
    "architectural_impact": [],
    "durable_consequence": [],
    "lower_level_artifact": [
      "The complete decision is one glyph in the card renderer."
    ],
    "existing_adr_check": [
      "No ADR owns card copy; ADR-0015 owns stream completion."
    ]
  }
}
```

## Architecture Impact

Affected Arc42 chapter: 5 (the Review card renderer row names the amber
ellipsis for stages not decomposed instead of the nested-syntax note).
Chapters 1-4 and 6-12 are unchanged: no flow, quality scenario, term, risk, or
decision changes.

## Verification Contract

The existing Cucumber runner remains the executable specification. One
removed+added scenario covers the marker. `./run-tests.sh` and
`./run-tests.sh --e2e` remain the verify commands.
