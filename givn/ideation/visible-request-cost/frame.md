# Frame: visible-request-cost

Frame status: **confirmed** on problem, vision, stakeholder, and scope
(2026-09-24, user). Decisions below are derived either from the user's answer
to Q1 or from reading the permanent specifications.

## Problem

Watn bills the developer for every request it makes, and the amount is only
ever visible after the money is spent: one `$0.####` line on stderr once the
response has completed. While the developer is deciding — accept this
candidate, or reject it and spend another request on a regeneration or a
higher tier — the review pane shows no amount at all and cannot answer what
the action just taken cost.

The request that opened this topic framed it as "we read prices, but we do
nothing with them". That premise is half wrong and the correction matters:

- Watn does use prices, but only on some paths. A cost is computed from the
  response's token usage and the recorded price and printed on stderr at the
  end of the *direct* path (`src/main.rs:655`) and the *review-accepted* path
  (`src/main.rs:1336`). The same computation is absent from `watn explain`
  (`src/main.rs:833`), from every rejected or regenerated request, and from a
  cancelled review or an in-pane permanent disable — on those paths the money
  is not displayed at all, not even after the fact.
- The review pane is money-blind: `ReviewContext` carries intent, tier,
  provider, and model only (`src/review/panel.rs:72`).

## Vision

The developer sees what the request they just made cost them, in cents, in the
review surface that the request produced — so the price of the action is part
of the same pane that asks for the next action, and no longer arrives after
the fact on a line that scrolls away.

## Confirmed scope (Q1)

The number is the **already-billed cost of the current request**: the tokens
the provider reported for the request that produced the candidate on screen,
multiplied by that model's recorded price. It is not a rate, not a prediction,
and not a session total.

## Decisions derived from the confirmed direction

Reported for correction rather than asked, because they follow from the
confirmed direction plus existing permanent behavior:

- **Placement.** The pane's header right segment already carries the model
  label — `◆ claude-haiku-4.5` in the simple view, `◆ tier 1 ·
  provider/model` in the detailed view (`src/review/card.rs:264`). The amount
  belongs to that label, so it appears in both views without adding a row:
  `◆ claude-haiku-4.5 · 0.06 ¢`. A separate row would compete with the pane's
  row budget for a number that qualifies the label it sits beside.
- **The explanation card carries it too, and that is new plumbing.**
  `watn explain` bills an explanation request, and its card is the surface that
  request produced; today that path computes no cost at all, so the card's
  amount is a computation the path never had.
- **Non-review paths are unchanged.** Direct non-interactive paths and a
  permanently disabled review surface keep the existing post-request stderr
  metadata and nothing else.
- **The amount is transient.** It disappears with the pane; accepting,
  cancelling, or closing leaves nothing behind. Regenerating or escalating
  replaces it with the new request's amount, and the previous amount leaves
  with the previous candidate.

## Constraints recorded for later phases

- **Sub-cent amounts are the normal case.** One cent is $0.01 and a request
  typically lands between $0.0001 and $0.02 — 0.01 ¢ to 2 ¢. See Q3 for the
  precision fork.
- **The number can be missing, and its absence is silent.** Decided in Q2:
  without a recorded price for the response model, no amount is shown and
  nothing replaces it. A fabricated number is forbidden; so is a marker that
  would have to be kept in the pane's smallest presentation.
- **Currency is USD only**, per the recorded pricing configuration.
- **The amount never becomes command output.** It belongs to the review
  surface, which renders on the controlling-terminal channel; stdout stays the
  command-output channel.

## Existing context

- Pricing configuration holds per-model input and output USD per one million
  tokens, keyed by model id; written by setup from published catalog prices
  (`Price capture`, `src/setup.rs:306`), and writable by hand.
- The cost of a request is `prompt_tokens × input price + completion_tokens ×
  output price`, divided by one million, looked up by the provider's response
  model id. A model without a matching entry yields no amount at all. A request
  whose response reports no usage yields `Some(0.0)` today and prints
  `$0.0000`; the review surface will not repeat that, see Q2's policy.
- The review path buffers the complete candidate and opens the pane after the
  response ends, so the request's billed tokens exist before the pane opens.
- Review decisions that spend money: reject and regenerate with another model,
  escalate to another model or tier, rephrase. Accept, edit, cancel, and
  disable spend nothing further.
- Purpose loading adds no separate request: purposes arrive inside the
  structured review response.

## Stakeholders

- **Terminal developer** — the person who expresses the intent, makes the
  request, decides, and pays. Confirmed by the frame as the topic's stakeholder
  and matched to the recorded persona `terminal-developer--interactive` with a
  topic stance; see `personas/`.
- **Billing owner** — pays the provider bill without using Watn. Not evidenced:
  the request speaks in the first person and names no team, budget, or
  reimbursement concern. Recorded as considered and not confirmed.

## Out of scope

- Rates and comparisons for models not yet used, and any prediction of future
  spend (rejected with Q1 option 2).
- Session accumulation, budget alarms, and spending limits (rejected with Q1
  option 3).
- Provider invoices, reconciliation, and currencies other than USD.

## Frame boundary review

The Frame's own checklist — research protocol recorded, stakeholders named with
interests, problem and vision stated in domain language — is satisfied; the
frame is confirmed. Persona confirmation and the Event Storming chain are the
next gates.
