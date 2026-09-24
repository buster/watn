# Proposal: amount-from-requested-model

## Use-Case Context

- Use-case ID: observe-request-cost (permanent; this change amends its rules)
- Ideation topic: visible-request-cost (supersedes the Q2 rejection of the
  requested-model fallback)
- Seed path: none (follow-up change; the seed's status is
  `implemented in observe-request-cost`)
- Confirmed Personas: `terminal-developer--interactive` (cost stance,
  2026-09-24)

## Problem / Opportunity

The Amount is keyed on the model the provider reports for a response, while
prices are recorded under the identifier the developer configured. Those
identifiers routinely differ:

- A configuration written by quick setup or by hand carries an alias such as
  `~deepseek/deepseek-flash-latest`; the provider answers with its canonical id,
  `deepseek/deepseek-v4.1-flash`.
- A `:variant` suffix (`:nitro`) is stripped or replaced by the provider.

Observed on this machine with one real request: the review surface showed
`◆ deepseek-flash-latest` and no amount, and the post-request metadata line
printed no cost either — the existing cost display has the same gap, so the
Amount is invisible for exactly the configuration form the README recommends.

## Proposed Solution

- The Amount for a request is taken from the recorded price of the model the
  provider reported; when that model has no recorded price, it is taken from
  the recorded price of the model the request was sent with.
- When neither has a recorded price, the Model label stands alone, exactly as
  today: nothing replaces the Amount and no value is invented.
- The same lookup serves every place the Amount appears: the review surface,
  the explanation card, and the post-request metadata line, so a configuration
  that records prices under configured aliases gets its cost back everywhere.
- The Amount itself is rendered as cents with one significant digit and never
  more than four decimal places, trailing zeros trimmed, whole cents from one
  cent on — the operator's reading form, replacing the four-decimal form that
  prints digits nobody reads (`0.1001 ¢`).

## Capability Routing

| Proposed capability | Route's recommendation | Decision | Rationale |
|---|---|---|---|
| visible-request-amount | — (route reported no signal) | `EXTEND visible-request-amount` | The new scenario asserts the Amount rule this capability already owns; the resolution order of its price lookup is part of that rule. |
| ask | — (route reported no signal) | `NEW in fragments` | The `ask` scenario lives in the `corpus-infra` fragment: its capability is declared in `givn/specs/fragments/fragment.md`, the delta keeps the fragment's path (`specs/fragments/ask.feature`), and fragment capabilities carry no interactions. One scenario is added to that capability, asserting the post-request metadata line's cost when only the requested model's price is recorded. |

## Out of Scope

- Capturing prices under the provider's canonical identifiers, and any catalog
  call at request time.
- Guessing a price when neither model has a recorded entry, and showing a
  marker in place of an absent Amount.
- Routing behaviour: when the provider serves a different model than the one
  requested and that model has no recorded price, the requested model's price
  is shown.

## Open Questions

### D1 — Is the requested model's price an acceptable amount when the provider's model has no entry?

- Question: Should the Amount fall back to the price recorded for the model the
  request was sent with, when the model the provider reported has no recorded
  price?
- Options:
  1. **Yes, fall back** — reported model first, requested model second; gain:
     the Amount works for alias configurations, which is the common case, and
     the same lookup repairs the metadata line; cost: if the provider serves a
     different model than the one requested, the number is the requested
     model's price; in practice: `◆ deepseek-flash-latest · 0.07 ¢` appears
     where today nothing does.
  2. **No, keep the exact rule and mark the gap** — the Model label gains
     `price unknown`; gain: nothing shown is ever another model's price; cost:
     the developer still sees no money until they re-key `[pricing]` by
     canonical id, and must repeat that whenever an alias moves.
- Disposition: answered — the operator chose option 1 on 2026-09-24, after
  observing the reported id `deepseek/deepseek-v4.1-flash` against the
  configured `~deepseek/deepseek-flash-latest`. This supersedes the rejection
  recorded in the ideation topic's Q2 decision, which assumed the two
  identifiers agree.

### D2 — How is the Amount rendered?

- Question: In what form does the surface show the Amount, now that the price
  lookup makes it appear?
- Options:
  1. **One significant digit, never more than four decimal places** — gain:
     the model label carries the magnitude in the fewest digits the operator
     accepts; cost: `0.1001` and `0.11` both read `0.1`, and an amount below
     0.00005 cents reads `0`; in practice: 1200 prompt and 90 completion tokens
     at 0.15 and 0.60 per million is $0.000234 → `0.02 ¢`.
  2. **Four decimals of a cent, trailing zeros trimmed, never rounding a
     non-zero amount to `0`** — the shipped form; gain: the computation stays
     visible to the last digit and a billed request never reads as free; cost:
     it prints digits the operator rejected as noise (`0.1001 ¢`, `0.11 ¢`) and
     spends header width the model name needs.
- Disposition: answered — the operator chose option 1 on 2026-09-24 while
  reading a live session, accepting `0.0009¢`, `0.001¢`, and `0.1¢` as the
  wanted forms and rejecting `0.1000¢`, `0.1001¢`, and `0.11¢`.
