# Review: observe-request-cost

## Step 0 — fabrication audit

| # | Check | Result |
|---|---|---|
| 0 | `@e2e` tag integrity in the delta specs | Clean. `givn/changes/observe-request-cost/specs/observe-request-cost/observe-request-cost.feature` carries two `@e2e` scenarios and `specs/use-shell/interactive-shell-shortcut.feature` carries the modified `@e2e` scenario; no scenario lost an `@e2e` tag at any point. |
| 1 | Empty or no-op step bodies | Clean. `grep -n "{}"` over both new step files → 0; `grep -rn "unimplemented!\|todo!()"` → 0. Every body asserts, configures the scenario, or reads the rendered surface. |
| 2 | Every `[x]` task has a commit touching production source | Clean. `b31abe0` touches `src/amount.rs`, `src/lib.rs`, `src/review/*`, `src/main.rs`; `48bbfb5` and `3a853a1` touch the step files and harness; `db227a2` touches `src/main.rs` (the explanation-path amount) and the e2e steps. The planning commit `560464d` carries only artifacts, and no task claims it. |
| 3 | Components `design.md` promised exist | Clean. `src/amount.rs` with `BilledAmount`, `billed_amount`, `cents_text`; `ReviewContext.amount`; budget-aware `header_right`; the plumbing table's sites (`run_review_path`, `apply_regeneration`, `run_explanation_card`); `tests/steps/observe_request_cost_steps.rs` and `tests/steps/observe_request_cost_e2e_steps.rs`. |
| 4 | Proof of strictness present with non-zero exit | Present in `tasks.md` (T1): the `unimplemented!("TODO")` stub produced exit status 1. |
| 5 | Every `@e2e` scenario asserts on the real interface | Clean. All three assert on the pseudo-terminal's rendered text (`pty_snapshot`) or the shell buffer the widget prints; none asserts against a repository or database. |
| 6 | Browser-UI step fidelity | Not applicable: the interface is a terminal, and no browser or HTTP client is used as a driver. |
| 7 | `verify.e2e_command` target and duplicate implementations | `./run-tests.sh --e2e` → `cargo test --test features_runner -- --tags "@e2e and not @wip"`. Searched the tree: one e2e implementation per step, in `tests/steps/observe_request_cost_e2e_steps.rs` and the shared harness. |
| 8 | E2E scope: one e2e scenario per normalized action | Clean. Two inventory entries, two `@e2e` scenarios; the other eleven scenarios are regular. |
| 9 | Local run command starts the stack including twins | Clean. `./run-tests.sh` and `./run-tests.sh --e2e`; no containers and no network; the provider twin is the in-process `httpmock` loopback server. |
| 10 | `verify.command` vs `verify.e2e_command`, isolation proven | Distinct strings, and the counts prove isolation: 270 regular scenarios vs 92 e2e. |
| 11 | Implementation vs `design.md` deviations | One deviation, recorded rather than silent: the in-process scenarios S2–S11 share one commit instead of one commit per scenario, because their steps and harness fields were authored in one pass (recorded in `tasks.md`, "In-process batch"). `design.md` was updated during implementation for the `cents_text` refinement, and `design-review.md` records it. |
| 12 | Findings reopened as work | One finding (the too-narrow fallback with no scenario, bucket 2) was reopened as S15 with its own RED/GREEN/COMMIT (`3a853a1`). |
| 13 | Interaction coverage cross-reference | See the table below. |
| 14 | Coverage measurement validity | Valid. `./measure-coverage.sh` instruments the binaries and the runner, writes collision-safe profiles, and `./merge-coverages.sh` merges the two scope reports; `src/amount.rs` shows 82/82 lines covered in the merged union, a known exercised production path. |

### Interaction coverage cross-reference

| Inventory entry | `@e2e` scenario title | Real interface | Driving mechanism | Verified |
|---|---|---|---|---|
| ask interactively and read the billed amount in the review surface | The review surface shows the billed amount of the request that produced it | CLI / terminal | Real `watn` subprocess on a `portable-pty` terminal: PTY write of the question, wait for the card, read the rendered header | Clean — `tests/steps/observe_request_cost_e2e_steps.rs` |
| explain an existing command and read the explanation request's billed amount | The explanation card shows the billed amount of its explanation request | CLI / terminal | Real `watn explain` subprocess on a `portable-pty` terminal: command as one argument, `pty_wait_for_label`, read the rendered header | Clean — same file |

## Arc42 implementation conformance

| Chapter / fact | Durable source | Implementation evidence | Result |
|---|---|---|---|
| 03 review-surface boundary names the Amount | `docs/arc42/03-context-and-scope.md` | the card header renders `◆ model · amount ¢` | Match |
| 05 `Amount` building block | `docs/arc42/05-building-block-view.md` | `src/amount.rs`: `BilledAmount`, `billed_amount`, `cents_text` | Match |
| 06 review and explain sequences show the Amount | `docs/arc42/06-runtime-view.md` | `run_review_path` and `run_explain_command` both fill `ReviewContext.amount` | Match |
| 08 cost tracking: one computation, usage presence, cents form, stderr divergence | `docs/arc42/08-crosscutting-concepts.md` | `billed_amount` returns `usage_reported`; the metadata line still prints `$0.0000` (permanent scenario unchanged) | Match |
| 11 R-092 silent absence | `docs/arc42/11-risks-and-technical-debt.md` | Q2's silence is implemented and covered by S2/S3 | Match |
| 12 glossary `Amount`, `Model label`, `Simple review view`, `Model short name` | `docs/arc42/12-glossary.md` | the specs, the design, and the code use those terms; `ReviewContext.amount`, `model_short_name` | Match |
| 04, 10 review presentation carries the amount | `docs/arc42/04-solution-strategy.md`, `10-quality-requirements.md` | the simple and detailed headers both render it (S7) | Match |

