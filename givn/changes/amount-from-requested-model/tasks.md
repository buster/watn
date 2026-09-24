# Tasks: amount-from-requested-model

## Domain constraints (apply to every scenario task)

From `givn/specs/observe-request-cost/usecase.md` and its capability
`visible-request-amount`, and from the `ask` fragment's metadata line:

- The price that applies to a request is the recorded price of the model the
  provider reported; when that model has no recorded price, the recorded price
  of the model the request was sent with applies. Nothing is invented when
  neither has an entry — the Model label then stands alone.
- The Amount renders with one significant digit and never more than four
  decimal places of a cent, trailing zeros trimmed, whole cents from one cent
  on; an amount below the last displayed step reads `0` (R-094).
- The Amount is shown only when the response reported usage and a price
  applies; a reported zero shows a genuine zero, an unaccounted request shows
  nothing, and the stderr metadata line keeps its `$0.0000` behaviour.
- The Amount never becomes command output and gates no review decision.
- Persona lens: `terminal-developer--interactive` (cost stance, 2026-09-24) —
  the money must reach the developer inside the surface that asks for the next
  decision, in a form readable at a glance.

Design references: `design.md` (D1, D2, the visual design contract),
`design-review.md`, `arc42.md`.

## Deviation: implementation precedes this task list

The change was implemented before this file existed, as the predecessor change
recorded for itself: the two new scenarios' steps, the price fallback, and the
rendering rule were authored in one pass, and the design-review grilling ran
against that plan. Every scenario below keeps its own RED and GREEN evidence.
Commits are whole-change commits rather than one per scenario because both
rules share one delta feature file (`specs/observe-request-cost/
visible-request-amount.feature`) and every revision must stay green; the
mapping is in "Commit mapping" below.

## Setup task

- [x] **T1 — Harness for both new scenarios.**
  - `tests/steps/interactive_shell_shortcut_steps.rs`: `surface_amount_for`
    builds the pricing map from the scenario's recorded prices and calls the
    production `amount::recorded_price`, taking the billed model and the
    requested model separately, so the in-process scenarios exercise the same
    lookup the product uses.
  - `tests/steps/observe_request_cost_e2e_steps.rs`: `commit_e2e_transcript`
    takes the owning change's directory, and a progress line sharing the
    header's row no longer takes the model label and the amount out of the
    committed transcript.
  - Strict mode unchanged: `.fail_on_skipped()` plus the skipped-count exit in
    `tests/features_runner.rs`; no scenario in scope carries `@wip`.
  - Evidence: `./run-tests.sh` exit 0, `279 scenarios (279 passed)` (271 before
    this change; the eight modified duplicates are the only additions while the
    change is open).

## S1 — A provider's unknown model falls back to the requested model's recorded price

(`@givn.added`, in-process, `visible-request-amount`)

- [x] **RED** — temporarily neuter the fallback in the production lookup
      (`let _ = requested_model; pricing.get(reported_model)`), run
      `./run-tests.sh --name "A provider's unknown model falls back to the requested model's recorded price"`.
  - Evidence: exit=1; the model label carries no amount at all for the aliased
    configuration, so the step panics with
    `the model label should carry a billed amount`.
- [x] **GREEN** — `amount::recorded_price` plus the `surface_amount` call sites;
      restore the lookup; same command → exit 0.
  - Evidence: exit=0, `1 scenario (1 passed)`; command output in the commit's
    change delta.
- [x] **REFACTOR** — the search for `config.pricing.get(` finds only
      `recorded_price`'s own body and the harness call; no duplicated lookup
      remains. Same command → exit 0.
  - Evidence: exit=0.
- [x] COMMIT: `e1d2848` — `feat(visible-request-amount): the requested-model price fallback and the one-significant-digit cents form`

## S2 — The metadata line prices a request from the requested model when the reported model has none

(`@givn.added`, e2e-shaped step driving the real binary, `ask`)

- [x] **RED** — same temporary neutering of the fallback; run
      `./run-tests.sh --name "The metadata line prices a request from the requested model when the reported model has none"`.
  - Evidence: exit=1; stderr carries no cost for `response-model`.
