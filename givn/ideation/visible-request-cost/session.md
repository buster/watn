# Ideation Topic: visible-request-cost

## Current phase

Handoff (complete, confirmed by the user 2026-09-24)

## Status

Complete. Handoff records confirmed and published: the topic document, the
use-case-to-permanent mapping, the persona promotion, the durable term, and the
change seed. No proposal, specification, design, task list, implementation, or
change was started.

## Trigger

User request: "Currently, we read prices, but we do nothing with them. Show the
price of the actions i take in Cents in the review pane."

## Confirmed so far

- Frame confirmed 2026-09-24: the amount a request costs is visible only after
  the fact and only on some paths, never while the review decisions that spend
  money are made; the surface is money-blind.
- Q1 closed: the number is the **already-billed cost of the current request** —
  the usage the provider reported, times the recorded price of the response
  model, in cents.
- Q2 closed: when the response model has no recorded price, the model label
  stands alone; silence is the policy, no marker replaces the amount.
- Q4 closed: when the header cannot hold both, the amount survives and the model
  label is shortened first. Arithmetic corrected in the resolution.
- Q5 closed: the amount belongs to the model the provider billed for, and it is
  shown under the label that names the model the request was sent with.
- Stakeholder: the terminal developer who pays for their own requests. A billing
  owner who never uses Watn is recorded as considered and not evidenced.
- One use case: `observe-request-cost` (`!`, observing), capability
  `visible-request-amount` — both ids provisional.
- Elaborated contract: `use-cases/observe-request-cost.md`, with rules,
  examples, scenario skeletons, extensions, guarantees, the amendments this
  change will need, and the recorded divergences.
- Open: nothing on the critical path. Q3 (cents precision) is parked as
  Design-level: the rules state the unit and that a billed request is never
  shown as zero; the decimal count is a rendering decision.

## Independent review at the Definition boundary

A fresh-context reviewer checked the artifacts against the eight sizing rules
and against the source. Findings applied:

- The premise "printed on stderr after every completed request" was false. Only
  the direct path (`src/main.rs:655`) and the review-accepted path
  (`src/main.rs:1336`) print metadata; `watn explain`, rejected and regenerated
  requests, a cancelled review, and an in-pane permanent disable print none. The
  frame, the chain, and the elaborated case now say so, and the explanation
  card's amount is recorded as new plumbing rather than a second display.
- "No usage means no amount" was false for the existing stderr line: a priced
  model with no reported usage yields `Some(0.0)` and prints `$0.0000`
  (`src/main.rs:629`, `src/output/render.rs:79`). The case now states the
  divergence deliberately: the surface shows nothing there, and the stderr line
  is left as it is.
- The capability id `request-amount-display` named a mechanism; renamed to
  `visible-request-amount`.
- The Definition card lacked its guarantee fields and its trigger restated two
  preconditions; both fixed.
- The Definition claimed the corpus fragment sources the amount computation; the
  computation exists only as two inline copies, and its rules are owned by the
  `ask`, `incremental-sse-rendering`, and `config` capabilities. The artifacts
  now name those capabilities instead of the fragment.
- The change amends `use-shell`'s simple-view rule
  (`givn/specs/use-shell/usecase.md:66`) and the matching glossary entries; the
  Definition now names that amendment instead of claiming the change is purely
  additive.
- Q4's arithmetic was wrong in both directions: the example string was 35
  characters, not 33, and the renderer truncates the tail, so the chosen option
  needs the header to reserve the amount's width first. Corrected in the
  resolution and in the case's examples.
- Two further findings are recorded rather than resolved: the label names the
  requested model while the amount is keyed on the reported model (Q5), and the
  event chain carries `use-shell`'s own outcomes, which are now declared as
  that goal's rather than this one's.

## Independent review at the Elaboration boundary

A second fresh-context reviewer checked the elaborated case against the
confirmed decisions and the source. Findings applied:

