# Tasks: setup-shell-and-style

Use-case: `configure-model` and `configure-interactive`. Actor: `Watn user`.
Personas: none.
Verify: `./run-tests.sh` (regular), `./run-tests.sh --e2e` (e2e).
Single scenario: `./run-tests.sh --name "<scenario title>"`.

## Setup

- [x] Confirm the runner and strict mode.

  `givn/commands.yaml` declares `verify.command: "./run-tests.sh"` and
  `verify.e2e_command: "./run-tests.sh --e2e"`. Strict mode is
  `.fail_on_skipped()` in `tests/features_runner.rs` plus the non-zero exit on
  failed/skipped. The runner filters `@givn.removed` scenarios before
  execution, so removed permanent scenarios do not run during this change.

  Evidence (undefined step exits non-zero):
  `./run-tests.sh --name "Detected shells are preselected on the shell pages"`
  before implementation → `Step doesn't match any function`, exit 1.

---

## Scenario: Detected shells are preselected on the shell pages

- [x] RED — run targeted; new givens/steps undefined, exit 1. Evidence: setup
      proof run above.
- [x] GREEN — production: removed `ShellInstallFocus`, list-first key handling,
      preselection from `shells_available_on_path` plus managed blocks,
      `enabled` at advance, `●`/`○` markers; tests: PATH-fixture given,
      isolated HOME/XDG and `NO_COLOR` handling in `start_pty_command`, new
      shell steps. Evidence: targeted run → `1 scenario (1 passed)`,
      `13 steps (13 passed)`, exit 0. Files: `src/setup.rs`,
      `tests/steps/mod.rs`, `tests/features_runner.rs`,
      `tests/steps/streamlined_setup_steps.rs`,
      `specs/configure-model/streamlined-setup.feature`.
- [x] REFACTOR — removed the question key-inner helper with the focus enum; no
      behavior change. Evidence: targeted re-run → `1 scenario (1 passed)`,
      exit 0; `cargo test --lib` → `86 passed`.
- [x] COMMIT: feat(streamlined-setup): Detected shells are preselected on the shell pages — b6ba08e

## Scenario: Shell setup prefills installed integrations and removes only managed blocks when deselected

- [x] RED — mutation proof: forcing `enabled=false` leaves the managed block in
      place and the scenario fails (`2 scenarios (2 failed)`, exit 1); restored
      from git.
- [x] GREEN — list-first deselect flow without `y`; managed block removal.
      Evidence: targeted run → `2 scenarios (2 passed)`, `15 steps`, exit 0.
      Production shared with b6ba08e; step glue in b6ba08e.
- [x] REFACTOR — no behavior change. Evidence: targeted re-run → `2 scenarios
      (2 passed)`, exit 0.
- [x] COMMIT: feat(streamlined-setup): Shell setup prefills installed integrations and removes only managed blocks when deselected — b6ba08e, 20d1e59

## Scenario: Shell setup refuses malformed managed markers

- [x] RED — mutation proof: with `enabled=false` the malformed-marker error is
      never reported and the scenario fails (`2 scenarios (2 failed)`, exit 1);
      restored from git.
- [x] GREEN — PATH `bash` fixture, explicit toggle-off, Enter, malformed
      report. Evidence: targeted run → `2 scenarios (2 passed)`, `9 steps`,
      exit 0. Production and step glue shared with b6ba08e.
- [x] REFACTOR — no behavior change. Evidence: targeted re-run → `2 scenarios
      (2 passed)`, exit 0.
- [x] COMMIT: feat(streamlined-setup): Shell setup refuses malformed managed markers — b6ba08e

## Scenario: Declining shell setup writes no shell target

- [x] RED — mutation proof: forcing every shell preselected creates
      `home/.bashrc` and the scenario fails (`1 scenario (1 failed)`, exit 1);
      restored from git.
- [x] GREEN — empty PATH, no blocks, `enabled=false` on both pages, Review
      confirm; no target creation, config unchanged. Evidence: targeted run →
      `1 scenario (1 passed)`, `6 steps`, exit 0. Production shared with
      b6ba08e; new givens/steps in b6ba08e.
- [x] REFACTOR — no behavior change. Evidence: targeted re-run → `1 scenario
      (1 passed)`, exit 0.
- [x] COMMIT: feat(streamlined-setup): Declining shell setup writes no shell target — b6ba08e

## Scenario: A managed shell without a binary stays selected

- [x] RED — mutation proof: ignoring managed blocks leaves Bash `○` and the
      scenario fails (`1 scenario (1 failed)`, exit 1); restored from git.
- [x] GREEN — managed block alone drives preselection with empty PATH.
      Evidence: targeted run → `1 scenario (1 passed)`, `8 steps`, exit 0.
      Production shared with b6ba08e.
- [x] REFACTOR — no behavior change. Evidence: targeted re-run → `1 scenario
      (1 passed)`, exit 0.
