# Design review: observe-request-cost

## Method

Phase 1 grilling ran in a fresh context: a subagent with no part in writing the
proposal, the specs, or the design read all planning artifacts, the permanent
neighbours, the source, the canonical specs/design instructions, and the
glossary, then returned findings and a ranked question list. Phase 2 hardening
was applied by the orchestrator from the answers.

## Findings (settled by the grilling agent, with evidence)

| # | Finding | Disposition |
|---|---|---|
| 1 | Nine non-end-to-end scenarios used `When I ask interactively for …`, a step bound to the pseudo-terminal driver (`tests/steps/interactive_shell_shortcut_e2e_steps.rs:407`), contradicting the design's in-process step split | Fixed: the nine scenarios are rewritten to in-process steps; question 1 decided the driver |
| 2 | No scenario asserted the Amount in the detailed view, and `header_right`'s detailed branch renders tier/provider/model with no short name (`src/review/card.rs:265`) | Fixed: the design defines the detailed branch and a new scenario covers it; question 2 decided it |
| 3 | The permanent Ctrl-W scenario asserted the buffer *contains* the accepted candidate (`givn/specs/use-shell/interactive-shell-shortcut.feature:183`), so a leaked Amount would pass | Fixed: question 5 decided the strengthened `@givn.modified` scenario |
| 4 | The durable arc42 chapters and glossary described the Amount while the code and the permanent review-goal rule still described the old surface | Question 3 decided: the docs stay ahead, and the change's delta for the review goal converges them at archive |
| 5 | Chapters 04 and 10 still contradicted the amended glossary | Fixed: both are reconciled in this change |
| 6 | `arc42.md` recorded `lower_level_artifact: FAIL` for two candidates that name a canonical lower-level home | Fixed: both now read `PASS`, which is what makes them `NOT_QUALIFIED` with a canonical artifact rather than ADR candidates |
| 7 | The design named `Pricing` where the type is `ModelPricing` (`src/config/types.rs:218`) | Fixed |
| 8 | The visual contract claimed the detailed view and 40-column width as covered, while only the two end-to-end scenarios carry transcripts | Fixed: the contract now states exactly which states the end-to-end scenarios drive and which are covered in-process |
| 9 | The regeneration plumbing was unnamed, so a successful regeneration could leave the previous request's Amount behind (`src/review/panel.rs:217`) | Fixed: the design names the plumbing table, including the failure path that must preserve the candidate's own Amount |
| 10 | `run_explanation_card` never receives the response (`src/main.rs:726`, called at `:1018`) | Fixed: the design names the signature and call-site change and both outcome branches |
| 11 | `stderr` output stays byte-identical | Confirmed, no change needed |
| 12 | All five asserted amounts are arithmetically consistent with the decided format | Confirmed, no change needed |
| 13 | Inventory maps one-to-one onto the two end-to-end scenarios; no duplication of a permanent scenario | Confirmed, no change needed |
| 14 | Persona use is correct: review lens, not actor; no auto-promotion | Confirmed, no change needed |
| 15 | Interface classification is correct: terminal, with the provider already mocked | Confirmed, no change needed |
| 16 | ADR qualification: no candidate qualifies; no cheaper canonical home was missed | Confirmed, no change needed |
| 17 | D1 is resolved in the design; the design cited the seed reference for it | Confirmed, no change needed |

## Questions and answers

1. **Driver for the nine non-end-to-end scenarios** — answered: in-process. The
   scenarios are bound to in-process steps and reuse the existing review
   harness; the two `@e2e` scenarios remain the only pseudo-terminal drivers.
2. **Amount in the detailed view** — answered: yes, the detailed view carries it
   too, with its own label text and reservation rule, and a new scenario covers
   it.
3. **Durable docs ahead of the code** — answered: yes, the chapters and the
   glossary stay ahead; the change's delta for the review goal amends the
   contradicting rule when the change archives, and chapters 04 and 10 are
   reconciled now.
4. **The four scenarios that cannot fail in RED** — answered: keep them as
   recorded regression guards. `tasks.md` marks `A model with no recorded price
   leaves the model name without an amount`, `A request the provider did not
   account for shows no amount`, `A failed request shows no billed amount`, and
   `The billed amount never reaches command output` as already green and checks
   them off with the nearest scenario that does fail in RED. No RED evidence is
   fabricated for them.
5. **The permanent Ctrl-W scenario** — answered: strengthen it. `Developer
   accepts an explained candidate from Ctrl-W` is carried as a `@givn.modified`
   scenario in this change, asserting the Bash command line is exactly the
   accepted candidate and contains no billed amount.

## RED-test audit

Twelve in-process and two end-to-end scenarios: twelve can fail in RED against
today's code (the Amount is not rendered anywhere), two of those twelve are the
end-to-end ones, and four are recorded regression guards per question 4. The
audit table with per-scenario reasons is in the grilling report and reflected in
`tasks.md`.

## Deferrals

None. Nothing mandatory was deferred, and no `@e2e` tag was removed or weakened;
`verify.e2e_command` is configured (`./run-tests.sh --e2e`).

## Verification

`givn lint --change observe-request-cost` exits 2 (findings only), with `@wip`
markers on every unauthored scenario and shape/subset advisories that the review
step will re-examine. `givn spec route` reported no signal, so the capability
routing decision is recorded with its rationale in the proposal.

## Hardening notes after implementation started

- `BilledAmount::cents_text()` was refined while implementing S1: four decimals
  round a very small billed amount (one token at $0.02 per million is 0.000002
  cents) to `0`, which reads as free. The form now keeps one more decimal at a
  time, up to twelve, until a non-zero amount is visible; a genuine zero still
  renders as `0`. The decision (four decimals, trailing zeros trimmed, cents)
  is unchanged; only its guarantee is now true at every magnitude, and
  `design.md` records the refinement.

## Result

All questions resolved, findings applied to `design.md`, `specs/**`,
`arc42.md`, and the durable chapters. No open item blocks the task list.

DESIGN-REVIEW: PASS
