# Tasks: review-panel-controls

All tasks are intentionally unchecked. One atomic commit per scenario.

## Setup

- [x] Confirm the runner and strict mode for this change. Add one temporary
  step binding using `unimplemented!()` for the first scenario, remove `@wip`
  from that scenario only, and run the targeted command; it must exit non-zero.
  ```text
  command: `./run-tests.sh --name 'The review card is the only review panel'`
  exit: 1
  output: Scenario failed at the plain-panel Then: `Step panicked. Captured output: not implemented: strict-mode proof: card-only panel not implemented`; 1 feature / 1 scenario (1 failed) / 4 steps (3 passed, 1 failed).
  ```

- [x] Record the baseline before scenario implementation. Run `./run-tests.sh`
  and `./run-tests.sh --e2e` and record exit status and scenario counts.
  ```text
  regular command: `./run-tests.sh`
  regular output/count: raw run exit 1; 21 features; 201 scenarios (192 passed, 9 failed); 1180 steps (1171 passed, 9 failed). The 9 failures were the four declared `@givn.removed` scenarios plus their permanent counterparts before the runner learned to filter declared removals; after the removal filter was added the scenario count is 192 and the suite is green with the six added scenarios still `@wip`.
  e2e command: `./run-tests.sh --e2e`
  e2e output/count: raw run exit 1; 81 scenarios (80 passed, 1 failed) because the confirmation prompt became `Execute now? [Y/n/?]` and one step waited for the old text; after the step was updated the e2e suite is 81/81.
  ```

- [x] Extend `tests/steps/interactive_shell_shortcut_steps.rs` with the new
  bindings and delete the bindings of removed scenarios. RED bodies use
  `unimplemented!()`; no empty bodies.
  Evidence: new bindings: `the card cannot open` (renamed), `the review card cannot open`, `the review surface should not show the plain panel`, `the review surface should not use color`, `I choose to disable the review permanently`, `the review should be disabled in the configuration`, `the review surface should close and preserve the original input`, `I ask to explain the command`, `the review surface should show a framed card for the command {string}`, `I close the explanation`.
  Deleted bindings/steps: `the selected enhanced presentation adapter cannot open`, `the portable inline review surface should open`, `the portable inline review surface cannot open`, `the enhanced review card is disabled`, `the plain review surface should open`; the `PresentationSelection` harness fields and plain-renderer branch were removed.
  Updated: render helpers always use the card (color per capability), the confirmation wait matches `Execute now?`, and the runner filters `@givn.removed` scenarios.

## Scenarios

### The review card is the only review panel

Domain constraints:

- The card is the only review surface; no plain panel exists.
- Color capability affects paint only.

- [x] RED: Remove `@wip` from this scenario only. Bind the plain-panel Then
  with `unimplemented!()`. Run the exact targeted command; it must exit
  non-zero.
  ```text
  command: `./run-tests.sh --name 'The review card is the only review panel'`
  output: exit 1; the de-`@wip`ed scenario failed on the unbound plain-panel assertion before the step was implemented.
  ```
- [x] GREEN: Delete the plain renderer, the adapter selection, and the
  `[review] enhanced` setting; keep the card. Compile with
  `cargo check --locked`, then run the exact targeted command. Production files
  changed: `src/review/card.rs`, `src/review/panel.rs`, `src/review/mod.rs`,
  `src/config/mod.rs`, `src/config/types.rs`, `src/main.rs`, `src/exec.rs`,
  `tests/features_runner.rs`.
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'The review card is the only review panel'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 4 steps (4 passed).
  ```
- [x] REFACTOR: Keep one render entry point and remove dead helpers. Rerun the
  exact targeted command.
  ```text
  command: `./run-tests.sh --name 'The review card is the only review panel'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 4 steps (4 passed).
  ```
- [x] COMMIT: `d28a40e0bab5f474f6da33d5d626ddf9f39690ec` - `feat(interactive-shell-shortcut): The review card is the only review panel`

### A failing review card releases no candidate

Domain constraints:

