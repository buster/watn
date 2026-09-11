# Tasks: polish-review-detail-block

One atomic commit per scenario (RED + GREEN + REFACTOR together). Tick each box
immediately when its evidence is recorded; never batch-check.

Use-case constraints for every scenario: the detailed view is the same review
surface as the simple view; `Enter` accepts; `D` is explicit about disabling
the review; nothing evaluates a Candidate.

## Setup

- [ ] Record the baseline before any change.
  ```text
  regular output/count: <paste>
  e2e output/count: <paste>
  lib output/count: <paste>
  ```

- [ ] Prove strict mode again: remove `@wip` from one modified scenario and run
  it; the new undefined step must fail. Restore `@wip`.
  ```text
  scenario: The review card exposes the decision shortcuts
  command: `./run-tests.sh --name 'The review card exposes the decision shortcuts'`
  output: <paste>
  ```

## Scenarios

### The review card exposes the decision shortcuts (modified)

Domain constraints: the detailed hints expose accept, edit, reject, and
`disable review`; `a` is no longer a shortcut and the accept hint is unmarked.

- [ ] RED: Remove `@wip`; synchronize the permanent body; bind
  `accept should be shown with the enter key` with `unimplemented!()`. Run
  non-zero.
  ```text
  command: `./run-tests.sh --name 'The review card exposes the decision shortcuts'`
  output: <paste>
  ```
- [ ] GREEN: Delete the `Char('a')|Char('A')` accept arm, delete
  `Ink::accept_key`, render `⏎ accept` unmarked, and change the disable hint to
  `D disable review`; add the `panel.rs` unit test for ignored `a`/`A` and the
  exact enter-key step. Production files changed: `src/review/panel.rs`,
  `src/review/card.rs`.
  ```text
  command: `./run-tests.sh --name 'The review card exposes the decision shortcuts'`
  output: <paste>
  ```
- [ ] REFACTOR: Delete `review_accept_is_default` and the dead accept-shortcut
  step; re-drive `review_explain_accept_shortcut` with `r`; rerun.
  ```text
  command: `./run-tests.sh --name 'The review card exposes the decision shortcuts'`
  output: <paste>
  ```
- [ ] COMMIT: `<hash>` - feat(interactive-shell-shortcut): The review card exposes the enter accept and disable-review wording

### The view toggle switches between the simple and detailed reviews (modified)

Domain constraints: the detailed command block matches the simple view with no
label, the same indent, and a blank row before the purpose.

- [ ] RED: Remove `@wip`; synchronize the permanent body; bind the
  no-label and blank-row steps with `unimplemented!()`. Run non-zero.
  ```text
  command: `./run-tests.sh --name 'The view toggle switches between the simple and detailed reviews'`
  output: <paste>
  ```
- [ ] GREEN: Render the detailed stack with the simple four-column prefix and
  without the `Command` label, add the blank row before the purpose, and set
  `essential_rows` to `purpose + 5` for detailed only; add the card unit test
  for the equal block. Production files changed: `src/review/card.rs`.
  ```text
  command: `./run-tests.sh --name 'The view toggle switches between the simple and detailed reviews'`
  output: <paste>
  ```
- [ ] REFACTOR: Delete the now-dead `review_detailed_shows_stack` step; update
  the detailed-card unit test; rerun.
  ```text
  command: `./run-tests.sh --name 'The view toggle switches between the simple and detailed reviews'`
  output: <paste>
  ```
- [ ] COMMIT: `<hash>` - feat(interactive-shell-shortcut): The detailed view shares the simple command block

## Final verification

- [ ] Run `cargo fmt --all -- --check`
  ```text
  output: <paste>
  ```
- [ ] Run `cargo clippy --locked --all-targets -- -D warnings`
  ```text
  output: <paste>
  ```
- [ ] Run `givn lint --change polish-review-detail-block`
  ```text
  output: <paste>
  ```
- [ ] Run the full regular suite `./run-tests.sh`
  ```text
  output: <paste>
  ```
- [ ] Run the full E2E suite `./run-tests.sh --e2e`
  ```text
  output: <paste>
  ```
- [ ] Run `cargo test --locked --lib`
  ```text
  output: <paste>
  ```
- [ ] Run `givn status --change polish-review-detail-block`
  ```text
  output: <paste>
  ```
