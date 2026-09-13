# Tasks: explain-failure-guidance

Owning use case: `use-shell` (`givn/specs/use-shell/usecase.md`).
Capability: `explain-command` (extended).
Delta spec: `givn/changes/explain-failure-guidance/specs/use-shell/explain-command.feature` (18 scenarios: 11 modified, 7 added).
Design: `givn/changes/explain-failure-guidance/design.md`.

## Domain constraints (apply to every scenario)

- An Explained command is never generated, edited, accepted, or executed.
- The card still opens after a failed/unusable explanation so the command stays reviewable.
- Setup delegation reuses the existing quick setup and setup wizard unchanged; no new setup surface.
- Diagnostics are plain English with no review-path anti-terms (no "candidate").
- Exit codes come from the shared `exit_code` mapping.
- Persona review lens: `terminal-developer--interactive` (not a Gherkin actor).

## Setup task

- [x] Step skeletons: extend the existing per-capability files `tests/steps/explain_command_steps.rs` (regular) and `tests/steps/explain_command_e2e_steps.rs` (e2e). New step bodies start as `unimplemented!()`.
  - Evidence:
    ```
    $ cargo check --locked --all-targets --features test-support
        Checking watn v0.4.1 (/home/buster/projects/watn)
        Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.38s
    # 18 new step bodies in tests/steps/explain_command_steps.rs are `unimplemented!()`:
    configured_provider_no_stage_purposes, configured_provider_endpoint_refuses_connections,
    configured_provider_no_default_model, run_explain_let_setup_flow_start,
    run_explain_let_quick_setup_start, run_explain_stdin_setup_guidance,
    run_explain_interrupt_request, run_explain_provider_missing,
    run_explain_provider_openrouter, run_explain_provider_custom, abandon_setup_flow,
    setup_flow_should_start, watn_reports_setup_complete_rerun, watn_reports_setup_required,
    watn_reports_unknown_provider, watn_reports_missing_credential,
    watn_reports_explanation_request_failed, watn_reports_explanation_not_usable
    ```
- [x] Runner and strict mode unchanged and already proven: `./run-tests.sh` / `./run-tests.sh --e2e`, cucumber-rs `.fail_on_skipped()` (`tests/features_runner.rs:210`), skipped -> exit 1.
  - Evidence:
    ```
    # temporary probe scenario with an undefined step, then removed:
    $ ./run-tests.sh --name "Strictness probe scenario"
      Scenario: Strictness probe scenario
       ✘  Given a step that is not defined anywhere in this suite
          Step doesn't match any function
    [Summary]
    1 feature
    1 scenario (1 failed)
    1 step (1 failed)
    exit=1
    ```
- [ ] Permanent-spec lockstep (design.md `## Permanent-spec lockstep`): re-base the modified permanent scenarios in `givn/specs/use-shell/explain-command.feature` to a ready provider in the same commit as their delta GREEN, so the permanent copies do not fail under the new readiness gate. The permanent `@e2e` scenario is unaffected.
  - Evidence:
    ```
    # after the readiness gate landed (S1), the full suite identified exactly these
    # stale permanent copies to re-base in lockstep:
    $ ./run-tests.sh
    239 scenarios (232 passed, 7 failed)
    # S2 Shell metacharacters..., S3 A command beginning with a dash..., S4 A command
    # read from standard input..., S5 A piped command..., S6 A positional command...,
    # S7 The review-panel switches..., S9 A configured provider without a usable
    # credential... (S8's permanent copy already seeds a ready provider; re-based
    # for text lockstep in S8)
    # re-based one by one in S2..S9; verified green by the final full-suite run
    ```
- [x] Confirm `verify.command` and `verify.e2e_command` are unchanged (`./run-tests.sh`, `./run-tests.sh --e2e`) and the current suite is green before starting.
  - Evidence:
    ```
    $ cat givn/commands.yaml
    verify:
      command: "./run-tests.sh"
      e2e_command: "./run-tests.sh --e2e"

    $ ./run-tests.sh
    [Summary]
    22 features
    238 scenarios (238 passed)
    1442 steps (1442 passed)
    exit=0

    $ ./run-tests.sh --e2e
    [Summary]
    25 features
    89 scenarios (89 passed)
    682 steps (682 passed)
    exit=0
    ```

## S1: Enter closes the explanation card without releasing the command (@givn.modified)

