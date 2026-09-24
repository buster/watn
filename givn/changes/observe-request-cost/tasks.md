# Tasks: observe-request-cost

## Domain constraints (apply to every scenario task)

From `givn/specs/observe-request-cost/usecase.md` and its capability
`visible-request-amount`:

- The Amount is the billed cost of the request that produced the surface: the
  usage the provider reported, paired with the recorded per-million price of
  the model the provider reported, in cents.
- Shown only when the request reported usage **and** a recorded price matches
  the reported model. Otherwise the Model label stands alone, silently.
- The billed model's Amount is shown under the label naming the model the
  request was sent with.
- Never estimated, never substituted, never zero for an unaccounted request; a
  reported zero is shown as a genuine zero.
- The Amount is transient, appears at the Model label in both views while the
  terminal is wide enough, never becomes command output, and gates no review
  decision.
- Persona lens: `terminal-developer--interactive` (cost stance, 2026-09-24) —
  the money must reach the developer inside the surface that asks for the next
  decision.

Design references: `design.md` (Amount data model, header layout, plumbing
table, cents form), `design-review.md` (the five decisions), `arc42.md`.

## Setup task

- [x] **T1 — Step skeletons and proof of strictness.**
  - Create `tests/steps/observe_request_cost_steps.rs` (in-process assertions
    and the new Amount/usage/price steps) and
    `tests/steps/observe_request_cost_e2e_steps.rs` (the two end-to-end
    drivers), and register both from `tests/steps/mod.rs`. One file per
    capability, separate from the existing review harness steps.
  - Strict mode is already configured: `.fail_on_skipped()` at
    `tests/features_runner.rs:210` plus the skipped-count exit at `:227`.
  - Proof of strictness: add one step body using `unimplemented!("TODO")`,
    `./run-tests.sh --name "<scenario using it>"`, confirm non-zero exit.
  - Evidence: `./run-tests.sh --name "A rejected candidate's replacement carries its own billed amount"` → non-zero exit (status 1):
    ```
    Feature: Billed amount of a request
      Scenario: A rejected candidate's replacement carries its own billed amount
       ✔  Given an installed Bash shortcut and a provider candidate "find . -type f"
       ✘  And a recorded price of 0.15 input and 0.60 output per million tokens for the configured model
          Step failed:
          Matched: tests/steps/observe_request_cost_steps.rs:11:1
          Step panicked. Captured output: not implemented: TODO: record the configured model's price for the surface
    [Summary]
    1 feature
    1 scenario (1 failed)
    2 steps (1 passed, 1 failed)
    error: test failed ... (exit status: 1)
    ```
    Step skeletons: `tests/steps/observe_request_cost_steps.rs`, registered from
    `tests/steps/mod.rs`; the e2e step file is created by the e2e setup task. command + output pasted below.
  - Evidence:

## Scenarios — in-process (non-@e2e), in feature-file order

### S1 — A rejected candidate's replacement carries its own billed amount

- [x] **RED** — remove `@wip` from this scenario only; unimplemented steps use
      `unimplemented!("TODO")`; `./run-tests.sh --name "A rejected candidate's replacement carries its own billed amount"`; non-zero exit required.
  - Evidence: non-zero exit (status 1), the recorded-price stub panicking:
    ```
     ✘  And a recorded price of 0.15 input and 0.60 output per million tokens for the configured model
        Step panicked. Captured output: not implemented: TODO: record the configured model's price for the surface
    [Summary] 1 scenario (1 failed); error: test failed ... (exit status: 1)
    ```
- [x] **GREEN** — implement: `src/amount.rs` (`BilledAmount`,
      `billed_amount`, `cents_text`), `ReviewContext.amount`,
      `header_right` reservation, `run_review_path` fill, and
      `ReviewPanelState::apply_regeneration` taking the new Amount.
      Production files created/modified: `src/amount.rs`, `src/lib.rs`,
      `src/review/panel.rs`, `src/review/card.rs`, `src/main.rs`.
      Same command → zero exit.
  - Evidence: `exit=0`;
    ```
     ✔  Then the review surface should show a billed amount of "0.575" cents with the model name
    [Summary] 1 feature, 1 scenario (1 passed), 8 steps (8 passed)
    ```
    `cargo test --lib --features test-support` → 104 passed, 0 failed.
