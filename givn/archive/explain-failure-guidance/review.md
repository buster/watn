# Review: explain-failure-guidance

## Fabrication Audit (do this first)

**Step definition scan** — every step definition file checked for empty/no-op
bodies (`{}`, bare `pass`, bare `return`, no assertion reference):

```
grep -nE "fn .*\{\s*\}|unimplemented!|todo!" tests/steps/explain_command_steps.rs tests/steps/explain_command_e2e_steps.rs
-> 0 empty step bodies and 0 unimplemented!() stubs across the two files.
Every new step asserts, drives the real binary/PTY, configures a mock or
fixture, or reuses an existing proven step.
```

**Commit cross-check** — every checked-off `[x]` task in tasks.md has a
matching commit that touches production source (not just spec/stub files):

| Scenario | Commit hash | Touches production code? |
|---|---|---|
| S1 Enter closes | `c333742` | Yes: readiness/outcome plumbing in `src/main.rs` |
| S2 metacharacters | `de2a82e` | Test-only; production from `c333742` |
| S3 option terminator | `55e5123` | Test-only; production from `c333742` |
| S4 stdin quoting | `4541b8a` | Test-only; production from `c333742` |
| S5 piped without marker | `c75bb93` | Test-only; production from `c333742` |
| S6 positional precedence | `80e8d8f` | Test-only; production from `c333742` |
| S7 switches inert | `f90581b` | Test-only; production from `c333742` |
| S8 review disabled | `d8e6da8` | Test-only; production from `c333742` |
| S9 no usable credential | `0104fc0` | Test-only; setup delegation from `c333742` |
| S10 failed explanation | `aeba739` | Yes: request-failure outcome, diagnostic, exit status (`src/main.rs`, `src/review/session.rs`, `src/review/response.rs`) |
| S11 network failure | `6f943bc` | Test-only; exit mapping from `aeba739` |
| S12 unusable response | `1b3440d` | Yes: `apply_explanation_outcome` + wording map (`src/review/response.rs`, `src/main.rs`) |
| S13 quick setup | `1be1e10` | Yes: setup delegation (`src/main.rs`) |
| S14 setup wizard | `9edccba` | Yes: wizard delegation (`src/main.rs`) |
| S15 interrupt | `540adc2` | Yes: pre-card interrupt check (`src/main.rs`) |
| S16 stdin guidance | `746d936` | Test-only; guidance path from `c333742` |
| S17 explicit provider errors | `c2b0189` | Test-only; strict resolution from `c333742` |
| S18 explicit model error | `53b1412` | Test-only; strict resolution from `c333742` |
| Final style/evidence | `9df4743` | Yes: `src/main.rs` (rustfmt only) |

No commit touches only the `.feature` file or an empty stub. Test-only
commits carry real step implementations against production code delivered in
`c333742`, `aeba739`, `1b3440d`, `1be1e10`, `9edccba`, and `540adc2`.
S8 recorded an immediate GREEN (exit 0) instead of a failing RED: its
permanent copy already seeds a ready provider via the persisted-panel fixture,
so the readiness gate never triggers; this is documented in tasks.md S8 and is
a legitimate all-steps-reused GREEN, not a fabricated RED.

**Promised-architecture cross-check** — every component design.md said would
be created actually exists:

| Component (from design.md) | Exists in tree? | Path |
|---|---|---|
| `ExplanationOutcome` + `apply_explanation_outcome` | Yes | `src/review/response.rs`, exported in `src/review/mod.rs` |
| `explain_command_candidate` reporting `Result<ExplanationOutcome, Error>` | Yes | `src/review/session.rs` |
| Readiness delegation + diagnostics + interrupt check in `run_explain_command` | Yes | `src/main.rs` |
| Explain-domain reason wording (no anti-terms) | Yes | `src/main.rs`, `src/review/response.rs` |
| Permanent-spec lockstep for the modified scenarios | Yes | `givn/specs/use-shell/explain-command.feature` |
| New regular steps | Yes | `tests/steps/explain_command_steps.rs` |
| Shared wizard-step adaptation | Yes | `tests/steps/setup_wizard_steps.rs` |