- [x] RED: remove `@wip`; run `./run-tests.sh --name "Enter closes the explanation card without releasing the command"`; must exit non-zero.
  - Evidence:
    ```
    $ ./run-tests.sh --name "Enter closes the explanation card without releasing the command"
      Scenario: Enter closes the explanation card without releasing the command
       ✘  Given a configured provider whose explanation returns no stage purposes
          Step failed:
          Matched: tests/steps/explain_command_steps.rs:462:1
          Step panicked. Captured output: not implemented
    [Summary]
    2 features
    2 scenarios (1 passed, 1 failed)
    6 steps (5 passed, 1 failed)
    exit=1
    ```
- [x] GREEN: production readiness/outcome plumbing (`src/main.rs`, `src/review/session.rs`); re-base the permanent scenario. Run targeted; zero.
  - Evidence:
    ```
    $ ./run-tests.sh --name "Enter closes the explanation card without releasing the command"
    [Summary]
    2 features
    2 scenarios (2 passed)
    10 steps (10 passed)
    exit=0
    # permanent copy re-based to "Given a configured provider whose explanation returns no stage purposes"
    # src/main.rs: readiness gate (stdin terminal -> quicksetup/wizard/guidance) + strict provider/model/credential resolution
    ```
- [x] REFACTOR: re-run; zero.
  - Evidence:
    ```
    $ ./run-tests.sh --name "Enter closes the explanation card without releasing the command"
    [Summary]
    2 features
    2 scenarios (2 passed)
    10 steps (10 passed)
    exit=0
    ```
- [x] COMMIT: `feat(explain-command): Enter closes the explanation card without releasing the command` -> hash: `c333742`

## S2: Shell metacharacters and embedded quoting reach the card unchanged (@givn.modified)

- [x] RED/GREEN/REFACTOR/COMMIT as above with `./run-tests.sh --name "Shell metacharacters and embedded quoting reach the card unchanged"`.
  - Evidence:
    ```
    RED: $ ./run-tests.sh --name "Shell metacharacters and embedded quoting reach the card unchanged"
      Scenario: Shell metacharacters and embedded quoting reach the card unchanged
       ✔  Given no config file exists
       ✘  When I run `watn explain` with this single argument:
          Step panicked. Captured output: PTY did not render label "esc close"; output: ""
    [Summary]
    2 scenarios (1 passed, 1 failed)
    9 steps (8 passed, 1 failed)
    exit=1

    GREEN: $ ./run-tests.sh --name "Shell metacharacters and embedded quoting reach the card unchanged"
    [Summary]
    2 features
    2 scenarios (2 passed)
    14 steps (14 passed)
    exit=0

    REFACTOR: $ ./run-tests.sh --name "Shell metacharacters and embedded quoting reach the card unchanged"
    [Summary]
    2 features
    2 scenarios (2 passed)
    14 steps (14 passed)
    exit=0
    # permanent copy re-based to the ready-provider Given in the same commit
    ```
- [x] COMMIT hash: `de2a82e`

## S3: A command beginning with a dash passes after the option terminator (@givn.modified)

- [x] RED/GREEN/REFACTOR/COMMIT with the same scenario title.
  - Evidence:
    ```
    RED: $ ./run-tests.sh --name "A command beginning with a dash passes after the option terminator"
      Scenario: A command beginning with a dash passes after the option terminator
       ✔  Given a configured provider whose explanation returns no stage purposes
       ✔  When I run `watn explain --` with this single argument:
       ✔  Then the explanation card should show the stage:
    Feature: Explain an existing command
      Scenario: A command beginning with a dash passes after the option terminator
       ✔  Given no config file exists
       ✘  When I run `watn explain --` with this single argument:
          Step panicked. Captured output: PTY did not render label "esc close"; output: ""
    exit=1

    GREEN: $ ./run-tests.sh --name "A command beginning with a dash passes after the option terminator"
    [Summary]
    2 features
    2 scenarios (2 passed)
    6 steps (6 passed)
    exit=0

    REFACTOR: $ ./run-tests.sh --name "A command beginning with a dash passes after the option terminator"
    [Summary]
    2 features
    2 scenarios (2 passed)
    6 steps (6 passed)
    exit=0
    ```
- [x] COMMIT hash: `55e5123`

## S4: A command read from standard input keeps its quoting and line breaks (@givn.modified)

