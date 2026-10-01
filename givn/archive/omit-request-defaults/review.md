# Review: omit-request-defaults

## Step 0 — fabrication audit

| # | Check | Result |
|---|---|---|
| 0 | `@e2e` tag integrity in the delta specs | Clean. The delta scenario never carried `@e2e` in tracked state: it was drafted with `@e2e` while untracked, then design-review Q1 demoted it to a regular scenario because the canonical specs policy classifies a provider-rejection error variant of the existing ask action as a regular scenario. The decision is recorded in `design-review.md` (Q1) and `design.md`; the permanent `ask` `@e2e` coverage is untouched. No `@e2e` tag was removed from any committed revision. |
| 1 | Empty or no-op step bodies | Clean. `grep -rn "unimplemented!\|todo!()" tests/steps src` → 0 matches. The one new step (`tests/steps/ask_steps.rs:25`) sets `pending_mock_no_generation_defaults_assert`, a real harness setup action; the new helper (`tests/steps/mod.rs:156`) registers real blocking HTTP twins with a 400 response. |
| 2 | Every `[x]` task has a commit touching production source | Clean. `eb97fcc` touches `src/provider/mod.rs`, `src/provider/openai_compat.rs`, and `src/main.rs` (field and body-key removal) plus the test harness; `fbfda6c` records the revision in `tasks.md`. No checked task claims a spec-only commit. |
| 3 | Components `design.md` promised exist | Clean. `RequestOptions` without `temperature`/`max_tokens` (`src/provider/mod.rs:14-18`); body built without the two keys (`src/provider/openai_compat.rs:51-60`); `pending_mock_no_generation_defaults_assert` (`tests/features_runner.rs`); `setup_generation_default_blockers` in `tests/steps/mod.rs`; the Given in `tests/steps/ask_steps.rs`. |
| 4 | Proof of strictness present with non-zero exit | Present in `tasks.md` (T1): the undefined step produced exit status 1 (`Step doesn't match any function`). |
| 5 | Every `@e2e` scenario asserts on the real interface | No delta `@e2e` scenario exists. The delta scenario is regular and drives the real binary subprocess through the CLI (`run_binary_with_state`) and asserts its exit status and stdout. |
| 6 | Browser-UI step fidelity | Not applicable: the interface is a terminal CLI; no browser or HTTP client is used as a driver. The provider is reached by the binary, not by the step. |
| 7 | `verify.e2e_command` target and duplicate implementations | `./run-tests.sh --e2e` → `cargo test --test features_runner -- --tags "@e2e and not @wip"`. The tree contains one ask step file (`tests/steps/ask_steps.rs`); no parallel or untracked second implementation exists. |
| 8 | E2E scope: one e2e scenario per normalized action | Clean for this change: it adds no inventory entry and no `@e2e` scenario; the rejecting provider is an error variant of the existing ask action. The pre-existing ten ask `@e2e` scenarios are permanent-corpus debt outside this change's scope. |
| 9 | Local run command starts the stack including twins | Clean. `./run-tests.sh` and `./run-tests.sh --e2e` build both debug binaries and run the whole system with no containers and no external network; the provider twin is the in-process `httpmock` loopback server. |
| 10 | `verify.command` vs `verify.e2e_command`, isolation proven | Distinct strings (`./run-tests.sh` vs `./run-tests.sh --e2e`), and the direct runs prove isolation: 275 regular scenarios vs 92 e2e scenarios — strictly smaller. |
| 11 | Implementation vs `design.md` deviations | None. The Given lives in `tests/steps/ask_steps.rs`, the blocking twins are registered before the answering mock, and no new e2e file was added, exactly as `design.md` names. The helper is also called in the third fallback mock-setup site, a superset of the "both mock-setup branches" requirement that changes no reviewed decision. |
| 12 | Findings reopened as work | None; the review found no fabrication finding. |
| 13 | Interaction coverage cross-reference | See the table below. |
| 14 | Coverage measurement validity | Review does not run an instrumented pass (the archive receipt carries the measured result). Static inspection: `./measure-coverage.sh` instruments both binaries and the `features_runner`, writes collision-safe profiles (`coverage/profraw/%p-%m.profraw`), and `./merge-coverages.sh` merges the non-e2e and e2e scope reports; the changed production regions (body construction in `src/provider/openai_compat.rs`, the removed fields, the three call sites) are exercised by the delta scenario and by the permanent provider scenarios. No measurement receipt exists yet for this change. |

### Interaction coverage cross-reference

The `ask` capability lives in the corpus-infra fragment, whose owning document
(`givn/specs/fragments/fragment.md`) records `Interactions: none`; the permanent
ask feature carries its own `@e2e` scenarios. This change adds no inventory
entry and no `@e2e` scenario.

| Inventory entry | `@e2e` scenario title | Real interface | Driving mechanism | Verified |
|---|---|---|---|---|
| none added — the rejecting provider is an error variant of the existing ask action | none added (regular scenario: Ask succeeds against a provider that rejects generation parameters) | CLI / terminal | Real `watn "list go files"` subprocess (`run_binary_with_state`) against an `httpmock` loopback twin that returns HTTP 400 for bodies carrying `temperature`, `max_tokens`, or `max_completion_tokens` | Clean — `tests/steps/ask_steps.rs` + `tests/steps/mod.rs` |

## Arc42 implementation conformance

