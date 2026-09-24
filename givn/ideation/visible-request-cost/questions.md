# Open Questions: visible-request-cost

## Q1 — Which decision should the money number serve?

- Status: closed 2026-09-24 (user)
- Blocks: the Frame problem sentence and scope boundary; every later artifact
  (personas, event chain, use cases) derives from it.
- Context: The request names a solution ("show the price of the actions I take
  in cents in the review pane") before naming the decision that needs it. The
  review pane already invites decisions that spend money — accept, reject and
  regenerate with another model, escalate to a higher tier — and today no
  amount is visible while those decisions are made. Three different numbers
  could be meant, and they are not the same number: the amount already billed
  for the candidate in front of the developer, the price of the alternative
  decisions still open, or the running total of the whole review. Which one is
  meant decides whether this is a display of an incurred amount or a pricing
  aid for future spend.
- Question: Which decision should the money number in the review pane inform?
- Options:
  1. **Already-billed cost of the current request** — the pane shows what the
     provider just charged for the request that produced this candidate.
     In practice: the developer opens a candidate and the pane frame reads
     `claude-haiku-4.5 · 0.06 ¢`; the tokens the provider billed for this
     request, times the configured price, no estimate involved.
     Gain: exact, honest, computable at card-open time from data already in
     hand (`final_usage` × recorded price); the amount is the developer's own
     spend and nothing speculative.
     Cost: it cannot help economize — by the time it is visible the money is
     spent; a developer weighing "accept, or reject and try the cheaper model"
     learns nothing about the second, unpriced option.
  2. **Price of the decisions still open** — the pane prices the alternatives
     before they are taken: another model, another tier, another regeneration.
     In practice: pressing `r` lists the configured models with expected cost
     for a request of this size, `1 gemini-3.7-flash ~0.02 ¢` beside
     `2 claude-opus-4.1 ~1.9 ¢`, so escalation is a priced choice.
     Gain: it changes what the developer does — the expensive option becomes
     visibly expensive while there is still time to decline it.
     Cost: the amount depends on tokens not yet generated, and the chosen model
     itself changes them (reasoning tiers emit more); what the pane can honestly
     show up front is a rate per million tokens, not a per-action amount in
     cents.
  3. **Running total of the review** — the pane accumulates every request the
     review has caused: candidate, regenerations, escalations, purpose loads.
     In practice: after a rejection and an escalation the pane footer reads
     `this review: 0.21 ¢` while the current candidate's own request was
     `0.06 ¢`.
     Gain: it prices the exploration loop the pane is built to invite, which is
     the one place where a session can quietly spend many times one command's
     worth.
     Cost: it needs a session accumulator and a defined reset boundary, and it
     still says nothing about future spend; a single number only makes sense
     once the developer knows whether it is "so far" or "this request".
- Resolution: **already-billed cost of the current request** because it is the
  only one of the three that is an exact, already-incurred amount rather than an
  estimate of tokens that do not exist yet, and it is computable from data Watn
  already holds when the pane opens. Rejected: **price of the decisions still
  open** because a request's token count is only known after it is generated
  and the chosen model changes it, so the pane could only show a per-million
  rate, not an amount in cents; **running total of the review** because it
  requires session accumulation and a reset boundary Watn does not have, and
  the request asked for the cost of the action taken, not the cost of the
  session.

## Q2 — What does the pane show when the response model has no recorded price?

- Status: closed 2026-09-24 (user)
- Blocks: the Event Storming read model for the amount display, the use-case
  rule for the display, and every example that shows a model label.
- Context: An amount exists only when a pricing entry matches the provider's
  response model id (`src/main.rs:629`, keyed on `response.model`; the permanent
  scenario `A usage-only final event supplies cost and throughput metadata`
  asserts metadata names exactly the response model). Two real cases leave no
  entry: a model configured without catalog price metadata — quick setup's
  manual entry, or a hand-written config — and a provider that reports a
  normalized id that differs from the configured one. Today the absence is
  silent: the stderr metadata line simply omits the money portion. In the pane
  the same silence would sit next to a model label, where "this request cost
  nothing" and "watn cannot price this model" would look identical.
- Question: When no recorded price matches the response model, what does the
  review pane show at the model label?
- Options:
  1. **Nothing — the model label stands alone** — `◆ claude-haiku-4.5` with no
     money text, identical for a cheap request and an unpriceable model.
     In practice: a developer who configured a model by hand sees the same pane
     as before this change and cannot tell whether the request cost 0.04 ¢ or
     was never priced.
     Gain: no new pane state, no new vocabulary, and exact consistency with the
     stderr metadata, which already omits the money portion silently.
     Cost: the absence carries no information; the case the developer would
     want to fix — a model with no recorded price — is the invisible one.
  2. **An explicit marker at the model label** — `◆ claude-haiku-4.5 · price
     unknown`.
     In practice: the developer learns the model has no recorded price while
     the request is still on screen, and can run `watn models` to capture it; a
     provider-id mismatch becomes visible instead of silent.
     Gain: absence becomes information; it names a configuration gap the
     developer can close.
     Cost: one more pane state and one more term to keep, in a surface that was
     deliberately reduced to the model, the stages, and the selected purpose;
     the label grows in exactly the case where there is no number to show.
  3. **Fall back to the price recorded for the model the request was sent
     with** — `◆ claude-haiku-4.5 · ~0.07 ¢`, using the configured model's
     entry when the response model has none.
     In practice: prices are captured under the same id a request is sent with,
     so most mismatches (a provider reporting a normalized id) produce the right
     amount instead of nothing.
     Gain: a number appears in nearly every review; the developer's most common
     setup keeps working.
     Cost: the number becomes an assumption whenever provider and configuration
     disagree about which model ran; it contradicts the exactness that made this
     number worth preferring over a rate, and the `~` would have to promise more
     than the pane can know.
- Resolution: **nothing — the model label stands alone** because the absence is
  already Watn's behavior everywhere else the amount appears, and a new pane
  state would have to be kept in the pane's smallest presentation for a case
  that is a configuration gap, not a review decision. Rejected: **an explicit
  marker** because it adds a state and a term to the deliberately reduced
  simple view without changing any decision the developer makes in the pane;
  **falling back to the configured model's price** because it contradicts the
  exactness that was the reason to prefer this number over a rate and would
  present an assumption as a billed amount.
- Consequence recorded: the absence stays silent, so a developer whose model
  has no recorded price sees no money anywhere and has no in-pane signal to
  explain it. The configuration path (`watn models`, `watn setup`) remains the
  place where a missing price is visible.

## Q3 — How precise is the cents display?

- Status: open (not asked; Design-level)
- Blocks: nothing in ideation. The use-case rule states the unit (cents) and
  that an amount is never rendered as zero when the provider billed something;
  the exact number of decimals is a rendering decision for Design and the
  specification examples.
- Context: One cent is $0.01 and a typical request lands between $0.0001 and
  $0.02 — 0.01 ¢ to 2 ¢. Whole cents would render nearly every request as
  `0 ¢`. Two decimals (`0.06 ¢`) cover the common range but collapse anything
  under 0.005 ¢ to `0.00 ¢`; more decimals keep the cheapest requests
  distinguishable at the cost of a longer label in an already shared header
  segment. The existing stderr metadata uses four decimals in dollars
  (`$0.0007`).
- Question: How many decimals does the cents amount carry before it stops
  carrying signal?
- Options:
  1. **Two decimals** — `0.06 ¢`; the common range reads cleanly and a request
     below half a hundredth of a cent shows `0.00 ¢`.
     Gain: shortest label; matches how people read cents.
     Cost: a billed request can look like zero.
  2. **Three or four decimals** — `0.0612 ¢`; every billed request reads
     non-zero.
     Gain: no silent zero; the exactness of the computation stays visible.
     Cost: longer label in a header that also carries the model name and is
     ellipsized when narrow.

## Q4 — When the header cannot fit both, does the model label or the amount survive?

- Status: closed 2026-09-24 (user)
- Blocks: the display rule in the use case (whether "the amount is visible" is
  a guarantee at every width), the narrow-layout examples, and the header
  layout Design.
- Context: The amount shares the pane header's right segment with the model
  label (`src/review/card.rs:264`). That segment is ellipsized to what remains
  after the left segment ` watn · review `: at 40 columns the right segment has
  19 visible characters, and the pane is tested at that width
  (`InlineLayout::for_dimensions(40, 8)`). `◆ deepseek-v4-flash-latest · 0.06 ¢`
  needs 33 characters, so at narrow widths one of the two is cut. Q2 has just
  decided that a missing amount is silent, which makes an ellipsized amount
  indistinguishable from an unpriced model.
- Question: When the header's right segment cannot hold both, which of the two
  survives?
- Options:
  1. **The amount survives; the model label is ellipsized first** — at 40
     columns the header reads `◆ deepseek-v4-f… · 0.06 ¢`.
     In practice: a developer in a narrow split pane still sees what the
     request cost, but not which model produced it.
     Gain: the display promise holds at every width the pane can render, which
     is what the topic asked for.
     Cost: the number stands without the label that explains it, and the model
     name — the one hint that would let the developer fix a missing price — is
     exactly what gets cut.
  2. **The model label survives; the amount is dropped first** — at 40 columns
     the header reads `◆ deepseek-v4-flash-latest` with no amount.
     In practice: the existing header contract is untouched at every width, and
     the amount appears as soon as there is room.
     Gain: no new layout state; the pane never shows a number detached from the
     model it belongs to.
     Cost: the feature silently disappears on narrow terminals, and because
     Q2 keeps absence silent, that case is indistinguishable from an unpriced
     model.
  3. **The amount falls back to its own row when the header cannot carry it** —
     at 40 columns a row `cost        0.06 ¢` appears above the hints while the
     header keeps the model label; at 100 columns the header carries the amount
     and no row appears.
     In practice: the amount stays visible and readable at any width.
     Gain: no silent disappearance and no detached number.
     Cost: a second layout state to keep, and the row competes for the pane's
     row budget with the stage stack and the purpose rows, which the pane
     already drops by priority when space runs out.
- Resolution: **the amount survives and the model label is shortened first**
because the point of the whole topic is that the money is visible, and a
shortened model name still identifies the model while a dropped amount is
indistinguishable from the unpriced case Q2 decided to keep silent. Rejected:
**the model label survives** because the feature would silently disappear on
narrow terminals, which is the failure this topic exists to remove;
**the amount falls back to its own row** because it adds a second layout state
and competes with the stage stack and the purpose rows for the pane's row
budget.
- Correction recorded: the widths in this question were wrong. The full label
`◆ deepseek-v4-flash-latest · 0.02 ¢` is 35 characters, and the renderer's
`ellipsize` cuts the *tail* of the header segment (`src/review/card.rs:105`),
so leaving the current truncation in place would have cut the amount first —
the option the user chose requires the header to reserve the amount's width
before shortening the label. At 40 columns the right segment holds 19
characters, so the realised example is `◆ deepse… · 0.02 ¢`, not the 25
characters shown in option 1. Below roughly 33 columns the segment cannot
hold the amount at all and the pane shows what fits; that floor belongs to
Design.

## Q5 — When the provider reports a different model than the one requested, which model does the amount belong to?

- Status: closed 2026-09-24 (user)
- Blocks: the display rule and the examples for the case where the label and
the billed model disagree.
- Context: The pane's label is the model the request was sent with
(`ReviewContext.model`, `src/review/panel.rs:72`), while the amount is keyed
on the model the provider reports for that response (`src/main.rs:629`,
`:1311`). The permanent scenario `A usage-only final event supplies cost and
throughput metadata` pins that keying and asserts the stderr line names
exactly the response model
(`givn/specs/fragments/incremental-sse-rendering.feature:27`). Usually the two
ids are identical; when a provider normalizes one — a routing suffix such as
`:nitro`, or a renamed snapshot — the label names one model and the amount is
billed for another. Q2 covers only the case where no entry matches; it says
nothing about an entry that matches for the billed model while the label names
a different one.
- Question: When the provider's reported model differs from the model the
request was sent with, which model does the displayed amount belong to?
- Options:
1. **The billed amount, shown under the requested label** — the label stays
   `◆ claude-haiku-4.5` and the amount is the one computed for the model the
   provider reported.
   In practice: the developer asked for the `:nitro` variant, the provider
   answered with the plain snapshot, and the pane shows the amount the
   provider actually billed for the plain snapshot next to the variant's name.
   Gain: the number is always the true billed amount; nothing known is hidden;
   it is the same relationship the stderr metadata line already has to the
   response model.
   Cost: the header implies the amount belongs to the model it names, so when
   the provider routed elsewhere the pane is quietly about two models.
2. **The amount only when both names agree** — a mismatch means the label
   stands alone, exactly like the unpriced case.
   In practice: after such a mismatch the developer sees no money in the pane
   and cannot tell whether the request was priced or the names differed.
   Gain: nothing shown is ever misleading, and one silence rule covers both
   the unpriced and the renamed case.
   Cost: a true, already-computed amount is hidden because of a naming
   difference, which is the case Q2's silence already makes hardest to
   diagnose.
3. **The amount labelled with the model the provider reported** — the header
   names the billed model whenever it differs from the requested one.
   In practice: the label changes from `◆ claude-haiku-4.5:nitro` to
   `◆ claude-haiku-4.5` the moment the response arrives.
   Gain: label and amount always belong together, so the number is never
   ambiguous.
   Cost: the label changes under the developer mid-flow, and the model they
   chose is no longer the model they see.
- Resolution: **the billed amount, shown under the requested label** because the
number is the truth of what was billed and hiding it over a naming difference
would remove money Watn already knows, in a surface whose whole purpose is to
show it. Rejected: **showing the amount only when both names agree** because
the resulting silence is indistinguishable from the unpriced case and hides a
real amount; **labelling the amount with the provider's model** because the
label would change under the developer mid-flow and the model they chose would
no longer be the model they see.