**Strict-mode proof** — confirmed present and passing in tasks.md setup task:

```
./run-tests.sh --name "Strictness probe scenario"
 ✘ Given a step that is not defined anywhere in this suite
   Step doesn't match any function
[Summary] 1 scenario (1 failed), 1 step (1 failed)
```

**Downgraded e2e scenario scan** — every `@e2e` scenario's Then steps
checked for a primary assertion on the observable output, return value,
acknowledgement, or visible state of the interface named in design.md, not
repository-only; for a browser-UI capability, the step definitions checked
for actually driving the browser (click/type/navigate), not an HTTP request or
in-page `fetch()`:

| @e2e Scenario | Real-interface assertion present? | Driven via real browser (if UI)? | Downgraded? |
|---|---|---|---|
| Developer explains an existing command in the review card (permanent, unchanged by this delta) | Yes — real PTY transcript assertions on the card | N/A (CLI) | No |

The delta introduces no new `@e2e` scenario; the design documents that the
readiness delegation is a precondition variant of the same consumer action
whose real-interface evidence is the permanent `@e2e` scenario.

**E2e scope check** — each capability has one `@e2e` scenario per distinct
happy-path action, not one per variant/error case:

| Capability | @e2e scenario count | Distinct actions? | Over-produced? |
|---|---|---|---|
| explain-command | 1 (permanent; delta adds none) | Yes — explain an existing command in the review card | No |

**Duplicate-implementation check** — the exact file(s) `verify.e2e_command`
invokes, and confirmation the tree (including untracked files, via `git
status`) was searched for any other/parallel e2e implementation of the same
capability that might be stronger than the one being reviewed:

| Capability | File(s) verify.e2e_command invokes | Other implementation found? (incl. untracked) | Stronger one used instead? |
|---|---|---|---|
| explain-command | `./run-tests.sh --e2e` -> `cargo test --test features_runner --tags '@e2e and not @wip'`; collects `tests/steps/explain_command_e2e_steps.rs` | No — `grep -rn "watn explain" tests/steps` finds only the explain-command step files; `git status` clean | N/A |

**Local environment check** — the local run command (design.md's Local
Runnability section) starts the full stack, including digital twins,
cleanly:

```
cargo run --release --locked -- --help -> exit 0, release profile finished cleanly
The product is a one-shot CLI; the only external service (the
OpenAI-compatible endpoint) is replaced by the in-process httpmock twin in
every scenario. No container, database, port, or live network is required.
```

**E2e isolation check** — `verify.command` and `verify.e2e_command` read
literally from `givn/commands.yaml`; if identical, this is always a finding
(the "e2e run" is not isolating `@e2e` scenarios at all). Isolation proof
from the two scopes' scenario counts (the archive receipt will carry the
durable record):

| `verify.command` value | `verify.e2e_command` value | Identical? | Regular scope scenario count | E2e scope scenario count | E2e count strictly smaller (or all scenarios @e2e)? |
|---|---|---|---|---|---|
| `./run-tests.sh` | `./run-tests.sh --e2e` | No | 256 | 89 | Yes |

## Use-Case And Persona Conformance

| Contract | Evidence | Pass? |
|---|---|---|
| Owning use-case guarantees and rules preserved | `givn/specs/use-shell/usecase.md`: the Explained command is never generated/edited/accepted/executed; the readiness precondition now names setup/guidance; a single-line extension covers the failed/unusable explanation | Yes |
| Capability ownership and typed relationships preserved | Capability `explain-command` stays owned by `use-shell`; setup surfaces keep their `configure-interactive`/`configure-model` ownership and are only delegated to | Yes |
| Confirmed Persona-relevant outcomes preserved | `terminal-developer--interactive` sees the cause and the next action on every new path (persona review PASS) | Yes |
| Actors remain distinct from Persona biographies | Gherkin uses developer actions; the Persona stays a review lens | Yes |
| Migration ledger and coverage evidence complete, if applicable | No migration in this change | N/A |

