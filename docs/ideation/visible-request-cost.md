# Ideation: visible-request-cost

The developer pays for every request Watn makes. The amount was visible only
after the fact, on one stderr line, on some paths — and never while the review
decisions that spend money were being made. This topic makes the money visible
where those decisions happen: in cents, in the surface the request produced.

Topic state lives in `givn/ideation/visible-request-cost/`. This document is the
readable record.

## The problem, corrected

The request that opened this topic said "we read prices, but we do nothing with
them". Prices were in fact used: a cost is computed from the response's token
usage and the recorded per-million price and printed on stderr — but only on the
direct path (`src/main.rs:655`) and the review-accepted path
(`src/main.rs:1336`). `watn explain` (`src/main.rs:833`), every rejected or
regenerated request, a cancelled review, and an in-pane permanent disable
printed no money at all. And the review surface could not show it in any case:
its context carries intent, tier, provider, and model, and nothing else
(`src/review/panel.rs:72`).

So the gap was not unused prices. It was that the amount arrives after the money
is spent, on a line that scrolls away, and is missing entirely on the paths where
a developer explores alternatives.

## What the developer gets

The request's already-billed amount, in cents, at the model label of the surface
that request produced — the review pane for an interactive request or a Ctrl-W
candidate, the explanation card for `watn explain`. It is replaced when a
rejected candidate is regenerated with another model, and it disappears with the
surface. It gates no decision: accept, edit, cancel, reject, regenerate,
escalate, and disable behave exactly as before.

## The boundary

One use case, observing, with one capability:

- **observe-request-cost** (`!`) — the developer's goal of knowing what the
  request they just made cost.
- Capability: **visible-request-amount** (ids provisional).

It is not a capability of the review goal. The amount deliberately informs no
accept-or-regenerate decision, which was the rejected reading of the original
request, so it advances no existing goal's success guarantee. Channel is not the
argument: the pane and the card are two presentations of one outcome.

Sizing was verified against the eight falsifiable rules, and both boundary
reviews' findings are applied. The elaborated contract is
`givn/ideation/visible-request-cost/use-cases/observe-request-cost.md`.

## The confirmed rules

- The amount is the value the existing post-request cost computation produces:
  the usage the provider reported, paired with the recorded per-million price of
  the model the provider reported, expressed in cents.
- It is never estimated, assumed, or substituted — no fallback price, no rate,
  no session total, no comparison.
- It is shown only when the response carried a usage report and a recorded price
  matches the model the provider reported. Otherwise the model label stands
  alone.
- A request the provider did not account for is never shown as zero; a request
  it did account for shows what it was billed, including a genuine zero.
- No amount appears without the model it belongs to.
- It appears at the model label in the header, in every view, for as long as it
  fits: when the header cannot hold both, the model label is shortened first.
- It is transient, rendered on the controlling-terminal channel, and never
  becomes command output.
- A surface whose request never completed shows the existing failure contract
  and no amount; a completed but unusable response still shows its billed
  amount.

## Decisions and what was rejected

| Decision | Chosen | Rejected |
|---|---|---|
| What the number is | The already-billed cost of the current request | Pricing the decisions still open, because the honest up-front figure is a rate and not an amount; a running review total, because it needs accumulation Watn does not have and answers a different question |
| A missing recorded price | The model label stands alone; silence | A `price unknown` marker, because it adds a state and a term without changing a decision; a fallback to the requested model's price, because it presents an assumption as a billed amount |
| A header too narrow for both | The amount keeps its width; the label is shortened first | Letting the label win, because the feature would silently vanish on narrow terminals; giving the amount its own row, because it competes for the row budget |
| Provider reports another model | The billed amount, under the requested label | Showing it only when the names agree, because it hides known money; relabelling with the provider's model, because the label would change under the developer |
| The explanation card | It carries its request's amount | Leaving it money-blind while its request is billed |
| No usage reported | Distinguish it from a reported zero | Reusing the existing value, which would show an unaccounted request as free |

## What the change must amend

- The review goal's simple-view rule — the simple view "names only the model,
  stacks the command-flow stages with their separators, and shows the selected
  stage's purpose" (`givn/specs/use-shell/usecase.md:66`), repeated in the
  glossary's `Simple review view` and `Model short name`. It must admit the
  amount at the model label.
- Nothing else: the amount computation, the response-model keying, and the
  stderr metadata line stay with the capabilities that own them (`ask`,
  `incremental-sse-rendering`, `config`).

The change also needs one refactor: the computation exists as two inline copies
(`src/main.rs:629`, `:1311`) that cannot distinguish "no usage report" from
"reported zero", and the explanation path computes no amount at all — its card's
amount is new machinery.

## Durable records

- **Term proposed and promoted:** `Amount` in `docs/arc42/12-glossary.md`.
- **Term reported and rejected:** the persona's `Proposed action` as a synonym
  for the user's "actions I take" — the specs use `Candidate` and `Review
  decision`, and the persona file was the only place that word lived.
- **Persona promotion:** the dated cost stance was appended to the durable
  `givn/personas/terminal-developer--interactive.md`; the topic-local file stays
  as the historical record. No new persona was created.
- **Change seed:** `givn/ideation/visible-request-cost/changes/observe-request-cost.md`,
  status ready. It is input to `/givn-propose`; no change was started.

## Still open

The precision of the cents display: two decimals read cleanly but collapse the
cheapest requests to `0.00 ¢`, more decimals keep them distinguishable at the
cost of a longer label. The rules fix cents as the unit and forbid showing a
billed request as zero; the decimal count and rounding rule are Design's. One
scenario skeleton carries the qualifier.

## Recorded tensions

- The persona's cost stance says a silently absent number is not actionable,
  while the confirmed decision keeps the unpriced case silent. The user
  arbitrated; the tension stays on record.
- The ideation method cites a durable `docs/use-case-model.md` with the worked
  sizing example. This repository has none; the eight sizing rules were applied
  from the method directly.
