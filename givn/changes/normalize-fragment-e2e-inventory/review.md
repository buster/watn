# Review: normalize-fragment-e2e-inventory

Review kind: self-review
Author: maki
Reviewer context: maki

PERSONAS: terminal-developer--interactive

The confirmed persona remains the review lens; the change classifies existing tests and changes no product surface.

## Implementation Evidence Check

**Step definition scan** — the change introduces no step definition and no new step phrase:

```
0 empty step bodies found across 0 step definition files introduced by this change.
```

**Commit cross-check** — the batches cite the revisions that contain their work:

| Batch | Commit hash | Content |
|---|---|---|
| Delta reclassification and inventory | `9c6f633` | 5 delta feature files, the 11-row fragment inventory, and the change artifacts |
| Change verification adapter | `3bd20da` | `GIVN_FEATURES` collection and the `change` scope in the test harness and adapter |
| Adapter results | `4c14dfa` | Change-scope and suite counts recorded in `tasks.md` |

No production behaviour source changed; the adapter commits touch the test runner and `run-tests.sh` only. No scenario text changed.

**Promised-architecture cross-check** — `design.md` promises no component:

| Component (from design.md) | Exists in tree? | Path |
|---|---|---|
| none | N/A | N/A |

**Strict-mode proof** — inherited, unchanged in substance: `tests/features_runner.rs:217` calls `.fail_on_skipped()`. The change adds no step; the change-scope run executed 15 scenarios and both suite scopes executed real scenarios.

**Justified e2e reclassification** — every moved scenario is a variant, error case, or output-affecting flag of an action that keeps one e2e scenario, and its text is identical:

| Capability | Scenario moved to regular | Classification |
|---|---|---|
| ask | Explicit tier -1 uses the small/fast model | output-affecting flag variant |
| ask | Tier -2 uses the normal model | output-affecting flag variant |
| ask | Tier -3 uses the thinking/reasoning model | output-affecting flag variant |
| ask | Cost is displayed when pricing is configured | output variant |
| ask | Tokens/second is displayed after response completes | output variant |
| ask | Ask via stdin pipe | input-channel variant (D1) |
| ask | Execute flag with explicit "y" confirmation | confirmation variant |
| ask | Execute flag with "n" answer skips execution | refusal variant |
| cancel-running-completion | One Ctrl+C cancels a completion waiting for a connection | timing variant |
| config | Environment variable overrides config file | precedence variant |
| config | CLI flag overrides environment variable | precedence variant |
| incremental-sse-rendering | Verbose streaming keeps reasoning on stderr and command text on stdout | verbose flag variant |
| incremental-sse-rendering | A mid-stream failure preserves visible content and exits unsuccessfully | error case |
| incremental-sse-rendering | Piped confirmation remains available after streamed output | input-channel variant (D1) |
| transport | Missing or whitespace test overrides fall back to the configured provider | fallback variant |

The retained e2e set is 11 scenarios and the inventory declares exactly those 11 (0 missing, 0 extra). A parse comparison found 0 title or step-text errors across the 15 delta scenarios.

**Downgraded e2e scenario scan** — no delta `@e2e` scenario exists, so none can be downgraded. The reclassification above is the recorded justification the specs instruction requires.

| @e2e Scenario | Real-interface assertion present? | Driven via real browser (if UI)? | Downgraded? |
|---|---|---|---|
| none added or downgraded | N/A | N/A | No |

**E2e scope check** — same actions per capability, no over-production remains:

| Capability | @e2e scenario count after merge | Distinct actions? | Over-produced? |
|---|---|---|---|
| ask | 2 | ask; execute | No |
| cancel-running-completion | 1 | cancel | No |
| config | 2 | configure tiers; configure pricing | No |
| incremental-sse-rendering | 2 | stream rendering; confirm after streaming | No |
| release-truth | 2 | read version; generate changelog | No |
| transport | 2 | normal debug routing; test-support routing | No |

**Duplicate-implementation check** — no e2e implementation is added:

| Capability | File(s) verify.e2e_command invokes | Other implementation found? (incl. untracked) | Stronger one used instead? |
|---|---|---|---|
| none | `tests/features_runner.rs` via `./run-tests.sh --e2e` | No | N/A |

**Local environment check** — no local server or external digital twin is introduced; the existing in-process provider twins and PTY sessions are unchanged.

**E2E scope check** — read literally from `givn/commands.yaml` and the runner result:

| `verify.command` value | `verify.e2e_command` value | Explicit scope selection | Regular scope evidence | E2e scope evidence | Zero failed or skipped? |
|---|---|---|---|---|---|
| `./run-tests.sh` | `./run-tests.sh --e2e` | `GIVN_RESULT_SCOPE=regular` / `e2e`, and `change` for `givn check tasks` | 28 features, 290 scenarios, 1756 steps, exit 0 | 26 features, 92 scenarios, 699 steps, exit 0 | Yes |