**Design.md conformance check** — implementation's actual commands, file
layout, and framework/driver choices diffed against what design.md
explicitly named. Any deviation implemented without first updating
design.md and re-running design-review is a finding, not an acceptable
shortcut:

| Decision (from design.md) | design.md says | Actually built | Matches? | If not: was design.md updated + design-review re-run? |
|---|---|---|---|---|
| Readiness delegation | question-path predicates and surfaces; terminal stdin required for quick setup/wizard; guidance + exit 1 otherwise | Identical in `run_explain_command` | Yes | — |
| Explicit selection | `--provider`/`--model`/`WATN_PROVIDER` resolve strictly with existing errors; `watn --provider X explain` syntax | Identical; scenarios use the working top-level syntax | Yes | — |
| Request failure | `explain request failed: <error>` on stderr, card still opens, mapped status after close | Identical | Yes | — |
| Unusable response | `explain response was not usable: <reason>`, explain-domain wording, exit 0 | Identical | Yes | — |
| Interrupt | shared flag re-checked after the fetch and before the card; exit 130, no card | Identical | Yes | — |
| Permanent-spec lockstep | modified permanent scenarios re-based in lockstep | Applied to `givn/specs/use-shell/explain-command.feature` | Yes | — |
| Runner/step files | `./run-tests.sh` / `./run-tests.sh --e2e`; existing per-capability step files | Identical | Yes | — |
| S14 shared wizard step | design names the wizard test pattern | The shared step was adapted to accept the preselected provider; the full e2e suite re-run green | Yes (test harness adaptation, not a product decision) | — |

**Interaction coverage verification** — spec's User Interaction Inventory
cross-referenced against design's Interaction Coverage Matrix, `.feature`
file, and step definitions. Every inventory entry must have a matrix row,
a matching `@e2e` scenario, and step definitions that use the promised
driving mechanism:

| Inventory entry | Matrix row exists? | @e2e scenario exists? (title) | Promised driving mechanism | Actual driving mechanism (from step defs) | Match? |
|---|---|---|---|---|---|
| explain-command / explain an existing command in the review card | Yes (design.md Interaction Coverage Matrix) | Yes — "Developer explains an existing command in the review card" (permanent) | Real `watn` binary in a `portable-pty` session; transcript; redirected stdout | `start_pty_command` + `pty_write`/`pty_snapshot`/`finish_pty_session`; stdout file; marker file | Yes |

No new inventory entry: the readiness delegation is a precondition variant of
the same explain action; the delegated setup interactions are already
inventoried under `configure-interactive` and `configure-model`. This
normalization is recorded in the design's matrix.

## Visual Review

| @e2e Scenario | Viewport | Evidence file | Present and non-empty? |
|---|---|---|---|
| Developer explains an existing command in the review card (unchanged by this delta) | terminal PTY 120x40 | `givn/archive/explain-command/evidence/visual/developer-explains-an-existing-command-in-the-review-card/transcript.txt` | Yes (3,376 bytes) |

| confirmed persona | Goal | Finding | Disposition |
|---|---|---|---|
| `terminal-developer--interactive` | Understand what happened on each new failure/setup path and what to do next | No findings. Paths 1-6 name the cause and the next action (`setup complete; rerun \`watn explain\` with the command`, setup guidance naming `watn setup`, `explain request failed: <error>`, `explain response was not usable: <reason>`, Ctrl+C semantics). The card's rendered content is unchanged; the archived happy-path transcript still satisfies the baseline (frame, `esc close`, `watn` header, no internal identifiers, no duplicates, English). | Accepted |

**Baseline:** every control labeled, no internal identifier visible, a real
header, no duplicated visible content, no error state on the happy path, the
configured language, no motion. All pass.

VISUAL-REVIEW: PASS

## Retirements

N/A — this change retires nothing.

## README impact

README-IMPACT: updated - Usage

