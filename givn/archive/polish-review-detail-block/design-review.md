# Design Review: polish-review-detail-block

## Review Scope

Grilling covered the proposal, delta feature, design, arc42 delta, `card.rs`,
`panel.rs`, the review step definitions, and the changed arc42 chapters. The
grilling subagent ran in a fresh context; findings are dispositioned below.

## Findings And Dispositions

### The enter-key assertion was not RED and the green `a` had no absence check

**Finding (blocker):** a plain `⏎ accept` assertion passes before the change
because ANSI is stripped; the green `a` could survive implementation.

**Disposition:** Resolved. The step asserts the exact `⏎ accept` SGR sequence
and the absence of the green `1;38;5;114ma` sequence.

### The flat `essential_rows = purpose + 5` would shrink the simple budget

**Finding (blocker):** both views share the budget code; a flat `+5` reduces
the simple stack budget.

**Disposition:** Resolved. The design now specifies `purpose + 4` for simple
and `purpose + 5` for detailed.

### The blank-row assertion was inverted on the fixture

**Finding (major):** `purpose_index == first_stage_index + 2` passes before the
change and fails after; first-match indexing also breaks with wrapped stages.

**Disposition:** Resolved. The step anchors to the last stack row:
`purpose_index == last_stack_row_index + 2` with the row above the purpose
blank.

### Stale chapter-08 statements and a vacuous explanation step

**Finding (major):** chapter 08 still attributed the disable to `d` and
described three focus regions; the explanation-card step drove the now-removed
`a`, so it no longer proves that review decisions are ignored.

**Disposition:** Resolved. Chapter 08 names `D` and the current no-focus-region
keyboard contract; the explanation step drives `r` (a decision ignored in
explanation mode); a `panel.rs` unit test pins `a`/`A` as `Continue`.

### Migration and coverage gaps

**Finding (minor):** `review_accept_is_default` and `review_detailed_shows_stack`
were missing from the migration list; indentation equality and QS-067 lacked
assertions.

**Disposition:** Resolved. The migration list deletes the two stale steps,
adds a card unit test for the identical command block (including the
four-column indent), and QS-067 records the shared block plus blank separator.

## E2E Fidelity And Interaction Coverage

No interaction is added, removed, or re-driven; every accept scenario uses
Enter and no e2e step presses `a`. The twelve inventory rows keep their real
drivers.

## ADR Qualification

`NOT_QUALIFIED` with this design as the canonical artifact. ADR-0015's release
gate is untouched.

## Arc42 Independent Walk

| # | Independent result | Disposition |
|---:|---|---|
| 3 | Yes | Key list drops `a`, names `D disable review`. |
| 6 | Yes | Review narrative says Enter accepts. |
| 8 | Yes | Disable wording and focus-region fact corrected. |
| 10 | Yes | QS-067 and QS-070 describe the shared block and Enter accept. |
| 1, 2, 4, 5, 7, 9, 11, 12 | No | No other change. |

## Ubiquitous Language

No term is added or redefined; accept remains the Enter decision and disable
keeps its meaning.

## Hardening Receipt

- `design.md`: conditional `essential_rows`, exact enter-key assertions,
  last-stack-row anchoring, migration list, unit tests, QS-067.
- feature delta: removed the accept-shortcut scenario; modified the
  exposed-shortcuts and view-toggle scenarios.
- Arc42 chapters 03, 06, 08, 10 updated; `arc42.md` corrected.
- `givn lint --change polish-review-detail-block`: `@wip` notices only.
- `tasks.md`: not written.

## Status

DESIGN-REVIEW: PASS