- [x] **GREEN** — the ask path's metadata site calls `recorded_price`; restore
      the lookup; same command → exit 0.
  - Evidence: exit=0, `1 scenario (1 passed)`; stderr now contains a non-zero
    cost while the final metadata still names exactly `response-model`.
- [x] **REFACTOR** — the metadata sites share the one helper; no change; same
      command → exit 0.
  - Evidence: exit=0.
- [x] COMMIT: `e1d2848` — shared with S1; one lookup function serves both, and
      the two feature files must land with it to keep the revision green.

## S3 — The Amount keeps one significant digit, never more than four decimal places

(`@givn.modified` × 8, `visible-request-amount`; D2)

- [x] **RED** — force the shipped four-decimal form in `cents_text`
      (`let decimals = 4;`) and run both scopes.
  - Evidence: `./run-tests.sh` → exit 1, `279 scenarios (266 passed, 13 failed)`
    — the added S1 scenario and the six in-process modifications, each in its
    permanent and delta copy; `./run-tests.sh --e2e` → exit 1,
    `93 scenarios (89 passed, 4 failed)` — the two e2e modifications, each in
    its permanent and delta copy. Representative panics:
    `expected a billed amount of 0.02 cents at 4 decimals, got "0.0234"`,
    `expected a billed amount of 0.6 cents at 3 decimals, got "0.575"`,
    `expected a billed amount of 1 cents at 2 decimals, got "1.35"`.
- [x] **GREEN** — implement the rule in `BilledAmount::cents_text()` (one digit
      after the leading zeroes, capped at four decimals, trailing zeros
      trimmed, whole cents from one cent on) and update the unit tests; write
      the eight modified scenarios and pre-apply them to the permanent spec.
  - Evidence: `cargo test --lib --features test-support amount` → `8 passed`
    (`one_significant_digit_carries_the_value_past_its_leading_zeroes` with
    `0.0009`, `0.001`, `0.01`, `0.1`, `0.1`, `0.1`;
    `four_decimal_places_is_the_smallest_step_shown` with `0.0001` and `0`;
    `an_amount_of_a_cent_or_more_reads_as_whole_cents` with `1` and `2`;
    `reported_usage_is_billed_at_the_recorded_price` now `0.02`);
    `./run-tests.sh` → exit 0, `279 scenarios (279 passed)`;
    `./run-tests.sh --e2e` → exit 0, `93 scenarios (93 passed)`.
- [x] **REFACTOR** — `a_billed_request_never_reads_as_zero` is deleted: the cap
      makes its guarantee false by decision D2, and R-094 records the residual
      risk. `trailing_zeros_are_trimmed` is replaced by the three rule tests.
      Same commands → exit 0.
  - Evidence: exit=0 in both scopes.
- [x] COMMIT: `e1d2848` — shared with S1 and S2; the rendering rule and the
      price rule share the delta feature file, so no green per-scenario split
      exists.

## Evidence of the rendering

- [x] **Visual evidence re-committed under this change.** The two money
      scenarios write their transcripts through the hardened transform, so the
      archived predecessor's transcripts are not overwritten and this change
      proves its own rendering.
  - Evidence: `givn/changes/amount-from-requested-model/evidence/visual/` —
    `the-review-surface-shows-the-billed-amount-of-the-request-that-produced-it/
    transcript.txt` shows `◆ gpt-4o-mini · 1 ¢`; the explanation-card transcript
    shows `◆ tier 1 · test/gpt-4o-mini · 1 ¢`.
  - COMMIT: `aebdd80` — `test(amount-from-requested-model): commit the money transcripts as the change's visual evidence`

## Commit mapping

| Task | Commit |
|---|---|
| S1 (fallback, review surface) | `e1d2848` |
| S2 (fallback, metadata line) | `e1d2848` |
| S3 (rendering form) | `e1d2848` |
| Visual evidence | `aebdd80` |
| Task list and review artifacts | the change's documentation commits, tracked with `givn/changes/amount-from-requested-model/` |
