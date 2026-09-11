# Tasks: setup-shell-and-style

Use-case: `configure-model` and `configure-interactive`. Actor: `Watn user`.
Personas: none.
Verify: `./run-tests.sh` (regular), `./run-tests.sh --e2e` (e2e).
Single scenario: `./run-tests.sh --name "<scenario title>"`.

## Setup

- [ ] Confirm the runner and strict mode.

  `givn/commands.yaml` declares `verify.command: "./run-tests.sh"` and
  `verify.e2e_command: "./run-tests.sh --e2e"`. Strict mode is
  `.fail_on_skipped()` in `tests/features_runner.rs` plus the non-zero exit on
  failed/skipped. The runner filters `@givn.removed` scenarios before
  execution, so removed permanent scenarios do not run during this change.

  Evidence (undefined step must exit non-zero):
  _pending_

---

## Scenario: Detected shells are preselected on the shell pages

Capability `streamlined-setup`. Core behavior: list-first pages, PATH
preselection, `●`/`○` markers, Enter applies.

- [ ] RED — remove `@wip` (none: added scenarios run when implemented); run
      targeted; new givens/steps must fail non-zero. Evidence: _pending_
- [ ] GREEN — production: remove `ShellInstallFocus`, list-first key handling,
      preselection from `shells_available_on_path` plus managed blocks,
      `enabled` at advance, `●`/`○` shell markers, delete
      `ShellEnvironment::detected_shells`; tests: PATH-fixture given, isolated
      HOME/XDG and `NO_COLOR` handling in `start_pty_command`, new shell steps
      in `streamlined_setup_steps.rs`. Evidence: _pending_
- [ ] REFACTOR — remove now-dead question steps; no behavior change.
      Evidence: _pending_
- [ ] COMMIT: feat(streamlined-setup): Detected shells are preselected on the shell pages — _pending_

## Scenario: Shell setup prefills installed integrations and removes only managed blocks when deselected

Capability `streamlined-setup`. Modified scenario.

- [ ] RED — run targeted; the list-first deselect flow must fail non-zero
      before the step glue matches it. Evidence: _pending_
- [ ] GREEN — deselect step toggles the preselected Bash row without a `y`;
      managed block removal. Evidence: _pending_
- [ ] REFACTOR — no behavior change. Evidence: _pending_
- [ ] COMMIT: feat(streamlined-setup): Shell setup prefills installed integrations and removes only managed blocks when deselected — _pending_

## Scenario: Shell setup refuses malformed managed markers

Capability `streamlined-setup`. Modified scenario.

- [ ] RED — run targeted; `y`-based step must fail or mismatch.
      Evidence: _pending_
- [ ] GREEN — PATH `bash` fixture, toggle Bash off explicitly, Enter, malformed
      report unchanged. Evidence: _pending_
- [ ] REFACTOR — no behavior change. Evidence: _pending_
- [ ] COMMIT: feat(streamlined-setup): Shell setup refuses malformed managed markers — _pending_

## Scenario: Declining shell setup writes no shell target

Capability `streamlined-setup`. Added replacement for the removed
inspection-phase scenario.

- [ ] RED — run targeted; new given/steps undefined. Evidence: _pending_
- [ ] GREEN — empty PATH, no blocks, `enabled=false` on both pages; drive
      through Review; assert no target creation and unchanged config.
      Evidence: _pending_
- [ ] REFACTOR — no behavior change. Evidence: _pending_
- [ ] COMMIT: feat(streamlined-setup): Declining shell setup writes no shell target — _pending_

## Scenario: A managed shell without a binary stays selected

Capability `streamlined-setup`. Added scenario.

- [ ] RED — run targeted; new steps undefined. Evidence: _pending_
- [ ] GREEN — managed block alone drives preselection when PATH is empty.
      Evidence: _pending_
- [ ] REFACTOR — no behavior change. Evidence: _pending_
- [ ] COMMIT: feat(streamlined-setup): A managed shell without a binary stays selected — _pending_