- [x] RED/GREEN/REFACTOR/COMMIT with the same scenario title.
  - Evidence:
    ```
    RED: $ ./run-tests.sh --name "A command read from standard input keeps its quoting and line breaks"
      Scenario: A command read from standard input keeps its quoting and line breaks
       ✘  When I run `watn explain -` in a terminal with this command on standard input:
          Step panicked. Captured output: explain temp dir
    Feature: Explain an existing command
       ✘  Then the explanation card should show the stage:
          Step panicked. Captured output: explanation card should show "awk '{print $1}' access.log", got:
    exit=1

    GREEN: $ ./run-tests.sh --name "A command read from standard input keeps its quoting and line breaks"
    [Summary]
    2 features
    2 scenarios (2 passed)
    10 steps (10 passed)
    exit=0
    # ready-provider Given now materializes the isolated config/temp dir; permanent copy re-based

    REFACTOR: $ ./run-tests.sh --name "A command read from standard input keeps its quoting and line breaks"
    [Summary]
    2 features
    2 scenarios (2 passed)
    10 steps (10 passed)
    exit=0

    regression: $ ./run-tests.sh --name "Enter closes the explanation card|Shell metacharacters and embedded"
    [Summary]
    2 features
    4 scenarios (4 passed)
    24 steps (24 passed)
    exit=0
    ```
- [x] COMMIT hash: `4541b8a`

## S5: A piped command without the marker is explained (@givn.modified)

- [x] RED/GREEN/REFACTOR/COMMIT with the same scenario title.
  - Evidence:
    ```
    RED: $ ./run-tests.sh --name "A piped command without the marker is explained"
       ✘  Then the explanation card should show the stage:
          Step panicked. Captured output: explanation card should show "ls", got:
    exit=1

    GREEN: $ ./run-tests.sh --name "A piped command without the marker is explained"
    [Summary]
    2 features
    2 scenarios (2 passed)
    10 steps (10 passed)
    exit=0

    REFACTOR: $ ./run-tests.sh --name "A piped command without the marker is explained"
    [Summary]
    2 features
    2 scenarios (2 passed)
    10 steps (10 passed)
    exit=0
    ```
- [x] COMMIT hash: `c75bb93`

## S6: A positional command takes precedence over standard input (@givn.modified)

- [x] RED/GREEN/REFACTOR/COMMIT with the same scenario title.
  - Evidence:
    ```
    RED: $ ./run-tests.sh --name "A positional command takes precedence over standard input"
       ✘  Then the explanation card should show the stage:
          Step panicked. Captured output: explanation card should show "echo positional", got:
    exit=1

    GREEN: $ ./run-tests.sh --name "A positional command takes precedence over standard input"
    [Summary]
    2 features
    2 scenarios (2 passed)
    8 steps (8 passed)
    exit=0

    REFACTOR: $ ./run-tests.sh --name "A positional command takes precedence over standard input"
    [Summary]
    2 features
    2 scenarios (2 passed)
    8 steps (8 passed)
    exit=0
    ```
- [x] COMMIT hash: `80e8d8f`

## S7: The review-panel switches are inert for explain (@givn.modified)

- [x] RED/GREEN/REFACTOR/COMMIT with the same scenario title.
  - Evidence:
    ```
    RED: $ ./run-tests.sh --name "The review-panel switches are inert for explain"
       ✘  When I run `watn explain --no-review-panel` with this single argument:
          Step panicked. Captured output: PTY did not render label "esc close"; output: ""
    exit=1

    GREEN: $ ./run-tests.sh --name "The review-panel switches are inert for explain"
    [Summary]
    2 features
    2 scenarios (2 passed)
    6 steps (6 passed)
    exit=0

    REFACTOR: $ ./run-tests.sh --name "The review-panel switches are inert for explain"
    [Summary]
    2 features
    2 scenarios (2 passed)
    6 steps (6 passed)
    exit=0
    ```
- [x] COMMIT hash: `f90581b`

## S8: The explanation card opens even when the review surface is disabled (@givn.modified)

