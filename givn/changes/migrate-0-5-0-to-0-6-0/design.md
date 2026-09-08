# Design: .migrate-0-5-0-to-0-6-0.tmp-3733965

## Migration metadata

- source_version: `0.5.0`
- target_version: `0.6.0`
- upgrade_commit: `00b2faab0713762482c70449eff0a531fa03b554`
- migration_type: aggregate maintenance change without a product specification

## Execution order

Complete the following phases strictly in sequence. Do not create independentparallel migration changes for these bundles. The complete sequence is alsorecorded in `migration.yaml`.

### Phase 1: migrate-0-5-0-to-0-6-0 (0.5.0 -> 0.6.0)

# Design: migrate-usecases-and-ideation

## Migration contract

This bundle is an LLM-guided semantic migration, not a mechanical rename. The
agent must stop on an unrelated dirty worktree, record the Givn
version/configuration, enumerate active changes and ideation state, and write
baselines before edits. The preflight for this repository finds that the target
use-case/fragment layout was already applied by the archived
`migrate-usecases-watn` change. Therefore this phase is verification-only: it
must not repeat file moves or rewrite the active corpus. Any drift from the
target layout is a blocker to resolve in the ledger, not permission to make an
unreviewed mutation.

## Required ledger

Create `usecase-migration.md` before verification. When legacy entries exist,
every old scenario appears exactly once in this table or is explicitly retired.
When preflight finds the target corpus already applied, record one
`VERIFY_ALREADY_MIGRATED` row for the verified corpus and reference the prior
ledger; no scenario is silently reclassified:

| Old entry | Old owner | Operation | New owner | New capability | Old @e2e | New @e2e | Behavior changed | Reason |
|---|---|---|---|---|---:|---:|---|---|

Allowed operations are `MOVE`, `SPLIT`, `MERGE`, `REASSIGN_CAPABILITY`,
`CLASSIFY_AS_FRAGMENT`, `CREATE`, `RENAME`, `RETIRE`,
`UPDATE_RELATIONSHIP`, and `VERIFY_ALREADY_MIGRATED`. The last operation is
valid only when no legacy owner remains in the active corpus and the target
schema, ownership, relationships, and evidence can be verified without edits.

## Phases

1. Preflight and baselines.
2. Semantic corpus inventory and human confirmation.
3. Verify the existing use-case and fragment documents, capability ownership,
   typed relationships, and capability-aware interactions. Use `git mv` only
   if a future repository actually contains legacy entries and the ledger has
   recorded them; this repository must perform no move.
4. Verify active change paths, proposals, designs, reviews, and tasks without
   rewriting historical archives.
5. Preserve ideation topics, map stable IDs, and present Handoff and Persona
   promotion decisions without executing either automatically.
6. Run `givn spec coverage --format json`, compare the evidence, run the
   configured verification and E2E commands, and compare source line/branch
   counters from the configured Cobertura reports.
7. Audit active paths for old vocabulary while excluding `givn/archive/`.

## Blocking checks

- Every use-case and fragment document parses under the active strict schema.
- Every capability reference exists exactly once.
- Every typed relationship resolves; extensions do not target fragments; cycles
  are rejected.
- Every interaction maps to exactly one `@e2e` scenario.
- No behavior hash, E2E evidence, or interaction coverage is lost without an
  explicit ledger retirement.
- Line and branch coverage after migration are each at least the baseline.
- A stale use-case replacement is rejected before mutation.

## Evidence files

Retain these files in the migration change:

```text
usecase-migration.md
coverage-before.json
coverage-after.json
coverage-comparison.json
source-coverage-before.json
source-coverage-after.json
```

The supported final gates are `givn lint`, `givn check arc42-docs --change
migrate-0-5-0-to-0-6-0`, `givn check review --change
migrate-0-5-0-to-0-6-0`, and the configured verification commands. There is no
`migration` artifact in the active Givn 0.6 graph, so `givn check migration` is
not a valid command and must not be used as evidence.

## Evidence commands

Capture the current inventory before the no-op verification and again after it:

```text
givn spec coverage --format json > coverage-before.json
givn spec coverage --format json --baseline coverage-before.json > coverage-comparison.json
givn spec coverage --format json > coverage-after.json
./run-tests.sh
./run-tests.sh --e2e
./measure-coverage.sh
./merge-coverages.sh
```

Derive `source-coverage-before.json` and `source-coverage-after.json` from the
`lines-covered`, `lines-valid`, `branches-covered`, and `branches-valid`
attributes in `coverage/cobertura-coverage.xml`, and write
`source-coverage-comparison.json` with an explicit non-regression comparison:
covered lines and branches may increase but must not decrease, while valid
counter totals must remain compatible. The current producer reports zero branch
counters; record those counters honestly and do not claim branch coverage that
the configured producer does not measure.

The interaction matrix is represented by the declared interaction rows and the
`interaction` field in the coverage inventory. Assert that the inventory
contains the same 52 interaction mappings as the use-case documents and that
every mapped entry is attached to an E2E scenario. The field identifies the
owning capability, so repeated capability values are expected when one
capability has multiple consumer actions. No new product scenarios are required
because `.givn-skip` excludes the change's `specs/` artifact.

## Domain and ADR routing

The active corpus has no ideation topics, confirmed Personas, or Handoff to
execute. Existing use-case documents declare `Personas: none`; the migration
records that absence and does not invent actors or biographies.

The stable use-case/fragment layout was checked against the active and archived
ADR indexes. ADR-0025 governs repository-wide scenario ownership, but it does
not decide the use-case/fragment migration mechanism. The complete
qualification result is:

```json
{
  "qualification": "NOT_QUALIFIED",
  "alternatives": "FAIL",
  "architectural_impact": "PASS",
  "durable_consequence": "PASS",
  "lower_level_artifact": "FAIL",
  "existing_adr_check": "PASS",
  "must_be_shared": "SUPPORTING",
  "routing": "CANONICAL_ARTIFACT",
  "canonical_artifact": "givn/changes/migrate-0-5-0-to-0-6-0/design.md",
  "target_adr": null,
  "replacement_adr": null,
  "evidence": {
    "alternatives": [],
    "architectural_impact": [
      "The target layout fixes permanent specification ownership and relationship boundaries."
    ],
    "durable_consequence": [
      "Reversing the corpus layout would require broad path, ownership, and evidence changes."
    ],
    "lower_level_artifact": [
      "The migration contract and rationale are completely owned by this design and its ledger."
    ],
    "existing_adr_check": [
      "The active ADR register and archived ADR records were searched; ADR-0025 is adjacent but not the same decision."
    ]
  }
}
```


## Completion boundary

This generated design does not contain `DESIGN-REVIEW: PASS`, `REVIEW: PASS`, or implementation evidence. The project LLM must derive tasks and complete the normal Givn gates after the ordered bundle phases are understood.
