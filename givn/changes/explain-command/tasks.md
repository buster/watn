# Tasks: explain-command

Owning use case: `use-shell` (`givn/specs/use-shell/usecase.md`).
Capability: `explain-command`.
Spec: `givn/changes/explain-command/specs/use-shell/explain-command.feature` (17 scenarios, 1 `@e2e`).
Design: `givn/changes/explain-command/design.md`.

## Domain constraints (apply to every scenario)

- Review never evaluates generated or edited text; explain never executes anything.
- No candidate is released on the explanation path: the command-output channel stays empty.
- The developer's command is authoritative; a model response never replaces it.
- Purpose loading is shown only for a structured response that supports it; otherwise `purpose-unavailable`.
- An unsupported command-flow portion remains visible and reviewable.
- Review-surface text is rendered through the controlling-terminal channel.
- Non-TTY and redirected requests retain the terminal-required behavior.
- Persona review lens: `terminal-developer--interactive` (not a Gherkin actor).

## Setup task

- [x] Create the two step-definition skeleton files from design.md, one per capability:
  - `tests/steps/explain_command_steps.rs` (regular), module `explain_command_steps`
  - `tests/steps/explain_command_e2e_steps.rs` (e2e), module `explain_command_e2e_steps`
  Register both in `tests/steps/mod.rs`. New step bodies use `unimplemented!()` until implemented.
- [x] Configure the runner and strict mode. Already in place and verified against design.md:
  - `verify.command`: `./run-tests.sh` (`givn/commands.yaml`)
  - `verify.e2e_command`: `./run-tests.sh --e2e` (`givn/commands.yaml`)
  - cucumber-rs strict mode: `.fail_on_skipped()` at `tests/features_runner.rs:210`; runner exits 1 when `stats.skipped > 0` (`tests/features_runner.rs:227`).
- [x] Proof-of-strictness: temporarily add a scratch scenario with one undefined step (no `@wip`), run it with `./run-tests.sh --name "<temp title>"`, confirm a NON-ZERO exit, then delete the scratch scenario.
  - Command + output:
    ```
    ./run-tests.sh --name "Scratch undefined step fails" -> exit 1
    Scenario: Scratch undefined step fails
     ✘  Given a step that is not defined anywhere in this project
        Step failed: Step doesn't match any function
    [Summary] 1 scenario (1 failed), 1 step (1 failed)
    ```
- [x] Confirm `./run-tests.sh` and `./run-tests.sh --e2e` are distinct strings and both currently exit 0 on the unmodified suite.

## S1: Enter closes the explanation card without releasing the command

- [x] RED: remove `@wip` from this scenario only; run `./run-tests.sh --name "Enter closes the explanation card without releasing the command"`; must exit non-zero.
  - Evidence:
    ```
    exit 1: ✘ When I run `watn explain` with this single argument in a terminal, then press Enter:
    Step panicked. Captured output: not implemented
    ```
- [x] GREEN: implement the minimum production code (`src/main.rs` explain dispatch, `src/review/panel.rs` terminal predicate, `src/review/mod.rs` exports); replace stubs with real assertions. Production files: `src/main.rs`, `src/review/mod.rs`, `src/review/panel.rs`, `tests/steps/explain_command_steps.rs`, `tests/steps/mod.rs`. Run the single-scenario command; must exit zero.
  - Evidence:
    ```
    exit 0: 1 scenario (1 passed), 5 steps (5 passed)
    ```
- [x] REFACTOR: clean up without behaviour change; re-run the single-scenario command; still zero.
  - Evidence:
    ```
    exit 0: 1 scenario (1 passed), 5 steps (5 passed)
    ```
- [x] COMMIT: `feat(explain-command): Enter closes the explanation card without releasing the command` → hash: `2ac3c1e`

## S2: Shell metacharacters and embedded quoting reach the card unchanged

- [x] RED: remove `@wip`; run `./run-tests.sh --name "Shell metacharacters and embedded quoting reach the card unchanged"`; must exit non-zero.
  - Evidence:
    ```
    exit 1: ✘ When I run `watn explain` with this single argument:
    Step panicked. Captured output: not implemented
    ```
- [x] GREEN: production code for verbatim argument delivery and local flow derivation (`src/main.rs`, `src/review/response.rs`, `tests/steps/explain_command_steps.rs`). Single-scenario run must exit zero.
  - Evidence:
    ```
    exit 0: 1 scenario (1 passed); all 4 stage assertions and purpose-unavailable pass
    ```
