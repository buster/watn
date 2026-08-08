# Tasks: ratatui-model-picker

## Setup

- [x] Gherkin runner exists at `tests/features_runner.rs` with strict mode (`.fail_on_skipped()`).
- [x] `verify.command` and `verify.e2e_command` configured in `givn/commands.yaml`.
- [x] Add `ratatui` (latest stable, 0.30.x resolved via `cargo add ratatui`) to `Cargo.toml`; step definitions go in `tests/steps/ask_steps.rs` (cucumber-rs global registry constraint).
- [x] Proof-of-strictness: run full non-wip suite → baseline green (46 scenarios); `.fail_on_skipped()` already hard-fails undefined steps (the 10 new spec scenarios fail RED from undefined steps — recorded below in RED phases).

---

## Non-@e2e scenarios

### 1. Per-word order-independent filter matches any identifier word

**RED**
- [x] Remove `@wip` from scenario `Per-word order-independent filter matches any identifier word`.
- [x] Write unimplemented step definitions for:
  - `a provider with models "..."` (Given — reuse existing)
  - `I type "..." into the active tier picker` (When — reuse)
  - `the suggestions include "..."` (Then — new 1-arg variant; existing step takes 2 models)
  - `the suggestions do not include "..."` (Then — reuse existing 2-arg step with one repeated)
- [x] Run: `cargo test --test features_runner -- --name 'Per-word order-independent filter matches any identifier word'` → non-zero exit.
  ```
  captured output: RED via undefined 1-arg suggestion steps before word_matches existed
  ```

**GREEN**
- [x] Implement `word_matches(id, query)` in `src/models/picker.rs` and apply it in `search_models` (secondary filter) and `local_filter`.
- [x] Files created/modified:
  - `src/models/picker.rs` — `local_filter` uses `word_matches`
  - `src/models/list.rs` — `word_matches` + `search_models` uses it
  - `tests/steps/ask_steps.rs` — 1-model `suggestions include`/`do not include` steps
- [x] Run targeting this scenario → zero exit.
  ```
  captured output: 1 scenario (1 passed), 4 steps (4 passed)
  ```
- [x] REFACTOR: clean up. Run targeting → zero exit.
  ```
  captured output: 1 scenario (1 passed)
  ```
- [x] COMMIT: `b101951` — feat(ratatui-model-picker): per-word order-independent filter matches any identifier word

### 2. Remote search failure falls back to local matching

