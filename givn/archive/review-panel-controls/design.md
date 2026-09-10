# Design: review-panel-controls

## Use-Case Traceability

| Contract | Design decision | Evidence |
|---|---|---|
| One review surface | The card renderer is the only review presentation; color is a property, not an adapter choice | Only-panel and mono-card scenarios |
| Optional and disableable, enabled by default | `[review] panel` with `--review-panel`/`--no-review-panel`; the panel's `d` decision persists `panel = false` | Disabled-path and permanent-disable scenarios |
| No invented content | The card renders the same command, stage, and purpose text; `d` and `?` change control flow only | Existing card assertions |
| Provider-written command remains the subject | The `-x` explanation card is built from the generated command | Confirmation-explanation scenario |

## Single Review Renderer

`render_card_lines` is the review surface. There is no plain panel, no
`PresentationAdapter`, and no `[review] enhanced` setting. Terminal color
capability decides only whether the card paints SGR colors.

- `InlineReviewPanel` always renders the card; it takes a `color` flag.
- `ControllingTerminal` exposes only `render_lines`, used with card lines.
- `--enhanced-review-panel`, `--no-enhanced-review-panel`, and
  `[review] enhanced` are removed. Existing configuration files containing
  `enhanced` still parse because unknown fields are ignored.

The card also renders the command editor state (`Edit` row, `⏎ commit · esc
discard` hints) and the `d` hint (`d disable`) in review mode.

## Permanent Disable From The Panel

- `d` in review mode returns `PanelOutcome::DisableReviewPermanently`.
- The application loads the current configuration, sets `review.panel = false`,
  and saves it through the existing atomic `save_config` path.
- The surface closes, the original input is preserved, and a short note states
  that the review surface is disabled.
- Failure to write the configuration leaves the surface open and reports an
  error; nothing is released.

Test seam: `persist_review_disabled_at(path)` performs the read-modify-write
against a given path so the harness can use an isolated file while production
uses `xdg_config_path()`.

## `?` Explanation From The `-x` Confirmation

- The interactive confirmation prompt becomes `Execute now? [Y/n/?]` when an
  explanation can be shown (a usable controlling terminal).
- `y`/empty executes, `n` cancels, `?` returns `PromptResult::Explain` without
  executing.
- The application opens the card for the generated command in explain-only
  mode: footer hint `esc close`, Enter and Escape close it, `d` is not offered,
  and nothing is released or executed.
- Closing returns to the confirmation prompt. Execution still requires a
  confirmation answer.
- Review-eligible `-x` behavior is unchanged: the card is the review and
  acceptance authorizes execution without a second prompt.

## Failure Outcomes

| Condition | Outcome |
|---|---|
| Card renders, color-capable terminal | Color card |
| Card renders, no color capability | Monochrome card |
| Card cannot open | `Unavailable`; preserve input; release nothing |
| `d` config write fails | Surface stays open; error reported; release nothing |
| `?` explanation card cannot open | Note printed; confirmation prompt remains |

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
  "canonical_artifact": "givn/changes/review-panel-controls/design.md",
  "target_adr": null,
  "replacement_adr": null,
  "evidence": {
    "alternatives": [
      "Keep the plain panel or remove it; keep the confirmation fixed or let it explain on demand."
    ],
    "architectural_impact": [
      "Controls stay inside the review-surface capability; configuration persistence reuses the existing atomic save path."
    ],
    "durable_consequence": [],
    "lower_level_artifact": [],
    "existing_adr_check": [
      "ADR-0015 owns stream completion; this change only adjusts presentation selection and confirmation options."
    ]
  }
}
```

## Architecture Impact

Affected Arc42 chapters: 4 (one review presentation and its controls), 5
(adapter building block removed, card remains), 6 (confirmation explain loop
and permanent-disable flow), 8 (color as a property; configuration precedence),
10 (QS-067/QS-071 adjusted), 11 (renderer fallback risk reduced). Chapter 09 is
unchanged: no ADR is created or amended.

## Verification Contract

The existing Cucumber runner remains the executable specification. Removed
scenarios are declared with `@givn.removed`; added scenarios cover the card-only
surface, mono card, card failure, disabled path, permanent disable, and the
confirmation explanation. `./run-tests.sh` and `./run-tests.sh --e2e` remain the
verify commands.
