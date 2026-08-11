# Design Review: Highlight Active Setup Input

## Grilling Findings

### Scope

The proposal requires the active setup input location to be green, inactive
locations to retain their styling, and existing layout, keyboard, and cursor
behavior to remain unchanged. The design applies that rule to URL, credential,
model/reasoning, and optional shortcut focus states. The initial plan omitted
the optional shortcut screen from the executable spec; it was hardened with the
scenario `The green border follows optional shortcut focus` and a fourth
interaction-inventory entry.

No scope remains unresolved.

### Technical choices

Using the existing SetupWizard focus enums and conditional Ratatui block border
styles is the smallest implementation. A separate focus model, theme system,
or dependency would add state without improving the observable behavior.

PTY-based E2E testing is the correct boundary because the requirement is about
what a terminal user sees, including ANSI color. A unit test of an internal
style value would not prove the rendered terminal output.

### Missing scenarios and boundaries

- URL focus is covered by `The initial URL input has a green border`.
- Credential-source and credential-value focus, including inactive styling, are
  covered in both directions by `The green border follows API key focus`.
- Model-table and reasoning focus, including inactive styling, are covered by
  in both directions by `The green border follows model focus`.
- Optional shortcut-question and shell-list focus, including inactive styling,
  are covered in both directions by `The green border follows optional shortcut
  focus`.
- Validation and save/discard behavior are not changed by the rendering rule.
  Existing setup scenarios cover those transitions, and the focus state remains
  the same while validation text is drawn.

No additional error scenario is required for the stated behavior.

### Testability

Every new Then-step asserts a concrete ANSI-rendered border state. Before the
production border style is added, each active-border assertion fails because no
green SGR is emitted. The inactive assertions fail if the implementation
styles every block or leaks the active style across redraws.

The PTY transcript is cumulative and Ratatui may emit incremental diffs, so the
test design reconstructs the current 120x40 screen from cursor, erase, and SGR
commands before locating titled widget borders. It parses SGR parameters
semantically: foreground `32` or `38;5;2` is green, even when combined with a
background reset such as `49`. Before each two-widget focus transition, the
inactive companion's border is captured as the default baseline; after the
keypress, the old active widget is compared against that baseline and the new
active widget is checked for green. The PTY child removes inherited `NO_COLOR`
before setting `TERM=xterm-256color`, so ANSI color is deterministic.

### E2E fidelity and interaction coverage

The interface is a CLI terminal UI. `portable-pty` starts the real `watn
setup` subprocess, sends real keyboard bytes, and inspects its terminal output.
No browser or HTTP client substitutes for the user interface. The four
inventory entries each have exactly one matching E2E scenario and a non-empty
matrix row in `design.md`.

The model catalog is an in-process `httpmock` digital twin. No scenario uses a
live provider.

### Risk

The most likely implementation failure is applying green styling to the wrong
widget or to every widget because focus is not threaded into all draw methods.
The mitigation is a single conditional block-style helper, one scenario per
focus family, symmetric inactive-border assertions, semantic SGR parsing, and
removal of inherited `NO_COLOR` from the PTY child. Terminal palette contrast
remains documented as R-050; the cursor and focus text remain as redundant cues.

### Arc42 independent cross-check

The independent chapter assessment agrees with `arc42.md`:

| # | Chapter | Expected impact | Match |
|---|---|---|---|
| 1 | Introduction and goals | Yes: usability requirement and goal | Yes |
| 2 | Architecture constraints | No: existing terminal constraints remain | Yes |
| 3 | Context and scope | Yes: visible setup output changes | Yes |
| 4 | Solution strategy | Yes: focus-derived border styling | Yes |
| 5 | Building block view | Yes: SetupWizard rendering responsibility | Yes |
| 6 | Runtime view | Yes: focus transitions visibly move the border | Yes |
| 7 | Deployment view | No: no artifact or infrastructure change | Yes |
| 8 | Cross-cutting concepts | Yes: terminal focus presentation | Yes |
| 9 | Architecture decisions | No: refinement of ADR-0012, no new tradeoff | Yes |
| 10 | Quality requirements | Yes: QS-053 | Yes |
| 11 | Risks and technical debt | Yes: R-050 | Yes |
| 12 | Glossary | Yes: active input and focused widget | Yes |

All twelve chapter files exist, contain substantive content, and the affected
chapters describe the green focused-border behavior. The chapter-09 MADR list
already records the governing structured-widget decision (ADR-0012); this
change does not introduce a separate architectural decision. No ASCII-art
diagram was added; existing diagrams remain Mermaid or ordinary tables.

## Hardening Applied

- Added the optional shortcut focus scenario and inventory entry.
- Added its interaction-matrix row and runtime/quality documentation.
- Added symmetric inactive-state assertions for credential, model, and shortcut
  focus transitions.
- Specified 120x40 screen reconstruction, semantic SGR parsing, and PTY color
  policy (`NO_COLOR` removal).
- Specified the exact CSI controls handled by the screen reconstruction parser
  and the companion-widget baseline comparison for inactive styling.
- Updated the explicit `watn provider` runtime flow documentation.
- Ran `givn lint --change highlight-active-setup-input`; the only findings are
  the three expected `@wip` scenario markers; the initial URL scenario is the
  sole non-WIP scenario used for strict-mode proof.

## Open Questions

None.

DESIGN-REVIEW: PASS
