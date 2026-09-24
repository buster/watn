# Use Case Definitions: visible-request-cost

## The cut

One use case, observing, with one capability. The event chain contains no
second actor goal: the developer's only goal here is to learn what the request
they just made cost, and every event in the chain either produces that amount
(billed usage, recorded price, computation) or presents it (the surface the
request produced).

## Use case: observe-request-cost

| Field | Value |
|---|---|
| Level | `!` |
| Primary actor | Terminal developer |
| Goal | Know the amount the provider billed for the request one just made. |
| Trigger | A provider request the developer submitted has completed with a response. |
| Observing or changing | Observing. It owns no mutating interaction; every event that spends money or releases a command belongs to the request flow that produced the amount. |
| Minimal guarantee | When the request reported no usage or no recorded price matches its model, no amount and no substitute for it is displayed, and the surface behaves exactly as without this case. |
| Success guarantee | The developer sees, in the surface that their request produced, the amount the provider billed for that request. |
| Boundary | The money consequence of one's own request. Prices as data, model choice, and command review stay in their own boundaries. |
| Personas | `terminal-developer--interactive` (cost stance, 2026-09-24) |
| Capability | `visible-request-amount` |
| Permanent mapping | A new leaf document `givn/specs/observe-request-cost/` — not a capability inside another goal, because the amount advances no existing goal's success guarantee |

Provisional identifiers: the use-case id `observe-request-cost` and the
capability id `visible-request-amount` are proposals of this phase, to be
confirmed with the boundary. The capability was renamed from
`request-amount-display` after review: "display" names a mechanism, and a
capability name may not.

## Sizing verification

Checked against the falsifiable rules:

1. **One goal, one primary actor.** Goal is one verb phrase — know an amount —
   naming one actor's outcome. The success guarantee states one outcome, and
   the minimal guarantee is the failure-side of the same one. No "and" joins
   two verb phrases. Closure test passes: the developer can stop after reading
   the amount and the system is consistent.
2. **Goal-shaped, never component-shaped.** Neither the use-case id nor the
   capability id names a component, mechanism, or channel. Placement test: the
   display advances the developer's goal of knowing their own spend, and it
   does **not** advance the existing goal of reviewing and returning a
   candidate — Q1 rejected the reading that this number informs the
   accept-or-regenerate decision — so the display is not a capability of that
   goal. Channel is not the placement argument: the review pane and the
   explanation card are two presentations of the same outcome.
3. **Level discipline.** `!` owns the capability directly. One capability, one
   goal, no child use cases, so no promotion to `+` is due.
4. **Mutation containment.** The case is observing and owns zero mutating
   interactions. Requesting a completion, buffering a candidate, replacing a
   shell buffer, and authorizing execution remain with `use-shell` and with the
   asking capability of the corpus substrate.
5. **Extension discipline.** Every extension in the elaborated case names the
   main-flow step it branches from and the guarantee it protects; none
   introduces a new actor goal, a new success outcome, or a new capability.
   Rejecting a candidate and thereby causing another request is the request
   flow's business; this case's extension only replaces the amount the surface
   displays when that request succeeds, and keeps the preserved candidate's own
   amount when it fails.
6. **Precondition and flow consistency.** Preconditions (a configured provider
   and model; a request that completed with a response; a review-eligible path)
   hold before the trigger and are not established by it. The trigger is an
   event, not a state or desire. The success guarantee is established by the
   main flow, never assumed.
7. **Implementation independence.** Goal, trigger, main flow, and guarantees
   survive a rewrite in another language or framework: they name the developer,
   the request, the amount, and the surface, not headers, escape sequences,
   stream events, or token counters. Provider and Watn are supporting actors,
   not the goal.
8. **Relationship semantics.** `Includes` is the unconditional substrate
   include (`fragment: corpus-infra`), which every use case carries.
   `Extends` is `none`, and no fragment is a target of it. The graph stays
   acyclic: no permanent document depends on this new leaf.

## Boundary collisions checked

- `configure-model` owns choosing models and capturing their prices; its out of
  scope already excludes behavior after setup. This case consumes the recorded
  price and adds nothing to that goal.
- `use-shell` owns the review surface, the candidate, and the decisions. This
  case owns only what the amount is and that it appears at the model label; the
  surface it appears in stays `use-shell`'s — with one amendment, named below,
  because `use-shell`'s simple-view rule says the simple view names *only* the
  model, the stages, and the selected purpose.
- The amount's computation, the response-model keying, and the existing stderr
  presentation are owned by the corpus substrate capabilities `ask`
  (`givn/specs/fragments/ask.feature:60`), `incremental-sse-rendering`
  (`givn/specs/fragments/incremental-sse-rendering.feature:27`), and `config`
  (`givn/specs/fragments/config.feature:36`). This case presents their result in
  one more place and restates none of their rules.

## Amendments this change will need

- `givn/specs/use-shell/usecase.md:66` — the simple view's "names only the
  model" rule must admit the amount.
- `docs/arc42/12-glossary.md` — `Simple review view` and `Model short name`
  carry the same rule; the durable term for the incurred amount is missing
  entirely.
- Not an amendment but a scope fact: the explanation path computes no amount at
  all today, so the card's amount is new plumbing.

## Personas affected

- `terminal-developer--interactive` — the only persona. Its cost stance is the
  review lens for this definition and for the elaborated case, where its
  critique and its recorded tension with Q2's silent absence live.

## Open questions carried forward

- Q3 — cents precision (Design-level, not gating).

All boundary questions (Q1, Q2, Q4, Q5) are closed; see `questions.md` for the
full text, the options, and the rejected alternatives.

## Artifacts

- Elaborated contract: `use-cases/observe-request-cost.md`.