- [x] COMMIT: feat(streamlined-setup): A managed shell without a binary stays selected — b6ba08e

## Scenario: Setup surfaces use the review visual language

- [x] RED — targeted run before the style implementation → undefined style
      step, exit 1.
- [x] GREEN — `SetupInk`, dim default-foreground borders, `watn · setup`
      frame, `◆` active tab, `▶` markers, bold-key/dim-label hints, cyan
      labels; test steps in `setup_wizard_steps.rs` and the model-table marker
      update in `ask_steps.rs`. Evidence: targeted run → `1 scenario
      (1 passed)`, `6 steps`, exit 0.
- [x] REFACTOR — removed the unused palette method and fixed affected test
      assertions (reasoning label stripping, shell-removal cleanup, save-prompt
      wording); full regular suite green. Evidence: `224 scenarios (224
      passed)`.
- [x] COMMIT: feat(unified-setup-wizard): Setup surfaces use the review visual language — ed39095

## Scenario: Setup warnings use amber attention markup

- [x] RED — full-run evidence: `Then the setup warning should be marked with
      "⚠" in amber` failed as an undefined step before the binding existed.
- [x] GREEN — manual-catalog notice and validation render `⚠` in indexed 214.
      Evidence: targeted run → `1 scenario (1 passed)`, `3 steps`, exit 0.
      Production shared with ed39095.
- [x] REFACTOR — no behavior change.
- [x] COMMIT: feat(unified-setup-wizard): Setup warnings use amber attention markup — ed39095

## Scenario: Setup renders readable text without color support

- [x] RED — mutation proof: forcing `SetupInk::new(true)` emits indexed colors
      under `NO_COLOR` and fails the indexed-color assertion; restored.
- [x] GREEN — `SetupInk` returns default styles when
      `terminal_supports_color` is false. Evidence: targeted run → `1 scenario
      (1 passed)`, `5 steps`, exit 0. Production shared with ed39095.
- [x] REFACTOR — no behavior change.
- [x] COMMIT: feat(unified-setup-wizard): Setup renders readable text without color support — ed39095

---

## E2E setup

- [x] Confirm the e2e environment and filter.

  In-process `httpmock` twins, PTY at 120x40, isolated HOME/XDG in the child
  env, prompt/catalog mocks. E2E bindings live in
  `tests/steps/streamlined_setup_e2e_steps.rs` and
  `tests/steps/highlight_active_setup_input_steps.rs`.

  Evidence (full vs e2e scenario counts, e2e strictly less):
  `./run-tests.sh` → `22 features, 224 scenarios (224 passed)`, exit 0.
  `./run-tests.sh --e2e` → `26 features, 89 scenarios (89 passed)`, exit 0.
  89 < 224, tag filter proven.

## Scenario: Shell setup independently configures completion and Ctrl-W integrations

- [x] RED — e2e targeted before the step rewrite → both delta and permanent
      fail (`Bash should contain a Watn-managed completion block`,
      `2 scenarios (2 failed)`, exit 1).
- [x] GREEN — `choose_only_shell` clears other selected rows, keeps Bash /
      selects Zsh, Review confirm. Evidence: e2e targeted run → `2 scenarios
      (2 passed)`, `21 steps`, exit 0.
- [x] REFACTOR — no behavior change. Evidence: full e2e suite green.
- [x] COMMIT: test(e2e): Shell setup independently configures completion and Ctrl-W integrations — df520cb

## Scenario: The green border follows the shell lists

- [x] RED — e2e targeted; `I advance past the completion selection` undefined,
      exit 1.
- [x] GREEN — completion list and shortcut list both render the green focused
      border; question-focus bindings removed. Evidence: e2e targeted run →
      `1 scenario (1 passed)`, `14 steps`, exit 0.
- [x] REFACTOR — removed the retired question/inactive step bindings and the
      unused `detected_shells` helper and its test-only use-shell steps; full
      suites green. Evidence: regular `224 passed`, e2e `89 passed`.
- [x] COMMIT: test(e2e): The green border follows the shell lists — 8bb02b9

---

## Removals and final checks

- [x] Removed scenarios and dead steps cleaned: `@givn.removed` scenarios are
      filtered by the runner; step bindings used only by them are deleted from
      `interactive_shell_shortcut_steps.rs` and
      `highlight_active_setup_input_steps.rs`; `ShellEnvironment::detected_shells`
      removed. Evidence: commit 8bb02b9; full suites green.
- [x] Full regular suite green: `./run-tests.sh` → `22 features, 224 scenarios
      (224 passed), 1377 steps`, exit 0.
- [x] Full e2e suite green: `./run-tests.sh --e2e` → `26 features, 89 scenarios
      (89 passed), 680 steps`, exit 0.
- [x] No empty/no-op step bodies introduced: grep for `unimplemented!`/`todo!`
      returns none in touched step files.
