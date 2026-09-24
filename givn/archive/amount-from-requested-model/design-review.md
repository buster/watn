# Design review: amount-from-requested-model

Reviewed plan: `proposal.md`, `specs/observe-request-cost/visible-request-amount.feature`,
`specs/observe-request-cost/usecase.md`, `specs/fragments/ask.feature`,
`design.md`, `arc42.md`.

Grilling ran as a fresh-context subagent against the plan as it stood after
design; every branch below was walked. The rendering question (D2) was added by
the operator while the change was open, and its plan text is reviewed here on
the same terms as the rest.

## Findings (settled by the grilling agent, with evidence)

| # | Finding | Disposition |
|---|---|---|
| 1 | The glossary's `Amount` definition and the `Model label` anti-term still described the reported-model-only price rule (`docs/arc42/12-glossary.md`), so the durable vocabulary would have contradicted the amended use case | Fixed: both rows amended to the price that applies to the request |
| 2 | `arc42.md` marked chapter 12 unaffected and omitted `12-glossary.md` from Files touched while the change amends both | Fixed: row 12 reads Yes and the file is listed |
| 3 | Three defects in the use-case text: a duplicated price rule, `no fallback price, no per-million rate` colliding with the new fallback, and a broken Minimal-guarantee sentence | Fixed: the price rule is stated once, the rule now forbids estimation rather than a fallback, and the Minimal guarantee names both models |
| 4 | No test distinguished the reported model's price from the requested model's price, so the fallback order was asserted only through the surface | Fixed: unit test `the_reported_model_price_wins_over_the_requested_model_price` asserts both ends, including the `None` case |
| 5 | `tests/steps/interactive_shell_shortcut_steps.rs` derived the amount from a single model id, so an in-process scenario could not exercise the fallback at all — a scenario written on that harness would have proven nothing | Fixed: `surface_amount_for` builds the pricing map from the scenario's recorded prices and calls the production `recorded_price`, taking the requested model explicitly; regeneration passes the regeneration model |
| 6 | `docs/arc42/05-building-block-view.md` still described `cents_text()` as the four-decimal form with a never-zero guarantee and `billed_amount` as keyed on the reported model — stale after change 1's refinement and wrong after D2 | Fixed: both rows and the section's responsibility sentence describe the shipped behaviour; row 5 of `arc42.md` flips to Yes |

## Operator decisions

- **D1 — Is the requested model's price an acceptable amount when the provider's
  model has no entry?** Answered by the operator (option 1: fall back) on
  2026-09-24, after observing the reported id `deepseek/deepseek-v4.1-flash`
  against the configured `~deepseek/deepseek-flash-latest`. Options, gain, cost,
  and the rejected marker option are recorded in the proposal; the design's D1
  names the realization and the rejected alternative.
- **D2 — How is the Amount rendered?** The operator refined the form while
  reading a live session: `0.0009`, `0.001`, and `0.1` are wanted; `0.1000`,
  `0.1001`, and `0.11` are not. The proposal records the question with both
  options and the disposition; the design's D2 records the rule and its
  consequences, including that an amount below 0.00005 cents reads `0` — the
  previous form's never-zero guarantee is deliberately replaced by the
  four-decimal cap, and R-094 records the residual risk.

## Branch walk

