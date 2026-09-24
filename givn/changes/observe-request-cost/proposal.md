# Proposal: observe-request-cost

## Use-Case Context

- Use-case ID: observe-request-cost (new leaf; this change creates its
  permanent document at `givn/specs/observe-request-cost/`)
- Ideation topic: visible-request-cost
- Seed path: `givn/ideation/visible-request-cost/changes/observe-request-cost.md`
- Confirmed Personas: `terminal-developer--interactive` (cost stance, recorded
  2026-09-24) — the developer who pays for their own requests
- Related permanent contracts: the review surface and the explanation card
  (`use-shell`); the amount's computation and its stderr line (`ask`,
  `incremental-sse-rendering`, `config`)

## Problem / Opportunity

Every request Watn makes is billed to the developer. The amount is shown only
after the fact, only on some paths, and never where the money is spent:

- An interactive invocation that ends in a direct answer prints the amount on
  stderr after the response; a review that ends in acceptance prints it after
  the surface has closed.
- `watn explain`, every rejected or regenerated request, a cancelled review, and
  a surface disabled from inside itself print no amount at all.
- The review surface itself — where a candidate is read, compared, accepted, or
  rejected in favour of another request — carries no amount, even though the
  amount already exists by the time the surface opens.

The developer therefore learns what a request cost on a line that scrolls away,
or never; and the surface that invites spending another request shows nothing
about what the last one cost.

## Proposed Solution

The developer sees, in the surface a request produced, the amount that request
was billed:

- The review surface shows, with the model it names, the amount the provider
  billed for the request that produced the surface, in cents.
- The explanation card shows the amount billed for its explanation request the
  same way.
- The amount appears when the surface opens and leaves with it.
- When a candidate is rejected and the next request succeeds, the surface shows
  the new request's amount in place of the previous one. When the next request
  fails or is interrupted, the preserved candidate keeps its own amount.
- When the response carried no usage report, or no recorded price matches the
  model the provider reported, the model name stands alone: nothing takes the
  amount's place, and a request the provider did not account for is never shown
  as a free one. A response that does report usage of zero tokens has been
  accounted for and shows a zero amount.
- The amount billed for the model the provider reported is shown under the name
  of the model that was requested, even when the two differ.
- When the model name and the amount cannot both fit, the model name is
  shortened first so the amount stays visible while it can be; no amount ever
  appears without a model name.
- The amount is never part of command output, and it gates nothing: accepting,
  editing, cancelling, rejecting, regenerating, escalating, and disabling
  behave exactly as they do today.
- Everything outside the surface is unchanged, including the existing stderr
  line and its current `$0.0000` for a priced request with no usage report, and
  every non-interactive path.

## Capability Routing

Route reported no signal (top candidates tied at 13.00), so no recommendation
column is recorded.

| Proposed capability | Decision | Rationale |
|---|---|---|
| visible-request-amount | `NEW in observe-request-cost` | The amount informs no accept-or-regenerate decision — that reading was rejected during ideation — so it advances no existing goal's success guarantee and belongs to no existing capability. The surface it appears in stays `use-shell`'s; the computation and its stderr line stay with `ask`, `incremental-sse-rendering`, and `config`. |

Document amendments this change must carry, because one existing rule
contradicts the new behaviour: the review goal's simple-view rule, "names only
the model, stacks the command-flow stages with their separators, and shows the
selected stage's purpose" (`givn/specs/use-shell/usecase.md:66`), and its
restatement in the glossary's `Simple review view` and `Model short name`
entries, must admit the amount with the model it names.

## Out of Scope

- Predicting or comparing spend for requests not yet made; rates for models not
  yet used.
- Session totals, budget alarms, and spending limits.
- The amount's computation and its existing stderr line, unchanged here,
  including the `$0.0000` the line prints for a priced request with no usage
  report.
- Non-interactive paths and a permanently disabled surface: no surface, no
  amount.
- Choosing or pricing models (`configure-model`) and provider configuration
  (`configure-provider`).
- The review decisions themselves (`use-shell`): the amount changes none of
  them.
- Provider invoices, reconciliation, currencies other than USD, refunds.

## Open Questions

### D1 — How precise is the cents display?

- Question: How many decimals does the amount carry before it stops carrying
  signal — the unit is cents, so whole cents would render nearly every request
  as `0 ¢`.
- Options:
  1. **Two decimals** — the common range reads cleanly as `0.02 ¢`, and a
     request below half a hundredth of a cent shows `0.00 ¢`; gain: the
     shortest label, matching how cents are read; cost: a billed request can
     look like zero; in practice: 1200 prompt and 90 completion tokens at 0.15
     and 0.60 per million is $0.000234 → `0.02 ¢`.
  2. **Three or four decimals** — every billed request reads non-zero as
     `0.0234 ¢`; gain: no silent zero and the exactness of the computation stays
     visible; cost: a longer label in a header segment the model name also
     needs; in practice: the same request → `0.0234 ¢`.
- Disposition: carried — owner: this change's design step, decided in
  `design.md` together with the rounding rule; the specification fixes the unit
  (cents) and that a billed amount is never displayed as zero.
