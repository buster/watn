# Design Review: robust-review-response-recovery

## Method

A fresh-context grilling subagent read the proposal, both delta specs, the
permanent use-case and capability specs, the design, the arc42 update, and the
relevant production and test code, then returned a ranked question list plus
resolved findings. The orchestrator hardened the artifacts with every resolved
finding and asked the operator the two decisions that change cost or privacy.

## Resolved findings and fixes

| # | Finding | Fix applied |
|---|---|---|
| R1 | `watn explain -v` does not parse: `-v` is a root-only flag and the `Explain` subcommand has no verbose field. | Added `Explain::verbose` to the design's data-model and primitives; `run_explain_command` accepts the root or subcommand flag. |
| R2 | The in-process review harness has no stderr channel, so the verbose/capture/warning scenarios could not assert their claims, and a second `stderr should contain` step would be an ambiguous match. | Decided the diagnostics scenarios drive the real binary in a `portable-pty` session and assert the merged transcript (as the explain failure scenarios already do); recorded in the design. |
| R3 | The unescaped-quotation-mark fixture could not satisfy its own Then: a command containing the unescaped quotes cannot display as `df -h`. | Reworded the scenario to "values contain unescaped quotation marks" and moved the quotes into a stage purpose; the command stays `df -h`. |
| R4 | The scanner wording allowed end-of-input as a delimiter, which would recover a command cut off mid-value and contradict the "cut off inside the command releases nothing" scenario. | The design now requires a following review field or a closing quote before end-of-input; an unterminated value recovers nothing. |
| R5 | The mismatched-stage fixture uses `purpose_status: "incomplete"`, so validation fails as invalid JSON and the asserted stage-mismatch reason is unreachable. | The shared fixture changes to `"purpose_status": "ready"`; the permanent mismatch scenario stays green, and the fallback path carries the validation error into `unavailable_reason`. |
| R6 | Repair alone does not make a line-broken command valid: validation compares the raw response command with the normalized candidate command. | The design normalizes before comparison so a repaired response reaches `Ready` through strict validation. |
| R7 | `finish_reason` was only claimed for `parse_generated_candidate`, which the initial review path does not call. | The design threads the accepted response's `finish_reason` into the initial review parse as well as regeneration. |
| R8 | `-v` cannot be passed through the Ctrl-W widget, so "I invoke Ctrl-W with verbose output" is not a real action. | The verbose scenario now uses the direct review invocation in an eligible terminal. |
| R9 | The Purpose reason mapping was not enumerated while the spec asserts exact sentences. | Added the fixed `card_reason()` mapping table to the design. |
| R10 | The proposal promises tabs/carriage returns, the not-valid-JSON reason, and the overwrite contract, but the first spec draft covered none. | Scenario 1 now covers literal line breaks and tabs; the unescaped-quotes scenario asserts the not-valid-JSON reason; a new scenario proves a usable response does not overwrite the captured unusable response. |
| R11 | The Interaction Coverage Matrix listed 3 of the 12 inventory entries. | The matrix now lists every entry for both touched capabilities with its `@e2e` title and driving mechanism, plus a normalization note. |
| R12 | The review-path reason and capture had no use-case contract element. | Added the review extension to the use-case delta and to the design traceability table. |
| R13 | The review-shape classifier had no stated rules while a guarded behavior depends on it. | The design states the quoted-key signal and the command-only counterexample. |
| R14 | The standard SSE mock helper escapes only quotes, so a literal newline fixture would split the event. | Recorded as an anticipated obstacle: the new fixtures JSON-serialize their content. |
| R15 | The runtime view captured only malformed/truncated payloads, not unmatched ones. | Updated the sequence diagram and prose to capture every unusable response, including no-recoverable-command and mismatch. |
| R16 | After implementation, `design.md` was edited (still uncommitted) to document the as-built card wording: one `Command mismatch` row, `card_reason()` owning both card paths, `explain_reason()` keeping the stderr wording. The committed design still carried two path-split rows and an unimplemented command-only row while the code had the single wording since S2 (`9fb145f`), so design and code had drifted without a re-review. | Re-reviewed in fresh context (see below); the table now matches `card_reason()` arm-for-arm and gains the missing empty-command reason. |
| R17 | The re-review found the new command-only paragraph wrong on two counts: on the explain path plain text maps to `the provider response was not valid JSON` rather than a bare label, and on the review path command-only text is still captured as an unusable provider response. | Paragraph scoped per path and reworded to the glossary term; no code change. |

## Operator decisions

1. **Completion cap** (asked and answered): review and explanation requests
   raise `max_tokens` from the provider default 1024 to 4096. Recorded in the
   design; the cost consequence is R-091.
2. **State-file privacy** (asked and answered at proposal time): keep the
   overwritten capture file; the design adds Unix mode `0600` and no history.
   Recorded as R-089.

