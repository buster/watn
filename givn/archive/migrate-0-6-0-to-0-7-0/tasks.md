# Tasks: migrate-0-6-0-to-0-7-0

## Setup

- [x] Confirm the existing Cucumber runner is strict, runs the permanent and
  change `givn/specs/**/*.feature` trees, and is configured by
  `givn/commands.yaml`; no new runner is needed for this maintenance change.
  Evidence: `tests/features_runner.rs` calls `.fail_on_skipped()` and collects
  permanent plus change specs; `givn/commands.yaml` configures
  `./run-tests.sh` and `./run-tests.sh --e2e`. Strictness proof: a temporary
  scenario `Strict runner rejects an undefined step` ran with
  `./run-tests.sh --name 'Strict runner rejects an undefined step'` and exited
  1 with `1 scenario (1 failed)`, `1 step (1 failed)`; the temporary feature
  was removed afterward.
- [x] Record the Givn version/configuration, the upgrade commit, and the prior
  proven receipt before touching the runner. Evidence: `givn --version` is
  0.7.0, `givn/config.yaml` declares `givn_config_version: "0.7.0"`, the
  managed preparation is `29b032cc1d6c10b0e87c4e05fd286dfdd4777acb`, and
  `givn/archive/setup-shell-and-style/verification.json` is `proven` with
  regular 222/222 and e2e 88/88.

## Runner result contract

Domain constraints: counts come from the runner's own scenario statistics,
never hardcoded or inferred from the exit code; `scope` is exactly `regular`
or `e2e`; the result is written whenever the runner produced counts, even on
failure; the exit code reflects the run including parsing and hook errors;
Givn's gate is fail-closed.

- [x] RED: Add a temporary malformed feature to the change specs and run one
  regular scenario with `GIVN_RESULT_FILE` set. The current runner exits 0 and
  writes a reconciling result although the malformed file dropped its
  scenarios. Evidence: command and output showing exit 0 plus the result JSON.
  `GIVN_RESULT_FILE=/tmp/givn-red.json ./run-tests.sh --name 'Custom
  OpenAI-compatible provider from config'` reported `1 scenario (1 passed)`,
  `1 parsing error`, exited 0, and wrote
  `{"failed":0,"passed":1,"scope":"regular","skipped":0,"total":1}`.
- [x] GREEN: Add the harness failure signal (`Writer::execution_has_failed()`)
  to the exit condition in `tests/features_runner.rs`, keeping the result
  write before the exit and correcting the stale panic message. Re-run the
  same command. Evidence: exit 1 with the result file still written.
  `writer.execution_has_failed()` is provided by cucumber's `Stats` trait,
  imported as `StatsWriter`. The same command exited 1 while still writing
  `{"failed":0,"passed":1,"scope":"regular","skipped":0,"total":1}`.
- [x] REFACTOR: Remove the temporary malformed feature; run the regular suite
  normally (without `GIVN_RESULT_FILE`) to confirm the runner still works
  outside Givn. Evidence: command and passing output.
  `./run-tests.sh` reported `21 features`, `222 scenarios (222 passed)`,
  `1366 steps (1366 passed)`, exit 0; the temporary feature was removed.
- [x] COMMIT: `6c52d08` — `fix(givn): fail closed on runner parsing and hook errors`.

## Evidence and validation

- [x] Write `runner-result-contract.md` with the scope inventory table
  (regular, e2e), the before/after commands, and the sample result JSON.
  Evidence: `givn/changes/migrate-0-6-0-to-0-7-0/runner-result-contract.md`
  contains the two scope rows, the pre-fix malformed-feature evidence, the
  post-fix exit-1 evidence, and both sample results.
- [x] Validate the regular scope manually with a temporary result file and
  confirm the JSON parses and reconciles. Evidence:
  `result=$(mktemp); GIVN_RESULT_FILE="$result" ./run-tests.sh` reported
  `222 scenarios (222 passed)`, exit 0, and wrote
  `{"failed":0,"passed":222,"scope":"regular","skipped":0,"total":222}`.
- [x] Validate the e2e scope manually with a temporary result file and confirm
  the JSON parses and reconciles. Evidence:
  `result=$(mktemp); GIVN_RESULT_FILE="$result" ./run-tests.sh --e2e` reported
  `88 scenarios (88 passed)`, exit 0, and wrote
  `{"failed":0,"passed":88,"scope":"e2e","skipped":0,"total":88}`.
- [x] COMMIT: `ffc6cfc` — `docs(givn): record migrate-0-6-0-to-0-7-0 runner result contract`.

## Final gates

- [x] Run `givn lint --change migrate-0-6-0-to-0-7-0`; exit 0 or 2.
  Evidence: exit 0 (no `.feature` files in the change; specs are not
  applicable).
- [x] Run `givn check arc42-docs --change migrate-0-6-0-to-0-7-0`; passed.
  Evidence: exit 0.
- [x] Write `review.md` with `REVIEW: PASS` after the fabrication audit, static
  checks, and the archive receipt. Evidence: `review.md` records the clean
  fabrication audit (0 empty step bodies across 31 files), the Arc42
  conformance table, `README-IMPACT: none`, and `REVIEW: PASS`;
  `givn check review --change migrate-0-6-0-to-0-7-0` exits 0.
- Completion boundary: this change is done only when `givn archive` succeeds
  and `givn/archive/migrate-0-6-0-to-0-7-0/verification.json` shows both scopes
  passed (or e2e `not_applicable`) with `status: "proven"`.
