# Tasks: migrate-0-6-0-to-0-7-0

## Setup

- [ ] Confirm the existing Cucumber runner is strict, runs the permanent and
  change `givn/specs/**/*.feature` trees, and is configured by
  `givn/commands.yaml`; no new runner is needed for this maintenance change.
  Evidence: `tests/features_runner.rs` calls `.fail_on_skipped()` and collects
  permanent plus change specs; `givn/commands.yaml` configures
  `./run-tests.sh` and `./run-tests.sh --e2e`.
- [ ] Record the Givn version/configuration, the upgrade commit, and the prior
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

- [ ] RED: Add a temporary malformed feature to the change specs and run one
  regular scenario with `GIVN_RESULT_FILE` set. The current runner exits 0 and
  writes a reconciling result although the malformed file dropped its
  scenarios. Evidence: command and output showing exit 0 plus the result JSON.
- [ ] GREEN: Add the harness failure signal (`Writer::execution_has_failed()`)
  to the exit condition in `tests/features_runner.rs`, keeping the result
  write before the exit and correcting the stale panic message. Re-run the
  same command. Evidence: exit 1 with the result file still written.
- [ ] REFACTOR: Remove the temporary malformed feature; run the regular suite
  normally (without `GIVN_RESULT_FILE`) to confirm the runner still works
  outside Givn. Evidence: command and passing output.
- [ ] COMMIT: `<hash>` — `fix(givn): fail closed on runner parsing and hook errors`.

## Evidence and validation

- [ ] Write `runner-result-contract.md` with the scope inventory table
  (regular, e2e), the before/after commands, and the sample result JSON.
  Evidence: file present in the change directory.
- [ ] Validate the regular scope manually with a temporary result file and
  confirm the JSON parses and reconciles. Evidence: command and JSON output.
- [ ] Validate the e2e scope manually with a temporary result file and confirm
  the JSON parses and reconciles. Evidence: command and JSON output.
- [ ] COMMIT: `<hash>` — `docs(givn): record migrate-0-6-0-to-0-7-0 runner result contract`.

## Final gates

- [ ] Run `givn lint --change migrate-0-6-0-to-0-7-0`; exit 0 or 2.
- [ ] Run `givn check arc42-docs --change migrate-0-6-0-to-0-7-0`; passed.
- [ ] Write `review.md` with `REVIEW: PASS` after the fabrication audit, static
  checks, and the archive receipt.
- [ ] Archive this change; the receipt must show both scopes passed (or e2e
  `not_applicable`) and `status: "proven"`.
