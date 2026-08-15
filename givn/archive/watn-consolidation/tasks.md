# Tasks: watn-consolidation

## Setup

- [x] Confirmed `tests/features_runner.rs` uses `.fail_on_skipped()` and runs
  both permanent specs and active change specs. Proof command:
  `./run-tests.sh --e2e --name "Repository-wide review accepts the consolidation dispositions"`.
- [x] Added and registered separate capability bindings:
  `tests/steps/watn_consolidation_steps.rs` for the non-E2E rollback scenario,
  `tests/steps/watn_consolidation_e2e_steps.rs` for the CLI smoke scenarios,
  and `tests/steps/watn_consolidation_support.rs` for shared fixture helpers.
  New executable steps started with `unimplemented!()`.
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
- [x] COMMIT: `54d4d97` for the task record; the F5 delta was introduced in
  `3010e28` with the adjacent F4 delta, and its retained-buffer assertion was
  refactored in `fae8415`.

## Scenario: F6 empty model subset is removed

- [x] RED: Confirmed both empty-picker contracts in the baseline tree.
- [x] GREEN: Applied the F6 removal and retained the picker scenario that also
  preserves the entered filter.
- [x] REFACTOR: Ran the retained picker scenario and full non-E2E runner.
- [x] COMMIT: `e593bae`.

## Scenario: Failed archive preserves the fixture permanent specification tree

- [x] RED: The initial consolidation step skeleton failed non-zero under
  `.fail_on_skipped()` before implementation.
- [x] GREEN: Configured a deterministic failing fixture hook, invoked the real
  archive subprocess, and asserted non-zero status plus byte-for-byte permanent
  tree preservation. The targeted scenario passed four steps.
- [x] REFACTOR: Re-ran the failure scenario after cleanup; one scenario and
  four steps passed with exit 0 for the expected command failure.
- [x] COMMIT: `9379c3e`.

## Scenario: Repository-wide review accepts the consolidation dispositions

- [x] RED: The initial consolidation step skeleton failed non-zero under
  `.fail_on_skipped()` before implementation.
- [x] GREEN: Created a fresh fixture, invoked `givn check review --change
  fixture-consolidation`, and asserted exact disposition/net-delta stdout and
  exit status through the real subprocess. The targeted E2E scenario passed
  five steps with an absolute `GIVN_BIN`.
- [x] REFACTOR: Re-ran the targeted E2E scenario after extracting shared
  fixture stdout handling; one scenario and five steps passed.
- [x] COMMIT: `1034be9`.

## Scenario: Archive publishes the consolidated permanent specifications

- [x] RED: The initial consolidation step skeleton failed non-zero under
  `.fail_on_skipped()` before implementation.
- [x] GREEN: Invoked `givn archive --change fixture-consolidation` in a fresh
  fixture and asserted archive stdout, canonical title presence, obsolete title
  absence, and no duplicate titles through the real subprocess.
- [x] REFACTOR: Re-ran the targeted E2E scenario after extracting duplicate-title
  verification; one scenario and seven steps passed with an absolute `GIVN_BIN`,
  and the real Watn checkout remained unchanged.
- [x] COMMIT: `c021902`.

## Final Gate Evidence

- [x] `givn lint --change watn-consolidation` was clean before archive; the
  post-archive repository-wide lint also completed cleanly with only existing
  shape/subset/long-scenario advisory output.
- [x] `./run-tests.sh` passes `149` scenarios and `855` steps.
- [x] `./run-tests.sh --e2e` passes `77` scenarios and `567` steps; the E2E
  count is strictly smaller than the combined `226` scenarios and `1422`
  steps.
- [x] The complete suite is the combination of the non-E2E and E2E commands;
  both pass with the isolated consolidation fixture and the permanent Watn
  suite.
- [x] Pre-archive `./measure-coverage.sh` and `./merge-coverages.sh` produced a
  fresh merged report: 13042/14210 lines (91.78%) and 0/0 branches (n/a),
  including the Gherkin runner and fixture subprocesses.
- [x] `givn check review --change watn-consolidation` passed verify, verify-e2e,
  integrity, and overlap dispositions; it reported `net delta: 3 added,
  0 modified, 6 removed`.
- [x] Confirmed the archive prerequisites after the signed review: all tracked
  tasks have evidence, `review.md` contains `REVIEW: PASS`, lint is clean, and
  the configured verify and verify-e2e hooks pass.
- [x] `givn archive --change watn-consolidation` succeeded and merged the delta
  into `givn/specs/`; the change moved to `givn/archive/watn-consolidation/`.
- [x] The archive hook observed `78` E2E scenarios and `574` steps while the
  transaction was still carrying the active delta; the direct post-archive
  E2E rerun passed `76` scenarios and `562` steps against the permanent tree.
  The archive coverage gate published `13011/14210` lines (`92%`) and `0/0`
  branches (`n/a`); the fresh post-archive coverage rerun produced
  `13012/14210` lines (`91.57%`) and `0/0` branches (`n/a`).
- [x] Archive updated the README coverage badge and merged coverage summary;
  the merged summary now matches the fresh post-archive report; the permanent
  tree contains the canonical retained scenarios and no removed placeholder
  scenario.