- [x] RED/GREEN/REFACTOR/COMMIT with the same scenario title.
  - Evidence:
    ```
    RED: $ ./run-tests.sh --name "The explanation card opens even when the review surface is disabled"
      Scenario: The explanation card opens even when the review surface is disabled
       ✔  Given a configured provider whose explanation returns no stage purposes
       ✔  And the persisted review surface is disabled
       ✔  When I run `watn explain` with this single argument:
       ✔  Then the explanation card should show the stage:
       ✔  And the review surface should be disabled in the configuration
    Feature: Explain an existing command
      Scenario: The explanation card opens even when the review surface is disabled
       ✔  Given the persisted review surface is disabled
       ...
    [Summary]
    2 features
    2 scenarios (2 passed)
    9 steps (9 passed)
    exit=0
    # ANOMALY (recorded, not fabricated): this permanent copy does not fail under
    # the readiness gate because its `the persisted review surface is disabled`
    # fixture calls ensure_test_env, which already writes a ready provider
    # (provider "test", api key "test-key", default_model "test-model"). The
    # design's lockstep list assumed it assumed an unconfigured machine; it does
    # not. The scenario is a pure text lockstep re-base with no failing RED.

    GREEN: $ ./run-tests.sh --name "The explanation card opens even when the review surface is disabled"
    [Summary]
    2 features
    2 scenarios (2 passed)
    10 steps (10 passed)
    exit=0
    # permanent copy re-based to carry the explicit ready-provider Given in lockstep

    REFACTOR: $ ./run-tests.sh --name "The explanation card opens even when the review surface is disabled"
    [Summary]
    2 features
    2 scenarios (2 passed)
    10 steps (10 passed)
    exit=0
    ```
- [x] COMMIT hash: `d8e6da8`

## S9: A configured provider without a usable credential is not contacted (@givn.modified)

- [x] RED: remove `@wip`; targeted run must fail (setup-delegation steps unimplemented).
  - Evidence:
    ```
    $ ./run-tests.sh --name "A configured provider without a usable credential is not contacted"
      Scenario: A configured provider without a usable credential is not contacted
       ✘  When I run `watn explain` with this single argument and let the setup flow start:
          Matched: tests/steps/explain_command_steps.rs:480:1
          Step panicked. Captured output: not implemented
    Feature: Explain an existing command
       ✘  When I run `watn explain` with this single argument:
          Step panicked. Captured output: PTY did not render label "esc close"; output: "warning: config file is world-readable (644)\r\n"
    [Summary]
    2 scenarios (2 failed)
    6 steps (4 passed, 2 failed)
    exit=1
    ```
- [x] GREEN: readiness delegation in `run_explain_command` (terminal stdin + config -> setup wizard; after cancel -> exit 1); new setup steps; permanent lockstep. Targeted run zero.
  - Evidence:
    ```
    $ ./run-tests.sh --name "A configured provider without a usable credential is not contacted"
      Scenario: A configured provider without a usable credential is not contacted
       ✔  Given a configured provider without a usable credential
       ✔  When I run `watn explain` with this single argument and let the setup flow start:
       ✔  Then the setup flow should start
       ✔  And no explanation card should open
       ✔  And no provider request should have been made
       ✔  When I abandon the setup flow
       ✔  Then the exit status should be 1
    [Summary]
    2 features
    2 scenarios (2 passed)
    14 steps (14 passed)
    exit=0
    # new steps: let the setup flow start / the setup flow should start / I abandon the setup flow;
    # no explanation card should open now also inspects the live PTY snapshot; permanent copy re-based
    ```
- [x] REFACTOR: re-run; zero.
  - Evidence:
    ```
    $ ./run-tests.sh --name "A configured provider without a usable credential is not contacted"
    [Summary]
    2 features
    2 scenarios (2 passed)
    14 steps (14 passed)
    exit=0
    ```
- [x] COMMIT: `feat(explain-command): A configured provider without a usable credential is not contacted` -> hash: `0104fc0`

## S10: A failed explanation keeps the command reviewable (@givn.modified)

- [x] RED: remove `@wip`; targeted run must fail.
  - Evidence:
    ```
    $ ./run-tests.sh --name "A failed explanation keeps the command reviewable"
      Scenario: A failed explanation keeps the command reviewable
       ✘  And watn should report that the explanation request failed
          Matched: tests/steps/explain_command_steps.rs:592:1
          Step panicked. Captured output: not implemented
    [Summary]
    9 steps (8 passed, 1 failed)
    exit=1
    ```
