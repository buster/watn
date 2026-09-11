# Design Review: migrate-0-6-0-to-0-7-0

## Method

A fresh-context grilling subagent read the full plan (`proposal.md`,
`design.md`, `migration.yaml`, `arc42.md`), the affected chapters,
`run-tests.sh`, `tests/features_runner.rs`, `givn/commands.yaml`,
`givn/config.yaml`, and the pinned cucumber 0.23.0 sources. The orchestrator
independently verified the critical finding against the crate source and the
runner code. Questions were put to the user one at a time.

## Branch dispositions

| Branch | Disposition |
|---|---|
| Scope | Design covers proposal items 1-4 exactly; no product behavior added. Two artifact defects found: the required `runner-result-contract.md` evidence file is absent (F-S1, routed to tasks), and the Phase 5 command exported a stale `GIVN_FEATURES` variable (F-S3, fixed). |
| Tech choices | Counts come from cucumber's `Summarize` stats (`scenarios_stats()`); no simpler alternative satisfies the machine-readable requirement. |
| Missing scenarios | No product scenarios expected: specs are legitimately N/A (`.givn-skip` contains `specs`, `givn status` shows `⊘ specs`). The contract is validated manually per design Phase 5. |
| Testability | RED for the runner fix is a malformed feature fixture: before the fix the run exits 0 with a reconciling result; after the fix it exits 1. No Gherkin scenario is required for this maintenance change. |
| E2E fidelity | N/A: no spec or E2E scenario changes. |
| Interaction coverage | N/A: no user-interaction inventory in this change. |
| Risk | The fail-open exit condition (F-I5) was the top risk and is fixed in this change (Q1). Tag drift: no non-`givn.removed` `@wip` scenarios exist; `@wip` exclusion is givn's implementation-time mechanism, so no runner guard was added (a runner-level guard would fail legitimate in-progress work). Residual drift stays under R-080. |
| Domain context | No use case, typed relationships, or Personas apply: the change type has no specs. |
| ADR qualification | Structured verdict NOT_QUALIFIED; routing CANONICAL_ARTIFACT to `design.md`; existing-ADR check found no record for the result contract or archive boundary. `arc42.md` row 9 and Notes now carry the verdict; no MADR created. |
| arc42 | Independent 12-row re-derivation matches `arc42.md` exactly (Yes: 2, 6, 8, 11, 12; No: 1, 3, 4, 5, 7, 9, 10). Every Yes row was opened and matches design.md; no placeholders; no ASCII-art diagrams in any of the 12 chapter files. |
| Ubiquitous language | "Verification result" and "Archive receipt" are in the glossary, and the definition names the `regular`/`e2e` scope literals; no contradictory usage. |

## Findings

- F-S1: `runner-result-contract.md` (design.md:45-46, 159-160) is absent. Routed to tasks as a deliverable.
- F-S2: Completion boundary. The proposal's "next archived change produces a proven receipt" is already satisfied by `givn/archive/setup-shell-and-style/verification.json` (regular 222/222, e2e 88/88, `proven`, produced after commit `0aec03b`). The design's stricter Phase 5.3 requirement is followed: this change archives through the normal workflow and its own receipt must be proven.
- F-S3: Stale `GIVN_FEATURES=givn/specs` removed from design.md Phase 5; no code reads that variable.
- F-I5: Fail-open exit condition. `tests/features_runner.rs` replaced cucumber's `run_and_exit()` with `.run()` plus a `stats.failed`/`stats.skipped` check, while cucumber's `Writer::execution_has_failed()` also covers parsing and hook errors. A malformed feature silently dropped scenarios while the result still reconciled. Fixed in this change (Q1).
- F-I6: `GIVN_RESULT_SCOPE` defaults to `regular`, so a direct e2e runner invocation without the variable would be mislabeled. Low risk: `run-tests.sh` always sets it. No change.
- F-I7: Stale panic message "write the review result file" corrected with the F-I5 fix.

## Questions and decisions

- Q1 (fail-open exit regression): user chose "fix now". The runner exit condition will use `Writer::execution_has_failed()` (failed steps, parsing errors, hook errors) while still writing the result file; implemented as a task with a malformed-feature RED fixture.
- Q2 (already-committed implementation, commit `0aec03b`): recorded in tasks.md as completed phase tasks with the commit hash; the F-I5 fix is a follow-up scenario commit.
- Q3 (completion boundary): follow design Phase 5.3 - archive this change; the proposal boundary is already satisfied.
- Q4 (ADR routing): apply the structured NOT_QUALIFIED / CANONICAL_ARTIFACT verdict.
- Q5 (tag-drift guard): no guard; `@wip` is givn's implementation-time tag and no such scenarios exist; R-080 covers residual drift.

## Hardening applied

- `design.md`: added the parsing/hook-error exit rule to Phase 2; removed the stale `GIVN_FEATURES` variable from the Phase 5 command.
- `arc42.md`: row 9 carries the structured verdict; Notes name exactly one canonical artifact.
- `docs/arc42/11-risks-and-technical-debt.md`: R-080 mitigation now names the harness failure signal.
- `givn lint --change migrate-0-6-0-to-0-7-0`: exit 0.

## Open items for tasks

- Runner exit-condition fix (RED: malformed feature fixture; GREEN: `execution_has_failed()`; REFACTOR: panic message).
- `runner-result-contract.md` with the scope inventory, before/after commands, and sample result JSON.

DESIGN-REVIEW: PASS