- The rules could not express "no usage report" versus "a report of zero"; the
  case now defines usage as a report the response carried and requires the
  surface to track that presence separately, because the existing computation
  folds both into `Some(0.0)`.
- "No amount on a failed request" contradicted the explanation contract, which
  keeps the card open with the command and purpose-unavailable; the extension
  now says no amount is shown on whatever surface the failure contract opens.
  A completed but unusable response is stated to show its billed amount.
- A failed or interrupted regeneration was undefined; the preserved candidate
  now keeps its own request's amount.
- Two examples were wrong: at 40 columns the label shortens to `◆ deepsee…`,
  not `◆ deepse…`, and `0.58 ¢` was a rounding tie that no rule covers — written
  as `0.57 ¢` from $0.00575.
- The differing-model example had no numbers and used a `:nitro` variant that
  the simple view's short name strips; replaced with `openai/gpt-4o` against a
  reported `gpt-4o-2024-08-06`.
- The success guarantee was unconditional while Q4 admits a width floor; the
  guarantee and extension now carry the floor.
- Step 4 and one rule leaked layout wording; the main flow now says "with the
  model the surface names".
- The Definition and chain carried stale statuses for Q4 and Q5; corrected.

## Persona inventory

| Stakeholder type | Candidate persona | Source | Decision |
|---|---|---|---|
| Terminal developer (pays their own requests) | `terminal-developer--interactive` | Durable `givn/personas/terminal-developer--interactive.md`; sibling topic `terminal-wow-factor` | Reuse; topic stance block added in `personas/terminal-developer--interactive.md` |
| Billing owner (pays, never uses Watn) | none | — | Not evidenced by the request; recorded as considered and not confirmed |

Reuse justification: the cost-aware developer shares the existing persona's
authority boundary and evaluation capability and adds only a topic-specific
demand, which belongs in a dated stance block.

## Research protocol

- Permanent specifications under `givn/specs/` and the durable architecture
  under `docs/arc42/` are the source of truth for current Watn behavior.
- Commands run: `givn instructions`, `givn status`, `givn spec tree`,
  `grep -rn -i "price|cost" src/ givn/specs/`, `grep -rInic "price" ...`,
  `grep -rInic "cost estimate" ...`, `grep -rInic "proposed action" ...`,
  `grep -n "print_metadata|fn run_explain|fn run_explanation_card|fn
  run_review_path" src/main.rs`.
- Read: `README.md`, `docs/arc42/12-glossary.md`,
  `givn/specs/fragments/ask.feature`, `givn/specs/fragments/config.feature`,
  `givn/specs/fragments/incremental-sse-rendering.feature`,
  `givn/specs/fragments/fragment.md`, `givn/specs/use-shell/usecase.md`,
  `givn/specs/configure-model/usecase.md`,
  `givn/archive/model-pricing-visibility/proposal.md`,
  `givn/ideation/terminal-wow-factor/` (frame, session, terms, event chain),
  `src/main.rs`, `src/output/render.rs`, `src/setup.rs`, `src/review/card.rs`,
  `src/review/panel.rs`, `tests/steps/incremental_sse_rendering_steps.rs`.
- This topic may identify gaps and new domain behavior; it must not alter
  permanent specifications during ideation.
- Unknowns are recorded in `questions.md`, never resolved by invention.
- Gap noted for Handoff: the ideation method references a durable
  `docs/use-case-model.md` with the worked sizing example; this repository has
  no such file.

## Sibling topic relationship

- `terminal-wow-factor` — Handoff complete. It owns the Review surface,
  Candidate, Stage purpose, and reviewed-output vocabulary this topic builds
  on. It records no cost, price, or amount behavior, and this topic does not
  reopen any of its confirmed decisions.

## Next step

None. Run `/givn-propose observe-request-cost` from the seed in
`changes/observe-request-cost.md` when implementation is wanted. Handoff never
starts a change.
