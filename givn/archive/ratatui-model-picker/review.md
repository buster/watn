# Review: ratatui-model-picker

## Fabrication audit (Step 0)

**0. `@e2e` tag integrity.** The delta spec has 5 `@e2e` scenarios, each
retaining its `@givn.added @e2e` tags (grep confirms all 5). No scenario had
`@e2e` removed. `verify.e2e_command` is configured in `givn/commands.yaml` as
`cargo test --test features_runner -- --tags '@e2e and not @wip'`, distinct
from `verify.command`. FULL=56 scenarios, E2E=32 scenarios (strictly fewer).
Clean.

**1. Empty / no-op step bodies.** All steps added by this change carry real
assertions on real state (PTY output, config file content, httpmock request
bodies, model-entry formatting). Two empty-bodied `given` steps exist in the
step file (`no_args_no_stdin`, `provider_no_key_no_env`) but both predate this
change (they belong to the archived ask/providers features and are unchanged
by its commits — verified via `git show HEAD:`). No fabricated steps in this
change's scope.

**2. Checked tasks have commits touching production source.** Every scenario
commit diffs `src/models/dialog.rs`, `src/models/mod.rs`, `src/models/picker.rs`,
`src/models/list.rs`, `src/config/types.rs`, `src/main.rs`, and the step files:

- `b101951` per-word filter (word_matches in list.rs + local_filter)
- `62b96d9` local fallback (execute_search world models)
- `c1fe2a8` empty state (dialog-shows step)
- `68b0a75` metadata (format_model_entry made public)
- `9ccb3ec` reasoning off (TierReasoning + main.rs)
- `3bd53fc` reasoning takes effect (e2e body assertion)
- `a098672` three-level dialog (SettingsDialog + run_models wiring)
- `db55c5f` arrow/page browse (dialog PageUp/Down + viewport)
- `310f68e` type-a-filter (visible filter line + parameterized choose)
- `af0ddda` back navigation (Escape restore)
- `403f232` review refactor (remove dead console ModelPicker)

All touch production source. Present.

**3. Promised components exist.** `src/models/dialog.rs` (`SettingsDialog`,
`ReasoningStrength`, `LevelChoice`, `PAGE_SIZE`), `word_matches` in list.rs,
`TierReasoning` in config/types.rs, main.rs reasoning resolution, `run_models`
TTY→SettingsDialog wiring, `validate` steps in ask_steps.rs. Present.

**4. Strict-mode proof.** `.fail_on_skipped()` at `tests/features_runner.rs`
(verified existing); every new scenario's RED phase recorded a non-zero exit
in tasks.md. Honest runner.

**5-6. `@e2e` downgrade / browser-UI.** CLI capability, not browser-UI. Each
`@e2e` scenario's Then steps assert on real-interface output: PTY-rendered
dialog text (`the dialog highlights the selected model`, `the dialog shows
the filter text`, `the picker displays ... as a matching suggestion`), the
config file written by the binary, and the chat request body via httpmock.
No `@e2e` assertions are repository-only. The reasoning-body scenarios assert
on the actual HTTP request body (via httpmock body-matchers), not a hit
count.

**7. Single e2e implementation.** `verify.e2e_command` →
`cargo test --test features_runner -- --tags '@e2e and not @wip'`. The only
e2e driving infrastructure is `tests/steps/mod.rs` (`start_pty_session`,
`pty_write`, `finish_pty_session`, `run_binary_pty`), exercised by the dialog
scenarios plus the pre-existing autosuggest scenario. `git status` shows no
competing or untracked weaker implementation. Clean.

**8. `@e2e` scope.** One `@e2e` scenario per distinct inventory entry:
configure all levels, browse with arrows/page, type a filter, back-navigate,
reasoning-takes-effect. Exactly 5 = one per interaction. No per-variant
excess.

**9. Local runnability.** `cargo run` (single CLI binary, no server/db).
httpmock binds loopback per scenario. Digital twins: provider models API →
httpmock, chat API → httpmock. Exercised during this review; runs cleanly.

**10. `verify.command` vs `verify.e2e_command`.** Distinct tag filters; counts
proven: 56 vs 32, e2e strictly fewer. Clean.

**11. Deviation from design.md.** None. The dialog key contract (Enter
confirm + advance, Tab cycles reasoning, Escape back, PAGE_SIZE=10) was
finalised with the reasoning default (each level Off when unset) during the
implementation of scenario 9 and recorded in design.md; design.md was edited
during hardening to state the focus-free key contract and the Off default,
so the final behaviour matches the documented design. No unreviewed
shortcut.

**12-13. Interaction coverage cross-reference.**