- [x] **REFACTOR** — no behaviour change; same command → zero exit.
  - Evidence: `exit=0` after the `cents_text` refinement (a billed amount too
    small for four decimals keeps more decimals until it is visible);
    ```
     ✔  Then the review surface should show a billed amount of "0.575" cents with the model name
    [Summary] 1 feature, 1 scenario (1 passed), 8 steps (8 passed)
    ```
- [x] **COMMIT** — `feat(visible-request-amount): A rejected candidate's replacement carries its own billed amount`
  - Hash:

### S2 — A model with no recorded price leaves the model name without an amount

- **Already green — recorded regression guard (design-review question 4).**
  Verified together with S1; no RED evidence is fabricated.
- [ ] **VERIFY** — `./run-tests.sh --name "A model with no recorded price leaves the model name without an amount"` → zero exit, after S1's GREEN.
  - Evidence:
- [ ] **COMMIT** — covered by S1's commit; no separate commit is made for an
      already-green guard. Recorded here as a deliberate exception.
  - Hash: covered by S1

### S3 — A request the provider did not account for shows no amount

- **Already green — recorded regression guard (design-review question 4).**
- [ ] **VERIFY** — same command, zero exit, after S1's GREEN; also asserts the
      existing metadata line still reports a zero cost.
  - Evidence:
- [ ] **COMMIT** — covered by S1's commit. Same recorded exception.
  - Hash: covered by S1

### S4 — A reported zero usage shows a zero amount

- [ ] **RED** — remove `@wip`; `./run-tests.sh --name "A reported zero usage shows a zero amount"`; non-zero exit required.
  - Evidence:
- [ ] **GREEN** — the `usage_reported` distinction and the `0` form of
      `cents_text`. Production files: `src/amount.rs`, `src/main.rs`.
      Same command → zero exit.
  - Evidence:
- [ ] **REFACTOR** — same command → zero exit.
  - Evidence:
- [ ] **COMMIT** — `feat(visible-request-amount): A reported zero usage shows a zero amount`
  - Hash:

### S5 — The billed model's amount is shown under the requested model name

- [ ] **RED** — remove `@wip`; same-command pattern with this title; non-zero exit.
  - Evidence:
- [ ] **GREEN** — Amount keyed on the reported model, label on the requested
      model. Production files: `src/main.rs`, `src/amount.rs`.
  - Evidence:
- [ ] **REFACTOR** — same command → zero exit.
  - Evidence:
- [ ] **COMMIT** — `feat(visible-request-amount): The billed model's amount is shown under the requested model name`
  - Hash:

### S6 — A narrow terminal keeps the billed amount at the model name

- [ ] **RED** — remove `@wip`; same-command pattern with this title; non-zero exit.
  - Evidence:
- [ ] **GREEN** — the reservation rule in `header_right` and the
      40-column fallback. Production files: `src/review/card.rs`.
  - Evidence:
- [ ] **REFACTOR** — same command → zero exit.
  - Evidence:
- [ ] **COMMIT** — `feat(visible-request-amount): A narrow terminal keeps the billed amount at the model name`
  - Hash:

### S7 — The detailed view keeps the billed amount at the model name

- [ ] **RED** — remove `@wip`; same-command pattern with this title; non-zero exit.
  - Evidence:
- [ ] **GREEN** — the detailed label plus the same suffix rule. Production
      files: `src/review/card.rs`.
  - Evidence:
- [ ] **REFACTOR** — same command → zero exit.
  - Evidence:
- [ ] **COMMIT** — `feat(visible-request-amount): The detailed view keeps the billed amount at the model name`
  - Hash:

### S8 — A failed regeneration keeps the preserved candidate's billed amount

- [ ] **RED** — remove `@wip`; same-command pattern with this title; non-zero exit.
  - Evidence:
- [ ] **GREEN** — `apply_regeneration_failure` leaves the state untouched
      while a successful regeneration replaces the Amount. Production files:
      `src/review/panel.rs`, `src/main.rs`.
  - Evidence:
- [ ] **REFACTOR** — same command → zero exit.
  - Evidence:
- [ ] **COMMIT** — `feat(visible-request-amount): A failed regeneration keeps the preserved candidate's billed amount`
  - Hash:

