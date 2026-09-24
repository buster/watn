# Handoff: visible-request-cost

Status: complete 2026-09-24, confirmed by the user. Handoff started no change.

## Published record

`docs/ideation/visible-request-cost.md` — the topic document, written for humans
and readable start to finish. Note for the reader: the sibling topic's document
sits at `docs/ideation-terminal-wow-factor.md`, an older single-file convention;
the current ideation method publishes to `docs/ideation/<topic-id>.md`.

## Use-case to permanent mapping

| Ideation use case | Permanent destination | Note |
|---|---|---|
| observe-request-cost (`!`) | New leaf document `givn/specs/observe-request-cost/` | Owns the capability `visible-request-amount`. Not mapped into an existing goal: the amount informs no accept-or-regenerate decision, so it advances no existing success guarantee. |

Amendments the later change must apply — named here so the change does not
silently rewrite them:

- `givn/specs/use-shell/usecase.md:66` — the simple view's "names only the
  model, stacks the command-flow stages with their separators, and shows the
  selected stage's purpose" must admit the amount at the model label.
- `docs/arc42/12-glossary.md` — the `Simple review view` and `Model short name`
  entries carry the same rule.

Nothing else changes: the amount computation, the response-model keying, and
the stderr metadata line stay with `ask`, `incremental-sse-rendering`, and
`config`.

## Durable terms

- Promoted: `Amount` — added to `docs/arc42/12-glossary.md`.
- Reported and rejected: `Proposed action` as a synonym for the user's "actions
  I take" — the specs use `Candidate` and `Review decision`.

## Persona promotion

- Appended to `givn/personas/terminal-developer--interactive.md`: the cost
  stance dated 2026-09-24, as a decision-record entry. No new persona was
  created; the topic-local file `personas/terminal-developer--interactive.md`
  remains the historical record.

## Change seeds

| Seed | Status | Depends on | Suggested start |
|---|---|---|---|
| `changes/observe-request-cost.md` | ready | nothing; uses the existing review surface, explanation card, and cost computation | first |

The seed covers the whole guarantee: the pane and the explanation card show the
billed amount. If it must be cut smaller, the card half splits off cleanly and
the pane half stands alone.

## What was explicitly not started

- No proposal, specification, design, task list, or implementation. The seed is
  input to `/givn-propose`.
- No change to any specification under `givn/specs/`: the amendments above are
  recorded for the change that implements them.

## Open questions carried to the change

- The precision of the cents display: the unit (cents) is fixed and a billed
  request must never read as zero; the decimal count and the rounding rule are
  Design's. One scenario skeleton carries the qualifier.

## Gaps noticed for this repository

- The ideation method cites a durable `docs/use-case-model.md` holding the
  sizing rules with a worked example. This repository has no such file; the
  rules were applied from the method text directly.