| Chapter / fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| Provider request carries only model, messages, stream, and configured reasoning | `docs/arc42/05-building-block-view.md` (`OpenAICompatibleProvider` row) | Chapter 05 affected | Data model section | S1 GREEN | `src/provider/openai_compat.rs:51-63`; no `temperature`/`max_tokens` keys | Match |
| Provider request-defaults rule | `docs/arc42/08-crosscutting-concepts.md` (`## Provider request defaults`) | Chapter 08 affected | Overview and Decisions | S1 GREEN | `RequestOptions` has no temperature/token-limit fields (`src/provider/mod.rs:14-18`) | Match |
| R-091 mitigation no longer a scoped 4096 cap | `docs/arc42/11-risks-and-technical-debt.md` | Chapter 11 affected | Decisions (D1) | S1 GREEN | `main.rs` review/explanation/regeneration sites no longer set `max_tokens` | Match |
| R-097 provider-default truncation risk | `docs/arc42/11-risks-and-technical-debt.md` | Chapter 11 affected | Decisions (D2, out of scope) | S1 GREEN | unusable-response recovery unchanged (`src/review/response.rs`) | Match |
| `Provider default` term | `docs/arc42/12-glossary.md` | Chapter 12 affected | Overview | S1 GREEN | specs, design, and chapter 08 use the term consistently | Match |

ARC42 CONFORMANCE: CLEAN

## Ubiquitous language conformance

| Term | Glossary | Specs / design | Code |
|---|---|---|---|
| Provider default | present (`docs/arc42/12-glossary.md`) | used in the proposal, design, and chapter 08 | no Watn fallback exists; omitted keys let the provider decide |
| Request options | existing concept, unchanged name | design names `RequestOptions` | `src/provider/mod.rs` |
| Reasoning effort | present, unchanged | reasoning field behavior unchanged | `reasoning_effort` still conditionally inserted |

No term is used without a glossary record; no glossary term is reused
inconsistently.

## Visual review

- Interface: terminal. The change renders no new or visibly changed screen:
  no delta `@e2e` scenario exists, so the terminal-transcript rule is
  vacuously satisfied, and no transcript is committed.
- Opposite classification tested for the record: a screen that renders HTML,
  CSS, or JavaScript in a browser would be a Web UI needing a browser driver;
  this change renders no browser content and its only observable surface is
  the existing command output, so it is a terminal CLI change.
- No visual evidence exists to review because no rendered state changed; the
  persona's goal here is behavioural (the request succeeds on a model that
  rejects the removed parameters) and is proven by the regular scenario.

PERSONAS: terminal-developer--interactive

VISUAL-REVIEW: PASS

## Coverage classification

No measured receipt exists yet for this change; the archive receipt carries the
merged measurement. Classification from inspection of the changed regions:

| Region | Uncovered changed lines | Bucket | Resolution |
|---|---|---|---|
| `src/provider/mod.rs` (field removal) | none — fewer lines | — | — |
| `src/provider/openai_compat.rs` (body construction) | none — exercised by every provider scenario, including the delta scenario | — | — |
| `src/main.rs` (three call sites) | none — ask/review, explanation, and regeneration paths are covered by permanent scenarios and all use the one builder | — | — |
| test harness (`tests/**`) | none — exercised by the delta scenario | — | — |

No dead code was added; no coverage gap was found to classify under buckets 2
or 3. The removed lines are gone, not uncovered.

## Overlap dispositions

The deterministic gate reports no shape-match finding for this change: the one
added scenario does not share an assertion shape with any permanent scenario
because it asserts success under a rejecting-provider precondition that no
permanent scenario establishes. No disposition rows are required.

## Split-or-keep

No scenario exceeds the deterministic long-scenario threshold.

## Persona conformance

- Confirmed Persona: `terminal-developer--interactive`. Disposition:
  satisfied — the developer can now ask against a model that accepts only its
  provider defaults, which previously failed with a provider error.
- Actors in Gherkin remain Terminal developer, Watn, and Provider; no Persona
  biography appears in a scenario.
- Interface classification is terminal, with the opposite classification
  stated and rejected in the Visual review block.

## README impact

README-IMPACT: none

The change removes request parameters a consumer never configures and adds no
command, flag, configuration key, or output. The README documents none of the
removed fields, and the public Rust API break is recorded in the changelog via
the breaking revision, not in the README.

## Sign-off

- [x] Fabrication audit clean
- [x] Every checked task has a verified commit touching production code
- [x] Every promised component exists
- [x] Strict-mode proof present and passing
- [x] Both runners green by direct execution during this change: 275 regular scenarios (1678 steps) and 92 e2e scenarios (699 steps)
- [x] `@e2e` isolation proven by scenario counts (275 vs 92)
- [x] Coverage classification complete; no bucket 2 or 3 gap found
- [x] Dead code deleted (the removed fields and body keys)
- [x] No `@wip` tags remain; no implementation-layer detail in the spec
- [x] Canonical E2E policy applied: the delta scenario is a variant, not an inventory action
- [x] No browser UI; no downgraded e2e scenario exists
- [x] Local run command starts the full stack including the provider twin
- [x] `verify.e2e_command` identified and read; no parallel e2e implementation exists
- [x] Implementation matches `design.md`; no silent deviation
- [x] Interaction coverage cross-reference clean
- [x] No finding excused with a classification outside the three buckets

REVIEW: PASS