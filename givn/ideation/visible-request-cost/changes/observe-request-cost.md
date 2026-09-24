# Change seed: observe-request-cost

Status: proposed by observe-request-cost

- Use-case IDs: observe-request-cost
- Topic: visible-request-cost
- Personas: terminal-developer--interactive — the developer who pays for their
  own requests; the cost stance dated 2026-09-24 is the review lens.
- Suggested start: first; no dependencies on other seeds. It depends only on
  the existing corpus: the review surface (`use-shell`), the explanation card
  (`explain-command`), and the cost computation in the asking capability.
- Handoff: published 2026-09-24; the topic document is
  `docs/ideation/visible-request-cost.md`. No change was started.

## The deliverable

The surface a request produced shows, in cents, the amount the provider billed
for that request: the review pane for an interactive request or a Ctrl-W
candidate, and the explanation card for `watn explain`. The amount appears with
the model the surface names, is replaced when a rejected candidate is
regenerated, and disappears with the surface. It gates no decision and never
reaches stdout.

One refactor is part of the seed: the amount computation exists as two inline
copies (`src/main.rs:629`, `src/main.rs:1311`) and cannot distinguish "no usage
report" from "reported zero" — the surface needs that presence tracked. The
explanation path (`src/main.rs:833`) computes no amount at all today, so the
card's amount is new computation; if the change must be cut smaller, that half
splits off cleanly and the pane half stands alone.

## Accepted decisions

### The number is the already-billed cost of the current request

- Context: three different amounts could be meant — the cost already incurred
  for the candidate on screen, the price of the decisions still open, or the
  running total of the review. The pane shows a number at the moment those
  decisions are made, so the choice decides whether this is a record of spend or
  a pricing aid.
- Options:
  1. **Already-billed cost of the current request** — what the provider charged
     for the request that produced the candidate. In practice: opening a
     candidate shows `◆ claude-haiku-4.5 · 0.06 ¢`, computed from the tokens
     the provider just reported. Gain: exact and already known when the surface
     opens. Cost: the money is spent by the time it is visible, so it cannot
     help economize.
  2. **Price of the decisions still open** — the alternatives priced before they
     are taken. In practice: the model chooser shows `1 gemini-3.7-flash
     ~0.02 ¢` beside `2 claude-opus-4.1 ~1.9 ¢`. Gain: the expensive option is
     visibly expensive in time to decline it. Cost: token counts do not exist
     yet and the chosen model changes them, so only a per-million rate could be
     shown, not an amount.
  3. **Running total of the review** — every request the review caused, summed.
     In practice: after a rejection and an escalation the footer reads
     `this review: 0.21 ¢`. Gain: it prices the exploration loop. Cost: needs a
     session accumulator and a reset boundary, and says nothing about future
     spend.
- Chosen: **already-billed cost of the current request** because it is the only
  exact, already-incurred amount of the three and is computable from what Watn
  already holds when the surface opens. Rejected: **price of the decisions still
  open** because the honest up-front figure is a rate, not an amount in cents;
  **running total of the review** because it requires accumulation Watn does not
  have and answers a different question.

### A missing price is silent

- Context: an amount exists only when a pricing entry matches the model the
  provider reported. A hand-configured model, or a provider that normalizes its
  model id, leaves none. In the surface that silence would sit next to a model
  label, where "free" and "unpriceable" look identical.
- Options:
  1. **The model label stands alone** — `◆ claude-haiku-4.5`, no money, nothing
     in its place. Gain: no new state or vocabulary, consistent with the stderr
     line, which already omits it silently. Cost: the absence carries no
     information, so a missing price stays invisible.
  2. **An explicit marker** — `◆ claude-haiku-4.5 · price unknown`. Gain: the
     absence becomes information the developer can act on with `watn models`.
     Cost: a new state and term in a deliberately reduced view.
  3. **Fall back to the requested model's recorded price** — `~0.07 ¢` from the
     entry keyed by the id the request was sent with. Gain: a number in nearly
     every review. Cost: an assumption presented as a billed amount.
- Chosen: **the model label stands alone** because the silence matches what
  Watn already does everywhere this amount appears, and a new state would have
  to be kept in the smallest presentation for a configuration gap rather than a
  review decision. Rejected: **an explicit marker** because it adds a state and
  a term without changing any decision in the surface; **the fallback price**
  because it would present an estimate as a billed amount.

