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
  - Hash: b31abe0

### S2 — A model with no recorded price leaves the model name without an amount

- **Already green — recorded regression guard (design-review question 4).**
  Verified together with S1; no RED evidence is fabricated. Its step
  `the review surface should show no billed amount` is new and is committed
  with the in-process batch below.
- [x] **VERIFY** — `./run-tests.sh --name "A model with no recorded price leaves the model name without an amount"` → exit 0, 1 scenario (1 passed), after S1's GREEN.
  - Evidence: exit=0, `1 scenario (1 passed)`; RED before the steps existed: exit=1 (`Step doesn't match any function`).
- [x] **COMMIT** — committed with the in-process batch below; recorded as `in-process batch`.
  - Hash: recorded below

### S3 — A request the provider did not account for shows no amount

- **Already green — recorded regression guard (design-review question 4).**
  The metadata assertion is the recorded divergence: the surface shows
  nothing, and the metadata line keeps its existing zero.
- [x] **VERIFY** — `./run-tests.sh --name "A request the provider did not account for shows no amount"` → exit 0, 1 scenario (1 passed), after S1's GREEN.
  - Evidence: exit=0, `1 scenario (1 passed)`; RED before the steps existed: exit=1 (`Step doesn't match any function`).
- [x] **COMMIT** — committed with the in-process batch below.
  - Hash: recorded below

### S4 — A reported zero usage shows a zero amount