- [x] REFACTOR: re-run; still zero.
  - Evidence:
    ```
    exit 0: 1 scenario (1 passed)
    ```
- [x] COMMIT: `feat(explain-command): Shell metacharacters and embedded quoting reach the card unchanged` → hash: `0319495`

## S3: A command beginning with a dash passes after the option terminator

- [x] RED: remove `@wip`; run `./run-tests.sh --name "A command beginning with a dash passes after the option terminator"`; must exit non-zero.
  - Evidence:
    ```
    exit 1: ✘ When I run `watn explain --` with this single argument:
    Step panicked. Captured output: not implemented
    ```
- [x] GREEN: clap option terminator handling for the explain positional (`src/main.rs`). Single-scenario run must exit zero.
  - Evidence:
    ```
    exit 0: 1 scenario (1 passed), 3 steps (3 passed)
    ```
- [x] REFACTOR: re-run; still zero.
  - Evidence:
    ```
    exit 0: 1 scenario (1 passed), 3 steps (3 passed)
    ```
- [x] COMMIT: `feat(explain-command): A command beginning with a dash passes after the option terminator` → hash: `b4c3ec4`

## S4: A command read from standard input keeps its quoting and line breaks

- [x] RED: remove `@wip`; run `./run-tests.sh --name "A command read from standard input keeps its quoting and line breaks"`; must exit non-zero.
  - Evidence:
    ```
    exit 1: ✘ When I run `watn explain -` in a terminal with this command on standard input:
    Step panicked. Captured output: not implemented
    ```
- [x] GREEN: stdin marker `-` handling plus the separate-stage boundary assertion; production code in `src/main.rs`, `tests/steps/explain_command_steps.rs`. Single-scenario run must exit zero.
  - Evidence:
    ```
    exit 0: 1 scenario (1 passed), 5 steps (5 passed)
    ```
  - Production fix: the first GREEN attempt failed with `explain unavailable: review panel requires a controlling terminal` because `ControllingTerminal::open` checked terminal stdin. Per design.md:354 it now validates the controlling-terminal channel (`explanation_terminal_is_usable()`) in `src/review/panel.rs`, so `watn explain - < file` opens the card.
- [x] REFACTOR: re-run; still zero.
  - Evidence:
    ```
    exit 0: 1 scenario (1 passed), 5 steps (5 passed)
    ```
- [x] COMMIT: `feat(explain-command): A command read from standard input keeps its quoting and line breaks` → hash: `abc1a8c`

## S5: A piped command without the marker is explained

- [x] RED: remove `@wip`; run `./run-tests.sh --name "A piped command without the marker is explained"`; must exit non-zero.
  - Evidence:
    ```
    exit 1: ✘ When I run `watn explain` in a terminal with the command "ls | wc -l" piped on standard input
    Step panicked. Captured output: not implemented
    ```
- [x] GREEN: piped-stdin detection when no positional argument is present; production code in `src/main.rs`, `tests/steps/explain_command_steps.rs`. Single-scenario run must exit zero.
  - Evidence:
    ```
    exit 0: 1 scenario (1 passed), 5 steps (5 passed)
    ```
- [x] REFACTOR: re-run; still zero.
  - Evidence:
    ```
    exit 0: 1 scenario (1 passed), 5 steps (5 passed)
    ```
- [x] COMMIT: `feat(explain-command): A piped command without the marker is explained` → hash: `af0c529`

## S6: A positional command takes precedence over standard input

- [x] RED: remove `@wip`; run `./run-tests.sh --name "A positional command takes precedence over standard input"`; must exit non-zero.
  - Evidence:
    ```
    exit 1: ✘ When I run watn explain with the argument "echo positional" in a terminal with the command "echo standard input" on standard input
    Step panicked. Captured output: not implemented
    ```
- [x] GREEN: positional-wins precedence and no silent pipe consumption; production code in `src/main.rs`, `tests/steps/explain_command_steps.rs`. Single-scenario run must exit zero.
  - Evidence:
    ```
    exit 0: 1 scenario (1 passed), 4 steps (4 passed)
    ```
- [x] REFACTOR: re-run; still zero.
  - Evidence:
    ```
    exit 0: 1 scenario (1 passed), 4 steps (4 passed)
    ```
- [ ] COMMIT: `feat(explain-command): A positional command takes precedence over standard input` → hash: ``

## S7: Empty input is rejected without opening a card

