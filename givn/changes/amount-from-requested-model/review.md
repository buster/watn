# Review: amount-from-requested-model

## Step 0 — fabrication audit

| # | Check | Result |
|---|---|---|
| 0 | `@e2e` tag integrity in the delta specs | Clean. `specs/observe-request-cost/visible-request-amount.feature` carries one `@givn.added` scenario and eight `@givn.modified` copies; the two end-to-end ones keep `@givn.modified @e2e`, the other six are in-process, and the `ask` fragment's scenario keeps `@givn.added`. No scenario lost an `@e2e` tag; the two e2e modifications are duplicates of permanent e2e scenarios, not new actions. |
| 1 | Empty or no-op step bodies | Clean. `grep -rn "unimplemented!\|todo!()" tests/` → 0 matches; the two cost step files carry 16 assertion calls; this change adds no step body that does not assert, configure, or read a rendered surface. |
| 2 | Every `[x]` task has a commit touching production source | `e1d2848` touches `src/amount.rs`, `src/main.rs`, and the three test-support files; `aebdd80` is the visual-evidence commit for the two transcripts, which is an artifact by nature — the production change it evidences is in `e1d2848`. No task claims a `.feature`-only or stub commit. |
| 3 | Components `design.md` promised exist | Clean. `amount::recorded_price`, the rewritten `cents_text()`, the requested-model parameter at all four call sites, `accepted_model` in the review path, the harness's explicit requested model, and `commit_e2e_transcript`'s owning-change parameter and header rescue. |
| 4 | Proof of strictness present with non-zero exit | Present in `tasks.md` (T1/S1–S3): the RED runs exit non-zero — the neutered lookup (`the model label should carry a billed amount`), and the forced four-decimal form (`279 scenarios (266 passed, 13 failed)` regular, `93 scenarios (89 passed, 4 failed)` e2e). No `unimplemented!` stub was introduced by this change; the runner's `.fail_on_skipped()` and skipped-count exit are unchanged. |
| 5 | Every `@e2e` scenario asserts on the real interface | Clean. Both money scenarios read the pseudo-terminal's rendered header (`pty_snapshot`); the Ctrl-W scenario asserts the shell buffer. None asserts against a repository or database. |
| 6 | Browser-UI step fidelity | Not applicable: the interface is a terminal; no browser and no HTTP client is used as a driver. |
| 7 | `verify.e2e_command` target and duplicate implementations | `./run-tests.sh --e2e` → `cargo test --test features_runner -- --tags "@e2e and not @wip"`. Searched the tree: one implementation per step, in `tests/steps/observe_request_cost_e2e_steps.rs` and the shared review harness. |
| 8 | E2E scope: one e2e scenario per normalized action | Clean. The two inventory entries keep their two `@e2e` scenarios; this change adds no third e2e action. The eight modifications duplicate their permanent scenario verbatim except for the asserted amount, and the merge replaces each by title. |
| 9 | Local run command starts the stack including twins | Clean. `./run-tests.sh` and `./run-tests.sh --e2e`; no containers, no network — the provider twin is the in-process `httpmock` loopback server. |
| 10 | `verify.command` vs `verify.e2e_command`, isolation proven | Distinct strings, and the counts prove isolation: 279 regular scenarios vs 93 e2e, the same separation as before this change. |
| 11 | Implementation vs `design.md` deviations | Recorded, not silent: `design.md` carries D1, D2, and the architecture table naming both test-support files this change touches; `design-review.md` records the operator's D2 addition, the pre-applied modifications, and the evidence-transform hardening. The per-scenario commit rule is deviated from and recorded in `tasks.md` ("Deviation: implementation precedes this task list") with the reason: both rules share one delta feature file, so no per-scenario split stays green. |
| 12 | Findings reopened as work | The visual-evidence transform was found wanting at review time (the progress line shares the header's row, and the filter dropped the model label and the amount): fixed in the harness and the two transcripts were re-committed in `aebdd80`. `docs/arc42/05-building-block-view.md` was stale for both Amount rows and is corrected in `e1d2848`. |
| 13 | Interaction coverage cross-reference | See the table below. |
| 14 | Coverage measurement validity | Valid. `./measure-coverage.sh` instruments the default and test-support binaries and the runner, writes `%p-%m` collision-safe profiles, and `./merge-coverages.sh` merges the two scope reports into `coverage/cobertura-coverage.xml` (freshly produced: 21039/22654 lines, 92.9%). `src/amount.rs` reads 116/116 lines, a known exercised production path; the changed lines are classified below. |

### Interaction coverage cross-reference

| Inventory entry | `@e2e` scenario title | Real interface | Driving mechanism | Verified |
|---|---|---|---|---|
| ask interactively and read the billed amount in the review surface | The review surface shows the billed amount of the request that produced it | CLI / terminal | Real `watn` subprocess on a `portable-pty` terminal; the modified scenario now expects `1 ¢` and commits this change's transcript | Clean — `tests/steps/observe_request_cost_e2e_steps.rs` |
| explain an existing command and read the explanation request's billed amount | The explanation card shows the billed amount of its explanation request | CLI / terminal | Real `watn explain` subprocess on a `portable-pty` terminal; the modified scenario expects `1 ¢` and commits this change's transcript | Clean — same file |

The `ask` fragment's new scenario (`The metadata line prices a request from the
requested model when the reported model has none`) drives the real binary as a
regular subprocess and asserts its stderr; it adds no inventory entry, because
the interaction — submitting a question and reading the metadata line — already
belongs to `ask`'s own inventory.

## Arc42 implementation conformance

| Chapter / fact | Durable source | Implementation evidence | Result |
|---|---|---|---|
| 05 `Amount` names the price lookup and the shipped form | `docs/arc42/05-building-block-view.md` | `src/amount.rs`: `recorded_price`, `billed_amount`, `cents_text` | Match |
| 08 cost tracking: reported-model first, requested-model fallback, usage presence, the cents form, stderr divergence | `docs/arc42/08-crosscutting-concepts.md` | the four `recorded_price` call sites; `usage_reported`; the metadata line's `$0.0000` (permanent scenario unchanged) | Match |
| 11 R-092 residual absence, R-093 the fallback's routing caveat, R-094 the four-decimal floor | `docs/arc42/11-risks-and-technical-debt.md` | the fallback in `recorded_price`; the cap in `cents_text`, asserted by `four_decimal_places_is_the_smallest_step_shown` | Match |
| 12 glossary `Amount` (price rule and form), `Model label` anti-term | `docs/arc42/12-glossary.md` | spec, design, and code reuse both terms; `ReviewContext.amount`, `header_right` | Match |
| 04, 06, 10 unchanged by this change | `docs/arc42/04-solution-strategy.md`, `06-runtime-view.md`, `10-quality-requirements.md` | the flows, surfaces, and quality scenarios are untouched; only the price source and the digits change | Match |

ARC42 CONFORMANCE: CLEAN

## Ubiquitous language conformance

| Term | Glossary | Specs / design | Code |
|---|---|---|---|
| Amount | present, amended in this change with the price rule and the rendering form | used in the use case rules and examples, the delta scenarios, and the design | `amount::BilledAmount`, `ReviewContext.amount`, `cents_text` |
| the price that applies to a request | named in the Amended `Amount` definition and the use case rule | the rule the scenarios assert | `amount::recorded_price` |
| Model label | present, amended anti-term (`priced from first`) | used in the scenarios and the design | `ReviewContext.model` + `header_right` |
| Model short name, Review surface, Candidate, Stage purpose | present, unchanged | reused unchanged | unchanged |

No term is used without a glossary record; no glossary term is reused
inconsistently. The rendering form has no separate term — the glossary's
`Amount` states it, which keeps one vocabulary for one concept.

## Visual review

- Interface: terminal. Evidence: this change's own transcripts under
  `givn/changes/amount-from-requested-model/evidence/visual/<scenario-slug>/transcript.txt`,
  re-committed in `aebdd80` so the predecessor's frozen evidence is not
  overwritten.
- The review-surface transcript shows `◆ gpt-4o-mini · 1 ¢` on the header with
  the stage stack, `purpose-unavailable`, and the decision hints; the
  explanation-card transcript shows `◆ tier 1 · test/gpt-4o-mini · 1 ¢` with
  both stage purposes and the close hint.
- Baseline holds: every control has a visible label, no internal identifier
  appears in the visible text, nothing is duplicated, and the hints name their
  keys.
- Limitation, stated rather than hidden: the two `@e2e` scenarios both produce a
  whole-cent amount, so the transcripts show the whole-cent form only. The
  sub-cent and smallest-step forms (`0.02`, `0.0009`) are proven by the
  in-process scenarios and the unit tests, not by a transcript. The alternative
  — repricing the e2e scenarios to reach a sub-cent amount — was rejected
  because it would weaken the whole-cent rounding coverage that no other
  scenario carries.
- A vision subagent is not applicable to terminal transcripts; the evidence is
  text and was read directly.

PERSONAS: terminal-developer--interactive

VISUAL-REVIEW: PASS

## Coverage classification

Merged union over the non-e2e and e2e scopes, restricted to the lines this
change adds (`git diff 261effc..HEAD -- src/`, instrumented lines only):

| Region | Uncovered changed lines | Bucket | Resolution |
|---|---|---|---|
| `src/amount.rs` | none — 53 instrumented added lines, all covered | — | — |
| `src/main.rs` | 1 line (`src/main.rs:667`) | 3 — legitimately hard to test | Same line and same justification the predecessor change recorded |
| `tests/steps/**` | test-support code, not production | — | — |

Justification for bucket 3 at `src/main.rs:667`: the line is the Amount passed
into the explanation card opened from the `-x` execution confirmation. It is
reachable only when a real `wt -x` invocation shows the confirmation and the
developer answers with the explanation choice while the review surface is
disabled, so the prompt and the card must both be driven inside one interactive
session. The prompt's decision is covered by the permanent scenario
`The -x confirmation offers to explain the command`, and the card's amount
rendering is covered by the two money scenarios; the line only passes the same
computed amount into that same covered card. The predecessor change recorded
the identical justification and the identical follow-up: an end-to-end scenario
for the confirmation prompt's explanation choice would remove even this.

No gap was resolved as missing coverage at review time; no dead code was found
in the changed regions.

## Overlap dispositions

`givn lint --change amount-from-requested-model` exits 0 and reports 19 SHAPE
and 41 SUBST advisories. Every one of them falls into the classes below; the
counts add up to the reported totals.

| Class | Pairs | Disposition |
|---|---:|---|
| Delta scenario × delta scenario, same step shape | 6 SHAPE | Boundary: the nine amount scenarios share a shape because the amount is asserted the same way, but each asserts a different invariant — a first display through a different price source, replacement on regeneration, the genuinely-zero case, the whole-cent rounding on the real terminal, and the distinct label/layout of each view. Merging any pair would lose a rule. |
| Permanent scenario × its `@givn.modified` copy | 13 SHAPE, 24 SUBST | Supersession, not duplication: the delta copy is the same scenario with the new expectation; it replaces the permanent text at the merge, and both run identically while the change is open because the runner executes permanent and delta specs together. |
| Delta scenario × any other permanent amount scenario | 12 SUBST | Boundary, as in the first row: subset advisories fire because every amount scenario ends at the same assertion, which is the assertion, not the invariant. |
| `A provider's unknown model falls back to the requested model's recorded price` × `The billed model's amount is shown under the requested model name` | — | Boundary: the first proves the price *source* when the reported model has no entry at all (an alias), the second proves the label keeps the requested name while the amount comes from a *priced* reported model. Neither rule implies the other. |
| Delta metadata scenario × `A usage-only final event supplies cost and throughput metadata` | 1 SUBST | Boundary: the ask scenario proves the metadata line's price source for a reported model with no recorded price; the incremental-SSE scenario proves the metadata line's behaviour for a usage-only final event with a recorded price. Different rules, same line. |
| Explanation-card modification × `use-shell` explain scenarios | 3 SUBST | Boundary: the modified scenario adds the billed amount to a card whose stages, purposes, escaping, and closing behaviour are asserted elsewhere; it does not re-state them. |

No `@givn.removed` scenario exists in this change, so there is no removed+added
pair to explain as supersession or unrelated work. No two scenarios were
removed and re-added: the eight modifications replace their own titles.

## Split-or-keep

No scenario exceeds the deterministic long-scenario threshold; the longest new
or modified scenario carries seven steps.

## Persona conformance

- Confirmed Persona: `terminal-developer--interactive` (cost stance,
  2026-09-24). Disposition: satisfied — D1 makes the amount appear for the
  configuration the README recommends, and D2 makes it readable at a glance;
  the developer sees the money inside the surface that asks for the next
  decision.
- Actors in Gherkin remain Terminal developer, Watn, and Provider; no Persona
  biography appears in a scenario.
- Interface classification, stated for the record as the opposite first: a
  screen that renders HTML/CSS/JS in a browser would be a Web UI needing a
  browser driver; this change renders no browser content, its only interface is
  the terminal, and its end-to-end evidence is a real subprocess on a
  pseudo-terminal.

## README impact

README-IMPACT: updated - Usage

The review-surface paragraph now states the rendering form (`0.1`, `0.02`,
`0.0009`, whole cents from one cent on) and the price that applies to a request,
including the requested-model fallback and the residual silence. Committed in
`e1d2848`.

## Sign-off

- [x] Fabrication audit clean
- [x] Both runners green: 279 regular scenarios (1712 steps) and 93 e2e scenarios (705 steps)
- [x] `@e2e` isolation proven by scenario counts
- [x] Coverage measured, merged, and every changed region classified
- [x] ARC42 CONFORMANCE: CLEAN
- [x] PERSONAS: terminal-developer--interactive
- [x] VISUAL-REVIEW: PASS with committed transcripts
- [x] Persona disposition recorded
- [x] README-IMPACT recorded and the README edited
- [x] No `@wip` left in scope; no `@e2e` tag removed
- [x] `givn lint --change amount-from-requested-model` exits 0; its 19 SHAPE and 41 SUBST advisories are dispositioned above

REVIEW: PASS
