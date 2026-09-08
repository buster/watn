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
agent must stop on a dirty worktree, record the Givn version/configuration,
enumerate active changes and ideation state, and write baselines before edits.

## Required ledger

Create `usecase-migration.md` before applying moves. Every old scenario appears
exactly once in this table or is explicitly retired:

| Old entry | Old owner | Operation | New owner | New capability | Old @e2e | New @e2e | Behavior changed | Reason |
|---|---|---|---|---|---:|---:|---|---|

Allowed operations are `MOVE`, `SPLIT`, `MERGE`, `REASSIGN_CAPABILITY`,
`CLASSIFY_AS_FRAGMENT`, `CREATE`, `RENAME`, `RETIRE`, and
`UPDATE_RELATIONSHIP`.

## Phases

1. Preflight and baselines.
2. Semantic corpus inventory and human confirmation.
3. Move capabilities with `git mv`; create complete `usecase.md` and fragment
   documents; update typed relationships and capability-aware interactions.
4. Migrate active change paths, proposals, designs, reviews, and tasks.
5. Preserve ideation topics, map stable IDs, and present Handoff and Persona
   promotion decisions without executing either automatically.
6. Run `givn spec coverage --format json`, compare the evidence, run the
   configured verification and E2E commands, and compare source line/branch
   counters.
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

`givn check migration --change <migration-id>` is the final machine gate.


## Completion boundary

This generated design does not contain `DESIGN-REVIEW: PASS`, `REVIEW: PASS`, or implementation evidence. The project LLM must derive tasks and complete the normal Givn gates after the ordered bundle phases are understood.