- [x] GREEN: request-failure outcome + stderr diagnostic + exit code after the card closes (`src/main.rs`, `src/review/session.rs`). Targeted run zero.
  - Evidence:
    ```
    $ ./run-tests.sh --name "A failed explanation keeps the command reviewable"
      Scenario: A failed explanation keeps the command reviewable
       ✔  Given a configured provider whose explanation request fails
       ✔  When I run `watn explain` with this single argument:
       ✔  Then the explanation card should show the stage:
       ✔  And the explanation card should show purpose-unavailable
       ✔  And watn should report that the explanation request failed
       ✔  And the exit status should be 2
    [Summary]
    2 features
    2 scenarios (2 passed)
    12 steps (12 passed)
    exit=0
    # ExplanationOutcome + apply_explanation_outcome (src/review/response.rs);
    # explain_command_candidate -> Result<ExplanationOutcome, Error> (src/review/session.rs);
    # failure status applied after run_explanation_card (src/main.rs); permanent copy re-based
    $ cargo test --locked --lib
    test result: ok. 87 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
    ```
- [x] REFACTOR: re-run; zero.
  - Evidence:
    ```
    $ ./run-tests.sh --name "A failed explanation keeps the command reviewable"
    [Summary]
    2 features
    2 scenarios (2 passed)
    12 steps (12 passed)
    exit=0
    ```
- [x] COMMIT: `feat(explain-command): A failed explanation keeps the command reviewable` -> hash: `aeba739`

## S11: A network failure reports the mapped exit status (@givn.added)

- [x] RED: remove `@wip`; targeted run must fail.
  - Evidence:
    ```
    $ ./run-tests.sh --name "A network failure reports the mapped exit status"
      Scenario: A network failure reports the mapped exit status
       ✘  Given a configured provider whose endpoint refuses connections
          Matched: tests/steps/explain_command_steps.rs:474:1
          Step panicked. Captured output: not implemented
    exit=1
    ```
- [x] GREEN: connection-refused mock/step; assert exit 3. Targeted run zero.
  - Evidence:
    ```
    $ ./run-tests.sh --name "A network failure reports the mapped exit status"
      Scenario: A network failure reports the mapped exit status
       ✔  Given a configured provider whose endpoint refuses connections
       ✔  When I run `watn explain` with this single argument:
       ✔  Then the explanation card should show the stage:
       ✔  And the explanation card should show purpose-unavailable
       ✔  And watn should report that the explanation request failed
       ✔  And the exit status should be 3
    [Summary]
    1 feature
    1 scenario (1 passed)
    6 steps (6 passed)
    exit=0
    # closed ephemeral port recorded, listener dropped, ready-provider config written against it
    ```
- [x] REFACTOR: re-run; zero.
  - Evidence:
    ```
    $ ./run-tests.sh --name "A network failure reports the mapped exit status"
    [Summary]
    1 feature
    1 scenario (1 passed)
    6 steps (6 passed)
    exit=0
    ```
- [x] COMMIT: `feat(explain-command): A network failure reports the mapped exit status` -> hash: `6f943bc`

## S12: An explanation that does not cover the command is not trusted (@givn.modified)

- [x] RED: remove `@wip`; targeted run must fail.
  - Evidence:
    ```
    $ ./run-tests.sh --name "An explanation that does not cover the command is not trusted"
      Scenario: An explanation that does not cover the command is not trusted
       ✘  And watn should report that the explanation response was not usable
          Matched: tests/steps/explain_command_steps.rs:609:1
          Step panicked. Captured output: not implemented
    exit=1
    ```
- [x] GREEN: unusable-response diagnostic with explain-domain wording (no anti-terms); exit 0. Targeted run zero.
  - Evidence:
    ```
    $ ./run-tests.sh --name "An explanation that does not cover the command is not trusted"
      Scenario: An explanation that does not cover the command is not trusted
       ✔  Given a configured provider whose explanation does not cover the command
       ✔  When I run `watn explain` with this single argument:
       ✔  Then the explanation card should show the stage:
       ✔  And the explanation card should show purpose-unavailable
       ✔  And the explanation card should not show the text "unrelated stage"
       ✔  And watn should report that the explanation response was not usable
       ✔  And watn should exit successfully
    [Summary]
    2 features
    2 scenarios (2 passed)
    14 steps (14 passed)
    exit=0
    # ReviewResponseError::explain_reason() added; diagnostic "explain response was not usable: ..."
    # printed on stderr with exit 0; step also guards against the "candidate" anti-term;
    # permanent copy re-based
    $ cargo test --locked --lib review::response
    test result: ok. 18 passed; 0 failed; 0 ignored; 0 measured; 70 filtered out
    ```