The change-scope receipt records 15/15 delta scenarios with scope `change`. `92 < 290` confirms the e2e invocation is a strict subset before the merge; after the merge the e2e count is expected to be 77.

## Use-Case And Persona Conformance

| Contract | Evidence | Pass? |
|---|---|---|
| Owning use-case guarantees and rules preserved | The fragment document keeps its capabilities, interactions, and diagram; no rule changes | Yes |
| Capability ownership and typed relationships preserved | No capability is added, moved, or retired | Yes |
| Confirmed Persona-relevant outcomes preserved | No product surface changes; the persona remains a review lens | Yes |
| Actors remain distinct from Persona biographies | No Gherkin actor is introduced | Yes |
| Migration ledger and coverage evidence complete, if applicable | No migration; the change-scope receipt and suite runs are recorded | Yes |

## Deferred mandatory checks

None.

**Design.md conformance check** — the implementation matches the design:

| Decision (from design.md) | design.md says | Actually built | Matches? | If not: was design.md updated + design-review re-run? |
|---|---|---|---|---|
| Retag mechanism | `@givn.modified` deltas without `@e2e`, same titles and steps | 5 delta files, 15 scenarios, 0 text diffs | Yes | N/A |
| Inventory | 11 actions matching the retained tags | 11 rows, 11 retained, 0 missing, 0 extra | Yes | N/A |
| Change verification adapter | Honor `GIVN_FEATURES` and `GIVN_RESULT_SCOPE=change` | Runner restricts collection; script preserves the change scope and uses `not @wip` | Yes | Yes — design and design-review re-passed in `3bd20da` |
| Default scopes | Regular 290, e2e 92 before merge | 290/290 and 92/92 after the adapter change | Yes | N/A |

**Interaction coverage verification** — the fragment inventory, matrix, and features agree:

| Inventory entry | Matrix row exists? | @e2e scenario exists? (title) | Promised driving mechanism | Actual driving mechanism (from step defs) | Match? |
|---|---|---|---|---|---|
| ask / ask a question | Yes | Ask with default tier returns a copy-pasteable command | CLI subprocess | `cucumber-rs` steps run the built `watn` binary | Yes |
| ask / execute a command | Yes | Execute flag prompts for confirmation | PTY | PTY step sends Enter to `watn -x` | Yes |
| cancel-running-completion / cancel | Yes | One Ctrl+C cancels a completion waiting for streamed output | PTY | PTY step sends Ctrl+C | Yes |
| config / configure tiers | Yes | Configure model tiers in config file | CLI subprocess | isolated config plus local twin assertion | Yes |
| config / configure pricing | Yes | Model pricing configured for cost display | CLI subprocess | isolated config plus printed cost assertion | Yes |
| incremental / stream rendering | Yes | Command text appears before a delayed stream completes | PTY | delayed streaming twin and terminal read | Yes |
| incremental / confirm after streaming | Yes | Raw terminal confirmation happens after the complete command arrives | PTY | PTY reads stream then sends Enter | Yes |
| release-truth / read version | Yes | Version flag reports the package version | CLI subprocess | release binary `--version` assertion | Yes |
| release-truth / generate changelog | Yes | Repeated revisions of one change yield one changelog entry | CLI subprocess | fixture range and emitted entry assertion | Yes |
| transport / normal routing | Yes | Normal debug requests ignore test routing settings | CLI subprocess | default binary against two local twins | Yes |
| transport / test-support routing | Yes | Test-support requests use isolated routing without changing saved configuration | CLI subprocess | test-support binary against the isolated twin | Yes |

## Visual Review

N/A. The design declares no rendered screen and no changed CLI output.

## Retirements

N/A. Nothing is retired.

## Arc42 implementation conformance

