# Tasks: robust-review-response-recovery

Owning use case: `use-shell` (`givn/specs/use-shell/usecase.md`).
Capabilities: `interactive-shell-shortcut` (extended), `explain-command` (extended).
Delta specs: 9 added scenarios in
`givn/changes/robust-review-response-recovery/specs/use-shell/interactive-shell-shortcut.feature`,
5 added scenarios in
`givn/changes/robust-review-response-recovery/specs/use-shell/explain-command.feature`.
Design: `givn/changes/robust-review-response-recovery/design.md`.

## Domain constraints (apply to every scenario)

- Watn never invents a command or a Stage purpose; recovery returns only the
  provider's own delimited text.
- A review-shaped payload is never shown as the command.
- Nothing is released without explicit acceptance; the explained command is
  never generated, edited, accepted, or executed.
- The Purpose reason wording is fixed by design.md's mapping table; no reason
  may contain the anti-term `candidate`.
- Diagnostics are plain English; stdout stays the command-output channel.
- The capture is best-effort: a write failure warns and never changes the
  review outcome.
- Review and explanation requests raise `max_tokens` to 4096 only.
- Persona review lens: `terminal-developer--interactive` (not a Gherkin actor).
- Shared-code scenarios must keep the permanent specs green: after REFACTOR,
  run the full `./run-tests.sh` (not only the named scenario) and paste the
  summary; a shared parsing/card change that breaks a permanent scenario is a
  hard stop, not a review-time surprise.

## Setup task

- [x] Step skeletons: extend the existing per-capability files
  `tests/steps/interactive_shell_shortcut_steps.rs` (regular) and
  `tests/steps/explain_command_steps.rs` (regular). New step bodies start as
  `unimplemented!()`; the shared fixture changes (mismatched stage status
  `ready`, echo-covering explanation fixture) land in the scenario that needs
  them. No new e2e files: no new `@e2e` scenarios.
  - Evidence:
    ```
    $ cargo check --locked --all-targets --features test-support
        Checking watn v0.5.0 (/home/buster/projects/watn)
        Finished `dev` profile [unoptimized + debuginfo] target(s) in 19.45s
    # new unimplemented!() step bodies in tests/steps/interactive_shell_shortcut_steps.rs:
    review_response_with_control_characters, review_response_truncated_after_command,
    review_response_truncated_inside_command, review_response_with_unescaped_quotes,
    review_names_incomplete_response, review_names_invalid_json, review_names_stage_mismatch,
    review_hides_raw_payload, review_served_structured_response, review_run_verbose_direct,
    review_run_direct, review_close_without_accepting, review_reports_raw_response,
    review_invocation_shows_command, unusable_response_saved, review_names_state_file_path,
    unusable_response_state_dir_blocked, review_warns_state_file_write,
    unusable_response_state_file_prefilled, unusable_response_state_file_unchanged
    # new unimplemented!() step bodies in tests/steps/explain_command_steps.rs:
    configured_provider_line_broken_explanation, configured_provider_truncated_explanation,
    card_shows_stage_purpose, card_names_incomplete_response, card_names_stage_mismatch,
    run_explain_verbose, explain_reports_raw_response, explain_names_state_file_path
    ```
- [x] Runner and strict mode unchanged and proven: `./run-tests.sh`,
  cucumber-rs `.fail_on_skipped()` (`tests/features_runner.rs`), skipped -> exit 1.
  - Evidence:
    ```
    # temporary probe scenario with an undefined step, then removed:
    $ ./run-tests.sh --name "Strictness probe scenario"
    Feature: Strictness probe
      Scenario: Strictness probe scenario
       ✘  Given a step that is not defined anywhere in this suite
          Step failed:
          Defined: givn/changes/robust-review-response-recovery/specs/use-shell/strictness-probe.feature:4:5
          Step doesn't match any function
    [Summary]
    1 feature
    1 scenario (1 failed)
    1 step (1 failed)
    error: test failed, to rerun pass `--test features_runner`
    Caused by:
      process didn't exit successfully: `target/debug/deps/features_runner-...` (exit status: 1)
    ```
- [x] Baseline is green before starting.
  - Evidence:
    ```
    $ cat givn/commands.yaml
    verify:
      command: "./run-tests.sh"
      e2e_command: "./run-tests.sh --e2e"

    $ ./run-tests.sh
    [Summary]
    22 features
    245 scenarios (245 passed)
    1501 steps (1501 passed)

    $ ./run-tests.sh --e2e
    [Summary]
    25 features
    89 scenarios (89 passed)
    682 steps (682 passed)
    ```

