# Tasks: migrate-0-5-0-to-0-6-0

## Setup

- [x] Confirm the existing Cucumber runner is strict, runs all permanent
  `givn/specs/` features, and is configured by `givn/commands.yaml`; no new
  runner or step files are needed for this verification-only migration. Evidence:
  `tests/features_runner.rs` calls `.fail_on_skipped()`, collects permanent
  `givn/specs/**/*.feature`, and `givn/commands.yaml` configures `./run-tests.sh`
  plus `./run-tests.sh --e2e`. Strictness proof: a temporary undefined-step
  scenario run with `./run-tests.sh --name 'Strict runner rejects an undefined
  step'` exited `101` with `1 scenario (1 failed)`, `1 step (1 failed)`, and
  `1 step failed`; the temporary feature was removed afterward.
- [x] Record the Givn version/configuration, active changes, active use-case and
  fragment roots, ideation topics, confirmed Personas, and archived migration
  baseline before verification. Evidence: `givn/config.yaml` declares 0.6.0;
  `givn status` reports this active change; five target root documents exist;
  no ideation/Persona files or legacy `group.md` files are tracked; the prior
  baseline is `givn/archive/migrate-usecases-watn/`.

## Verification: already-applied corpus

Domain constraints: stable use-case IDs own independent user goals; fragments
own reusable infrastructure; each capability has exactly one owner; typed
relationships must resolve; interactions map to one E2E scenario; historical
archives are not rewritten; absent Personas and ideation topics remain absent.

- [x] RED: N/A for the already-applied verification state; the audit command is
  fail-fast and would exit non-zero for a tracked legacy `group.md`, missing
  target root, active/archive modification, or active `group.md` reference.
- [x] GREEN: Record `VERIFY_ALREADY_MIGRATED` in `usecase-migration.md`, retain
  active files unchanged, and verify `givn lint` plus the inventory checks.
  Evidence: zero legacy roots/references, five target root documents, no active
  or archive diff, and `givn lint` exit 0.
- [x] REFACTOR: Confirm no active feature text, step binding, archived file, or
  product source changed; keep only canonical use-case/fragment paths. Evidence:
  `git status --short -- givn/specs givn/archive` is empty and the active
  feature tree remains unchanged.
- [x] COMMIT: `b4a9ea7` — verification implementation, migration ledger,
  evidence, and Arc42 updates; no product source was changed by design.

## Verification: behavior and coverage evidence

Domain constraints: migration must preserve scenario behavior hashes, E2E tags,
interaction mappings, and must not reduce source line/branch coverage unless an
explicit ledger retirement records a difference.

- [x] RED: Compare the current coverage inventory and source counters against
  the pre-verification baseline; the comparison must fail if any behavior,
  E2E mapping, interaction, or counter changes without a ledger disposition.
  Evidence: the inventory comparison passed with no missing behavior or lost
  E2E mapping; the source comparison uses the non-regression rule.
- [x] GREEN: Capture `coverage-before.json`, `coverage-after.json`,
  `coverage-comparison.json`, `source-coverage-before.json`,
  `source-coverage-after.json`, and `source-coverage-comparison.json`; run the
  configured non-E2E and E2E verification commands. Evidence: `./run-tests.sh`
  passed 155 scenarios/913 steps and `./run-tests.sh --e2e` passed 77
  scenarios/568 steps.
- [x] REFACTOR: Confirm the inventory contains the same 52 declared interaction
  mappings, every mapped entry is attached to an E2E scenario, source coverage
  has not regressed, branch counters are reported honestly by the current
  producer, and historical evidence remains untouched. Evidence: source
  coverage changed from 13487/14743 to 13489/14743 covered/valid lines, both
  runs report 0/0 branches, and no active spec or archive diff exists.
- [x] COMMIT: `b4a9ea7` — verification implementation, migration ledger,
  evidence, and Arc42 updates; no product source was changed by design.

## Final gates

- [x] Run `givn lint`; 26 active feature files checked and clean.
- [x] Run `givn check arc42-docs --change migrate-0-5-0-to-0-6-0`; passed.
- [x] Run `givn check review --change migrate-0-5-0-to-0-6-0` after review is
  written; verify, verify-e2e, integrity, run declaration, and net delta 0
  passed. Do not use the nonexistent `givn check migration` command.
- [x] Record `REVIEW: PASS` only after every evidence file and gate is complete;
  `review.md` ends with `REVIEW: PASS`.