### The amount survives a narrow header

- Context: the amount shares the header's right segment with the model label,
  which the renderer truncates at the tail. At 40 columns the segment holds 19
  characters and `◆ deepseek-v4-flash-latest · 0.02 ¢` needs 35.
- Options:
  1. **The amount survives** — the label is shortened to what the amount leaves:
     `◆ deepsee… · 0.02 ¢`. Gain: the number is always visible while it can be.
     Cost: on a narrow terminal the model name is cut.
  2. **The label survives** — the amount is dropped first. Gain: today's header
     untouched. Cost: the feature silently disappears on narrow terminals and
     looks exactly like an unpriced model.
  3. **The amount falls back to its own row**. Gain: visible at any width.
     Cost: a second layout state competing for the row budget.
- Chosen: **the amount survives** because the topic exists to make money
  visible, and a dropped amount is indistinguishable from the silent absence.
  Rejected: **the label survives** because it silently removes the feature;
  **its own row** because it adds a layout state and competes with the stage
  stack. The correction recorded with the decision: this requires the header to
  reserve the amount's width before shortening the label — the current tail
  truncation would drop the amount instead.

### The billed amount is shown under the requested label

- Context: the label names the model the request was sent with; the amount is
  keyed on the model the provider reports. A routing suffix or a renamed
  snapshot can make them differ.
- Options:
  1. **The billed amount under the requested label** — the number is what the
     provider actually charged, wherever it routed. Gain: nothing known is
     hidden. Cost: the header is quietly about two models.
  2. **Only when both names agree**. Gain: nothing misleading. Cost: a real
     amount is hidden by a naming difference, the hardest case to diagnose.
  3. **Label the amount with the provider's model**. Gain: label and amount
     always belong together. Cost: the label changes under the developer.
- Chosen: **the billed amount under the requested label** because it is the
  truth of what was billed, and hiding it is the silence already reserved for
  the unpriced case. Rejected: **only when both agree** because it hides known
  money; **relabelling** because the model the developer chose would stop being
  the model they see.

### The explanation card carries its request's amount

- Context: `watn explain` bills an explanation request, and its card is the
  surface that request produced; the path today computes and prints no amount at
  all.
- Options:
  1. **Show it on the card.** Gain: every surface a request produced reports its
     cost. Cost: new computation on a path that never had any.
  2. **Leave the card out.** Gain: smaller change. Cost: the one surface that
     explains a command stays money-blind even though it is billed.
- Chosen: **show it on the card** because the guarantee is about the surface a
  request produced, not about the review pane. Rejected: **leave the card out**
  because the billed explanation request would remain invisible.

### A reported zero is shown; an unreported usage is not

- Context: the existing computation folds "no usage report" and "reported zero
  tokens" into the same `Some(0.0)`, which the stderr line prints as `$0.0000`.
- Options:
  1. **Distinguish them.** Gain: an unaccounted request never looks free, and an
     accounted zero is reported as what it is. Cost: the shared computation must
     track usage presence separately.
  2. **Reuse the existing value.** Gain: no refactor. Cost: an unaccounted
     request would display `0.00 ¢`, which reads as "this was free".
- Chosen: **distinguish them** because a fabricated zero is the one thing the
  topic forbids. Rejected: **reuse the existing value** because it presents an
  unaccounted request as free.

## Open questions

### How precise is the cents display?

One cent is $0.01 and a typical request lands between $0.0001 and $0.02 — 0.01 ¢
to 2 ¢. Whole cents would render nearly every request as `0 ¢`. Two decimals
(`0.02 ¢`) read cleanly and cover the common range but collapse anything under
0.005 ¢ to `0.00 ¢`; three or four decimals (`0.0234 ¢`) keep the cheapest
requests distinguishable at the cost of a longer label in a header segment the
model label also needs. The rules fix the unit, cents, and forbid rendering a
billed request as zero; the decimal count and the rounding rule are Design's.
Example: 1200 prompt and 90 completion tokens at 0.15 and 0.60 per million is
$0.000234 — `0.02 ¢` at two decimals, `0.0234 ¢` at four.