## S1: A review response with literal line breaks and tabs in its values is read as a structured response

- [x] RED: remove `@wip`; write the given step (payload with real `\n` and
  `\t` bytes inside a purpose); run
  `./run-tests.sh --name "A review response with literal line breaks and tabs in its values is read as a structured response"`; must exit non-zero.
  - Evidence:
    ```
    Scenario: A review response with literal line breaks and tabs in its values is read as a structured response
     ✔  Given an installed Bash shortcut and a provider candidate for "show disk usage"
     ✘  And the provider returns a structured review response with literal line breaks and tabs inside its values
        Step failed:
        Matched: tests/steps/interactive_shell_shortcut_steps.rs:3937:1
        Step panicked. Captured output: not implemented
    [Summary]
    1 feature
    1 scenario (1 failed)
    2 steps (1 passed, 1 failed)
    exit=1
    ```
- [x] GREEN: `src/review/response.rs` — repair raw control characters inside
  string values in `parse_structured_review_response`, and normalize the
  response command before validation so a repaired response reaches `Ready`.
  Run targeted; zero.
  - Evidence:
    ```
    Scenario: A review response with literal line breaks and tabs in its values is read as a structured response
     ✔  Given an installed Bash shortcut and a provider candidate for "show disk usage"
     ✔  And the provider returns a structured review response with literal line breaks and tabs inside its values
     ✔  When I invoke Ctrl-W with the current input
     ✔  Then the review surface should show the command "df -h"
     ✔  And the review surface should show the stage purpose "Show local disk usage."
     ✔  And the review surface should not claim that purposes are loading
    [Summary]
    1 feature
    1 scenario (1 passed)
    6 steps (6 passed)
    # production files: src/review/response.rs (repair_json_control_characters,
    # parse_structured_review_response, validate normalization)
    ```
- [x] REFACTOR: clean up; targeted run still zero; then full `./run-tests.sh`
  green (shared parser).
  - Evidence:
    ```
    $ ./run-tests.sh --name "A review response with literal line breaks and tabs in its values is read as a structured response"
    1 scenario (1 passed), 6 steps (6 passed)
    $ ./run-tests.sh
    [Summary]
    23 features
    246 scenarios (246 passed)
    1507 steps (1507 passed)
    ```
- [x] COMMIT: `feat(review): A review response with literal line breaks and tabs in its values is read as a structured response`.
  - Evidence:
    ```
    0b24aaf
    ```

## S2: A review response cut off after a complete command stays reviewable