ARC42 CONFORMANCE: CLEAN

## Ubiquitous language conformance

| Term | Glossary | Specs / design | Code |
|---|---|---|---|
| Amount | present | used in the use case, the rules, and the design | `amount::BilledAmount`, `ReviewContext.amount`, `cents_text` |
| Model label | present | used in the rules and the design | `ReviewContext.model` + `header_right` label |
| Model short name | present | used in the design and the scenarios | `model_short_name` |
| Simple review view | amended to carry the Amount | the review goal's rule amended in this change | `header_right(detailed = false)` |
| Review surface, Candidate, Stage purpose, Command flow | present, unchanged | reused unchanged | unchanged |

No term is used without a glossary record; no glossary term is reused inconsistently.

## Visual review

- Interface: terminal. Evidence: one transcript per `@e2e` scenario under
  `givn/changes/observe-request-cost/evidence/visual/<scenario-slug>/transcript.txt`.
- The review-surface transcript shows `◆ gpt-4o-mini · 1.35 ¢` on the header
  with the stage stack, `purpose-unavailable`, and the decision hints; the
  explanation-card and Ctrl-W transcripts likewise show the amount and, for the
  Ctrl-W scenario, the accepted buffer.
- Baseline holds: every control has a visible label, no internal identifier
  appears in the visible text, nothing is duplicated, and the hints name their
  keys.
- A vision subagent is not applicable to terminal transcripts; the evidence is
  text and was read directly.

PERSONAS: terminal-developer--interactive

VISUAL-REVIEW: PASS

## Coverage classification

Merged union over the non-e2e and e2e scopes, classified against the lines this
change added (`git diff 560464d..HEAD -- src/`):

| Region | Uncovered changed lines | Bucket | Resolution |
|---|---|---|---|
| `src/amount.rs` | none (82/82 lines) | — | — |
| `src/review/card.rs` | none | — | — |
| `src/review/panel.rs` | none | — | — |
| `src/main.rs:667` | 1 line | 3 — legitimately hard to test | Justification below |
| `src/review/card.rs` fallback (found at review) | 1 line | 2 — missing coverage | Resolved: S15 added, RED/GREEN/COMMIT `3a853a1`; the fallback is now exercised |

Justification for bucket 3 at `src/main.rs:667`: the line is reachable only when
a real `wt -x` invocation shows the execution confirmation and the developer
answers `?` while the review surface is disabled, so the prompt and the
explanation card must both be driven inside one interactive session. The
prompt's decision is covered at its decision function by the permanent scenario
`The -x confirmation offers to explain the command`, and the card's amount
rendering is covered by the two end-to-end scenarios; the line passes the same
computed amount into that same covered card, so it carries no behaviour of its
own. Recorded follow-up: an end-to-end scenario for the confirmation prompt's
explanation choice would remove even this.

## Overlap dispositions

| Scenario A | Scenario B | Disposition |
|---|---|---|
| The review surface shows the billed amount of the request that produced it | A rejected candidate's replacement carries its own billed amount | boundary |
| The review surface shows the billed amount of the request that produced it | A reported zero usage shows a zero amount | boundary |
| The review surface shows the billed amount of the request that produced it | The detailed view keeps the billed amount at the model name | boundary |
| A rejected candidate's replacement carries its own billed amount | A reported zero usage shows a zero amount | boundary |
| A rejected candidate's replacement carries its own billed amount | The detailed view keeps the billed amount at the model name | boundary |
| A reported zero usage shows a zero amount | The detailed view keeps the billed amount at the model name | boundary |

All six pairs share a step shape because the amount is asserted the same way,
but each asserts a different invariant: a first display through the real binary,
replacement on regeneration, the genuinely-zero case that a non-zero assertion
must reject, and the detailed layout with its own label and reservation. None is
a variant of another; merging any pair would lose a rule.

## Split-or-keep

No scenario exceeds the deterministic long-scenario threshold.

## Persona conformance

- Confirmed Persona: `terminal-developer--interactive` (cost stance,
  2026-09-24). Disposition: satisfied — the amount reaches the developer inside
  the surface that asks for the next decision, at the model label, in both
  views; the silent-absence tension is recorded in the persona review.
- Actors in Gherkin remain Terminal developer, Watn, and Provider; no Persona
  biography appears in a scenario.
- Interface classification, stated for the record as the opposite first: a
  screen that renders HTML/CSS/JS in a browser would be a Web UI needing a
  browser driver; this change renders no browser content, its only interface is
  the terminal, and its end-to-end evidence is a real subprocess on a
  pseudo-terminal.

## README impact

README-IMPACT: updated - Usage

The review-surface and explain subsections now name the billed amount, its
cents form, the silence rule, and the shortening rule.

## Sign-off

- [x] Fabrication audit clean
- [x] Both runners green: 270 regular scenarios (1647 steps) and 92 e2e scenarios (701 steps)
- [x] `@e2e` isolation proven by scenario counts
- [x] Coverage measured, merged, and every changed region classified
- [x] ARC42 CONFORMANCE: CLEAN
- [x] PERSONAS: terminal-developer--interactive

VISUAL-REVIEW: PASS with committed transcripts
- [x] Persona disposition recorded
- [x] README-IMPACT recorded and the README edited
- [x] No `@wip` left in scope; no `@e2e` tag removed

REVIEW: PASS
