# Design Review: watn-consolidation

## Grilling log

| # | Branch | Question | Outcome |
|---|---|---|---|
| 1 | Scope | Is the consolidation bounded by evidence? | PASS. F1-F6 are mapped to source files, titles, decisions, retained contracts, and assertion deltas. Later findings require separate dispositions. |
| 2 | Technology | Does Watn runtime behavior or deployment change? | PASS. Only Gherkin ownership, review evidence, test support, and archive output change. |
| 3 | Missing behavior | Are review, archive, rollback, removal, and retained-contract outcomes observable? | PASS. Two CLI E2E scenarios, one rollback scenario, and six exact removal deltas cover them. |
| 4 | Testability | Can review/archive run without mutating the real checkout? | PASS. Each smoke scenario creates a fresh TempDir and uses `fixture-consolidation` as the current-directory change. |
| 5 | Strictness | Can undefined steps or removal placeholders silently pass? | PASS. `.fail_on_skipped()` remains active; normal, E2E, and coverage filters exclude `@givn.removed`; named runs reject removed titles. |
| 6 | E2E fidelity | Do smoke tests drive the real user interface? | PASS. Both scenarios invoke real `givn` subprocesses and assert stdout, exit status, and resulting fixture state. |
| 7 | Interaction coverage | Does every inventory row map to exactly one E2E scenario and mechanism? | PASS. Review and archive each have one CLI subprocess scenario. |
| 8 | Risk | Are deletion, orphan binding, fixture mutation, and rollback risks controlled? | PASS. Retained-contract matrix, usage scan, TempDir boundary, exact tree assertions, and rollback scenario are explicit. |

## Resolved by codebase exploration

| Branch | Finding |
|---|---|
| Candidate evidence | F1-F6 are copied into the design matrix from the historical report branch and the current feature sources; the working tree does not depend on an absent report file. |
| Removal semantics | Each removed scenario has exactly one placeholder step and targets the original capability tag. `@givn.removed` is excluded from active runner and coverage filters. |
| Archive lifecycle | Added smoke scenarios invoke `fixture-consolidation`, not the active `watn-consolidation` change, so they remain valid after this change is archived. |
| Existing runner | Watn's feature runner is cucumber-rs with `.fail_on_skipped()` and the project-owned `run-tests.sh` commands. |
| Runtime boundary | No LLM provider, server, database, queue, or deployment service is involved in the consolidation fixture. |

## Open questions

None.

## Architecture Documentation (Arc42)

| # | Chapter | Independent assessment | arc42.md | Content aligned? |
|---|---|---|---|---|
| 1 | Introduction and Goals | Yes | Yes | Yes |
| 2 | Architecture Constraints | Yes | Yes | Yes |
| 3 | Context and Scope | Yes | Yes | Yes |
| 4 | Solution Strategy | Yes | Yes | Yes |
| 5 | Building-Block View | Yes | Yes | Yes |
| 6 | Runtime View | Yes | Yes | Yes |
| 7 | Deployment View | No | No | Yes |
| 8 | Crosscutting Concepts | Yes | Yes | Yes |
| 9 | Architecture Decisions | Yes | Yes | Yes |
| 10 | Quality Requirements | Yes | Yes | Yes |
| 11 | Risks and Technical Debt | Yes | Yes | Yes |
| 12 | Glossary | Yes | Yes | Yes |

- [x] All 12 rows were independently assessed.
- [x] Chapter 9 contains a full ADR-0025 section and the standalone MADR.
- [x] Every ADR consequence has a corresponding risk or technical-debt entry.
- [x] No affected chapter is placeholder-only or uses a forbidden diagram.

## Hardening Changes

| Artifact | Change |
|---|---|
| `proposal.md` | Added explicit removal-placeholder runner semantics and retained-contract scope. |
| `design.md` | Added F1-F6 evidence matrix, exact fixture layout, local runnability, risks, rollback, fixture lifecycle, and the separate regular/E2E/support step files. |
| `run-tests.sh` | Excludes removal placeholders from normal/E2E runs and rejects named removal targets. |
| `measure-coverage.sh` | Excludes removal placeholders from both coverage reports. |
| `.github/workflows/{ci,release,prepare-release}.yml` | Excludes removal placeholders from direct CI and release acceptance invocations. |
| `tests/steps/{watn_consolidation_steps,watn_consolidation_e2e_steps,watn_consolidation_support}.rs` | Separates the non-E2E rollback binding, CLI E2E bindings, and shared fixture helpers. |
| `arc42.md` | Added full Chapter 9 ADR-0025 decision summary. |
| Durable Arc42 docs | Added ownership constraint, context, runtime flow, ADR, quality scenarios, risks, glossary, and no-deployment impact. |

## Sign-off

- [x] Every branch has been walked.
- [x] All questions are resolved.
- [x] `design.md` matches the current scripts, fixture lifecycle, and separate
  regular/E2E/support step files.
- [x] Specs contain the complete interaction inventory and removal placeholders.
- [x] `givn lint --change watn-consolidation` reports no WIP scenarios after
  implementation.
- [x] Arc42 and ADR checks are complete.

**DESIGN-REVIEW: PASS**