## Scenario: Setup surfaces use the review visual language

Capability `unified-setup-wizard`. Added scenario.

- [ ] RED — run targeted; new style steps undefined. Evidence: _pending_
- [ ] GREEN — `SetupInk`, dim default-foreground borders, `watn · setup`
      frame, `◆` active tab, `▶` markers, bold-key/dim-label hints, cyan
      labels; tests in `setup_wizard_steps.rs` and the model-table step update
      in `ask_steps.rs`. Evidence: _pending_
- [ ] REFACTOR — no behavior change. Evidence: _pending_
- [ ] COMMIT: feat(unified-setup-wizard): Setup surfaces use the review visual language — _pending_

## Scenario: Setup warnings use amber attention markup

Capability `unified-setup-wizard`. Added scenario.

- [ ] RED — run targeted; warning markup step undefined. Evidence: _pending_
- [ ] GREEN — validation and manual-catalog notices render `⚠` in amber; new
      then-step. Evidence: _pending_
- [ ] REFACTOR — no behavior change. Evidence: _pending_
- [ ] COMMIT: feat(unified-setup-wizard): Setup warnings use amber attention markup — _pending_

## Scenario: Setup renders readable text without color support

Capability `unified-setup-wizard`. Added scenario.

- [ ] RED — run targeted; color-capability given and assertion undefined.
      Evidence: _pending_
- [ ] GREEN — `SetupInk` returns default styles when
      `terminal_supports_color` is false; harness `NO_COLOR` handling; assert
      no indexed color sequences while `◆` remains. Evidence: _pending_
- [ ] REFACTOR — no behavior change. Evidence: _pending_
- [ ] COMMIT: feat(unified-setup-wizard): Setup renders readable text without color support — _pending_

---

## E2E setup

- [ ] Confirm the e2e environment and filter.

  In-process `httpmock` twins, PTY at 120x40, isolated HOME/XDG in the child
  env, prompt/catalog mocks. E2E step bindings for the shell flow live in
  `tests/steps/streamlined_setup_e2e_steps.rs` and
  `tests/steps/highlight_active_setup_input_steps.rs`;
  `verify.e2e_command` remains a strict subset of `verify.command`.

  Evidence (full vs e2e scenario counts): _pending_

## Scenario: Shell setup independently configures completion and Ctrl-W integrations

Capability `streamlined-setup`. Modified `@e2e` scenario.

- [ ] RED — run e2e targeted; PATH fixture and list-first steps must fail.
      Evidence: _pending_
- [ ] GREEN — choose-Bash step clears other selected rows and keeps Bash;
      choose-Zsh step toggles Zsh; Review confirm. Evidence: _pending_
- [ ] REFACTOR — no behavior change. Evidence: _pending_
- [ ] COMMIT: test(e2e): Shell setup independently configures completion and Ctrl-W integrations — _pending_

## Scenario: The green border follows the shell lists

Capability `highlight-active-setup-input`. Added `@e2e` replacement.

- [ ] RED — run e2e targeted; new focus steps undefined. Evidence: _pending_
- [ ] GREEN — completion list and shortcut list both render the green focused
      border; remove the question-focus step bindings. Evidence: _pending_
- [ ] REFACTOR — no behavior change. Evidence: _pending_
- [ ] COMMIT: test(e2e): The green border follows the shell lists — _pending_

---

## Removals and final checks

- [ ] Removed scenarios and dead steps cleaned: `@givn.removed` scenarios are
      filtered by the runner; step bindings used only by them are deleted from
      `interactive_shell_shortcut_steps.rs` and
      `highlight_active_setup_input_steps.rs`. Evidence: _pending_
- [ ] Full regular suite green: `./run-tests.sh`. Evidence: _pending_
- [ ] Full e2e suite green: `./run-tests.sh --e2e`. Evidence: _pending_
- [ ] No empty/no-op step bodies introduced. Evidence: _pending_
