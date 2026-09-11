# Tasks: polish-review-detail-block

One atomic commit per scenario (RED + GREEN + REFACTOR together). Tick each box
immediately when its evidence is recorded; never batch-check.

Use-case constraints for every scenario: the detailed view is the same review
surface as the simple view; `Enter` accepts; `D` is explicit about disabling
the review; nothing evaluates a Candidate.

## Setup

- [x] Record the baseline before any change.
  ```text
  regular output/count: exit 0; 21 features; 218 scenarios (218 passed); 1328 steps (1328 passed)
  e2e output/count: exit 0; 25 features; 88 scenarios (88 passed); 665 steps (665 passed)
  lib output/count: exit 0; 84 passed; 0 failed
  ```

- [x] Prove strict mode again: remove `@wip` from one modified scenario and run
  it; the new undefined step must fail. Restore `@wip`.
  ```text
  scenario: The review card exposes the decision shortcuts
  command: `./run-tests.sh --name 'The review card exposes the decision shortcuts'`
  output: exit 1; 2 features; 2 scenarios (1 passed, 1 failed); 17 steps (16 passed, 1 failed); undefined enter-key step.
  ```

## Scenarios

### The review card exposes the decision shortcuts (modified)

Domain constraints: the detailed hints expose accept, edit, reject, and
`disable review`; `a` is no longer a shortcut and the accept hint is unmarked.

- [x] RED: Remove `@wip`; synchronize the permanent body; bind
  `accept should be shown with the enter key` with `unimplemented!()`. Run
  non-zero.
  ```text
  command: `./run-tests.sh --name 'The review card exposes the decision shortcuts'`
  output: exit 1 (strict proof) on the undefined enter-key step.
  ```
- [x] GREEN: Delete the `Char('a')|Char('A')` accept arm, delete
  `Ink::accept_key`, render `⏎ accept` unmarked, and change the disable hint to
  `D disable review`; add the `panel.rs` unit test for ignored `a`/`A` and the
  exact enter-key step. Production files changed: `src/review/panel.rs`,
  `src/review/card.rs`.
  ```text
  command: `./run-tests.sh --name 'The review card exposes the decision shortcuts'`
  output: exit 0; 2 features; 2 scenarios (2 passed); 18 steps (18 passed).
  ```
- [x] REFACTOR: Delete `review_accept_is_default` and the dead accept-shortcut
  step; re-drive `review_explain_accept_shortcut` with `r`; rerun.
  ```text
  command: `./run-tests.sh --name 'The review card exposes the decision shortcuts'`
  output: exit 0; 2 features; 2 scenarios (2 passed); 18 steps (18 passed).
  ```
- [x] COMMIT: `e6414a0` - feat(interactive-shell-shortcut): The review card exposes the enter accept and disable-review wording

### The view toggle switches between the simple and detailed reviews (modified)

Domain constraints: the detailed command block matches the simple view with no
label, the same indent, and a blank row before the purpose.

- [x] RED: Remove `@wip`; synchronize the permanent body; bind the
  no-label and blank-row steps with `unimplemented!()`. Run non-zero.
  ```text
  command: `./run-tests.sh --name 'The view toggle switches between the simple and detailed reviews'`
  output: undefined-step failure before the new steps existed; first synchronized run then failed on the last-stack-row anchor, which was fixed to the last stage rather than the selected stage.
  ```
- [x] GREEN: Render the detailed stack with the simple four-column prefix and
  without the `Command` label, add the blank row before the purpose, and set
  `essential_rows` to `purpose + 5` for detailed only; add the card unit test
  for the equal block. Production files changed: `src/review/card.rs`.
  ```text
  command: `./run-tests.sh --name 'The view toggle switches between the simple and detailed reviews'`
  output: exit 0; 2 features; 2 scenarios (2 passed); 28 steps (28 passed).
  ```
- [x] REFACTOR: Delete the now-dead `review_detailed_shows_stack` step; update
  the detailed-card unit test; rerun.
  ```text
  command: `./run-tests.sh --name 'The view toggle switches between the simple and detailed reviews'`
  output: exit 0; 2 features; 2 scenarios (2 passed); 28 steps (28 passed).
  ```
- [x] COMMIT: `00328ff` - feat(interactive-shell-shortcut): The detailed view shares the simple command block

## Final verification

- [x] Run `cargo fmt --all -- --check`
  ```text
  output: exit 0; clean.
  ```
- [x] Run `cargo clippy --locked --all-targets -- -D warnings`
  ```text
  output: exit 0; clean.
  ```
- [x] Run `givn lint --change polish-review-detail-block`
  ```text
  output: exit 0; clean.
  ```
- [x] Run the full regular suite `./run-tests.sh`
  ```text
  output: exit 0; 21 features; 219 scenarios (219 passed); 1347 steps (1347 passed).
  ```
- [x] Run the full E2E suite `./run-tests.sh --e2e`
  ```text
  output: exit 0; 24 features; 88 scenarios (88 passed); 665 steps (665 passed).
  ```
- [x] Run `cargo test --locked --lib`
  ```text
  output: exit 0; 86 passed; 0 failed.
  ```
- [x] Run `givn status --change polish-review-detail-block`
  ```text
  output: next required artifact is `review`.
  ```
