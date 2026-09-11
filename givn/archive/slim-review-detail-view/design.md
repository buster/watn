# Design: slim-review-detail-view

## Use-Case Traceability

| Contract (usecase.md) | Design decision | Evidence |
|---|---|---|
| The review surface opens in a simple view with the stage stack and selected-stage purpose | Unchanged; the simple view keeps its layout and only loses the `cancel` word from the hints | Existing simple scenarios |
| A direct shortcut switches to the detailed view and back | The detailed view keeps Intent, stack, purpose, decision hints, editor, and chooser; the `Flow` strip and `Stage` row are removed | "The view toggle switches between the simple and detailed reviews" |
| An unsupported command-flow portion remains visible and reviewable | Unsupported stages stay visible with their exact text; the amber ellipsis is removed and the support state remains internal | "Unsupported command flow remains reviewable" |
| Permanently disabling the review releases the current candidate, tells how to re-enable, and ends the invocation | The re-enable hint moves before the command, is printed on stderr with an amber `⚠` and the bold phrase `review panel disabled` | "The panel can permanently disable the review" |

## Rendering Changes

`src/review/card.rs`:

- Delete `flow_strip` and the `Flow` row; delete the `Stage x/y` row and its
  `supported`/`unsupported` label, and delete the blank row that preceded the
  Flow row. `reserve` no longer adds marker width.
- Delete `UNDECOMPOSED_STAGE_MARKER` and its append in `stage_group_rows`. The
  `StageSupport` state stays in the flow model and is no longer rendered; the
  `StageSupport` reference in `card.rs` disappears with `flow_strip` and the
  marker. The empty-flow placeholder text becomes `no flow stages`.
- Detailed layout rows become exactly: Intent, blank, stack, purpose, [editor],
  [chooser], [error], blank, hints. `essential_rows` for the detailed view drops
  from `purpose + 7` to `purpose + 4` (intent + intent blank + hint blank +
  hints). A unit test pins the row order and the removed labels.
- Hints lose the `cancel` word in both views. `c` remains a working shortcut;
  only its advertisement is removed. Detailed:
  `⏎ accept · ↑↓ stage · e edit · r reject · d/? simple · D disable · esc`.
  Simple: `⏎ accept · ↑↓ stage · d/? details · esc`.
- The truncation `…` for an over-long single stage and the `⋮` hidden-window
  marker stay; they signal cut or hidden content, not support state.

## Disable Hint

`src/review/card.rs` gains a pure formatter so the bin stays thin and the
wording is unit-tested:

- `pub fn disable_hint(use_color: bool) -> String`:
  - color: `\u{1b}[1;38;5;214m⚠ review panel disabled\u{1b}[0m — re-enable with: \u{1b}[1;97mwatn --review-panel\u{1b}[0m`
  - no color: `⚠ review panel disabled — re-enable with: watn --review-panel`
- `src/main.rs` calls it with the already computed `use_color`. The disable
  branch is only reachable after `controlling_terminal_is_usable()` accepted the
  invocation, so stderr is a terminal whenever the hint is colored; `NO_COLOR`
  and `TERM=dumb` fall back to plain.
- The disable branch calls `eprintln!` with the hint, then `println!` with the
  candidate, then exits 0. Rust stdout is line-buffered on a terminal and
  stderr is unbuffered, so the hint is visible above the command when both
  reach the terminal. The command stays the only stdout content, and the Ctrl-W
  widget captures stdout only, so the hint stays visible there. Ordering is by
  construction; the e2e scenario proves the wording and release, the unit test
  pins the formatted strings.
- The flag-only confirmations keep their current plain stderr wording.

## Interfaces And Step Migration

- `src/review/card.rs`: renderer rows, marker constant, hints, placeholder text,
  `disable_hint`, unit tests including the removed labels and the exact
  colored/plain hint strings.
- `src/review/panel.rs`: the narrow unit test drops its amber `…` assertion;
  the empty-flow placeholder unit test follows the new text.
