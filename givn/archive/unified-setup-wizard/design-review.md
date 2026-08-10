# Design Review: unified-setup-wizard

## Grilling Outcomes

### Keyboard Contract

The change from the previous model dialog contract is deliberate. Tab now
advances wizard pages and Shift-Tab/BackTab returns. Escape opens save/discard;
it is not page-back. Reasoning uses a dedicated Ctrl-R focus toggle on model
pages, then Up/Down changes the model-specific supported effort. This avoids
confusing `r` with model search and supports model catalogs where reasoning
efforts differ per model.

The PTY sequence for Shift-Tab is `ESC [ Z`. Ctrl-U remains available for
clearing URL and API-key inputs. The visible focus marker identifies whether
the model table or reasoning control is active.

### Persistence And Completion

The user chose valid-progress saving. A model row is completed only after
Enter/Return confirms it. Escape followed by Save validates the provider and
saves it plus only completed model pages; uncompleted tiers remain unchanged.
Escape followed by Discard and Ctrl-C perform no write. Enter on Large Model
confirms the final row, saves the full completed provider/tier result, and exits
successfully.

Catalog discovery failure is inline and retryable. It does not write an invalid
provider or close the wizard. The provider-only command stops after API key;
`setup` traverses all pages; `models` starts at Small Model. Non-TTY `setup`
uses the existing actionable guidance path and does not initialize Ratatui.

### Model-Specific Reasoning

The model catalog's optional reasoning object becomes runtime metadata on each
model. The page derives available efforts, default effort, enabled state, and
mandatory state from the selected model. Mandatory models cannot select `off`;
disabled models offer `off`; unsupported efforts are not shown. Changing the
model resets reasoning to that model's default or first valid effort. The
persisted tier reasoning format remains unchanged.

### Migration

The existing provider-layout and model-picker behavior conflicts with the new
page contract. A `@givn.modified` delta migrates the provider layout scenario;
existing model scenarios retain their intent while their step drivers move from
Tab/Escape to Ctrl-R/Shift-Tab. The implementation updates the permanent
step-driver behavior through the shared step modules and retains all `@e2e`
tags.

### Testability And E2E Fidelity

All scenarios use the real CLI subprocess through `portable-pty`. The new steps
assert the latest frame's explicit markers: `Page n of 5`, active tab, focus
label, and block cursor. They do not rely on cumulative output alone. Existing
PTY cleanup kills, waits, drains, and restores the test process if an assertion
fails. Model catalog calls use loopback `httpmock` fixtures.

### Arc42 Assessment

Arc42 is enabled. Independent assessment agrees with `arc42.md`: chapters 1, 3,
4, 5, 6, 8, 9, 10, 11, and 12 are affected; chapters 2 and 7 are not. All 12
chapter files exist with project-specific content and Mermaid-only diagrams.
ADR-0013 records the shared wizard and model-specific reasoning decision;
chapters 10 and 11 record the corresponding quality scenarios and migration /
partial-save risks.

## Hardening Applied

- Added a dedicated model reasoning focus and Ctrl-R keyboard contract.
- Added model-specific reasoning metadata, default, enabled, mandatory, and
  supported-effort behavior to the design and proposal.
- Defined valid-progress persistence, final-page save/exit, inline catalog retry,
  non-TTY behavior, and command-specific page ranges.
- Added a `@givn.modified` provider entry-point migration scenario and an
  interaction-matrix row.
- Updated arc42 chapters 1, 3, 4, 5, 6, 8, 9, 10, 11, and 12, plus ADR-0013.
- All active scenario `@wip` tags have been removed after their PTY bindings and
  implementation passed targeted verification.

## Status

DESIGN-REVIEW: PASS