| Arc42 chapter or architectural fact | Durable-doc source | `arc42.md` claim | `design.md` treatment | `tasks.md` treatment | Completed implementation evidence | Match? |
|---|---|---|---|---|---|---|
| 01 introduction-and-goals | docs/arc42/01-introduction-and-goals.md | No | No goal change | No goal task | Test classification only | Yes |
| 02 architecture-constraints | docs/arc42/02-architecture-constraints.md | No | No constraint change | No constraint task | No constraint touched | Yes |
| 03 context-and-scope | docs/arc42/03-context-and-scope.md | No | No interface change | No interface task | Existing CLI scenarios only | Yes |
| 04 solution-strategy | docs/arc42/04-solution-strategy.md | No | No strategy change | No strategy task | No product strategy touched | Yes |
| 05 building-block-view | docs/arc42/05-building-block-view.md | No | No component change | No component task | No building block changed | Yes |
| 06 runtime-view | docs/arc42/06-runtime-view.md | No | No runtime flow change | No runtime task | No runtime path changed | Yes |
| 07 deployment-view | docs/arc42/07-deployment-view.md | No | No deployment change | No deployment task | No deployment artifact changed | Yes |
| 08 crosscutting-concepts | docs/arc42/08-crosscutting-concepts.md | No | No cross-cutting concept | No concept task | No concept changed | Yes |
| 09 architecture-decisions | docs/arc42/09-architecture-decisions.md | No | ADR action `NONE` | Qualification recorded | No ADR file or register entry changed | Yes |
| 10 quality-requirements | docs/arc42/10-quality-requirements.md | No | No quality change | No quality task | No requirement changed | Yes |
| 11 risks-and-technical-debt | docs/arc42/11-risks-and-technical-debt.md | No | Merge/count risk is an implementation check | Recorded in tasks | No durable debt added | Yes |
| 12 glossary | docs/arc42/12-glossary.md | No | No domain term | No glossary task | No glossary file changed | Yes |

- [x] Every chapter marked affected by either the review or `arc42.md` was opened and checked for stale, contradictory, or placeholder content.
- [x] Every Arc42 fact or architecture decision relevant to this change is represented in `design.md` and mapped to one or more tasks in `tasks.md`.
- [x] The completed implementation follows those facts and decisions.
- [x] Architecture changes discovered in the completed implementation are recorded in the durable Arc42 chapter and the change-level assessment.
- [x] No mismatch remains.

ARC42 CONFORMANCE: CLEAN

## Coverage

The most recent measured report (migration verification) is `coverage/cobertura-coverage.xml` at `line-rate="0.9308600337268128"` (93.09%). This change adds no product source line; the measured population is unchanged. The archive receipt carries the merged measurement for the committed candidate.

```
Line coverage: 93.09% (21522/23127)
Branch coverage: n/a (configured invocation emits no branch data)
```

## Coverage gap classification

| Location | Lines | Classification | Action taken |
|---|---|---|---|
| none | 0 | No gap introduced | No product line added or removed |

The three buckets are empty: no dead code, no missing coverage, no hard-to-test region.

## Scenario execution evidence

```
Change scope (givn check tasks): 15 scenarios, 15 passed, 0 failed, 0 skipped; receipt at givn/changes/normalize-fragment-e2e-inventory/verification/tasks-verification.json.
Regular scope: 28 features, 290 scenarios, 1756 steps; exit 0.
E2e scope: 26 features, 92 scenarios, 699 steps; exit 0.
Post-merge expectation: 26 features, 77 e2e scenarios; 28 features, 290 regular scenarios.
Receipt status: change receipt proven; archive receipt pending.
```

One pre-adapter regular run reported 1 failed scenario out of 290 before the failure output was captured; three subsequent full regular runs, including the post-adapter run, passed 290/290. The flake is recorded as test-infrastructure risk, not a coverage gap and not a deterministic finding.

## Overlap dispositions

| Scenario A | Scenario B | Disposition |
|---|---|---|

`givn check review` reported `net delta: 0 added, 15 modified, 0 removed` and `overlap dispositions passed`; no shape-match pair involving the delta requires a disposition. The `[SUBST]` advisories are the same permanent-versus-delta pairs created by the modification mechanism, not duplicate scenarios. Zero rows.

## Split-or-keep

| Scenario | Decision |
|---|---|

No long-scenario finding involves this delta; the modified scenarios keep their existing length. Zero rows.

## README impact decision

README-IMPACT: none

The change touches no command, flag, configuration, or output a consumer observes.

## Sign-off

- [x] Fabrication audit: CLEAN — 0 empty step bodies, all batch commits verified, no promised component, no downgraded e2e scenario, no over-produced e2e scope.
- [x] Scenario execution evidence recorded per scope; the archive receipt will carry the post-merge result.
- [x] Every coverage gap is classified and resolved; no gap was introduced.
- [x] No `@wip` tags exist in the delta.
- [x] The delta contains no implementation-layer detail.
- [x] No local stack is required; the existing in-process twins and PTY run locally.
- [x] The exact e2e entry point (`tests/features_runner.rs` via `./run-tests.sh --e2e`) was identified; no parallel implementation exists.
- [x] The E2E execution selects its scope explicitly; the change scope is preserved by the adapter.
- [x] Implementation matches `design.md`, including the verification adapter added and re-reviewed in `3bd20da`.
- [x] Interaction coverage verified: 11 inventory entries, 11 matrix rows, 11 matching `@e2e` scenarios, and the promised driver for each.
- [x] `design.md` introduces no internal primitives.
- [x] arc42 slug mapping verified: all claims are `No`; all twelve chapter files exist and were checked for integrity.
- [x] No finding is excused with a classification outside the three buckets.

REVIEW: PASS