- `src/main.rs`: print order and the `disable_hint` call.
- `tests/steps/interactive_shell_shortcut_steps.rs`:
  - delete `unsupported command-flow portions should be marked` and
    `the card should mark the stage as not decomposed`;
  - delete the now-unused `review_compact_overview` and the dead
    `Stage {}/{}` step;
  - add `the review surface should not show {string}` over ANSI-stripped plain
    text, `the review surface should not mark undecomposed stages` (asserts the
    amber `…` is absent), and `the detailed review should not show the flow or
    stage labels`;
  - rewrite `review_acceptance_and_cancellation_offered` to assert the `esc`
    hint and the `Cancelled` outcome instead of the word `cancel`;
  - update the emphasis step to the new keys.
- `tests/steps/interactive_shell_shortcut_e2e_steps.rs`: the disable scenario
  additionally asserts `review panel disabled`.
- Delta feature:
  `givn/changes/slim-review-detail-view/specs/use-shell/interactive-shell-shortcut.feature`.
- Modified permanent scenario bodies are synchronized during implementation.

## Test Runner

- Unit/integration: `./run-tests.sh`
- E2E: `./run-tests.sh --e2e`
- Single scenario:
  `./run-tests.sh --name 'The view toggle switches between the simple and detailed views'`
- Strict mode: `tests/features_runner.rs:208` calls `.fail_on_skipped()`;
  not-yet-implemented steps use `unimplemented!()`.
- Formatting and lints: `cargo fmt --all -- --check` and
  `cargo clippy --locked --all-targets -- -D warnings` (the CI gates); removing
  `flow_strip` and the marker must not leave unused imports or locals.
- Toolchain: `rust-toolchain.toml` pins `1.97.1`; no new dependencies.

## Interaction Coverage Matrix

No interaction is added, removed, or re-driven. The existing twelve
`interactive-shell-shortcut` and `shell-completions` inventory rows keep their
`@e2e` scenarios and real PTY/subprocess drivers; only assertion texts inside
two existing scenarios change.

## Failure Outcomes

| Condition | Outcome |
|---|---|
| Unsupported stage | Stage text renders as-is; no marker, no support label |
| Empty flow | Placeholder reads `no flow stages` |
| Over-long single stage | Unstyled `…` truncation marker stays |
| Hidden stage window | Dim `⋮` stays |
| No color support | Disable hint prints without escape sequences |
| stdout redirected | Hint on the terminal, command only in the redirected stream |

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
  "canonical_artifact": "givn/changes/slim-review-detail-view/design.md",
  "target_adr": null,
  "replacement_adr": null,
  "evidence": {
    "alternatives": [],
    "architectural_impact": [],
    "durable_consequence": [],
    "lower_level_artifact": [
      "Row removal, hint wording, and the disable hint styling are presentation contracts owned by this design and the Gherkin scenarios; the ADR-0015 release boundary itself is unchanged."
    ],
    "existing_adr_check": [
      "ADR-0015 owns the release gate, which keeps its explicit-final-decision semantics; only the human-readable hint changes."
    ]
  }
}
```

## Architecture Impact

- Chapter 04 solution-strategy: the review presentation row no longer lists a
  position row, and the review-strategy line no longer claims visible marker
  text for unsupported portions.
- Chapter 05 building-block-view: the Review card renderer loses the flow strip,
  stage row, and support marker; the Command flow row tracks support internally.
- Chapter 06 runtime-view: the unsupported-flow sentence no longer says the
  surface marks unsupported portions.
- Chapter 10 quality-requirements: QS-067 describes the slimmer detailed view.
- Chapter 12 glossary: the Detailed review view definition drops the flow
  position and the Simple review view definition drops the `cancel` hint.
- Chapters 01, 02, 03, 07, 08, 09, 11: unchanged (the key behavior and the
  ADR-0015 release boundary are untouched).

## Verification Contract

The existing Cucumber runner (`./run-tests.sh` and `./run-tests.sh --e2e`)
remains the executable specification; the delta `.feature` file is the
authoritative test surface.
