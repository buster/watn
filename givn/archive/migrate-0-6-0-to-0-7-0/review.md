# Review: migrate-0-6-0-to-0-7-0

## Fabrication Audit (do this first)

**Step definition scan** — every step definition file checked for empty/no-op
bodies (`{}`, bare `pass`, bare `return`, no assertion reference):

```
rg -n -U 'fn [A-Za-z_][^{]*\{\s*\}' tests/steps/*.rs  -> no matches
rg -n '\|\s*[a-z_]*\s*\|\s*\{\s*\}' tests/steps/*.rs  -> no matches
0 empty step bodies found across 31 step definition files.
```

**Commit cross-check** — every checked-off `[x]` task in tasks.md has a
matching commit that touches production source (not just spec/stub files):

| Task | Commit hash | Touches production code? |
|---|---|---|
| Setup 1-2 (runner config, baseline) | evidence only, no source change | N/A (evidence tasks) |
| Runner result contract RED/GREEN/REFACTOR | `6c52d08` | yes - `tests/features_runner.rs` (the change's deliverable) |
| Evidence and validation | `ffc6cfc` | no source change; adds `runner-result-contract.md` |

**Promised-architecture cross-check** — every component design.md said would
be created actually exists:

| Component (from design.md) | Exists in tree? | Path |
|---|---|---|
| Scope export in the regular/e2e wrapper | yes | `run-tests.sh:24-30` |
| Runner-derived result emission | yes | `tests/features_runner.rs:215-226` |
| Fail-closed exit including parsing/hook errors | yes | `tests/features_runner.rs:227` |
| Evidence file | yes | `givn/changes/migrate-0-6-0-to-0-7-0/runner-result-contract.md` |

**Strict-mode proof** — confirmed present and passing in tasks.md setup task:

```
tasks.md Setup task 1: temporary scenario `Strict runner rejects an undefined
step` ran via `./run-tests.sh --name ...` and exited 1 with
`1 scenario (1 failed)`, `1 step (1 failed)`.
```

**Downgraded e2e scenario scan** — every `@e2e` scenario's Then steps
checked for a primary assertion on the observable output, return value,
acknowledgement, or visible state of the interface named in design.md, not
repository-only; for a browser-UI capability, the step definitions checked
for actually driving the browser (click/type/navigate), not an HTTP request or
in-page `fetch()`:

| @e2e Scenario | Real-interface assertion present? | Driven via real browser (if UI)? | Downgraded? |
|---|---|---|---|
| N/A - this maintenance change adds no delta scenarios | N/A | N/A | N/A |

**E2e scope check** — each capability has one `@e2e` scenario per distinct
happy-path action, not one per variant/error case:

| Capability | @e2e scenario count | Distinct actions? | Over-produced? |
|---|---|---|---|
| N/A - no delta scenarios | 0 | N/A | N/A |

**Duplicate-implementation check** — the exact file(s) `verify.e2e_command`
invokes, and confirmation the tree (including untracked files, via `git
status`) was searched for any other/parallel e2e implementation of the same
capability that might be stronger than the one being reviewed:

| Capability | File(s) verify.e2e_command invokes | Other implementation found? (incl. untracked) | Stronger one used instead? |
|---|---|---|---|
| N/A - no delta scenarios | `tests/features_runner.rs` via `./run-tests.sh --e2e` | no | N/A |

**Local environment check** — the local run command (design.md's Local
Runnability section) starts the full stack, including digital twins,
cleanly:

```
N/A - this maintenance change has no Local Runnability section; it changes
only the runner's result emission and exit condition.
```

**E2e isolation check** — `verify.command` and `verify.e2e_command` read
literally from `givn/config.yaml`; if identical, this is always a finding
(the "e2e run" is not isolating `@e2e` scenarios at all). Prove isolation
with the per-scope scenario counts recorded in the archive verification
receipt (`givn/archive/<id>/verification.json`):

| `verify.command` value | `verify.e2e_command` value | Identical? | Regular scope scenario count | E2e scope scenario count | E2e count strictly smaller (or all scenarios @e2e)? |
|---|---|---|---|---|---|
| `./run-tests.sh` | `./run-tests.sh --e2e` | no | 222 | 88 | yes |

The counts come from the manual scope validation recorded in
`runner-result-contract.md`; the archive receipt will carry the authoritative
per-scope counts.

**Design.md conformance check** — implementation's actual commands, file
layout, and framework/driver choices diffed against what design.md
explicitly named. Any deviation implemented without first updating
design.md and re-running design-review is a finding, not an acceptable
shortcut:

| Decision (from design.md) | design.md says | Actually built | Matches? | If not: was design.md updated + design-review re-run? |
|---|---|---|---|---|
| Commands | keep `./run-tests.sh` and `./run-tests.sh --e2e` | unchanged | yes | N/A |
| Result write | one JSON to `GIVN_RESULT_FILE` with runner counts | `tests/features_runner.rs:216-226` | yes | N/A |
| Exit semantics | exit code reflects the run including parse/hook errors | `writer.execution_has_failed() \|\| stats.skipped > 0` at `tests/features_runner.rs:227` | yes | N/A (rule added to design.md during design-review) |
| Scope value | exactly `regular`/`e2e` | `run-tests.sh:26,29` exports `GIVN_RESULT_SCOPE` | yes | N/A |
| Validation command | `GIVN_RESULT_FILE="$result" <scope command>` | no `GIVN_FEATURES` variable | yes | N/A (stale variable removed during design-review) |

**Interaction coverage verification** — spec's User Interaction Inventory
cross-referenced against design's Interaction Coverage Matrix, `.feature`
file, and step definitions. Every inventory entry must have a matrix row,
a matching `@e2e` scenario, and step definitions that use the promised
driving mechanism:

| Inventory entry | Matrix row exists? | @e2e scenario exists? (title) | Promised driving mechanism | Actual driving mechanism (from step defs) | Match? |
|---|---|---|---|---|---|
| N/A - specs are not applicable to this maintenance change | N/A | N/A | N/A | N/A | N/A |

## Use-Case And Persona Conformance

| Contract | Evidence | Pass? |
|---|---|---|
| Owning use-case guarantees and rules preserved | No use case: `givn status` shows `⊘ specs`; the change type has no spec. | N/A |
| Capability ownership and typed relationships preserved | No capability changes; net delta 0. | N/A |
| Confirmed Persona-relevant outcomes preserved | No Persona applies. | N/A |
| Actors remain distinct from Persona biographies | N/A. | N/A |
| Migration ledger and coverage evidence complete, if applicable | This is a version migration without corpus changes; `runner-result-contract.md` carries the contract evidence. | N/A |

## Retirements

N/A - this change retires nothing.

## Arc42 implementation conformance

| Arc42 chapter or architectural fact | Durable-doc source | `arc42.md` claim | `design.md` treatment | `tasks.md` treatment | Completed implementation evidence | Match? / finding |
|---|---|---|---|---|---|---|
| Git-backed archive boundary and runner-derived fail-closed verification convention | `docs/arc42/02-architecture-constraints.md:39,57` | Yes | Phase 2 rules, Phase 4 | Setup, Runner result contract | `givn/commands.yaml`, `tests/features_runner.rs:216-229` | yes |
| Archive flow returns per-scope result JSON and rejects non-reconciling results | `docs/arc42/06-runtime-view.md:1042-1050` | Yes | Phase 2 | Runner result contract, validation | `run-tests.sh:24-30`, `tests/features_runner.rs:215-226` | yes |
| Archive verification contract | `docs/arc42/08-crosscutting-concepts.md:508-519` | Yes | Phase 2, Phase 4 | Runner result contract | same as above | yes |
| R-080 runner drift risk with harness failure signal | `docs/arc42/11-risks-and-technical-debt.md:82` | Yes | Phase 2 exit rule | RED/GREEN/REFACTOR | commit `6c52d08` | yes |
| Verification result and archive receipt terms | `docs/arc42/12-glossary.md:126-127` | Yes | Phase 2 | validation | result JSON and receipt file | yes |

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

Coverage is measured once by the archive against the committed candidate and
recorded in the verification receipt. Review does not run an instrumented
pass; classify gaps from the most recent measured evidence plus static
inspection, and say explicitly when no measured evidence exists yet.

**Coverage result:** No receipt exists yet for this change (unarchived
candidate). The most recent measured evidence is the prior receipt
`givn/archive/setup-shell-and-style/verification.json`: line coverage 19005 /
20586 (92%). This change touches no production source (`src/`); it modifies
the test harness (`tests/features_runner.rs`), `run-tests.sh`, and docs, so
the measured production coverage is unaffected.

```
{"coverage": {"status": "passed", "line_covered": 19005, "line_valid": 20586,
 "line_percent": 92}}
```

## Coverage gap classification

| Location | Lines | Classification | Action taken |
|---|---|---|---|
| None - no production source changed | - | - | - |

The new runner branch (`writer.execution_has_failed()`) is exercised by the
malformed-feature RED/GREEN evidence recorded in `tasks.md` and
`runner-result-contract.md`; no gap remains.

---

## Scenario execution evidence

Review does not re-run the suite. Record the last archive verification receipt
for this change (or state that the change has not been archived yet):

```
unproven candidate — no receipt yet. Both scopes were validated manually
during implementation and recorded in runner-result-contract.md:
regular 222/222 {"failed":0,"passed":222,"scope":"regular","skipped":0,"total":222}
e2e 88/88 {"failed":0,"passed":88,"scope":"e2e","skipped":0,"total":88}
```

**Receipt status:** not archived yet

## Overlap dispositions

Net delta 0; no shape-match findings involving this change's delta.

## Split-or-keep

No scenario longer than the lint threshold in this change.

## README-IMPACT

README-IMPACT: none

## Sign-off

- [x] Fabrication audit: CLEAN (0 empty step bodies, all commits verified,
      all promised components exist, no downgraded e2e scenarios, no e2e
      scenario driven via HTTP/fetch() in place of real browser interaction,
      no over-produced e2e scope).
- [x] Scenario execution evidence recorded per scope in the archive
      verification receipt (review does not run the suite).
- [x] Every coverage gap is classified and resolved (or justified) from the
      most recent measured evidence plus static inspection.
- [x] No `@wip` tags remain in the delta spec (no delta spec).
- [x] The delta spec contains no implementation-layer detail (no delta spec).
- [x] Local run command starts the full stack, including digital twins,
      cleanly (N/A - no such section for this maintenance change).
- [x] The exact file(s) `verify.e2e_command` invokes were identified, and
      the tree (including untracked files) was searched for a parallel e2e
      implementation before accepting a weaker one.
- [x] `verify.e2e_command` is not identical to `verify.command`; scenario
      counts prove real isolation (222 regular vs 88 e2e).
- [x] Implementation (commands, file layout, framework/driver) matches
      design.md as reviewed; any deviation went through a design.md update
      + design-review re-run, not a silent shortcut.
- [x] Interaction coverage verified: N/A - specs are not applicable to this
      maintenance change.
- [x] Design.md's `## Internal Primitives` section accurately lists every
      internal data structure and abstraction this change introduces (N/A -
      the generated migration design has no such section; the fix reuses
      cucumber's existing `Stats` trait).
- [x] arc42 slug-mapping verified: every arc42 chapter slug in arc42.md was
      mapped to a real `docs/arc42/<slug>.md` file per
      `src/integrity_hook.rs:check_arc42_conformance`.
- [x] No finding anywhere in this report is excused with a classification
      outside the three buckets.

**REVIEW: PASS**