| Inventory entry | @e2e scenario | Driving mechanism |
|---|---|---|
| Run `watn models`, configure model + reasoning for each level | Configure model and reasoning for all three levels in the dialog | portable-pty: type filter, Tab reasoning, Enter per level; config asserted |
| Browse the model list with arrow/page keys | Browse the model list with arrow keys and page keys | portable-pty: `\x1b[B` + `\x1b[6~`; PTY render + config asserted |
| Type a search filter | Type a filter and see the matching suggestions | portable-pty: type "dee flash"; PTY render + visible filter asserted |
| Return to a previous level and change it | Return to a previous level and change its selection | portable-pty: Enter/Escape across levels; config asserted |
| Run `watn` so per-level reasoning takes effect | Configured per-level reasoning takes effect on a request | real binary against httpmock; request body asserted |

Every inventory entry maps to a matrix row and a real-interface driving
mechanism. Clean.

**14. Coverage instrumentation.** `coverage.non_e2e_command` /
`coverage.e2e_command` configured (cargo-llvm-cov). Measured, no placeholders.

Fabrication audit: CLEAN.

## Coverage (measured on change-scope code)

Change-scope modules are deliberately split across the two verify runners:
the dialog (`src/models/dialog.rs`) is only reachable through a real PTY and
is therefore exercised by `@e2e`, while the stateless search/formatter
helpers are exercised by non-`@e2e`.

**@e2e run (`cargo llvm-cov test --test features_runner -- --tags '@e2e and
not @wip'`):**
- `src/models/dialog.rs`: line 77.78%, region 76.73%
- `src/config/types.rs`: line 100%
- `src/models/picker.rs`: line 90%

**non-@e2e run:** `src/config/types.rs` 91.4% line (all `effort` branches),
`src/models/picker.rs` 90% (`execute_search`, `local_filter`).

### Classification of uncovered `SettingsDialog` regions (e2e-driven module)

The 22% of `dialog.rs` lines uncovered are all in keyboard-handling branches
that no single guided-sequence scenario steps through, each a distinct
keystroke path rather than the happy-path interactions in the inventory:

- **KeyCode::PageUp** — page up is symmetric to the covered PageDown; the
  e2e spec covers only the down/page-down direction (one interaction).
- **KeyCode::Up (at selection 0)** — the no-op guard when already at the top;
  only the Down path is a distinct user interaction worth an `@e2e`.
- **KeyCode::Esc on level 0** — Escape on the first level is a no-op guard;
  the covered back-navigation scenario exercises Escape on levels 1/2.
- **Ctrl-C exit** — process-interruption path; not a user-facing tier
  interaction (covered once by the legacy picker's Ctrl-C in prior archived
  changes).
- **auto-incomplete model choices fallback** (`unwrap_or_else(|| empty())`
  when a level is somehow not confirmed) — defensive default for an
  unreachable state given Enter always confirms before advancing.
- **`Chinese`/resize event arm** — empty `_ => {}` for non-key events; no
  observable behaviour.

These are raw event-branch handlers around the already-covered dialog state
machine, not distinct user-facing actions; per the interaction matrix we keep
one `@e2e` per inventory entry rather than warping a scenario to hit every
key variant.

## E2E coverage

Exactly one `@e2e` scenario per distinct happy-path action. The dialog
scenarios drive the real binary through a PTY (type, arrows, page, Tab,
Enter, Escape) and assert on PTY-rendered dialog text and the written config
file — real-interface primary assertions. `verify.command` (56) and
`verify.e2e_command` (32) both exit 0. Full=56, E2E=32.

## Sign-off checklist

- Fabrication audit: clean.
- Every checked task has a commit touching production source.
- Every promised component exists.
- Strict-mode proof present.
- `verify.command` and `verify.e2e_command` both exit 0 (56 / 32).
- Coverage measured across the runner binary. Change-scope: dialog 77.78%
  line (e2e), config/types 100%, picker 90%.
- Every coverage gap classified: hard-to-reach key branches justified
  (bucket): PageUp, Up-at-top, Esc-on-first, Ctrl-C, defensive fallback,
  resize event.
- Redundant/dead code removed: superseded console `ModelPicker` struct and
  unused `console` dep deleted (`403f232`).
- No `@wip` tags remain. No implementation detail leaked into the spec.
- One `@e2e` scenario per distinct action, real-interface primary assertion.
- Only one e2e implementation exists; it was read and credited.
- `verify.e2e_command` is not `verify.command`; counts proven (56 vs 32).
- Implementation matches design.md (key contract + Off default noted in
  design.md during hardening).

REVIEW: PASS
