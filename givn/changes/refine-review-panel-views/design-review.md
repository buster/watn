# Design Review: refine-review-panel-views

## Review Scope

Grilling covered `proposal.md`, the change feature delta, `design.md`,
`arc42.md`, the change-side `usecase.md`, the permanent `use-shell` contract
and capability, the glossary, the review panel/renderer/flow/response code, the
review and question paths in `src/main.rs`, the flag handling and config
persistence, the regular and E2E step harness, and the affected Arc42 chapters.
The grilling subagent ran in a fresh context and returned a ranked finding list
with file:line evidence; its findings are dispositioned below. The user
delegated the change through archive, so the orchestrator resolved the four
product questions with the rationale recorded here.

## Findings And Dispositions

### Key gating would have broken unmodified permanent scenarios

**Finding (blocker):** the design ignored `e`/`r` in the simple view, but more
than a dozen permanent scenarios drive those keys directly and assert the
editor or chooser.

**Disposition:** Resolved. `e` and `r` stay available in the simple view and
promote the surface to the detailed layout (`details = true`). The simple view
stays minimal in what it *shows*; an experienced developer is not forced to
toggle first. The planned `@givn.modified` for "The card's edit shortcut opens
the command editor" was dropped because the scenario remains true unchanged.

### Simple view hides Intent, tier, and decision hints used by permanent scenarios

**Finding (blocker):** "Rephrasing starts a new candidate cycle", "Higher-tier
review generates a candidate at the next configured tier", "The review card
emphasizes the decision shortcut keys", and "The review card exposes the
decision shortcuts" assert Intent, tier/provider, or full hint emphasis in what
is now the simple default.

**Disposition:** Resolved. A new step "I switch to the detailed view" is added
to those four scenarios as `@givn.modified` entries; the detailed view owns
those assertions. Editor and chooser scenarios need no change because opening
them promotes to the detailed layout.

### Modified-scenario duplication in the runner

**Finding (major):** `tests/features_runner.rs` only suppresses
`@givn.removed` titles; permanent and delta `@givn.modified` copies run side by
side, and the old bodies would fail after implementation.

**Disposition:** Resolved. The design records that modified permanent scenario
bodies are synchronized with their delta bodies as each task lands, matching
the established practice in `preserve-ctrl-w-requests-in-shell-config`.

### Purpose-readability and view assertions were not falsifiable

**Finding (major):** "marked and readable" could pass on the current dim/italic
purpose; "simple view" could pass on any render; the E2E toggle waited for a
label the old card also showed.

**Disposition:** Resolved. The purpose step must assert the cyan `↳` marker
(`38;5;81`), the `38;5;252` text style, the absence of the dim SGR, and the
row order below the stack. The simple-view step asserts
`ReviewPanelState.details == false` and the absence of the `Intent` row. The
E2E toggle waits for `Intent` only after writing `?`, which the simple view
never renders.

### Hidden-window marker collided with the undecomposed marker

**Finding (minor):** window edges and undecomposed stages both used the amber
`…`.

**Disposition:** Resolved. Hidden windows use a dim `⋮`; the amber `…` stays
reserved for an undecomposed stage; an over-long single stage is truncated with
an unstyled `…`. A new scenario covers the truncated stage.

### Wide characters could reintroduce terminal artifacts

**Finding (major):** `visible_len` counts codepoints, and the stack renders far
more untrusted text on exact-width rows; wide CJK/emoji would break padding and
the move-up/clear cycle.

**Disposition:** Resolved. `unicode-width = "0.2"` becomes a direct dependency
(resolved from `Cargo.lock` to `0.2.2`, already transitive via ratatui) and
`visible_len` uses display width. A unit test pins wide-character padding.

### Flag-only path had onboarding and failure-behavior gaps

**Finding (blocker):** a flag-only switch on a clean machine would have written
a skeleton config and skipped first-run quicksetup; stdin reuse and persist
failure behavior were undefined.

**Disposition:** Resolved. The flag-only branch reads stdin once; a non-empty
buffer becomes the question and the invocation continues. It persists only when
a configuration file already exists, prints the applied state to stderr, exits
0, and exits non-zero with the error when persistence fails. The clean machine
keeps its onboarding path.

### `D` semantics and view availability

**Finding (major):** the design allowed `D` in both views while the glossary
listed only four simple decisions; `-x` disable behavior was unspecified.

**Disposition:** Resolved. `D` is accepted in both views but only advertised in
the detailed hints; the glossary now states that edit, reject, and disable stay
available while the simple presentation stays minimal. Under eligible `-x`, a
disable releases the candidate to stdout without executing it; the Failure
Outcomes table records this and the ADR-0015 amendment carries it.

### ADR amendment was recorded in the register but not the ADR body