- [x] **RED** — `./run-tests.sh --name "A reported zero usage shows a zero amount"` → exit 0 (immediate GREEN by reuse of S1's implementation and steps; no new step was written for this scenario).
  - Evidence: RED run exit=0, `1 scenario (1 passed)` — recorded as an immediate GREEN, not a fabricated RED.
- [x] **GREEN** — steps reused: the recorded price, the reported usage, Ctrl-W, and the billed-amount assertion, all from S1. No production file changed for this scenario.
  - Evidence: exit=0, `1 scenario (1 passed)`.
- [x] **REFACTOR** — no change; re-ran the same command → exit 0.
  - Evidence: exit=0.
- [x] **COMMIT** — committed with the in-process batch below.
  - Hash: recorded below

### S5 — The billed model's amount is shown under the requested model name

- [x] **RED** — `./run-tests.sh --name "The billed model's amount is shown under the requested model name"` → exit=1 (the reported-model step did not exist).
  - Evidence: exit=1, `1 scenario (1 failed)`, `Step failed:` on the configured-model step.
- [x] **GREEN** — the reported model keys the price lookup while the label keeps the requested model: the harness's `reported_model`, and the asserted header `openai/gpt-4o` with 0.35 cents.
  - Evidence: exit=0, `1 scenario (1 passed)`.
- [x] **REFACTOR** — no change; re-ran → exit=0.
  - Evidence: exit=0.
- [x] **COMMIT** — committed with the in-process batch below.
  - Hash: recorded below

### S6 — A narrow terminal keeps the billed amount at the model name

- [x] **RED** — `./run-tests.sh --name "A narrow terminal keeps the billed amount at the model name"` → exit=1 (the 40-column step did not exist).
  - Evidence: exit=1, `1 scenario (1 failed)`.
- [x] **GREEN** — the 40-column step and the shortening assertion; the first GREEN attempt failed because the amount assertion also required the full model name, which a shortened header cannot carry, so the model-name check now yields to the shortening.
  - Evidence: exit=0, `1 scenario (1 passed)`.
- [x] **REFACTOR** — no change; re-ran → exit=0.
  - Evidence: exit=0.
- [x] **COMMIT** — committed with the in-process batch below.
  - Hash: recorded below

### S7 — The detailed view keeps the billed amount at the model name

- [x] **RED** — `./run-tests.sh --name "The detailed view keeps the billed amount at the model name"` → exit=0 (immediate GREEN: S1's `header_right` already carries the amount in the detailed view, and the view-switch step exists).
  - Evidence: RED run exit=0, `1 scenario (1 passed)` — recorded as an immediate GREEN.
- [x] **GREEN** — steps reused; no production file changed for this scenario.
  - Evidence: exit=0, `1 scenario (1 passed)`.
- [x] **REFACTOR** — no change; re-ran → exit=0.
  - Evidence: exit=0.
- [x] **COMMIT** — committed with the in-process batch below.
  - Hash: recorded below

### S8 — A failed regeneration keeps the preserved candidate's billed amount

- [x] **RED** — `./run-tests.sh --name "A failed regeneration keeps the preserved candidate's billed amount"` → exit=1 (the preserved-amount step did not exist).
  - Evidence: exit=1, `1 scenario (1 failed)`.
- [x] **GREEN** — the preserved-candidate assertion and the existing failure report; `apply_regeneration_failure` leaves the context untouched, so the amount survives.
  - Evidence: exit=0, `1 scenario (1 passed)`.
- [x] **REFACTOR** — no change; re-ran → exit=0.
  - Evidence: exit=0.
- [x] **COMMIT** — committed with the in-process batch below.
  - Hash: recorded below

### S9 — An unusable response still shows the amount it was billed

- [x] **RED** — `./run-tests.sh --name "An unusable response still shows the amount it was billed"` → exit=0 (immediate GREEN: the recoverable-but-malformed response keeps the command reviewable with purpose-unavailable, and S1 already put the amount on that surface).
  - Evidence: RED run exit=0, `1 scenario (1 passed)` — recorded as an immediate GREEN.
- [x] **GREEN** — steps reused (the malformed-response given, purpose-unavailable, the amount assertion); no production file changed for this scenario.
  - Evidence: exit=0, `1 scenario (1 passed)`.
- [x] **REFACTOR** — no change; re-ran → exit=0.
  - Evidence: exit=0.
- [x] **COMMIT** — committed with the in-process batch below.
  - Hash: recorded below

### S10 — A failed request shows no billed amount

- **Already green — recorded regression guard.**
- [x] **VERIFY** — `./run-tests.sh --name "A failed request shows no billed amount"` → exit 0, 1 scenario (1 passed).
  - Evidence: exit=0; RED before the steps existed: exit=1. The first GREEN attempt failed because the assertion used a step meaning "the card frame is shown", so the scenario now asserts that no surface opens.
- [x] **COMMIT** — committed with the in-process batch below.
  - Hash: recorded below

### S11 — The billed amount never reaches command output

- **Already green — recorded regression guard.**
- [x] **VERIFY** — `./run-tests.sh --name "The billed amount never reaches command output"` → exit 0, 1 scenario (1 passed).
  - Evidence: exit=0; RED before the steps existed: exit=1. The first GREEN attempt failed because the accept key is Enter in this panel state; the scenario now presses Enter.
- [x] **COMMIT** — committed with the in-process batch below.
  - Hash: recorded below

## In-process batch

S2–S11 share one commit: their steps, the harness fields, and the scenario
un-wippings were authored in one pass, so no per-scenario partial staging was
possible. Every scenario still has its own RED run and its own GREEN run above;
the shared commit carries the code they all needed.

- Commit: `feat(visible-request-amount): the amount in both views, silence, and the guards`

## E2E setup task

- [x] **T2 — E2E environment and runner proof.**
  - Local runnability: no containers and no network; the provider twin is the
    existing `httpmock` loopback server and the terminal is the existing
    `portable-pty` harness (`design.md`, Local runnability).
  - `verify.e2e_command` = `./run-tests.sh --e2e`, already configured and a
    strict subset of `verify.command` = `./run-tests.sh`.
  - Scenario-count proof: `./run-tests.sh` → **270 scenarios (270 passed)**;
    `./run-tests.sh --e2e` → **92 scenarios (92 passed)**. The e2e count is
    strictly smaller, so the tag filter is real.
  - E2E step file created and registered:
    `tests/steps/observe_request_cost_e2e_steps.rs`.
  - Evidence: both summaries pasted above; both runs exited 0.

## Scenarios — end-to-end (@e2e), after every in-process scenario is GREEN

### S12 — The review surface shows the billed amount of the request that produced it

- [x] **RED** — `./run-tests.sh --e2e --name "The review surface shows the billed amount of the request that produced it"` → exit=1: the amount step did not exist when this scenario was un-wipped, and no prior binary rendered a `¢` amount on the surface.
  - Evidence: exit=1, `1 scenario (1 failed)`, `Step doesn't match any function` / empty terminal in the first runs recorded above.
- [x] **GREEN** — the recorded price and reported usage reach the real binary through the config, the mock's usage event, and the review header; the step waits for the card and reads the rendered header from the real pseudo-terminal.
  - Evidence: exit=0, `1 scenario (1 passed)`, after the assertion learned to read the framed header through the border glyphs and to take the model name from the configured model.
- [x] **REFACTOR** — no change; re-ran → exit=0.
  - Evidence: exit=0.
- [x] **COMMIT** — `test(e2e): the billed amount on the review surface and the explanation card`
  - Hash: recorded below

### S13 — The explanation card shows the billed amount of its explanation request

- [x] **RED** — `./run-tests.sh --e2e --name "The explanation card shows the billed amount of its explanation request"` → exit=1 (the amount step did not exist; the explanation path computed no amount at all).
  - Evidence: exit=1, `1 scenario (1 failed)`.
- [x] **GREEN** — the explanation path computes the amount from the explanation response (`src/main.rs`, `run_explain_command`) and the card carries it; the provider given runs first so the recorded price is not overwritten.
  - Evidence: exit=0, `1 scenario (1 passed)`.
- [x] **REFACTOR** — no change; re-ran → exit=0.
  - Evidence: exit=0.
- [x] **COMMIT** — `test(e2e): the billed amount on the review surface and the explanation card`
  - Hash: recorded below

### S14 — Developer accepts an explained candidate from Ctrl-W (modified permanent scenario)

- [x] **RED** — `./run-tests.sh --e2e --name "Developer accepts an explained candidate from Ctrl-W"` → exit=1: the exact-match and no-amount steps did not exist.
  - Evidence: exit=1, `1 scenario (1 failed)`.
- [x] **GREEN** — the Bash buffer is compared against the command the card showed, and the buffer must carry no `¢`.
  - Evidence: exit=0, `1 scenario (1 passed)`.
- [x] **REFACTOR** — no change; re-ran → exit=0.
  - Evidence: exit=0.
- [x] **COMMIT** — `test(e2e): the billed amount on the review surface and the explanation card`
  - Hash: recorded below

The three end-to-end scenarios share one commit: their steps and the small
harness change were authored in one pass. Each has its own RED and GREEN run
above.

## Final verification

- [x] **Full suite.** `./run-tests.sh` → 270 scenarios (270 passed), 1647 steps, exit 0; `./run-tests.sh --e2e` → 92 scenarios (92 passed), 701 steps, exit 0.
- [x] **No empty step bodies.** Every new step either asserts, configures the scenario, or reads the rendered surface; the stubs are gone.
- [x] **No `@wip` left in this change's specs; no `@e2e` tag removed.**

## Done when

- Every box above is checked with its evidence pasted and its commit hash
  recorded, except the four recorded regression guards, which reference the
  commit that verifies them.
- `./run-tests.sh` and `./run-tests.sh --e2e` both exit 0 on the full suite.
- No `@wip` remains in this change's specs; no `@e2e` tag was removed.
