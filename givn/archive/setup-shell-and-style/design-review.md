# Design Review: setup-shell-and-style

## Grilling (fresh-context subagent)

The plan was grilled in a fresh context against the code, both use-case
inventories, all touched permanent features, the PTY harness, and the arc42
chapters. Findings and dispositions:

| # | Severity | Finding | Disposition |
|---|---|---|---|
| 1 | Blocker | `highlight-active-setup-input` scenario "The green border follows optional shortcut focus" drives the removed question→list transition and asserts a question border that no longer exists; the design wrongly claimed all four focus scenarios stay valid. | Fixed: the old scenario is removed and replaced by `The green border follows the shell lists`, an `@e2e` scenario driving completion list → shortcut list; the use-case inventory title is updated; the design lists the replacement. |
| 2 | Blocker | Every existing "skip shell setup" step presses Enter, which now accepts preselection, and PTY children inherit the real `HOME`; coordinated e2e scenarios would write to real startup files. | Fixed: `start_pty_command` passes isolated `HOME`/`XDG_CONFIG_HOME` directly to the child when a scenario has not set them (never through `env_vars`, which `Drop` strips from the runner process); every skip step deselects the rendered selected shells. |
| 3 | Blocker | Coloring inactive borders indexed `240` contradicts the `assert_default` Foreground::Default checks in the focus scenarios. | Fixed: inactive borders use the default foreground plus the dim attribute; only labels/keys use indexed cyan. The palette table documents it. |
| 4 | Major | The `use-shell` scenarios "Enter accepts the default decline", "Selecting no shells…", and "The shell basename alone controls shortcut preselection" describe the retired question and `$SHELL` rule. | Fixed: the three scenarios are `@givn.removed`; `ShellEnvironment::detected_shells` is deleted; the replacements live in `streamlined-setup`. |
| 5 | Major | The initial "Declining shell setup" modification lost the baseline fixture and never reached `apply_shell_result`; the "no target inspection" title was also false because preselection inspects managed blocks. | Fixed: the old scenario is `@givn.removed` and replaced by "Declining shell setup writes no shell target", which keeps the baseline, sets an empty PATH, and drives through both pages plus Review. |
| 6 | Major | The proposal's "deselected shell removes its block" does not hold for coordinated `watn setup`, which installs only. | Fixed: the proposal scopes removal to the focused `watn shell` flow and keeps coordinated setup install-only, matching existing behavior. |
| 7 | Major | Style/test updates outside the design's step table (`ask_steps` `> model-12`, skip steps, highlight `y`, e2e choose steps, `[x]`). | Fixed: the Step Definition Locations table now names every affected file and binding. |
| 8 | Major | A Ratatui PTY always emits foreground/background reset sequences, so "no ANSI color sequences" is untestable; `NO_COLOR` could also leak from the runner. | Fixed: the assertion is "no indexed color sequences"; the harness removes inherited `NO_COLOR` before applying scenario env vars. |
| 9 | Major | PATH-fixture determinism was unspecified and `shells_available_on_path` checks `is_file`. | Fixed: the design specifies replace-PATH fixtures, isolated HOME, and executable fixture stubs; the glossary wording is "binary file present on PATH". |
| 10 | Major | Removing the opt-in question changes ADR-0018's documented default-decline and preselection clauses without an amendment. | Fixed: ADR-0018 carries an explicit amendment; chapter 9 and the change arc42 table mark it affected. |
| 11 | Minor | Stale chapter 8 and chapter 11 text still described default decline and `$SHELL` preselection. | Fixed in both chapters; the arc42 README summary was updated and noted outside the 12-row table. |
| 12 | Minor | Missing scenarios for managed-block-without-binary and manual selection. | Fixed: "A managed shell without a binary stays selected" was added; the modified independent-configuration scenario selects Zsh manually from an undetected state. |
| 13 | Minor | "No target inspection" could not be literally true; preselection reads managed blocks. | Fixed: the replacement scenario and all prose now say no target write or creation. |
| 14 | Minor | Malformed-marker deselect semantics drifted with preselection. | Fixed: the scenario sets `PATH=bash` so Bash starts `●` and the deselect step toggles it off explicitly before Enter. |
| 15 | Minor | Vocabulary drift and a missing step-table row. | Fixed: the design uses "setup visual language" consistently and lists `no shell opt-in question should be shown`. |

### Human questions raised

1. Removal scope — resolved: focused `watn shell` only; coordinated setup
   remains install-only.
2. Malformed block rendering — resolved: `PATH` preselection marks Bash `●`; a
   plain accept surfaces the malformed-marker error instead of silently keeping
   the block, and the scenario toggles it off explicitly.
3. `$SHELL` basename contract — resolved: retired with its scenarios and the
   `detected_shells` helper; `PATH` binaries plus managed blocks are the rule.
4. ADR-0018 — resolved: explicit amendment.
5. Plain-terminal guarantee — resolved: no indexed color (`38;5;`/`48;5;`);
   terminal reset sequences are allowed and are not an assertion target.

No question remains open.

## E2E Fidelity and Interaction Coverage

- Both interaction matrices list every inventory entry from the two use
  cases; all rows resolve to existing `@e2e` titles and real PTY or subprocess
  drivers.
- The change adds no inventory entry and no new consumer action. It modifies
  existing shell scenarios and adds regular presentation scenarios under the
  existing `unified-setup-wizard` and `streamlined-setup` actions.
- `@e2e` tags are retained on all modified e2e scenarios; no tag was removed.
- The e2e runner stays `./run-tests.sh --e2e`, a strict subset of
  `./run-tests.sh`; strict mode is `.fail_on_skipped()` plus the non-zero exit
  on skipped/failed.

## Domain and Ubiquitous Language

- Use cases `configure-model` and `configure-interactive`, Actor `Watn user`,
  confirmed Personas: none. No Persona was invented or promoted.
- `Shell preselection` and `Setup visual language` are recorded in the arc42
  glossary and used consistently in the proposal, specs, and design.

## Arc42 Cross-check

The 12-row assessment in `arc42.md` was independently re-derived: chapters 5,
6, 8, 9, 11, and 12 affected; 1, 2, 3, 4, 7, and 10 unaffected. The affected
chapter files contain the changes; all 12 chapters exist, contain real content,
and use Mermaid diagrams only. ADR-0018 is amended with a dated section;
chapter 9 is marked affected because of that amendment.

## Hardening Applied

- `proposal.md`: removal scoped to `watn shell`; no-write decline wording;
  routing table includes `interactive-shell-shortcut` and
  `highlight-active-setup-input`.
- `design.md`: default-foreground dim borders; `$SHELL` removal; enabled rule;
  coordinated install-only; harness HOME/XDG/NO_COLOR handling; full step table;
  ADR-0018 amendment route; new scenarios.
- `specs/configure-model/streamlined-setup.feature`: removed+added decline,
  detection scenario, managed-without-binary scenario, PATH fixtures.
- `specs/configure-interactive/unified-setup-wizard.feature`: indexed-color
  assertion.
- `specs/configure-interactive/highlight-active-setup-input.feature`: modified
  shortcut-focus scenario.
- `specs/use-shell/interactive-shell-shortcut.feature`: three removals.
- `docs/adr/0018-...md`, `docs/arc42/{05,06,08,11,12,README}`: amended or
  corrected.

`givn lint --change setup-shell-and-style` reports only advisory shape/subset
findings that are dispositioned in review (the decline supersession pair and
the retained distinct invariants).

DESIGN-REVIEW: PASS
