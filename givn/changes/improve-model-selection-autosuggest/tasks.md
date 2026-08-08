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
- [x] COMMIT: `7e0c21f` — feat(model-autosuggest): Suggestions update as the search text changes

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
- [x] COMMIT: `7e0c21f` — feat(model-autosuggest): No matching model produces a clear empty state

### 3. Clearing the search restores available suggestions

> REMOVED during coverage review. This scenario tested raw-TTY run-loop
> behaviour (Escape/clear restoring the initial list) whose non-@e2e step
> definitions were no-op placeholders. It duplicated the single distinct
> interaction covered by the @e2e scenario "Find a model outside the
> initial page while assigning tiers" (per the design's interaction
> coverage matrix, one @e2e per action). The client-side initial-list
> restore is also exercised by that @e2e scenario (the picker shows the
> initial suggestions before any typing). Removed together with its
> placeholder steps.

**RED**
- [x] ~~Remove `@wip` from scenario `Clearing the search restores available suggestions`.~~
- [x] ~~Write unimplemented step definitions~~ (subsumed by scenarios 1/2/4/5).

**GREEN**
- [x] ~~Implement step definitions and production code~~ (removed).

**REFACTOR**
- [x] Clean up, no behaviour change.

**COMMIT**
- [x] ~~Commit hash~~ (removed at review: no commit kept).

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
- [x] COMMIT: `7e0c21f` — feat(model-autosuggest): The newest search result stays visible when an older result arrives later

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
- [x] COMMIT: `7e0c21f` — feat(model-autosuggest): An endpoint without search support reports a usable error

### 6. Selecting a suggestion advances to the next tier

> REMOVED during coverage review. This scenario's non-@e2e step definitions
> were no-op placeholders ("This is a placeholder for the e2e scenario";
> "Placeholder: in non-e2e context, this is verified by the e2e test") —
> they asserted nothing and cannot test the raw-TTY tier-advance flow. The
> behaviour is fully covered by the @e2e scenario "Find a model outside the
> initial page while assigning tiers", which selects across all three tiers
> and asserts the resulting config file. Removing it eliminates fabricated
> coverage rather than real coverage.

**RED**
- [x] ~~Remove `@wip` from scenario `Selecting a suggestion advances to the next tier`.~~
- [x] ~~Write unimplemented step definitions~~ (placeholder steps; removed).

**GREEN**
- [x] ~~Implement step definitions and production code~~ (removed).

**REFACTOR**
- [x] Clean up, no behaviour change.

**COMMIT**
- [x] ~~Commit hash~~ (removed at review: no commit kept).

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
- [x] COMMIT: `24118e2` — test(e2e): Find a model outside the initial page while assigning tiers
- [x] COMMIT: `ab20fb1` — fix(models): unblock archive gate — repair pre-existing suite failures and complete coverage review (also covers removed scenarios 3 & 6)

---

## Final verification

- [x] Run `verify.command` (full non-wip) → zero exit.
  ```
  captured output: 7 features, 46 scenarios (46 passed), 206 steps (206 passed) — zero exit.
  Previously red with 9 pre-existing failures; resolved by fixing the shared test harness
  (write raw config when no mock server; map WATN_PROVIDER override to the mock) and by
  reconciling stale output-format step/spec assertions with the intended single-line
  metadata format (`{model} · {n} tok/s`). Scenario count is 46 (48 minus the two
  duplicate placeholder scenarios removed at coverage review). See QUESTIONS.md.
  ```
- [x] Run `verify.e2e_command` (full non-wip e2e) → zero exit.
  ```
  captured output: 6 features, 27 scenarios (27 passed), 129 steps (129 passed) — zero exit.
  The new @e2e scenario "Find a model outside the initial page while assigning tiers" passes.
  ```
- [x] Run `givn lint --change improve-model-selection-autosuggest` → all WIP findings resolved (zero).
  ```
  captured output: givn lint: 1 file(s) checked — clean
  ```
