# Design: render-review-card

## Use-Case Traceability

| Contract | Design decision | Evidence |
|---|---|---|
| Understandable terminal action | The card gives intent, command, selected stage, purpose, position, and actions distinct visual roles | Card scenarios |
| Optional and disableable, enabled by default | `[review] enhanced` default enabled, per-invocation overrides, automatic color-capability fallback to the plain panel | Disabled and color-incapable scenarios |
| Model-written content only | Color and framing never add or replace command, stage, or purpose text | Unsupported-syntax and provider-split scenarios |
| No new interaction | Card is a presentation adapter over the existing review surface | Full suite unchanged; no `@e2e` change |

## Card Layout

The card is an inline, bounded, full-content-width region:

- Header: `watn · review` on the left; `◆ selected/total · tier <t> · provider/model` on the right.
- `Intent` and the command, separated by blank space; the command is syntax
  colored and wrapped to at most two content rows, then ellipsized.
- `Flow`: a compact stage position (`1 · 2 · ▸3 · 4`), with unsupported stages
  amber.
- `Stage selected/total`: the selected stage text, wrapped.
- The model-written purpose under the stage, or the purpose status label when
  no purpose is available.
- `Focus`: the three regions (`Flow`, `Candidates`, `Actions`) with the active
  one highlighted.
- The four actions (`Accept`, `Edit`, `Reject`, `Cancel`) with the selected one
  highlighted.
- A dim key-hint line: `←→ stages · ⇥ region · ⏎ accept · e edit · esc cancel`.

The card height is content-driven and capped at
`min(terminal height - 1, 18)` rows; overflow ellipsizes the purpose. Below 60
columns the card drops the command preview and intent to a compact form while
keeping the selected stage and its purpose readable.

## Color Roles

| Role | SGR |
|---|---|
| Frame | dim gray (256-color 240) |
| Labels | cyan (256-color 81) |
| Intent, hints, purpose | dim |
| Command text | white; flags yellow; quoted strings green; separators dim |
| Unsupported marker | amber (256-color 214) `⚠` |
| Selected action | reverse video |
| Focus tab / position | white; inactive dim |

Color is applied only when the terminal supports it. The plain renderer
(`render_lines`) remains the portable fallback and stays monochrome.

## Renderer Selection

1. Review enabled and eligible (existing resolution).
2. Enhanced card enabled: `--enhanced-review-panel` / `--no-enhanced-review-panel`
   override `[review] enhanced`, default enabled.
3. Terminal color capability: `NO_COLOR` unset, `TERM` is not `dumb`, and
   `TERM`/`COLORTERM` indicates color.
4. If any condition fails, the enhanced adapter is unavailable and the portable
   plain panel opens. If the portable open also fails, the review is
   `Unavailable` and releases nothing.

`PresentationSelection` already models enhanced/portable selection; the card
becomes the enhanced renderer and the existing panel the portable renderer.

## Keyboard

The reviewed three-region contract is unchanged. Additions:

- In the `Flow` region, Left/Right and Up/Down move between stages and the
  purpose follows the selection (horizontal navigation within the focused
  region).
- In review mode, `e` opens the separate command editor; editor Enter/Escape
  behavior is unchanged.

## Failure Outcomes

| Condition | Outcome |
|---|---|
| Card enabled, color-capable terminal | Card renders the review surface |
| Card disabled by override or configuration | Plain panel opens, candidate available |
| Terminal lacks color capability | Plain panel opens, candidate available |
| Card open or render fails | Plain panel opens with the same candidate and state |
| Plain panel fails too | `Unavailable`; preserve input; release nothing |

## ADR Qualification And Routing

```json
{
  "qualification": "NOT_QUALIFIED",
  "alternatives": "PASS",
  "architectural_impact": "PASS",
  "durable_consequence": "FAIL",
  "lower_level_artifact": "FAIL",
  "existing_adr_check": "PASS",
  "must_be_shared": "SUPPORTING",
  "routing": "CANONICAL_ARTIFACT",
  "canonical_artifact": "givn/changes/render-review-card/design.md",
  "target_adr": null,
  "replacement_adr": null,
  "evidence": {
    "alternatives": [
      "The review surface can stay text-only, or gain an optional enhanced card while retaining the plain fallback."
    ],
    "architectural_impact": [
      "The card is a renderer behind the existing presentation-adapter seam; channels, routing, and the provider contract are untouched."
    ],
    "durable_consequence": [],
    "lower_level_artifact": [],
    "existing_adr_check": [
      "ADR-0015 owns stream completion; this change only chooses how the completed response is presented."
    ]
  }
}
```

## Architecture Impact

Affected Arc42 chapters: 5 (a presentation building block is added), 6 (review
flow presents through the card), 8 (color capability and fallback rules), 10
(QS-067 gains a readable presentation requirement), 11 (R-068/R-071 mitigation
gains the card fallback). No new endpoint or deployment fact.

## Verification Contract

The existing Cucumber runner remains the executable specification. Unit tests
cover token coloring, wrapping, capability detection, and configuration
resolution. `./run-tests.sh` and `./run-tests.sh --e2e` remain the verify
commands.