- [ ] RED: remove `@wip`; run `./run-tests.sh --name "Empty input is rejected without opening a card"`; must exit non-zero.
  - Evidence:
    ```
    ```
- [ ] GREEN: empty-argument and empty-stdin rejection with exit 2 and no card; production code in `src/main.rs`, `tests/steps/explain_command_steps.rs`. Single-scenario run must exit zero.
  - Evidence:
    ```
    ```
- [ ] REFACTOR: re-run; still zero.
  - Evidence:
    ```
    ```
- [ ] COMMIT: `feat(explain-command): Empty input is rejected without opening a card` → hash: ``

## S8: A second positional argument is refused

- [ ] RED: remove `@wip`; run `./run-tests.sh --name "A second positional argument is refused"`; must exit non-zero.
  - Evidence:
    ```
    ```
- [ ] GREEN: single-argument clap contract (no lossy join); production code in `src/main.rs`, `tests/steps/explain_command_steps.rs`. Single-scenario run must exit zero.
  - Evidence:
    ```
    ```
- [ ] REFACTOR: re-run; still zero.
  - Evidence:
    ```
    ```
- [ ] COMMIT: `feat(explain-command): A second positional argument is refused` → hash: ``

## S9: A configured provider without a usable credential is not contacted

- [ ] RED: remove `@wip`; run `./run-tests.sh --name "A configured provider without a usable credential is not contacted"`; must exit non-zero.
  - Evidence:
    ```
    ```
- [ ] GREEN: tolerant readiness check, zero provider requests, `purpose-unavailable`; production code in `src/main.rs`, `src/review/session.rs`, `tests/steps/explain_command_steps.rs`. Single-scenario run must exit zero.
  - Evidence:
    ```
    ```
- [ ] REFACTOR: re-run; still zero.
  - Evidence:
    ```
    ```
- [ ] COMMIT: `feat(explain-command): A configured provider without a usable credential is not contacted` → hash: ``

## S10: A failed explanation keeps the command reviewable

- [ ] RED: remove `@wip`; run `./run-tests.sh --name "A failed explanation keeps the command reviewable"`; must exit non-zero.
  - Evidence:
    ```
    ```
- [ ] GREEN: provider-failure degradation to `purpose-unavailable` while keeping the command; production code in `src/main.rs`, `src/review/session.rs`, `tests/steps/explain_command_steps.rs`. Single-scenario run must exit zero.
  - Evidence:
    ```
    ```
- [ ] REFACTOR: re-run; still zero.
  - Evidence:
    ```
    ```
- [ ] COMMIT: `feat(explain-command): A failed explanation keeps the command reviewable` → hash: ``

## S11: An explanation that does not cover the command is not trusted

- [ ] RED: remove `@wip`; run `./run-tests.sh --name "An explanation that does not cover the command is not trusted"`; must exit non-zero.
  - Evidence:
    ```
    ```
- [ ] GREEN: strict `apply_explanation` trust rules (exact echo or verbatim covering split); production code in `src/review/response.rs`, `src/main.rs`, `tests/steps/explain_command_steps.rs`. Single-scenario run must exit zero.
  - Evidence:
    ```
    ```
- [ ] REFACTOR: re-run; still zero.
  - Evidence:
    ```
    ```
- [ ] COMMIT: `feat(explain-command): An explanation that does not cover the command is not trusted` → hash: ``

## S12: Explain never executes the command

- [ ] RED: remove `@wip`; run `./run-tests.sh --name "Explain never executes the command"`; must exit non-zero.
  - Evidence:
    ```
    ```
- [ ] GREEN: `-x` rejection on both invocation positions with exit 2 and no execution; production code in `src/main.rs`, `tests/steps/explain_command_steps.rs`. Single-scenario run must exit zero.
  - Evidence:
    ```
    ```
- [ ] REFACTOR: re-run; still zero.
  - Evidence:
    ```
    ```
- [ ] COMMIT: `feat(explain-command): Explain never executes the command` → hash: ``

## S13: The explanation card opens even when the review surface is disabled

- [ ] RED: remove `@wip`; run `./run-tests.sh --name "The explanation card opens even when the review surface is disabled"`; must exit non-zero.
  - Evidence:
    ```
    ```
- [ ] GREEN: explain bypasses the persisted review-surface preference; production code in `src/main.rs`, `tests/steps/explain_command_steps.rs`. Single-scenario run must exit zero.
  - Evidence:
    ```
    ```