### S9 — An unusable response still shows the amount it was billed

- [ ] **RED** — remove `@wip`; same-command pattern with this title; non-zero exit.
  - Evidence:
- [ ] **GREEN** — the unusable-response branch fills the Amount from the same
      response's usage. Production files: `src/main.rs`.
  - Evidence:
- [ ] **REFACTOR** — same command → zero exit.
  - Evidence:
- [ ] **COMMIT** — `feat(visible-request-amount): An unusable response still shows the amount it was billed`
  - Hash:

### S10 — A failed request shows no billed amount

- **Already green — recorded regression guard.**
- [ ] **VERIFY** — same-command pattern with this title → zero exit, after S9's GREEN.
  - Evidence:
- [ ] **COMMIT** — covered by S9's commit. Same recorded exception.
  - Hash: covered by S9

### S11 — The billed amount never reaches command output

- **Already green — recorded regression guard.**
- [ ] **VERIFY** — same-command pattern with this title → zero exit, after S1's GREEN.
  - Evidence:
- [ ] **COMMIT** — covered by S1's commit. Same recorded exception.
  - Hash: covered by S1

## E2E setup task

- [ ] **T2 — E2E environment and runner proof.**
  - Local runnability: no containers and no network; the provider twin is the
    existing `httpmock` loopback server and the terminal is the existing
    `portable-pty` harness (`design.md`, Local runnability).
  - `verify.e2e_command` = `./run-tests.sh --e2e`, already configured and a
    strict subset of `verify.command` = `./run-tests.sh`.
  - Scenario-count proof: run `./run-tests.sh` and record its count; run
    `./run-tests.sh --e2e` and record its count; the e2e count must be
    strictly smaller.
  - Evidence:

## Scenarios — end-to-end (@e2e), after every in-process scenario is GREEN

### S12 — The review surface shows the billed amount of the request that produced it

- [ ] **RED** — remove `@wip`; write the e2e steps with
      `unimplemented!("TODO")`; `./run-tests.sh --e2e --name "The review surface shows the billed amount of the request that produced it"`; non-zero exit required.
  - Evidence:
- [ ] **GREEN** — real subprocess on a pseudo-terminal; the steps configure the
      recorded price and the reported usage, then read the rendered header.
      Production files: any remaining wiring in `src/main.rs`.
  - Evidence:
- [ ] **REFACTOR** — same command → zero exit.
  - Evidence:
- [ ] **COMMIT** — `test(e2e): The review surface shows the billed amount of the request that produced it`
  - Hash:

### S13 — The explanation card shows the billed amount of its explanation request

- [ ] **RED** — remove `@wip`; e2e steps with `unimplemented!("TODO")`;
      `./run-tests.sh --e2e --name "The explanation card shows the billed amount of its explanation request"`; non-zero exit required.
  - Evidence:
- [ ] **GREEN** — `run_explanation_card` takes the Amount and its call site
      computes it from the explanation response (`src/main.rs`).
  - Evidence:
- [ ] **REFACTOR** — same command → zero exit.
  - Evidence:
- [ ] **COMMIT** — `test(e2e): The explanation card shows the billed amount of its explanation request`
  - Hash:

### S14 — Developer accepts an explained candidate from Ctrl-W (modified permanent scenario)

- [ ] **RED** — the `@givn.modified` scenario in
      `specs/use-shell/interactive-shell-shortcut.feature` asserts the Bash
      command line is exactly the accepted candidate and contains no billed
      amount; its new steps are undefined today.
      `./run-tests.sh --e2e --name "Developer accepts an explained candidate from Ctrl-W"`; non-zero exit required.
  - Evidence:
- [ ] **GREEN** — bind the exact-match and no-amount assertions to the accepted
      candidate and the Bash buffer.
  - Evidence:
- [ ] **REFACTOR** — same command → zero exit.
  - Evidence:
- [ ] **COMMIT** — `test(e2e): Developer accepts an explained candidate from Ctrl-W`
  - Hash:

## Done when

- Every box above is checked with its evidence pasted and its commit hash
  recorded, except the four recorded regression guards, which reference the
  commit that verifies them.
- `./run-tests.sh` and `./run-tests.sh --e2e` both exit 0 on the full suite.
- No `@wip` remains in this change's specs; no `@e2e` tag was removed.
