# Review: Givn 0.3.0 to 0.5.0 Migration (aggregate)

## Step 0 — Fabrication audit

- **@e2e tag integrity:** The change contains no delta `.feature` files
  (`.givn-skip` contains `specs`; zero `.feature` files under the change
  directory), so no scenario tag was added, changed, or removed. No
  fabrication finding.
- **Empty step bodies:** The change introduces 0 step-definition files and 0
  step bodies.
- **Checked tasks and commits:** Every checked evidence task in `tasks.md`
  cites a concrete commit (`8cd0d581`, `4384bb0`, `9a6e95c`, `e656609`,
  `9f5fe2b`), command result, or recorded decision. No production-code commit
  is claimed and none is required — the proposal's out-of-scope boundary
  forbids inventing domain work.
- **Strict-mode proof:** Inherited runner infrastructure (provenance
  `tests/features_runner.rs` `.fail_on_skipped()`, proven in the
  migrate-0-2-0-to-0-3-0 archive). This change adds no step and no runner
  change; both verify commands ran with real scenario counts (155 / 77), not
  silent passes.
- **verify.e2e_command binding:** `givn/commands.yaml` names
  `verify.command: ./run-tests.sh` and
  `verify.e2e_command: ./run-tests.sh --e2e` (distinct strings). Runs:
  155 scenarios (non-e2e) vs 77 scenarios (e2e); 77 < 155 proves the filter
  is a strict subset, not a whole-suite rerun.
- **Design conformance:** All activity used exactly the artifacts, commands,
  and boundaries named in design.md's prompt pack. One hardening edit fixed
  the change-id typo inside design.md (dots → hyphens) and was recorded in
  `design-review.md` — no tech decision changed, so no design-review rerun
  was required beyond that recorded one.
- **Interaction coverage verification:** Not applicable — no delta feature
  file or inventory entry exists in this change.

Fabrication audit result: clean; 0 findings.

## Arc42 implementation conformance

`addons.arc42` is enabled, so the conformance check is mandatory.

| Arc42 chapter or fact | Durable-doc source | arc42.md claim | design.md | tasks.md | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| All 12 chapters: no impact | docs/arc42/*.md | All rows `No` | Migration touches tooling process and repository policy only | All-`No` assessment recorded, independently re-derived in design-review with zero diff | Only `givn/changes/migrate-0-3-0-to-0-5-0/`, managed generated files, `givn/config.yaml`, `givn/commands.yaml` changed; no `docs/arc42/*.md` modification | Yes |
| ADR register unchanged | docs/arc42/09-architecture-decisions.md, docs/arc42/adr/ | No new/amended/superseded ADR | Semantic-features candidate NOT_QUALIFIED → change evidence | Qualification walked; all existing ADRs are watn product decisions | No ADR file or index entry changed | Yes |
| Diagram policy | all chapter files | n/a (no chapter edit) | n/a | Integrity scan: all 12 chapters real content, zero ASCII-art | Scan confirmed during design-review grilling | Yes |
| Config marker 0.5.0 | givn/config.yaml | n/a (tooling fact) | Machine-safe upgrade boundary | Inventory verified | Marker `0.5.0`, overrides preserved, Commit A allowlist inspected | Yes |

ARC42 CONFORMANCE: CLEAN

## Coverage

Measured with the project's configured instrumentation
(`cargo llvm-cov` over the Gherkin runner binary and library tests):

- `./measure-coverage.sh` (non-e2e and e2e passes) and `./merge-coverages.sh`
  produced `coverage/cobertura-coverage.xml`.
- Merged line coverage: 91.49% (`line-rate="0.9149426846639083"`).
- Both `./run-tests.sh` (20 features, 155 scenarios, 913 steps — all passed)
  and `./run-tests.sh --e2e` (24 features, 77 scenarios, 568 steps — all
  passed) exited 0.

## Classification

1. **Dead code:** none introduced — no production source changed. The empty
   untracked residue directory `givn/artifacts/arc42-docs/` was removed.
2. **Missing test coverage:** none — no new observable behavior exists to
   cover; the existing permanent suite passes under both filters.
3. **Legitimately hard to test:** none applicable.

Zero classification findings.

## E2E coverage

The canonical spec instruction was read during the workflow. This change
creates no E2E scenario, so no new inventory mapping applies. The permanent
E2E suite was verified unchanged: 24 features, 77 scenarios (77 passed),
568 steps (568 passed), exit 0.

## Retirements

None. No capability was retired (`@givn.retired` unused); the corpus contains
no semantic-engine capability, and the feature-free semantic decision removes
no watn behaviour.

## Capability Routing

Not applicable: this maintenance change has `.givn-skip: specs` and no delta,
so the proposal template's Capability Routing table has no target.
`givn spec route --change migrate-0-3-0-to-0-5-0` was run and recorded as
advisory only; it was not acted on and recommends nothing for this change's
(non-existent) delta.

## Overlap dispositions

This change introduces no delta scenario, so it participates in no shape
match and there are no finding pairs to disposition. Zero rows.
`givn check review` reports `net delta: 0 added, 0 modified, 0 removed`.

## Split-or-keep

No long-scenario finding involves this change (zero delta scenarios), so
there are no rows to disposition. The permanent-tree [LONG] advisory notices
pre-date this change and involve no delta of it.

## Sign-off checklist

- Fabrication audit: clean.
- Every checked task has concrete evidence; no production-code commit claimed
  or required.
- No new components promised; none missing.
- Strict-mode proof inherited and unchanged; both verify commands report real
  scenario counts.
- `verify.command` (155/155) and `verify.e2e_command` (77/77) both exit 0;
  commands are distinct strings; 77 < 155 proves the filter.
- Coverage measured across both passes and merged: 91.49% line.
- Every coverage gap classified: zero gaps introduced.
- Dead code: none (residue dir removed). Missing tests: none. Hard-to-test:
  none.
- No `@wip` tags exist in this change; no spec exists to leak implementation
  detail.
- Canonical E2E policy read; no delta interaction exists.
- No external stack required by this migration.
- No deviation from design.md beyond the recorded, non-technical id
  correction; design-review was run and passed in this change's own history.
- Deterministic disposition contract present in resolved review guidance;
  no retired semantic section exists anywhere in the resolved guidance.
- Disposition decisions use only the enforced vocabulary (no rows to
  disposition; tables present with exact headings).
- No finding excused outside the three buckets.

REVIEW: PASS
