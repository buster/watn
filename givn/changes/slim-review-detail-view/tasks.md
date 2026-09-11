# Tasks: slim-review-detail-view

One atomic commit per scenario (RED + GREEN + REFACTOR together). Tick each box
immediately when its evidence is recorded; never batch-check.

Use-case constraints for every scenario: the review surface stays small,
transient, and inline; unsupported command-flow portions remain visible and
reviewable without a presentation marker; every Candidate requires an explicit
final decision; the ADR-0015 release boundary is unchanged.

## Setup

- [x] Record the baseline before any change.
  ```text
  regular command: `./run-tests.sh`
  regular output/count: exit 0; 21 features; 223 scenarios (223 passed); 1363 steps (1363 passed)
  e2e command: `./run-tests.sh --e2e`
  e2e output/count: exit 0; 25 features; 86 scenarios (86 passed); 637 steps (637 passed)
  lib command: `cargo test --locked --lib`
  lib output/count: exit 0; 84 passed; 0 failed
  ```

- [x] Prove strict mode again for this change: remove `@wip` from one modified
  scenario with no updated step definitions and run it; it must exit non-zero.
  Restore `@wip`.
  ```text
  scenario: Unsupported command flow remains reviewable
  command: `./run-tests.sh --name 'Unsupported command flow remains reviewable'`
  output: exit 1; 2 features; 2 scenarios (1 passed, 1 failed); 9 steps (8 passed, 1 failed); the delta copy failed on the undefined marker-absence step.
  ```

## Non-E2E scenarios

### Unsupported command flow remains reviewable (modified)

Domain constraints: unsupported stages stay visible with their exact text; the
amber ellipsis and support labels are gone; acceptance and cancellation remain.

- [x] RED: Remove `@wip`; synchronize the permanent body; bind the marker
  absence step with `unimplemented!()`. Run non-zero.
  ```text
  command: `./run-tests.sh --name 'Unsupported command flow remains reviewable'`
  output: exit 1 (strict proof) on the undefined marker-absence step.
  ```
- [x] GREEN: Delete `UNDECOMPOSED_STAGE_MARKER` and its append and reserve in
  `stage_group_rows`; implement the marker-absence step as an assertion on the
  ANSI render. Production files changed: `src/review/card.rs`.
  ```text
  command: `./run-tests.sh --name 'Unsupported command flow remains reviewable'`
  output: exit 0; 2 features; 2 scenarios (2 passed); 10 steps (10 passed).
  ```
- [x] REFACTOR: Update `panel.rs` and `card.rs` unit tests for the missing
  marker; rerun.
  ```text
  command: `./run-tests.sh --name 'Unsupported command flow remains reviewable'`
  output: exit 0; 2 features; 2 scenarios (2 passed); 10 steps (10 passed); lib unit tests updated in the same commit.
  ```
- [x] COMMIT: `c15cc91` - feat(interactive-shell-shortcut): Unsupported command flow remains reviewable without a marker

### The review card exposes the decision shortcuts (modified)

Domain constraints: the detailed hints expose accept, edit, reject, and
disable; `cancel` is no longer advertised although Escape still cancels.

- [x] RED: Remove `@wip`; synchronize the permanent body; bind the negative
  `not show "cancel"` step with `unimplemented!()`. Run non-zero.
  ```text
  command: `./run-tests.sh --name 'The review card exposes the decision shortcuts'`
  output: undefined-step failure under strict mode before the negative step existed.
  ```
- [x] GREEN: Drop the `cancel` hint from both view hint builders; implement the
  ANSI-stripped negative step; rewrite the acceptance/cancellation step to the
  `esc` hint plus the `Cancelled` outcome; update the emphasis step keys.
  Production files changed: `src/review/card.rs`.
  ```text
  command: `./run-tests.sh --name 'The review card exposes the decision shortcuts'`
  output: exit 0; 2 features; 2 scenarios (2 passed); 18 steps (18 passed).
  ```
- [x] REFACTOR: Rerun.
  ```text
  command: `./run-tests.sh --name 'The review card exposes the decision shortcuts'`
  output: exit 0; 2 features; 2 scenarios (2 passed); 18 steps (18 passed).
  ```
- [x] COMMIT: `0329b13` - feat(interactive-shell-shortcut): The review card stops advertising cancel

### The view toggle switches between the simple and detailed reviews (modified)

Domain constraints: the detailed view keeps Intent, stack, purpose, and hints,
and no longer renders flow or stage labels; the row budget stays correct.

- [x] RED: Remove `@wip`; synchronize the permanent body; bind the label
  absence step with `unimplemented!()`. Run non-zero.
  ```text
  command: `./run-tests.sh --name 'The view toggle switches between the simple and detailed reviews'`
  output: undefined-step failure under strict mode before the label-absence step existed.
  ```
- [x] GREEN: Delete `flow_strip`, the `Flow` and `Stage` rows, and the blank
  before them; set `essential_rows` to `purpose + 4`; change the empty-flow
  placeholder to `no flow stages`; implement the label-absence step. Production
  files changed: `src/review/card.rs`.
  ```text
  command: `./run-tests.sh --name 'The view toggle switches between the simple and detailed reviews'`
  output: exit 0; 2 features; 2 scenarios (2 passed); 26 steps (26 passed).
  ```
- [x] REFACTOR: Update the detailed-card and panel unit tests; delete the dead
  `review_compact_overview` and `show only the selected stage` steps; rerun.
  ```text
  command: `./run-tests.sh --name 'The view toggle switches between the simple and detailed reviews'`
  output: exit 0; 2 features; 2 scenarios (2 passed); 26 steps (26 passed).
  ```
- [x] COMMIT: `0bd83a2` - feat(interactive-shell-shortcut): The detailed view drops the flow and stage rows

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