The "Explain an existing command" subsection now states that an unready
provider starts quick setup or the setup wizard (or prints setup guidance for
piped input) instead of opening the card, and that a failed request is
reported on stderr while the card still shows the command and
`purpose-unavailable`.

## Arc42 implementation conformance

| Arc42 chapter or architectural fact | Durable-doc source | `arc42.md` claim | `design.md` treatment | `tasks.md` treatment | Completed implementation evidence | Match? / finding |
|---|---|---|---|---|---|---|
| Requirement 40 extended (setup/guidance when no usable model) | `docs/arc42/01-introduction-and-goals.md` | Yes | Scope + behaviour contract | S9, S13, S14, S16 | `run_explain_command` delegation; scenarios pass | Yes |
| Context/interface rows for setup delegation | `docs/arc42/03-context-and-scope.md` | Yes | Setup delegation | S9, S13-S14, S16-S18 | Quick setup / wizard / guidance reuse | Yes |
| Solution strategy (reuse the question-path onboarding rule) | `docs/arc42/04-solution-strategy.md` | Yes | Setup delegation | S9, S13-S14 | Identical predicates and surfaces | Yes |
| CLI responsibility (readiness delegation, diagnostics) | `docs/arc42/05-building-block-view.md` | Yes | Scope | All | `src/main.rs` | Yes |
| Runtime flow with setup, failure, and interrupt branches | `docs/arc42/06-runtime-view.md:276-316` | Yes | Explain runtime flow | S9-S18 | Diagnostics and exit statuses as documented | Yes |
| Error handling cross-cutting concept | `docs/arc42/08-crosscutting-concepts.md:223-233` | Yes | Behaviour contract | S10-S12, S15 | `explain request failed`, `explain response was not usable`, interrupt 130 | Yes |
| QS-079 (onboarding), QS-077 update (failure diagnostics) | `docs/arc42/10-quality-requirements.md:68,152` | Yes | Behaviour contract | S9-S14, S16-S18 | Scenario coverage for each path | Yes |
| R-086..R-088 and consequence coverage | `docs/arc42/11-risks-and-technical-debt.md:88-90,232-238` | Yes | Risks | S13, S10, S12 | Config mutation, exit-status change, and residual ambiguity documented and covered | Yes |
| Glossary: Explanation request | `docs/arc42/12-glossary.md:131` | Yes | Domain language | All | Spec, design, code, README use the term | Yes |
| ADR routing (NOT_QUALIFIED -> design.md) | `arc42.md` `## ADR qualification` | Yes | Rationale owned by design.md | No ADR task | No ADR file; no `docs/arc42/adr/` change | Yes |

- [x] Every chapter marked affected by either the independent review or
      `arc42.md` was opened and checked for stale, contradictory, or placeholder
      content.
- [x] Every Arc42 fact or architecture decision relevant to this change is
      represented in `design.md` and mapped to one or more tasks in `tasks.md`.
- [x] The completed implementation follows those facts and decisions; evidence
      names the built file, command, test, or observable result.
- [x] Architecture changes discovered in the completed implementation are
      recorded in the durable Arc42 chapter and the change-level assessment.
- [x] No mismatch remains.

ARC42 CONFORMANCE: CLEAN

**Result:** CLEAN.

---

## Coverage

No archive receipt exists yet for this change (it has not been archived).
Classification below is from static inspection of the committed production
diff plus the scenario-to-code mapping; the archive will measure the committed
candidate and record the merged result.

**Coverage result:** no measured receipt yet — classified from inspection.

```
No `givn/archive/explain-failure-guidance/verification.json` exists yet.
Both scopes ran GREEN during implementation and review preparation
(256 regular scenarios, 89 e2e scenarios), but the archive receipt is the
durable measured authority.
```

## Coverage gap classification

Every uncovered region must be classified as exactly one of:
1. **Dead code** (YAGNI/KISS) → delete it.
2. **Missing test coverage** → fix step definitions, or add a scenario and run RED/GREEN/REFACTOR.
3. **Legitimately hard to test** — rare; requires concrete technical justification.