- [x] RED: remove `@wip`; write the given step (payload truncated after the
  command's closing quote); run the named scenario; must exit non-zero.
  - Evidence:
    ```
    Scenario: A review response cut off after a complete command stays reviewable
     ✔  Given an installed Bash shortcut and a provider candidate for "show disk usage"
     ✘  And the provider returns a structured review response cut off after the command "df -h"
        Step failed:
        Matched: tests/steps/interactive_shell_shortcut_steps.rs:3943:1
        Step panicked. Captured output: not implemented
    [Summary]
    1 feature
    1 scenario (1 failed)
    2 steps (1 passed, 1 failed)
    exit=1
    ```
- [x] GREEN: `src/review/response.rs` — payload-shape truncation detection,
  the delimiter-bounded `recover_command_value` (closing quote or next review
  field; never end-of-input), `ReviewResponseError::IncompleteResponse`, and
  `unavailable_reason` on the candidate; `src/review/card.rs` renders the
  reason. `src/provider/openai_compat.rs` / `src/provider/mod.rs` capture the
  last non-null SSE `finish_reason` (unit test). Raise review/explain
  `max_tokens` to 4096 (unit test on the request options). Run targeted; zero.
  - Evidence:
    ```
    Scenario: A review response cut off after a complete command stays reviewable
     ✔  Given an installed Bash shortcut and a provider candidate for "show disk usage"
     ✔  And the provider returns a structured review response cut off after the command "df -h"
     ✔  When I invoke Ctrl-W with the current input
     ✔  Then the review surface should show the command "df -h"
     ✔  And the review surface should show purpose-unavailable
     ✔  And the review surface should name that the provider response was incomplete
     ✔  And final acceptance should still be required
    [Summary]
    1 feature
    1 scenario (1 passed)
    7 steps (7 passed)
    $ cargo test --locked --lib --features test-support review::response
    test result: ok. 24 passed; 0 failed
    # production files: src/review/response.rs (locate_json_region,
    # looks_like_review_payload, unescape_json_string, recover_command_value,
    # unreadable_reason, IncompleteResponse, card_reason, unavailable_reason,
    # candidate_from_provider_response_with_finish), src/review/card.rs,
    # src/review/session.rs, src/provider/mod.rs, src/provider/openai_compat.rs,
    # src/main.rs (review/explain max_tokens 4096)
    ```
- [x] REFACTOR: clean up; targeted run still zero; full `./run-tests.sh` green.
  - Evidence:
    ```
    $ ./run-tests.sh --name "A review response cut off after a complete command stays reviewable"
    1 scenario (1 passed), 7 steps (7 passed)
    $ ./run-tests.sh
    [Summary]
    23 features
    247 scenarios (247 passed)
    1514 steps (1514 passed)
    ```
- [x] COMMIT: `feat(review): A review response cut off after a complete command stays reviewable`.
  - Evidence:
    ```
    9fb145f
    ```
## S3: A review response whose values contain unescaped quotation marks is still read

- [x] RED: remove `@wip`; write the given step (unescaped quotes in a stage
  purpose); run the named scenario; must exit non-zero.
  - Evidence:
    ```
    Scenario: A review response whose values contain unescaped quotation marks is still read
     ✔  Given an installed Bash shortcut and a provider candidate for "show disk usage"
     ✘  And the provider returns a structured review response whose values contain unescaped quotation marks
        Step failed:
        Matched: tests/steps/interactive_shell_shortcut_steps.rs:3957:1
        Step panicked. Captured output: not implemented
    [Summary]
    1 feature
    1 scenario (1 failed)
    2 steps (1 passed, 1 failed)
    exit=1
    ```
- [x] GREEN: `src/review/response.rs` — `looks_like_review_payload`,
  recovery of the command from the malformed payload, and the
  not-valid-JSON Purpose reason. Run targeted; zero.
  - Evidence:
    ```
    Scenario: A review response whose values contain unescaped quotation marks is still read
     ✔  Given an installed Bash shortcut and a provider candidate for "show disk usage"
     ✔  And the provider returns a structured review response whose values contain unescaped quotation marks
     ✔  When I invoke Ctrl-W with the current input
     ✔  Then the review surface should show the command "df -h"
     ✔  And the review surface should not show the raw provider payload
     ✔  And the review surface should show purpose-unavailable
     ✔  And the review surface should name that the provider response was not valid JSON
    [Summary]
    1 feature
    1 scenario (1 passed)
    7 steps (7 passed)
    # production files: none beyond S2 (recovery and reason already landed);
    # step assertions for the raw-payload guard and the reason wording
    ```
- [x] REFACTOR: clean up; targeted run still zero; full `./run-tests.sh` green.
  - Evidence:
    ```
    $ ./run-tests.sh --name "A review response whose values contain unescaped quotation marks is still read"
    1 scenario (1 passed), 7 steps (7 passed)
    $ ./run-tests.sh
    [Summary]
    23 features
    248 scenarios (248 passed)
    1521 steps (1521 passed)
    ```
- [x] COMMIT: `feat(review): A review response whose values contain unescaped quotation marks is still read`.
  - Evidence:
    ```
    86137de
    ```

## S4: A review response cut off inside the command releases nothing

- [x] RED: remove `@wip`; write the given step (unterminated command value);
  run the named scenario; must exit non-zero.
  - Evidence:
    ```
    Scenario: A review response cut off inside the command releases nothing
     ✔  Given an installed Bash shortcut and a provider candidate for "show disk usage"
     ✘  And the provider returns a structured review response cut off inside the command
        Step failed:
        Matched: tests/steps/interactive_shell_shortcut_steps.rs:3951:1
        Step panicked. Captured output: not implemented
    [Summary]
    1 feature
    1 scenario (1 failed)
    2 steps (1 passed, 1 failed)
    exit=1
    ```
- [x] GREEN: `src/review/response.rs` — the scanner refuses an unterminated
  value and a review-shaped payload never falls through to command text. Run
  targeted; zero.
  - Evidence:
    ```
    Scenario: A review response cut off inside the command releases nothing
     ✔  Given an installed Bash shortcut and a provider candidate for "show disk usage"
     ✔  And the provider returns a structured review response cut off inside the command
     ✔  When I invoke Ctrl-W with the current input
     ✔  Then the original Bash command line should remain unchanged
     ✔  And no candidate should be released to the shell
    [Summary]
    1 feature
    1 scenario (1 passed)
    5 steps (5 passed)
    # production files: none beyond S2 (the scanner already refuses an
    # unterminated value; unit test payload_cut_off_inside_the_command_recovers_nothing)
    ```
- [x] REFACTOR: clean up; targeted run still zero; full `./run-tests.sh` green.
  - Evidence:
    ```
    $ ./run-tests.sh --name "A review response cut off inside the command releases nothing"
    1 scenario (1 passed), 5 steps (5 passed)
    $ ./run-tests.sh
    [Summary]
    23 features
    249 scenarios (249 passed)
    1526 steps (1526 passed)
    ```
- [x] COMMIT: `feat(review): A review response cut off inside the command releases nothing`.
  - Evidence:
    ```
    3d9ab4f
    ```

## S5: The review surface names why stage purposes are unavailable

- [x] RED: remove `@wip`; change the shared mismatched-stage fixture to
  `"purpose_status":"ready"` (keeps the permanent scenario green); write the
  reason assertion step; run the named scenario; must exit non-zero.
  - Evidence:
    ```
    # before the fixture change the card names the invalid-JSON reason:
    Scenario: The review surface names why stage purposes are unavailable
     ✔  Given an installed Bash shortcut and a provider candidate for "show disk usage"
     ✔  And the provider returns a review response with mismatched stage text
     ✔  When I invoke Ctrl-W with the current input
     ✔  Then the review surface should show purpose-unavailable
     ✘  And the review surface should name that the response stages did not match the command
        card showed: purpose-unavailable · the provider response was not valid JSON
    [Summary]
    1 feature
    1 scenario (1 failed)
    5 steps (4 passed, 1 failed)
    exit=1
    ```
- [x] GREEN: `src/review/response.rs` — carry the validation error into
  `unavailable_reason` in the fallback path and add `card_reason()` per the
  design mapping; `src/review/card.rs` renders `purpose-unavailable · <reason>`.
  Run targeted; zero.
  - Evidence:
    ```
    Scenario: The review surface names why stage purposes are unavailable
     ✔  Given an installed Bash shortcut and a provider candidate for "show disk usage"
     ✔  And the provider returns a review response with mismatched stage text
     ✔  When I invoke Ctrl-W with the current input
     ✔  Then the review surface should show purpose-unavailable
     ✔  And the review surface should name that the response stages did not match the command
    [Summary]
    1 feature
    1 scenario (1 passed)
    5 steps (5 passed)
    # production files: none beyond S2 (validation_error and card_reason landed
    # there); fixture status changed to "ready" so the stage-mismatch path is reached
    ```
- [x] REFACTOR: clean up; targeted run still zero; full `./run-tests.sh` green
  (permanent mismatch and invalid-response scenarios).
  - Evidence:
    ```
    $ ./run-tests.sh --name "The review surface names why stage purposes are unavailable"
    1 scenario (1 passed), 5 steps (5 passed)
    $ ./run-tests.sh
    [Summary]
    23 features
    250 scenarios (250 passed)
    1531 steps (1531 passed)
    ```
- [x] COMMIT: `feat(review): The review surface names why stage purposes are unavailable`.
  - Evidence:
    ```
    fe224d4
    ```

## S6: Verbose review prints the raw provider response after the surface closes

- [x] RED: remove `@wip`; write the PTY steps (`a configured provider that
  serves this structured review response:`, `I run watn -v for ... in an
  eligible terminal`, `I cancel the review surface` direct variant, transcript
  assertions); run the named scenario; must exit non-zero.
  - Evidence:
    ```
    Scenario: Verbose review prints the raw provider response after the surface closes
     ✘  Given a configured provider that serves this structured review response:
        Step failed:
        Matched: tests/steps/interactive_shell_shortcut_steps.rs:3992:1
        Step panicked. Captured output: not implemented
    [Summary]
    1 feature
    1 scenario (1 failed)
    1 step (1 failed)
    exit=1
    ```
- [x] GREEN: `src/output/render.rs` — labelled raw-response printer on stderr;
  `src/main.rs` — print after `panel.finish()` for the review path and thread
  the accepted response's `finish_reason` into the initial parse. Run
  targeted; zero.
  - Evidence:
    ```
    Scenario: Verbose review prints the raw provider response after the surface closes
     ✔  Given a configured provider that serves this structured review response:
     ✔  When I run `watn -v` for "show disk usage" in an eligible terminal
     ✔  And I accept the candidate in the review surface
     ✔  Then the review invocation should report the raw provider response
     ✔  And normal command output should contain only "df -h"
    [Summary]
    1 feature
    1 scenario (1 passed)
    5 steps (5 passed)
    # production files: src/output/render.rs (print_raw_response), src/main.rs
    # (verbose print after panel.finish(); initial parse already threaded
    # finish_reason in S2), step helper start_direct_review_run
    ```
- [x] REFACTOR: clean up; targeted run still zero; full `./run-tests.sh` green.
  - Evidence:
    ```
    $ ./run-tests.sh --name "Verbose review prints the raw provider response after the surface closes"
    1 scenario (1 passed), 5 steps (5 passed)
    $ ./run-tests.sh
    [Summary]
    23 features
    251 scenarios (251 passed)
    1536 steps (1536 passed)
    ```
- [x] COMMIT: `feat(review): Verbose review prints the raw provider response after the surface closes`.
  - Evidence:
    ```
    c5bd4ca
    ```

## S7: An unusable provider response is saved for bug reports

- [x] RED: remove `@wip`; write the PTY steps (state-file read, path naming);
  run the named scenario; must exit non-zero.
  - Evidence:
    ```
    Scenario: An unusable provider response is saved for bug reports
     ✘  Given a configured provider that serves this review response:
        Step doesn't match any function
    [Summary]
    1 feature
    1 scenario (1 failed)
    1 step (1 failed)
    exit=1
    # after adding the given, the direct close step matched the e2e widget
    # cancel step and failed waiting for HIST<<, confirming RED for the
    # direct-path diagnostics
    ```
- [x] GREEN: `src/config/mod.rs` — `unusable_response_path()` from
  `$XDG_STATE_HOME` with `~/.local/state` fallback; `src/review/diagnostics.rs`
  (new) — `capture_unusable_response` writes the raw payload with mode `0600`;
  `src/main.rs` — capture every unusable review response and name the saved
  path on stderr after the surface closes. Run targeted; zero.
  - Evidence:
    ```
    Scenario: An unusable provider response is saved for bug reports
     ✔  Given a configured provider that serves this review response:
     ✔  When I run `watn` for "show disk usage" in an eligible terminal
     ✔  And I close the review surface without accepting
     ✔  Then the raw provider response should be saved to the unusable-response state file
     ✔  And the review invocation should name the unusable-response state file path
    [Summary]
    1 feature
    1 scenario (1 passed)
    5 steps (5 passed)
    $ cargo test --locked --lib --features test-support review::diagnostics
    test result: ok. 2 passed; 0 failed
    # production files: src/config/mod.rs (xdg_state_dir, unusable_response_path),
    # src/review/diagnostics.rs (capture_unusable_response), src/review/mod.rs,
    # src/main.rs (capture on unusable initial and regenerated candidates,
    # print_capture_diagnostics on accept, cancel, and disable)
    ```
- [x] REFACTOR: clean up; targeted run still zero; full `./run-tests.sh` green.
  - Evidence:
    ```
    $ ./run-tests.sh --name "An unusable provider response is saved for bug reports"
    1 scenario (1 passed), 5 steps (5 passed)
    $ ./run-tests.sh
    [Summary]
    23 features
    252 scenarios (252 passed)
    1541 steps (1541 passed)
    ```
- [x] COMMIT: `feat(review): An unusable provider response is saved for bug reports`.
  - Evidence:
    ```
    3c28749
    ```

## S8: An unwritable unusable-response state file does not break the review

- [x] RED: remove `@wip`; write the unwritable-directory given step and the
  warning assertion; run the named scenario; must exit non-zero.
  - Evidence:
    ```
    Scenario: An unwritable unusable-response state file does not break the review
     ✔  Given a configured provider that serves this review response:
     ✘  And the unusable-response state directory cannot be created
        Step failed:
        Matched: tests/steps/interactive_shell_shortcut_steps.rs:4128:1
        Step panicked. Captured output: not implemented
    [Summary]
    1 feature
    1 scenario (1 failed)
    2 steps (1 passed, 1 failed)
    exit=1
    ```
- [x] GREEN: `src/review/diagnostics.rs` / `src/main.rs` — warn on capture
  failure without changing the review outcome or exit status. Run targeted;
  zero.
  - Evidence:
    ```
    Scenario: An unwritable unusable-response state file does not break the review
     ✔  Given a configured provider that serves this review response:
     ✔  And the unusable-response state directory cannot be created
     ✔  When I run `watn` for "show disk usage" in an eligible terminal
     ✔  And I close the review surface without accepting
     ✔  Then the review invocation should show the command "df -h"
     ✔  And the review invocation should warn that the unusable-response state file could not be written
    [Summary]
    1 feature
    1 scenario (1 passed)
    6 steps (6 passed)
    # production files: src/main.rs (print_capture_diagnostics on cancel and
    # disable, warning wording); step helper creates the blocker after the
    # isolated config exists
    ```
- [x] REFACTOR: clean up; targeted run still zero; full `./run-tests.sh` green.
  - Evidence:
    ```
    $ ./run-tests.sh --name "An unwritable unusable-response state file does not break the review"
    1 scenario (1 passed), 6 steps (6 passed)
    $ ./run-tests.sh
    [Summary]
    23 features
    253 scenarios (253 passed)
    1547 steps (1547 passed)
    ```
- [x] COMMIT: `feat(review): An unwritable unusable-response state file does not break the review`.
  - Evidence:
    ```
    a0e3466
    ```

## S9: A usable review response does not overwrite the captured unusable response

- [x] RED: remove `@wip`; write the sentinel prefill and the unchanged-file
  assertion; run the named scenario; must exit non-zero.
  - Evidence:
    ```
    Scenario: A usable review response does not overwrite the captured unusable response
     ✘  Given an unusable-response state file that already holds a previous response
        Step failed:
        Matched: tests/steps/interactive_shell_shortcut_steps.rs:4160:1
        Step panicked. Captured output: not implemented
    [Summary]
    1 feature
    1 scenario (1 failed)
    1 step (1 failed)
    exit=1
    ```
- [x] GREEN: `src/main.rs` — capture only when the response is unusable. Run
  targeted; zero.
  - Evidence:
    ```
    Scenario: A usable review response does not overwrite the captured unusable response
     ✔  Given an unusable-response state file that already holds a previous response
     ✔  And a configured provider that serves this review response:
     ✔  When I run `watn` for "show disk usage" in an eligible terminal
     ✔  And I close the review surface without accepting
     ✔  Then the unusable-response state file should still hold the previous response
    [Summary]
    1 feature
    1 scenario (1 passed)
    5 steps (5 passed)
    # production files: none beyond S7 (capture already guards on
    # PurposeStatus::Unavailable); step helper prefills the state file
    ```
- [x] REFACTOR: clean up; targeted run still zero; full `./run-tests.sh` green.
  - Evidence:
    ```
    $ ./run-tests.sh --name "A usable review response does not overwrite the captured unusable response"
    1 scenario (1 passed), 5 steps (5 passed)
    $ ./run-tests.sh
    [Summary]
    23 features
    254 scenarios (254 passed)
    1552 steps (1552 passed)
    ```
- [x] COMMIT: `feat(review): A usable review response does not overwrite the captured unusable response`.
  - Evidence:
    ```
    57274fa
    ```

## S10: An explanation response with literal line breaks in its values is read as a structured response

- [ ] RED: remove `@wip`; write the JSON-serialized SSE fixture given step and
  the stage-purpose assertion; run the named scenario; must exit non-zero.
  - Evidence:
    ```
    <paste>
    ```
- [ ] GREEN: reuse the S1 repair through `apply_explanation_outcome`; the
  fixture must echo the developer's command exactly. Run targeted; zero.
  - Evidence:
    ```
    <paste>
    # production files: none expected beyond S1 (reuse); investigate if the list is empty for the wrong reason
    ```
- [ ] REFACTOR: clean up; targeted run still zero; full `./run-tests.sh` green.
  - Evidence:
    ```
    <paste targeted + full summary>
    ```
- [ ] COMMIT: `feat(explain): An explanation response with literal line breaks in its values is read as a structured response`.
  - Evidence:
    ```
    <commit hash>
    ```

## S11: An explanation response cut off after a complete command keeps the command reviewable

- [ ] RED: remove `@wip`; write the truncated explanation fixture and the
  incomplete-reason assertion; run the named scenario; must exit non-zero.
  - Evidence:
    ```
    <paste>
    ```
- [ ] GREEN: `src/review/session.rs` — pass the provider `finish_reason` into
  `apply_explanation_outcome`; `src/review/response.rs` — the truncated echo
  keeps the developer's command with the incomplete reason. Run targeted; zero.
  - Evidence:
    ```
    <paste>
    # production files: src/review/session.rs, src/review/response.rs
    ```
- [ ] REFACTOR: clean up; targeted run still zero; full `./run-tests.sh` green.
  - Evidence:
    ```
    <paste targeted + full summary>
    ```
- [ ] COMMIT: `feat(explain): An explanation response cut off after a complete command keeps the command reviewable`.
  - Evidence:
    ```
    <commit hash>
    ```

## S12: The explanation card names why stage purposes are unavailable

- [ ] RED: remove `@wip`; change the shared not-covering fixture so the
  response command echoes the developer's command (stages still mismatch); add
  the reason assertion; run the named scenario; must exit non-zero.
  - Evidence:
    ```
    <paste>
    ```
- [ ] GREEN: `src/review/card.rs` / `src/review/response.rs` — explain-domain
  card reason for the stage mismatch. Run targeted; zero.
  - Evidence:
    ```
    <paste>
    # production files: src/review/card.rs, src/review/response.rs
    ```
- [ ] REFACTOR: clean up; targeted run still zero; full `./run-tests.sh` green
  (permanent "does not cover" scenario).
  - Evidence:
    ```
    <paste targeted + full summary>
    ```
- [ ] COMMIT: `feat(explain): The explanation card names why stage purposes are unavailable`.
  - Evidence:
    ```
    <commit hash>
    ```

## S13: Verbose explain prints the raw provider response after the card closes

- [ ] RED: remove `@wip`; write the `watn explain -v` PTY step and the
  raw-response assertion; run the named scenario; must exit non-zero.
  - Evidence:
    ```
    <paste>
    ```
- [ ] GREEN: `src/main.rs` — `Explain::verbose` and the post-card raw-response
  printer; `src/review/session.rs` — return the `StreamingResponse` from
  `explain_command_candidate`. Run targeted; zero.
  - Evidence:
    ```
    <paste>
    # production files: src/main.rs, src/review/session.rs
    ```
- [ ] REFACTOR: clean up; targeted run still zero; full `./run-tests.sh` green.
  - Evidence:
    ```
    <paste targeted + full summary>
    ```
- [ ] COMMIT: `feat(explain): Verbose explain prints the raw provider response after the card closes`.
  - Evidence:
    ```
    <commit hash>
    ```

## S14: An unusable explanation response is saved for bug reports

- [ ] RED: remove `@wip`; write the state-file read and path-naming
  assertions; run the named scenario; must exit non-zero.
  - Evidence:
    ```
    <paste>
    ```
- [ ] GREEN: `src/main.rs` — capture the unusable explanation response and
  name the path in the existing stderr diagnostic. Run targeted; zero.
  - Evidence:
    ```
    <paste>
    # production files: src/main.rs
    ```
- [ ] REFACTOR: clean up; targeted run still zero; full `./run-tests.sh` green.
  - Evidence:
    ```
    <paste targeted + full summary>
    ```
- [ ] COMMIT: `feat(explain): An unusable explanation response is saved for bug reports`.
  - Evidence:
    ```
    <commit hash>
    ```

## Final verification

- [ ] Full regular suite green: `./run-tests.sh`.
  - Evidence:
    ```
    <paste summary>
    ```
- [ ] Full e2e suite green: `./run-tests.sh --e2e` (no new `@e2e` scenarios;
  confirms the unchanged e2e contract).
  - Evidence:
    ```
    <paste summary>
    ```
- [ ] `givn lint --change robust-review-response-recovery` exits 0 or 2 with
  only accepted advisory findings.
  - Evidence:
    ```
    <paste>
    ```