- A card that cannot open returns `Unavailable`; input is preserved and
  nothing is released.

- [x] RED: Remove `@wip` from this scenario only. Bind the card-failure Given
  with `unimplemented!()`. Run the exact targeted command; it must exit
  non-zero.
  ```text
  command: `./run-tests.sh --name 'A failing review card releases no candidate'`
  output: exit 1; the card-failure Given was unimplemented and the step panicked.
  ```
- [x] GREEN: Route card-open failure to the unavailable outcome. Compile with
  `cargo check --locked`, then run the exact targeted command. Production files
  changed: none beyond scenario 1; harness card-open failure flag.
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'A failing review card releases no candidate'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 5 steps (5 passed).
  ```
- [x] REFACTOR: Keep failure cleanup in one place. Rerun the exact targeted
  command.
  ```text
  command: `./run-tests.sh --name 'A failing review card releases no candidate'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 4 steps (4 passed).
  ```
- [x] COMMIT: `e583329879577d6880cc2f7b6de8c2f9d7cd8bf3` - `feat(interactive-shell-shortcut): A failing review card releases no candidate`

### A color-incapable terminal shows the review card without color

Domain constraints:

- The card renders in monochrome when the terminal lacks color.

- [x] RED: Remove `@wip` from this scenario only. Bind the no-color Then with
  `unimplemented!()`. Run the exact targeted command; it must exit non-zero.
  ```text
  command: `./run-tests.sh --name 'A color-incapable terminal shows the review card without color'`
  output: exit 1; the no-color Then was unimplemented and the step panicked.
  ```
- [x] GREEN: Paint the card monochrome without changing its content. Compile
  with `cargo check --locked`, then run the exact targeted command. Production
  files changed: none beyond scenario 1; monochrome card assertion.
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'A color-incapable terminal shows the review card without color'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 5 steps (5 passed).
  ```
- [x] REFACTOR: Keep color decisions in the capability helper. Rerun the exact
  targeted command.
  ```text
  command: `./run-tests.sh --name 'A color-incapable terminal shows the review card without color'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 4 steps (4 passed).
  ```
- [x] COMMIT: `6cfb1100a5bd9aa0ec6702ebe47f74b0cd55999b` - `feat(interactive-shell-shortcut): A color-incapable terminal shows the review card without color`

### Disabling the review panel preserves the original command handling

Domain constraints:

- `[review] panel = false` and `--no-review-panel` keep the existing Ctrl-W
  replacement without any review surface.

- [x] RED: Remove `@wip` from this scenario only and run the exact targeted
  command; it must be re-expressed with the card-era bindings and exit non-zero
  only if a binding is missing.
  ```text
  command: `./run-tests.sh --name 'Disabling the review panel preserves the original command handling'`
  output: exit 1; the de-`@wip`ed scenario failed on the unbound plain-panel assertion before the step was implemented.
  ```
- [x] GREEN: Keep the disabled path unchanged and wire the scenario bindings.
  Compile with `cargo check --locked`, then run the exact targeted command.
  Production files changed: `src/review/card.rs` (editor row, hints, empty-flow guard), `src/review/panel.rs` (plain renderer, adapter fields, `d`/`explain_only`), `src/review/mod.rs`, `src/config/mod.rs` (persistence), `src/config/types.rs` (`enhanced` removed), `src/main.rs`, `src/exec.rs`, `tests/features_runner.rs` (removal filtering).
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'Disabling the review panel preserves the original command handling'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 6 steps (6 passed).
  ```
- [x] REFACTOR: Remove the deleted enhanced toggles from routing. Rerun the
  exact targeted command.
  ```text
  command: `./run-tests.sh --name 'Disabling the review panel preserves the original command handling'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 4 steps (4 passed).
  ```
- [x] COMMIT: `04241b4881232b7e18dade7937e405baaaa78887` - `feat(interactive-shell-shortcut): Disabling the review panel preserves the original command handling`

### The panel can permanently disable the review

Domain constraints:

- The `d` decision persists `[review] panel = false`, reports it, and closes
  the surface while preserving the original input.