| Location | Lines | Classification | Action taken |
|---|---|---|---|
| `src/main.rs` readiness delegation, diagnostics, exit mapping, interrupt check | new/changed | Covered by scenarios | S9-S18 (one scenario per path), S1-S8 (ready-provider baseline) |
| `src/review/session.rs` outcome return | changed | Covered by scenarios | S10 (failure), S12 (unusable), S13-S14 (delegation), E1 happy path |
| `src/review/response.rs` `ExplanationOutcome` + `apply_explanation_outcome` + wording | new | Covered by scenarios | S12, S10, S11; module unit tests updated |
| `givn/specs/use-shell/explain-command.feature` lockstep | changed | Covered | Full regular suite green |
| `tests/steps/setup_wizard_steps.rs` shared-step adaptation | changed | Covered | S14 + full e2e suite green |

No dead code found. No uncovered region required a bucket-2 addition or a
bucket-3 justification.

---

## Scenario execution evidence

```
unproven candidate — no receipt yet (the change has not been archived).
The archive gate will run both scopes and write
givn/archive/explain-failure-guidance/verification.json.
```

**Receipt status:** not archived yet

## Overlap dispositions

For every shape-match finding involving this change's delta, one row.
Decision must be one of: `duplicate`, `variant`, `boundary`.

| Scenario A | Scenario B | Disposition |
|---|---|---|
| A command beginning with a dash passes after the option terminator | The review-panel switches are inert for explain | boundary |
| A command read from standard input keeps its quoting and line breaks | A piped command without the marker is explained | variant |
| A failed explanation keeps the command reviewable | A network failure reports the mapped exit status | boundary |
| Developer explains an existing command in the review card | Enter closes the explanation card without releasing the command | boundary |

`[SUBST]` advisories name "A configured provider without a usable credential
is not contacted" as a lexical subset of several `configure-*` scenarios. That
is a shared-vocabulary heuristic, not a duplicate: the scenario asserts the
explain-specific setup delegation and exit 1, which no configure scenario
asserts. Disposition: `boundary` for each; merging would delete the
explain-path invariant.

## Split-or-keep

No scenario exceeds the lint length threshold; `givn lint` reported no
long-scenario finding.

## Sign-off

- [x] Fabrication audit: CLEAN (0 empty step bodies, all commits verified,
      all promised components exist, no downgraded e2e scenarios, no e2e
      scenario driven via HTTP/fetch() in place of real browser interaction,
      no over-produced e2e scope).
- [x] Scenario execution evidence recorded per scope in the archive
      verification receipt (review does not run the suite).
- [x] Every coverage gap is classified and resolved (or justified) from the
      most recent measured evidence plus static inspection.
- [x] No `@wip` tags remain in the delta spec.
- [x] The delta spec contains no implementation-layer detail.
- [x] Local run command starts the full stack, including digital twins,
      cleanly.
- [x] The exact file(s) `verify.e2e_command` invokes were identified, and
      the tree (including untracked files) was searched for a parallel e2e
      implementation before accepting a weaker one.
- [x] `verify.e2e_command` is not identical to `verify.command`; scenario
      counts prove real isolation (or "all scenarios are @e2e" is stated).
- [x] Implementation (commands, file layout, framework/driver) matches
      design.md as reviewed; any deviation went through a design.md update
      + design-review re-run, not a silent shortcut.
- [x] Interaction coverage verified: every spec inventory entry maps to a
      design matrix row, an existing `@e2e` scenario, and step definitions
      using the promised driving mechanism — no mismatches.
- [x] Design.md's `## Internal Primitives` section accurately lists every
      internal data structure and abstraction this change introduces.
- [x] arc42 slug-mapping verified: every arc42 chapter slug in arc42.md
      was mapped to a real `docs/arc42/<slug>.md` file.
- [x] No finding anywhere in this report is excused with a classification
      outside the three buckets.

**REVIEW: PASS**