No mandatory check is deferred. There is no deferral to record.

## Branch verdicts

| Branch | Verdict |
|---|---|
| Scope | CLEAN after fixes: every proposal commitment now has a scenario or an explicit design statement; nothing extra. |
| Tech choices | CLEAN: repair/scan/refuse stays in `src/review/response.rs` with no new dependency; `finish_reason` is a provider-level signal with a unit-testable parser. |
| Missing scenarios | CLEAN after adding the tab coverage, the third reason assertion, and the no-overwrite negative. |
| Testability | CLEAN after R2/R5/R9: card scenarios run in-process without a provider; diagnostics scenarios drive the real binary through the PTY; every Then asserts concrete text or file content. |
| Risk | Most likely failure is the real-binary diagnostics wiring (flag, capture path, post-close printing); mitigation is the PTY harness, the isolated `XDG_STATE_HOME`, and the unwritable-directory scenario. |
| Use-case / Persona | CLEAN: `use-shell` owns both capabilities; `terminal-developer--interactive` is confirmed and stays a review lens; actors are unchanged. |
| Visual Design Contract | CLEAN: terminal interface, one changed purpose row, no new screen or control; classification as terminal (not Web UI) is correct. |
| Interface classification | CLEAN: the interface is a CLI and a terminal card, not HTML/CSS/JS; the CLI subprocess is the real driver. |
| Deferrals | CLEAN: none. |
| Interaction coverage | CLEAN after R11: all 12 entries have matrix rows with non-empty CLI driving mechanisms. |
| ADR qualification | CLEAN: independently re-derived as `NOT_QUALIFIED`; 26 ADR records searched; canonical destination is this design plus the Gherkin contract. |
| arc42 | CLEAN after R15: independently re-derived 12-row table matches `arc42.md`; every Yes chapter carries matching content; all 12 chapter files exist with real content and no ASCII-art diagrams; no missing MADR. |
| Ubiquitous language | CLEAN after terminology alignment on "unusable-response state file"; the six new terms and three refinements are recorded in the glossary and used consistently. |
| Exact reason wording | CLEAN after R5/R9: the spec sentences match the glossary and the fixed `card_reason()` mapping. |
| Recovery fixture | CLEAN after R3/R4: the scanner recovers `df -h` from values with unescaped quotes and refuses an unterminated command. |

## Lint findings accepted

`givn lint --change robust-review-response-recovery` exits 2 with advisory
findings only:

- The two `releases nothing` scenarios share a shape but have distinct
  preconditions (valid payload without a command vs. payload cut off inside the
  command). Both are retained; the preconditions are the contract.
- The `[SUBST]` findings compare delta scenarios against permanent scenarios
  that share harness Given/Then text; the observables differ (recovery and
  reason naming). No consolidation is warranted inside this change.
- The verbose scenario shape-matches the regeneration scenario because both
  drive the real binary; their observables differ.

## Re-review after implementation

`design.md` was edited after the original sign-off to document the as-built
reason wording (R16). Per the review gate, the changed decision was re-reviewed
in a fresh context: the subagent read the updated design, the original review
record, both delta specs, the use-case delta, the permanent `use-shell` specs,
the implementation (`src/review/response.rs`, `src/review/card.rs`,
`src/review/session.rs`, `src/main.rs`, `src/review/diagnostics.rs`,
`src/config/mod.rs`, `src/provider/*`, `src/output/render.rs`), the glossary,
and the arc42 update.

Verified in the re-review:

- Every row of the updated Purpose-reason table matches a `card_reason()` arm
  string-for-string; `card.rs` renders `card_reason()` for both the review and
  explanation cards.
- No delta or permanent scenario asserts any superseded wording; the explain
  stderr diagnostic still uses `explain_reason()`.
- The glossary's Purpose-reason examples are a subset of `card_reason()`; no
  stale wording exists under `docs/`.
- No structural consequence: ADR `NOT_QUALIFIED` stands and the arc42 chapter
  table is unchanged.

Findings R17 fixed in `design.md` (path-scoped wording, capture wording). No
operator decision changed, no mandatory check is deferred, and `givn lint`
still exits 0.

## Hardening applied

- `specs/use-shell/interactive-shell-shortcut.feature`: 9 scenarios after the
  rewrites and the added no-overwrite scenario.
- `specs/use-shell/explain-command.feature`: 5 scenarios, terminology aligned.
- `specs/use-shell/usecase.md`: review-path extension added.
- `design.md`: completion cap, state-file mode, `Explain::verbose`,
  normalization-before-validation, scanner delimiter rule, classifier rule,
  `finish_reason` threading, Purpose reason table, step responsibilities,
  complete Interaction Coverage Matrix, resolved open questions.
- `arc42.md` and `docs/arc42/06-runtime-view.md`: capture for every unusable
  response.

DESIGN-REVIEW: PASS