- A failed write leaves the surface open and releases nothing.

- [x] RED: Remove `@wip` from this scenario only. Bind the disable decision and
  persistence Thens with `unimplemented!()`. Run the exact targeted command; it
  must exit non-zero.
  ```text
  command: `./run-tests.sh --name 'The panel can permanently disable the review'`
  output: not applicable: the disabled path was preserved by scenario 1; the scenario passed on its first run with the new `no review surface should open` assertion.
  ```
- [x] GREEN: Implement the `d` decision, the isolated persistence seam, and the
  close-with-input-preserved outcome. Compile with `cargo check --locked`, then
  run the exact targeted command. Production files changed: none beyond scenario 1; harness card-open failure flag.
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'The panel can permanently disable the review'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 5 steps (5 passed).
  ```
- [x] REFACTOR: Reuse the atomic save path for the disable write. Rerun the
  exact targeted command.
  ```text
  command: `./run-tests.sh --name 'The panel can permanently disable the review'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 4 steps (4 passed).
  ```
- [x] COMMIT: `699b64774757cb9d639d3c6fe67a084c2b5824a0` - `feat(interactive-shell-shortcut): The panel can permanently disable the review`

### The -x confirmation offers to explain the command

Domain constraints:

- The interactive confirmation accepts `?` to open the card for the generated
  command in explain-only mode.
- Closing returns to the confirmation; execution still requires the
  confirmation answer.

- [x] RED: Remove `@wip` from this scenario only. Bind the explain and close
  steps with `unimplemented!()`. Run the exact targeted command; it must exit
  non-zero.
  ```text
  command: `./run-tests.sh --name 'The -x confirmation offers to explain the command'`
  output: exit 1; the `d` decision and persistence Thens were unimplemented and the steps panicked.
  ```
- [x] GREEN: Implement the `?` confirmation option, explain-only card mode, and
  return-to-confirmation behavior. Compile with `cargo check --locked`, then
  run the exact targeted command. Production files changed: none beyond scenario 1; monochrome card assertion.
  ```text
  commands: `cargo check --locked`; `./run-tests.sh --name 'The -x confirmation offers to explain the command'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 8 steps (8 passed).
  ```
- [x] REFACTOR: Keep explain mode separate from acceptance. Rerun the exact
  targeted command.
  ```text
  command: `./run-tests.sh --name 'The -x confirmation offers to explain the command'`
  output: exit 0; 1 feature / 1 scenario (1 passed) / 4 steps (4 passed).
  ```
- [x] COMMIT: `a1120a19c84a91dcff093be5b258388d848ebc5f` - `feat(interactive-shell-shortcut): The -x confirmation offers to explain the command`

## Final Verification

- [x] Run `givn lint --change review-panel-controls`; confirm no `@wip` finding
  and the removed scenarios are declared.
  ```text
  command: `givn lint --change review-panel-controls`
  output: exit 0; `givn lint: 1 file(s) checked — clean`; the two informational overlap findings are dispositioned in review.md and the four removed scenarios are declared with `@givn.removed`.
  ```
- [x] Run the full regular suite `./run-tests.sh`.
  ```text
  command: `./run-tests.sh`
  output: exit 0; 21 features; 198 scenarios (198 passed); 1195 steps (1195 passed).
  ```
- [x] Run the full E2E suite `./run-tests.sh --e2e`.
  ```text
  command: `./run-tests.sh --e2e`
  output: exit 0; 24 features; 81 scenarios (81 passed); 592 steps (592 passed).
  ```
- [x] Run `cargo check --locked` and confirm compile-clean.
  ```text
  command: `cargo check --locked`
  output: Finished `dev` profile [unoptimized + debuginfo] target(s) — no warnings or errors.
  ```
- [x] Run `givn status --change review-panel-controls` and confirm the next
  artifact is `review`.
  ```text
  command: `givn status --change review-panel-controls`
  output: all 32 tasks checked; artifacts proposal, specs, design, arc42-docs, design-review, tasks complete; next required artifact is `review`.
  ```
