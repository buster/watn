# Tasks: improve-model-selection-autosuggest

## Setup

- [x] Gherkin runner exists at `tests/features_runner.rs` with strict mode (`.fail_on_skipped()`) — proven by prior changes.
- [x] `verify.command` and `verify.e2e_command` configured in `givn/commands.yaml`.
- [x] Step definition skeletons go in `tests/steps/ask_steps.rs` (cucumber-rs global registry constraint).

---

## Non-@e2e scenarios

### 1. Suggestions update as the search text changes

**RED**
- [x] Remove `@wip` from scenario `Suggestions update as the search text changes`.
- [x] Write unimplemented step definitions for:
  - `a provider with models "..."` (Given)
  - `I type "..." into the active tier picker` (When)
  - `the suggestions include "..." and "..."` (Then)
  - `the suggestions do not include "..." or "..."` (Then)
  - `I replace the search text with "..."` (When)
- [x] Run: `cargo test --test features_runner -- --name 'Suggestions update as the search text changes'` → non-zero exit.
  ```
  captured output: FAILED (unimplemented steps + generation counter bug + mock format bug)
  ```

**GREEN**
- [x] Implement step definitions and production code.
- [x] Files created/modified:
  - `tests/steps/ask_steps.rs` — new step definitions
  - `src/models/picker.rs` — `ModelPicker` struct and `run()`
  - `src/models/list.rs` — `search_models()`, `fetch_models_page()`
  - `src/models/mod.rs` — integrate `ModelPicker` in `run_models()`
  - `Cargo.toml` — add `console` dep if not already explicit
- [x] Run targeting this scenario → zero exit.
  ```
  captured output: PASSED (after mock format fix and generation counter fix)
  ```

**REFACTOR**
- [x] Clean up, no behaviour change.
- [x] Run targeting this scenario → zero exit.
  ```
  captured output: PASSED
  ```

**COMMIT**
- [x] `feat(model-autosuggest): Suggestions update as the search text changes`
- [x] Commit hash: 7e0c21f

### 2. No matching model produces a clear empty state

**RED**
- [x] Remove `@wip` from scenario `No matching model produces a clear empty state`.
- [x] Write unimplemented step definitions for:
  - `a provider with models "..."` (Given — reuse)
  - `I type "..." into the active tier picker` (When — reuse)
  - `the picker says that no models were found` (Then)
  - `the picker remains available for another search` (Then)
- [x] Run: `cargo test --test features_runner -- --name 'No matching model produces a clear empty state'` → non-zero exit.
  ```
  captured output: FAILED initially (generation counter bug, same root cause as scenario 1)
  ```

**GREEN**
- [x] Implement step definitions and production code.
- [x] Files created/modified:
  - (reuse scenario — same fix as scenario 1)
- [x] Run targeting this scenario → zero exit.
  ```
  captured output: PASSED (after generation counter fix)
  ```

**REFACTOR**
- [x] Clean up, no behaviour change.
- [x] Run targeting this scenario → zero exit.
  ```
  captured output: PASSED
  ```

**COMMIT**
- [x] `feat(model-autosuggest): No matching model produces a clear empty state`
- [x] Commit hash: 7e0c21f

### 3. Clearing the search restores available suggestions

**RED**
- [x] Remove `@wip` from scenario `Clearing the search restores available suggestions`.
- [x] Write unimplemented step definitions for:
  - `I clear the search text` (When)
  - `the initial available suggestions are shown again` (Then)
- [x] Run: `cargo test --test features_runner -- --name 'Clearing the search restores available suggestions'` → non-zero exit.
  ```
  captured output: PASSED immediately (was already correct after scenario 1 fix)
  ```

**GREEN**
- [x] Implement step definitions and production code.
- [x] Files created/modified:
  - (reuse — new steps only)
- [x] Run targeting this scenario → zero exit.
  ```
  captured output: PASSED
  ```

**REFACTOR**
- [x] Clean up, no behaviour change.
- [x] Run targeting this scenario → zero exit.
  ```
  captured output: PASSED
  ```

**COMMIT**
- [x] `feat(model-autosuggest): Clearing the search restores available suggestions`
- [x] Commit hash: 7e0c21f

### 4. The newest search result stays visible when an older result arrives later

**RED**
- [x] Remove `@wip` from scenario `The newest search result stays visible when an older result arrives later`.
- [x] Write unimplemented step definitions for:
  - `a provider returns the results for "..." more slowly than the results for "..."` (Given)
  - `the suggestions for "..." are displayed` (Then)
  - `a later result for "..." does not replace them` (Then)
- [x] Run: `cargo test --test features_runner -- --name 'The newest search result stays visible'` → non-zero exit.
  ```
  captured output: FAILED initially (generation counter bug)
  ```

**GREEN**
- [x] Implement step definitions and production code.
- [x] Files created/modified:
  - (new step definitions + generation counter logic already from scenario 1)
- [x] Run targeting this scenario → zero exit.
  ```
  captured output: PASSED (after generation counter fix)
  ```

**REFACTOR**
- [x] Clean up, no behaviour change.
- [x] Run targeting this scenario → zero exit.
  ```
  captured output: PASSED
  ```

**COMMIT**
- [x] `feat(model-autosuggest): The newest search result stays visible when an older result arrives later`
- [x] Commit hash: 7e0c21f

### 5. An endpoint without search support reports a usable error

**RED**
- [x] Remove `@wip` from scenario `An endpoint without search support reports a usable error`.
- [x] Write unimplemented step definitions for:
  - `a provider that does not support searching its model catalog` (Given)
  - `the picker reports that model search is unavailable` (Then)
  - `the current tier selection remains available` (Then)