- [x] REFACTOR: re-run; zero.
  - Evidence:
    ```
    $ ./run-tests.sh --name "An explanation that does not cover the command is not trusted"
    [Summary]
    2 features
    2 scenarios (2 passed)
    14 steps (14 passed)
    exit=0
    ```
- [x] COMMIT: `feat(explain-command): An explanation that does not cover the command is not trusted` -> hash: `1b3440d`

## S13: An unconfigured machine starts quick setup instead of explaining (@givn.added)

- [x] RED: remove `@wip`; targeted run must fail.
  - Evidence:
    ```
    $ ./run-tests.sh --name "An unconfigured machine starts quick setup instead of explaining"
      Scenario: An unconfigured machine starts quick setup instead of explaining
       ✘  When I run `watn explain` with this single argument and let quick setup start:
          Matched: tests/steps/explain_command_steps.rs:506:1
          Step panicked. Captured output: not implemented
    exit=1
    ```
- [x] GREEN: quick-setup delegation on a clean machine (terminal stdin, no config), rerun hint, exit 0; reused quicksetup steps typed correctly. Targeted run zero.
  - Evidence:
    ```
    $ ./run-tests.sh --name "An unconfigured machine starts quick setup instead of explaining"
      Scenario: An unconfigured machine starts quick setup instead of explaining
       ✔  Given no config file exists
       ✔  When I run `watn explain` with this single argument and let quick setup start:
       ✔  Then the quick setup should announce that no configuration was found
       ✔  When I accept the suggested endpoint
       ✔  And I answer the credential with "sk-quick-test"
       ✔  And I accept the suggested small model
       ✔  And I accept the pre-filled normal model
       ✔  And I accept the pre-filled thinking model
       ✔  And I keep the pre-selected shell integrations and confirm
       ✔  Then watn should exit successfully
       ✔  And watn should report that setup is complete and the command must be rerun
       ✔  And no provider request should have been made
       ✔  And no explanation card should open
    [Summary]
    1 feature
    1 scenario (1 passed)
    13 steps (13 passed)
    exit=0
    # quick-setup branch now prints the rerun hint and exits 0; new quick-setup start
    # step and rerun-hint assertion step
    ```
- [x] REFACTOR: re-run; zero.
  - Evidence:
    ```
    $ ./run-tests.sh --name "An unconfigured machine starts quick setup instead of explaining"
    [Summary]
    1 feature
    1 scenario (1 passed)
    13 steps (13 passed)
    exit=0
    ```
- [x] COMMIT: `feat(explain-command): An unconfigured machine starts quick setup instead of explaining` -> hash: `1be1e10`

## S14: An existing configuration without a usable model completes the setup wizard (@givn.added)

- [x] RED: remove `@wip`; targeted run must fail.
  - Evidence:
    ```
    $ ./run-tests.sh --name "An existing configuration without a usable model completes the setup wizard"
      Scenario: An existing configuration without a usable model completes the setup wizard
       ✘  When I run `watn explain` with this single argument and let the setup flow start:
          Matched: tests/steps/explain_command_steps.rs:492:1
          Step panicked. Captured output: PTY did not render label "watn · setup"; output:
          "... Asking test-model · 0.0s ... explain response was not usable: invalid explanation JSON ...
           ┌ watn · review ... git log --oneline | head -5 ... purpose-unavailable ... esc close ..."
    exit=1
    # the harness rebuild made the catalog fixture ready; explain opened the card
    # instead of the wizard. RED is the real failure.
    ```
- [x] GREEN: wizard delegation (terminal stdin, config exists), rerun hint, exit 0, no card, no request. Targeted run zero.
  - Evidence:
    ```
    $ ./run-tests.sh --name "An existing configuration without a usable model completes the setup wizard"
      Scenario: An existing configuration without a usable model completes the setup wizard
       ✔  Given a configured provider with catalog models "model-small", "model-middle", and "model-large"
       ✔  When I run `watn explain` with this single argument and let the setup flow start:
       ✔  Then the setup flow should start
       ✔  When I configure the provider and models through the wizard
       ✔  And I complete the optional shell pages without integrations
       ✔  Then watn should exit successfully
       ✔  And watn should report that setup is complete and the command must be rerun
       ✔  And no explanation card should open
       ✔  And no provider request should have been made
    [Summary]
    1 feature
    1 scenario (1 passed)
    9 steps (9 passed)
    exit=0
    # harness: `preserve_explain_config` keeps the catalog fixture's provider
    # model-less while rewriting the mock endpoint; setup_wizard_steps accepts the
    # preselected provider on the provider-choice entry; wizard-save branch prints
    # the rerun hint and exits 0.
    # shared-step regressions:
    $ ./run-tests.sh --e2e --name "Setup wizard guides provider and model configuration page by page"
    1 scenario (1 passed), 11 steps (11 passed), exit=0
    $ ./run-tests.sh --e2e --name "Provider setup configures an OpenAI provider with an environment credential"
    1 scenario (1 passed), 12 steps (12 passed), exit=0
    ```
