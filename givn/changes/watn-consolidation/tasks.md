# Tasks: watn-consolidation

## Setup

- [x] Confirmed `tests/features_runner.rs` uses `.fail_on_skipped()` and runs
  both permanent specs and active change specs. Proof command:
  `./run-tests.sh --e2e --name "Repository-wide review accepts the consolidation dispositions"`.
- [x] Added and registered `tests/steps/watn_consolidation_steps.rs` as the
  single capability step file. New executable steps start with
  `unimplemented!()`.
- [x] Proved strictness with the pending consolidation E2E step: the targeted
  runner exited non-zero and reported `Step panicked. Captured output: not
  implemented` followed by `1 step failed`. The named-run removal guard is
  implemented and will be verified in the final gate.
- [x] Confirmed `run-tests.sh` and `measure-coverage.sh` exclude
  `@givn.removed`; the no-argument runner executes the retained non-E2E suite
  and the separate `--e2e` command executes the E2E suite.

## Scenario: F1 credential precedence duplicate is removed

- [x] RED: Baseline `givn lint` reported the duplicate title in
  `credential-sources` and `provider-setup`.
- [x] GREEN: Added the F1 `@givn.removed` delta targeting `provider-setup`;
  the canonical real-request scenario remains in `credential-sources` and
  `givn lint --change watn-consolidation` parses the removal cleanly.
- [x] REFACTOR: Re-ran the scoped lint and retained credential scenario; both
  remain green.
- [x] COMMIT: `0269099`.

## Scenario: F2 stale-search duplicate regular seam is removed

- [x] RED: Confirmed the duplicate title was present in the active tree and
  the removal delta was isolated to the regular seam.
- [x] GREEN: Applied the F2 removal and retained the four transitional regular
  bindings until archive; strengthened the separate terminal E2E boundary with
  exact newer/stale result assertions.
- [x] REFACTOR: Ran the retained stale-search E2E scenario; exact newer IDs
  remained and stale IDs were absent before worker teardown.
- [x] COMMIT: `cc4bdee`.

## Scenario: F3 missing-config subset is removed

- [x] RED: Confirmed the duplicate guidance contract in the baseline lint
  output and isolated the F3 removal delta.
- [x] GREEN: Applied the F3 removal; the stronger auto-init scenario remains
  the owner and proves that no config file is created.
- [x] REFACTOR: Ran the retained auto-init scenario and the non-E2E runner.
- [x] COMMIT: `8fadd07`.

## Scenario: F4 weaker Bash widget E2E is removed

- [x] RED: Confirmed both Bash widget contracts and the stronger request-
  preservation assertion in the baseline tree.
- [x] GREEN: Applied the F4 removal without changing the retained Bash E2E
  interaction.
- [x] REFACTOR: Ran the retained Bash E2E scenario and asserted shell output,
  request preservation, and non-evaluation.
- [x] COMMIT: `3010e28`.

## Scenario: F5 failed/empty Bash subset is removed

- [x] RED: Confirmed both failure/empty-output contracts in the baseline tree.
- [x] GREEN: Applied the F5 removal and retained the exact-buffer contract.
- [x] REFACTOR: Ran the retained shortcut failure scenario and the complete
  non-E2E runner.
- [x] COMMIT: `e593bae`.

## Scenario: F6 empty model subset is removed

- [x] RED: Confirmed both empty-picker contracts in the baseline tree.
- [x] GREEN: Applied the F6 removal and retained the picker scenario that also
  preserves the entered filter.
- [x] REFACTOR: Ran the retained picker scenario and full non-E2E runner.
- [x] COMMIT: pending until the scenario commit is created.

## Scenario: Failed archive preserves the fixture permanent specification tree

- [ ] RED: Leave the failure-path step pending and target this scenario only;
  the strict runner must exit non-zero.
- [ ] GREEN: Configure a deterministic failing fixture hook, invoke the real
  archive subprocess, and assert non-zero status plus byte-for-byte permanent
  tree preservation.
- [ ] REFACTOR: Re-run the failure scenario after cleanup; record exit 0 for
  the scenario's assertion of the expected command failure.
- [ ] COMMIT: pending.

## Scenario: Repository-wide review accepts the consolidation dispositions

- [ ] RED: Leave the review fixture step pending and target this E2E scenario;
  the strict runner must exit non-zero.
- [ ] GREEN: Create a fresh fixture, invoke `givn check review --change
  fixture-consolidation`, and assert exact disposition/net-delta stdout and
  exit status through the real subprocess.
- [ ] REFACTOR: Re-run the targeted E2E scenario; record exit 0.
- [ ] COMMIT: pending.

## Scenario: Archive publishes the consolidated permanent specifications

- [ ] RED: Leave the archive fixture step pending and target this E2E scenario;
  the strict runner must exit non-zero.
- [ ] GREEN: Invoke `givn archive --change fixture-consolidation` in a fresh
  fixture and assert archive stdout, canonical title presence, obsolete title
  absence, and no duplicate titles.
- [ ] REFACTOR: Re-run the targeted E2E scenario; record exit 0 and confirm the
  real Watn checkout is unchanged.
- [ ] COMMIT: pending.

## Final Gate Evidence

- [ ] Run `givn lint --change watn-consolidation` and record only expected WIP
  findings before implementation and clean output after implementation.
- [ ] Run `./run-tests.sh` and record the non-E2E scenario count.
- [ ] Run `./run-tests.sh --e2e` and record the strictly smaller scenario count.
- [ ] Run `./run-tests.sh` and record the complete scenario count.
- [ ] Run `./measure-coverage.sh` and `./merge-coverages.sh`; record merged
  line/branch output with runner and fixture processes included.
- [ ] Run `givn check review --change watn-consolidation` and
  `givn archive --change watn-consolidation`; record both gates.