| Branch | Result |
|---|---|
| Scope | The change contains exactly the two proposal questions: one lookup rule, proven by the review-surface scenario and the metadata-line scenario, and one rendering rule, expressed as eight `@givn.modified` scenarios and three unit tests. Nothing from the proposal's Out of Scope is implemented: no canonical-identifier capture, no catalog call at request time, no marker for an absent Amount, no routing change. |
| Tech choices | No new dependency. The fallback is one function (`amount::recorded_price`) with four call sites; the form is one formatting computation inside `cents_text()`. The alternative for D1 — repairing the configuration so the reported id is keyed — is recorded as out of scope in the proposal. |
| Missing scenarios | Found and fixed: D2 changes the expectation of eight existing permanent scenarios, each carried as `@givn.modified` and pre-applied to `givn/specs/`. The residual case (neither model priced) and the reported-zero case keep their existing scenarios; the fallback on a *regeneration* is not separately scenario'd because the regeneration path passes the regeneration model through the same `surface_amount` call the review-surface scenario covers — recorded here as a known thin spot, not a silent gap. |
| Testability | Every new or changed scenario fails in RED without the production change: the fallback by neutering the lookup (`let _ = requested_model;` — exit 1 for both scenarios), the form by forcing the four-decimal rendering (279 regular scenarios: 266 passed, 13 failed; 93 e2e: 89 passed, 4 failed). The Then-steps assert concrete values rendered at the model label, not "it ran". |
| Risk | The most likely failure is a stale expectation left somewhere the form change touches — the permanent spec, the use-case examples, the README, or the visual evidence. Mitigation: every value is computed with the shipped formula before it is written, the permanent scenarios are pre-applied so the pre-archive run executes the new expectations, and this change's two e2e scenarios commit their own transcripts. |
| Use case and Persona context | `observe-request-cost` keeps its ID, actors, guarantees, and capability ownership; the delta carries the amended use case with its base hash and the archive replaces it. The confirmed Persona `terminal-developer--interactive` (cost stance, 2026-09-24) remains the review lens; the interaction inventory keeps its two entries and their two `@e2e` scenarios. No Persona biography appears as a Gherkin actor. |
| Decision ledger | Proposal D1 → design D1; proposal D2 → design D2. No unresolved item, no seed reference to carry (this change has no seed). |
| Visual design contract | Present in `design.md`: `Interface: terminal`, rendered references for the whole-cent, sub-cent, and smallest shown steps, evidence kind transcripts, and the states covered. The opposite classification was tested first: a screen rendering HTML/CSS/JS in a browser would be a Web UI needing a browser driver; nothing here renders browser content. |
| Deferrals | None. |
| Persona disposition | `terminal-developer--interactive`: relevant — the change exists so the money reaches the developer inside the surface that asks for the next decision, and D2 exists so that money is readable at a glance. |
| ADR qualification | `arc42.md` carries two candidates (the lookup rule, the rendering form); both read `NOT_QUALIFIED` with `lower_level_artifact: PASS` and a named canonical home, and the existing-ADR check found no pricing or display-form decision in ADR-0001…ADR-0026 or `docs/adr/`. |
| Architecture documentation (arc42) | Re-derived before opening `arc42.md`: rows 5, 8, 11, and 12 are Yes — the amended `Amount` building block, the cost-tracking section, R-093 with the amended R-092 and the new R-094, and the glossary; rows 1–4, 6, 7, 9, and 10 are No (no goal, constraint, interface, flow, deployment, decision, or quality scenario changes). `arc42.md` matches, every Yes chapter was opened and checked against `design.md`, all twelve chapter files exist with real content, and no chapter carries an ASCII-art diagram. |

## Hardening applied

- The eight modified scenarios' expectations and the delta copies are written
  from the shipped formula; the permanent text is pre-applied so the runner's
  permanent-plus-delta view is green while the change is open, and the archive
  merge replaces each scenario by title with identical text.
- The evidence transform was hardened after reviewing a captured transcript:
  the progress line shares the header's row, so dropping every line containing
  it silently dropped the model label and the amount from the evidence;
  `commit_e2e_transcript` now keeps the header text and commits under the change
  that owns the scenario.
- `docs/arc42/05-building-block-view.md`, chapter 8's cost-tracking paragraph,
  the glossary, R-092, the README's review-surface paragraph, and the use case's
  examples were all brought onto the shipped rules.
- The design's architecture table names the two test-support files this change
  touches, so the implementation's write set matches the reviewed design.

DESIGN-REVIEW: PASS
