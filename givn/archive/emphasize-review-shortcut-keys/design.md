# Design: emphasize-review-shortcut-keys

## Use-Case Traceability

| Contract | Design decision | Evidence |
|---|---|---|
| Review decisions are explicit, small, and direct | The decision keys in the card hints are drawn bold and colored; wording and key map are unchanged | Emphasis scenario |
| The card stays small and transient | Emphasis adds no rows or layout change; mono terminals keep plain text | Emphasis scenario; existing mono-card scenario |
| Accept is the default decision | The accept key keeps the green emphasis; other keys use the label color | Existing accept-default assertion and the emphasis scenario |

## Ink Helpers

`src/review/card.rs` gains two `Ink` methods:

- `key(text)` → `\u{1b}[1;38;5;81m{text}\u{1b}[0m` (bold, label color).
- `accept_key(text)` → `\u{1b}[1;38;5;114m{text}\u{1b}[0m` (bold, green).

When color is disabled both return the plain text, so the monochrome card is
byte-identical to the current hint wording.

## Hint Rendering

| Mode | Rendered segments |
|---|---|
| Review | `key("⏎") + " " + accept_key("a") + dim("ccept") · key("e") + dim("dit") · key("r") + dim("eject") · key("c") + dim("ancel") · key("d") + dim("isable") · key("esc")` |
| Command editor | `key("⏎") + dim(" commit") · key("esc") + dim(" discard")` |
| Model chooser | `key("1-3") + dim(" tier") · key("⏎") + dim(" choose") · key("esc") + dim(" close")` |

The review hint no longer repeats the shortcut letter as a separate token: the
acting letter is emphasized inside the word. Explain-only mode keeps its
current dim `esc close`: it offers no decision keys, so it is out of scope.

Deterministic assertion contract (raw SGR, never stripped text):

- Review: `\u{1b}[1;38;5;114ma\u{1b}[0m\u{1b}[2mccept\u{1b}[0m` for accept;
  `\u{1b}[1;38;5;81m{e,r,c,d}\u{1b}[0m\u{1b}[2m{rest}\u{1b}[0m` for the
  other words; `\u{1b}[1;38;5;81m⏎\u{1b}[0m` and
  `\u{1b}[1;38;5;81mesc\u{1b}[0m` for the key tokens.
- Command editor: `\u{1b}[1;38;5;81m` immediately before `⏎` and `esc`.
- Model chooser: `\u{1b}[1;38;5;81m` immediately before `1-3`, `⏎`, and `esc`.

The accept-default step (`review_accept_is_default`) matches the green `a` in
"accept". With the letter split by escape codes, the old E2E `accept ·` wait no
longer sees a contiguous `accept` word; the card wait label becomes the
emphasized `⏎` token, which the hint renders once the card is up (the word
`accept` alone is not a stable PTY label anymore).

## Interfaces

- `src/review/card.rs`: `Ink::key`, `Ink::accept_key`, and the three hint
  builders.
- `tests/steps/interactive_shell_shortcut_steps.rs`: the accept-default
  assertion and the three new emphasis assertions. RED uses raw SGR substring
  checks so the steps fail before the renderer changes (stripped-text helpers
  would pass falsely).
- Existing unit tests that pin exact hint text must move to raw SGR checks:
  `card.rs` `card_frames_content_and_colors_roles` (`esc cancel`) and
  `chooser_rows_render_highlight_and_error` (`1-3 tier`).

## Test Runner

- Unit/integration: `./run-tests.sh`
- Single scenario: `./run-tests.sh --name 'The review card emphasizes every shortcut key'`
- E2E: `./run-tests.sh --e2e` (unchanged; no new E2E)
- Strict mode: `.fail_on_skipped()`; stubs use `unimplemented!()`
- Step definitions: `tests/steps/interactive_shell_shortcut_steps.rs` (one file
  per capability, as before)
- No new dependencies.

## E2E Infrastructure

No interaction is added, changed, or removed. The Interaction Coverage Matrix
from `simplify-review-card-controls` remains the current inventory and stays
valid: the same nine actions map to the same `@e2e` scenarios, and the driver
for each is unchanged. Styling is asserted at the card-renderer layer.

## Failure Outcomes

| Condition | Outcome |
|---|---|
| Color-capable terminal | Keys are bold and colored; labels stay dim |
| Monochrome terminal | Same words, no escape sequences |
| Any review mode | Wording and key behavior are unchanged |

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
  "canonical_artifact": "givn/changes/emphasize-review-shortcut-keys/design.md",
  "target_adr": null,
  "replacement_adr": null,
  "evidence": {
    "alternatives": [],
    "architectural_impact": [],
    "durable_consequence": [],
    "lower_level_artifact": [
      "The complete decision is local ANSI styling in the card renderer."
    ],
    "existing_adr_check": [
      "No ADR owns hint styling; ADR-0015 owns stream completion."
    ]
  }
}
```

## Architecture Impact

Affected Arc42 chapters: 5 (review card renderer row: hint keys emphasized) and
10 (QS-067: decision hints with accept emphasized). Chapters 1-4, 6-9, 11, 12
are unchanged: no new flow, term, risk, or decision.

## Verification Contract

The existing Cucumber runner remains the executable specification. One added
scenario asserts the emphasis in all three hint modes. `./run-tests.sh` and
`./run-tests.sh --e2e` remain the verify commands.