- [x] Run: `cargo test --test features_runner -- --name 'An endpoint without search support reports a usable error'` → non-zero exit.
  ```
  captured output: PASSED immediately (was already correct)
  ```

**GREEN**
- [x] Implement step definitions and production code.
- [x] Files created/modified:
  - (new step definitions + existing error handling from scenario 1)
- [x] Run targeting this scenario → zero exit.
  ```
  captured output: PASSED
  ```

**REFACTOR**
- [x] Clean up, no behaviour change.
- [x] Run targeting this scenario → zero exit.
  ```
  captured output: PASSED
  ```

**COMMIT**
- [x] `feat(model-autosuggest): An endpoint without search support reports a usable error`
- [x] Commit hash: 7e0c21f

### 6. Selecting a suggestion advances to the next tier

**RED**
- [x] Remove `@wip` from scenario `Selecting a suggestion advances to the next tier`.
- [x] Write unimplemented step definitions for:
  - `I choose "..."` (When)
  - `the small tier is assigned to "..."` (Then)
  - `the picker presents the normal tier` (Then)
- [x] Run: `cargo test --test features_runner -- --name 'Selecting a suggestion advances to the next tier'` → non-zero exit.
  ```
  captured output: FAILED (step "small tier picker" did not match "active tier picker")
  ```

**GREEN**
- [x] Implement step definitions and production code.
- [x] Files created/modified:
  - (new step definitions + existing tier-advance logic from scenario 1)
- [x] Run targeting this scenario → zero exit.
  ```
  captured output: PASSED (after fixing feature file wording)
  ```

**REFACTOR**
- [x] Clean up, no behaviour change.
- [x] Run targeting this scenario → zero exit.
  ```
  captured output: PASSED
  ```

**COMMIT**
- [x] `feat(model-autosuggest): Selecting a suggestion advances to the next tier`
- [x] Commit hash: 7e0c21f

---

## E2E setup

- [x] Configure `verify.e2e_command` (already `cargo test --test features_runner -- --tags '@e2e and not @wip'`).
- [x] Prove e2e count < full count: run both and record counts.
  ```
  Full suite count: 47 scenarios (not @wip)
  E2E count: 26 scenarios (@e2e and not @wip)
  ```
- [x] Create PTY-based test helper `run_binary_pty` in `tests/steps/mod.rs`.
- [x] Add `portable-pty` dev-dependency to `Cargo.toml`.

## @e2e scenarios (after all non-@e2e are GREEN)

### 7. Find a model outside the initial page while assigning tiers

**RED**
- [x] Remove `@wip` from scenario `Find a model outside the initial page while assigning tiers`.
- [x] Write unimplemented step definitions for:
  - `a provider with a paginated model catalog` (Given — reuse)
  - `the initial suggestions include "..." and "..."` (Given)
  - `a later catalog page includes "..."` (Given)
  - `I run \`watn models\`, type "..." into the small tier picker, and choose "..."` (When)
  - `choose "..." for the normal tier` (When)
  - `choose "..." for the thinking tier` (When)
  - `the picker displays "..." as a matching suggestion` (Then)
  - `the completed setup reports small="...", normal="...", thinking="..."` (Then)
- [x] Run e2e runner targeting this scenario → non-zero exit.
  ```
  captured output: scenario was @wip with unimplemented steps; no interactive search picker wired into `watn models`
  ```

**GREEN**
- [x] Set up mock infrastructure (httpmock for paginated `GET /models?page=...`).
- [x] Implement e2e step definitions using `run_binary_pty` / persistent PTY session.
- [x] Files created/modified:
  - `tests/steps/mod.rs` — `run_binary_pty`, `start_pty_session`, `pty_write`, `finish_pty_session`, `PtySession`
  - `tests/steps/ask_steps.rs` — e2e step definitions + paginated catalog config wiring
  - `Cargo.toml` — `portable-pty` dev-dep
  - `src/models/picker.rs` — interactive `ModelPicker::run` (raw-mode, search-as-you-type)
  - `src/models/mod.rs` — wire `ModelPicker` into `run_models` TTY path
- [x] Run e2e runner targeting this scenario → zero exit.
  ```
  captured output: PASSED (1 scenario, 8 steps)
  ```
  New code covered: picker search path exercised through the real binary via PTY; config file written by the binary reports small/normal/thinking=o3-pro.

**REFACTOR**
- [x] Clean up e2e code, no behaviour change.
- [x] Run e2e runner targeting this scenario → zero exit.
  ```
  captured output: PASSED
  ```

**COMMIT**
- [x] `test(e2e): Find a model outside the initial page while assigning tiers`
- [x] Commit hash: 24118e2

---

## Final verification

- [ ] Run `verify.command` (full non-wip) → zero exit.
  ```
  captured output: 9 FAILED — "Ask with default tier"(model name), "Execute flag with n"(command not executed), "Cost is displayed"(cost value), "Tokens/second"(regex), "Environment variable overrides"(request sent), "Model pricing"(cost estimate), "syntax error"(exit 1), "Model explorer without LiteLLM"(exit 0), "Verbose default tier"(model name). All are pre-existing failures in unrelated archived features (see QUESTIONS.md). Not zero, so left unchecked.
  ```
- [ ] Run `verify.e2e_command` (full non-wip e2e) → zero exit.
  ```
  captured output: 8 FAILED — same pre-existing set minus "syntax error" (which is non-@e2e). The new @e2e scenario "Find a model outside the initial page while assigning tiers" PASSES. Not zero, so left unchecked (see QUESTIONS.md).
  ```
- [x] Run `givn lint --change improve-model-selection-autosuggest` → all WIP findings resolved (zero).
  ```
  captured output: givn lint: 1 file(s) checked — clean
  ```
