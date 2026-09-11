# Tasks: slim-review-detail-view

One atomic commit per scenario (RED + GREEN + REFACTOR together). Tick each box
immediately when its evidence is recorded; never batch-check.

Use-case constraints for every scenario: the review surface stays small,
transient, and inline; unsupported command-flow portions remain visible and
reviewable without a presentation marker; every Candidate requires an explicit
final decision; the ADR-0015 release boundary is unchanged.

## Setup

- [ ] Record the baseline before any change.
  ```text
  regular command: `./run-tests.sh`
  regular output/count: <paste>
  e2e command: `./run-tests.sh --e2e`
  e2e output/count: <paste>
  lib command: `cargo test --locked --lib`
  lib output/count: <paste>
  ```

- [ ] Prove strict mode again for this change: remove `@wip` from one modified
  scenario with no updated step definitions and run it; it must exit non-zero.
  Restore `@wip`.
  ```text
  scenario: Unsupported command flow remains reviewable
  command: `./run-tests.sh --name 'Unsupported command flow remains reviewable'`
  output: <paste>
  ```

## Non-E2E scenarios

### Unsupported command flow remains reviewable (modified)

Domain constraints: unsupported stages stay visible with their exact text; the
amber ellipsis and support labels are gone; acceptance and cancellation remain.

- [ ] RED: Remove `@wip`; synchronize the permanent body; bind the marker
  absence step with `unimplemented!()`. Run non-zero.
  ```text
  command: `./run-tests.sh --name 'Unsupported command flow remains reviewable'`
  output: <paste>
  ```
- [ ] GREEN: Delete `UNDECOMPOSED_STAGE_MARKER` and its append and reserve in
  `stage_group_rows`; implement the marker-absence step as an assertion on the
  ANSI render. Production files changed: `src/review/card.rs`.
  ```text
  command: `./run-tests.sh --name 'Unsupported command flow remains reviewable'`
  output: <paste>
  ```
- [ ] REFACTOR: Update `panel.rs` and `card.rs` unit tests for the missing
  marker; rerun.
  ```text
  command: `./run-tests.sh --name 'Unsupported command flow remains reviewable'`
  output: <paste>
  ```
- [ ] COMMIT: `<hash>` - feat(interactive-shell-shortcut): Unsupported command flow remains reviewable without a marker

### The review card exposes the decision shortcuts (modified)

Domain constraints: the detailed hints expose accept, edit, reject, and
disable; `cancel` is no longer advertised although Escape still cancels.

- [ ] RED: Remove `@wip`; synchronize the permanent body; bind the negative
  `not show "cancel"` step with `unimplemented!()`. Run non-zero.
  ```text
  command: `./run-tests.sh --name 'The review card exposes the decision shortcuts'`
  output: <paste>
  ```
- [ ] GREEN: Drop the `cancel` hint from both view hint builders; implement the
  ANSI-stripped negative step; rewrite the acceptance/cancellation step to the
  `esc` hint plus the `Cancelled` outcome; update the emphasis step keys.
  Production files changed: `src/review/card.rs`.
  ```text
  command: `./run-tests.sh --name 'The review card exposes the decision shortcuts'`
  output: <paste>
  ```
- [ ] REFACTOR: Rerun.
  ```text
  command: `./run-tests.sh --name 'The review card exposes the decision shortcuts'`
  output: <paste>
  ```
- [ ] COMMIT: `<hash>` - feat(interactive-shell-shortcut): The review card stops advertising cancel

### The view toggle switches between the simple and detailed reviews (modified)

Domain constraints: the detailed view keeps Intent, stack, purpose, and hints,
and no longer renders flow or stage labels; the row budget stays correct.

- [ ] RED: Remove `@wip`; synchronize the permanent body; bind the label
  absence step with `unimplemented!()`. Run non-zero.
  ```text
  command: `./run-tests.sh --name 'The view toggle switches between the simple and detailed reviews'`
  output: <paste>
  ```
- [ ] GREEN: Delete `flow_strip`, the `Flow` and `Stage` rows, and the blank
  before them; set `essential_rows` to `purpose + 4`; change the empty-flow
  placeholder to `no flow stages`; implement the label-absence step. Production
  files changed: `src/review/card.rs`.
  ```text
  command: `./run-tests.sh --name 'The view toggle switches between the simple and detailed reviews'`
  output: <paste>
  ```
- [ ] REFACTOR: Update the detailed-card and panel unit tests; rerun.
  ```text
  command: `./run-tests.sh --name 'The view toggle switches between the simple and detailed reviews'`
  output: <paste>
  ```
- [ ] COMMIT: `<hash>` - feat(interactive-shell-shortcut): The detailed view drops the flow and stage rows

### The review panel reports its disable clearly

Domain constraints: the re-enable hint names `review panel disabled`, is
printed before the released command, is amber/bold with a `⚠`, and falls back
to plain text without color support.

- [ ] RED: Add the `disable_hint` unit test for the exact colored and plain
  strings and run the lib test; it must fail before the formatter exists.
  ```text
  command: `cargo test --locked --lib disable_hint`
  output: <paste>
  ```
- [ ] GREEN: Add `disable_hint(use_color)` to `src/review/card.rs`, export it,
  and print it on stderr before the candidate in the disable branch. Production
  files changed: `src/review/card.rs`, `src/review/mod.rs`, `src/main.rs`.
  ```text
  command: `cargo test --locked --lib disable_hint`
  output: <paste>
  ```

## E2E scenario

### The panel can permanently disable the review (modified)

Domain constraints: the real watn PTY shows the new hint wording and still
releases only the candidate to stdout with exit 0.

- [ ] RED: Remove `@wip`; synchronize the permanent body; run the E2E command
  targeted at the scenario; the new wording assertion must fail.
  ```text
  command: `./run-tests.sh --e2e --name 'The panel can permanently disable the review'`
  output: <paste>
  ```
- [ ] GREEN: Implement the e2e wording assertion (and the PTY step if needed);
  run the targeted E2E command.
  ```text
  command: `./run-tests.sh --e2e --name 'The panel can permanently disable the review'`
  output: <paste>
  ```
- [ ] COMMIT: `<hash>` - feat(interactive-shell-shortcut): The panel reports its disable clearly

## Final verification

- [ ] Run `cargo fmt --all -- --check`
  ```text
  output: <paste>
  ```
- [ ] Run `cargo clippy --locked --all-targets -- -D warnings`
  ```text
  output: <paste>
  ```
- [ ] Run `givn lint --change slim-review-detail-view`
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
- [ ] Run `givn status --change slim-review-detail-view`
  ```text
  output: <paste>
  ```
