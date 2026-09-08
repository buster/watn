# Design: migrate-usecases-watn

## Migration phases

1. Record the clean worktree, permanent feature inventory, and coverage
   baseline.
2. Confirm the semantic mapping in `usecase-migration.md`.
3. Move capabilities with `git mv`, create use-case/fragment documents, and
   preserve feature text and tags.
4. Run lint, verification, E2E verification, and coverage comparison.
5. Archive this maintenance change without adding a migration feature to the
   permanent corpus.

## Target layout

```text
givn/specs/use-shell/
givn/specs/configure-provider/
givn/specs/configure-model/
givn/specs/configure-interactive/
givn/specs/fragments/
```

## Coverage contract

`coverage-before.json` and `coverage-after.json` are Givn behavior-hash
inventories. `source-coverage-before.json` and `source-coverage-after.json`
record line and branch counters. `givn check migration --change
migrate-usecases-watn` blocks missing behavior, lost E2E evidence, lower
interaction coverage, or lower source coverage.

## Active changes and ideation

Watn has no active changes and no `givn/ideation/` topic. No Persona or Handoff
operation is performed.