- [x] REFACTOR: re-run; zero.
  - Evidence:
    ```
    $ ./run-tests.sh --name "An existing configuration without a usable model completes the setup wizard"
    [Summary]
    1 feature
    1 scenario (1 passed)
    9 steps (9 passed)
    exit=0
    ```
- [x] COMMIT: `feat(explain-command): An existing configuration without a usable model completes the setup wizard` -> hash: `9edccba`

## S15: Interrupting the explanation request opens no card (@givn.added)

- [x] RED: remove `@wip`; targeted run must fail.
  - Evidence:
    ```
    $ ./run-tests.sh --name "Interrupting the explanation request opens no card"
      Scenario: Interrupting the explanation request opens no card
       ✘  When I run `watn explain` with this single argument and interrupt the request:
          Matched: tests/steps/explain_command_steps.rs:533:1
          Step panicked. Captured output: not implemented
    exit=1
    ```
- [x] GREEN: explicit interrupt check after the fetch and before the card; exit 130, no card; hanging-provider step. Targeted run zero.
  - Evidence:
    ```
    $ ./run-tests.sh --name "Interrupting the explanation request opens no card"
      Scenario: Interrupting the explanation request opens no card
       ✔  Given a provider accepts a connection and never sends a response
       ✔  When I run `watn explain` with this single argument and interrupt the request:
       ✔  Then no explanation card should open
       ✔  And the exit status should be 130
    [Summary]
    1 feature
    1 scenario (1 passed)
    4 steps (4 passed)
    exit=0
    # step waits for the in-flight spinner, sends Ctrl+C, captures exit 130;
    # run_explain_command re-checks the shared interrupt flag after the fetch and
    # before the card (the hanging provider exits through generate_candidate's
    # grace contract, so both paths guarantee no card).
    ```
- [x] REFACTOR: re-run; zero.
  - Evidence:
    ```
    $ ./run-tests.sh --name "Interrupting the explanation request opens no card"
    [Summary]
    1 feature
    1 scenario (1 passed)
    4 steps (4 passed)
    exit=0
    ```
- [x] COMMIT: `feat(explain-command): Interrupting the explanation request opens no card` -> hash: `540adc2`

## S16: A command supplied through standard input without a usable model reports setup guidance (@givn.added)

- [x] RED: remove `@wip`; targeted run must fail.
  - Evidence:
    ```
    $ ./run-tests.sh --name "A command supplied through standard input without a usable model reports setup guidance"
      Scenario: A command supplied through standard input without a usable model reports setup guidance
       ✘  When I run `watn explain` in a terminal with this command on standard input and let it report setup guidance:
          Matched: tests/steps/explain_command_steps.rs:525:1
          Step panicked. Captured output: not implemented
    exit=1
    ```
- [x] GREEN: guidance + exit 1 when setup is needed but stdin is not a terminal. Targeted run zero.
  - Evidence:
    ```
    $ ./run-tests.sh --name "A command supplied through standard input without a usable model reports setup guidance"
      Scenario: A command supplied through standard input without a usable model reports setup guidance
       ✔  Given no config file exists
       ✔  When I run `watn explain` in a terminal with this command on standard input and let it report setup guidance:
       ✔  Then watn should report that setup is required
       ✔  And no explanation card should open
       ✔  And no provider request should have been made
       ✔  And the exit status should be 1
    [Summary]
    1 feature
    1 scenario (1 passed)
    6 steps (6 passed)
    exit=0
    # step redirects the docstring through a temp file as stdin; the readiness
    # gate's non-terminal branch prints the existing setup guidance and exits 1.
    ```
- [x] REFACTOR: re-run; zero.
  - Evidence:
    ```
    $ ./run-tests.sh --name "A command supplied through standard input without a usable model reports setup guidance"
    [Summary]
    1 feature
    1 scenario (1 passed)
    6 steps (6 passed)
    exit=0
    ```
