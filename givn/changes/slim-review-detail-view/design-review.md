# Design Review: slim-review-detail-view

## Review Scope

Grilling covered `proposal.md`, the delta feature, `design.md`, `arc42.md`,
the permanent `use-shell` scenarios and step definitions, `src/review/card.rs`,
`src/review/panel.rs`, the disable branch in `src/main.rs`, and the changed
arc42 chapters. The grilling subagent ran in a fresh context; its findings are
dispositioned below.

## Findings And Dispositions

### The cancellation step contradicts the removed hint

**Finding (blocker):** `the review surface should still offer final acceptance
and cancellation` asserts the literal word `cancel`, which the same change
removes from the simple hints.

**Disposition:** Resolved. The step now asserts the `esc` hint and drives
Escape to `PanelOutcome::Cancelled`, so the contract is "cancellation is
offered", not "the word is printed".

### The negative assertions were vacuous

**Finding (major):** `should not show "cancel"` would pass on the colorized
pre-change render because `c` and `ancel` are separate SGR spans, and the
view-toggle fixture is a supported candidate, so `should not mark undecomposed
stages` could not fail.

**Disposition:** Resolved. The negative assertion compares ANSI-stripped plain
text and therefore fails on the old hints. The marker absence assertion moved
to the unsupported-syntax scenario, where the old amber `…` is actually
produced, so it fails before the change.

### Row-budget and blank accounting

**Finding (major):** `essential_rows` becomes `purpose + 4` only if the blank
row before `Flow` is deleted with the rows; otherwise a stale blank remains.

**Disposition:** Resolved. The design names the blank row deletion explicitly,
sets `essential_rows = purpose + 4`, and adds a unit test for the detailed row
order and removed labels.

### Under-enumerated test migration

**Finding (major):** `src/review/panel.rs`, three step definitions, and the
empty-flow placeholder were missing from the migration list; `StageSupport`
could become an unused import under `-D warnings`.

**Disposition:** Resolved. The migration list now names `panel.rs`, the
placeholder text change, the deleted steps, and exact replacement assertions.
The verification contract includes `cargo fmt --all -- --check` and
`cargo clippy --locked --all-targets -- -D warnings`.

### Arc42 omissions

**Finding (major):** Chapter 06 still claimed incomplete flow "marks
unsupported portions"; chapter 04 still claimed "visible unsupported
portions"; the glossary Simple review view still listed the `cancel` hint.

**Disposition:** Resolved. All three lines were updated, chapter 06 is marked
affected in `arc42.md`, and the design's Architecture Impact list matches.

### Hint ordering and styling observability

**Finding (major):** The e2e redirects stdout, so it cannot observe the hint
position or the amber/bold styling; color detection was not gated on stderr.

**Disposition:** Resolved. `disable_hint(use_color)` lives in `card.rs` and is
unit-tested for the exact colored and plain strings. The design states that
ordering is by construction (`eprintln!` before `println!`; the panel path
already requires a usable controlling terminal, so colored stderr is a
terminal), and the e2e proves the wording and release through real interfaces.

### Styling mismatch and placeholder wording

**Finding (minor):** The proposal and design disagreed on the bold invocation,
and `no supported flow stages` was the last visible support word.

**Disposition:** Resolved. The exact strings are pinned in the design and the
unit test; the placeholder becomes `no flow stages`.

## E2E Fidelity And Interaction Coverage

No interaction is added, removed, or re-driven. The twelve inventory rows keep
their `@e2e` scenarios and real PTY/subprocess drivers; the disable scenario
additionally asserts the new wording. No `@e2e` tag was removed.

## ADR Qualification

`NOT_QUALIFIED` with this design as the canonical artifact. ADR-0015's release
gate and its stderr promise are unchanged; only the hint wording and ordering
change.

## Arc42 Independent Walk

| # | Independent result | Disposition |
|---:|---|---|
| 4 | Yes | Review presentation row drops the position row and the visible-marker claim. |
| 5 | Yes | Renderer drops flow/stage rows and the support marker; flow tracks support internally. |
| 6 | Yes | Unsupported-flow sentence stops claiming visible marking. |
| 10 | Yes | QS-067 describes the slimmer detailed view. |
| 12 | Yes | Detailed view drops the flow position; simple view drops the cancel hint. |
| 1, 2, 3, 7, 8, 9, 11 | No | No goals, constraints, context, deployment, crosscut, decision, or risk change. |

All 12 chapter files exist with real content and no ASCII-art diagram.

## Ubiquitous Language

No term is added. `Detailed review view` and `Simple review view` are narrowed
to the new presentation; all other terms are unchanged and used consistently.

## Hardening Receipt

- `design.md`: blank-row deletion and `purpose + 4`, `disable_hint` in
  `card.rs` with exact strings, stderr-terminal rationale, migration list,
  fmt/clippy gate, chapter set.
- feature delta: one removed, four modified (one E2E); the marker-absence
  assertion moved to the unsupported fixture.
- Arc42 chapters 04, 05, 06, 10, 12 updated; `arc42.md` corrected.
- `givn lint --change slim-review-detail-view`: exit 0 with `@wip` notices and
  one advisory subset notice dispositioned in the coverage review.
- `tasks.md`: not written.

## Status

DESIGN-REVIEW: PASS