- [ ] REFACTOR: re-run; still zero.
  - Evidence:
    ```
    ```
- [ ] COMMIT: `feat(explain-command): The explanation card opens even when the review surface is disabled` → hash: ``

## S14: The review-panel switches are inert for explain

- [ ] RED: remove `@wip`; run `./run-tests.sh --name "The review-panel switches are inert for explain"`; must exit non-zero.
  - Evidence:
    ```
    ```
- [ ] GREEN: switches accepted and not persisted or honored on the explain path; production code in `src/main.rs`, `tests/steps/explain_command_steps.rs`. Single-scenario run must exit zero.
  - Evidence:
    ```
    ```
- [ ] REFACTOR: re-run; still zero.
  - Evidence:
    ```
    ```
- [ ] COMMIT: `feat(explain-command): The review-panel switches are inert for explain` → hash: ``

## S15: A terminal is required for the explanation card

- [ ] RED: remove `@wip`; run `./run-tests.sh --name "A terminal is required for the explanation card"`; must exit non-zero.
  - Evidence:
    ```
    ```
- [ ] GREEN: `explanation_terminal_is_usable` check with a non-zero exit and terminal-required message; production code in `src/main.rs`, `src/review/panel.rs`, `tests/steps/explain_command_steps.rs`. Single-scenario run must exit zero.
  - Evidence:
    ```
    ```
- [ ] REFACTOR: re-run; still zero.
  - Evidence:
    ```
    ```
- [ ] COMMIT: `feat(explain-command): A terminal is required for the explanation card` → hash: ``

## S16: A malformed configuration file is reported

- [ ] RED: remove `@wip`; run `./run-tests.sh --name "A malformed configuration file is reported"`; must exit non-zero.
  - Evidence:
    ```
    ```
- [ ] GREEN: config parse error propagation with exit 1 and no card; production code in `src/main.rs`, `tests/steps/explain_command_steps.rs`. Single-scenario run must exit zero.
  - Evidence:
    ```
    ```
- [ ] REFACTOR: re-run; still zero.
  - Evidence:
    ```
    ```
- [ ] COMMIT: `feat(explain-command): A malformed configuration file is reported` → hash: ``

## E2E setup task

- [ ] Local environment: no containers or databases; the only external service is the LLM provider endpoint, replaced by the in-process `httpmock` twin (`MockServerWrap` in `tests/features_runner.rs`). Confirm `cargo run --release -- explain 'ls'` fails cleanly without a terminal and that the PTY harness starts.
- [ ] Create the e2e step skeleton in `tests/steps/explain_command_e2e_steps.rs` (already registered in the setup task); `unimplemented!()` bodies until implemented.
- [ ] Strict mode for the e2e runner: same `.fail_on_skipped()` configuration; proof-of-strictness already recorded in the setup task.
- [ ] Prove `verify.e2e_command` is a strict subset of `verify.command`: run both on the current suite and record the scenario counts (e2e count strictly smaller).
  - `./run-tests.sh` count:
    ```
    ```
  - `./run-tests.sh --e2e` count:
    ```
    ```
- [ ] Confirm the e2e runner reaches the PTY harness and the mock twin with a trivial smoke check.

## E1 (@e2e): Developer explains an existing command in the review card

- [ ] RED: remove `@wip` from this scenario only; run `./run-tests.sh --e2e --name "Developer explains an existing command in the review card"`; must exit non-zero.
  - Evidence:
    ```
    ```
- [ ] GREEN: e2e steps drive the real `watn` binary in a `portable-pty` session, assert the stages, model-written purposes, and terminal baseline through the PTY transcript, press Escape, and commit the ANSI-stripped transcript to `givn/changes/explain-command/evidence/visual/developer-explains-an-existing-command-in-the-review-card/transcript.txt`. Production files: `tests/steps/explain_command_e2e_steps.rs`. Run the single-scenario e2e command; must exit zero.
  - Evidence:
    ```
    ```
- [ ] REFACTOR: clean up e2e code without behaviour change; re-run; still zero.
  - Evidence:
    ```
    ```
- [ ] COMMIT: `test(e2e): Developer explains an existing command in the review card` → hash: ``

## Completion

- [ ] All scenarios GREEN: `./run-tests.sh` exits 0 and `./run-tests.sh --e2e` exits 0.
- [ ] `givn lint --change explain-command` exits 0 or 2.
- [ ] Every scenario has a recorded commit hash; no commit touches only the spec or a stub.