- [x] COMMIT: `feat(explain-command): A command supplied through standard input without a usable model reports setup guidance` -> hash: `746d936`

## S17: An explicit provider selection with a broken configuration reports its error (@givn.added)

- [x] RED: remove `@wip`; targeted run must fail.
  - Evidence:
    ```
    $ ./run-tests.sh --name "An explicit provider selection with a broken configuration reports its error"
      Scenario: An explicit provider selection with a broken configuration reports its error
       ✘  When I run `watn --provider missing explain` with this single argument in a terminal:
          Matched: tests/steps/explain_command_steps.rs:563:1
          Step panicked. Captured output: not implemented
    exit=1
    ```
- [x] GREEN: strict explicit-selection resolution (`watn --provider missing explain ...` exit 1; `watn --provider openrouter explain ...` exit 2). Targeted run zero.
  - Evidence:
    ```
    $ ./run-tests.sh --name "An explicit provider selection with a broken configuration reports its error"
      Scenario: An explicit provider selection with a broken configuration reports its error
       ✔  Given no config file exists
       ✔  When I run `watn --provider missing explain` with this single argument in a terminal:
       ✔  Then watn should report an unknown provider error
       ✔  And no explanation card should open
       ✔  And the exit status should be 1
       ✔  When I run `watn --provider openrouter explain` with this single argument in a terminal:
       ✔  Then watn should report a missing credential error
       ✔  And no explanation card should open
       ✔  And the exit status should be 2
    [Summary]
    1 feature
    1 scenario (1 passed)
    9 steps (9 passed)
    exit=0
    # top-level provider flag steps start the PTY directly with the flag before
    # the subcommand; strict resolution (S1) reports unknown provider (exit 1) and
    # missing credential (exit 2) with no card.
    ```
- [x] REFACTOR: re-run; zero.
  - Evidence:
    ```
    $ ./run-tests.sh --name "An explicit provider selection with a broken configuration reports its error"
    [Summary]
    1 feature
    1 scenario (1 passed)
    9 steps (9 passed)
    exit=0
    ```
- [x] COMMIT: `feat(explain-command): An explicit provider selection with a broken configuration reports its error` -> hash: `c2b0189`

## S18: An explicit provider without a resolvable model reports its error (@givn.added)

- [x] RED: remove `@wip`; targeted run must fail.
  - Evidence:
    ```
    $ ./run-tests.sh --name "An explicit provider without a resolvable model reports its error"
      Scenario: An explicit provider without a resolvable model reports its error
       ✘  Given a configured provider with no default model
          Matched: tests/steps/explain_command_steps.rs:487:1
          Step panicked. Captured output: not implemented
    exit=1
    ```
- [x] GREEN: explicit-provider model resolution error, exit 1, no card. Targeted run zero.
  - Evidence:
    ```
    $ ./run-tests.sh --name "An explicit provider without a resolvable model reports its error"
      Scenario: An explicit provider without a resolvable model reports its error
       ✔  Given a configured provider with no default model
       ✔  When I run `watn --provider custom explain` with this single argument in a terminal:
       ✔  Then watn should report a configuration error
       ✔  And no explanation card should open
       ✔  And the exit status should be 1
    [Summary]
    1 feature
    1 scenario (1 passed)
    5 steps (5 passed)
    exit=0
    # fixture writes [defaults] provider = "custom" plus [providers.custom] with
    # endpoint and credential but no default_model, no tiers, no defaults.model;
    # strict model resolution reports config error: no default model configured.
    ```
- [x] REFACTOR: re-run; zero.
  - Evidence:
    ```
    $ ./run-tests.sh --name "An explicit provider without a resolvable model reports its error"
    [Summary]
    1 feature
    1 scenario (1 passed)
    5 steps (5 passed)
    exit=0
    ```
- [ ] COMMIT: `feat(explain-command): An explicit provider without a resolvable model reports its error` -> hash: ``

## Completion

- [ ] All scenarios GREEN: `./run-tests.sh` exits 0 and `./run-tests.sh --e2e` exits 0.
- [ ] `givn lint --change explain-failure-guidance` exits 0 or 2.
- [ ] `cargo fmt --all -- --check` and `cargo clippy --locked --all-targets -- -D warnings` are clean.
- [ ] Every scenario has a recorded commit hash; no commit touches only the spec or a stub.
- [ ] Permanent-spec lockstep verified: the modified permanent scenarios pass under the new readiness gate.