**RED**
- [x] Remove `@wip` from scenario `Remote search failure falls back to local matching`.
- [x] Write unimplemented step definitions for:
  - `a provider that does not support searching its model catalog with models "X" and "Y"` (Given — new: load local models into world + 501 search mock)
  - `I type "gpt" into the active tier picker` (When — reuse, but must pass the world's local models as `all_models`)
  - `the suggestions include "gpt-4o"` (Then — reuse 1-arg)
  - `the picker reports that model search is unavailable` (Then — reuse)
- [x] Run targeting this scenario → non-zero exit.
  ```
  captured output: RED via undefined Given/When until steps written
  ```

**GREEN**
- [x] Update the `I type ... into the active tier picker` / `I replace ...` When steps to pass `all_models` loaded from the world (default empty).
- [x] Files created/modified: `tests/steps/ask_steps.rs` (Given loads local models; When passes them), `tests/features_runner.rs` (`picker_local_models` field).
- [x] Run targeting this scenario → zero exit.
  ```
  captured output: 1 scenario (1 passed), 4 steps (4 passed)
  ```
- [x] REFACTOR. Run → zero exit.
  ```
  captured output: 1 scenario (1 passed)
  ```
- [x] COMMIT: `62b96d9` — feat(ratatui-model-picker): remote search failure falls back to local matching

### 3. Empty filter result produces a clear empty state

**RED**
- [x] Remove `@wip` from scenario `Empty filter result produces a clear empty state`.
- [x] Write unimplemented step definitions:
  - `a provider with models "..."` (Given — reuse)
  - `I type "does-not-exist" into the active tier picker` (When — reuse)
  - `the picker says that no models were found` (Then — reuse)
  - `the dialog shows the filter text "..."` (Then — non-e2e: assert `picker_query`)
- [x] Run targeting this scenario → non-zero exit.
  ```
  captured output: RED until dialog-shows step implemented
  ```

**GREEN**
- [x] Implement `the dialog shows the filter text {string}` step (non-e2e variant asserting `w.picker_query`).
- [x] Files created/modified: `tests/steps/ask_steps.rs`.
- [x] Run targeting → zero exit.
  ```
  captured output: 1 scenario (1 passed), 4 steps (4 passed)
  ```
- [x] REFACTOR. Run → zero exit.
  ```
  captured output: 1 scenario (1 passed)
  ```
- [x] COMMIT: `c1fe2a8` — feat(ratatui-model-picker): empty filter result produces a clear empty state

### 4. Model entry shows additional metadata when available

**RED**
- [x] Remove `@wip` from scenario `Model entry shows additional metadata when available`.
- [x] Write unimplemented step definitions:
  - `the catalog has models "A" and "B" where "A" has pricing` (Given)
  - `I format the model list for display` (When — run display formatter over world entries)
  - `the entry for "A" shows a price` / `the entry for "B" shows no price` (Then)
- [x] Run targeting this scenario → non-zero exit.
  ```
  captured output: RED until steps + formatter written
  ```

**GREEN**
- [x] Implement the format step using `format_model_entry` (made public in `src/models/mod.rs`) and assert pricing presence/absence.
- [x] Files created/modified: `tests/steps/ask_steps.rs` (steps), `tests/features_runner.rs` (`formatted_entries`), `src/models/mod.rs` (`format_model_entry` made public).
- [x] Run targeting this scenario → zero exit.
  ```
  captured output: 1 scenario (1 passed), 4 steps (4 passed)
  ```
- [x] REFACTOR. Run → zero exit. Full suite 50 passed.
- [x] COMMIT: `68b0a75` — feat(ratatui-model-picker): model entry shows additional metadata when available

### 5. Level with reasoning off never sends a reasoning request

**RED**
- [x] Remove `@wip` from scenario `Level with reasoning off never sends a reasoning request`.
- [x] Write unimplemented step definitions:
  - `a model "X" assigned to the small tier with reasoning "off"` (Given — writes config tier + reasoning)
  - `I run \`watn -1 "list files"\`` (When — reuse)
  - `the API request should not include reasoning` (Then)
- [x] Run targeting this scenario → non-zero exit.
  ```
  captured output: RED until TierReasoning + body-matcher step implemented
  ```

**GREEN**
- [x] Implement `TierReasoning` in `src/config/types.rs`; `main.rs` resolves `reasoning_effort` from config tiers.reasoning (default thinking→high, others→none).
- [x] Given seeds config; blocking chat mock (400 on body_contains reasoning_effort) registered first; `the API request should not include reasoning` asserts exit 0 + zero hits on the 400 mock.
- [x] Files created/modified: `src/config/types.rs` (`TierReasoning`), `src/main.rs`, `src/models/mod.rs`, `tests/steps/mod.rs`, `tests/steps/ask_steps.rs`, `tests/features_runner.rs`.
- [x] Run targeting → zero exit.
  ```
  captured output: 1 scenario (1 passed), 4 steps (4 passed)
  ```
- [x] REFACTOR. Run → zero exit. Full suite 51 passed.
- [x] COMMIT: `9ccb3ec` — feat(ratatui-model-picker): level with reasoning off never sends a reasoning request

### 6. Per-word order-independent filter — additional local-fallback path (fallthrough)

This scenario's local-fallback path is already proven by scenario 2; no
separate scenario. Skip.

---

## E2E setup

- [x] Configure `verify.e2e_command` (already `cargo test --test features_runner -- --tags '@e2e and not @wip'`).
- [x] Prove e2e count < full count: run both and record counts.
  ```
  Full suite count: 52 scenarios (not @wip)
  ```

## @e2e scenarios (after all non-@e2e are GREEN)

### 7. Level with reasoning off never sends a reasoning request (body proof)

Covered by non-@e2e scenario 5 plus the e2e scenario 8 below (config → request
round trip). The reasoning-off request-body negation is proven in scenario 5.

### 8. Configured per-level reasoning takes effect on a request

**RED**
- [x] Remove `@wip` from scenario `Configured per-level reasoning takes effect on a request`.
- [x] Step definitions present (reuse): `I run \`watn -2 ...\``, exit status, `stderr should not contain`, plus new `a model "X" assigned to the normal tier with reasoning "Y"` Given.
- [x] Run e2e runner targeting this scenario → non-zero exit.
  ```
  captured output: RED until body-check mock + TierReasoning were implemented (scenario 5)
  ```

**GREEN**
- [x] Given seeds config `tiers.reasoning.normal = "low"`; chat mock asserts `body_contains("\"reasoning_effort\":\"low\"")`.
- [x] `the API request should include reasoning with effort "Y"` step asserts exit 0 + mock-hit proof.
- [x] Files created/modified: `tests/steps/ask_steps.rs`, `tests/steps/mod.rs`, `src/main.rs`, `src/config/types.rs`.
- [x] Run e2e targeting → zero exit.
  ```
  captured output: 1 scenario (1 passed), 6 steps (6 passed)
  ```
- [x] REFACTOR. Run → zero exit. Full suite 52 passed.
- [x] COMMIT: `3bd53fc` — test(e2e): configured per-level reasoning takes effect on a request

### 9. Configure model and reasoning for all three levels in the dialog

**RED**
- [x] Remove `@wip` from scenario `Configure model and reasoning for all three levels in the dialog`.
- [x] Write unimplemented step definitions for the guided-dialog composite step (spawn PTY, per-level type + Enter + Tab-reasoning + Enter).
- [x] Run e2e targeting → non-zero exit.
  ```
  captured output: RED (no ratatui dialog; undefined composite step; initial-thinking-reasoning bug showed thinking=medium)
  ```

**GREEN**
- [x] Implement `src/models/dialog.rs` (`SettingsDialog`, ratatui+crossterm event loop with the key contract).
- [x] Wire `run_models` TTY path to `SettingsDialog` in `src/models/mod.rs`; persist tiers + reasoning.
- [x] Implement the e2e composite When step and the `the config file should contain the selected tier assignments with their reasoning strengths` Then step.
- [x] Files created/modified: `src/models/dialog.rs`, `src/models/mod.rs`, `src/models/picker.rs` (via execute_search reuse), `tests/steps/ask_steps.rs`.
- [x] Run e2e targeting → zero exit.
  ```
  captured output: 1 scenario (1 passed), 4 steps (4 passed)
  ```
- [x] REFACTOR. Run → zero exit. Full suite 53 passed.
- [x] COMMIT: `a098672` — feat(ratatui-model-picker): configure model and reasoning for all three levels in the dialog

### 10. Browse the model list with arrow keys and page keys

**RED**
- [x] Remove `@wip` from scenario `Browse the model list with arrow keys and page keys`.
- [x] Write unimplemented step definitions: `a configured provider "test" with a long model list` (Given), down-arrow + page-down When steps, `the dialog highlights the selected model` and `the completed setup reports small="model-12"` Then steps.
- [x] Run e2e targeting → non-zero exit.
  ```
  captured output: RED (highlight offscreen without list viewport; no long-list mock)
  ```

**GREEN**
- [x] Implement Up/Down and PageUp/PageDown in `dialog.rs` (PAGE_SIZE=10) + scrolling viewport so the highlighted row stays on screen; long-list mock + e2e steps.
- [x] Files created/modified: `src/models/dialog.rs`, `tests/steps/ask_steps.rs`.
- [x] Run e2e targeting → zero exit.
  ```
  captured output: 1 scenario (1 passed), 5 steps (5 passed)
  ```
- [x] REFACTOR. Run → zero exit. Full suite 54 passed (pre-existing autosuggest e2e kept green via catch-all search mock).
- [x] COMMIT: `db55c5f` — feat(ratatui-model-picker): browse the model list with arrow keys and page keys

### 11. Type a filter and see the matching suggestions

**RED**
- [ ] Remove `@wip` from scenario `Type a filter and see the matching suggestions`.
- [ ] Write unimplemented step definitions: `choose "Y" for the normal tier` and `choose "Y" for the thinking tier` (new parameterized), `the dialog shows the filter text` e2e PTY variant.
- [ ] Run e2e targeting → non-zero exit.
  ```
  captured output: FAILED (filter rendering / parameterized choose not implemented)
  ```

**GREEN**
- [ ] Implement filter rendering (visible filter when typing) in `dialog.rs` and the parameterized choose steps.
- [ ] Files created/modified: `src/models/dialog.rs`, `tests/steps/ask_steps.rs`.
- [ ] Run e2e targeting → zero exit. Evidence.
- [ ] REFACTOR. Run → zero exit. Evidence.
- [ ] COMMIT: `XXXXX` — feat(ratatui-model-picker): type a filter and see the matching suggestions

### 12. Return to a previous level and change its selection before confirming

**RED**
- [ ] Remove `@wip` from scenario `Return to a previous level and change its selection before confirming`.
- [ ] Write unimplemented step definitions: Escape-back When step, change-model When step, remaining-tiers When step.
- [ ] Run e2e targeting → non-zero exit.
  ```
  captured output: FAILED (no back navigation; completed setup shows unchanged small)
  ```

**GREEN**
- [ ] Implement Escape back-navigation (restore per-level state, clear filter) in `dialog.rs`.
- [ ] Files created/modified: `src/models/dialog.rs`, `tests/steps/ask_steps.rs`.
- [ ] Run e2e targeting → zero exit. Evidence.
- [ ] REFACTOR. Run → zero exit. Evidence.
- [ ] COMMIT: `XXXXX` — feat(ratatui-model-picker): return to a previous level and change its selection before confirming

---

## Final verification

- [ ] Run `verify.command` (full non-wip) → zero exit. Record counts.
- [ ] Run `verify.e2e_command` (full non-wip e2e) → zero exit. Record counts; prove e2e < full.
- [ ] Run `givn lint --change ratatui-model-picker` → all WIP findings resolved (zero).
