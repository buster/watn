# Design Review: ratatui-model-picker

## Grilling Results

A grilling subagent (fresh context) read the proposal, spec, design, arc42
assessment, the permanent specs, and the implementation, and produced a ranked
findings list. Each finding below was resolved during hardening without
needing a user decision; the resolutions are recorded here and in
`QUESTIONS.md`.

### Scope
The spec covers every proposal bullet: guided three-level sequence, back
navigation, per-level model + reasoning strength (off/low/medium/high),
persistence, arrow + page navigation, visible filter, per-word
order-independent matching, metadata display, empty state, local fallback,
and reasoning taking effect on requests. No extra scope. "Done" is
unambiguous.

### Tech Choices
ratatui + crossterm is the right fit and is explicitly required by the
proposal ("built with the ratatui crate"). The config `TierReasoning` table
and main.rs resolution are the minimal persistence stack.

**Finding resolved (F3):** the per-word predicate was originally scoped to
`local_filter` only, but typed filters travel through the remote
`search_models` path. Design now specifies a single shared `word_matches`
predicate applied in both `search_models` (remote) and `local_filter`
(fallback), backward compatible with all single-word autosuggest scenarios.

**Finding resolved (F8):** focus contract unspecified. Design now states:
focus defaults to the model list per level (so "type filter + Enter selects
the top suggestion", preserving the existing autosuggest e2e contract); Tab
cycles focus to the reasoning selector and back. Page size is fixed at
`PAGE_SIZE = 10` for deterministic tests.

### Missing Scenarios
No observable behaviour from the proposal lacks a scenario. Debounce timing
(~200 ms) is implementation behaviour rendered as "responsive"; the spec
asserts the observable outcome (filter updates), not the wall-clock timing.
A persistence round-trip (dialog → config → request) is covered by e2e
scenario 1 (config file content) plus e2e scenario 5 (config → request),
which together form the round trip.

### Testability
**BLOCKER resolved (F1):** the existing `the API request should include
reasoning with effort "X"` step only asserts `mock.hits() > 0` — vacuous.
httpmock 0.7 (verified in Cargo.lock) exposes no client-side request-body
capture, so the assertion mechanism is changed to httpmock request
body-matchers: the Given registers the chat mock with
`when.body_contains("\"reasoning_effort\":\"Y\"")`, so a request without that
body fails to match → non-zero exit; the Then asserts `exit status 0` as the
primary real-interface proof plus `mock.hits() > 0`. The negative variant
uses a first-registered 400 mock with `.body_contains("\"reasoning_effort\"")`
and a fallback path-only 200 mock; it asserts `exit status 0` and zero hits
on the 400 mock. Without this, the new reasoning scenarios could not fail in
RED. design.md was updated to the body-matcher mechanism (no
`last_request_body`; the archived change's vacuous step is superseded).

**BLOCKER resolved (F2):** the local-fallback scenario passed an empty
`all_models` to `execute_search`, so "the suggestions include gpt-4o" could
never pass. The Given now loads the local models into the world and the When
passes them through.

**MAJOR resolved (F4):** the browse scenario was self-referential (no target
model named, unpinned list). The mock now returns a pinned 40-entry list
`model-01..model-40`; down + one page (PAGE_SIZE=10) deterministically lands
on `model-12`, asserted exactly.

**MAJOR resolved (F5):** scenarios 1 and 4 used colliding model sets where
a filter matched two models (e.g. "gpt-4o" matches both `gpt-4o-mini` and
`gpt-4o`). Rewritten with unique-prefix model sets so the typed filter
uniquely identifies the target.

**MAJOR resolved (F6):** the design reused legacy `choose "M" for the normal
tier` steps that hardcode typing "o3". New parameterized steps type the
chosen model id.

**MAJOR resolved (F7):** the metadata scenario was non-e2e but asserted
dialog (PTY) output through placeholder steps. Rewritten as a formatting
scenario (`I format the model list for display`) driving the real display
formatter, with concrete price presence/absence assertions.

### E2E fidelity
Interface type is CLI; driving mechanism is a PTY subprocess with raw escape
sequences. Five distinct interactions, exactly one `@e2e` per interaction —
no over/under-covering. The metadata display is an attribute of the
configure interaction's rendered dialog, covered by a non-@e2e formatting
scenario (matching the existing autosuggest precedent).

### Interaction Coverage
All 5 inventory entries map to matrix rows with non-empty driving mechanisms
(after F1/F4 fixes). Verified cross-reference between the `.feature` comment
and the design.md matrix.

### Risk
Most likely failure: breaking the pre-existing PTY autosuggest e2e scenario
(which types a query then Enter expecting top-suggestion selection) if the
dialog's default focus were the filter input. Mitigated by the explicit
focus contract (F8): default focus is the model list, Enter selects the top
suggestion. Existing non-TTY e2e goes through `select_model_non_interactive`
and is unaffected.

### Architecture documentation (arc42)
Independently walked all 12 rows: my Yes/No matches arc42.md. All Yes-row
chapters (03, 04, 05, 06, 08, 09, 11, 12) contain decision-specific content,
no placeholder text, and no ASCII-art diagrams (all Mermaid). Chapter 09
gains ADR-0010; chapter 11 reflects its consequences (R-010, TD-004).

**Finding resolved (F9):** arc42.md marked chapter 03 as affected but the
chapter was not updated. The context diagram and interfaces tables in chapter
03 now show the dialog's keyboard input and the config write of tier +
reasoning assignment.

## Hardening

Applied during this review:
- Spec: rewritten model sets (F5), pinned browse list + exact assertion
  (F4), local-fallback Given loads models (F2), parameterized tier-choice
  wording (F6), metadata scenario as a formatting scenario (F7).
- design.md: shared `word_matches` predicate across remote + local paths (F3),
  focus contract + PAGE_SIZE=10 (F8), body-capture in the chat mock + body
  assertions (F1), matrix rows for all 5 inventory entries.
- arc42 chapter 03: dialog keyboard input + config write in context (F9).
- `givn lint --change ratatui-model-picker`: clean (exit 0).

No `@e2e` tags removed. All 5 `@e2e` scenarios retain their tags; the e2e
runner is configured in `givn/commands.yaml`.

## Sign-off

DESIGN-REVIEW: PASS

All questions resolved. Findings F1–F9 remediated; no open blockers.