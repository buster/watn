# Design: polish-review-detail-block

## Use-Case Traceability

| Contract (usecase.md) | Design decision | Evidence |
|---|---|---|
| The simple and detailed views are two presentations of the same surface | The detailed command block uses the simple view's stack rows and blank separator; only Intent and the extra rows differ | View-toggle scenario |
| Review decisions are direct shortcuts | `Enter` remains the accept decision; the `a` alias is removed from handling and hints | Exposed-shortcuts scenario, removed accept-shortcut scenario |
| A permanent disable is explicit about what it disables | The detailed hint reads `D disable review` | Exposed-shortcuts scenario |

## Rendering Changes

`src/review/card.rs`:

- Detailed layout rows become exactly: Intent, blank, stack, blank, purpose,
  [editor], [chooser], [error], blank, hints.
- The stack no longer uses the `Command` label or a per-view base indent; both
  views render `stack_window_rows` with the same four-column stage prefix and
  `stack_text_width = inner - 4`.
- `essential_rows` stays `purpose + 4` for the simple view and becomes
  `purpose + 5` for the detailed view (intent, intent blank, stack blank,
  purpose blank, hints). The conditional is required: a flat `+5` would shrink
  the simple stack budget.
- The purpose indent is the simple view's four spaces in both views.
- Detailed hints lose the green accept letter: `⏎ accept · ↑↓ stage · e edit ·
  r reject · d/? simple · D disable review · esc`. `Ink::accept_key` is deleted.
- Simple hints are unchanged: `⏎ accept · ↑↓ stage · d/? details · esc`.

## Input Changes

`src/review/panel.rs`: delete the `Char('a') | Char('A')` accept arm. `Enter`
still accepts, and `Escape`/`c` still cancel. The `explain_only` decision set is
unchanged.

## Interfaces And Step Migration

- `src/review/card.rs`: detailed rows, hint strings, `essential_rows`, deleted
  `accept_key`, unit tests.
- `src/review/panel.rs`: deleted accept alias.
- `tests/steps/interactive_shell_shortcut_steps.rs`:
  - synchronize the permanent view-toggle body, then delete the now-dead
    `review_detailed_shows_stack` step;
  - add `the detailed review should show the command without a label` (asserts
    no `Command` label and the same four-column stage prefix as the simple
    view), `the detailed review should keep a blank row between command and
    purpose` (anchored to the last stack row: `purpose_index ==
    last_stack_row_index + 2`, with the row above the purpose blank), and
    `accept should be shown with the enter key` (asserts the exact
    `⏎ accept` SGR sequence and the absence of the green `a`);
  - update `review_decision_keys_emphasized` to `D disable review`, drop the
    green-accept assertion, and delete `review_accept_is_default`;
  - re-drive `review_explain_accept_shortcut` with `r` (a decision that is
    ignored in explanation mode) instead of the removed `a`;
  - delete the dead accept-shortcut step.
- Unit tests: `panel.rs` asserts `a`/`A` return `Continue` and leave the state
  unchanged; `card.rs` asserts the detailed command block equals the simple
  block with the extra intent row and the blank separator.
- Delta feature:
  `givn/changes/polish-review-detail-block/specs/use-shell/interactive-shell-shortcut.feature`.
- Modified permanent scenario bodies are synchronized during implementation.

## Test Runner

- Unit/integration: `./run-tests.sh`
- E2E: `./run-tests.sh --e2e`
- Single scenario:
  `./run-tests.sh --name 'The view toggle switches between the simple and detailed reviews'`
- Strict mode: `tests/features_runner.rs:208` calls `.fail_on_skipped()`;
  not-yet-implemented steps use `unimplemented!()`.
- Formatting and lints: `cargo fmt --all -- --check` and
  `cargo clippy --locked --all-targets -- -D warnings`.
- Toolchain: `rust-toolchain.toml` pins `1.97.1`; no new dependencies.

## Interaction Coverage Matrix

No interaction is added, removed, or re-driven. The existing twelve
`interactive-shell-shortcut` and `shell-completions` inventory rows keep their
`@e2e` scenarios and real drivers; only the accept key alias disappears, and
Enter already drives every accept scenario.

## Failure Outcomes

| Condition | Outcome |
|---|---|
| `a` pressed in review | `Continue` with unchanged state (unit-tested); `Enter` accepts |
| Detailed view | Command block identical to the simple view with a blank separator before the purpose |
| No color support | The same hint text without escape sequences |

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
  "canonical_artifact": "givn/changes/polish-review-detail-block/design.md",
  "target_adr": null,
  "replacement_adr": null,
  "evidence": {
    "alternatives": [],
    "architectural_impact": [],
    "durable_consequence": [],
    "lower_level_artifact": [
      "One removed key alias and one shared render block are presentation and input contracts owned by this design and the Gherkin scenarios."
    ],
    "existing_adr_check": [
      "ADR-0015 owns the release gate, which is untouched; no ADR owns the accept alias."
    ]
  }
}
```

## Architecture Impact

- Chapter 03 context-and-scope: the Developer key list drops `a`.
- Chapter 06 runtime-view: the review narrative says Enter accepts, not Enter or
  `a`.
- Chapter 08 crosscutting-concepts: the explicit-review-decisions line says
  Enter accepts.
- Chapter 10 quality-requirements: QS-070 says Enter accepts.
- Chapters 01, 02, 04, 05, 07, 09, 11, 12: unchanged.

## Verification Contract

The existing Cucumber runner (`./run-tests.sh` and `./run-tests.sh --e2e`)
remains the executable specification; the delta `.feature` file is the
authoritative test surface.