**Finding (blocker):** the verdict routed `AMEND_ADR → ADR-0015`, but only the
chapter-09 summary was edited.

**Disposition:** Resolved. A "Permanent-disable amendment" section and a "Bad"
consequence were added to
`docs/adr/0015-synchronous-stream-callback-and-completion-boundary.md`; chapter
09 keeps the register entry and its summary; chapter 11 records R-078.

### Architecture-impact list disagreed with the arc42 delta

**Finding (blocker):** `design.md` claimed chapters 04/06/09/11 were unchanged
while `arc42.md` and the working tree changed them.

**Disposition:** Resolved. The design's Architecture Impact list now matches
`arc42.md`: chapters 03, 04, 05, 06, 09, 10, 11, and 12 affected; 01, 02, 07,
08 unchanged.

### Completeness gaps

**Finding (major):** the matrix omitted the `shell-completions` inventory row;
missing scenarios covered single-stage truncation, persist failure, and the
empty-model fallback.

**Disposition:** Resolved. The matrix gains the completions row with its real
subprocess mechanism. A truncated-stage scenario was added. Disable persist
failure and the empty-model fallback stay explicit design decisions (the
persist path is already owned by ADR-0024's atomic write and its config tests;
the empty-model frame fallback is a one-line render guard) and are recorded in
Failure Outcomes as deliberate non-goals for new Gherkin scenarios.

### Provider separator scan and tri-state review reality

**Finding (minor):** `following_separator` could misread a quoted operator; and
the proposal's visual claims were unasserted.

**Disposition:** Quoted operators cannot appear in an accepted provider gap
because `provider_stage_split` rejects any gap containing anything but
whitespace and separators; the design records this. Separator color, arrow, and
purpose marker are asserted through the exact SGR sequences.

## E2E Fidelity And Interaction Coverage

The 12 inventory rows in the change-side `usecase.md` each map to one `@e2e`
scenario in the design matrix, including the existing completions row. New
rows use real interfaces only: the Bash widget and watn CLI through the
portable-PTY harness, and real subprocess invocations for the flag-only
switch. No `@e2e` tag was removed; the derive tooling and mock provider are the
existing local digital twins.

## ADR Qualification

One `AMEND_ADR` verdict with all mandatory gates passing routes to ADR-0015;
the amendment is applied to the ADR body, the chapter-09 register keeps the
entry, and chapter 11 records the consequence. The view presentation and the
flag-only switch are `NOT_QUALIFIED` with this design as the single canonical
artifact; no new MADR is created.

## Arc42 Independent Walk

| # | Independent result | Disposition |
|---:|---|---|
| 1 | No | Goals and stakeholders unchanged. |
| 2 | No | No new constraint; the surface stays inline. |
| 3 | Yes | Developer keys, flag-only switch, command-output release. |
| 4 | Yes | Review strategy names both views and the disable release. |
| 5 | Yes | Review surface and renderer responsibilities updated. |
| 6 | Yes | Review narrative, lifecycle, and routing updated. |
| 7 | No | No deployment change. |
| 8 | No | No new crosscutting concept. |
| 9 | Yes | ADR-0015 amended (body + register + summary). |
| 10 | Yes | QS-067, QS-070, QS-073 updated. |
| 11 | Yes | R-073 wording and new R-078. |
| 12 | Yes | Five new terms and three updated rows. |

All 12 chapter files exist with real content; no ASCII-art diagram was found
(box-drawing scan clean), and mermaid remains the only diagram format.

## Ubiquitous Language

The glossary now holds `Simple review view`, `Detailed review view`, `Review
view toggle`, `Stage stack`, and `Model short name`, and updates `Review
decision`, `Review outcome`, and `Command-output channel`. Proposal, specs,
design, and `usecase.md` use those terms consistently; no new term is left
undefined, and no Persona was invented or promoted.

## Hardening Receipt

- `design.md`: input promotion, explain/sub-mode rendering, `⋮`/`…` marker
  split, unicode-width decision, flag-only semantics, `-x` disable outcome,
  synchronized modified scenarios, completions matrix row, corrected chapter
  list, third ADR verdict.
- feature delta: one removed; nine added (two E2E) and six modified (one E2E)
  scenarios; every new or modified scenario carries `@wip`.
- `usecase.md` (change side): main-flow, rules, and examples wording for the
  two views; three inventory rows.
- `docs/adr/0015-...md`: permanent-disable amendment and consequence.
- Arc42 chapters 03, 04, 05, 06, 09, 10, 11, 12 updated.
- `givn lint --change refine-review-panel-views`: exit 0; the 15 findings are
  `@wip` notices plus two advisory subset notices dispositioned here.
- `tasks.md`: not written.

## Status

DESIGN-REVIEW: PASS
